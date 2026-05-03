//! Blaze2 protocol notes.
//!
//! This module is being rebuilt around the shape of Blaze's interleaved packed
//! RAA protocol. The key point is that the committed object is an interleaved
//! final codeword matrix, while the RAA intermediate states are checked only
//! for a folded instance during opening.
//!
//! Let `F` be the binary extension field, `t` the number of interleaved rows,
//! `k` the packed RAA message length per row, and `n` the packed RAA codeword
//! length per row. The input polynomial table is viewed as
//!
//! ```text
//! m in F^{t x k}
//! m_i in F^k
//! ```
//!
//! For each original row, packed RAA encoding is
//!
//! ```text
//! u1_i = R  m_i
//! u2_i = P0 u1_i
//! u3_i = A  u2_i
//! u4_i = P1 u3_i
//! u5_i = A  u4_i
//! c_i  = u5_i
//! ```
//!
//! The paper's PCS commitment does not commit to `u1_i, u2_i, u3_i, u4_i`
//! for every original row. It commits to the final interleaved codeword
//! matrix:
//!
//! ```text
//! c in F^{t x n}
//! c_i = PRAA(m_i)
//! ```
//!
//! The natural Merkle leaf/address is one column symbol:
//!
//! ```text
//! c[:, j] = (c_0[j], c_1[j], ..., c_{t-1}[j]) in F^t
//! ```
//!
//! So, yes: in the final commitment address space, each `c[:, j]` is one
//! atomic opened item/leaf. It is not a separate Merkle root; it is one address
//! inside the global committed vector.
//!
//! For an opening point `z = (z_row, z_col)`, where
//! `z_row in F^{log t}` and `z_col in F^{log k}`, the prover sends row
//! evaluations
//!
//! ```text
//! e_i = m_hat_i(z_col)
//! e   = (e_0, ..., e_{t-1})
//! ```
//!
//! The verifier checks `e_hat(z_row)` against the claimed evaluation. Thus
//! `e_i = m_i[z_col]` only when `z_col` is Boolean. For a random field point,
//! `e_i` is the multilinear extension of row `m_i` evaluated at `z_col`. This
//! larger field domain is the binding trick: one random non-Boolean evaluation
//! checks a whole row with high probability.
//!
//! The verifier then samples row-folding challenges `rho in F^t`:
//!
//! ```text
//! m_star = sum_i rho_i m_i
//! c_star = sum_i rho_i c_i
//! ```
//!
//! The row-evaluation vector is bound to the folded message by the scalar
//! equation
//!
//! ```text
//! m_star(z_col) = sum_i rho_i e_i
//! ```
//!
//! Because `rho` is sampled after `e` is transcript-bound, this is the
//! reduction from the original matrix opening to one folded-message opening.
//! The inner backend is responsible for proving the folded PRAA relation:
//! `m_star(z_col)` equals the folded evaluation and the queried values of
//! `PRAA(m_star)` match the input-oracle values supplied by Blaze2. Blaze2
//! supplies those input-oracle values by opening one interleaved codeword
//! column per backend query.
//!
//! Since packed RAA is linear, `c_star = PRAA(m_star)`. The RAA checks are run
//! for this single folded instance:
//!
//! ```text
//! v1 = R  m_star
//! v2 = P0 v1
//! v3 = A  v2
//! v4 = P1 v3
//! v5 = A  v4
//! ```
//!
//! Queried values of `v5` should be derived from opened committed columns:
//!
//! ```text
//! v5[j] = c_star[j] = sum_i rho_i c_i[j]
//! ```
//!
//! Current implementation note: `Blaze2OpeningProof` must only carry the
//! interleaved codeword column openings and the folded-message backend proof.
//! The auxiliary RAA trace types below are independent test/reference
//! machinery for auditing the RAA algebra; they are not part of the Blaze2
//! opening proof shape.
use crate::backend::{
    arithmetic::Field,
    avx_int_types::BlazeField,
    binary_extension_fields::B128,
    code::PackedRaaCode,
    hash::{Blake2s, Hash, Output},
    transcript::{TranscriptRead, TranscriptWrite},
    Deserialize, DeserializeOwned, Error, Serialize,
};
use crate::plonky2_util::{log2_strict, reverse_index_bits_in_place};
use crate::transcript::Transcript as CfriTranscript;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2InterleavedCodewordCommitment<H: Hash> {
    codeword_rows: Vec<Vec<B128>>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2InterleavedCodewordPublicCommitment<H: Hash> {
    root: Output<H>,
    codeword_len: usize,
    num_rows: usize,
}

#[derive(Clone, Debug)]
pub struct Blaze2InterleavedColumnQuery<H: Hash> {
    pub index: usize,
    pub values: Vec<B128>,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blaze2RaaAuxTrace {
    pub u2: Vec<B128>,
    pub u3: Vec<B128>,
    pub u4: Vec<B128>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2RaaAuxTraceCommitment<H: Hash> {
    trace_rows: Vec<Vec<B128>>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blaze2RaaAuxTracePublicCommitment<H: Hash> {
    root: Output<H>,
    codeword_len: usize,
    num_rows: usize,
}

#[derive(Clone, Debug)]
pub struct Blaze2RaaAuxTraceQuery<H: Hash> {
    pub index: usize,
    pub values: Vec<B128>,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug)]
pub struct Blaze2RaaAuxTraceSpotQuery<H: Hash> {
    pub index: usize,
    pub queries: Vec<Blaze2RaaAuxTraceQuery<H>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blaze2OpeningClaim {
    pub row_point: Vec<B128>,
    pub col_point: Vec<B128>,
    pub value: B128,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blaze2CodeSeed(pub [u8; 32]);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum Blaze2FieldId {
    B128,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum Blaze2HashId {
    Blake2s,
    Blake2s256,
}

pub trait Blaze2HashSpec: Hash {
    const BLAZE2_HASH_ID: Blaze2HashId;
}

impl Blaze2HashSpec for Blake2s {
    const BLAZE2_HASH_ID: Blaze2HashId = Blaze2HashId::Blake2s;
}

impl Blaze2HashSpec for blake2::Blake2s256 {
    const BLAZE2_HASH_ID: Blaze2HashId = Blaze2HashId::Blake2s256;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum RaaVariant {
    PackedPrefixAccumulator,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum Blaze2PackingLayout {
    PackedInterleavedRows,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum Blaze2LeafLayout {
    InterleavedColumn,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blaze2CodeSpec {
    pub version: u32,
    pub field_id: Blaze2FieldId,
    pub hash_id: Blaze2HashId,
    pub raa_variant: RaaVariant,
    pub packing: Blaze2PackingLayout,
    pub leaf_layout: Blaze2LeafLayout,
    pub praa_message_len: usize,
    pub praa_expansion_factor: usize,
    pub praa_codeword_len: usize,
    pub seed: Blaze2CodeSeed,
}

#[derive(Clone, Debug)]
pub struct Blaze2Code {
    spec: Blaze2CodeSpec,
    packed: PackedRaaCode,
}

#[derive(Clone, Debug)]
pub struct Blaze2OpeningQuery<H: Hash> {
    pub column_opening: Blaze2InterleavedColumnQuery<H>,
}

#[derive(Clone, Copy, Debug)]
pub struct Blaze2FoldedMessageOpenRequest<'a> {
    pub code: &'a PackedRaaCode,
    pub col_point: &'a [B128],
    pub folded_eval: B128,
    pub input_indices: &'a [usize],
}

#[derive(Clone, Debug)]
pub struct Blaze2FoldedMessageProof<B: Blaze2FoldedMessageBackend> {
    pub commitment: B::Commitment,
    pub eval: B128,
    pub backend_proof: B::Proof,
}

#[derive(Clone, Debug)]
pub struct Blaze2OpeningProof<H: Hash, B: Blaze2FoldedMessageBackend> {
    pub row_evals: Vec<B128>,
    pub folded_message: Blaze2FoldedMessageProof<B>,
    pub queries: Vec<Blaze2OpeningQuery<H>>,
}

pub trait Blaze2FoldedMessageBackend {
    type Commitment: Clone + std::fmt::Debug + PartialEq + Eq;
    type ProverState;
    type Proof: Clone + std::fmt::Debug + PartialEq + Eq;

    fn commit(message: &[B128]) -> Result<(Self::Commitment, Self::ProverState), Error>;

    fn absorb_commitment<H: Hash, S>(
        transcript: &mut CfriTranscript<H, S>,
        commitment: &Self::Commitment,
    );

    fn open(
        state: &Self::ProverState,
        request: &Blaze2FoldedMessageOpenRequest<'_>,
    ) -> Result<Self::Proof, Error>;

    fn verify(
        commitment: &Self::Commitment,
        proof: &Self::Proof,
        message_len: usize,
        request: &Blaze2FoldedMessageOpenRequest<'_>,
        input_values: &[B128],
    ) -> Result<(), Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blaze2RaaTrace {
    pub u2: Vec<B128>,
    pub u3: Vec<B128>,
    pub u4: Vec<B128>,
    pub u5: Vec<B128>,
}

impl Blaze2Code {
    pub fn new(spec: Blaze2CodeSpec) -> Result<Self, Error> {
        validate_blaze2_code_spec(&spec)?;
        let mut rng = ChaCha8Rng::from_seed(spec.seed.0);
        let packed =
            PackedRaaCode::new(spec.praa_message_len, spec.praa_expansion_factor, &mut rng);
        debug_assert_eq!(packed.codeword_len(), spec.praa_codeword_len);
        Ok(Self { spec, packed })
    }

    pub fn spec(&self) -> &Blaze2CodeSpec {
        &self.spec
    }

    pub fn packed(&self) -> &PackedRaaCode {
        &self.packed
    }
}

pub fn absorb_blaze2_code_spec<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    spec: &Blaze2CodeSpec,
) {
    transcript.absorb("blaze2-code-spec-v1");
    absorb_usize(transcript, spec.version as usize);
    absorb_usize(transcript, spec.field_id as usize);
    absorb_usize(transcript, spec.hash_id as usize);
    absorb_usize(transcript, spec.raa_variant as usize);
    absorb_usize(transcript, spec.packing as usize);
    absorb_usize(transcript, spec.leaf_layout as usize);
    absorb_usize(transcript, spec.praa_message_len);
    absorb_usize(transcript, spec.praa_expansion_factor);
    absorb_usize(transcript, spec.praa_codeword_len);
    transcript.absorb(&spec.seed.0);
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

impl<H: Hash> PartialEq for Blaze2InterleavedColumnQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == other.values && self.path == other.path
    }
}

impl<H: Hash> Eq for Blaze2InterleavedColumnQuery<H> {}

impl<H: Hash> PartialEq for Blaze2RaaAuxTraceQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == other.values && self.path == other.path
    }
}

impl<H: Hash> Eq for Blaze2RaaAuxTraceQuery<H> {}

impl<H: Hash> PartialEq for Blaze2RaaAuxTraceSpotQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.queries == other.queries
    }
}

impl<H: Hash> Eq for Blaze2RaaAuxTraceSpotQuery<H> {}

impl<B: Blaze2FoldedMessageBackend> PartialEq for Blaze2FoldedMessageProof<B> {
    fn eq(&self, other: &Self) -> bool {
        self.commitment == other.commitment
            && self.eval == other.eval
            && self.backend_proof == other.backend_proof
    }
}

impl<B: Blaze2FoldedMessageBackend> Eq for Blaze2FoldedMessageProof<B> {}

impl<H: Hash> PartialEq for Blaze2OpeningQuery<H> {
    fn eq(&self, other: &Self) -> bool {
        self.column_opening == other.column_opening
    }
}

impl<H: Hash> Eq for Blaze2OpeningQuery<H> {}

impl<H: Hash, B: Blaze2FoldedMessageBackend> PartialEq for Blaze2OpeningProof<H, B> {
    fn eq(&self, other: &Self) -> bool {
        self.row_evals == other.row_evals
            && self.folded_message == other.folded_message
            && self.queries == other.queries
    }
}

impl<H: Hash, B: Blaze2FoldedMessageBackend> Eq for Blaze2OpeningProof<H, B> {}

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

impl<H: Hash> Blaze2InterleavedCodewordCommitment<H> {
    pub fn commit_codeword_rows(rows: &[Vec<B128>]) -> Result<Self, Error> {
        validate_b128_rows(rows)?;
        let codeword_rows = rows.to_vec();
        let merkle_tree = merkelize_b128_columns::<H>(&codeword_rows);
        Ok(Self {
            codeword_rows,
            merkle_tree,
        })
    }

    pub fn public(&self) -> Blaze2InterleavedCodewordPublicCommitment<H> {
        Blaze2InterleavedCodewordPublicCommitment {
            root: self.root().clone(),
            codeword_len: self.codeword_len(),
            num_rows: self.num_rows(),
        }
    }

    pub fn codeword_rows(&self) -> &[Vec<B128>] {
        &self.codeword_rows
    }

    pub fn root(&self) -> &Output<H> {
        &self.merkle_tree[self.merkle_tree.len() - 1][0]
    }

    pub fn codeword_len(&self) -> usize {
        self.codeword_rows[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.codeword_rows.len()
    }

    pub fn query(&self, index: usize) -> Result<Blaze2InterleavedColumnQuery<H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        let mut values = Vec::with_capacity(self.num_rows());
        for row in &self.codeword_rows {
            values.push(row[index]);
        }
        Ok(Blaze2InterleavedColumnQuery {
            index,
            values,
            path: merkle_column_sibling_path::<H>(&self.merkle_tree, index),
        })
    }
}

impl<H: Hash> Blaze2RaaAuxTraceCommitment<H> {
    pub fn commit_trace(trace: &Blaze2RaaAuxTrace) -> Result<Self, Error> {
        let codeword_len = validate_raa_aux_trace_commitment_shape(trace)?;
        let trace_rows = vec![trace.u2.clone(), trace.u3.clone(), trace.u4.clone()];
        let merkle_tree = merkelize_b128_columns::<H>(&trace_rows);
        debug_assert_eq!(trace_rows[0].len(), codeword_len);
        Ok(Self {
            trace_rows,
            merkle_tree,
        })
    }

    pub fn public(&self) -> Blaze2RaaAuxTracePublicCommitment<H> {
        Blaze2RaaAuxTracePublicCommitment {
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

    pub fn query(&self, index: usize) -> Result<Blaze2RaaAuxTraceQuery<H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        let mut values = Vec::with_capacity(self.num_rows());
        for row in &self.trace_rows {
            values.push(row[index]);
        }
        Ok(Blaze2RaaAuxTraceQuery {
            index,
            values,
            path: merkle_column_sibling_path::<H>(&self.merkle_tree, index),
        })
    }

    pub fn spot_query(
        &self,
        code: &PackedRaaCode,
        index: usize,
    ) -> Result<Blaze2RaaAuxTraceSpotQuery<H>, Error> {
        validate_query_index(index, self.codeword_len())?;
        if code.codeword_len() != self.codeword_len() {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 RAA auxiliary trace commitment length does not match code".to_string(),
            ));
        }

        let permutation = code.permutation();
        let mut opened_indices = Vec::with_capacity(3);
        push_unique_index(&mut opened_indices, index);
        if index > 0 {
            push_unique_index(&mut opened_indices, index - 1);
        }
        push_unique_index(&mut opened_indices, permutation.permutation2[index]);

        let mut queries = Vec::with_capacity(opened_indices.len());
        for opened_index in opened_indices {
            queries.push(self.query(opened_index)?);
        }
        Ok(Blaze2RaaAuxTraceSpotQuery { index, queries })
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

impl<H: Hash> Blaze2InterleavedCodewordPublicCommitment<H> {
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

impl<H: Hash> Blaze2RaaAuxTracePublicCommitment<H> {
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

impl<H: Hash> AsRef<Output<H>> for Blaze2InterleavedCodewordCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2InterleavedCodewordCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2InterleavedCodewordPublicCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2InterleavedCodewordPublicCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2RaaAuxTraceCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2RaaAuxTraceCommitment<H> {
    fn as_ref(&self) -> &[Output<H>] {
        slice::from_ref(self.root())
    }
}

impl<H: Hash> AsRef<Output<H>> for Blaze2RaaAuxTracePublicCommitment<H> {
    fn as_ref(&self) -> &Output<H> {
        self.root()
    }
}

impl<H: Hash> AsRef<[Output<H>]> for Blaze2RaaAuxTracePublicCommitment<H> {
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

impl<H: Hash> Blaze2InterleavedColumnQuery<H> {
    pub fn authenticate(
        &self,
        root: &Output<H>,
        num_rows: usize,
        codeword_len: usize,
    ) -> Result<(), Error> {
        authenticate_b128_column_query::<H>(self, root, num_rows, codeword_len)
    }
}

impl<H: Hash> Blaze2RaaAuxTraceQuery<H> {
    pub fn authenticate(
        &self,
        root: &Output<H>,
        num_rows: usize,
        codeword_len: usize,
    ) -> Result<(), Error> {
        authenticate_raa_aux_trace_query::<H>(self, root, num_rows, codeword_len)
    }
}

impl<H: Hash> Blaze2RaaAuxTraceSpotQuery<H> {
    pub fn verify(
        &self,
        code: &PackedRaaCode,
        message: &[B128],
        aux_root: &Output<H>,
        column_root: &Output<H>,
        column_queries: &[Blaze2InterleavedColumnQuery<H>],
        challenges: &[B128],
    ) -> Result<(), Error> {
        verify_raa_aux_trace_spot_query(
            code,
            message,
            aux_root,
            self,
            column_root,
            column_queries,
            challenges,
        )
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

pub fn evaluate_packed_matrix_at_point_into(
    packed_rows: &[Vec<B128>],
    row_point: &[B128],
    col_point: &[B128],
    row_evals: &mut [B128],
    scratch: &mut [B128],
) -> Result<B128, Error> {
    let row_len = validate_packed_rows(packed_rows)?;
    let num_rows = packed_rows.len();
    if !num_rows.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening expects a power-of-two number of packed rows".to_string(),
        ));
    }
    if row_point.len() != log2_strict(num_rows) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening row point has {} coordinates for {num_rows} packed rows",
            row_point.len()
        )));
    }
    if row_evals.len() != num_rows {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening row evaluation output has incompatible length".to_string(),
        ));
    }
    let scratch_len = row_len.max(num_rows);
    if scratch.len() != scratch_len {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening scratch has {} entries but needs {scratch_len}",
            scratch.len()
        )));
    }

    evaluate_packed_rows_at_point_into(packed_rows, col_point, row_evals, &mut scratch[..row_len])?;
    evaluate_multilinear(row_evals, row_point, &mut scratch[..num_rows])
}

