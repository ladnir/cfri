use crate::backend::{
    arithmetic::Field,
    avx_int_types::BlazeField,
    binary_extension_fields::B128,
    code::Permutation,
    hash::{Hash, Output},
    poly::multilinear::MultilinearPolynomial,
    transcript::{FieldTranscript, FieldTranscriptRead, TranscriptRead, TranscriptWrite},
    Deserialize, DeserializeOwned, Error, Serialize,
};
use crate::plonky2_util::log2_strict;
use halo2_curves::ff::PrimeField;
use rand::RngCore;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rayon::prelude::*;
use sha3::digest::{FixedOutputReset, Update};
use std::slice;

/// Blaze2 is the clean rewrite of the Blaze compression layer.
///
/// The proof shape is intentionally direct:
/// - commit once to the original encoded rows;
/// - expose the per-row evaluations at the column point;
/// - fold the packed rows with transcript challenges;
/// - prove queried original codeword columns against the folded backend codeword.
///
/// This first backend reveals the folded message directly. That keeps the
/// binding structure correct and small enough for focused tests while giving us
/// the right attachment point for a later succinct FRI/BaseFold backend.
pub type CommitmentChunk<H> = Output<H>;

const BLAZE2_LOG_RATE: usize = 2;
const BLAZE2_RATE: usize = 1 << BLAZE2_LOG_RATE;
const DEFAULT_BLAZE2_NUM_ROWS: usize = 64;
const DEFAULT_BLAZE2_NUM_QUERIES: usize = 16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2Params {
    permutation: Permutation,
    num_vars: usize,
    num_rows: usize,
    num_queries: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2ProverParam {
    permutation: Permutation,
    num_vars: usize,
    num_rows: usize,
    num_queries: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2VerifierParam {
    permutation: Permutation,
    num_vars: usize,
    num_rows: usize,
    num_queries: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "F: Serialize", deserialize = "F: DeserializeOwned"))]
pub struct Blaze2Commitment<F: BlazeField, H: Hash> {
    codeword: Vec<Vec<F>>,
    codeword_tree: Vec<Vec<Output<H>>>,
    message_rows: Vec<Vec<F>>,
}

impl<F: BlazeField, H: Hash> Blaze2Commitment<F, H> {
    pub fn public(&self) -> Self {
        Self {
            codeword: Vec::new(),
            codeword_tree: vec![vec![
                self.codeword_tree[self.codeword_tree.len() - 1][0].clone()
            ]],
            message_rows: Vec::new(),
        }
    }
}

impl<F: BlazeField, H: Hash> AsRef<Output<H>> for Blaze2Commitment<F, H> {
    fn as_ref(&self) -> &Output<H> {
        &self.codeword_tree[self.codeword_tree.len() - 1][0]
    }
}

impl<F: BlazeField, H: Hash> AsRef<[Output<H>]> for Blaze2Commitment<F, H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(&self.codeword_tree[self.codeword_tree.len() - 1][0])
    }
}

pub fn setup<H: Hash>(
    poly_size: usize,
    _: usize,
    mut rng: impl RngCore,
    num_rows: Option<usize>,
    num_queries: Option<usize>,
) -> Blaze2Params {
    let num_rows = num_rows.unwrap_or(DEFAULT_BLAZE2_NUM_ROWS);
    let num_queries = num_queries.unwrap_or(DEFAULT_BLAZE2_NUM_QUERIES);
    let num_vars = log2_strict(poly_size);
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut rng = ChaCha8Rng::from_seed(seed);
    let permutation = Permutation::create(&mut rng, poly_size * BLAZE2_RATE);
    Blaze2Params {
        permutation,
        num_vars,
        num_rows,
        num_queries,
    }
}

pub fn trim<H: Hash>(
    param: &Blaze2Params,
    poly_size: usize,
    _: usize,
) -> (Blaze2ProverParam, Blaze2VerifierParam) {
    assert_eq!(param.num_vars, log2_strict(poly_size));
    (
        Blaze2ProverParam {
            permutation: param.permutation.clone(),
            num_vars: param.num_vars,
            num_rows: param.num_rows,
            num_queries: param.num_queries,
        },
        Blaze2VerifierParam {
            permutation: param.permutation.clone(),
            num_vars: param.num_vars,
            num_rows: param.num_rows,
            num_queries: param.num_queries,
        },
    )
}

