use crate::backend::{
    arithmetic::Field,
    avx_int_types::BlazeField,
    binary_extension_fields::B128,
    code::PackedRaaCode,
    hash::{Hash, Output},
    transcript::{TranscriptRead, TranscriptWrite},
    Deserialize, DeserializeOwned, Error, Serialize,
};
use crate::plonky2_util::log2_strict;
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

impl<F: BlazeField, H: Hash> PartialEq for Blaze2RaaQuery<F, H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == other.values && self.path == other.path
    }
}

impl<F: BlazeField, H: Hash> Eq for Blaze2RaaQuery<F, H> {}

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
        left += *challenge * F::to_b128_vec(vec![values[offset], values[offset + 2]]);
        right += *challenge * F::to_b128_vec(vec![values[offset + 1], values[offset + 3]]);
    }
    Ok((left, right))
}

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