pub fn evaluate_packed_matrix_at_point(
    packed_rows: &[Vec<B128>],
    row_point: &[B128],
    col_point: &[B128],
) -> Result<B128, Error> {
    let row_len = validate_packed_rows(packed_rows)?;
    let num_rows = packed_rows.len();
    let mut row_evals = vec![B128::ZERO; num_rows];
    let mut scratch = vec![B128::ZERO; row_len.max(num_rows)];
    evaluate_packed_matrix_at_point_into(
        packed_rows,
        row_point,
        col_point,
        &mut row_evals,
        &mut scratch,
    )
}

pub fn absorb_blaze2_opening_public<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    code: &PackedRaaCode,
    code_seed: &Blaze2CodeSeed,
    commitment: &Blaze2InterleavedCodewordPublicCommitment<H>,
    claim: &Blaze2OpeningClaim,
    num_queries: usize,
) {
    transcript.absorb("cfri-blaze2-opening-v1");
    absorb_packed_raa_code(transcript, code, code_seed);
    absorb_usize(transcript, num_queries);
    absorb_usize(transcript, commitment.codeword_len());
    absorb_usize(transcript, commitment.num_rows());
    transcript.absorb(commitment.root());
    transcript.absorb_slice(&claim.row_point);
    transcript.absorb_slice(&claim.col_point);
    transcript.absorb(&claim.value);
}