pub fn commit_and_write<F: BlazeField, H: Hash>(
    pp: &Blaze2ProverParam,
    rows: &[Vec<F>],
    transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
) -> Result<Blaze2Commitment<F, H>, Error> {
    validate_message_shape(rows, pp.num_rows, 1 << pp.num_vars)?;
    let codeword = rows
        .par_iter()
        .map(|row| encode_blaze_row(row, &pp.permutation))
        .collect::<Vec<_>>();
    let codeword_tree = merkelize_rows::<F, H>(&codeword);
    transcript.write_commitment(&codeword_tree[codeword_tree.len() - 1][0])?;
    Ok(Blaze2Commitment {
        codeword,
        codeword_tree,
        message_rows: rows.to_vec(),
    })
}

pub fn evaluate_commitment<F: BlazeField, H: Hash>(
    comm: &Blaze2Commitment<F, H>,
    point: &[B128],
) -> Result<B128, Error> {
    let b128_rows = packed_message_rows(&comm.message_rows)?;
    evaluate_matrix(&b128_rows, point)
}

pub fn open<F: BlazeField, H: Hash>(
    pp: &Blaze2ProverParam,
    rows: &[Vec<F>],
    comm: &Blaze2Commitment<F, H>,
    point: &[B128],
    eval: &B128,
    blaze_transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    b128_transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, B128>,
) -> Result<B128, Error> {
    validate_message_shape(rows, pp.num_rows, 1 << pp.num_vars)?;
    validate_opening_point(point, pp.num_rows, pp.num_vars)?;
    if comm.codeword.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 prover requires a full commitment".to_string(),
        ));
    }

    let packed_rows = packed_message_rows(rows)?;
    let num_row_vars = log2_strict(packed_rows.len());
    let (row_point, col_point) = point.split_at(num_row_vars);
    let row_evals = packed_rows
        .iter()
        .map(|row| evaluate_mle(row, col_point))
        .collect::<Result<Vec<_>, _>>()?;
    let claimed_eval = evaluate_mle(&row_evals, row_point)?;
    if &claimed_eval != eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 claimed evaluation does not match row evaluations".to_string(),
        ));
    }
    b128_transcript.write_field_elements(&row_evals)?;

    let fold_challenges = b128_transcript.squeeze_challenges(packed_rows.len());
    let folded_message = linear_combination(&packed_rows, &fold_challenges);
    let folded_eval = evaluate_mle(&folded_message, col_point)?;
    let expected_folded_eval = inner_product(&row_evals, &fold_challenges);
    if folded_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded evaluation does not match folded row evaluations".to_string(),
        ));
    }

    b128_transcript.write_field_elements(&folded_message)?;
    let query_indices = squeeze_query_indices(
        b128_transcript,
        pp.num_queries,
        1 << (pp.num_vars + BLAZE2_LOG_RATE),
    );

    for &query_index in &query_indices {
        write_blaze_query::<F, H>(
            &comm.codeword,
            &comm.codeword_tree,
            query_index,
            blaze_transcript,
        )?;
    }

    Ok(claimed_eval)
}

pub fn verify<F: BlazeField, H: Hash>(
    vp: &Blaze2VerifierParam,
    comm: &Blaze2Commitment<F, H>,
    point: &[B128],
    eval: &B128,
    b128_transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blaze_transcript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> Result<(), Error> {
    validate_opening_point(point, vp.num_rows, vp.num_vars)?;
    let transcript_point = b128_transcript.squeeze_challenges(point.len());
    if transcript_point != point {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening point does not match transcript challenge".to_string(),
        ));
    }

    let proof_root = blaze_transcript.read_commitment()?;
    if &proof_root != AsRef::<Output<H>>::as_ref(comm) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 proof commitment root does not match public commitment".to_string(),
        ));
    }

    let num_packed_rows = vp.num_rows >> 1;
    let num_row_vars = log2_strict(num_packed_rows);
    let (row_point, col_point) = point.split_at(num_row_vars);
    let row_evals = b128_transcript.read_field_elements(num_packed_rows)?;
    let claimed_eval = evaluate_mle(&row_evals, row_point)?;
    if &claimed_eval != eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 claimed evaluation does not match row evaluations".to_string(),
        ));
    }

    let fold_challenges = b128_transcript.squeeze_challenges(num_packed_rows);
    let folded_message = b128_transcript.read_field_elements(1 << vp.num_vars)?;
    let folded_eval = evaluate_mle(&folded_message, col_point)?;
    let expected_folded_eval = inner_product(&row_evals, &fold_challenges);
    if folded_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded backend proof does not match row evaluations".to_string(),
        ));
    }

    let folded_codeword = encode_b128_row(&folded_message, &vp.permutation);
    let query_indices = squeeze_query_indices(
        b128_transcript,
        vp.num_queries,
        1 << (vp.num_vars + BLAZE2_LOG_RATE),
    );

    for &query_index in &query_indices {
        let leaf = read_blaze_query::<F, H>(
            vp.num_rows,
            1 << (vp.num_vars + BLAZE2_LOG_RATE),
            blaze_transcript,
        )?;
        authenticate_blaze_query::<F, H>(&leaf, query_index, AsRef::<Output<H>>::as_ref(comm))?;
        let folded_pair = fold_query_leaf(&leaf.values, &fold_challenges)?;
        let p0 = query_index & !1;
        let expected_pair = (folded_codeword[p0], folded_codeword[p0 + 1]);
        if folded_pair != expected_pair {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 folded query does not match backend folded codeword".to_string(),
            ));
        }
    }

    Ok(())
}

