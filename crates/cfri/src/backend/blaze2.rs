use crate::backend::{
    arithmetic::Field,
    avx_int_types::BlazeField,
    binary_extension_fields::B128,
    code::PackedRaaCode,
    hash::{Hash, Output},
    transcript::{TranscriptRead, TranscriptWrite},
    Deserialize, DeserializeOwned, Error, Serialize,
};
use crate::plonky2_util::{log2_strict, reverse_index_bits_in_place};
use rayon::prelude::*;
use sha3::digest::{FixedOutputReset, Update};
use std::slice;

pub type CommitmentChunk<H> = Output<H>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "F: Serialize", deserialize = "F: DeserializeOwned"))]
pub struct Blaze2RaaCommitment<F: BlazeField, H: Hash> {
    code: PackedRaaCode,
    codeword_rows: Vec<Vec<F>>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2RaaPublicCommitment<H: Hash> {
    root: Output<H>,
    codeword_len: usize,
    num_rows: usize,
}

#[derive(Clone, Debug)]
pub struct Blaze2RaaQuery<F: BlazeField, H: Hash> {
    pub index: usize,
    pub values: Vec<F>,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2RaaTraceCommitment<H: Hash> {
    trace_rows: Vec<Vec<B128>>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2RaaTracePublicCommitment<H: Hash> {
    root: Output<H>,
    codeword_len: usize,
    num_rows: usize,
}

#[derive(Clone, Debug)]
pub struct Blaze2RaaTraceQuery<H: Hash> {
    pub index: usize,
    pub values: Vec<B128>,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug)]
pub struct Blaze2RaaTraceSpotQuery<H: Hash> {
    pub index: usize,
    pub queries: Vec<Blaze2RaaTraceQuery<H>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blaze2RaaTrace {
    pub u2: Vec<B128>,
    pub u3: Vec<B128>,
    pub u4: Vec<B128>,
    pub u5: Vec<B128>,
}

impl<F: BlazeField, H: Hash> PartialEq for Blaze2RaaQuery<F, H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == other.values && self.path == other.path
    }
}

impl<F: BlazeField, H: Hash> Eq for Blaze2RaaQuery<F, H> {}

impl<H: Hash> PartialEq for Blaze2RaaTraceQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == other.values && self.path == other.path
    }
}

impl<H: Hash> Eq for Blaze2RaaTraceQuery<H> {}

impl<H: Hash> PartialEq for Blaze2RaaTraceSpotQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.queries == other.queries
    }
}

impl<H: Hash> Eq for Blaze2RaaTraceSpotQuery<H> {}

impl<F: BlazeField, H: Hash> Blaze2RaaCommitment<F, H> {
    pub fn commit_rows(code: PackedRaaCode, rows: &[Vec<F>]) -> Result<Self, Error> {
        validate_message_rows(rows, code.message_len())?;
        let codeword_rows = code.encode_rows(rows);
        let merkle_tree = merkelize_rows::<F, H>(&codeword_rows);
        Ok(Self {
            code,
            codeword_rows,
            merkle_tree,
        })
    }

    pub fn commit_rows_and_write(
        code: PackedRaaCode,
        rows: &[Vec<F>],
        transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    ) -> Result<Self, Error> {
        let comm = Self::commit_rows(code, rows)?;
        transcript.write_commitment(comm.root())?;
        Ok(comm)
    }

    pub fn public(&self) -> Blaze2RaaPublicCommitment<H> {
        Blaze2RaaPublicCommitment {
            root: self.root().clone(),
            codeword_len: self.codeword_len(),
            num_rows: self.num_rows(),
        }
    }

    pub fn code(&self) -> &PackedRaaCode {
        &self.code
    }

    pub fn codeword_rows(&self) -> &[Vec<F>] {
        &self.codeword_rows
    }

    pub fn root(&self) -> &Output<H> {
        &self.merkle_tree[self.merkle_tree.len() - 1][0]
    }

    pub fn codeword_len(&self) -> usize {
        self.code.codeword_len()
    }