pub fn absorb_blaze2_opening_public_with_code_spec<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    code: &Blaze2Code,
    commitment: &Blaze2InterleavedCodewordPublicCommitment<H>,
    claim: &Blaze2OpeningClaim,
    num_queries: usize,
) {
    transcript.absorb("cfri-blaze2-opening-code-spec-v1");
    absorb_blaze2_code_spec(transcript, code.spec());
    absorb_usize(transcript, num_queries);
    absorb_usize(transcript, commitment.codeword_len());
    absorb_usize(transcript, commitment.num_rows());
    transcript.absorb(commitment.root());
    transcript.absorb_slice(&claim.row_point);
    transcript.absorb_slice(&claim.col_point);
    transcript.absorb(&claim.value);
}

pub fn absorb_blaze2_opening_row_evals<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    row_evals: &[B128],
) {
    transcript.absorb("blaze2-opening-row-evals");
    transcript.absorb_slice(row_evals);
}

pub fn squeeze_blaze2_opening_folding_challenges<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    out: &mut [B128],
) {
    transcript.absorb("blaze2-opening-folding-challenges");
    transcript.squeeze_into(out);
}

pub fn absorb_blaze2_opening_folded_eval<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    folded_eval: &B128,
) {
    transcript.absorb("blaze2-opening-folded-eval");
    transcript.absorb(folded_eval);
}