fn validate_message_shape<F: BlazeField>(
    rows: &[Vec<F>],
    num_rows: usize,
    row_len: usize,
) -> Result<(), Error> {
    if rows.len() != num_rows || rows.is_empty() || rows.len() % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 expects a non-empty even number of rows".to_string(),
        ));
    }
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 rows have incompatible lengths".to_string(),
        ));
    }
    Ok(())
}

fn validate_opening_point(
    point: &[B128],
    num_rows: usize,
    num_col_vars: usize,
) -> Result<(), Error> {
    if num_rows == 0 || num_rows % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 verifier expects an even number of rows".to_string(),
        ));
    }
    let num_row_vars = log2_strict(num_rows >> 1);
    if point.len() != num_row_vars + num_col_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze2 opening point length: expected {}, got {}",
            num_row_vars + num_col_vars,
            point.len()
        )));
    }
    Ok(())
}

fn packed_message_rows<F: BlazeField>(rows: &[Vec<F>]) -> Result<Vec<Vec<B128>>, Error> {
    if rows.is_empty() || rows.len() % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 expects a non-empty even number of rows".to_string(),
        ));
    }
    let row_len = rows[0].len();
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 rows have incompatible lengths".to_string(),
        ));
    }
    Ok(rows
        .chunks_exact(2)
        .map(|pair| {
            (0..row_len)
                .map(|col| F::to_b128_vec(vec![pair[0][col], pair[1][col]]))
                .collect::<Vec<_>>()
        })
        .collect())
}

fn evaluate_matrix(rows: &[Vec<B128>], point: &[B128]) -> Result<B128, Error> {
    let num_row_vars = log2_strict(rows.len());
    let (row_point, col_point) = point.split_at(num_row_vars);
    let row_evals = rows
        .iter()
        .map(|row| evaluate_mle(row, col_point))
        .collect::<Result<Vec<_>, _>>()?;
    evaluate_mle(&row_evals, row_point)
}

fn evaluate_mle(evals: &[B128], point: &[B128]) -> Result<B128, Error> {
    if evals.len() != (1usize << point.len()) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 multilinear evaluation shape mismatch".to_string(),
        ));
    }
    Ok(MultilinearPolynomial::new(evals.to_vec()).evaluate(point))
}

fn linear_combination(rows: &[Vec<B128>], coeffs: &[B128]) -> Vec<B128> {
    debug_assert_eq!(rows.len(), coeffs.len());
    let row_len = rows[0].len();
    (0..row_len)
        .into_par_iter()
        .map(|col| {
            let mut acc = B128::ZERO;
            for row in 0..rows.len() {
                acc += coeffs[row] * rows[row][col];
            }
            acc
        })
        .collect()
}

fn inner_product(lhs: &[B128], rhs: &[B128]) -> B128 {
    debug_assert_eq!(lhs.len(), rhs.len());
    lhs.iter()
        .zip(rhs.iter())
        .fold(B128::ZERO, |acc, (lhs, rhs)| acc + *lhs * *rhs)
}

fn encode_blaze_row<F: BlazeField>(row: &[F], permutation: &Permutation) -> Vec<F> {
    let mut repeated = vec![F::zero(); row.len() * BLAZE2_RATE];
    for (idx, value) in row.iter().enumerate() {
        for repeat in 0..BLAZE2_RATE {
            repeated[idx * BLAZE2_RATE + repeat] = *value;
        }
    }
    let mut first = vec![F::zero(); repeated.len()];
    first
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, value)| *value = repeated[permutation.permutation1[idx]]);
    serial_accumulate_blaze(&mut first);

    let mut second = vec![F::zero(); first.len()];
    second
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, value)| *value = first[permutation.permutation2[idx]]);
    serial_accumulate_blaze(&mut second);
    second
}