    pub fn num_rows(&self) -> usize {
        self.codeword_rows.len()
    }

    pub fn query(&self, index: usize) -> Result<Blaze2RaaQuery<F, H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        let pair_start = index & !1;
        let mut values = Vec::with_capacity(self.num_rows() * 2);
        for row in &self.codeword_rows {
            values.push(row[pair_start]);
            values.push(row[pair_start + 1]);
        }
        Ok(Blaze2RaaQuery {
            index,
            values,
            path: merkle_sibling_path::<H>(&self.merkle_tree, index),
        })
    }

    pub fn write_query(
        &self,
        index: usize,
        transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    ) -> Result<(), Error> {
        self.query(index)?.write(transcript)
    }
}

impl<H: Hash> Blaze2RaaTraceCommitment<H> {
    pub fn commit_trace(trace: &Blaze2RaaTrace) -> Result<Self, Error> {
        let codeword_len = validate_raa_trace_commitment_shape(trace)?;
        let trace_rows = vec![
            trace.u2.clone(),
            trace.u3.clone(),
            trace.u4.clone(),
            trace.u5.clone(),
        ];
        let merkle_tree = merkelize_b128_rows::<H>(&trace_rows);
        debug_assert_eq!(trace_rows[0].len(), codeword_len);
        Ok(Self {
            trace_rows,
            merkle_tree,
        })
    }

    pub fn public(&self) -> Blaze2RaaTracePublicCommitment<H> {
        Blaze2RaaTracePublicCommitment {
            root: self.root().clone(),
            codeword_len: self.codeword_len(),
            num_rows: self.num_rows(),
        }
    }

    pub fn trace_rows(&self) -> &[Vec<B128>] {
        &self.trace_rows
    }

    pub fn root(&self) -> &Output<H> {
        &self.merkle_tree[self.merkle_tree.len() - 1][0]
    }

    pub fn codeword_len(&self) -> usize {
        self.trace_rows[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.trace_rows.len()
    }

    pub fn query(&self, index: usize) -> Result<Blaze2RaaTraceQuery<H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        let pair_start = index & !1;
        let mut values = Vec::with_capacity(self.num_rows() * 2);
        for row in &self.trace_rows {
            values.push(row[pair_start]);
            values.push(row[pair_start + 1]);
        }
        Ok(Blaze2RaaTraceQuery {
            index,
            values,
            path: merkle_sibling_path::<H>(&self.merkle_tree, index),
        })
    }

    pub fn spot_query(
        &self,
        code: &PackedRaaCode,
        index: usize,
    ) -> Result<Blaze2RaaTraceSpotQuery<H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        if code.codeword_len() != self.codeword_len() {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 RAA trace commitment length does not match code".to_string(),
            ));
        }

        let permutation = code.permutation();
        let mut opened_pair_starts = Vec::with_capacity(3);
        push_unique_pair_start(&mut opened_pair_starts, index);
        if index > 0 {
            push_unique_pair_start(&mut opened_pair_starts, index - 1);
        }
        push_unique_pair_start(&mut opened_pair_starts, permutation.permutation2[index]);

        let mut queries = Vec::with_capacity(opened_pair_starts.len());
        for pair_start in opened_pair_starts {
            queries.push(self.query(pair_start)?);
        }
        Ok(Blaze2RaaTraceSpotQuery { index, queries })
    }
}

impl<H: Hash> Blaze2RaaPublicCommitment<H> {
    pub fn root(&self) -> &Output<H> {
        &self.root
    }

    pub fn codeword_len(&self) -> usize {
        self.codeword_len
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}

impl<H: Hash> Blaze2RaaTracePublicCommitment<H> {
    pub fn root(&self) -> &Output<H> {
        &self.root
    }