pub fn squeeze_blaze2_opening_query_indices<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    codeword_len: usize,
    out: &mut [usize],
) -> Result<(), Error> {
    validate_query_index(0, codeword_len)?;
    if out.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening transcript expects at least one query".to_string(),
        ));
    }

    transcript.absorb("blaze2-opening-query-indices");
    for index in out {
        *index = squeeze_query_index(transcript, codeword_len);
    }
    Ok(())
}

pub fn prove_blaze2_opening<H: Hash, B: Blaze2FoldedMessageBackend>(
    code: &PackedRaaCode,
    code_seed: &Blaze2CodeSeed,
    packed_rows: &[Vec<B128>],
    codeword_commitment: &Blaze2InterleavedCodewordCommitment<H>,
    claim: &Blaze2OpeningClaim,
    num_queries: usize,
) -> Result<Blaze2OpeningProof<H, B>, Error> {
    validate_blaze2_opening_inputs(
        code,
        packed_rows,
        codeword_commitment.codeword_len(),
        codeword_commitment.num_rows(),
        claim,
    )?;
    if num_queries == 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening proof expects at least one query".to_string(),
        ));
    }

    let row_len = code.message_len();
    let num_rows = packed_rows.len();
    let mut row_evals = vec![B128::ZERO; num_rows];
    let mut eval_scratch = vec![B128::ZERO; row_len.max(num_rows)];
    let value = evaluate_packed_matrix_at_point_into(
        packed_rows,
        &claim.row_point,
        &claim.col_point,
        &mut row_evals,
        &mut eval_scratch,
    )?;
    if value != claim.value {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening claim does not match witness rows".to_string(),
        ));
    }

    let public_commitment = codeword_commitment.public();
    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_opening_public(
        &mut transcript,
        code,
        code_seed,
        &public_commitment,
        claim,
        num_queries,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &row_evals);

    let mut folding_challenges = vec![B128::ZERO; num_rows];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);

    let mut folded_message = vec![B128::ZERO; row_len];
    fold_packed_rows_into(packed_rows, &folding_challenges, &mut folded_message)?;
    let folded_eval = evaluate_multilinear(
        &folded_message,
        &claim.col_point,
        &mut eval_scratch[..row_len],
    )?;
    let expected_folded_eval = inner_product_b128(&row_evals, &folding_challenges);
    if folded_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded message evaluation does not match row-evaluation fold".to_string(),
        ));
    }
    let (folded_commitment, folded_state) = B::commit(&folded_message)?;
    B::absorb_commitment(&mut transcript, &folded_commitment);
    absorb_blaze2_opening_folded_eval(&mut transcript, &folded_eval);

    let mut query_indices = vec![0usize; num_queries];
    squeeze_blaze2_opening_query_indices(&mut transcript, code.codeword_len(), &mut query_indices)?;

    let mut queries = Vec::with_capacity(query_indices.len());
    for &index in &query_indices {
        validate_query_index(index, code.codeword_len())?;
        queries.push(Blaze2OpeningQuery {
            column_opening: codeword_commitment.query(index)?,
        });
    }
    let folded_request = Blaze2FoldedMessageOpenRequest {
        code,
        col_point: &claim.col_point,
        folded_eval,
        input_indices: &query_indices,
    };
    let backend_proof = B::open(&folded_state, &folded_request)?;

    Ok(Blaze2OpeningProof {
        row_evals,
        folded_message: Blaze2FoldedMessageProof {
            commitment: folded_commitment,
            eval: folded_eval,
            backend_proof,
        },
        queries,
    })
}