fn encode_b128_row(row: &[B128], permutation: &Permutation) -> Vec<B128> {
    let mut repeated = vec![B128::ZERO; row.len() * BLAZE2_RATE];
    for (idx, value) in row.iter().enumerate() {
        for repeat in 0..BLAZE2_RATE {
            repeated[idx * BLAZE2_RATE + repeat] = *value;
        }
    }
    let mut first = vec![B128::ZERO; repeated.len()];
    first
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, value)| *value = repeated[permutation.permutation1[idx]]);
    serial_accumulate_b128(&mut first);

    let mut second = vec![B128::ZERO; first.len()];
    second
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, value)| *value = first[permutation.permutation2[idx]]);
    serial_accumulate_b128(&mut second);
    second
}

fn serial_accumulate_blaze<F: BlazeField>(values: &mut [F]) {
    let mut prev = F::zero();
    for value in values {
        *value = *value ^ prev;
        prev = *value;
    }
}

fn serial_accumulate_b128(values: &mut [B128]) {
    let mut prev = B128::ZERO;
    for value in values {
        *value += prev;
        prev = *value;
    }
}

fn merkelize_rows<F: BlazeField, H: Hash>(rows: &[Vec<F>]) -> Vec<Vec<Output<H>>> {
    let row_len = rows[0].len();
    let log_len = log2_strict(row_len);
    let mut tree = Vec::with_capacity(log_len);
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

struct BlazeQueryLeaf<F: BlazeField, H: Hash> {
    path: Vec<Output<H>>,
    values: Vec<F>,
}

fn write_blaze_query<F: BlazeField, H: Hash>(
    codeword: &[Vec<F>],
    tree: &[Vec<Output<H>>],
    query_index: usize,
    transcript: &mut impl TranscriptWrite<Output<H>, F>,
) -> Result<(), Error> {
    for sibling in merkle_sibling_path::<H>(tree, query_index) {
        transcript.write_commitment(&sibling)?;
    }
    let p0 = query_index & !1;
    for row in codeword {
        transcript.write_field_element(&row[p0])?;
        transcript.write_field_element(&row[p0 + 1])?;
    }
    Ok(())
}

fn read_blaze_query<F: BlazeField, H: Hash>(
    num_rows: usize,
    codeword_len: usize,
    transcript: &mut impl TranscriptRead<Output<H>, F>,
) -> Result<BlazeQueryLeaf<F, H>, Error> {
    let path_len = log2_strict(codeword_len) - 1;
    let path = transcript.read_commitments(path_len)?;
    let values = transcript.read_field_elements(num_rows * 2)?;
    Ok(BlazeQueryLeaf { path, values })
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

fn authenticate_blaze_query<F: BlazeField, H: Hash>(
    leaf: &BlazeQueryLeaf<F, H>,
    mut query_index: usize,
    root: &Output<H>,
) -> Result<(), Error> {
    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    for pair in leaf.values.chunks_exact(2) {
        hasher.update_blaze_field(&pair[0]);
        hasher.update_blaze_field(&pair[1]);
    }
    hasher.finalize_into_reset(&mut hash);

    query_index >>= 1;
    for sibling in &leaf.path {
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
            "Blaze2 Merkle query does not authenticate".to_string(),
        ));
    }
    Ok(())
}

fn fold_query_leaf<F: BlazeField>(
    values: &[F],
    challenges: &[B128],
) -> Result<(B128, B128), Error> {
    if values.len() != challenges.len() * 4 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 query leaf has wrong number of row values".to_string(),
        ));
    }
    let mut left = B128::ZERO;
    let mut right = B128::ZERO;
    for (packed_row, challenge) in challenges.iter().enumerate() {
        let offset = packed_row * 4;
        left += *challenge * F::to_b128_vec(vec![values[offset], values[offset + 2]]);
        right += *challenge * F::to_b128_vec(vec![values[offset + 1], values[offset + 3]]);
    }
    Ok((left, right))
}

fn squeeze_query_indices(
    transcript: &mut impl FieldTranscript<B128>,
    num_queries: usize,
    codeword_len: usize,
) -> Vec<usize> {
    transcript
        .squeeze_challenges(num_queries)
        .into_iter()
        .map(|challenge| {
            let repr = challenge.to_repr();
            let bytes = repr.as_ref();
            let (int_bytes, _) = bytes.split_at(std::mem::size_of::<u32>());
            let index = u32::from_be_bytes(
                int_bytes
                    .try_into()
                    .expect("B128 challenge representation has at least four bytes"),
            ) as usize;
            index % codeword_len
        })
        .collect()
}