    pub fn codeword_len(&self) -> usize {
        self.codeword_len
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}

impl<F: BlazeField, H: Hash> AsRef<Output<H>> for Blaze2RaaCommitment<F, H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<F: BlazeField, H: Hash> AsRef<[Output<H>]> for Blaze2RaaCommitment<F, H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2RaaPublicCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2RaaPublicCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2RaaTraceCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2RaaTraceCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2RaaTracePublicCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2RaaTracePublicCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<F: BlazeField, H: Hash> Blaze2RaaQuery<F, H> {
    pub fn read(
        index: usize,
        num_rows: usize,
        codeword_len: usize,
        transcript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
    ) -> Result<Self, Error> {
        validate_query_index(index, codeword_len)?;
        if num_rows == 0 {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 RAA query expects at least one row".to_string(),
            ));
        }
        let path_len = log2_strict(codeword_len) - 1;
        let path = transcript.read_commitments(path_len)?;
        let values = transcript.read_field_elements(num_rows * 2)?;
        Ok(Self {
            index,
            values,
            path,
        })
    }

    pub fn write(
        &self,
        transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    ) -> Result<(), Error> {
        for sibling in &self.path {
            transcript.write_commitment(sibling)?;
        }
        transcript.write_field_elements(&self.values)
    }

    pub fn authenticate(&self, root: &Output<H>) -> Result<(), Error> {
        authenticate_query::<F, H>(self, root)
    }
}

impl<H: Hash> Blaze2RaaTraceQuery<H> {
    pub fn authenticate(
        &self,
        root: &Output<H>,
        num_rows: usize,
        codeword_len: usize,
    ) -> Result<(), Error> {
        authenticate_b128_query::<H>(self, root, num_rows, codeword_len)
    }
}

impl<H: Hash> Blaze2RaaTraceSpotQuery<H> {
    pub fn verify(
        &self,
        code: &PackedRaaCode,
        message: &[B128],
        root: &Output<H>,
    ) -> Result<(), Error> {
        verify_raa_trace_spot_query(code, message, root, self)
    }
}

pub fn fold_packed_query_pair<F: BlazeField>(
    values: &[F],
    challenges: &[B128],
) -> Result<(B128, B128), Error> {
    if values.len() != challenges.len() * 4 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA query does not match packed fold width".to_string(),
        ));
    }

    let mut left = B128::ZERO;
    let mut right = B128::ZERO;
    for (packed_row, challenge) in challenges.iter().enumerate() {
        let offset = packed_row * 4;
        left += *challenge * F::pack_pair_to_b128(values[offset], values[offset + 2]);
        right += *challenge * F::pack_pair_to_b128(values[offset + 1], values[offset + 3]);
    }
    Ok((left, right))
}

pub fn pack_interleaved_rows<F: BlazeField>(rows: &[Vec<F>]) -> Result<Vec<Vec<B128>>, Error> {
    let (packed_rows, row_len) = validate_interleaved_rows(rows)?;
    let mut packed = vec![vec![B128::ZERO; row_len]; packed_rows];
    pack_interleaved_rows_into(rows, &mut packed)?;
    Ok(packed)
}

pub fn pack_interleaved_rows_into<F: BlazeField>(
    rows: &[Vec<F>],
    packed: &mut [Vec<B128>],
) -> Result<(), Error> {
    let (packed_rows, row_len) = validate_interleaved_rows(rows)?;
    if packed.len() != packed_rows || !packed.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed row output has incompatible shape".to_string(),
        ));
    }

    packed
        .par_iter_mut()
        .zip(rows.par_chunks_exact(2))
        .for_each(|(out, pair)| {
            for col in 0..row_len {
                out[col] = F::pack_pair_to_b128(pair[0][col], pair[1][col]);
            }
        });
    Ok(())
}

pub fn fold_packed_rows_into(
    packed_rows: &[Vec<B128>],
    challenges: &[B128],
    out: &mut [B128],
) -> Result<(), Error> {
    let row_len = validate_packed_rows(packed_rows)?;
    if challenges.len() != packed_rows.len() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed fold challenge count does not match row count".to_string(),
        ));
    }
    if out.len() != row_len {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed fold output has incompatible length".to_string(),
        ));
    }

    out.par_iter_mut().enumerate().for_each(|(col, folded)| {
        let mut acc = B128::ZERO;
        for row in 0..packed_rows.len() {
            acc += challenges[row] * packed_rows[row][col];
        }
        *folded = acc;
    });
    Ok(())
}