pub fn prove_blaze2_opening_with_code_spec<H: Blaze2HashSpec, B: Blaze2FoldedMessageBackend>(
    code: &Blaze2Code,
    packed_rows: &[Vec<B128>],
    codeword_commitment: &Blaze2InterleavedCodewordCommitment<H>,
    claim: &Blaze2OpeningClaim,
    num_queries: usize,
) -> Result<Blaze2OpeningProof<H, B>, Error> {
    validate_blaze2_code_hash::<H>(code.spec())?;
    let packed_code = code.packed();
    validate_blaze2_opening_inputs(
        packed_code,
        packed_rows,
        codeword_commitment.codeword_len(),
        codeword_commitment.num_rows(),
        claim,
    )?;
    if num_queries == 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening proof expects at least one query".to_string(),
        ));
    }

    let row_len = packed_code.message_len();
    let num_rows = packed_rows.len();
    let mut row_evals = vec![B128::ZERO; num_rows];
    let mut eval_scratch = vec![B128::ZERO; row_len.max(num_rows)];
    let value = evaluate_packed_matrix_at_point_into(
        packed_rows,
        &claim.row_point,
        &claim.col_point,
        &mut row_evals,
        &mut eval_scratch,
    )?;
    if value != claim.value {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening claim does not match witness rows".to_string(),
        ));
    }

    let public_commitment = codeword_commitment.public();
    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_opening_public_with_code_spec(
        &mut transcript,
        code,
        &public_commitment,
        claim,
        num_queries,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &row_evals);

    let mut folding_challenges = vec![B128::ZERO; num_rows];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);

    let mut folded_message = vec![B128::ZERO; row_len];
    fold_packed_rows_into(packed_rows, &folding_challenges, &mut folded_message)?;
    let folded_eval = evaluate_multilinear(
        &folded_message,
        &claim.col_point,
        &mut eval_scratch[..row_len],
    )?;
    let expected_folded_eval = inner_product_b128(&row_evals, &folding_challenges);
    if folded_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded message evaluation does not match row-evaluation fold".to_string(),
        ));
    }
    let (folded_commitment, folded_state) = B::commit(&folded_message)?;
    B::absorb_commitment(&mut transcript, &folded_commitment);
    absorb_blaze2_opening_folded_eval(&mut transcript, &folded_eval);

    let mut query_indices = vec![0usize; num_queries];
    squeeze_blaze2_opening_query_indices(
        &mut transcript,
        packed_code.codeword_len(),
        &mut query_indices,
    )?;

    let mut queries = Vec::with_capacity(query_indices.len());
    for &index in &query_indices {
        validate_query_index(index, packed_code.codeword_len())?;
        queries.push(Blaze2OpeningQuery {
            column_opening: codeword_commitment.query(index)?,
        });
    }
    let folded_request = Blaze2FoldedMessageOpenRequest {
        code: packed_code,
        col_point: &claim.col_point,
        folded_eval,
        input_indices: &query_indices,
    };
    let backend_proof = B::open(&folded_state, &folded_request)?;

    Ok(Blaze2OpeningProof {
        row_evals,
        folded_message: Blaze2FoldedMessageProof {
            commitment: folded_commitment,
            eval: folded_eval,
            backend_proof,
        },
        queries,
    })
}

pub fn verify_blaze2_opening<H: Hash, B: Blaze2FoldedMessageBackend>(
    code: &PackedRaaCode,
    code_seed: &Blaze2CodeSeed,
    commitment: &Blaze2InterleavedCodewordPublicCommitment<H>,
    claim: &Blaze2OpeningClaim,
    proof: &Blaze2OpeningProof<H, B>,
    num_queries: usize,
) -> Result<(), Error> {
    validate_blaze2_opening_public(
        code,
        commitment.codeword_len(),
        commitment.num_rows(),
        claim,
        proof,
        num_queries,
    )?;

    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_opening_public(
        &mut transcript,
        code,
        code_seed,
        commitment,
        claim,
        num_queries,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &proof.row_evals);
    let mut folding_challenges = vec![B128::ZERO; proof.row_evals.len()];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);
    let expected_folded_eval = inner_product_b128(&proof.row_evals, &folding_challenges);
    if proof.folded_message.eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded message evaluation does not match row-evaluation fold".to_string(),
        ));
    }
    B::absorb_commitment(&mut transcript, &proof.folded_message.commitment);
    absorb_blaze2_opening_folded_eval(&mut transcript, &proof.folded_message.eval);
    let mut query_indices = vec![0usize; num_queries];
    squeeze_blaze2_opening_query_indices(&mut transcript, code.codeword_len(), &mut query_indices)?;

    let mut scratch = vec![B128::ZERO; proof.row_evals.len()];
    let claim_value = evaluate_multilinear(&proof.row_evals, &claim.row_point, &mut scratch)?;
    if claim_value != claim.value {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening row evaluation invariant failed".to_string(),
        ));
    }

    let mut input_values = Vec::with_capacity(query_indices.len());
    for (query, expected_index) in proof.queries.iter().zip(&query_indices) {
        authenticate_blaze2_opening_column(
            commitment.root(),
            &query.column_opening,
            commitment.num_rows(),
            code.codeword_len(),
            *expected_index,
        )?;
        input_values.push(fold_interleaved_column(
            &query.column_opening,
            &folding_challenges,
        )?);
    }

    let folded_request = Blaze2FoldedMessageOpenRequest {
        code,
        col_point: &claim.col_point,
        folded_eval: proof.folded_message.eval,
        input_indices: &query_indices,
    };
    B::verify(
        &proof.folded_message.commitment,
        &proof.folded_message.backend_proof,
        code.message_len(),
        &folded_request,
        &input_values,
    )?;
    Ok(())
}