pub fn evaluate_multilinear(
    values: &[B128],
    point: &[B128],
    scratch: &mut [B128],
) -> Result<B128, Error> {
    validate_multilinear_eval_shape(values, point, scratch)?;
    scratch.copy_from_slice(values);
    reverse_index_bits_in_place(scratch);

    let mut active_len = values.len();
    for challenge in point {
        let half = active_len >> 1;
        for idx in 0..half {
            scratch[idx] = scratch[idx << 1] + *challenge * scratch[(idx << 1) + 1];
        }
        active_len = half;
    }
    Ok(scratch[0])
}

pub fn evaluate_packed_rows_at_point_into(
    packed_rows: &[Vec<B128>],
    point: &[B128],
    out: &mut [B128],
    scratch: &mut [B128],
) -> Result<(), Error> {
    let row_len = validate_packed_rows(packed_rows)?;
    if point.len() != log2_strict(row_len) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 packed row evaluation point has {} coordinates for row length {row_len}",
            point.len()
        )));
    }
    if out.len() != packed_rows.len() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed row evaluation output count does not match row count".to_string(),
        ));
    }
    if scratch.len() != row_len {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed row evaluation scratch has incompatible length".to_string(),
        ));
    }

    for (eval, row) in out.iter_mut().zip(packed_rows) {
        *eval = evaluate_multilinear(row, point, scratch)?;
    }
    Ok(())
}

pub fn build_raa_trace(code: &PackedRaaCode, message: &[B128]) -> Result<Blaze2RaaTrace, Error> {
    let len = validate_raa_message(code, message)?;
    let mut trace = Blaze2RaaTrace {
        u2: vec![B128::ZERO; len],
        u3: vec![B128::ZERO; len],
        u4: vec![B128::ZERO; len],
        u5: vec![B128::ZERO; len],
    };
    build_raa_trace_into(code, message, &mut trace)?;
    Ok(trace)
}

pub fn build_raa_trace_into(
    code: &PackedRaaCode,
    message: &[B128],
    trace: &mut Blaze2RaaTrace,
) -> Result<(), Error> {
    let len = validate_raa_message(code, message)?;
    validate_raa_trace_shape(trace, len)?;

    let permutation = code.permutation();
    for i in 0..len {
        trace.u2[i] = message[permutation.permutation1[i] / code.rate()];
    }

    prefix_accumulate_into(&trace.u2, &mut trace.u3);

    for i in 0..len {
        trace.u4[i] = trace.u3[permutation.permutation2[i]];
    }

    prefix_accumulate_into(&trace.u4, &mut trace.u5);
    Ok(())
}

pub fn check_raa_trace_at(
    code: &PackedRaaCode,
    message: &[B128],
    trace: &Blaze2RaaTrace,
    index: usize,
) -> Result<(), Error> {
    let len = validate_raa_message(code, message)?;
    validate_raa_trace_shape(trace, len)?;
    validate_query_index(index, len)?;

    let permutation = code.permutation();
    let expected_u2 = message[permutation.permutation1[index] / code.rate()];
    if trace.u2[index] != expected_u2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA spot check failed repetition/permutation layer".to_string(),
        ));
    }

    let expected_u3 = if index == 0 {
        trace.u2[0]
    } else {
        trace.u3[index - 1] + trace.u2[index]
    };
    if trace.u3[index] != expected_u3 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA spot check failed first accumulator layer".to_string(),
        ));
    }

    if trace.u4[index] != trace.u3[permutation.permutation2[index]] {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA spot check failed second permutation layer".to_string(),
        ));
    }

    let expected_u5 = if index == 0 {
        trace.u4[0]
    } else {
        trace.u5[index - 1] + trace.u4[index]
    };
    if trace.u5[index] != expected_u5 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA spot check failed second accumulator layer".to_string(),
        ));
    }

    Ok(())
}

pub fn verify_raa_trace_spot_query<H: Hash>(
    code: &PackedRaaCode,
    message: &[B128],
    root: &Output<H>,
    opening: &Blaze2RaaTraceSpotQuery<H>,
) -> Result<(), Error> {
    let len = validate_raa_message(code, message)?;
    validate_query_index(opening.index, len)?;
    authenticate_trace_spot_queries(root, opening, len)?;

    let permutation = code.permutation();
    let index = opening.index;
    let expected_u2 = message[permutation.permutation1[index] / code.rate()];
    let u2 = trace_spot_value(opening, RAA_TRACE_ROW_U2, index)?;
    if u2 != expected_u2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA authenticated spot check failed repetition/permutation layer".to_string(),
        ));
    }

    let u3 = trace_spot_value(opening, RAA_TRACE_ROW_U3, index)?;
    let expected_u3 = if index == 0 {
        u2
    } else {
        trace_spot_value(opening, RAA_TRACE_ROW_U3, index - 1)? + u2
    };
    if u3 != expected_u3 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA authenticated spot check failed first accumulator layer".to_string(),
        ));
    }

    let u4 = trace_spot_value(opening, RAA_TRACE_ROW_U4, index)?;
    let permuted_u3 = trace_spot_value(opening, RAA_TRACE_ROW_U3, permutation.permutation2[index])?;
    if u4 != permuted_u3 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA authenticated spot check failed second permutation layer".to_string(),
        ));
    }

    let u5 = trace_spot_value(opening, RAA_TRACE_ROW_U5, index)?;
    let expected_u5 = if index == 0 {
        u4
    } else {
        trace_spot_value(opening, RAA_TRACE_ROW_U5, index - 1)? + u4
    };
    if u5 != expected_u5 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA authenticated spot check failed second accumulator layer".to_string(),
        ));
    }

    Ok(())
}

const RAA_TRACE_NUM_ROWS: usize = 4;
const RAA_TRACE_ROW_U2: usize = 0;
const RAA_TRACE_ROW_U3: usize = 1;
const RAA_TRACE_ROW_U4: usize = 2;
const RAA_TRACE_ROW_U5: usize = 3;

fn validate_message_rows<F: BlazeField>(rows: &[Vec<F>], row_len: usize) -> Result<(), Error> {
    if rows.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA commitment expects at least one row".to_string(),
        ));
    }
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA rows have incompatible lengths".to_string(),
        ));
    }
    Ok(())
}

fn validate_raa_message(code: &PackedRaaCode, message: &[B128]) -> Result<usize, Error> {
    if message.len() != code.message_len() {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 RAA message has {} entries but code expects {}",
            message.len(),
            code.message_len()
        )));
    }
    Ok(code.codeword_len())
}

fn validate_raa_trace_commitment_shape(trace: &Blaze2RaaTrace) -> Result<usize, Error> {
    let len = trace.u2.len();
    validate_query_index(0, len)?;
    validate_raa_trace_shape(trace, len)?;
    Ok(len)
}

fn validate_raa_trace_shape(trace: &Blaze2RaaTrace, len: usize) -> Result<(), Error> {
    if trace.u2.len() != len
        || trace.u3.len() != len
        || trace.u4.len() != len
        || trace.u5.len() != len
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA trace has incompatible word lengths".to_string(),
        ));
    }
    Ok(())
}

fn authenticate_trace_spot_queries<H: Hash>(
    root: &Output<H>,
    opening: &Blaze2RaaTraceSpotQuery<H>,
    codeword_len: usize,
) -> Result<(), Error> {
    if opening.queries.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA authenticated spot check has no openings".to_string(),
        ));
    }

    let mut seen_pair_starts = Vec::with_capacity(opening.queries.len());
    for query in &opening.queries {
        query.authenticate(root, RAA_TRACE_NUM_ROWS, codeword_len)?;
        let pair_start = query.index & !1;
        if seen_pair_starts.contains(&pair_start) {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 RAA authenticated spot check repeats an opening pair".to_string(),
            ));
        }
        seen_pair_starts.push(pair_start);
    }
    Ok(())
}