pub fn verify_blaze2_opening_with_code_spec<H: Blaze2HashSpec, B: Blaze2FoldedMessageBackend>(
    code: &Blaze2Code,
    commitment: &Blaze2InterleavedCodewordPublicCommitment<H>,
    claim: &Blaze2OpeningClaim,
    proof: &Blaze2OpeningProof<H, B>,
    num_queries: usize,
) -> Result<(), Error> {
    validate_blaze2_code_hash::<H>(code.spec())?;
    let packed_code = code.packed();
    validate_blaze2_opening_public(
        packed_code,
        commitment.codeword_len(),
        commitment.num_rows(),
        claim,
        proof,
        num_queries,
    )?;

    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_opening_public_with_code_spec(
        &mut transcript,
        code,
        commitment,
        claim,
        num_queries,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &proof.row_evals);
    let mut folding_challenges = vec![B128::ZERO; proof.row_evals.len()];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);
    let expected_folded_eval = inner_product_b128(&proof.row_evals, &folding_challenges);
    if proof.folded_message.eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 folded message evaluation does not match row-evaluation fold".to_string(),
        ));
    }
    B::absorb_commitment(&mut transcript, &proof.folded_message.commitment);
    absorb_blaze2_opening_folded_eval(&mut transcript, &proof.folded_message.eval);
    let mut query_indices = vec![0usize; num_queries];
    squeeze_blaze2_opening_query_indices(
        &mut transcript,
        packed_code.codeword_len(),
        &mut query_indices,
    )?;

    let mut scratch = vec![B128::ZERO; proof.row_evals.len()];
    let claim_value = evaluate_multilinear(&proof.row_evals, &claim.row_point, &mut scratch)?;
    if claim_value != claim.value {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening row evaluation invariant failed".to_string(),
        ));
    }

    let mut input_values = Vec::with_capacity(query_indices.len());
    for (query, expected_index) in proof.queries.iter().zip(&query_indices) {
        authenticate_blaze2_opening_column(
            commitment.root(),
            &query.column_opening,
            commitment.num_rows(),
            packed_code.codeword_len(),
            *expected_index,
        )?;
        input_values.push(fold_interleaved_column(
            &query.column_opening,
            &folding_challenges,
        )?);
    }

    let folded_request = Blaze2FoldedMessageOpenRequest {
        code: packed_code,
        col_point: &claim.col_point,
        folded_eval: proof.folded_message.eval,
        input_indices: &query_indices,
    };
    B::verify(
        &proof.folded_message.commitment,
        &proof.folded_message.backend_proof,
        packed_code.message_len(),
        &folded_request,
        &input_values,
    )?;
    Ok(())
}

pub fn build_raa_aux_trace(
    code: &PackedRaaCode,
    message: &[B128],
) -> Result<Blaze2RaaAuxTrace, Error> {
    let len = validate_raa_message(code, message)?;
    let mut trace = Blaze2RaaAuxTrace {
        u2: vec![B128::ZERO; len],
        u3: vec![B128::ZERO; len],
        u4: vec![B128::ZERO; len],
    };
    build_raa_aux_trace_into(code, message, &mut trace)?;
    Ok(trace)
}

pub fn build_raa_aux_trace_into(
    code: &PackedRaaCode,
    message: &[B128],
    trace: &mut Blaze2RaaAuxTrace,
) -> Result<(), Error> {
    let len = validate_raa_message(code, message)?;
    validate_raa_aux_trace_shape(trace, len)?;

    let permutation = code.permutation();
    for i in 0..len {
        trace.u2[i] = message[permutation.permutation1[i] / code.rate()];
    }

    prefix_accumulate_into(&trace.u2, &mut trace.u3);

    for i in 0..len {
        trace.u4[i] = trace.u3[permutation.permutation2[i]];
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

pub fn verify_raa_aux_trace_spot_query<H: Hash>(
    code: &PackedRaaCode,
    message: &[B128],
    aux_root: &Output<H>,
    aux_opening: &Blaze2RaaAuxTraceSpotQuery<H>,
    column_root: &Output<H>,
    column_queries: &[Blaze2InterleavedColumnQuery<H>],
    challenges: &[B128],
) -> Result<(), Error> {
    let len = validate_raa_message(code, message)?;
    validate_query_index(aux_opening.index, len)?;
    authenticate_aux_trace_spot_queries(aux_root, aux_opening, len)?;
    authenticate_column_queries(column_root, column_queries, challenges.len(), len)?;

    let permutation = code.permutation();
    let index = aux_opening.index;
    let expected_u2 = message[permutation.permutation1[index] / code.rate()];
    let u2 = aux_trace_spot_value(aux_opening, RAA_AUX_TRACE_ROW_U2, index)?;
    if u2 != expected_u2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary spot check failed repetition/permutation layer".to_string(),
        ));
    }

    let u3 = aux_trace_spot_value(aux_opening, RAA_AUX_TRACE_ROW_U3, index)?;
    let expected_u3 = if index == 0 {
        u2
    } else {
        aux_trace_spot_value(aux_opening, RAA_AUX_TRACE_ROW_U3, index - 1)? + u2
    };
    if u3 != expected_u3 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary spot check failed first accumulator layer".to_string(),
        ));
    }

    let u4 = aux_trace_spot_value(aux_opening, RAA_AUX_TRACE_ROW_U4, index)?;
    let permuted_u3 = aux_trace_spot_value(
        aux_opening,
        RAA_AUX_TRACE_ROW_U3,
        permutation.permutation2[index],
    )?;
    if u4 != permuted_u3 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary spot check failed second permutation layer".to_string(),
        ));
    }

    let current_codeword = folded_column_value(column_queries, challenges, index)?;
    let expected_codeword = if index == 0 {
        u4
    } else {
        folded_column_value(column_queries, challenges, index - 1)? + u4
    };
    if current_codeword != expected_codeword {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary spot check failed final accumulator link".to_string(),
        ));
    }

    Ok(())
}

pub fn check_raa_folded_codeword_link<F: BlazeField, H: Hash>(
    row_query: &Blaze2RaaQuery<F, H>,
    challenges: &[B128],
    trace_opening: &Blaze2RaaTraceSpotQuery<H>,
) -> Result<(), Error> {
    if row_query.index & !1 != trace_opening.index & !1 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA folded codeword link uses different query pairs".to_string(),
        ));
    }

    let pair_start = row_query.index & !1;
    let folded_pair = fold_packed_query_pair(&row_query.values, challenges)?;
    let trace_pair = (
        trace_spot_value(trace_opening, RAA_TRACE_ROW_U5, pair_start)?,
        trace_spot_value(trace_opening, RAA_TRACE_ROW_U5, pair_start + 1)?,
    );
    if trace_pair != folded_pair {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA folded codeword link failed".to_string(),
        ));
    }
    Ok(())
}