fn trace_spot_value<H: Hash>(
    opening: &Blaze2RaaTraceSpotQuery<H>,
    row: usize,
    index: usize,
) -> Result<B128, Error> {
    let pair_start = index & !1;
    let value_offset = row * 2 + (index & 1);
    for query in &opening.queries {
        if query.index & !1 == pair_start {
            if value_offset >= query.values.len() {
                return Err(Error::InvalidPcsOpen(
                    "Blaze2 RAA trace query has too few values".to_string(),
                ));
            }
            return Ok(query.values[value_offset]);
        }
    }

    Err(Error::InvalidPcsOpen(
        "Blaze2 RAA authenticated spot check is missing a required opening".to_string(),
    ))
}

fn push_unique_pair_start(opened_pair_starts: &mut Vec<usize>, index: usize) {
    let pair_start = index & !1;
    if !opened_pair_starts.contains(&pair_start) {
        opened_pair_starts.push(pair_start);
    }
}

fn prefix_accumulate_into(input: &[B128], out: &mut [B128]) {
    let mut acc = B128::ZERO;
    for (dst, value) in out.iter_mut().zip(input) {
        acc += *value;
        *dst = acc;
    }
}

fn validate_interleaved_rows<F: BlazeField>(rows: &[Vec<F>]) -> Result<(usize, usize), Error> {
    if rows.is_empty() || rows.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaving expects a non-empty even number of rows".to_string(),
        ));
    }
    let row_len = rows[0].len();
    if row_len == 0 || !row_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaving row length must be a non-empty power of two".to_string(),
        ));
    }
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaving rows have incompatible lengths".to_string(),
        ));
    }
    Ok((rows.len() >> 1, row_len))
}

fn validate_packed_rows(rows: &[Vec<B128>]) -> Result<usize, Error> {
    if rows.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed fold expects at least one row".to_string(),
        ));
    }
    let row_len = rows[0].len();
    if row_len == 0 || !row_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed fold row length must be a non-empty power of two".to_string(),
        ));
    }
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 packed fold rows have incompatible lengths".to_string(),
        ));
    }
    Ok(row_len)
}

fn validate_multilinear_eval_shape(
    values: &[B128],
    point: &[B128],
    scratch: &[B128],
) -> Result<(), Error> {
    if values.is_empty() || !values.len().is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 multilinear evaluation expects a non-empty power-of-two value vector"
                .to_string(),
        ));
    }
    if point.len() != log2_strict(values.len()) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 multilinear evaluation point has {} coordinates for {} values",
            point.len(),
            values.len()
        )));
    }
    if scratch.len() != values.len() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 multilinear evaluation scratch has incompatible length".to_string(),
        ));
    }
    Ok(())
}

fn validate_query_index(index: usize, codeword_len: usize) -> Result<(), Error> {
    if codeword_len == 0 || !codeword_len.is_power_of_two() || codeword_len < 2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA codeword length must be a power of two at least two".to_string(),
        ));
    }
    if index >= codeword_len {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 RAA query index {index} is outside codeword length {codeword_len}"
        )));
    }
    Ok(())
}

fn merkelize_rows<F: BlazeField, H: Hash>(rows: &[Vec<F>]) -> Vec<Vec<Output<H>>> {
    let row_len = rows[0].len();
    let log_len = log2_strict(row_len);
    let leaves = (0..(row_len >> 1))
        .into_par_iter()
        .map(|leaf_idx| {
            let mut hasher = H::new();
            let mut hash = Output::<H>::default();
            for row in rows {
                hasher.update_blaze_field(&row[leaf_idx << 1]);
                hasher.update_blaze_field(&row[(leaf_idx << 1) + 1]);
            }
            hasher.finalize_into_reset(&mut hash);
            hash
        })
        .collect::<Vec<_>>();

    let mut tree = Vec::with_capacity(log_len);
    tree.push(leaves);
    for level in 1..log_len {
        let parents = tree[level - 1]
            .par_chunks_exact(2)
            .map(|children| {
                let mut hasher = H::new();
                let mut hash = Output::<H>::default();
                hasher.update(&children[0]);
                hasher.update(&children[1]);
                hasher.finalize_into_reset(&mut hash);
                hash
            })
            .collect::<Vec<_>>();
        tree.push(parents);
    }
    tree
}