pub fn fold_interleaved_column<H: Hash>(
    query: &Blaze2InterleavedColumnQuery<H>,
    challenges: &[B128],
) -> Result<B128, Error> {
    if query.values.len() != challenges.len() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaved column does not match fold width".to_string(),
        ));
    }
    Ok(inner_product_b128(&query.values, challenges))
}

const RAA_AUX_TRACE_NUM_ROWS: usize = 3;
const RAA_AUX_TRACE_ROW_U2: usize = 0;
const RAA_AUX_TRACE_ROW_U3: usize = 1;
const RAA_AUX_TRACE_ROW_U4: usize = 2;

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

fn validate_b128_rows(rows: &[Vec<B128>]) -> Result<usize, Error> {
    if rows.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 B128 row commitment expects at least one row".to_string(),
        ));
    }
    let row_len = rows[0].len();
    validate_query_index(0, row_len)?;
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 B128 rows have incompatible lengths".to_string(),
        ));
    }
    Ok(row_len)
}

fn validate_blaze2_opening_inputs(
    code: &PackedRaaCode,
    packed_rows: &[Vec<B128>],
    codeword_len: usize,
    num_committed_rows: usize,
    claim: &Blaze2OpeningClaim,
) -> Result<(), Error> {
    let row_len = validate_packed_rows(packed_rows)?;
    validate_blaze2_opening_shape(
        code,
        row_len,
        packed_rows.len(),
        codeword_len,
        num_committed_rows,
        claim,
    )
}

fn validate_blaze2_opening_public<H: Hash, B: Blaze2FoldedMessageBackend>(
    code: &PackedRaaCode,
    codeword_len: usize,
    num_committed_rows: usize,
    claim: &Blaze2OpeningClaim,
    proof: &Blaze2OpeningProof<H, B>,
    num_queries: usize,
) -> Result<(), Error> {
    if proof.queries.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening proof has no queries".to_string(),
        ));
    }
    if proof.queries.len() != num_queries {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening proof has {} queries but verifier expects {num_queries}",
            proof.queries.len()
        )));
    }
    validate_blaze2_opening_shape(
        code,
        code.message_len(),
        proof.row_evals.len(),
        codeword_len,
        num_committed_rows,
        claim,
    )
}

fn validate_blaze2_opening_shape(
    code: &PackedRaaCode,
    row_len: usize,
    num_rows: usize,
    codeword_len: usize,
    num_committed_rows: usize,
    claim: &Blaze2OpeningClaim,
) -> Result<(), Error> {
    if row_len != code.message_len() {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening message row has {row_len} entries but code expects {}",
            code.message_len()
        )));
    }
    if codeword_len != code.codeword_len() {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening commitment has codeword length {codeword_len} but code expects {}",
            code.codeword_len()
        )));
    }
    if num_rows == 0 || !num_rows.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening expects a non-empty power-of-two number of packed rows".to_string(),
        ));
    }
    if num_committed_rows != num_rows {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening commitment has {num_committed_rows} rows but proof uses {num_rows}"
        )));
    }
    if claim.row_point.len() != log2_strict(num_rows) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening row point has {} coordinates for {num_rows} packed rows",
            claim.row_point.len()
        )));
    }
    if claim.col_point.len() != log2_strict(row_len) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 opening column point has {} coordinates for {row_len} message columns",
            claim.col_point.len()
        )));
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

fn validate_raa_aux_trace_commitment_shape(trace: &Blaze2RaaAuxTrace) -> Result<usize, Error> {
    let len = trace.u2.len();
    validate_query_index(0, len)?;
    validate_raa_aux_trace_shape(trace, len)?;
    Ok(len)
}

fn validate_raa_aux_trace_shape(trace: &Blaze2RaaAuxTrace, len: usize) -> Result<(), Error> {
    if trace.u2.len() != len || trace.u3.len() != len || trace.u4.len() != len {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary trace has incompatible word lengths".to_string(),
        ));
    }
    Ok(())
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

fn authenticate_aux_trace_spot_queries<H: Hash>(
    root: &Output<H>,
    opening: &Blaze2RaaAuxTraceSpotQuery<H>,
    codeword_len: usize,
) -> Result<(), Error> {
    if opening.queries.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary spot check has no openings".to_string(),
        ));
    }

    let mut seen_indices = Vec::with_capacity(opening.queries.len());
    for query in &opening.queries {
        query.authenticate(root, RAA_AUX_TRACE_NUM_ROWS, codeword_len)?;
        if seen_indices.contains(&query.index) {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 RAA auxiliary spot check repeats an opening".to_string(),
            ));
        }
        seen_indices.push(query.index);
    }
    Ok(())
}

fn authenticate_column_queries<H: Hash>(
    root: &Output<H>,
    queries: &[Blaze2InterleavedColumnQuery<H>],
    num_rows: usize,
    codeword_len: usize,
) -> Result<(), Error> {
    if queries.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaved column check has no openings".to_string(),
        ));
    }

    let mut seen_indices = Vec::with_capacity(queries.len());
    for query in queries {
        query.authenticate(root, num_rows, codeword_len)?;
        if seen_indices.contains(&query.index) {
            return Err(Error::InvalidPcsOpen(
                "Blaze2 interleaved column check repeats an opening".to_string(),
            ));
        }
        seen_indices.push(query.index);
    }
    Ok(())
}

fn authenticate_blaze2_opening_column<H: Hash>(
    root: &Output<H>,
    query: &Blaze2InterleavedColumnQuery<H>,
    num_rows: usize,
    codeword_len: usize,
    expected_index: usize,
) -> Result<(), Error> {
    if query.index != expected_index {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 opening column index does not match transcript".to_string(),
        ));
    }
    query.authenticate(root, num_rows, codeword_len)
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