fn merkelize_b128_rows<H: Hash>(rows: &[Vec<B128>]) -> Vec<Vec<Output<H>>> {
    let row_len = rows[0].len();
    let log_len = log2_strict(row_len);
    let leaves = (0..(row_len >> 1))
        .into_par_iter()
        .map(|leaf_idx| {
            let mut hasher = H::new();
            let mut hash = Output::<H>::default();
            for row in rows {
                hasher.update_field_element(&row[leaf_idx << 1]);
                hasher.update_field_element(&row[(leaf_idx << 1) + 1]);
            }
            hasher.finalize_into_reset(&mut hash);
            hash
        })
        .collect::<Vec<_>>();

    let mut tree = Vec::with_capacity(log_len);
    tree.push(leaves);
    for level in 1..log_len {
        let parents = tree[level - 1]
            .par_chunks_exact(2)
            .map(|children| {
                let mut hasher = H::new();
                let mut hash = Output::<H>::default();
                hasher.update(&children[0]);
                hasher.update(&children[1]);
                hasher.finalize_into_reset(&mut hash);
                hash
            })
            .collect::<Vec<_>>();
        tree.push(parents);
    }
    tree
}

fn merkle_sibling_path<H: Hash>(tree: &[Vec<Output<H>>], mut query_index: usize) -> Vec<Output<H>> {
    let mut path = Vec::with_capacity(tree.len().saturating_sub(1));
    query_index >>= 1;
    for level in tree {
        if level.len() == 1 {
            break;
        }
        path.push(level[query_index ^ 1].clone());
        query_index >>= 1;
    }
    path
}

fn authenticate_b128_query<H: Hash>(
    query: &Blaze2RaaTraceQuery<H>,
    root: &Output<H>,
    num_rows: usize,
    codeword_len: usize,
) -> Result<(), Error> {
    validate_query_index(query.index, codeword_len)?;
    if num_rows == 0 || query.values.len() != num_rows * 2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA trace query value count does not match row count".to_string(),
        ));
    }
    if query.path.len() != log2_strict(codeword_len) - 1 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA trace query path has incompatible length".to_string(),
        ));
    }

    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    for pair in query.values.chunks_exact(2) {
        hasher.update_field_element(&pair[0]);
        hasher.update_field_element(&pair[1]);
    }
    hasher.finalize_into_reset(&mut hash);

    let mut query_index = query.index >> 1;
    for sibling in &query.path {
        let mut hasher = H::new();
        let mut next = Output::<H>::default();
        if query_index & 1 == 0 {
            hasher.update(&hash);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(&hash);
        }
        hasher.finalize_into_reset(&mut next);
        hash = next;
        query_index >>= 1;
    }

    if &hash != root {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA trace query does not authenticate".to_string(),
        ));
    }
    Ok(())
}

fn authenticate_query<F: BlazeField, H: Hash>(
    query: &Blaze2RaaQuery<F, H>,
    root: &Output<H>,
) -> Result<(), Error> {
    if query.values.is_empty() || query.values.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA query value count must be a non-empty list of pairs".to_string(),
        ));
    }

    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    for pair in query.values.chunks_exact(2) {
        hasher.update_blaze_field(&pair[0]);
        hasher.update_blaze_field(&pair[1]);
    }
    hasher.finalize_into_reset(&mut hash);

    let mut query_index = query.index >> 1;
    for sibling in &query.path {
        let mut hasher = H::new();
        let mut next = Output::<H>::default();
        if query_index & 1 == 0 {
            hasher.update(&hash);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(&hash);
        }
        hasher.finalize_into_reset(&mut next);
        hash = next;
        query_index >>= 1;
    }

    if &hash != root {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA query does not authenticate".to_string(),
        ));
    }
    Ok(())
}