fn aux_trace_spot_value<H: Hash>(
    opening: &Blaze2RaaAuxTraceSpotQuery<H>,
    row: usize,
    index: usize,
) -> Result<B128, Error> {
    for query in &opening.queries {
        if query.index == index {
            if row >= query.values.len() {
                return Err(Error::InvalidPcsOpen(
                    "Blaze2 RAA auxiliary trace query has too few values".to_string(),
                ));
            }
            return Ok(query.values[row]);
        }
    }

    Err(Error::InvalidPcsOpen(
        "Blaze2 RAA auxiliary spot check is missing a required opening".to_string(),
    ))
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

fn folded_column_value<H: Hash>(
    column_queries: &[Blaze2InterleavedColumnQuery<H>],
    challenges: &[B128],
    index: usize,
) -> Result<B128, Error> {
    for query in column_queries {
        if query.index == index {
            return fold_interleaved_column(query, challenges);
        }
    }

    Err(Error::InvalidPcsOpen(
        "Blaze2 RAA auxiliary spot check is missing a committed column opening".to_string(),
    ))
}

fn inner_product_b128(lhs: &[B128], rhs: &[B128]) -> B128 {
    let mut acc = B128::ZERO;
    for i in 0..lhs.len() {
        acc += lhs[i] * rhs[i];
    }
    acc
}

fn absorb_packed_raa_code<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    code: &PackedRaaCode,
    code_seed: &Blaze2CodeSeed,
) {
    transcript.absorb("packed-raa-code");
    absorb_usize(transcript, code.rate());
    absorb_usize(transcript, code.message_len());
    absorb_usize(transcript, code.codeword_len());
    transcript.absorb(&code_seed.0);
}

fn absorb_usize<H: Hash, S>(transcript: &mut CfriTranscript<H, S>, value: usize) {
    transcript.absorb(&(value as u64).to_le_bytes());
}

fn squeeze_query_index<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    codeword_len: usize,
) -> usize {
    let challenge: B128 = transcript.squeeze();
    (challenge.value[0] as usize) & (codeword_len - 1)
}

fn push_unique_pair_start(opened_pair_starts: &mut Vec<usize>, index: usize) {
    let pair_start = index & !1;
    if !opened_pair_starts.contains(&pair_start) {
        opened_pair_starts.push(pair_start);
    }
}

fn push_unique_index(opened_indices: &mut Vec<usize>, index: usize) {
    if !opened_indices.contains(&index) {
        opened_indices.push(index);
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

fn validate_blaze2_code_spec(spec: &Blaze2CodeSpec) -> Result<(), Error> {
    if spec.version == 0 {
        return Err(Error::InvalidPcsParam(
            "Blaze2 code spec version must be nonzero".to_string(),
        ));
    }
    if spec.praa_message_len == 0 || !spec.praa_message_len.is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "Blaze2 PRAA message length must be a nonzero power of two".to_string(),
        ));
    }
    if spec.praa_expansion_factor == 0 || !spec.praa_expansion_factor.is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "Blaze2 PRAA expansion factor must be a nonzero power of two".to_string(),
        ));
    }
    let expected_codeword_len = spec
        .praa_message_len
        .checked_mul(spec.praa_expansion_factor)
        .ok_or_else(|| {
            Error::InvalidPcsParam("Blaze2 PRAA codeword length overflows usize".to_string())
        })?;
    if spec.praa_codeword_len != expected_codeword_len {
        return Err(Error::InvalidPcsParam(
            "Blaze2 PRAA codeword length must equal message length times expansion factor"
                .to_string(),
        ));
    }
    if spec.praa_codeword_len < 2 || !spec.praa_codeword_len.is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "Blaze2 PRAA codeword length must be a power of two at least two".to_string(),
        ));
    }
    Ok(())
}

fn validate_blaze2_code_hash<H: Blaze2HashSpec>(spec: &Blaze2CodeSpec) -> Result<(), Error> {
    if spec.hash_id != H::BLAZE2_HASH_ID {
        return Err(Error::InvalidPcsParam(
            "Blaze2 code spec hash id does not match proof hash".to_string(),
        ));
    }
    Ok(())
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

fn merkelize_b128_columns<H: Hash>(rows: &[Vec<B128>]) -> Vec<Vec<Output<H>>> {
    let row_len = rows[0].len();
    let leaves = (0..row_len)
        .into_par_iter()
        .map(|col_idx| {
            let mut hasher = H::new();
            let mut hash = Output::<H>::default();
            for row in rows {
                hasher.update_field_element(&row[col_idx]);
            }
            hasher.finalize_into_reset(&mut hash);
            hash
        })
        .collect::<Vec<_>>();

    let mut tree = Vec::with_capacity(log2_strict(row_len) + 1);
    tree.push(leaves);
    while tree[tree.len() - 1].len() > 1 {
        let parents = tree[tree.len() - 1]
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

fn merkle_column_sibling_path<H: Hash>(
    tree: &[Vec<Output<H>>],
    mut query_index: usize,
) -> Vec<Output<H>> {
    let mut path = Vec::with_capacity(tree.len().saturating_sub(1));
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

fn authenticate_b128_column_query<H: Hash>(
    query: &Blaze2InterleavedColumnQuery<H>,
    root: &Output<H>,
    num_rows: usize,
    codeword_len: usize,
) -> Result<(), Error> {
    validate_query_index(query.index, codeword_len)?;
    if num_rows == 0 || query.values.len() != num_rows {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaved column query value count does not match row count".to_string(),
        ));
    }
    if query.path.len() != log2_strict(codeword_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 interleaved column query path has incompatible length".to_string(),
        ));
    }

    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    for value in &query.values {
        hasher.update_field_element(value);
    }
    hasher.finalize_into_reset(&mut hash);

    let mut query_index = query.index;
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
            "Blaze2 interleaved column query does not authenticate".to_string(),
        ));
    }
    Ok(())
}

fn authenticate_raa_aux_trace_query<H: Hash>(
    query: &Blaze2RaaAuxTraceQuery<H>,
    root: &Output<H>,
    num_rows: usize,
    codeword_len: usize,
) -> Result<(), Error> {
    validate_query_index(query.index, codeword_len)?;
    if num_rows == 0 || query.values.len() != num_rows {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary trace query value count does not match row count".to_string(),
        ));
    }
    if query.path.len() != log2_strict(codeword_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 RAA auxiliary trace query path has incompatible length".to_string(),
        ));
    }

    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    for value in &query.values {
        hasher.update_field_element(value);
    }
    hasher.finalize_into_reset(&mut hash);

    let mut query_index = query.index;
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
            "Blaze2 RAA auxiliary trace query does not authenticate".to_string(),
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
