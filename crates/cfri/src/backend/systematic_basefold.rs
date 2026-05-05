use crate::backend::{
    arithmetic::Field,
    binary_extension_fields::B128,
    blaze::permutation_check::ProductTree,
    blaze2::{
        absorb_blaze2_code_spec, fold_interleaved_column, raa_codeword_eval_weights, Blaze2Code,
        Blaze2CodeSpec, Blaze2InterleavedColumnQuery,
    },
    code::PackedRaaCode,
    hash::{Blake2s, Hash, Output},
    Error,
};
use crate::transcript::Transcript as CfriTranscript;
use std::{collections::BTreeMap, marker::PhantomData};

const RAA_AUX_U2_ROW: usize = 0;
const RAA_AUX_U3_ROW: usize = 1;
const RAA_AUX_U4_ROW: usize = 2;
const RAA_AUX_RELATION_ROW_COUNT: usize = 3;
const RAA_SECTION5_F1_0_ROW: usize = 3;
const RAA_SECTION5_F1_1_ROW: usize = 4;
const RAA_SECTION5_G1_0_ROW: usize = 5;
const RAA_SECTION5_G1_1_ROW: usize = 6;
const RAA_SECTION5_F2_0_ROW: usize = 7;
const RAA_SECTION5_F2_1_ROW: usize = 8;
const RAA_SECTION5_G2_0_ROW: usize = 9;
const RAA_SECTION5_G2_1_ROW: usize = 10;
const RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT: usize = 3;
const RAA_SECTION5_PERMUTATION_RESIDUAL_ROW: usize = 0;
const RAA_SECTION5_FIRST_ACCUMULATOR_RESIDUAL_ROW: usize = 1;
const RAA_SECTION5_SECOND_ACCUMULATOR_RESIDUAL_ROW: usize = 2;
const RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT: usize = 8;
const RAA_SECTION5_AUX_ROW_COUNT: usize = RAA_AUX_RELATION_ROW_COUNT;
const RAA_SECTION5_PERMUTATION_SUMCHECK_DEGREE: usize = 3;
const RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE: usize = 2;
const RAA_AUX_EVAL_BINDING_ROW_COUNT: usize = 0;
const RAA_AUX_ROW_COUNT: usize = RAA_AUX_RELATION_ROW_COUNT;

pub const BLAZE2_BASEFOLD_RELATION_AUXILIARY_ROW_COUNT: usize = RAA_AUX_RELATION_ROW_COUNT;
pub const BLAZE2_BASEFOLD_SECTION5_AUXILIARY_ROW_COUNT: usize = RAA_SECTION5_AUX_ROW_COUNT;
pub const BLAZE2_BASEFOLD_SECTION5_PERMUTATION_HELPER_ROW_COUNT: usize =
    RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT;
pub const BLAZE2_BASEFOLD_EVAL_BINDING_ROW_COUNT: usize = RAA_AUX_EVAL_BINDING_ROW_COUNT;
pub const BLAZE2_BASEFOLD_AUXILIARY_ROW_COUNT: usize = RAA_AUX_ROW_COUNT;

pub fn required_blaze2_basefold_auxiliary_oracle_len(spec: &Blaze2CodeSpec) -> usize {
    required_blaze2_basefold_relation_auxiliary_len(spec)
}

pub fn required_blaze2_basefold_relation_auxiliary_len(spec: &Blaze2CodeSpec) -> usize {
    spec.praa_codeword_len * BLAZE2_BASEFOLD_RELATION_AUXILIARY_ROW_COUNT
}

pub fn required_blaze2_basefold_section5_auxiliary_oracle_len(spec: &Blaze2CodeSpec) -> usize {
    spec.praa_codeword_len * BLAZE2_BASEFOLD_SECTION5_AUXILIARY_ROW_COUNT
}

pub fn required_blaze2_basefold_section5_permutation_helper_oracle_len(
    spec: &Blaze2CodeSpec,
) -> usize {
    spec.praa_codeword_len * BLAZE2_BASEFOLD_SECTION5_PERMUTATION_HELPER_ROW_COUNT
}

pub fn required_blaze2_basefold_auxiliary_oracle_len_for_strategy(
    spec: &Blaze2CodeSpec,
    strategy: RaaRelationProofStrategy,
) -> usize {
    spec.praa_codeword_len * raa_relation_auxiliary_row_count(strategy)
}

pub fn required_blaze2_basefold_eval_binding_len(spec: &Blaze2CodeSpec) -> usize {
    let _ = spec;
    0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodewordPart {
    Systematic,
    Parity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodewordAddress {
    pub part: CodewordPart,
    pub local_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FoldPair {
    pub left: usize,
    pub right: usize,
    pub out: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystematicFoldRule {
    Affine01,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendProofQueryDomain {
    CompilerParity,
    RelationAuxiliary,
    Section5RelationResidual,
    Section5PermutationHelper,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopQuery<T> {
    pub index: usize,
    pub value: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SystematicInputQuery {
    pub logical_index: usize,
    pub physical_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendProofQuery {
    pub domain: BackendProofQueryDomain,
    pub index: usize,
    pub physical_index: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaaFinalAccumulatorQuery {
    pub index: usize,
    pub current_input_query: usize,
    pub previous_input_query: usize,
    pub u4_auxiliary_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaaAuxiliaryRelationKind {
    U2Repetition {
        peer_u2_auxiliary_index: usize,
    },
    FirstAccumulatorStart {
        u2_auxiliary_index: usize,
    },
    FirstAccumulatorStep {
        previous_u3_auxiliary_index: usize,
        u2_auxiliary_index: usize,
    },
    SecondPermutation {
        permuted_u3_auxiliary_index: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaaAuxiliaryLocalQuery {
    pub sampled_auxiliary_ordinal: usize,
    pub main_auxiliary_index: usize,
    pub relation: RaaAuxiliaryRelationKind,
}

impl RaaAuxiliaryLocalQuery {
    fn extra_count(&self) -> usize {
        match self.relation {
            RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. } => 2,
            RaaAuxiliaryRelationKind::U2Repetition { .. }
            | RaaAuxiliaryRelationKind::FirstAccumulatorStart { .. }
            | RaaAuxiliaryRelationKind::SecondPermutation { .. } => 1,
        }
    }

    fn extra_index(&self, index: usize) -> Option<usize> {
        match (self.relation, index) {
            (
                RaaAuxiliaryRelationKind::U2Repetition {
                    peer_u2_auxiliary_index,
                },
                0,
            ) => Some(peer_u2_auxiliary_index),
            (RaaAuxiliaryRelationKind::FirstAccumulatorStart { u2_auxiliary_index }, 0) => {
                Some(u2_auxiliary_index)
            }
            (
                RaaAuxiliaryRelationKind::FirstAccumulatorStep {
                    previous_u3_auxiliary_index,
                    ..
                },
                0,
            ) => Some(previous_u3_auxiliary_index),
            (
                RaaAuxiliaryRelationKind::FirstAccumulatorStep {
                    u2_auxiliary_index, ..
                },
                1,
            ) => Some(u2_auxiliary_index),
            (
                RaaAuxiliaryRelationKind::SecondPermutation {
                    permuted_u3_auxiliary_index,
                },
                0,
            ) => Some(permuted_u3_auxiliary_index),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CompilerParityCommitment<H: Hash> {
    values: Vec<B128>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityPublicCommitment<H: Hash> {
    pub root: Output<H>,
    pub len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityQuery {
    pub logical_index: usize,
    pub value: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityQueryProof<H: Hash> {
    pub queries: Vec<CompilerParityQuery>,
    pub authentication_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug)]
pub struct AuxiliaryOracleCommitment<H: Hash> {
    values: Vec<B128>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug)]
pub struct RaaSection5PermutationHelperCommitment<H: Hash> {
    inner: AuxiliaryOracleCommitment<H>,
}

#[derive(Clone, Debug)]
pub struct RaaSection5RelationResidualCommitment<H: Hash> {
    inner: AuxiliaryOracleCommitment<H>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOraclePublicCommitment<H: Hash> {
    pub root: Output<H>,
    pub len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5PermutationHelperPublicCommitment<H: Hash> {
    pub root: Output<H>,
    pub len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationResidualPublicCommitment<H: Hash> {
    pub root: Output<H>,
    pub len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOracleQuery {
    pub logical_index: usize,
    pub value: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOracleQueryProof<H: Hash> {
    pub relation_queries: Vec<AuxiliaryOracleQuery>,
    pub raa_relation: RaaRelationProof,
    pub authentication_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5PermutationHelperQueryProof<H: Hash> {
    pub queries: Vec<AuxiliaryOracleQuery>,
    pub authentication_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationResidualQueryProof<H: Hash> {
    pub queries: Vec<AuxiliaryOracleQuery>,
    pub authentication_nodes: Vec<Output<H>>,
    pub terminal_proof: Option<RaaSection5RelationTerminalProof<H>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaSection5RelationTerminalProof<H: Hash> {
    pub rows: Vec<RaaSection5RelationTerminalRowProof<H>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaSection5RelationTerminalRowProof<H: Hash> {
    pub paths: Vec<RaaSection5RelationTerminalPath>,
    _hash_marker: PhantomData<H>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationTerminalLayerProof<H: Hash> {
    pub authentication_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationTerminalPath {
    pub top_value: B128,
    pub steps: Vec<RaaSection5RelationTerminalStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationTerminalStep {
    pub sibling_value: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaFinalAccumulatorQueryProof;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaaRelationProofStrategy {
    LocalQueries,
    Section5,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RaaRelationProof {
    LocalQueries(RaaLocalRelationProof),
    Section5(RaaSection5RelationProof),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaLocalRelationProof {
    pub final_accumulator_queries: Vec<RaaFinalAccumulatorQueryProof>,
    pub local_relation_queries: Vec<RaaAuxiliaryLocalRelationProof>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaSection5RelationProof {
    pub permutation_sumcheck: RaaRelationSumcheckProof,
    pub first_accumulator_sumcheck: RaaRelationSumcheckProof,
    pub second_accumulator_sumcheck: RaaRelationSumcheckProof,
    pub terminal_evaluations: Vec<B128>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaRelationSumcheckProof {
    pub round_polynomials: Vec<Vec<B128>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RaaRelationSumcheckCheck {
    challenges: Vec<B128>,
    initial_sum: B128,
    terminal_claim: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RaaSection5RelationChecks {
    permutation: RaaRelationSumcheckCheck,
    first_accumulator: RaaRelationSumcheckCheck,
    second_accumulator: RaaRelationSumcheckCheck,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RaaAuxiliaryLocalRelationProof {
    Equality,
    FirstAccumulatorStep { previous_value: B128 },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaaEvalSumcheckProof {
    pub round_polynomials: Vec<[B128; 3]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaaSection5RelationChallenges {
    pub alpha: B128,
    pub beta: B128,
    pub gamma: B128,
}

#[derive(Clone, Copy, Debug)]
pub struct Blaze2BaseFoldOpenRequest<'a> {
    pub col_point: &'a [B128],
    pub folded_eval: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldPrequeryPublic<H: Hash> {
    pub compiler_parity: CompilerParityPublicCommitment<H>,
    pub eval_sumcheck: Option<RaaEvalSumcheckProof>,
    pub folded_parity_layers: Vec<CompilerParityPublicCommitment<H>>,
    pub terminal_codeword: Vec<B128>,
    pub auxiliary: Option<AuxiliaryOraclePublicCommitment<H>>,
    pub section5_relation_residual: Option<RaaSection5RelationResidualPublicCommitment<H>>,
    pub section5_permutation_helper: Option<RaaSection5PermutationHelperPublicCommitment<H>>,
    pub section5_relation: Option<RaaSection5RelationProof>,
}

#[derive(Clone, Debug)]
pub struct Blaze2BaseFoldProverState<H: Hash> {
    compiler_parity: CompilerParityCommitment<H>,
    folded_parity_layers: Vec<CompilerParityCommitment<H>>,
    physical_layers: Vec<Vec<B128>>,
    fold_challenges: Vec<B128>,
    auxiliary: Option<AuxiliaryOracleCommitment<H>>,
    section5_relation_residual: Option<RaaSection5RelationResidualCommitment<H>>,
    section5_permutation_helper: Option<RaaSection5PermutationHelperCommitment<H>>,
    section5_relation: Option<RaaSection5RelationProof>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldQueryProof<H: Hash> {
    pub compiler_parity: CompilerParityQueryProof<H>,
    pub compiler_parity_folds: CompilerParityFoldQueryProof<H>,
    pub auxiliary: Option<AuxiliaryOracleQueryProof<H>>,
    pub section5_relation_residual: Option<RaaSection5RelationResidualQueryProof<H>>,
    pub section5_permutation_helper: Option<RaaSection5PermutationHelperQueryProof<H>>,
    pub authentication: BackendProofOracleAuthentication<H>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendProofOracleQuerySet {
    pub compiler_parity_queries: Vec<(usize, B128)>,
    pub compiler_parity_fold_layer_queries: Vec<Vec<(usize, B128)>>,
    pub auxiliary_queries: Vec<(usize, B128)>,
    pub section5_relation_residual_queries: Vec<(usize, B128)>,
    pub section5_permutation_helper_queries: Vec<(usize, B128)>,
}

impl BackendProofOracleQuerySet {
    pub fn compiler_parity_fold_query_count(&self) -> usize {
        self.compiler_parity_fold_layer_queries
            .iter()
            .map(Vec::len)
            .sum()
    }

    pub fn total_query_count(&self) -> usize {
        self.compiler_parity_queries.len()
            + self.compiler_parity_fold_query_count()
            + self.auxiliary_queries.len()
            + self.section5_relation_residual_queries.len()
            + self.section5_permutation_helper_queries.len()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendProofOracleAuthentication<H: Hash> {
    pub compiler_parity_layers: Vec<CompilerParityFoldLayerProof<H>>,
    pub auxiliary_nodes: Vec<Output<H>>,
    pub section5_relation_residual_nodes: Vec<Output<H>>,
    pub section5_relation_terminal_folded_layers: Vec<AuxiliaryOraclePublicCommitment<H>>,
    pub section5_relation_terminal_layer_authentication:
        Vec<RaaSection5RelationTerminalLayerProof<H>>,
    pub section5_permutation_helper_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldQueryProof<H: Hash> {
    pub paths: Vec<CompilerParityFoldPath>,
    pub layer_authentication: Vec<CompilerParityFoldLayerProof<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldLayerProof<H: Hash> {
    pub authentication_nodes: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldPath {
    pub steps: Vec<CompilerParityFoldStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldStep {
    pub sibling_value: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HolographicQuerySchedule {
    input_queries: Vec<SystematicInputQuery>,
    proof_queries: Vec<BackendProofQuery>,
    raa_relation_strategy: RaaRelationProofStrategy,
    raa_final_queries: Vec<RaaFinalAccumulatorQuery>,
    raa_auxiliary_queries: Vec<RaaAuxiliaryLocalQuery>,
    raa_auxiliary_authentication_queries: Vec<BackendProofQuery>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HolographicQueryScheduleSpec {
    pub q_raa_input: usize,
    pub q_backend_proof: usize,
    pub auxiliary_oracle_len: usize,
    pub raa_relation_strategy: RaaRelationProofStrategy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystematicFoldableCodeSpec {
    pub version: u32,
    pub compiler_message_len: usize,
    pub compiler_systematic_len: usize,
    pub compiler_parity_len: usize,
    pub compiler_codeword_len: usize,
    pub parity_expansion_factor: usize,
    pub seed: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldBackendSpec {
    pub praa: Blaze2CodeSpec,
    pub compiler_code: SystematicFoldableCodeSpec,
    pub q_raa_input: usize,
    pub q_backend_proof: usize,
    pub auxiliary_oracle_len: usize,
    pub raa_relation_strategy: RaaRelationProofStrategy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystematicAugmentedRfcLayout {
    spec: SystematicFoldableCodeSpec,
    num_rounds: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystematicAugmentedRfcCode {
    layout: SystematicAugmentedRfcLayout,
    parity_fold_table: Vec<Vec<B128>>,
}

#[derive(Clone, Debug)]
pub struct Blaze2BaseFoldBackendParams {
    spec: Blaze2BaseFoldBackendSpec,
    praa: Blaze2Code,
    compiler_code: SystematicAugmentedRfcCode,
}

impl SystematicAugmentedRfcLayout {
    pub fn new(spec: SystematicFoldableCodeSpec) -> Result<Self, Error> {
        validate_spec(&spec)?;
        Ok(Self {
            num_rounds: log2_strict(spec.compiler_message_len),
            spec,
        })
    }

    pub fn spec(&self) -> &SystematicFoldableCodeSpec {
        &self.spec
    }

    pub fn message_len(&self) -> usize {
        self.spec.compiler_message_len
    }

    pub fn systematic_len(&self) -> usize {
        self.spec.compiler_systematic_len
    }

    pub fn parity_len(&self) -> usize {
        self.spec.compiler_parity_len
    }

    pub fn codeword_len(&self) -> usize {
        self.spec.compiler_codeword_len
    }

    pub fn parity_expansion_factor(&self) -> usize {
        self.spec.parity_expansion_factor
    }

    pub fn num_rounds(&self) -> usize {
        self.num_rounds
    }

    pub fn systematic_fold_rule(&self) -> SystematicFoldRule {
        SystematicFoldRule::Affine01
    }

    pub fn systematic_to_physical(&self, logical_index: usize) -> Result<usize, Error> {
        if logical_index >= self.systematic_len() {
            return Err(Error::InvalidPcsParam(format!(
                "systematic logical index {logical_index} is outside length {}",
                self.systematic_len()
            )));
        }
        Ok(systematic_to_physical(
            logical_index,
            self.message_len(),
            self.parity_expansion_factor(),
        ))
    }

    pub fn parity_to_physical(&self, logical_index: usize) -> Result<usize, Error> {
        if logical_index >= self.parity_len() {
            return Err(Error::InvalidPcsParam(format!(
                "parity logical index {logical_index} is outside length {}",
                self.parity_len()
            )));
        }
        Ok(parity_to_physical(
            logical_index,
            self.message_len(),
            self.parity_expansion_factor(),
        ))
    }

    pub fn physical_to_logical(&self, physical_index: usize) -> Result<CodewordAddress, Error> {
        self.physical_to_logical_at_round(0, physical_index)
    }

    pub fn physical_to_logical_at_round(
        &self,
        round: usize,
        physical_index: usize,
    ) -> Result<CodewordAddress, Error> {
        let message_len = self.message_len_at_round(round)?;
        let codeword_len = message_len * (self.parity_expansion_factor() + 1);
        if physical_index >= codeword_len {
            return Err(Error::InvalidPcsParam(format!(
                "physical index {physical_index} is outside round {round} codeword length {codeword_len}"
            )));
        }
        Ok(physical_to_logical(
            physical_index,
            message_len,
            self.parity_expansion_factor(),
        ))
    }

    pub fn fold_pair(&self, round: usize, output_index: usize) -> Result<FoldPair, Error> {
        if round >= self.num_rounds() {
            return Err(Error::InvalidPcsParam(format!(
                "fold round {round} is outside {} rounds",
                self.num_rounds()
            )));
        }
        let current_len = self.codeword_len() >> round;
        let half_len = current_len >> 1;
        if output_index >= half_len {
            return Err(Error::InvalidPcsParam(format!(
                "fold output index {output_index} is outside round {round} output length {half_len}"
            )));
        }
        Ok(FoldPair {
            left: output_index,
            right: output_index + half_len,
            out: output_index,
        })
    }

    fn message_len_at_round(&self, round: usize) -> Result<usize, Error> {
        if round > self.num_rounds() {
            return Err(Error::InvalidPcsParam(format!(
                "round {round} is outside {} folding rounds",
                self.num_rounds()
            )));
        }
        Ok(self.message_len() >> round)
    }
}

impl Blaze2BaseFoldBackendParams {
    pub fn new(spec: Blaze2BaseFoldBackendSpec) -> Result<Self, Error> {
        validate_blaze2_backend_spec(&spec)?;
        let praa = Blaze2Code::new(spec.praa.clone())?;
        let compiler_code = SystematicAugmentedRfcCode::new(spec.compiler_code.clone())?;
        Ok(Self {
            spec,
            praa,
            compiler_code,
        })
    }

    pub fn spec(&self) -> &Blaze2BaseFoldBackendSpec {
        &self.spec
    }

    pub fn praa(&self) -> &Blaze2Code {
        &self.praa
    }

    pub fn compiler_code(&self) -> &SystematicAugmentedRfcCode {
        &self.compiler_code
    }

    pub fn query_schedule_spec(&self) -> HolographicQueryScheduleSpec {
        HolographicQueryScheduleSpec {
            q_raa_input: self.spec.q_raa_input,
            q_backend_proof: self.spec.q_backend_proof,
            auxiliary_oracle_len: self.spec.auxiliary_oracle_len,
            raa_relation_strategy: self.spec.raa_relation_strategy,
        }
    }

    pub fn prove_prequery<H: Hash>(
        &self,
        folded_codeword: &[B128],
        auxiliary_oracle: &[B128],
        request: &Blaze2BaseFoldOpenRequest<'_>,
    ) -> Result<
        (
            Blaze2BaseFoldPrequeryPublic<H>,
            Blaze2BaseFoldProverState<H>,
        ),
        Error,
    > {
        validate_blaze2_basefold_open_request(self, request)?;
        validate_auxiliary_oracle_shape(self.spec.auxiliary_oracle_len, auxiliary_oracle)?;
        validate_raa_relation_auxiliary_consistent_with_codeword(
            self.praa.packed(),
            folded_codeword,
            auxiliary_oracle,
        )?;
        let compiler_parity = self.compiler_code.commit_parity(folded_codeword)?;
        let compiler_parity_public = compiler_parity.public();
        let auxiliary = if self.spec.auxiliary_oracle_len == 0 {
            None
        } else {
            Some(AuxiliaryOracleCommitment::commit_values(
                auxiliary_oracle.to_vec(),
            )?)
        };
        let auxiliary_public = auxiliary.as_ref().map(AuxiliaryOracleCommitment::public);
        let (section5_permutation_helper, section5_relation_challenges) = self
            .commit_section5_permutation_helper::<H>(
                auxiliary_oracle,
                &compiler_parity_public,
                auxiliary_public.as_ref(),
                request,
            )?;
        let section5_permutation_helper_public = section5_permutation_helper
            .as_ref()
            .map(RaaSection5PermutationHelperCommitment::public);
        let section5_relation_residual =
            match (&section5_permutation_helper, section5_relation_challenges) {
                (Some(helper), Some(challenges)) => {
                    Some(RaaSection5RelationResidualCommitment::commit_values(
                        build_raa_section5_relation_residual_oracle(
                            self.praa.packed(),
                            auxiliary_oracle,
                            folded_codeword,
                            helper.values(),
                            challenges,
                        )?,
                    )?)
                }
                (None, None) => None,
                _ => {
                    return Err(Error::InvalidPcsOpen(
                        "RAA Section 5 helper and relation challenges are inconsistent".to_string(),
                    ));
                }
            };
        let section5_relation_residual_public = section5_relation_residual
            .as_ref()
            .map(RaaSection5RelationResidualCommitment::public);
        let section5_relation = match (
            &section5_permutation_helper,
            &section5_relation_residual,
            section5_relation_challenges,
        ) {
            (Some(helper), Some(residual), Some(challenges)) => {
                Some(prove_raa_section5_relation_proof(
                    self.praa.packed(),
                    auxiliary_oracle,
                    helper.values(),
                    residual.values(),
                    folded_codeword,
                    challenges,
                )?)
            }
            (None, None, None) => None,
            _ => {
                return Err(Error::InvalidPcsOpen(
                    "RAA Section 5 residual, helper, and relation challenges are inconsistent"
                        .to_string(),
                ));
            }
        };
        let (eval_sumcheck, eval_sumcheck_challenges) = if self.spec.auxiliary_oracle_len == 0 {
            (None, None)
        } else {
            let (proof, challenges) = prove_raa_eval_sumcheck::<H>(
                self.spec(),
                &compiler_parity_public,
                auxiliary_public.as_ref(),
                request,
                self.praa.packed(),
                folded_codeword,
            )?;
            (Some(proof), Some(challenges))
        };
        let (folded_parity_layers, folded_parity_public, physical_layers, fold_challenges) = self
            .prove_folded_parity_layers::<H>(
            folded_codeword,
            &compiler_parity_public,
            auxiliary_public.as_ref(),
            request,
            eval_sumcheck_challenges.as_deref(),
        )?;
        let public = Blaze2BaseFoldPrequeryPublic {
            compiler_parity: compiler_parity_public,
            eval_sumcheck,
            folded_parity_layers: folded_parity_public,
            terminal_codeword: physical_layers
                .last()
                .expect("folded parity prover returns at least the top physical layer")
                .clone(),
            auxiliary: auxiliary_public,
            section5_relation_residual: section5_relation_residual_public,
            section5_permutation_helper: section5_permutation_helper_public,
            section5_relation: section5_relation.clone(),
        };
        Ok((
            public,
            Blaze2BaseFoldProverState {
                compiler_parity,
                folded_parity_layers,
                physical_layers,
                fold_challenges,
                auxiliary,
                section5_relation_residual,
                section5_permutation_helper,
                section5_relation,
            },
        ))
    }

    pub fn sample_query_schedule<H: Hash, S>(
        &self,
        transcript: &mut CfriTranscript<H, S>,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
    ) -> Result<HolographicQuerySchedule, Error> {
        validate_blaze2_basefold_open_request(self, request)?;
        validate_section5_permutation_helper_public(self.spec(), prequery)?;
        absorb_blaze2_basefold_backend_spec(transcript, self.spec());
        absorb_blaze2_basefold_open_request(transcript, request);
        absorb_blaze2_basefold_prequery_public(transcript, prequery);
        let mut schedule = HolographicQuerySchedule::sample(
            transcript,
            self.compiler_code.layout(),
            self.query_schedule_spec(),
        )?;
        if self.spec.auxiliary_oracle_len != 0
            && self.spec.raa_relation_strategy == RaaRelationProofStrategy::LocalQueries
        {
            schedule.attach_raa_auxiliary_local_queries(self.praa.packed())?;
        }
        Ok(schedule)
    }

    pub fn open_query_proof<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<Blaze2BaseFoldQueryProof<H>, Error> {
        validate_section5_permutation_helper_state(self.spec(), state)?;
        let mut auxiliary = match &state.auxiliary {
            Some(auxiliary) => Some(auxiliary.prove_schedule(schedule)?),
            None => {
                let has_auxiliary_queries = schedule
                    .proof_queries()
                    .iter()
                    .any(|query| query.domain == BackendProofQueryDomain::RelationAuxiliary);
                if has_auxiliary_queries {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains auxiliary queries but no auxiliary oracle is committed"
                            .to_string(),
                    ));
                }
                None
            }
        };
        let compiler_parity = state.compiler_parity.prove_schedule(schedule)?;
        let compiler_parity_folds = self.open_compiler_parity_fold_paths(state, schedule)?;
        let mut section5_relation_residual = match &state.section5_relation_residual {
            Some(residual) => Some(residual.prove_schedule(schedule)?),
            None => {
                if schedule.section5_relation_residual_proof_query_count() != 0 {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains Section 5 residual queries but no residual oracle is committed"
                            .to_string(),
                    ));
                }
                None
            }
        };
        let section5_permutation_helper = match &state.section5_permutation_helper {
            Some(helper) => Some(helper.prove_schedule(schedule)?),
            None => {
                if schedule.section5_permutation_helper_proof_query_count() != 0 {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains Section 5 helper queries but no helper oracle is committed"
                            .to_string(),
                    ));
                }
                None
            }
        };
        let mut section5_relation_terminal_folded_layers = Vec::new();
        let mut section5_relation_terminal_layer_authentication = Vec::new();
        if schedule.raa_relation_strategy() == RaaRelationProofStrategy::Section5 {
            let section5_proof = state.section5_relation.clone().ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "Section 5 relation proof is missing from prover state".to_string(),
                )
            })?;
            if let (Some(residual_commitment), Some(residual_proof)) = (
                state.section5_relation_residual.as_ref(),
                section5_relation_residual.as_mut(),
            ) {
                let checks = section5_proof.verify_prequery::<H>(self.spec.auxiliary_oracle_len)?;
                let (terminal_proof, terminal_folded_layers, terminal_layer_authentication) =
                    prove_section5_relation_terminal_proof::<H>(
                        &checks,
                        residual_commitment,
                        schedule,
                    )?;
                residual_proof.terminal_proof = Some(terminal_proof);
                section5_relation_terminal_folded_layers = terminal_folded_layers;
                section5_relation_terminal_layer_authentication = terminal_layer_authentication;
            }
            let auxiliary_proof = auxiliary.as_mut().ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "Section 5 relation proof requires an auxiliary query proof".to_string(),
                )
            })?;
            auxiliary_proof.raa_relation = RaaRelationProof::Section5(section5_proof);
        }
        let query_set = collect_backend_query_set_from_prover_state(
            self.compiler_code.layout(),
            state,
            schedule,
            &compiler_parity,
            &compiler_parity_folds,
            auxiliary.as_ref(),
            section5_relation_residual.as_ref(),
            &section5_relation_terminal_folded_layers,
            section5_permutation_helper.as_ref(),
        )?;
        let mut authentication = self.open_backend_query_set_authentication(state, &query_set)?;
        authentication.section5_relation_terminal_folded_layers =
            section5_relation_terminal_folded_layers;
        authentication.section5_relation_terminal_layer_authentication =
            section5_relation_terminal_layer_authentication;
        Ok(Blaze2BaseFoldQueryProof {
            compiler_parity,
            compiler_parity_folds,
            auxiliary,
            section5_relation_residual,
            section5_permutation_helper,
            authentication,
        })
    }

    pub fn verify_query_proof<H: Hash>(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
        schedule: &HolographicQuerySchedule,
        proof: &Blaze2BaseFoldQueryProof<H>,
        top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        validate_blaze2_basefold_open_request(self, request)?;
        validate_section5_permutation_helper_public(self.spec(), prequery)?;
        schedule.validate_top_queries(top_queries)?;
        proof.compiler_parity.verify_schedule(
            &prequery.compiler_parity,
            self.compiler_code.layout(),
            schedule,
        )?;
        proof.compiler_parity_folds.verify_paths(
            prequery,
            self.compiler_code(),
            &proof.compiler_parity,
            &fold_challenges_from_prequery(self.spec(), prequery, request)?,
            schedule,
        )?;
        verify_auxiliary_query_proof(
            prequery.auxiliary.as_ref(),
            proof.auxiliary.as_ref(),
            self.spec.auxiliary_oracle_len,
            schedule,
            top_queries,
        )?;
        verify_section5_relation_query_proof_matches_prequery(
            prequery.section5_relation.as_ref(),
            proof.auxiliary.as_ref(),
            schedule,
        )?;
        verify_section5_relation_residual_query_proof(
            prequery.section5_relation_residual.as_ref(),
            proof.section5_relation_residual.as_ref(),
            schedule,
        )?;
        verify_section5_permutation_helper_query_proof(
            prequery.section5_permutation_helper.as_ref(),
            proof.section5_permutation_helper.as_ref(),
            schedule,
        )?;
        let query_set =
            self.collect_backend_query_set(prequery, request, schedule, proof, top_queries)?;
        proof.authentication.verify(prequery, &query_set)?;
        verify_raa_relation_openings(
            prequery.section5_relation_residual.as_ref(),
            proof.auxiliary.as_ref(),
            proof.section5_relation_residual.as_ref(),
            &proof.authentication,
            self.spec.auxiliary_oracle_len,
            schedule,
            top_queries,
        )
    }

    fn open_backend_query_set_authentication<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        query_set: &BackendProofOracleQuerySet,
    ) -> Result<BackendProofOracleAuthentication<H>, Error> {
        let layout = self.compiler_code.layout();
        if query_set.compiler_parity_fold_layer_queries.len() != layout.num_rounds() {
            return Err(Error::InvalidPcsOpen(
                "backend query set fold layer count does not match layout".to_string(),
            ));
        }

        let mut compiler_parity_layers = Vec::with_capacity(layout.num_rounds());
        for round in 0..layout.num_rounds() {
            let commitment = parity_commitment_at_round(
                &state.compiler_parity,
                &state.folded_parity_layers,
                round,
            )?;
            let mut queries = query_set.compiler_parity_fold_layer_queries[round].clone();
            if round == 0 {
                queries.extend(query_set.compiler_parity_queries.iter().copied());
            }
            compiler_parity_layers.push(CompilerParityFoldLayerProof {
                authentication_nodes: merkle_b128_multiproof_nodes::<H, _>(
                    &commitment.merkle_tree,
                    queries,
                )?,
            });
        }

        let auxiliary_nodes = match (&state.auxiliary, query_set.auxiliary_queries.is_empty()) {
            (Some(auxiliary), _) => merkle_b128_multiproof_nodes::<H, _>(
                &auxiliary.merkle_tree,
                query_set.auxiliary_queries.iter().copied(),
            )?,
            (None, true) => Vec::new(),
            (None, false) => {
                return Err(Error::InvalidPcsOpen(
                    "backend query set contains auxiliary leaves but no auxiliary oracle is committed"
                        .to_string(),
                ));
            }
        };
        let section5_relation_residual_nodes = match (
            &state.section5_relation_residual,
            query_set.section5_relation_residual_queries.is_empty(),
        ) {
            (Some(residual), _) => merkle_b128_multiproof_nodes::<H, _>(
                &residual.inner.merkle_tree,
                query_set.section5_relation_residual_queries.iter().copied(),
            )?,
            (None, true) => Vec::new(),
            (None, false) => {
                return Err(Error::InvalidPcsOpen(
                    "backend query set contains Section 5 residual leaves but no residual oracle is committed"
                        .to_string(),
                ));
            }
        };
        let section5_permutation_helper_nodes = match (
            &state.section5_permutation_helper,
            query_set.section5_permutation_helper_queries.is_empty(),
        ) {
            (Some(helper), _) => merkle_b128_multiproof_nodes::<H, _>(
                &helper.inner.merkle_tree,
                query_set
                    .section5_permutation_helper_queries
                    .iter()
                    .copied(),
            )?,
            (None, true) => Vec::new(),
            (None, false) => {
                return Err(Error::InvalidPcsOpen(
                    "backend query set contains Section 5 helper leaves but no helper oracle is committed"
                        .to_string(),
                ));
            }
        };

        Ok(BackendProofOracleAuthentication {
            compiler_parity_layers,
            auxiliary_nodes,
            section5_relation_residual_nodes,
            section5_relation_terminal_folded_layers: Vec::new(),
            section5_relation_terminal_layer_authentication: Vec::new(),
            section5_permutation_helper_nodes,
        })
    }

    pub fn collect_backend_query_set<H: Hash>(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
        schedule: &HolographicQuerySchedule,
        proof: &Blaze2BaseFoldQueryProof<H>,
        top_queries: &[TopQuery<B128>],
    ) -> Result<BackendProofOracleQuerySet, Error> {
        let fold_challenges = fold_challenges_from_prequery(self.spec(), prequery, request)?;
        proof.collect_backend_query_set(
            self.compiler_code.layout(),
            schedule,
            &fold_challenges,
            Some(&prequery.terminal_codeword),
            prequery.section5_relation.as_ref(),
            prequery.section5_relation_residual.as_ref(),
            self.spec.auxiliary_oracle_len,
            top_queries,
        )
    }

    pub fn top_queries_from_interleaved_columns<H: Hash>(
        &self,
        schedule: &HolographicQuerySchedule,
        column_openings: &[Blaze2InterleavedColumnQuery<H>],
        folding_challenges: &[B128],
    ) -> Result<Vec<TopQuery<B128>>, Error> {
        if column_openings.len() != schedule.input_queries().len() {
            return Err(Error::InvalidPcsOpen(format!(
                "opened Blaze column count {} does not match backend input query count {}",
                column_openings.len(),
                schedule.input_queries().len()
            )));
        }

        let mut top_queries = Vec::with_capacity(column_openings.len());
        for (expected, column) in schedule.input_queries().iter().zip(column_openings) {
            if column.index != expected.logical_index {
                return Err(Error::InvalidPcsOpen(
                    "opened Blaze column index does not match backend input query".to_string(),
                ));
            }
            if column.index >= self.praa.packed().codeword_len() {
                return Err(Error::InvalidPcsOpen(
                    "opened Blaze column index is outside PRAA codeword length".to_string(),
                ));
            }
            top_queries.push(TopQuery {
                index: expected.logical_index,
                value: fold_interleaved_column(column, folding_challenges)?,
            });
        }
        Ok(top_queries)
    }

    fn prove_folded_parity_layers<H: Hash>(
        &self,
        message: &[B128],
        compiler_parity: &CompilerParityPublicCommitment<H>,
        auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
        fold_challenge_override: Option<&[B128]>,
    ) -> Result<
        (
            Vec<CompilerParityCommitment<H>>,
            Vec<CompilerParityPublicCommitment<H>>,
            Vec<Vec<B128>>,
            Vec<B128>,
        ),
        Error,
    > {
        let layout = self.compiler_code.layout();
        let mut current = vec![B128::ZERO; layout.codeword_len()];
        let mut parity_scratch = vec![B128::ZERO; layout.parity_len()];
        self.compiler_code.encode_physical_codeword_into(
            message,
            &mut current,
            &mut parity_scratch,
        )?;

        let mut transcript = CfriTranscript::<H>::new();
        absorb_blaze2_basefold_fold_chain_prefix(
            &mut transcript,
            self.spec(),
            compiler_parity,
            auxiliary,
            request,
        );

        let mut physical_layers = Vec::with_capacity(layout.num_rounds() + 1);
        physical_layers.push(current.clone());
        let mut folded_parity_layers = Vec::with_capacity(layout.num_rounds());
        let mut folded_parity_public = Vec::with_capacity(layout.num_rounds());
        let mut fold_challenges = Vec::with_capacity(layout.num_rounds());

        if let Some(challenges) = fold_challenge_override {
            if challenges.len() != layout.num_rounds() {
                return Err(Error::InvalidPcsOpen(
                    "fold challenge override length does not match compiler layout".to_string(),
                ));
            }
        }

        for round in 0..layout.num_rounds() {
            let alpha = if let Some(challenges) = fold_challenge_override {
                challenges[round]
            } else {
                transcript.absorb("systematic-basefold-fold-challenge-v1");
                absorb_usize(&mut transcript, round);
                transcript.squeeze()
            };
            fold_challenges.push(alpha);

            let mut next = vec![B128::ZERO; current.len() >> 1];
            self.compiler_code
                .fold_physical_codeword_round_into(round, &current, alpha, &mut next)?;
            let parity = parity_values_at_round(layout, round + 1, &next)?;
            let commitment = CompilerParityCommitment::commit_values(parity)?;
            let public = commitment.public();
            absorb_folded_parity_public_commitment(&mut transcript, round + 1, &public);
            folded_parity_public.push(public);
            folded_parity_layers.push(commitment);
            physical_layers.push(next.clone());
            current = next;
        }

        Ok((
            folded_parity_layers,
            folded_parity_public,
            physical_layers,
            fold_challenges,
        ))
    }

    fn open_compiler_parity_fold_paths<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<CompilerParityFoldQueryProof<H>, Error> {
        let mut paths = Vec::new();
        for query in schedule.proof_queries() {
            if query.domain != BackendProofQueryDomain::CompilerParity {
                continue;
            }
            let top_physical_index = query.physical_index.ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "compiler parity query is missing its physical index".to_string(),
                )
            })?;
            paths.push(self.open_compiler_parity_fold_path(state, top_physical_index)?);
        }
        Ok(CompilerParityFoldQueryProof {
            paths,
            layer_authentication: Vec::new(),
        })
    }

    fn open_compiler_parity_fold_path<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        top_physical_index: usize,
    ) -> Result<CompilerParityFoldPath, Error> {
        let layout = self.compiler_code.layout();
        if state.physical_layers.len() != layout.num_rounds() + 1
            || state.folded_parity_layers.len() != layout.num_rounds()
            || state.fold_challenges.len() != layout.num_rounds()
        {
            return Err(Error::InvalidPcsOpen(
                "backend folded parity state has incompatible layer count".to_string(),
            ));
        }

        let mut steps = Vec::with_capacity(layout.num_rounds());
        let mut current_physical_index = top_physical_index;
        for round in 0..layout.num_rounds() {
            let current_len = layout.codeword_len() >> round;
            let half_len = current_len >> 1;
            let output_physical_index = current_physical_index & (half_len - 1);
            let pair = layout.fold_pair(round, output_physical_index)?;
            let sibling_physical_index = if current_physical_index == pair.left {
                pair.right
            } else if current_physical_index == pair.right {
                pair.left
            } else {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path current index does not lie in its fold pair"
                        .to_string(),
                ));
            };
            let sibling = parity_value_for_physical_index(
                layout,
                &state.physical_layers,
                round,
                sibling_physical_index,
            )?;
            let folded = parity_value_for_physical_index(
                layout,
                &state.physical_layers,
                round + 1,
                output_physical_index,
            )?;
            debug_assert_eq!(
                sibling.value,
                state.physical_layers[round][sibling_physical_index]
            );
            debug_assert_eq!(
                folded.value,
                state.physical_layers[round + 1][output_physical_index]
            );
            steps.push(CompilerParityFoldStep {
                sibling_value: sibling.value,
            });
            current_physical_index = output_physical_index;
        }

        Ok(CompilerParityFoldPath { steps })
    }

    fn commit_section5_permutation_helper<H: Hash>(
        &self,
        auxiliary_oracle: &[B128],
        compiler_parity: &CompilerParityPublicCommitment<H>,
        auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
    ) -> Result<
        (
            Option<RaaSection5PermutationHelperCommitment<H>>,
            Option<RaaSection5RelationChallenges>,
        ),
        Error,
    > {
        if self.spec.raa_relation_strategy != RaaRelationProofStrategy::Section5 {
            return Ok((None, None));
        }
        let challenges = squeeze_raa_section5_relation_challenges(
            self.spec(),
            compiler_parity,
            auxiliary,
            request,
        );
        let values = build_raa_section5_permutation_helper_oracle(
            self.praa.packed(),
            auxiliary_oracle,
            challenges,
        )?;
        Ok((
            Some(RaaSection5PermutationHelperCommitment::commit_values(
                values,
            )?),
            Some(challenges),
        ))
    }
}

impl SystematicAugmentedRfcCode {
    pub fn new(spec: SystematicFoldableCodeSpec) -> Result<Self, Error> {
        let layout = SystematicAugmentedRfcLayout::new(spec)?;
        let parity_fold_table = build_parity_fold_table(&layout);
        Ok(Self {
            layout,
            parity_fold_table,
        })
    }

    pub fn layout(&self) -> &SystematicAugmentedRfcLayout {
        &self.layout
    }

    pub fn parity_fold_table(&self) -> &[Vec<B128>] {
        &self.parity_fold_table
    }

    pub fn encode_parity_into(&self, message: &[B128], out: &mut [B128]) -> Result<(), Error> {
        self.encode_parity_at_round_into(0, message, out)
    }

    pub fn encode_parity_at_round_into(
        &self,
        round: usize,
        message: &[B128],
        out: &mut [B128],
    ) -> Result<(), Error> {
        validate_message_and_parity_output_at_round(self.layout(), round, message, out)?;

        let parity_expansion_factor = self.layout.parity_expansion_factor();
        let base_multipliers = &self.parity_fold_table[0];
        debug_assert_eq!(base_multipliers.len(), parity_expansion_factor);
        for (chunk, &value) in out.chunks_exact_mut(parity_expansion_factor).zip(message) {
            for (dst, &multiplier) in chunk.iter_mut().zip(base_multipliers) {
                *dst = value * multiplier;
            }
        }

        let mut half_chunk_len = parity_expansion_factor;
        while half_chunk_len < out.len() {
            let chunk_len = half_chunk_len << 1;
            for chunk in out.chunks_exact_mut(chunk_len) {
                for j in 0..half_chunk_len {
                    chunk[j + half_chunk_len] += chunk[j];
                }
            }
            half_chunk_len = chunk_len;
        }
        Ok(())
    }

    pub fn encode_logical_codeword_into(
        &self,
        message: &[B128],
        out: &mut [B128],
    ) -> Result<(), Error> {
        if out.len() != self.layout.codeword_len() {
            return Err(Error::InvalidPcsOpen(format!(
                "logical codeword output has length {}, expected {}",
                out.len(),
                self.layout.codeword_len()
            )));
        }
        if message.len() != self.layout.message_len() {
            return Err(Error::InvalidPcsOpen(format!(
                "systematic RFC message has length {}, expected {}",
                message.len(),
                self.layout.message_len()
            )));
        }
        out[..self.layout.systematic_len()].copy_from_slice(message);
        self.encode_parity_into(message, &mut out[self.layout.systematic_len()..])
    }

    pub fn encode_physical_codeword_into(
        &self,
        message: &[B128],
        out: &mut [B128],
        parity_scratch: &mut [B128],
    ) -> Result<(), Error> {
        self.encode_physical_codeword_at_round_into(0, message, out, parity_scratch)
    }

    pub fn encode_physical_codeword_at_round_into(
        &self,
        round: usize,
        message: &[B128],
        out: &mut [B128],
        parity_scratch: &mut [B128],
    ) -> Result<(), Error> {
        let message_len = self.layout.message_len_at_round(round)?;
        let codeword_len = message_len * (self.layout.parity_expansion_factor() + 1);
        if out.len() != codeword_len {
            return Err(Error::InvalidPcsOpen(format!(
                "physical codeword output has length {}, expected {codeword_len}",
                out.len()
            )));
        }
        if message.len() != message_len {
            return Err(Error::InvalidPcsOpen(format!(
                "systematic RFC message has length {}, expected {message_len}",
                message.len()
            )));
        }
        let parity_len = message_len * self.layout.parity_expansion_factor();
        if parity_scratch.len() != parity_len {
            return Err(Error::InvalidPcsOpen(format!(
                "physical codeword parity scratch has length {}, expected {parity_len}",
                parity_scratch.len()
            )));
        }
        self.encode_parity_at_round_into(round, message, parity_scratch)?;

        let parity_expansion_factor = self.layout.parity_expansion_factor();
        for (logical_index, &value) in message.iter().enumerate() {
            let physical_index =
                systematic_to_physical(logical_index, message_len, parity_expansion_factor);
            out[physical_index] = value;
        }
        for (logical_index, &value) in parity_scratch.iter().enumerate() {
            let physical_index =
                parity_to_physical(logical_index, message_len, parity_expansion_factor);
            out[physical_index] = value;
        }
        Ok(())
    }

    pub fn fold_physical_codeword_round_into(
        &self,
        round: usize,
        current: &[B128],
        alpha: B128,
        out: &mut [B128],
    ) -> Result<(), Error> {
        let message_len = self.layout.message_len_at_round(round)?;
        let current_len = message_len * (self.layout.parity_expansion_factor() + 1);
        let folded_len = current_len >> 1;
        if current.len() != current_len {
            return Err(Error::InvalidPcsOpen(format!(
                "current physical codeword has length {}, expected {current_len}",
                current.len()
            )));
        }
        if out.len() != folded_len {
            return Err(Error::InvalidPcsOpen(format!(
                "folded physical codeword output has length {}, expected {folded_len}",
                out.len(),
            )));
        }
        for (output_index, folded) in out.iter_mut().enumerate() {
            let pair = self.layout.fold_pair(round, output_index)?;
            let address = self.layout.physical_to_logical_at_round(round, pair.left)?;
            *folded = match address.part {
                CodewordPart::Systematic => {
                    fold_systematic_pair(current[pair.left], current[pair.right], alpha)
                }
                CodewordPart::Parity => {
                    fold_rfc_parity_pair(current[pair.left], current[pair.right], alpha)
                }
            };
        }
        Ok(())
    }

    pub fn commit_parity<H: Hash>(
        &self,
        message: &[B128],
    ) -> Result<CompilerParityCommitment<H>, Error> {
        let mut values = vec![B128::ZERO; self.layout.parity_len()];
        self.encode_parity_into(message, &mut values)?;
        CompilerParityCommitment::commit_values(values)
    }
}

impl<H: Hash> CompilerParityCommitment<H> {
    pub fn commit_values(values: Vec<B128>) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(Error::InvalidPcsParam(
                "compiler parity commitment needs a non-empty value vector".to_string(),
            ));
        }
        let merkle_tree = merkelize_b128_padded::<H>(&values);
        Ok(Self {
            values,
            merkle_tree,
        })
    }

    pub fn public(&self) -> CompilerParityPublicCommitment<H> {
        CompilerParityPublicCommitment {
            root: self.root().clone(),
            len: self.len(),
        }
    }

    pub fn root(&self) -> &Output<H> {
        &self.merkle_tree[self.merkle_tree.len() - 1][0]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values(&self) -> &[B128] {
        &self.values
    }

    pub fn query(&self, logical_index: usize) -> Result<CompilerParityQuery, Error> {
        validate_parity_query_index(logical_index, self.len())?;
        Ok(CompilerParityQuery {
            logical_index,
            value: self.values[logical_index],
        })
    }

    pub fn prove_schedule(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<CompilerParityQueryProof<H>, Error> {
        let mut queries = Vec::new();
        for query in schedule.proof_queries() {
            if query.domain == BackendProofQueryDomain::CompilerParity {
                queries.push(self.query(query.index)?);
            }
        }
        Ok(CompilerParityQueryProof {
            queries,
            authentication_nodes: Vec::new(),
        })
    }
}

impl<H: Hash> CompilerParityQueryProof<H> {
    fn verify_schedule(
        &self,
        public: &CompilerParityPublicCommitment<H>,
        layout: &SystematicAugmentedRfcLayout,
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        if public.len != layout.parity_len() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity commitment length does not match code layout".to_string(),
            ));
        }

        let expected_count = schedule
            .proof_queries()
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::CompilerParity)
            .count();
        if self.queries.len() != expected_count {
            return Err(Error::InvalidPcsOpen(
                "compiler parity query proof count does not match schedule".to_string(),
            ));
        }

        let mut supplied = self.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::CompilerParity {
                continue;
            }
            let query = supplied.next().expect("query count checked above");
            if query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity query index does not match schedule".to_string(),
                ));
            }
        }
        if !self.authentication_nodes.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity authentication must be carried by the backend proof oracle"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

impl<H: Hash> Blaze2BaseFoldQueryProof<H> {
    pub fn collect_backend_query_set(
        &self,
        layout: &SystematicAugmentedRfcLayout,
        schedule: &HolographicQuerySchedule,
        fold_challenges: &[B128],
        terminal_codeword: Option<&[B128]>,
        section5_relation: Option<&RaaSection5RelationProof>,
        section5_relation_residual_public: Option<&RaaSection5RelationResidualPublicCommitment<H>>,
        auxiliary_oracle_len: usize,
        top_queries: &[TopQuery<B128>],
    ) -> Result<BackendProofOracleQuerySet, Error> {
        let compiler_parity_queries = self
            .compiler_parity
            .queries
            .iter()
            .map(|query| (query.logical_index, query.value))
            .collect();
        let compiler_parity_fold_layer_queries = compiler_parity_fold_layer_queries(
            layout,
            &self.compiler_parity_folds.paths,
            &self.compiler_parity.queries,
            fold_challenges,
            terminal_codeword,
        )?;
        let auxiliary_queries = match &self.auxiliary {
            Some(auxiliary) => auxiliary.authentication_queries(schedule, top_queries)?,
            None => {
                if expected_auxiliary_query_proof_count(schedule) != 0 {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains auxiliary queries, but auxiliary query proof is absent"
                            .to_string(),
                    ));
                }
                Vec::new()
            }
        };
        let section5_permutation_helper_queries = match &self.section5_permutation_helper {
            Some(helper) => helper.authentication_queries(schedule)?,
            None => {
                if schedule.section5_permutation_helper_proof_query_count() != 0 {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains Section 5 helper queries, but helper query proof is absent"
                            .to_string(),
                    ));
                }
                Vec::new()
            }
        };
        let mut section5_relation_residual_queries = match &self.section5_relation_residual {
            Some(residual) => residual.authentication_queries(schedule)?,
            None => {
                if schedule.section5_relation_residual_proof_query_count() != 0 {
                    return Err(Error::InvalidPcsOpen(
                        "backend schedule contains Section 5 residual queries, but residual query proof is absent"
                            .to_string(),
                    ));
                }
                Vec::new()
            }
        };
        if matches!(
            self.auxiliary.as_ref().map(|proof| &proof.raa_relation),
            Some(RaaRelationProof::Section5(_))
        ) && self
            .section5_relation_residual
            .as_ref()
            .is_some_and(|proof| proof.terminal_proof.is_none())
        {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal binding proof is missing".to_string(),
            ));
        }
        if let (Some(residual), Some(relation), Some(public)) = (
            self.section5_relation_residual.as_ref(),
            section5_relation,
            section5_relation_residual_public,
        ) {
            let checks = relation.verify_prequery::<H>(auxiliary_oracle_len)?;
            section5_relation_residual_queries.extend(
                residual.terminal_residual_authentication_queries(
                    &checks,
                    public,
                    schedule,
                    &self.authentication.section5_relation_terminal_folded_layers,
                )?,
            );
        }

        Ok(BackendProofOracleQuerySet {
            compiler_parity_queries,
            compiler_parity_fold_layer_queries,
            auxiliary_queries,
            section5_relation_residual_queries,
            section5_permutation_helper_queries,
        })
    }
}

impl<H: Hash> BackendProofOracleAuthentication<H> {
    pub fn hash_node_count(&self) -> usize {
        self.compiler_parity_layers
            .iter()
            .map(|layer| layer.authentication_nodes.len())
            .sum::<usize>()
            + self.auxiliary_nodes.len()
            + self.section5_relation_residual_nodes.len()
            + self.section5_relation_terminal_folded_layers.len()
            + self
                .section5_relation_terminal_layer_authentication
                .iter()
                .map(|layer| layer.authentication_nodes.len())
                .sum::<usize>()
            + self.section5_permutation_helper_nodes.len()
    }

    pub fn verify(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        query_set: &BackendProofOracleQuerySet,
    ) -> Result<(), Error> {
        if self.compiler_parity_layers.len() != query_set.compiler_parity_fold_layer_queries.len() {
            return Err(Error::InvalidPcsOpen(
                "backend proof oracle authentication layer count does not match query set"
                    .to_string(),
            ));
        }
        for (round, (proof, fold_queries)) in self
            .compiler_parity_layers
            .iter()
            .zip(query_set.compiler_parity_fold_layer_queries.iter())
            .enumerate()
        {
            let public = parity_public_at_round(prequery, round)?;
            let mut queries = fold_queries.clone();
            if round == 0 {
                queries.extend(query_set.compiler_parity_queries.iter().copied());
            }
            verify_merkle_b128_multiproof::<H, _>(
                &public.root,
                public.len,
                queries,
                &proof.authentication_nodes,
            )?;
        }

        match (&prequery.auxiliary, query_set.auxiliary_queries.is_empty()) {
            (Some(public), _) => verify_merkle_b128_multiproof::<H, _>(
                &public.root,
                public.len,
                query_set.auxiliary_queries.iter().copied(),
                &self.auxiliary_nodes,
            )?,
            (None, true) if self.auxiliary_nodes.is_empty() => {}
            (None, _) => {
                return Err(Error::InvalidPcsOpen(
                    "backend proof oracle contains auxiliary authentication without an auxiliary commitment"
                        .to_string(),
                ));
            }
        }
        match (
            &prequery.section5_relation_residual,
            query_set.section5_relation_residual_queries.is_empty(),
        ) {
            (Some(public), _) => verify_merkle_b128_multiproof::<H, _>(
                &public.root,
                public.len,
                query_set.section5_relation_residual_queries.iter().copied(),
                &self.section5_relation_residual_nodes,
            )?,
            (None, true) if self.section5_relation_residual_nodes.is_empty() => {}
            (None, _) => {
                return Err(Error::InvalidPcsOpen(
                    "backend proof oracle contains Section 5 residual authentication without a residual commitment"
                        .to_string(),
                ));
            }
        }
        match (
            &prequery.section5_permutation_helper,
            query_set.section5_permutation_helper_queries.is_empty(),
        ) {
            (Some(public), _) => verify_merkle_b128_multiproof::<H, _>(
                &public.root,
                public.len,
                query_set
                    .section5_permutation_helper_queries
                    .iter()
                    .copied(),
                &self.section5_permutation_helper_nodes,
            )?,
            (None, true) if self.section5_permutation_helper_nodes.is_empty() => {}
            (None, _) => {
                return Err(Error::InvalidPcsOpen(
                    "backend proof oracle contains Section 5 helper authentication without a helper commitment"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl<H: Hash> CompilerParityFoldQueryProof<H> {
    fn verify_paths(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        code: &SystematicAugmentedRfcCode,
        top_queries: &CompilerParityQueryProof<H>,
        fold_challenges: &[B128],
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        let layout = code.layout();
        if prequery.folded_parity_layers.len() != layout.num_rounds()
            || fold_challenges.len() != layout.num_rounds()
        {
            return Err(Error::InvalidPcsOpen(
                "folded parity prequery layer count does not match layout".to_string(),
            ));
        }
        verify_terminal_codeword(prequery, code)?;

        let expected_count = schedule
            .proof_queries()
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::CompilerParity)
            .count();
        if self.paths.len() != expected_count {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold path count does not match schedule".to_string(),
            ));
        }
        if top_queries.queries.len() != expected_count {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold path top query count does not match schedule".to_string(),
            ));
        }
        top_queries.verify_schedule(&prequery.compiler_parity, layout, schedule)?;
        if !self.layer_authentication.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold authentication must be carried by the backend proof oracle"
                    .to_string(),
            ));
        }

        let mut supplied = self.paths.iter();
        let mut supplied_top_queries = top_queries.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::CompilerParity {
                continue;
            }
            let path = supplied.next().expect("path count checked above");
            let top_query = supplied_top_queries
                .next()
                .expect("top query count checked above");
            let expected_physical = expected.physical_index.ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "compiler parity query is missing its physical index".to_string(),
                )
            })?;
            if top_query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path does not match schedule".to_string(),
                ));
            }
            path.verify(layout, fold_challenges, top_query, expected_physical)?;
        }
        compiler_parity_fold_layer_queries(
            layout,
            &self.paths,
            &top_queries.queries,
            fold_challenges,
            Some(&prequery.terminal_codeword),
        )?;
        Ok(())
    }
}

impl CompilerParityFoldPath {
    fn verify(
        &self,
        layout: &SystematicAugmentedRfcLayout,
        fold_challenges: &[B128],
        top_query: &CompilerParityQuery,
        top_physical_index: usize,
    ) -> Result<(), Error> {
        if self.steps.len() != layout.num_rounds() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold path has wrong round count".to_string(),
            ));
        }
        let mut current_value = top_query.value;
        let mut current_physical_index = top_physical_index;
        for (round, step) in self.steps.iter().enumerate() {
            let current_len = layout.codeword_len() >> round;
            let half_len = current_len >> 1;
            let output_physical_index = current_physical_index & (half_len - 1);
            let pair = layout.fold_pair(round, output_physical_index)?;
            let (left_value, right_value, sibling_physical_index) =
                if current_physical_index == pair.left {
                    (current_value, step.sibling_value, pair.right)
                } else if current_physical_index == pair.right {
                    (step.sibling_value, current_value, pair.left)
                } else {
                    return Err(Error::InvalidPcsOpen(
                        "compiler parity fold path current index does not lie in its fold pair"
                            .to_string(),
                    ));
                };
            parity_logical_index_at_round(layout, round, sibling_physical_index)?;
            parity_logical_index_at_round(layout, round + 1, output_physical_index)?;

            let folded = fold_rfc_parity_pair(left_value, right_value, fold_challenges[round]);
            current_value = folded;
            current_physical_index = output_physical_index;
        }
        Ok(())
    }
}

impl<H: Hash> AuxiliaryOracleCommitment<H> {
    pub fn commit_values(values: Vec<B128>) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(Error::InvalidPcsParam(
                "auxiliary oracle commitment needs a non-empty value vector".to_string(),
            ));
        }
        let merkle_tree = merkelize_b128_padded::<H>(&values);
        Ok(Self {
            values,
            merkle_tree,
        })
    }

    pub fn public(&self) -> AuxiliaryOraclePublicCommitment<H> {
        AuxiliaryOraclePublicCommitment {
            root: self.root().clone(),
            len: self.len(),
        }
    }

    pub fn root(&self) -> &Output<H> {
        &self.merkle_tree[self.merkle_tree.len() - 1][0]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values(&self) -> &[B128] {
        &self.values
    }

    pub fn query(&self, logical_index: usize) -> Result<AuxiliaryOracleQuery, Error> {
        validate_auxiliary_query_index(logical_index, self.len())?;
        Ok(AuxiliaryOracleQuery {
            logical_index,
            value: self.values[logical_index],
        })
    }

    pub fn prove_schedule(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<AuxiliaryOracleQueryProof<H>, Error> {
        let mut relation_queries = Vec::new();
        for query in schedule.proof_queries() {
            if query.domain == BackendProofQueryDomain::RelationAuxiliary {
                relation_queries.push(self.query(query.index)?);
            }
        }
        if schedule.raa_relation_strategy() == RaaRelationProofStrategy::Section5 {
            return Ok(AuxiliaryOracleQueryProof {
                relation_queries,
                raa_relation: RaaRelationProof::Section5(RaaSection5RelationProof::default()),
                authentication_nodes: Vec::new(),
            });
        }
        let mut final_accumulator_queries = Vec::with_capacity(schedule.raa_final_queries().len());
        for query in schedule.raa_final_queries() {
            self.query(query.u4_auxiliary_index)?;
            final_accumulator_queries.push(RaaFinalAccumulatorQueryProof);
        }
        let mut local_relation_queries = Vec::with_capacity(schedule.raa_auxiliary_queries().len());
        for query in schedule.raa_auxiliary_queries() {
            local_relation_queries.push(self.local_relation_proof(query)?);
        }
        let proof = AuxiliaryOracleQueryProof {
            relation_queries,
            raa_relation: RaaRelationProof::LocalQueries(RaaLocalRelationProof {
                final_accumulator_queries,
                local_relation_queries,
            }),
            authentication_nodes: Vec::new(),
        };
        Ok(AuxiliaryOracleQueryProof { ..proof })
    }

    fn local_relation_proof(
        &self,
        query: &RaaAuxiliaryLocalQuery,
    ) -> Result<RaaAuxiliaryLocalRelationProof, Error> {
        let main = self.query(query.main_auxiliary_index)?;
        match query.relation {
            RaaAuxiliaryRelationKind::U2Repetition { .. }
            | RaaAuxiliaryRelationKind::FirstAccumulatorStart { .. }
            | RaaAuxiliaryRelationKind::SecondPermutation { .. } => {
                let extra = self.query(query.extra_index(0).expect("single extra relation"))?;
                if main.value != extra.value {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local equality relation failed during proof construction"
                            .to_string(),
                    ));
                }
                Ok(RaaAuxiliaryLocalRelationProof::Equality)
            }
            RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. } => {
                let previous = self.query(query.extra_index(0).expect("previous u3"))?;
                let u2 = self.query(query.extra_index(1).expect("u2"))?;
                if main.value != previous.value + u2.value {
                    return Err(Error::InvalidPcsOpen(
                        "RAA first accumulator relation failed during proof construction"
                            .to_string(),
                    ));
                }
                Ok(RaaAuxiliaryLocalRelationProof::FirstAccumulatorStep {
                    previous_value: previous.value,
                })
            }
        }
    }
}

impl<H: Hash> RaaSection5PermutationHelperCommitment<H> {
    pub fn commit_values(values: Vec<B128>) -> Result<Self, Error> {
        Ok(Self {
            inner: AuxiliaryOracleCommitment::commit_values(values)?,
        })
    }

    pub fn public(&self) -> RaaSection5PermutationHelperPublicCommitment<H> {
        RaaSection5PermutationHelperPublicCommitment {
            root: self.inner.root().clone(),
            len: self.inner.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn values(&self) -> &[B128] {
        self.inner.values()
    }

    pub fn query(&self, logical_index: usize) -> Result<AuxiliaryOracleQuery, Error> {
        self.inner.query(logical_index)
    }

    pub fn prove_schedule(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<RaaSection5PermutationHelperQueryProof<H>, Error> {
        let mut queries = Vec::new();
        for query in schedule.proof_queries() {
            if query.domain == BackendProofQueryDomain::Section5PermutationHelper {
                queries.push(self.query(query.index)?);
            }
        }
        Ok(RaaSection5PermutationHelperQueryProof {
            queries,
            authentication_nodes: Vec::new(),
        })
    }
}

impl<H: Hash> RaaSection5RelationResidualCommitment<H> {
    pub fn commit_values(values: Vec<B128>) -> Result<Self, Error> {
        Ok(Self {
            inner: AuxiliaryOracleCommitment::commit_values(values)?,
        })
    }

    pub fn public(&self) -> RaaSection5RelationResidualPublicCommitment<H> {
        RaaSection5RelationResidualPublicCommitment {
            root: self.inner.root().clone(),
            len: self.inner.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn values(&self) -> &[B128] {
        self.inner.values()
    }

    pub fn query(&self, logical_index: usize) -> Result<AuxiliaryOracleQuery, Error> {
        self.inner.query(logical_index)
    }

    pub fn prove_schedule(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<RaaSection5RelationResidualQueryProof<H>, Error> {
        let mut queries = Vec::new();
        for query in schedule.proof_queries() {
            if query.domain == BackendProofQueryDomain::Section5RelationResidual {
                queries.push(self.query(query.index)?);
            }
        }
        Ok(RaaSection5RelationResidualQueryProof {
            queries,
            authentication_nodes: Vec::new(),
            terminal_proof: None,
        })
    }
}

impl<H: Hash> RaaSection5PermutationHelperQueryProof<H> {
    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    pub fn serialized_value_count(&self) -> usize {
        self.queries.len()
    }

    fn verify_schedule(
        &self,
        public: &RaaSection5PermutationHelperPublicCommitment<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        if !self.authentication_nodes.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "Section 5 helper authentication must be carried by the backend proof oracle"
                    .to_string(),
            ));
        }
        if self.query_count() != schedule.section5_permutation_helper_proof_query_count() {
            return Err(Error::InvalidPcsOpen(
                "Section 5 helper query proof count does not match schedule".to_string(),
            ));
        }
        let mut queries = self.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::Section5PermutationHelper {
                continue;
            }
            let query = queries.next().ok_or_else(|| {
                Error::InvalidPcsOpen("Section 5 helper query opening is missing".to_string())
            })?;
            if query.logical_index != expected.index || query.logical_index >= public.len {
                return Err(Error::InvalidPcsOpen(
                    "Section 5 helper query index does not match schedule".to_string(),
                ));
            }
        }
        if queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many Section 5 helper query openings".to_string(),
            ));
        }
        Ok(())
    }

    fn authentication_queries(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<Vec<(usize, B128)>, Error> {
        let mut queries = Vec::with_capacity(self.queries.len());
        let mut supplied = self.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::Section5PermutationHelper {
                continue;
            }
            let query = supplied.next().ok_or_else(|| {
                Error::InvalidPcsOpen("Section 5 helper query opening is missing".to_string())
            })?;
            if query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "Section 5 helper query index does not match schedule".to_string(),
                ));
            }
            queries.push((query.logical_index, query.value));
        }
        if supplied.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many Section 5 helper query openings".to_string(),
            ));
        }
        Ok(queries)
    }
}

impl<H: Hash> RaaSection5RelationResidualQueryProof<H> {
    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    pub fn serialized_value_count(&self) -> usize {
        self.queries.len()
            + self
                .terminal_proof
                .as_ref()
                .map(RaaSection5RelationTerminalProof::serialized_value_count)
                .unwrap_or(0)
    }

    fn terminal_residual_authentication_queries(
        &self,
        checks: &RaaSection5RelationChecks,
        public: &RaaSection5RelationResidualPublicCommitment<H>,
        schedule: &HolographicQuerySchedule,
        folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        match self.terminal_proof.as_ref() {
            Some(proof) => {
                proof.residual_authentication_queries(checks, public, schedule, folded_layers)
            }
            None => Ok(Vec::new()),
        }
    }

    fn verify_schedule(
        &self,
        public: &RaaSection5RelationResidualPublicCommitment<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        if !self.authentication_nodes.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "Section 5 relation residual authentication must be carried by the backend proof oracle"
                    .to_string(),
            ));
        }
        if self.query_count() != schedule.section5_relation_residual_proof_query_count() {
            return Err(Error::InvalidPcsOpen(
                "Section 5 relation residual query proof count does not match schedule".to_string(),
            ));
        }
        let mut queries = self.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::Section5RelationResidual {
                continue;
            }
            let query = queries.next().ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "Section 5 relation residual query opening is missing".to_string(),
                )
            })?;
            if query.logical_index != expected.index || query.logical_index >= public.len {
                return Err(Error::InvalidPcsOpen(
                    "Section 5 relation residual query index does not match schedule".to_string(),
                ));
            }
        }
        if queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many Section 5 relation residual query openings".to_string(),
            ));
        }
        Ok(())
    }

    fn authentication_queries(
        &self,
        schedule: &HolographicQuerySchedule,
    ) -> Result<Vec<(usize, B128)>, Error> {
        let mut queries = Vec::with_capacity(self.queries.len());
        let mut supplied = self.queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::Section5RelationResidual {
                continue;
            }
            let query = supplied.next().ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "Section 5 relation residual query opening is missing".to_string(),
                )
            })?;
            if query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "Section 5 relation residual query index does not match schedule".to_string(),
                ));
            }
            queries.push((query.logical_index, query.value));
        }
        if supplied.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many Section 5 relation residual query openings".to_string(),
            ));
        }
        Ok(queries)
    }
}

impl<H: Hash> AuxiliaryOracleQueryProof<H> {
    pub fn query_count(&self) -> usize {
        self.relation_queries.len() + self.raa_relation.query_count()
    }

    pub fn serialized_value_count(&self) -> usize {
        self.relation_queries.len() + self.raa_relation.serialized_value_count()
    }

    fn verify_schedule(
        &self,
        public: &AuxiliaryOraclePublicCommitment<H>,
        auxiliary_oracle_len: usize,
        schedule: &HolographicQuerySchedule,
        _top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        if public.len != auxiliary_oracle_len {
            return Err(Error::InvalidPcsOpen(
                "auxiliary oracle commitment length does not match backend spec".to_string(),
            ));
        }
        if !self.authentication_nodes.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "auxiliary authentication must be carried by the backend proof oracle".to_string(),
            ));
        }

        let expected_count = expected_auxiliary_query_proof_count(schedule);
        if self.query_count() != expected_count {
            return Err(Error::InvalidPcsOpen(
                "auxiliary oracle query proof count does not match schedule".to_string(),
            ));
        }

        let mut relation_queries = self.relation_queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::RelationAuxiliary {
                continue;
            }
            let query = relation_queries.next().ok_or_else(|| {
                Error::InvalidPcsOpen("relation auxiliary query opening is missing".to_string())
            })?;
            if query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "auxiliary oracle query index does not match schedule".to_string(),
                ));
            }
        }
        if relation_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many relation auxiliary query openings".to_string(),
            ));
        }

        self.raa_relation.verify_schedule(schedule)
    }

    fn authentication_queries(
        &self,
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        let expected_count = expected_auxiliary_query_proof_count(schedule);
        let mut queries = Vec::with_capacity(expected_count);

        let mut relation_queries = self.relation_queries.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::RelationAuxiliary {
                continue;
            }
            let query = relation_queries.next().ok_or_else(|| {
                Error::InvalidPcsOpen("relation auxiliary query opening is missing".to_string())
            })?;
            if query.logical_index != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "auxiliary oracle query index does not match schedule".to_string(),
                ));
            }
            queries.push((query.logical_index, query.value));
        }
        if relation_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many relation auxiliary query openings".to_string(),
            ));
        }

        queries.extend(self.raa_relation.authentication_queries(
            &self.relation_queries,
            schedule,
            top_queries,
        )?);

        Ok(queries)
    }

    fn authentication_queries_from_oracle<HH: Hash>(
        &self,
        schedule: &HolographicQuerySchedule,
        oracle: &AuxiliaryOracleCommitment<HH>,
    ) -> Result<Vec<(usize, B128)>, Error> {
        let mut queries = Vec::with_capacity(expected_auxiliary_query_proof_count(schedule));
        queries.extend(
            self.relation_queries
                .iter()
                .map(|query| (query.logical_index, query.value)),
        );
        queries.extend(self.raa_relation.authentication_queries_from_oracle(
            &self.relation_queries,
            schedule,
            oracle,
        )?);
        Ok(queries)
    }
}

impl RaaRelationProof {
    pub fn local_queries(&self) -> Result<&RaaLocalRelationProof, Error> {
        match self {
            Self::LocalQueries(proof) => Ok(proof),
            Self::Section5(_) => Err(Error::InvalidPcsOpen(
                "RAA relation proof uses Section 5, not local queries".to_string(),
            )),
        }
    }

    pub fn local_queries_mut(&mut self) -> Result<&mut RaaLocalRelationProof, Error> {
        match self {
            Self::LocalQueries(proof) => Ok(proof),
            Self::Section5(_) => Err(Error::InvalidPcsOpen(
                "RAA relation proof uses Section 5, not local queries".to_string(),
            )),
        }
    }

    fn query_count(&self) -> usize {
        match self {
            Self::LocalQueries(proof) => proof.query_count(),
            Self::Section5(proof) => proof.query_count(),
        }
    }

    fn serialized_value_count(&self) -> usize {
        match self {
            Self::LocalQueries(proof) => proof.serialized_value_count(),
            Self::Section5(proof) => proof.serialized_value_count(),
        }
    }

    fn verify_schedule(&self, schedule: &HolographicQuerySchedule) -> Result<(), Error> {
        match self {
            Self::LocalQueries(proof) => proof.verify_schedule(schedule),
            Self::Section5(proof) => proof.verify_schedule(schedule),
        }
    }

    fn authentication_queries(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        match self {
            Self::LocalQueries(proof) => {
                proof.authentication_queries(relation_queries, schedule, top_queries)
            }
            Self::Section5(proof) => {
                proof.authentication_queries(relation_queries, schedule, top_queries)
            }
        }
    }

    fn authentication_queries_from_oracle<H: Hash>(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        oracle: &AuxiliaryOracleCommitment<H>,
    ) -> Result<Vec<(usize, B128)>, Error> {
        match self {
            Self::LocalQueries(proof) => {
                proof.authentication_queries_from_oracle(relation_queries, schedule, oracle)
            }
            Self::Section5(proof) => {
                proof.authentication_queries_from_oracle(relation_queries, schedule, oracle)
            }
        }
    }

    fn verify_openings<H: Hash>(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        section5_relation_residual_public: Option<&RaaSection5RelationResidualPublicCommitment<H>>,
        section5_relation_residual: Option<&RaaSection5RelationResidualQueryProof<H>>,
        authentication: &BackendProofOracleAuthentication<H>,
        auxiliary_oracle_len: usize,
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        match self {
            Self::LocalQueries(proof) => {
                proof.verify_openings(relation_queries, schedule, top_queries)
            }
            Self::Section5(proof) => proof.verify_openings::<H>(
                relation_queries,
                section5_relation_residual_public,
                section5_relation_residual,
                authentication,
                auxiliary_oracle_len,
                schedule,
                top_queries,
            ),
        }
    }
}

impl RaaLocalRelationProof {
    fn query_count(&self) -> usize {
        self.final_accumulator_queries.len()
            + self
                .local_relation_queries
                .iter()
                .map(|proof| proof.authentication_query_count())
                .sum::<usize>()
    }

    fn serialized_value_count(&self) -> usize {
        self.local_relation_queries
            .iter()
            .map(|proof| proof.serialized_value_count())
            .sum()
    }

    fn verify_schedule(&self, schedule: &HolographicQuerySchedule) -> Result<(), Error> {
        if self.final_accumulator_queries.len() != schedule.raa_final_queries().len() {
            return Err(Error::InvalidPcsOpen(
                "RAA final accumulator query proof count does not match schedule".to_string(),
            ));
        }

        let mut local_relation_queries = self.local_relation_queries.iter();
        let mut local_authentication_queries =
            schedule.raa_auxiliary_authentication_queries().iter();
        for expected in schedule.raa_auxiliary_queries() {
            let proof = local_relation_queries.next().ok_or_else(|| {
                Error::InvalidPcsOpen("RAA auxiliary local relation proof is missing".to_string())
            })?;
            consume_expected_raa_auxiliary_authentication_queries(
                &mut local_authentication_queries,
                expected,
            )?;
            match (expected.relation, proof) {
                (
                    RaaAuxiliaryRelationKind::U2Repetition { .. }
                    | RaaAuxiliaryRelationKind::FirstAccumulatorStart { .. }
                    | RaaAuxiliaryRelationKind::SecondPermutation { .. },
                    RaaAuxiliaryLocalRelationProof::Equality,
                ) => {}
                (
                    RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. },
                    RaaAuxiliaryLocalRelationProof::FirstAccumulatorStep { .. },
                ) => {}
                _ => {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local relation proof shape does not match schedule"
                            .to_string(),
                    ));
                }
            }
        }
        if local_relation_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many RAA auxiliary local relation openings".to_string(),
            ));
        }
        if local_authentication_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many scheduled RAA auxiliary local authentication queries".to_string(),
            ));
        }
        Ok(())
    }

    fn authentication_queries(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        self.verify_schedule(schedule)?;
        let mut queries = Vec::with_capacity(self.query_count());

        for (expected, _) in schedule
            .raa_final_queries()
            .iter()
            .zip(self.final_accumulator_queries.iter())
        {
            let current = top_queries
                .get(expected.current_input_query)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen("missing current RAA input query".to_string())
                })?;
            let previous = top_queries
                .get(expected.previous_input_query)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen("missing previous RAA input query".to_string())
                })?;
            if current.index != expected.index || previous.index + 1 != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "RAA final accumulator input queries do not match scheduled neighboring columns"
                        .to_string(),
                ));
            }
            queries.push((expected.u4_auxiliary_index, current.value - previous.value));
        }

        self.push_local_relation_authentication_queries(relation_queries, schedule, &mut queries)?;
        Ok(queries)
    }

    fn authentication_queries_from_oracle<H: Hash>(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        oracle: &AuxiliaryOracleCommitment<H>,
    ) -> Result<Vec<(usize, B128)>, Error> {
        self.verify_schedule(schedule)?;
        let mut queries = Vec::with_capacity(self.query_count());
        for expected in schedule.raa_final_queries() {
            let u4 = oracle.query(expected.u4_auxiliary_index)?;
            queries.push((u4.logical_index, u4.value));
        }
        self.push_local_relation_authentication_queries(relation_queries, schedule, &mut queries)?;
        Ok(queries)
    }

    fn push_local_relation_authentication_queries(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        queries: &mut Vec<(usize, B128)>,
    ) -> Result<(), Error> {
        let mut local_relation_proofs = self.local_relation_queries.iter();
        let mut local_authentication_queries =
            schedule.raa_auxiliary_authentication_queries().iter();
        for expected in schedule.raa_auxiliary_queries() {
            let main = relation_queries
                .get(expected.sampled_auxiliary_ordinal)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA auxiliary local relation main opening is missing".to_string(),
                    )
                })?;
            let proof = local_relation_proofs.next().ok_or_else(|| {
                Error::InvalidPcsOpen("RAA auxiliary local relation proof is missing".to_string())
            })?;
            for query in proof.authentication_queries(expected, main.value)? {
                let scheduled = local_authentication_queries.next().ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA auxiliary local authentication query is missing from schedule"
                            .to_string(),
                    )
                })?;
                if scheduled.domain != BackendProofQueryDomain::RelationAuxiliary
                    || scheduled.index != query.0
                {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local authentication query does not match schedule"
                            .to_string(),
                    ));
                }
                queries.push(query);
            }
        }
        if local_relation_proofs.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many RAA auxiliary local relation openings".to_string(),
            ));
        }
        if local_authentication_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many scheduled RAA auxiliary local authentication queries".to_string(),
            ));
        }
        Ok(())
    }

    fn verify_openings(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        self.verify_schedule(schedule)?;
        self.verify_final_accumulator_openings(schedule, top_queries)?;
        self.verify_local_relation_openings(relation_queries, schedule)
    }

    fn verify_final_accumulator_openings(
        &self,
        schedule: &HolographicQuerySchedule,
        top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        for (relative_index, expected) in schedule.raa_final_queries().iter().enumerate() {
            let current = top_queries
                .get(expected.current_input_query)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen("missing current RAA input query".to_string())
                })?;
            let previous = top_queries
                .get(expected.previous_input_query)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen("missing previous RAA input query".to_string())
                })?;
            if current.index != expected.index || previous.index + 1 != expected.index {
                return Err(Error::InvalidPcsOpen(
                    "RAA final accumulator input queries do not match scheduled neighboring columns"
                        .to_string(),
                ));
            }
            self.final_accumulator_queries
                .get(relative_index)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA final accumulator opening triple is missing".to_string(),
                    )
                })?;
        }
        Ok(())
    }

    fn verify_local_relation_openings(
        &self,
        relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        let mut local_relation_queries = self.local_relation_queries.iter();
        for expected in schedule.raa_auxiliary_queries() {
            let main = relation_queries
                .get(expected.sampled_auxiliary_ordinal)
                .ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA auxiliary local relation main opening is missing".to_string(),
                    )
                })?;
            if main.logical_index != expected.main_auxiliary_index {
                return Err(Error::InvalidPcsOpen(
                    "RAA auxiliary local relation main index does not match schedule".to_string(),
                ));
            }

            match expected.relation {
                RaaAuxiliaryRelationKind::U2Repetition { .. }
                | RaaAuxiliaryRelationKind::FirstAccumulatorStart { .. }
                | RaaAuxiliaryRelationKind::SecondPermutation { .. } => {
                    let proof = local_relation_queries.next().ok_or_else(|| {
                        Error::InvalidPcsOpen(
                            "RAA auxiliary local relation opening is missing".to_string(),
                        )
                    })?;
                    if !matches!(proof, RaaAuxiliaryLocalRelationProof::Equality) {
                        return Err(Error::InvalidPcsOpen(
                            "RAA auxiliary local equality proof has the wrong shape".to_string(),
                        ));
                    }
                }
                RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. } => {
                    let proof = local_relation_queries.next().ok_or_else(|| {
                        Error::InvalidPcsOpen(
                            "RAA first accumulator previous opening is missing".to_string(),
                        )
                    })?;
                    if !matches!(
                        proof,
                        RaaAuxiliaryLocalRelationProof::FirstAccumulatorStep { .. }
                    ) {
                        return Err(Error::InvalidPcsOpen(
                            "RAA first accumulator proof has the wrong shape".to_string(),
                        ));
                    }
                }
            }
        }
        if local_relation_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many RAA auxiliary local relation openings".to_string(),
            ));
        }
        Ok(())
    }
}

impl RaaSection5RelationProof {
    fn query_count(&self) -> usize {
        0
    }

    fn serialized_value_count(&self) -> usize {
        self.permutation_sumcheck.serialized_value_count()
            + self.first_accumulator_sumcheck.serialized_value_count()
            + self.second_accumulator_sumcheck.serialized_value_count()
            + self.terminal_evaluations.len()
    }

    fn verify_schedule(&self, schedule: &HolographicQuerySchedule) -> Result<(), Error> {
        self.verify_schedule_shape(schedule)
    }

    fn verify_prequery<H: Hash>(
        &self,
        auxiliary_oracle_len: usize,
    ) -> Result<RaaSection5RelationChecks, Error> {
        let checks = self.verify_sumcheck_transcripts::<H>(auxiliary_oracle_len)?;
        if !self.terminal_evaluations.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "unbound serialized RAA Section 5 terminal evaluations are not accepted"
                    .to_string(),
            ));
        }
        Ok(checks)
    }

    fn verify_schedule_shape(&self, schedule: &HolographicQuerySchedule) -> Result<(), Error> {
        if schedule.raa_relation_strategy() != RaaRelationProofStrategy::Section5 {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 proof supplied for a non-Section 5 schedule".to_string(),
            ));
        }
        if !schedule.raa_auxiliary_queries().is_empty()
            || !schedule.raa_auxiliary_authentication_queries().is_empty()
            || !schedule.raa_final_queries().is_empty()
        {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 schedule must not carry local companion queries".to_string(),
            ));
        }
        Ok(())
    }

    fn authentication_queries(
        &self,
        _relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        _top_queries: &[TopQuery<B128>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        self.verify_schedule_shape(schedule)?;
        Ok(Vec::new())
    }

    fn authentication_queries_from_oracle<H: Hash>(
        &self,
        _relation_queries: &[AuxiliaryOracleQuery],
        schedule: &HolographicQuerySchedule,
        _oracle: &AuxiliaryOracleCommitment<H>,
    ) -> Result<Vec<(usize, B128)>, Error> {
        self.verify_schedule_shape(schedule)?;
        Ok(Vec::new())
    }

    fn verify_openings<H: Hash>(
        &self,
        _relation_queries: &[AuxiliaryOracleQuery],
        section5_relation_residual_public: Option<&RaaSection5RelationResidualPublicCommitment<H>>,
        section5_relation_residual: Option<&RaaSection5RelationResidualQueryProof<H>>,
        authentication: &BackendProofOracleAuthentication<H>,
        auxiliary_oracle_len: usize,
        schedule: &HolographicQuerySchedule,
        _top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        self.verify_schedule_shape(schedule)?;
        let checks = self.verify_prequery::<H>(auxiliary_oracle_len)?;
        let residual_public = section5_relation_residual_public.ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA Section 5 terminal binding requires a residual commitment".to_string(),
            )
        })?;
        let terminal_proof = section5_relation_residual
            .and_then(|proof| proof.terminal_proof.as_ref())
            .ok_or_else(|| {
                Error::InvalidPcsOpen("RAA Section 5 terminal binding proof is missing".to_string())
            })?;
        terminal_proof.verify(
            &checks,
            residual_public,
            schedule,
            &authentication.section5_relation_terminal_folded_layers,
            &authentication.section5_relation_terminal_layer_authentication,
        )?;
        verify_section5_relation_residual_openings(section5_relation_residual, schedule)
    }

    fn verify_sumcheck_transcripts<H: Hash>(
        &self,
        auxiliary_oracle_len: usize,
    ) -> Result<RaaSection5RelationChecks, Error> {
        if auxiliary_oracle_len % RAA_SECTION5_AUX_ROW_COUNT != 0 {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 auxiliary oracle length is not row-aligned".to_string(),
            ));
        }
        let domain_len = auxiliary_oracle_len / RAA_SECTION5_AUX_ROW_COUNT;
        if !domain_len.is_power_of_two() {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 sumcheck domain length must be a power of two".to_string(),
            ));
        }
        let num_vars = log2_strict(domain_len);
        let permutation = self.permutation_sumcheck.verify_transcript::<H>(
            num_vars,
            RAA_SECTION5_PERMUTATION_SUMCHECK_DEGREE,
            "permutation",
        )?;
        let first_accumulator = self.first_accumulator_sumcheck.verify_transcript::<H>(
            num_vars,
            RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE,
            "first accumulator",
        )?;
        let second_accumulator = self.second_accumulator_sumcheck.verify_transcript::<H>(
            num_vars,
            RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE,
            "second accumulator",
        )?;
        if permutation.initial_sum != B128::ZERO
            || first_accumulator.initial_sum != B128::ZERO
            || second_accumulator.initial_sum != B128::ZERO
        {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 sumcheck initial zero claim failed".to_string(),
            ));
        }
        Ok(RaaSection5RelationChecks {
            permutation,
            first_accumulator,
            second_accumulator,
        })
    }
}

impl RaaRelationSumcheckProof {
    fn serialized_value_count(&self) -> usize {
        self.round_polynomials.iter().map(Vec::len).sum()
    }

    fn verify_shape(&self, num_vars: usize, degree: usize, label: &str) -> Result<(), Error> {
        let expected_rounds = num_vars + 1;
        let expected_values = degree + 1;
        if self.round_polynomials.len() != expected_rounds {
            return Err(Error::InvalidPcsOpen(format!(
                "RAA Section 5 {label} sumcheck has {} rounds, expected {expected_rounds}",
                self.round_polynomials.len()
            )));
        }
        if self
            .round_polynomials
            .iter()
            .any(|round| round.len() != expected_values)
        {
            return Err(Error::InvalidPcsOpen(format!(
                "RAA Section 5 {label} sumcheck round width does not match degree {degree}",
            )));
        }
        Ok(())
    }

    fn verify_transcript<H: Hash>(
        &self,
        num_vars: usize,
        degree: usize,
        label: &str,
    ) -> Result<RaaRelationSumcheckCheck, Error> {
        self.verify_shape(num_vars, degree, label)?;
        let mut transcript = CfriTranscript::<H>::new();
        transcript.absorb("raa-section5-relation-sumcheck-v1");
        transcript.absorb(label);
        absorb_usize(&mut transcript, num_vars);
        absorb_usize(&mut transcript, degree);

        let mut challenges = Vec::with_capacity(num_vars);
        for round in 0..num_vars {
            absorb_raa_relation_sumcheck_round(
                &mut transcript,
                label,
                round,
                &self.round_polynomials[round],
            );
            challenges.push(transcript.squeeze());
        }
        absorb_raa_relation_sumcheck_round(
            &mut transcript,
            label,
            num_vars,
            &self.round_polynomials[num_vars],
        );

        let initial_sum = raa_relation_sumcheck_zero_plus_one(&self.round_polynomials[0]);
        for round in 0..num_vars {
            let next_sum = raa_relation_sumcheck_zero_plus_one(&self.round_polynomials[round + 1]);
            let current_at_challenge =
                raa_relation_sumcheck_evaluate(&self.round_polynomials[round], challenges[round]);
            if current_at_challenge != next_sum {
                return Err(Error::InvalidPcsOpen(format!(
                    "RAA Section 5 {label} sumcheck consistency check failed at round {round}"
                )));
            }
        }

        let terminal = self
            .round_polynomials
            .last()
            .expect("sumcheck shape checked");
        let terminal_claim = if num_vars == 0 {
            terminal[0]
        } else {
            raa_relation_sumcheck_evaluate(
                &self.round_polynomials[num_vars - 1],
                challenges[num_vars - 1],
            )
        };
        if terminal[0] != terminal_claim || terminal[1..].iter().any(|value| *value != B128::ZERO) {
            return Err(Error::InvalidPcsOpen(format!(
                "RAA Section 5 {label} sumcheck terminal claim check failed",
            )));
        }

        Ok(RaaRelationSumcheckCheck {
            challenges,
            initial_sum,
            terminal_claim,
        })
    }
}

impl RaaAuxiliaryLocalRelationProof {
    fn authentication_query_count(&self) -> usize {
        match self {
            Self::Equality => 1,
            Self::FirstAccumulatorStep { .. } => 2,
        }
    }

    fn serialized_value_count(&self) -> usize {
        match self {
            Self::Equality => 0,
            Self::FirstAccumulatorStep { .. } => 1,
        }
    }

    fn authentication_queries(
        &self,
        expected: &RaaAuxiliaryLocalQuery,
        main_value: B128,
    ) -> Result<Vec<(usize, B128)>, Error> {
        match (expected.relation, self) {
            (
                RaaAuxiliaryRelationKind::U2Repetition { .. }
                | RaaAuxiliaryRelationKind::FirstAccumulatorStart { .. }
                | RaaAuxiliaryRelationKind::SecondPermutation { .. },
                Self::Equality,
            ) => Ok(vec![(
                expected.extra_index(0).expect("single extra relation"),
                main_value,
            )]),
            (
                RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. },
                Self::FirstAccumulatorStep { previous_value },
            ) => Ok(vec![
                (
                    expected.extra_index(0).expect("previous u3"),
                    *previous_value,
                ),
                (
                    expected.extra_index(1).expect("u2"),
                    main_value - *previous_value,
                ),
            ]),
            _ => Err(Error::InvalidPcsOpen(
                "RAA auxiliary local relation proof shape does not match schedule".to_string(),
            )),
        }
    }
}

pub fn fold_systematic_pair<F: Field>(left: F, right: F, alpha: F) -> F {
    (F::ONE - alpha) * left + alpha * right
}

pub fn fold_rfc_parity_pair<F: Field>(left: F, right: F, alpha: F) -> F {
    left + alpha * right
}

pub fn absorb_systematic_foldable_code_spec<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    spec: &SystematicFoldableCodeSpec,
) {
    transcript.absorb("systematic-augmented-rfc-code-v1");
    absorb_usize(transcript, spec.version as usize);
    absorb_usize(transcript, spec.compiler_message_len);
    absorb_usize(transcript, spec.compiler_systematic_len);
    absorb_usize(transcript, spec.compiler_parity_len);
    absorb_usize(transcript, spec.compiler_codeword_len);
    absorb_usize(transcript, spec.parity_expansion_factor);
    transcript.absorb(&spec.seed);
}

pub fn absorb_blaze2_basefold_backend_spec<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    spec: &Blaze2BaseFoldBackendSpec,
) {
    transcript.absorb("blaze2-basefold-backend-spec-v1");
    absorb_blaze2_code_spec(transcript, &spec.praa);
    absorb_systematic_foldable_code_spec(transcript, &spec.compiler_code);
    absorb_usize(transcript, spec.q_raa_input);
    absorb_usize(transcript, spec.q_backend_proof);
    absorb_usize(transcript, spec.auxiliary_oracle_len);
    absorb_raa_relation_proof_strategy(transcript, spec.raa_relation_strategy);
}

pub fn absorb_blaze2_basefold_open_request<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) {
    transcript.absorb("blaze2-basefold-open-request-v1");
    absorb_usize(transcript, request.col_point.len());
    transcript.absorb_slice(request.col_point);
    transcript.absorb(&request.folded_eval);
}

pub fn absorb_compiler_parity_public_commitment<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    public: &CompilerParityPublicCommitment<H>,
) {
    transcript.absorb("compiler-parity-public-commitment-v1");
    absorb_usize(transcript, public.len);
    transcript.absorb(&public.root);
}

pub fn absorb_blaze2_basefold_prequery_public<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    prequery: &Blaze2BaseFoldPrequeryPublic<H>,
) {
    transcript.absorb("blaze2-basefold-prequery-public-v1");
    absorb_compiler_parity_public_commitment(transcript, &prequery.compiler_parity);
    match &prequery.eval_sumcheck {
        Some(proof) => {
            transcript.absorb("raa-eval-sumcheck-present");
            absorb_raa_eval_sumcheck_proof(transcript, proof);
        }
        None => transcript.absorb("raa-eval-sumcheck-absent"),
    }
    absorb_usize(transcript, prequery.folded_parity_layers.len());
    for (round, public) in prequery.folded_parity_layers.iter().enumerate() {
        absorb_folded_parity_public_commitment(transcript, round + 1, public);
    }
    transcript.absorb("terminal-codeword-v1");
    absorb_usize(transcript, prequery.terminal_codeword.len());
    for value in &prequery.terminal_codeword {
        transcript.absorb(value);
    }
    match &prequery.auxiliary {
        Some(auxiliary) => {
            transcript.absorb("auxiliary-oracle-present");
            absorb_auxiliary_oracle_public_commitment(transcript, auxiliary);
        }
        None => transcript.absorb("auxiliary-oracle-absent"),
    }
    if let Some(residual) = &prequery.section5_relation_residual {
        transcript.absorb("raa-section5-relation-residual-present");
        absorb_section5_relation_residual_public_commitment(transcript, residual);
    }
    if let Some(helper) = &prequery.section5_permutation_helper {
        transcript.absorb("raa-section5-permutation-helper-present");
        absorb_section5_permutation_helper_public_commitment(transcript, helper);
    }
    if let Some(proof) = &prequery.section5_relation {
        transcript.absorb("raa-section5-relation-present");
        absorb_raa_section5_relation_proof(transcript, proof);
    }
}

fn absorb_raa_eval_sumcheck_proof<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    proof: &RaaEvalSumcheckProof,
) {
    transcript.absorb("raa-eval-sumcheck-proof-v1");
    absorb_usize(transcript, proof.round_polynomials.len());
    for (round, coeffs) in proof.round_polynomials.iter().enumerate() {
        absorb_raa_eval_sumcheck_round(transcript, round, coeffs);
    }
}

fn absorb_raa_eval_sumcheck_round<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    round: usize,
    coeffs: &[B128; 3],
) {
    transcript.absorb("raa-eval-sumcheck-round-v1");
    absorb_usize(transcript, round);
    for coeff in coeffs {
        transcript.absorb(coeff);
    }
}

fn absorb_raa_section5_relation_proof<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    proof: &RaaSection5RelationProof,
) {
    transcript.absorb("raa-section5-relation-proof-v1");
    absorb_raa_relation_sumcheck_proof(transcript, "permutation", &proof.permutation_sumcheck);
    absorb_raa_relation_sumcheck_proof(
        transcript,
        "first accumulator",
        &proof.first_accumulator_sumcheck,
    );
    absorb_raa_relation_sumcheck_proof(
        transcript,
        "second accumulator",
        &proof.second_accumulator_sumcheck,
    );
    absorb_usize(transcript, proof.terminal_evaluations.len());
    for value in &proof.terminal_evaluations {
        transcript.absorb(value);
    }
}

fn absorb_raa_relation_sumcheck_proof<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    label: &str,
    proof: &RaaRelationSumcheckProof,
) {
    transcript.absorb("raa-section5-relation-sumcheck-proof-v1");
    transcript.absorb(label);
    absorb_usize(transcript, proof.round_polynomials.len());
    for (round, coeffs) in proof.round_polynomials.iter().enumerate() {
        absorb_raa_relation_sumcheck_round(transcript, label, round, coeffs);
    }
}

fn absorb_blaze2_basefold_fold_chain_prefix<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    spec: &Blaze2BaseFoldBackendSpec,
    compiler_parity: &CompilerParityPublicCommitment<H>,
    auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) {
    transcript.absorb("blaze2-basefold-fold-chain-v1");
    absorb_blaze2_basefold_backend_spec(transcript, spec);
    absorb_blaze2_basefold_open_request(transcript, request);
    absorb_compiler_parity_public_commitment(transcript, compiler_parity);
    match auxiliary {
        Some(auxiliary) => {
            transcript.absorb("auxiliary-oracle-present");
            absorb_auxiliary_oracle_public_commitment(transcript, auxiliary);
        }
        None => transcript.absorb("auxiliary-oracle-absent"),
    }
}

fn squeeze_raa_section5_relation_challenges<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    compiler_parity: &CompilerParityPublicCommitment<H>,
    auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) -> RaaSection5RelationChallenges {
    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_basefold_fold_chain_prefix(
        &mut transcript,
        spec,
        compiler_parity,
        auxiliary,
        request,
    );
    transcript.absorb("raa-section5-relation-challenges-v1");
    RaaSection5RelationChallenges {
        alpha: transcript.squeeze(),
        beta: transcript.squeeze(),
        gamma: transcript.squeeze(),
    }
}

fn absorb_raa_relation_proof_strategy<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    strategy: RaaRelationProofStrategy,
) {
    if strategy == RaaRelationProofStrategy::Section5 {
        transcript.absorb("raa-relation-proof-strategy-v1");
        absorb_usize(transcript, 1);
    }
}

fn absorb_folded_parity_public_commitment<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    round: usize,
    public: &CompilerParityPublicCommitment<H>,
) {
    transcript.absorb("folded-parity-public-commitment-v1");
    absorb_usize(transcript, round);
    absorb_compiler_parity_public_commitment(transcript, public);
}

pub fn absorb_auxiliary_oracle_public_commitment<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    public: &AuxiliaryOraclePublicCommitment<H>,
) {
    transcript.absorb("auxiliary-oracle-public-commitment-v1");
    absorb_usize(transcript, public.len);
    transcript.absorb(&public.root);
}

pub fn absorb_section5_permutation_helper_public_commitment<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    public: &RaaSection5PermutationHelperPublicCommitment<H>,
) {
    transcript.absorb("raa-section5-permutation-helper-public-commitment-v1");
    absorb_usize(transcript, public.len);
    transcript.absorb(&public.root);
}

pub fn absorb_section5_relation_residual_public_commitment<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    public: &RaaSection5RelationResidualPublicCommitment<H>,
) {
    transcript.absorb("raa-section5-relation-residual-public-commitment-v1");
    absorb_usize(transcript, public.len);
    transcript.absorb(&public.root);
}

impl HolographicQuerySchedule {
    pub fn sample<H: Hash, S>(
        transcript: &mut CfriTranscript<H, S>,
        layout: &SystematicAugmentedRfcLayout,
        spec: HolographicQueryScheduleSpec,
    ) -> Result<Self, Error> {
        validate_query_schedule_spec(layout, &spec)?;
        transcript.absorb("systematic-basefold-query-schedule-v1");
        absorb_usize(transcript, spec.q_raa_input);
        absorb_usize(transcript, spec.q_backend_proof);
        absorb_usize(transcript, spec.auxiliary_oracle_len);
        absorb_raa_relation_proof_strategy(transcript, spec.raa_relation_strategy);

        let mut input_queries = Vec::with_capacity(spec.q_raa_input);
        let mut raa_final_queries = Vec::new();
        if spec.auxiliary_oracle_len == 0
            || spec.raa_relation_strategy == RaaRelationProofStrategy::Section5
        {
            for _ in 0..spec.q_raa_input {
                let logical_index = squeeze_bounded_index(transcript, layout.systematic_len())?;
                input_queries.push(SystematicInputQuery {
                    logical_index,
                    physical_index: layout.systematic_to_physical(logical_index)?,
                });
            }
        } else {
            let spot_count = spec.q_raa_input >> 1;
            raa_final_queries.reserve(spot_count);
            let len = layout.systematic_len();
            let u4_offset = raa_auxiliary_index(RAA_AUX_U4_ROW, 0, len);
            for spot in 0..spot_count {
                let index = if spot == 0 {
                    len - 1
                } else {
                    squeeze_bounded_index(transcript, len - 1)? + 1
                };
                let current_input_query = input_queries.len();
                input_queries.push(SystematicInputQuery {
                    logical_index: index,
                    physical_index: layout.systematic_to_physical(index)?,
                });
                let previous_input_query = input_queries.len();
                input_queries.push(SystematicInputQuery {
                    logical_index: index - 1,
                    physical_index: layout.systematic_to_physical(index - 1)?,
                });
                raa_final_queries.push(RaaFinalAccumulatorQuery {
                    index,
                    current_input_query,
                    previous_input_query,
                    u4_auxiliary_index: u4_offset + index,
                });
            }
        }

        let auxiliary_proof_len = raa_relation_auxiliary_len(spec.auxiliary_oracle_len);
        let section5_residual_len = section5_relation_residual_len_for_schedule(layout, &spec);
        let section5_helper_len = section5_permutation_helper_len_for_schedule(layout, &spec);
        let proof_domain_len =
            layout.parity_len() + auxiliary_proof_len + section5_residual_len + section5_helper_len;
        let mut proof_queries = Vec::with_capacity(spec.q_backend_proof);
        if spec.q_backend_proof != 0 {
            let parity_index = squeeze_bounded_index(transcript, layout.parity_len())?;
            proof_queries.push(BackendProofQuery {
                domain: BackendProofQueryDomain::CompilerParity,
                index: parity_index,
                physical_index: Some(layout.parity_to_physical(parity_index)?),
            });
        }
        for _ in proof_queries.len()..spec.q_backend_proof {
            let sampled_index = squeeze_bounded_index(transcript, proof_domain_len)?;
            proof_queries.push(proof_query_from_sampled_index(
                layout,
                spec.auxiliary_oracle_len,
                section5_residual_len,
                section5_helper_len,
                sampled_index,
            )?);
        }

        Ok(Self {
            input_queries,
            proof_queries,
            raa_relation_strategy: spec.raa_relation_strategy,
            raa_final_queries,
            raa_auxiliary_queries: Vec::new(),
            raa_auxiliary_authentication_queries: Vec::new(),
        })
    }

    pub fn input_queries(&self) -> &[SystematicInputQuery] {
        &self.input_queries
    }

    pub fn proof_queries(&self) -> &[BackendProofQuery] {
        &self.proof_queries
    }

    pub fn raa_final_queries(&self) -> &[RaaFinalAccumulatorQuery] {
        &self.raa_final_queries
    }

    pub fn raa_relation_strategy(&self) -> RaaRelationProofStrategy {
        self.raa_relation_strategy
    }

    pub fn raa_auxiliary_queries(&self) -> &[RaaAuxiliaryLocalQuery] {
        &self.raa_auxiliary_queries
    }

    pub fn raa_auxiliary_authentication_queries(&self) -> &[BackendProofQuery] {
        &self.raa_auxiliary_authentication_queries
    }

    pub fn raa_auxiliary_authentication_query_count(&self) -> usize {
        self.raa_auxiliary_authentication_queries.len()
    }

    pub fn relation_auxiliary_proof_query_count(&self) -> usize {
        self.proof_queries
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::RelationAuxiliary)
            .count()
    }

    pub fn section5_permutation_helper_proof_query_count(&self) -> usize {
        self.proof_queries
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::Section5PermutationHelper)
            .count()
    }

    pub fn section5_relation_residual_proof_query_count(&self) -> usize {
        self.proof_queries
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::Section5RelationResidual)
            .count()
    }

    pub fn expected_auxiliary_query_proof_count(&self) -> usize {
        match self.raa_relation_strategy {
            RaaRelationProofStrategy::LocalQueries => {
                self.relation_auxiliary_proof_query_count()
                    + self.raa_final_queries.len()
                    + self.raa_auxiliary_authentication_queries.len()
            }
            RaaRelationProofStrategy::Section5 => self.relation_auxiliary_proof_query_count(),
        }
    }

    pub fn attach_raa_auxiliary_local_queries(
        &mut self,
        code: &PackedRaaCode,
    ) -> Result<(), Error> {
        if self.raa_relation_strategy != RaaRelationProofStrategy::LocalQueries {
            return Err(Error::InvalidPcsParam(
                "RAA local auxiliary queries cannot be attached to a non-local relation strategy"
                    .to_string(),
            ));
        }
        self.raa_auxiliary_queries = build_raa_auxiliary_local_queries(code, &self.proof_queries)?;
        self.raa_auxiliary_authentication_queries =
            build_raa_auxiliary_authentication_queries(&self.raa_auxiliary_queries)?;
        Ok(())
    }

    pub fn validate_top_queries<T>(&self, top_queries: &[TopQuery<T>]) -> Result<(), Error> {
        if top_queries.len() != self.input_queries.len() {
            return Err(Error::InvalidPcsOpen(format!(
                "top query count {} does not match systematic input query count {}",
                top_queries.len(),
                self.input_queries.len()
            )));
        }
        for (expected, actual) in self.input_queries.iter().zip(top_queries) {
            if actual.index != expected.logical_index {
                return Err(Error::InvalidPcsOpen(
                    "top query index does not match transcript schedule".to_string(),
                ));
            }
        }
        Ok(())
    }
}

fn validate_spec(spec: &SystematicFoldableCodeSpec) -> Result<(), Error> {
    if spec.version == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic foldable code version must be nonzero".to_string(),
        ));
    }
    if spec.compiler_message_len == 0 || !spec.compiler_message_len.is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "compiler message length must be a nonzero power of two".to_string(),
        ));
    }
    if spec.compiler_systematic_len != spec.compiler_message_len {
        return Err(Error::InvalidPcsParam(
            "compiler systematic length must equal compiler message length".to_string(),
        ));
    }
    if spec.parity_expansion_factor == 0 {
        return Err(Error::InvalidPcsParam(
            "parity expansion factor must be nonzero".to_string(),
        ));
    }
    if !(spec.parity_expansion_factor + 1).is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "parity expansion plus one must be a power of two".to_string(),
        ));
    }
    if spec.compiler_parity_len != spec.compiler_message_len * spec.parity_expansion_factor {
        return Err(Error::InvalidPcsParam(
            "compiler parity length does not match message length times parity expansion"
                .to_string(),
        ));
    }
    if spec.compiler_codeword_len != spec.compiler_systematic_len + spec.compiler_parity_len {
        return Err(Error::InvalidPcsParam(
            "compiler codeword length does not match systematic plus parity lengths".to_string(),
        ));
    }
    if !spec.compiler_codeword_len.is_power_of_two() {
        return Err(Error::InvalidPcsParam(
            "compiler codeword length must be a power of two".to_string(),
        ));
    }
    Ok(())
}

fn validate_blaze2_backend_spec(spec: &Blaze2BaseFoldBackendSpec) -> Result<(), Error> {
    if spec.q_raa_input == 0 {
        return Err(Error::InvalidPcsParam(
            "Blaze2 BaseFold backend needs at least one RAA input query".to_string(),
        ));
    }
    if spec.q_backend_proof == 0 {
        return Err(Error::InvalidPcsParam(
            "Blaze2 BaseFold backend needs at least one backend proof query".to_string(),
        ));
    }
    let _ = Blaze2Code::new(spec.praa.clone())?;
    let compiler_layout = SystematicAugmentedRfcLayout::new(spec.compiler_code.clone())?;
    if compiler_layout.message_len() != spec.praa.praa_codeword_len {
        return Err(Error::InvalidPcsParam(
            "compiler message length must equal PRAA codeword length".to_string(),
        ));
    }
    if compiler_layout.systematic_len() != spec.praa.praa_codeword_len {
        return Err(Error::InvalidPcsParam(
            "compiler systematic length must equal PRAA codeword length".to_string(),
        ));
    }
    let expected_relation_auxiliary_len =
        required_blaze2_basefold_relation_auxiliary_len(&spec.praa);
    let expected_strategy_auxiliary_len =
        required_blaze2_basefold_auxiliary_oracle_len_for_strategy(
            &spec.praa,
            spec.raa_relation_strategy,
        );
    let expected_eval_binding_len = required_blaze2_basefold_eval_binding_len(&spec.praa);
    let expected_auxiliary_len = expected_strategy_auxiliary_len + expected_eval_binding_len;
    if spec.auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsParam(format!(
            "Blaze2 BaseFold backend needs {expected_relation_auxiliary_len} relation auxiliary entries"
        )));
    }
    if spec.auxiliary_oracle_len != expected_auxiliary_len {
        return Err(Error::InvalidPcsParam(format!(
            "auxiliary oracle length must be {expected_auxiliary_len}: {expected_relation_auxiliary_len} RAA relation entries plus strategy proof-oracle entries"
        )));
    }
    if spec.raa_relation_strategy == RaaRelationProofStrategy::LocalQueries
        && spec.q_raa_input & 1 != 0
    {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled Blaze2 BaseFold backend needs an even RAA input query count"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_section5_permutation_helper_public<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    prequery: &Blaze2BaseFoldPrequeryPublic<H>,
) -> Result<(), Error> {
    match (
        spec.raa_relation_strategy,
        &prequery.section5_relation_residual,
        &prequery.section5_permutation_helper,
        &prequery.section5_relation,
    ) {
        (RaaRelationProofStrategy::LocalQueries, None, None, None) => Ok(()),
        (RaaRelationProofStrategy::LocalQueries, Some(_), _, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual commitment was supplied for local relation mode".to_string(),
        )),
        (RaaRelationProofStrategy::LocalQueries, None, Some(_), _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper commitment was supplied for local relation mode".to_string(),
        )),
        (RaaRelationProofStrategy::LocalQueries, None, None, Some(_)) => {
            Err(Error::InvalidPcsOpen(
                "RAA Section 5 relation proof was supplied for local relation mode".to_string(),
            ))
        }
        (RaaRelationProofStrategy::Section5, None, _, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual commitment is missing".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(_), None, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper commitment is missing".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(_), Some(_), None) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 relation proof is missing from prequery".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(residual), Some(helper), Some(relation)) => {
            let expected_residual_len =
                spec.praa.praa_codeword_len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT;
            if residual.len != expected_residual_len {
                return Err(Error::InvalidPcsOpen(format!(
                    "RAA Section 5 residual commitment has length {}, expected {expected_residual_len}",
                    residual.len
                )));
            }
            let expected_len =
                required_blaze2_basefold_section5_permutation_helper_oracle_len(&spec.praa);
            if helper.len != expected_len {
                return Err(Error::InvalidPcsOpen(format!(
                    "RAA Section 5 helper commitment has length {}, expected {expected_len}",
                    helper.len
                )));
            }
            relation.verify_prequery::<H>(spec.auxiliary_oracle_len)?;
            Ok(())
        }
    }
}

fn validate_section5_permutation_helper_state<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    state: &Blaze2BaseFoldProverState<H>,
) -> Result<(), Error> {
    match (
        spec.raa_relation_strategy,
        &state.section5_relation_residual,
        &state.section5_permutation_helper,
        &state.section5_relation,
    ) {
        (RaaRelationProofStrategy::LocalQueries, None, None, None) => Ok(()),
        (RaaRelationProofStrategy::LocalQueries, Some(_), _, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual oracle was built for local relation mode".to_string(),
        )),
        (RaaRelationProofStrategy::LocalQueries, None, Some(_), _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle was built for local relation mode".to_string(),
        )),
        (RaaRelationProofStrategy::LocalQueries, None, None, Some(_)) => {
            Err(Error::InvalidPcsOpen(
                "RAA Section 5 relation proof was built for local relation mode".to_string(),
            ))
        }
        (RaaRelationProofStrategy::Section5, None, _, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual oracle is missing from prover state".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(_), None, _) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle is missing from prover state".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(_), Some(_), None) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 relation proof is missing from prover state".to_string(),
        )),
        (RaaRelationProofStrategy::Section5, Some(residual), Some(helper), Some(relation)) => {
            let expected_residual_len =
                spec.praa.praa_codeword_len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT;
            if residual.len() != expected_residual_len {
                return Err(Error::InvalidPcsOpen(format!(
                    "RAA Section 5 residual oracle has length {}, expected {expected_residual_len}",
                    residual.len()
                )));
            }
            let expected_len =
                required_blaze2_basefold_section5_permutation_helper_oracle_len(&spec.praa);
            if helper.len() != expected_len {
                return Err(Error::InvalidPcsOpen(format!(
                    "RAA Section 5 helper oracle has length {}, expected {expected_len}",
                    helper.len()
                )));
            }
            relation.verify_prequery::<H>(spec.auxiliary_oracle_len)?;
            Ok(())
        }
    }
}

fn validate_message_and_parity_output_at_round(
    layout: &SystematicAugmentedRfcLayout,
    round: usize,
    message: &[B128],
    out: &[B128],
) -> Result<(), Error> {
    let message_len = layout.message_len_at_round(round)?;
    let parity_len = message_len * layout.parity_expansion_factor();
    if message.len() != message_len {
        return Err(Error::InvalidPcsOpen(format!(
            "systematic RFC message has length {}, expected {message_len}",
            message.len()
        )));
    }
    if out.len() != parity_len {
        return Err(Error::InvalidPcsOpen(format!(
            "systematic RFC parity output has length {}, expected {parity_len}",
            out.len()
        )));
    }
    Ok(())
}

fn validate_blaze2_basefold_open_request(
    params: &Blaze2BaseFoldBackendParams,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) -> Result<(), Error> {
    let expected_col_point_len = log2_strict(params.praa.packed().message_len());
    if request.col_point.len() != expected_col_point_len {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze2 BaseFold opening column point has {} coordinates, expected {expected_col_point_len}",
            request.col_point.len()
        )));
    }
    Ok(())
}

fn validate_auxiliary_oracle_shape(expected_len: usize, values: &[B128]) -> Result<(), Error> {
    if values.len() != expected_len {
        return Err(Error::InvalidPcsOpen(format!(
            "auxiliary oracle has length {}, expected {expected_len}",
            values.len()
        )));
    }
    Ok(())
}

fn validate_raa_relation_auxiliary_consistent_with_codeword(
    code: &PackedRaaCode,
    codeword: &[B128],
    auxiliary_oracle: &[B128],
) -> Result<(), Error> {
    if auxiliary_oracle.is_empty() {
        return Ok(());
    }
    let len = code.codeword_len();
    let relation_len = RAA_AUX_ROW_COUNT * len;
    if codeword.len() != len
        || auxiliary_oracle.len() < relation_len
        || auxiliary_oracle.len() % len != 0
    {
        return Err(Error::InvalidPcsOpen(
            "RAA relation auxiliary oracle shape does not match folded codeword".to_string(),
        ));
    }

    let relation_auxiliary = &auxiliary_oracle[..relation_len];
    let (u2, rest) = relation_auxiliary.split_at(len);
    let (u3, u4) = rest.split_at(len);
    let permutation = code.permutation();

    let mut repetition_values = vec![None; code.message_len()];
    for index in 0..len {
        let source = permutation.permutation1[index] / code.rate();
        let slot = repetition_values.get_mut(source).ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA relation auxiliary u2 row uses an invalid repetition source".to_string(),
            )
        })?;
        match slot {
            Some(expected) if *expected != u2[index] => {
                return Err(Error::InvalidPcsOpen(
                    "RAA relation auxiliary u2 row does not match the repetition layer".to_string(),
                ));
            }
            Some(_) => {}
            None => *slot = Some(u2[index]),
        }
    }
    if repetition_values.iter().any(Option::is_none) {
        return Err(Error::InvalidPcsOpen(
            "RAA relation auxiliary u2 row does not cover every message coordinate".to_string(),
        ));
    }

    let mut accumulator = B128::ZERO;
    for index in 0..len {
        accumulator += u2[index];
        if u3[index] != accumulator {
            return Err(Error::InvalidPcsOpen(
                "RAA relation auxiliary u3 row does not match first accumulator".to_string(),
            ));
        }
    }

    for index in 0..len {
        if u4[index] != u3[permutation.permutation2[index]] {
            return Err(Error::InvalidPcsOpen(
                "RAA relation auxiliary u4 row does not match second permutation".to_string(),
            ));
        }
    }

    accumulator = B128::ZERO;
    for index in 0..len {
        accumulator += u4[index];
        if codeword[index] != accumulator {
            return Err(Error::InvalidPcsOpen(
                "RAA relation auxiliary u4 row does not accumulate to the folded codeword"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

fn raa_auxiliary_rows(
    auxiliary_oracle: &[B128],
    len: usize,
) -> Result<(&[B128], &[B128], &[B128]), Error> {
    let relation_len = RAA_AUX_ROW_COUNT * len;
    if len == 0 || auxiliary_oracle.len() < relation_len {
        return Err(Error::InvalidPcsOpen(
            "RAA relation auxiliary rows are missing".to_string(),
        ));
    }
    let u2 = &auxiliary_oracle
        [raa_auxiliary_index(RAA_AUX_U2_ROW, 0, len)..raa_auxiliary_index(RAA_AUX_U3_ROW, 0, len)];
    let u3 = &auxiliary_oracle
        [raa_auxiliary_index(RAA_AUX_U3_ROW, 0, len)..raa_auxiliary_index(RAA_AUX_U4_ROW, 0, len)];
    let u4 = &auxiliary_oracle[raa_auxiliary_index(RAA_AUX_U4_ROW, 0, len)..relation_len];
    Ok((u2, u3, u4))
}

fn prove_raa_section5_relation_proof(
    code: &PackedRaaCode,
    auxiliary_oracle: &[B128],
    helper_values: &[B128],
    residual_values: &[B128],
    folded_codeword: &[B128],
    challenges: RaaSection5RelationChallenges,
) -> Result<RaaSection5RelationProof, Error> {
    let len = code.codeword_len();
    if folded_codeword.len() != len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 relation proof codeword length does not match PRAA code length"
                .to_string(),
        ));
    }
    validate_raa_relation_auxiliary_consistent_with_codeword(
        code,
        folded_codeword,
        auxiliary_oracle,
    )?;
    let expected_helper =
        build_raa_section5_permutation_helper_oracle(code, auxiliary_oracle, challenges)?;
    if helper_values != expected_helper {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle is not consistent with relation challenges".to_string(),
        ));
    }
    let residuals = build_raa_section5_relation_residual_oracle(
        code,
        auxiliary_oracle,
        folded_codeword,
        helper_values,
        challenges,
    )?;
    if residual_values != residuals {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual oracle is not consistent with relation challenges".to_string(),
        ));
    }
    let len = code.codeword_len();

    Ok(RaaSection5RelationProof {
        permutation_sumcheck: prove_raa_relation_zero_sumcheck(
            &residuals[0..len],
            RAA_SECTION5_PERMUTATION_SUMCHECK_DEGREE,
            "permutation",
        )?,
        first_accumulator_sumcheck: prove_raa_relation_zero_sumcheck(
            &residuals[len..2 * len],
            RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE,
            "first accumulator",
        )?,
        second_accumulator_sumcheck: prove_raa_relation_zero_sumcheck(
            &residuals[2 * len..3 * len],
            RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE,
            "second accumulator",
        )?,
        terminal_evaluations: Vec::new(),
    })
}

fn build_raa_section5_relation_residual_oracle(
    code: &PackedRaaCode,
    auxiliary_oracle: &[B128],
    folded_codeword: &[B128],
    helper_values: &[B128],
    challenges: RaaSection5RelationChallenges,
) -> Result<Vec<B128>, Error> {
    let len = code.codeword_len();
    if folded_codeword.len() != len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual oracle codeword length does not match PRAA code length"
                .to_string(),
        ));
    }
    let (u2, u3, u4) = raa_auxiliary_rows(auxiliary_oracle, len)?;

    let permutation_residuals =
        build_raa_section5_permutation_residuals(len, helper_values, challenges.gamma)?;
    let mut first_accumulator_residuals = Vec::with_capacity(len);
    for index in 0..len {
        let previous = if index == 0 {
            B128::ZERO
        } else {
            u3[index - 1]
        };
        first_accumulator_residuals.push(u3[index] - previous - u2[index]);
    }

    let mut second_accumulator_residuals = Vec::with_capacity(len);
    let mut accumulator = B128::ZERO;
    for index in 0..len {
        accumulator += u4[index];
        second_accumulator_residuals.push(folded_codeword[index] - accumulator);
    }

    let mut residuals = Vec::with_capacity(len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT);
    residuals.extend(permutation_residuals);
    residuals.extend(first_accumulator_residuals);
    residuals.extend(second_accumulator_residuals);
    Ok(residuals)
}

fn build_raa_section5_permutation_residuals(
    len: usize,
    helper_values: &[B128],
    gamma: B128,
) -> Result<Vec<B128>, Error> {
    if len == 0 || !len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 permutation residual domain must be a non-empty power of two"
                .to_string(),
        ));
    }
    let tree_chunk_len = len << 1;
    if helper_values.len() != tree_chunk_len * 4 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle length does not match four packed product trees"
                .to_string(),
        ));
    }

    let f1 = &helper_values[0..tree_chunk_len];
    let g1 = &helper_values[tree_chunk_len..2 * tree_chunk_len];
    let f2 = &helper_values[2 * tree_chunk_len..3 * tree_chunk_len];
    let g2 = &helper_values[3 * tree_chunk_len..4 * tree_chunk_len];
    let mut f1_residuals = product_tree_packed_relation_residuals(f1)?;
    let g1_residuals = product_tree_packed_relation_residuals(g1)?;
    let mut f2_residuals = product_tree_packed_relation_residuals(f2)?;
    let g2_residuals = product_tree_packed_relation_residuals(g2)?;

    f1_residuals[len - 1] = product_tree_packed_root(f1)? - product_tree_packed_root(g1)?;
    f2_residuals[len - 1] = product_tree_packed_root(f2)? - product_tree_packed_root(g2)?;

    let mut residuals = vec![B128::ZERO; len];
    let mut coefficient = B128::ONE;
    for relation in [&f1_residuals, &g1_residuals, &f2_residuals, &g2_residuals] {
        for (out, value) in residuals.iter_mut().zip(relation.iter()) {
            *out += coefficient * *value;
        }
        coefficient *= gamma;
    }
    Ok(residuals)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RaaSection5PermutationResidualLocalQuery {
    residual: B128,
    helper_openings: Vec<(usize, B128)>,
}

fn raa_section5_permutation_residual_local_query(
    len: usize,
    helper_values: &[B128],
    gamma: B128,
    residual_index: usize,
) -> Result<RaaSection5PermutationResidualLocalQuery, Error> {
    if len == 0 || !len.is_power_of_two() || residual_index >= len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 permutation residual local query index is invalid".to_string(),
        ));
    }
    let tree_chunk_len = len << 1;
    if helper_values.len() != tree_chunk_len * 4 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle length does not match four packed product trees"
                .to_string(),
        ));
    }
    let f1_offset = 0;
    let g1_offset = tree_chunk_len;
    let f2_offset = tree_chunk_len * 2;
    let g2_offset = tree_chunk_len * 3;
    let chunks = [
        (&helper_values[f1_offset..g1_offset], f1_offset),
        (&helper_values[g1_offset..f2_offset], g1_offset),
        (&helper_values[f2_offset..g2_offset], f2_offset),
        (&helper_values[g2_offset..], g2_offset),
    ];

    if residual_index + 1 == len {
        let root_index = product_tree_packed_root_index(len);
        let padding_index = tree_chunk_len - 1;
        let f1_root = chunks[0].0[root_index];
        let g1_root = chunks[1].0[root_index];
        let g1_padding = chunks[1].0[padding_index];
        let f2_root = chunks[2].0[root_index];
        let g2_root = chunks[3].0[root_index];
        let g2_padding = chunks[3].0[padding_index];
        let gamma2 = gamma * gamma;
        let gamma3 = gamma2 * gamma;
        let residual = f1_root - g1_root
            + gamma * g1_padding
            + gamma2 * (f2_root - g2_root)
            + gamma3 * g2_padding;
        let helper_openings = vec![
            (chunks[0].1 + root_index, f1_root),
            (chunks[1].1 + root_index, g1_root),
            (chunks[1].1 + padding_index, g1_padding),
            (chunks[2].1 + root_index, f2_root),
            (chunks[3].1 + root_index, g2_root),
            (chunks[3].1 + padding_index, g2_padding),
        ];
        return Ok(RaaSection5PermutationResidualLocalQuery {
            residual,
            helper_openings,
        });
    }

    let mut residual = B128::ZERO;
    let mut coefficient = B128::ONE;
    let mut helper_openings = Vec::with_capacity(12);
    for (chunk, offset) in chunks {
        let query = product_tree_packed_relation_local_query(chunk, residual_index)?;
        residual += coefficient * query.residual;
        helper_openings.extend(
            query
                .openings
                .into_iter()
                .map(|(index, value)| (offset + index, value)),
        );
        coefficient *= gamma;
    }
    Ok(RaaSection5PermutationResidualLocalQuery {
        residual,
        helper_openings,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProductTreePackedRelationLocalQuery {
    residual: B128,
    openings: Vec<(usize, B128)>,
}

fn product_tree_packed_relation_local_query(
    packed: &[B128],
    relation_index: usize,
) -> Result<ProductTreePackedRelationLocalQuery, Error> {
    if packed.len() < 2 || packed.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 packed product-tree witness has invalid length".to_string(),
        ));
    }
    let leaf_len = packed.len() >> 1;
    if !leaf_len.is_power_of_two() || relation_index >= leaf_len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 product-tree relation query index is invalid".to_string(),
        ));
    }
    if relation_index + 1 == leaf_len {
        let padding_index = packed.len() - 1;
        return Ok(ProductTreePackedRelationLocalQuery {
            residual: packed[padding_index],
            openings: vec![(padding_index, packed[padding_index])],
        });
    }

    let mut relation_offset = 0;
    let mut level_offset = 0;
    let mut level_len = leaf_len;
    let mut parent_offset = leaf_len;
    while level_len > 1 {
        let parent_len = level_len >> 1;
        if relation_index < relation_offset + parent_len {
            let local_index = relation_index - relation_offset;
            let left_index = level_offset + (local_index << 1);
            let right_index = left_index + 1;
            let parent_index = parent_offset + local_index;
            let residual = packed[parent_index] - packed[left_index] * packed[right_index];
            return Ok(ProductTreePackedRelationLocalQuery {
                residual,
                openings: vec![
                    (left_index, packed[left_index]),
                    (right_index, packed[right_index]),
                    (parent_index, packed[parent_index]),
                ],
            });
        }
        relation_offset += parent_len;
        level_offset = parent_offset;
        parent_offset += parent_len;
        level_len = parent_len;
    }
    Err(Error::InvalidPcsOpen(
        "RAA Section 5 product-tree relation query index is invalid".to_string(),
    ))
}

fn product_tree_packed_root_index(leaf_len: usize) -> usize {
    if leaf_len == 1 {
        0
    } else {
        (leaf_len << 1) - 2
    }
}

fn product_tree_packed_relation_residuals(packed: &[B128]) -> Result<Vec<B128>, Error> {
    if packed.len() < 2 || packed.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 packed product-tree witness has invalid length".to_string(),
        ));
    }
    let leaf_len = packed.len() >> 1;
    if !leaf_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 packed product-tree leaf domain is not a power of two".to_string(),
        ));
    }

    let mut residuals = Vec::with_capacity(leaf_len);
    let mut level_offset = 0;
    let mut level_len = leaf_len;
    let mut parent_offset = leaf_len;
    while level_len > 1 {
        let parent_len = level_len >> 1;
        if parent_offset + parent_len > packed.len() {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 packed product-tree witness is truncated".to_string(),
            ));
        }
        for index in 0..parent_len {
            residuals.push(
                packed[parent_offset + index]
                    - packed[level_offset + 2 * index] * packed[level_offset + 2 * index + 1],
            );
        }
        level_offset = parent_offset;
        parent_offset += parent_len;
        level_len = parent_len;
    }
    residuals.push(*packed.last().expect("packed length checked"));
    if residuals.len() != leaf_len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 product-tree residual length does not match leaf domain".to_string(),
        ));
    }
    Ok(residuals)
}

fn product_tree_packed_root(packed: &[B128]) -> Result<B128, Error> {
    if packed.len() < 2 || packed.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 packed product-tree witness has invalid length".to_string(),
        ));
    }
    let leaf_len = packed.len() >> 1;
    if !leaf_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 packed product-tree leaf domain is not a power of two".to_string(),
        ));
    }
    if leaf_len == 1 {
        Ok(packed[0])
    } else {
        Ok(packed[packed.len() - 2])
    }
}

fn prove_raa_relation_zero_sumcheck(
    residuals: &[B128],
    degree: usize,
    label: &str,
) -> Result<RaaRelationSumcheckProof, Error> {
    if residuals.is_empty() || !residuals.len().is_power_of_two() {
        return Err(Error::InvalidPcsOpen(format!(
            "RAA Section 5 {label} zero-check domain must be a non-empty power of two",
        )));
    }
    if residuals.iter().any(|value| *value != B128::ZERO) {
        return Err(Error::InvalidPcsOpen(format!(
            "RAA Section 5 {label} residual vector is not zero",
        )));
    }
    let num_vars = log2_strict(residuals.len());
    Ok(RaaRelationSumcheckProof {
        round_polynomials: vec![vec![B128::ZERO; degree + 1]; num_vars + 1],
    })
}

fn validate_query_schedule_spec(
    layout: &SystematicAugmentedRfcLayout,
    spec: &HolographicQueryScheduleSpec,
) -> Result<(), Error> {
    if spec.q_raa_input == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic BaseFold schedule needs at least one input query".to_string(),
        ));
    }
    if spec.auxiliary_oracle_len != 0
        && spec.raa_relation_strategy == RaaRelationProofStrategy::LocalQueries
        && spec.q_raa_input & 1 != 0
    {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled systematic BaseFold schedule needs an even RAA input query count"
                .to_string(),
        ));
    }
    let expected_auxiliary_len =
        raa_relation_auxiliary_row_count(spec.raa_relation_strategy) * layout.systematic_len();
    if spec.auxiliary_oracle_len != 0 && spec.auxiliary_oracle_len != expected_auxiliary_len {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled systematic BaseFold schedule expects a flattened strategy-specific RAA relation auxiliary trace"
                .to_string(),
        ));
    }
    if spec.q_backend_proof == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic BaseFold schedule needs at least one backend proof query".to_string(),
        ));
    }
    if spec.auxiliary_oracle_len != 0 && layout.systematic_len() < 2 {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled systematic BaseFold schedule needs at least two systematic entries"
                .to_string(),
        ));
    }
    if layout.systematic_len() == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic query domain must be non-empty".to_string(),
        ));
    }
    if layout.parity_len() + raa_relation_auxiliary_len(spec.auxiliary_oracle_len) == 0 {
        return Err(Error::InvalidPcsParam(
            "backend proof query domain must be non-empty".to_string(),
        ));
    }
    Ok(())
}

fn fold_challenges_from_prequery<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    prequery: &Blaze2BaseFoldPrequeryPublic<H>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) -> Result<Vec<B128>, Error> {
    let layout = SystematicAugmentedRfcLayout::new(spec.compiler_code.clone())?;
    if prequery.folded_parity_layers.len() != layout.num_rounds() {
        return Err(Error::InvalidPcsOpen(
            "folded parity prequery layer count does not match compiler layout".to_string(),
        ));
    }
    if let Some(proof) = &prequery.eval_sumcheck {
        let code = Blaze2Code::new(spec.praa.clone())?;
        return verify_raa_eval_sumcheck::<H>(
            spec,
            &prequery.compiler_parity,
            prequery.auxiliary.as_ref(),
            request,
            code.packed(),
            &prequery.terminal_codeword,
            proof,
        );
    }
    if spec.auxiliary_oracle_len != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze2 BaseFold prequery is missing the RAA eval sumcheck".to_string(),
        ));
    }

    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_basefold_fold_chain_prefix(
        &mut transcript,
        spec,
        &prequery.compiler_parity,
        prequery.auxiliary.as_ref(),
        request,
    );
    let mut challenges = Vec::with_capacity(layout.num_rounds());
    for round in 0..layout.num_rounds() {
        transcript.absorb("systematic-basefold-fold-challenge-v1");
        absorb_usize(&mut transcript, round);
        challenges.push(transcript.squeeze());
        absorb_folded_parity_public_commitment(
            &mut transcript,
            round + 1,
            &prequery.folded_parity_layers[round],
        );
    }
    Ok(challenges)
}

fn prove_raa_eval_sumcheck<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    compiler_parity: &CompilerParityPublicCommitment<H>,
    auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
    code: &PackedRaaCode,
    codeword: &[B128],
) -> Result<(RaaEvalSumcheckProof, Vec<B128>), Error> {
    let len = code.codeword_len();
    if codeword.len() != len {
        return Err(Error::InvalidPcsOpen(format!(
            "RAA eval sumcheck codeword has length {}, expected {len}",
            codeword.len()
        )));
    }
    if !len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck requires a power-of-two codeword length".to_string(),
        ));
    }

    let mut weights = raa_codeword_eval_weights(code, request.col_point)?;
    let mut values = codeword.to_vec();
    let mut claimed_sum = request.folded_eval;
    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_basefold_fold_chain_prefix(
        &mut transcript,
        spec,
        compiler_parity,
        auxiliary,
        request,
    );

    let num_rounds = log2_strict(len);
    let mut round_polynomials = Vec::with_capacity(num_rounds);
    let mut challenges = Vec::with_capacity(num_rounds);
    let mut active_len = len;
    for round in 0..num_rounds {
        let coeffs = raa_eval_sumcheck_round(&weights[..active_len], &values[..active_len])?;
        if raa_eval_sumcheck_zero_plus_one(&coeffs) != claimed_sum {
            return Err(Error::InvalidPcsOpen(
                "RAA eval sumcheck initial sum does not match folded eval".to_string(),
            ));
        }
        absorb_raa_eval_sumcheck_round(&mut transcript, round, &coeffs);
        let challenge = transcript.squeeze();
        raa_eval_sumcheck_fold_round(&mut weights, &mut values, active_len, challenge);
        claimed_sum = raa_eval_sumcheck_evaluate(&coeffs, challenge);
        active_len >>= 1;
        round_polynomials.push(coeffs);
        challenges.push(challenge);
    }

    if active_len != 1 || weights[0] * values[0] != claimed_sum {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck terminal product does not match folded eval".to_string(),
        ));
    }

    Ok((RaaEvalSumcheckProof { round_polynomials }, challenges))
}

fn verify_raa_eval_sumcheck<H: Hash>(
    spec: &Blaze2BaseFoldBackendSpec,
    compiler_parity: &CompilerParityPublicCommitment<H>,
    auxiliary: Option<&AuxiliaryOraclePublicCommitment<H>>,
    request: &Blaze2BaseFoldOpenRequest<'_>,
    code: &PackedRaaCode,
    terminal_codeword: &[B128],
    proof: &RaaEvalSumcheckProof,
) -> Result<Vec<B128>, Error> {
    let len = code.codeword_len();
    let num_rounds = log2_strict(len);
    if proof.round_polynomials.len() != num_rounds {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck round count does not match codeword length".to_string(),
        ));
    }
    if terminal_codeword.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck needs the clear terminal codeword".to_string(),
        ));
    }

    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_basefold_fold_chain_prefix(
        &mut transcript,
        spec,
        compiler_parity,
        auxiliary,
        request,
    );

    let mut claimed_sum = request.folded_eval;
    let mut challenges = Vec::with_capacity(num_rounds);
    for (round, coeffs) in proof.round_polynomials.iter().enumerate() {
        if raa_eval_sumcheck_zero_plus_one(coeffs) != claimed_sum {
            return Err(Error::InvalidPcsOpen(
                "RAA eval sumcheck consistency check failed".to_string(),
            ));
        }
        absorb_raa_eval_sumcheck_round(&mut transcript, round, coeffs);
        let challenge = transcript.squeeze();
        claimed_sum = raa_eval_sumcheck_evaluate(coeffs, challenge);
        challenges.push(challenge);
    }

    let weights = raa_codeword_eval_weights(code, request.col_point)?;
    let folded_weight = fold_eval_sumcheck_vector(weights, &challenges)?;
    if claimed_sum != folded_weight * terminal_codeword[0] {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck terminal check failed".to_string(),
        ));
    }

    Ok(challenges)
}

fn raa_eval_sumcheck_round(weights: &[B128], values: &[B128]) -> Result<[B128; 3], Error> {
    if weights.len() != values.len() || weights.is_empty() || weights.len() & 1 != 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck round has incompatible vector lengths".to_string(),
        ));
    }
    let half = weights.len() >> 1;
    let mut coeffs = [B128::ZERO; 3];
    for idx in 0..half {
        let weight_left = weights[idx];
        let weight_delta = weights[idx + half] - weight_left;
        let value_left = values[idx];
        let value_delta = values[idx + half] - value_left;
        coeffs[0] += weight_left * value_left;
        coeffs[1] += weight_left * value_delta + weight_delta * value_left;
        coeffs[2] += weight_delta * value_delta;
    }
    Ok(coeffs)
}

fn raa_eval_sumcheck_fold_round(
    weights: &mut [B128],
    values: &mut [B128],
    active_len: usize,
    challenge: B128,
) {
    let half = active_len >> 1;
    for idx in 0..half {
        weights[idx] = fold_systematic_pair(weights[idx], weights[idx + half], challenge);
        values[idx] = fold_systematic_pair(values[idx], values[idx + half], challenge);
    }
}

fn fold_eval_sumcheck_vector(mut values: Vec<B128>, challenges: &[B128]) -> Result<B128, Error> {
    if values.len() != (1usize << challenges.len()) {
        return Err(Error::InvalidPcsOpen(
            "RAA eval sumcheck terminal vector shape is invalid".to_string(),
        ));
    }
    let mut active_len = values.len();
    for &challenge in challenges {
        let half = active_len >> 1;
        for idx in 0..half {
            values[idx] = fold_systematic_pair(values[idx], values[idx + half], challenge);
        }
        active_len = half;
    }
    Ok(values[0])
}

#[allow(dead_code)]
fn verify_section5_relation_terminal_evaluations_from_residual_values(
    checks: &RaaSection5RelationChecks,
    residual_values: &[B128],
    domain_len: usize,
) -> Result<(), Error> {
    if domain_len == 0 || !domain_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual terminal domain length must be a nonzero power of two"
                .to_string(),
        ));
    }
    let expected_len = domain_len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT;
    if residual_values.len() != expected_len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 residual terminal values are not row-aligned".to_string(),
        ));
    }
    let num_vars = log2_strict(domain_len);
    let (permutation_row, rest) = residual_values.split_at(domain_len);
    let (first_accumulator_row, second_accumulator_row) = rest.split_at(domain_len);
    verify_section5_relation_terminal_row_evaluation(
        "permutation",
        permutation_row,
        &checks.permutation,
        num_vars,
    )?;
    verify_section5_relation_terminal_row_evaluation(
        "first accumulator",
        first_accumulator_row,
        &checks.first_accumulator,
        num_vars,
    )?;
    verify_section5_relation_terminal_row_evaluation(
        "second accumulator",
        second_accumulator_row,
        &checks.second_accumulator,
        num_vars,
    )
}

#[allow(dead_code)]
fn verify_section5_relation_terminal_row_evaluation(
    label: &str,
    row: &[B128],
    check: &RaaRelationSumcheckCheck,
    num_vars: usize,
) -> Result<(), Error> {
    if check.challenges.len() != num_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "RAA Section 5 {label} terminal challenge length is invalid"
        )));
    }
    let terminal_eval = fold_eval_sumcheck_vector(row.to_vec(), &check.challenges)?;
    if terminal_eval != check.terminal_claim {
        return Err(Error::InvalidPcsOpen(format!(
            "RAA Section 5 {label} terminal residual evaluation does not match sumcheck claim"
        )));
    }
    Ok(())
}

fn prove_section5_relation_terminal_proof<H: Hash>(
    checks: &RaaSection5RelationChecks,
    residual: &RaaSection5RelationResidualCommitment<H>,
    schedule: &HolographicQuerySchedule,
) -> Result<
    (
        RaaSection5RelationTerminalProof<H>,
        Vec<AuxiliaryOraclePublicCommitment<H>>,
        Vec<RaaSection5RelationTerminalLayerProof<H>>,
    ),
    Error,
> {
    let domain_len = section5_relation_terminal_domain_len(residual.len())?;
    verify_section5_relation_terminal_evaluations_from_residual_values(
        checks,
        residual.values(),
        domain_len,
    )?;
    let query_count = section5_relation_terminal_query_count(schedule);
    let public = residual.public();
    let (permutation_row, rest) = residual.values().split_at(domain_len);
    let (first_accumulator_row, second_accumulator_row) = rest.split_at(domain_len);
    let row_inputs = [
        (
            RAA_SECTION5_PERMUTATION_RESIDUAL_ROW,
            &checks.permutation,
            permutation_row,
        ),
        (
            RAA_SECTION5_FIRST_ACCUMULATOR_RESIDUAL_ROW,
            &checks.first_accumulator,
            first_accumulator_row,
        ),
        (
            RAA_SECTION5_SECOND_ACCUMULATOR_RESIDUAL_ROW,
            &checks.second_accumulator,
            second_accumulator_row,
        ),
    ];
    let row_folded_values = row_inputs
        .iter()
        .map(|(_, check, row)| section5_relation_terminal_fold_row_values(check, row, domain_len))
        .collect::<Result<Vec<_>, _>>()?;
    let folded_rounds = log2_strict(domain_len).saturating_sub(1);
    let mut folded_commitments = Vec::with_capacity(folded_rounds);
    let mut folded_layers = Vec::with_capacity(folded_rounds);
    for round_offset in 0..folded_rounds {
        let layer_len = domain_len >> (round_offset + 1);
        let mut layer_values =
            Vec::with_capacity(RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT * layer_len);
        for row_values in &row_folded_values {
            layer_values.extend_from_slice(&row_values[round_offset]);
        }
        let commitment = AuxiliaryOracleCommitment::<H>::commit_values(layer_values)?;
        folded_layers.push(commitment.public());
        folded_commitments.push(commitment);
    }
    let rows = row_inputs
        .iter()
        .zip(row_folded_values.iter())
        .map(|((row_index, check, row), folded_values)| {
            prove_section5_relation_terminal_row_proof::<H>(
                *row_index,
                check,
                &public,
                row,
                folded_values,
                &folded_layers,
                domain_len,
                query_count,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut folded_layer_authentication = Vec::with_capacity(folded_commitments.len());
    for (round_offset, commitment) in folded_commitments.iter().enumerate() {
        let round = round_offset + 1;
        let queries = section5_relation_terminal_combined_folded_layer_queries(
            round,
            &rows,
            checks,
            &public,
            &folded_layers,
            domain_len,
            query_count,
        )?;
        folded_layer_authentication.push(RaaSection5RelationTerminalLayerProof {
            authentication_nodes: merkle_b128_multiproof_nodes::<H, _>(
                &commitment.merkle_tree,
                queries,
            )?,
        });
    }

    Ok((
        RaaSection5RelationTerminalProof { rows },
        folded_layers,
        folded_layer_authentication,
    ))
}

fn section5_relation_terminal_fold_row_values(
    check: &RaaRelationSumcheckCheck,
    row: &[B128],
    domain_len: usize,
) -> Result<Vec<Vec<B128>>, Error> {
    let num_vars = log2_strict(domain_len);
    if row.len() != domain_len || check.challenges.len() != num_vars {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof row shape is invalid".to_string(),
        ));
    }
    let mut folded_values = Vec::with_capacity(num_vars.saturating_sub(1));
    let mut current = row.to_vec();
    for &challenge in &check.challenges {
        let half = current.len() >> 1;
        let mut next = Vec::with_capacity(half);
        for index in 0..half {
            next.push(fold_systematic_pair(
                current[index],
                current[index + half],
                challenge,
            ));
        }
        if next.len() > 1 {
            folded_values.push(next.clone());
        }
        current = next;
    }
    if current.len() != 1 || current[0] != check.terminal_claim {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof row does not fold to the sumcheck claim".to_string(),
        ));
    }
    Ok(folded_values)
}

fn prove_section5_relation_terminal_row_proof<H: Hash>(
    row_index: usize,
    check: &RaaRelationSumcheckCheck,
    residual_public: &RaaSection5RelationResidualPublicCommitment<H>,
    row: &[B128],
    folded_values: &[Vec<B128>],
    folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    domain_len: usize,
    query_count: usize,
) -> Result<RaaSection5RelationTerminalRowProof<H>, Error> {
    let num_vars = log2_strict(domain_len);
    if check.challenges.len() != num_vars {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof challenge length is invalid".to_string(),
        ));
    }
    if folded_values.len() != num_vars.saturating_sub(1)
        || folded_layers.len() != num_vars.saturating_sub(1)
    {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof folded layer count is invalid".to_string(),
        ));
    }

    let query_indices = section5_relation_terminal_query_indices::<H>(
        row_index,
        check,
        residual_public,
        folded_layers,
        domain_len,
        query_count,
    )?;
    let mut paths = Vec::with_capacity(query_indices.len());
    for top_index in query_indices {
        paths.push(section5_relation_terminal_path(
            row,
            folded_values,
            &check.challenges,
            top_index,
        )?);
    }

    Ok(RaaSection5RelationTerminalRowProof {
        paths,
        _hash_marker: PhantomData,
    })
}

fn section5_relation_terminal_path(
    row: &[B128],
    folded_values: &[Vec<B128>],
    challenges: &[B128],
    top_index: usize,
) -> Result<RaaSection5RelationTerminalPath, Error> {
    if top_index >= row.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal path top index is outside the row".to_string(),
        ));
    }
    let mut steps = Vec::with_capacity(challenges.len());
    let mut current_index = top_index;
    for round in 0..challenges.len() {
        let layer: &[B128] = if round == 0 {
            row
        } else {
            &folded_values[round - 1]
        };
        let active_len = layer.len();
        let half = active_len >> 1;
        let sibling_index = if current_index < half {
            current_index + half
        } else {
            current_index - half
        };
        steps.push(RaaSection5RelationTerminalStep {
            sibling_value: layer[sibling_index],
        });
        current_index &= half - 1;
    }
    Ok(RaaSection5RelationTerminalPath {
        top_value: row[top_index],
        steps,
    })
}

impl<H: Hash> RaaSection5RelationTerminalProof<H> {
    fn serialized_value_count(&self) -> usize {
        self.rows
            .iter()
            .map(RaaSection5RelationTerminalRowProof::serialized_value_count)
            .sum()
    }

    fn residual_authentication_queries(
        &self,
        checks: &RaaSection5RelationChecks,
        public: &RaaSection5RelationResidualPublicCommitment<H>,
        schedule: &HolographicQuerySchedule,
        folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    ) -> Result<Vec<(usize, B128)>, Error> {
        let domain_len = section5_relation_terminal_domain_len(public.len)?;
        let query_count = section5_relation_terminal_query_count(schedule);
        if self.rows.len() != RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof row count is invalid".to_string(),
            ));
        }
        let mut queries = Vec::new();
        for (row_index, row) in self.rows.iter().enumerate() {
            let check = section5_relation_terminal_row_check(checks, row_index)?;
            let top_indices = section5_relation_terminal_query_indices::<H>(
                row_index,
                check,
                public,
                folded_layers,
                domain_len,
                query_count,
            )?;
            queries.extend(section5_relation_terminal_base_authentication_queries(
                row_index,
                domain_len,
                &row.paths,
                &top_indices,
            )?);
        }
        Ok(queries)
    }

    fn verify(
        &self,
        checks: &RaaSection5RelationChecks,
        public: &RaaSection5RelationResidualPublicCommitment<H>,
        schedule: &HolographicQuerySchedule,
        folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
        folded_layer_authentication: &[RaaSection5RelationTerminalLayerProof<H>],
    ) -> Result<(), Error> {
        let domain_len = section5_relation_terminal_domain_len(public.len)?;
        let query_count = section5_relation_terminal_query_count(schedule);
        if self.rows.len() != RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof row count is invalid".to_string(),
            ));
        }
        let folded_rounds = log2_strict(domain_len).saturating_sub(1);
        if folded_layers.len() != folded_rounds {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof folded layer count is invalid".to_string(),
            ));
        }
        for (offset, layer) in folded_layers.iter().enumerate() {
            let round = offset + 1;
            if layer.len != RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT * (domain_len >> round) {
                return Err(Error::InvalidPcsOpen(
                    "RAA Section 5 terminal proof folded layer length is invalid".to_string(),
                ));
            }
        }
        self.rows[RAA_SECTION5_PERMUTATION_RESIDUAL_ROW].verify(
            RAA_SECTION5_PERMUTATION_RESIDUAL_ROW,
            &checks.permutation,
            public,
            folded_layers,
            domain_len,
            query_count,
        )?;
        self.rows[RAA_SECTION5_FIRST_ACCUMULATOR_RESIDUAL_ROW].verify(
            RAA_SECTION5_FIRST_ACCUMULATOR_RESIDUAL_ROW,
            &checks.first_accumulator,
            public,
            folded_layers,
            domain_len,
            query_count,
        )?;
        self.rows[RAA_SECTION5_SECOND_ACCUMULATOR_RESIDUAL_ROW].verify(
            RAA_SECTION5_SECOND_ACCUMULATOR_RESIDUAL_ROW,
            &checks.second_accumulator,
            public,
            folded_layers,
            domain_len,
            query_count,
        )?;

        if folded_layer_authentication.len() != folded_layers.len() {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof folded authentication count is invalid".to_string(),
            ));
        }
        for (offset, (folded_public, proof)) in folded_layers
            .iter()
            .zip(folded_layer_authentication.iter())
            .enumerate()
        {
            let round = offset + 1;
            let queries = section5_relation_terminal_combined_folded_layer_queries(
                round,
                &self.rows,
                checks,
                public,
                folded_layers,
                domain_len,
                query_count,
            )?;
            verify_merkle_b128_multiproof::<H, _>(
                &folded_public.root,
                folded_public.len,
                queries,
                &proof.authentication_nodes,
            )?;
        }
        Ok(())
    }
}

impl<H: Hash> RaaSection5RelationTerminalRowProof<H> {
    fn serialized_value_count(&self) -> usize {
        self.paths
            .iter()
            .map(|path| 1 + path.steps.len())
            .sum::<usize>()
    }

    fn verify(
        &self,
        row_index: usize,
        check: &RaaRelationSumcheckCheck,
        public: &RaaSection5RelationResidualPublicCommitment<H>,
        folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
        domain_len: usize,
        query_count: usize,
    ) -> Result<(), Error> {
        let num_vars = log2_strict(domain_len);
        if check.challenges.len() != num_vars {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof challenge length is invalid".to_string(),
            ));
        }
        if folded_layers.len() != num_vars.saturating_sub(1) {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof folded layer count is invalid".to_string(),
            ));
        }
        let expected_indices = section5_relation_terminal_query_indices::<H>(
            row_index,
            check,
            public,
            folded_layers,
            domain_len,
            query_count,
        )?;
        if self.paths.len() != expected_indices.len() {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal proof path count is invalid".to_string(),
            ));
        }
        for (path, expected_index) in self.paths.iter().zip(expected_indices.iter().copied()) {
            verify_section5_relation_terminal_path(path, check, domain_len, expected_index)?;
        }
        Ok(())
    }
}

fn verify_section5_relation_terminal_path(
    path: &RaaSection5RelationTerminalPath,
    check: &RaaRelationSumcheckCheck,
    domain_len: usize,
    top_index: usize,
) -> Result<(), Error> {
    if top_index >= domain_len || path.steps.len() != check.challenges.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof path shape is invalid".to_string(),
        ));
    }
    let mut current_value = path.top_value;
    let mut current_index = top_index;
    for (round, (&challenge, step)) in check.challenges.iter().zip(path.steps.iter()).enumerate() {
        let active_len = domain_len >> round;
        let half = active_len >> 1;
        let (left, right) = if current_index < half {
            (current_value, step.sibling_value)
        } else {
            (step.sibling_value, current_value)
        };
        current_value = fold_systematic_pair(left, right, challenge);
        current_index &= half - 1;
    }
    if current_index != 0 || current_value != check.terminal_claim {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof path does not fold to the sumcheck claim".to_string(),
        ));
    }
    Ok(())
}

fn section5_relation_terminal_value_after_round(
    path: &RaaSection5RelationTerminalPath,
    check: &RaaRelationSumcheckCheck,
    domain_len: usize,
    target_round: usize,
    top_index: usize,
) -> Result<(usize, B128), Error> {
    if target_round > check.challenges.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal folded layer round is invalid".to_string(),
        ));
    }
    let mut current_value = path.top_value;
    let mut current_index = top_index;
    for (round, &challenge) in check.challenges.iter().take(target_round).enumerate() {
        let step = path.steps.get(round).ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA Section 5 terminal folded layer path step is missing".to_string(),
            )
        })?;
        let active_len = domain_len >> round;
        let half = active_len >> 1;
        let (left, right) = if current_index < half {
            (current_value, step.sibling_value)
        } else {
            (step.sibling_value, current_value)
        };
        current_value = fold_systematic_pair(left, right, challenge);
        current_index &= half - 1;
    }
    Ok((current_index, current_value))
}

fn section5_relation_terminal_domain_len(residual_len: usize) -> Result<usize, Error> {
    if residual_len % RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT != 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal residual length is not row-aligned".to_string(),
        ));
    }
    let domain_len = residual_len / RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT;
    if domain_len == 0 || !domain_len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal residual domain length is invalid".to_string(),
        ));
    }
    Ok(domain_len)
}

fn section5_relation_terminal_query_count(schedule: &HolographicQuerySchedule) -> usize {
    schedule
        .section5_relation_residual_proof_query_count()
        .max(1)
}

fn section5_relation_terminal_query_indices<H: Hash>(
    row_index: usize,
    check: &RaaRelationSumcheckCheck,
    residual_public: &RaaSection5RelationResidualPublicCommitment<H>,
    folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    domain_len: usize,
    query_count: usize,
) -> Result<Vec<usize>, Error> {
    let mut transcript = CfriTranscript::<H>::new();
    transcript.absorb("raa-section5-relation-terminal-queries-v1");
    absorb_usize(&mut transcript, row_index);
    absorb_usize(&mut transcript, domain_len);
    absorb_section5_relation_residual_public_commitment(&mut transcript, residual_public);
    absorb_usize(&mut transcript, check.challenges.len());
    transcript.absorb_slice(&check.challenges);
    transcript.absorb(&check.initial_sum);
    transcript.absorb(&check.terminal_claim);
    absorb_usize(&mut transcript, folded_layers.len());
    for (round_offset, public) in folded_layers.iter().enumerate() {
        transcript.absorb("raa-section5-relation-terminal-folded-layer-v1");
        absorb_usize(&mut transcript, round_offset + 1);
        absorb_usize(&mut transcript, public.len);
        transcript.absorb(&public.root);
    }
    let mut indices = Vec::with_capacity(query_count);
    for _ in 0..query_count {
        indices.push(squeeze_bounded_index(&mut transcript, domain_len)?);
    }
    Ok(indices)
}

fn section5_relation_terminal_base_authentication_queries(
    row_index: usize,
    domain_len: usize,
    paths: &[RaaSection5RelationTerminalPath],
    top_indices: &[usize],
) -> Result<Vec<(usize, B128)>, Error> {
    if paths.len() != top_indices.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal base path count is invalid".to_string(),
        ));
    }
    let row_offset = row_index * domain_len;
    let mut queries = Vec::with_capacity(paths.len() * 2);
    for (path, &top_index) in paths.iter().zip(top_indices) {
        if top_index >= domain_len {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 terminal base path index is invalid".to_string(),
            ));
        }
        queries.push((row_offset + top_index, path.top_value));
        if let Some(step) = path.steps.first() {
            let half = domain_len >> 1;
            let sibling_index = if top_index < half {
                top_index + half
            } else {
                top_index - half
            };
            queries.push((row_offset + sibling_index, step.sibling_value));
        }
    }
    Ok(queries)
}

fn section5_relation_terminal_folded_layer_queries(
    round: usize,
    paths: &[RaaSection5RelationTerminalPath],
    top_indices: &[usize],
    check: &RaaRelationSumcheckCheck,
    domain_len: usize,
) -> Result<Vec<(usize, B128)>, Error> {
    if round == 0 || round >= check.challenges.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal folded layer round is invalid".to_string(),
        ));
    }
    let active_len = domain_len >> round;
    let half = active_len >> 1;
    if paths.len() != top_indices.len() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal folded layer path count is invalid".to_string(),
        ));
    }
    let mut queries = Vec::with_capacity(paths.len() * 2);
    for (path, &top_index) in paths.iter().zip(top_indices) {
        let (current_index, current_value) = section5_relation_terminal_value_after_round(
            path, check, domain_len, round, top_index,
        )?;
        let step = path.steps.get(round).ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA Section 5 terminal folded layer path step is missing".to_string(),
            )
        })?;
        let expected_sibling_index = if current_index < half {
            current_index + half
        } else {
            current_index - half
        };
        queries.push((current_index, current_value));
        queries.push((expected_sibling_index, step.sibling_value));
    }
    Ok(queries)
}

fn section5_relation_terminal_combined_folded_layer_queries<H: Hash>(
    round: usize,
    rows: &[RaaSection5RelationTerminalRowProof<H>],
    checks: &RaaSection5RelationChecks,
    residual_public: &RaaSection5RelationResidualPublicCommitment<H>,
    folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    domain_len: usize,
    query_count: usize,
) -> Result<Vec<(usize, B128)>, Error> {
    if rows.len() != RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof row count is invalid".to_string(),
        ));
    }
    let active_len = domain_len >> round;
    let mut queries = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let check = section5_relation_terminal_row_check(checks, row_index)?;
        let top_indices = section5_relation_terminal_query_indices::<H>(
            row_index,
            check,
            residual_public,
            folded_layers,
            domain_len,
            query_count,
        )?;
        queries.extend(
            section5_relation_terminal_folded_layer_queries(
                round,
                &row.paths,
                &top_indices,
                check,
                domain_len,
            )?
            .into_iter()
            .map(|(index, value)| (row_index * active_len + index, value)),
        );
    }
    Ok(queries)
}

fn section5_relation_terminal_row_check(
    checks: &RaaSection5RelationChecks,
    row_index: usize,
) -> Result<&RaaRelationSumcheckCheck, Error> {
    match row_index {
        RAA_SECTION5_PERMUTATION_RESIDUAL_ROW => Ok(&checks.permutation),
        RAA_SECTION5_FIRST_ACCUMULATOR_RESIDUAL_ROW => Ok(&checks.first_accumulator),
        RAA_SECTION5_SECOND_ACCUMULATOR_RESIDUAL_ROW => Ok(&checks.second_accumulator),
        _ => Err(Error::InvalidPcsOpen(
            "RAA Section 5 terminal proof row index is invalid".to_string(),
        )),
    }
}

fn raa_eval_sumcheck_evaluate(coeffs: &[B128; 3], point: B128) -> B128 {
    coeffs[0] + point * (coeffs[1] + point * coeffs[2])
}

fn raa_eval_sumcheck_zero_plus_one(coeffs: &[B128; 3]) -> B128 {
    coeffs[0] + raa_eval_sumcheck_evaluate(coeffs, B128::ONE)
}

fn absorb_raa_relation_sumcheck_round<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    label: &str,
    round: usize,
    coeffs: &[B128],
) {
    transcript.absorb("raa-section5-relation-sumcheck-round-v1");
    transcript.absorb(label);
    absorb_usize(transcript, round);
    absorb_usize(transcript, coeffs.len());
    for coeff in coeffs {
        transcript.absorb(coeff);
    }
}

fn raa_relation_sumcheck_evaluate(coeffs: &[B128], point: B128) -> B128 {
    coeffs
        .iter()
        .rev()
        .fold(B128::ZERO, |acc, coeff| acc * point + *coeff)
}

fn raa_relation_sumcheck_zero_plus_one(coeffs: &[B128]) -> B128 {
    coeffs[0] + raa_relation_sumcheck_evaluate(coeffs, B128::ONE)
}

fn parity_values_at_round(
    layout: &SystematicAugmentedRfcLayout,
    round: usize,
    physical_codeword: &[B128],
) -> Result<Vec<B128>, Error> {
    let message_len = layout.message_len_at_round(round)?;
    let parity_len = message_len * layout.parity_expansion_factor();
    let codeword_len = message_len * (layout.parity_expansion_factor() + 1);
    if physical_codeword.len() != codeword_len {
        return Err(Error::InvalidPcsOpen(format!(
            "physical folded codeword has length {}, expected {codeword_len} at round {round}",
            physical_codeword.len()
        )));
    }

    let mut parity = vec![B128::ZERO; parity_len];
    for (physical_index, &value) in physical_codeword.iter().enumerate() {
        let address = layout.physical_to_logical_at_round(round, physical_index)?;
        if address.part == CodewordPart::Parity {
            parity[address.local_index] = value;
        }
    }
    Ok(parity)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParityLayerOpening {
    logical_index: usize,
    value: B128,
}

fn parity_value_for_physical_index(
    layout: &SystematicAugmentedRfcLayout,
    physical_layers: &[Vec<B128>],
    round: usize,
    physical_index: usize,
) -> Result<ParityLayerOpening, Error> {
    let address = layout.physical_to_logical_at_round(round, physical_index)?;
    if address.part != CodewordPart::Parity {
        return Err(Error::InvalidPcsOpen(
            "compiler parity fold path tried to open a systematic position as parity".to_string(),
        ));
    }
    let layer = physical_layers.get(round).ok_or_else(|| {
        Error::InvalidPcsOpen(format!(
            "missing physical folded codeword for round {round}"
        ))
    })?;
    let value = *layer.get(physical_index).ok_or_else(|| {
        Error::InvalidPcsOpen(format!(
            "physical index {physical_index} is outside physical folded codeword for round {round}"
        ))
    })?;
    Ok(ParityLayerOpening {
        logical_index: address.local_index,
        value,
    })
}

fn collect_backend_query_set_from_prover_state<H: Hash>(
    layout: &SystematicAugmentedRfcLayout,
    state: &Blaze2BaseFoldProverState<H>,
    schedule: &HolographicQuerySchedule,
    compiler_parity: &CompilerParityQueryProof<H>,
    compiler_parity_folds: &CompilerParityFoldQueryProof<H>,
    auxiliary: Option<&AuxiliaryOracleQueryProof<H>>,
    section5_relation_residual: Option<&RaaSection5RelationResidualQueryProof<H>>,
    section5_relation_terminal_folded_layers: &[AuxiliaryOraclePublicCommitment<H>],
    section5_permutation_helper: Option<&RaaSection5PermutationHelperQueryProof<H>>,
) -> Result<BackendProofOracleQuerySet, Error> {
    let compiler_parity_queries = compiler_parity
        .queries
        .iter()
        .map(|query| (query.logical_index, query.value))
        .collect();
    let compiler_parity_fold_layer_queries = compiler_parity_fold_layer_queries(
        layout,
        &compiler_parity_folds.paths,
        &compiler_parity.queries,
        &state.fold_challenges,
        None,
    )?;
    let auxiliary_queries = match (&state.auxiliary, auxiliary) {
        (Some(oracle), Some(proof)) => {
            proof.authentication_queries_from_oracle(schedule, oracle)?
        }
        (None, None) => {
            if expected_auxiliary_query_proof_count(schedule) != 0 {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains auxiliary queries, but auxiliary query proof is absent"
                        .to_string(),
                ));
            }
            Vec::new()
        }
        (Some(_), None) => {
            if expected_auxiliary_query_proof_count(schedule) == 0 {
                Vec::new()
            } else {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains auxiliary queries, but auxiliary query proof is absent"
                        .to_string(),
                ));
            }
        }
        (None, Some(_)) => {
            return Err(Error::InvalidPcsOpen(
                "auxiliary query proof supplied without an auxiliary oracle".to_string(),
            ));
        }
    };
    let mut section5_relation_residual_queries = match (
        &state.section5_relation_residual,
        section5_relation_residual,
    ) {
        (Some(_oracle), Some(proof)) => proof.authentication_queries(schedule)?,
        (None, None) => {
            if schedule.section5_relation_residual_proof_query_count() != 0 {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains Section 5 residual queries, but residual query proof is absent"
                        .to_string(),
                ));
            }
            Vec::new()
        }
        (Some(_), None) => {
            if schedule.section5_relation_residual_proof_query_count() == 0 {
                Vec::new()
            } else {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains Section 5 residual queries, but residual query proof is absent"
                        .to_string(),
                ));
            }
        }
        (None, Some(_)) => {
            return Err(Error::InvalidPcsOpen(
                "Section 5 residual query proof supplied without a residual oracle".to_string(),
            ));
        }
    };
    if let (Some(oracle), Some(proof), Some(relation), Some(auxiliary)) = (
        state.section5_relation_residual.as_ref(),
        section5_relation_residual,
        state.section5_relation.as_ref(),
        state.auxiliary.as_ref(),
    ) {
        let checks = relation.verify_prequery::<H>(auxiliary.len())?;
        let public = oracle.public();
        section5_relation_residual_queries.extend(proof.terminal_residual_authentication_queries(
            &checks,
            &public,
            schedule,
            section5_relation_terminal_folded_layers,
        )?);
    }
    let section5_permutation_helper_queries = match (
        &state.section5_permutation_helper,
        section5_permutation_helper,
    ) {
        (Some(_oracle), Some(proof)) => proof.authentication_queries(schedule)?,
        (None, None) => {
            if schedule.section5_permutation_helper_proof_query_count() != 0 {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains Section 5 helper queries, but helper query proof is absent"
                        .to_string(),
                ));
            }
            Vec::new()
        }
        (Some(_), None) => {
            if schedule.section5_permutation_helper_proof_query_count() == 0 {
                Vec::new()
            } else {
                return Err(Error::InvalidPcsOpen(
                    "backend schedule contains Section 5 helper queries, but helper query proof is absent"
                        .to_string(),
                ));
            }
        }
        (None, Some(_)) => {
            return Err(Error::InvalidPcsOpen(
                "Section 5 helper query proof supplied without a helper oracle".to_string(),
            ));
        }
    };

    Ok(BackendProofOracleQuerySet {
        compiler_parity_queries,
        compiler_parity_fold_layer_queries,
        auxiliary_queries,
        section5_relation_residual_queries,
        section5_permutation_helper_queries,
    })
}

fn compiler_parity_fold_layer_queries(
    layout: &SystematicAugmentedRfcLayout,
    paths: &[CompilerParityFoldPath],
    top_queries: &[CompilerParityQuery],
    fold_challenges: &[B128],
    terminal_codeword: Option<&[B128]>,
) -> Result<Vec<Vec<(usize, B128)>>, Error> {
    if top_queries.len() != paths.len() {
        return Err(Error::InvalidPcsOpen(
            "compiler parity fold top query count does not match paths".to_string(),
        ));
    }
    if fold_challenges.len() != layout.num_rounds() {
        return Err(Error::InvalidPcsOpen(
            "compiler parity fold challenge count does not match layout".to_string(),
        ));
    }
    let mut queries = vec![Vec::new(); layout.num_rounds()];
    for (path, top_query) in paths.iter().zip(top_queries) {
        if path.steps.len() != layout.num_rounds() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold path has wrong round count".to_string(),
            ));
        }
        let mut current_value = top_query.value;
        let mut current_physical_index = layout.parity_to_physical(top_query.logical_index)?;
        for (round, step) in path.steps.iter().enumerate() {
            let current_len = layout.codeword_len() >> round;
            let half_len = current_len >> 1;
            let output_physical_index = current_physical_index & (half_len - 1);
            let pair = layout.fold_pair(round, output_physical_index)?;
            let (left_value, right_value, sibling_physical_index) =
                if current_physical_index == pair.left {
                    (current_value, step.sibling_value, pair.right)
                } else if current_physical_index == pair.right {
                    (step.sibling_value, current_value, pair.left)
                } else {
                    return Err(Error::InvalidPcsOpen(
                        "compiler parity fold path current index does not lie in its fold pair"
                            .to_string(),
                    ));
                };
            let sibling_logical_index =
                parity_logical_index_at_round(layout, round, sibling_physical_index)?;
            queries[round].push((sibling_logical_index, step.sibling_value));
            let folded = fold_rfc_parity_pair(left_value, right_value, fold_challenges[round]);
            if round + 1 < layout.num_rounds() {
                let folded_logical_index =
                    parity_logical_index_at_round(layout, round + 1, output_physical_index)?;
                queries[round + 1].push((folded_logical_index, folded));
            } else if let Some(terminal_codeword) = terminal_codeword {
                let terminal_value =
                    terminal_codeword
                        .get(output_physical_index)
                        .ok_or_else(|| {
                            Error::InvalidPcsOpen(
                        "compiler parity fold path terminal index is outside terminal codeword"
                            .to_string(),
                    )
                        })?;
                if *terminal_value != folded {
                    return Err(Error::InvalidPcsOpen(
                        "compiler parity fold path terminal value does not match clear terminal codeword"
                            .to_string(),
                    ));
                }
            }
            current_value = folded;
            current_physical_index = output_physical_index;
        }
    }
    Ok(queries)
}

fn parity_commitment_at_round<'a, H: Hash>(
    top: &'a CompilerParityCommitment<H>,
    folded: &'a [CompilerParityCommitment<H>],
    round: usize,
) -> Result<&'a CompilerParityCommitment<H>, Error> {
    if round == 0 {
        return Ok(top);
    }
    folded.get(round - 1).ok_or_else(|| {
        Error::InvalidPcsOpen(format!(
            "missing folded parity commitment for round {round}"
        ))
    })
}

fn parity_public_at_round<'a, H: Hash>(
    prequery: &'a Blaze2BaseFoldPrequeryPublic<H>,
    round: usize,
) -> Result<&'a CompilerParityPublicCommitment<H>, Error> {
    if round == 0 {
        return Ok(&prequery.compiler_parity);
    }
    prequery.folded_parity_layers.get(round - 1).ok_or_else(|| {
        Error::InvalidPcsOpen(format!(
            "missing folded parity public commitment for round {round}"
        ))
    })
}

fn parity_logical_index_at_round(
    layout: &SystematicAugmentedRfcLayout,
    round: usize,
    physical_index: usize,
) -> Result<usize, Error> {
    let address = layout.physical_to_logical_at_round(round, physical_index)?;
    if address.part != CodewordPart::Parity {
        return Err(Error::InvalidPcsOpen(
            "compiler parity fold path tried to open a systematic position as parity".to_string(),
        ));
    }
    Ok(address.local_index)
}

fn verify_terminal_codeword<H: Hash>(
    prequery: &Blaze2BaseFoldPrequeryPublic<H>,
    code: &SystematicAugmentedRfcCode,
) -> Result<(), Error> {
    let layout = code.layout();
    let terminal_round = layout.num_rounds();
    let terminal_len = layout.parity_expansion_factor() + 1;
    if prequery.terminal_codeword.len() != terminal_len {
        return Err(Error::InvalidPcsOpen(format!(
            "terminal codeword has length {}, expected {terminal_len}",
            prequery.terminal_codeword.len()
        )));
    }

    let terminal_address = layout.physical_to_logical_at_round(terminal_round, 0)?;
    if terminal_address.part != CodewordPart::Systematic || terminal_address.local_index != 0 {
        return Err(Error::InvalidPcsOpen(
            "terminal systematic position is not at the expected physical address".to_string(),
        ));
    }
    let terminal_message = [prequery.terminal_codeword[0]];
    let mut expected = vec![B128::ZERO; terminal_len];
    let mut parity = vec![B128::ZERO; layout.parity_expansion_factor()];
    code.encode_physical_codeword_at_round_into(
        terminal_round,
        &terminal_message,
        &mut expected,
        &mut parity,
    )?;
    if expected != prequery.terminal_codeword {
        return Err(Error::InvalidPcsOpen(
            "terminal codeword does not satisfy the base systematic RFC code".to_string(),
        ));
    }

    let terminal_parity =
        parity_values_at_round(layout, terminal_round, &prequery.terminal_codeword)?;
    let terminal_public = parity_public_at_round(prequery, terminal_round)?;
    let terminal_commitment = CompilerParityCommitment::<H>::commit_values(terminal_parity)?;
    let terminal_from_clear = terminal_commitment.public();
    if terminal_from_clear.len != terminal_public.len
        || terminal_from_clear.root != terminal_public.root
    {
        return Err(Error::InvalidPcsOpen(
            "terminal parity root does not match the clear terminal codeword".to_string(),
        ));
    }
    Ok(())
}

fn squeeze_bounded_index<H: Hash, S>(
    transcript: &mut CfriTranscript<H, S>,
    domain_len: usize,
) -> Result<usize, Error> {
    if domain_len == 0 {
        return Err(Error::InvalidPcsParam(
            "query domain must be non-empty".to_string(),
        ));
    }

    if domain_len.is_power_of_two() {
        let challenge: B128 = transcript.squeeze();
        let value = b128_to_u128(challenge) & ((domain_len as u128) - 1);
        return Ok(value as usize);
    }

    let domain = domain_len as u128;
    let rejected_tail_len = (u128::MAX % domain + 1) % domain;
    let max_accepted = u128::MAX - rejected_tail_len;
    loop {
        let challenge: B128 = transcript.squeeze();
        let value = b128_to_u128(challenge);
        if rejected_tail_len == 0 || value <= max_accepted {
            return Ok((value % domain) as usize);
        }
    }
}

fn proof_query_from_sampled_index(
    layout: &SystematicAugmentedRfcLayout,
    auxiliary_oracle_len: usize,
    section5_residual_len: usize,
    section5_helper_len: usize,
    sampled_index: usize,
) -> Result<BackendProofQuery, Error> {
    let auxiliary_len = raa_relation_auxiliary_len(auxiliary_oracle_len);
    let proof_domain_len =
        layout.parity_len() + auxiliary_len + section5_residual_len + section5_helper_len;
    if sampled_index >= proof_domain_len {
        return Err(Error::InvalidPcsParam(format!(
            "proof query index {sampled_index} is outside proof query domain {proof_domain_len}"
        )));
    }
    if sampled_index < layout.parity_len() {
        Ok(BackendProofQuery {
            domain: BackendProofQueryDomain::CompilerParity,
            index: sampled_index,
            physical_index: Some(layout.parity_to_physical(sampled_index)?),
        })
    } else {
        let auxiliary_index = sampled_index - layout.parity_len();
        if auxiliary_index < auxiliary_len {
            return Ok(BackendProofQuery {
                domain: BackendProofQueryDomain::RelationAuxiliary,
                index: auxiliary_index,
                physical_index: None,
            });
        }
        let residual_index = auxiliary_index - auxiliary_len;
        if residual_index < section5_residual_len {
            return Ok(BackendProofQuery {
                domain: BackendProofQueryDomain::Section5RelationResidual,
                index: residual_index,
                physical_index: None,
            });
        }
        Ok(BackendProofQuery {
            domain: BackendProofQueryDomain::Section5PermutationHelper,
            index: residual_index - section5_residual_len,
            physical_index: None,
        })
    }
}

fn build_raa_auxiliary_local_queries(
    code: &PackedRaaCode,
    proof_queries: &[BackendProofQuery],
) -> Result<Vec<RaaAuxiliaryLocalQuery>, Error> {
    let mut queries = Vec::new();
    let mut auxiliary_ordinal = 0usize;
    for query in proof_queries {
        if query.domain != BackendProofQueryDomain::RelationAuxiliary {
            continue;
        }
        if let Some(local_query) =
            raa_auxiliary_local_query_for_index(code, query.index, auxiliary_ordinal)?
        {
            queries.push(local_query);
        }
        auxiliary_ordinal += 1;
    }
    Ok(queries)
}

fn build_raa_auxiliary_authentication_queries(
    local_queries: &[RaaAuxiliaryLocalQuery],
) -> Result<Vec<BackendProofQuery>, Error> {
    let extra_count = local_queries
        .iter()
        .map(RaaAuxiliaryLocalQuery::extra_count)
        .sum();
    let mut queries = Vec::with_capacity(extra_count);
    for query in local_queries {
        for extra_ordinal in 0..query.extra_count() {
            let index = query.extra_index(extra_ordinal).ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "RAA auxiliary local authentication query ordinal is invalid".to_string(),
                )
            })?;
            queries.push(BackendProofQuery {
                domain: BackendProofQueryDomain::RelationAuxiliary,
                index,
                physical_index: None,
            });
        }
    }
    Ok(queries)
}

fn consume_expected_raa_auxiliary_authentication_queries<'a, I>(
    scheduled: &mut I,
    expected: &RaaAuxiliaryLocalQuery,
) -> Result<(), Error>
where
    I: Iterator<Item = &'a BackendProofQuery>,
{
    for extra_ordinal in 0..expected.extra_count() {
        let expected_index = expected.extra_index(extra_ordinal).ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA auxiliary local authentication query ordinal is invalid".to_string(),
            )
        })?;
        let scheduled_query = scheduled.next().ok_or_else(|| {
            Error::InvalidPcsOpen(
                "RAA auxiliary local authentication query is missing from schedule".to_string(),
            )
        })?;
        if scheduled_query.domain != BackendProofQueryDomain::RelationAuxiliary
            || scheduled_query.index != expected_index
        {
            return Err(Error::InvalidPcsOpen(
                "RAA auxiliary local authentication query does not match schedule".to_string(),
            ));
        }
    }
    Ok(())
}

fn raa_auxiliary_local_query_for_index(
    code: &PackedRaaCode,
    auxiliary_index: usize,
    auxiliary_ordinal: usize,
) -> Result<Option<RaaAuxiliaryLocalQuery>, Error> {
    let len = code.codeword_len();
    if auxiliary_index >= RAA_AUX_ROW_COUNT * len {
        return Err(Error::InvalidPcsParam(format!(
            "RAA auxiliary query index {auxiliary_index} is outside flattened trace length {}",
            RAA_AUX_ROW_COUNT * len
        )));
    }

    let row = auxiliary_index / len;
    let index = auxiliary_index % len;
    let relation = match row {
        RAA_AUX_U2_ROW => raa_u2_repetition_peer(code, index).map(|peer_u2_auxiliary_index| {
            RaaAuxiliaryRelationKind::U2Repetition {
                peer_u2_auxiliary_index,
            }
        }),
        RAA_AUX_U3_ROW => {
            let u2_auxiliary_index = raa_auxiliary_index(RAA_AUX_U2_ROW, index, len);
            Some(if index == 0 {
                RaaAuxiliaryRelationKind::FirstAccumulatorStart { u2_auxiliary_index }
            } else {
                RaaAuxiliaryRelationKind::FirstAccumulatorStep {
                    previous_u3_auxiliary_index: raa_auxiliary_index(
                        RAA_AUX_U3_ROW,
                        index - 1,
                        len,
                    ),
                    u2_auxiliary_index,
                }
            })
        }
        RAA_AUX_U4_ROW => Some(RaaAuxiliaryRelationKind::SecondPermutation {
            permuted_u3_auxiliary_index: raa_auxiliary_index(
                RAA_AUX_U3_ROW,
                code.permutation().permutation2[index],
                len,
            ),
        }),
        _ => None,
    };

    Ok(relation.map(|relation| RaaAuxiliaryLocalQuery {
        sampled_auxiliary_ordinal: auxiliary_ordinal,
        main_auxiliary_index: auxiliary_index,
        relation,
    }))
}

fn raa_u2_repetition_peer(code: &PackedRaaCode, index: usize) -> Option<usize> {
    if code.rate() <= 1 {
        return None;
    }
    let source = code.permutation().permutation1[index] / code.rate();
    code.permutation()
        .permutation1
        .iter()
        .enumerate()
        .find_map(|(candidate, &permuted_index)| {
            (candidate != index && permuted_index / code.rate() == source).then_some(candidate)
        })
}

fn raa_auxiliary_index(row: usize, index: usize, len: usize) -> usize {
    row * len + index
}

fn raa_relation_auxiliary_row_count(strategy: RaaRelationProofStrategy) -> usize {
    match strategy {
        RaaRelationProofStrategy::LocalQueries => RAA_AUX_ROW_COUNT,
        RaaRelationProofStrategy::Section5 => RAA_SECTION5_AUX_ROW_COUNT,
    }
}

fn raa_section5_permutation_helper_rows() -> [usize; RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT] {
    [
        RAA_SECTION5_F1_0_ROW,
        RAA_SECTION5_F1_1_ROW,
        RAA_SECTION5_G1_0_ROW,
        RAA_SECTION5_G1_1_ROW,
        RAA_SECTION5_F2_0_ROW,
        RAA_SECTION5_F2_1_ROW,
        RAA_SECTION5_G2_0_ROW,
        RAA_SECTION5_G2_1_ROW,
    ]
}

fn build_raa_section5_permutation_helper_oracle(
    code: &PackedRaaCode,
    auxiliary_oracle: &[B128],
    challenges: RaaSection5RelationChallenges,
) -> Result<Vec<B128>, Error> {
    let len = code.codeword_len();
    let relation_len = RAA_AUX_ROW_COUNT * len;
    if auxiliary_oracle.len() < relation_len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle requires u2/u3/u4 auxiliary rows".to_string(),
        ));
    }
    let auxiliary = &auxiliary_oracle[..relation_len];
    let u2 = &auxiliary
        [raa_auxiliary_index(RAA_AUX_U2_ROW, 0, len)..raa_auxiliary_index(RAA_AUX_U3_ROW, 0, len)];
    let u3 = &auxiliary
        [raa_auxiliary_index(RAA_AUX_U3_ROW, 0, len)..raa_auxiliary_index(RAA_AUX_U4_ROW, 0, len)];
    let u4 = &auxiliary[raa_auxiliary_index(RAA_AUX_U4_ROW, 0, len)..relation_len];

    let mut u1 = vec![B128::ZERO; len];
    let permutation = code.permutation();
    for (index, value) in u2.iter().enumerate() {
        let target = permutation.permutation1[index];
        if target >= len {
            return Err(Error::InvalidPcsOpen(
                "RAA first permutation index is outside the helper domain".to_string(),
            ));
        }
        u1[target] = *value;
    }

    let mut values = Vec::with_capacity(len * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT);
    append_raa_section5_permutation_helper_witness(
        &mut values,
        u2,
        Some(&permutation.permutation1),
        challenges,
    )?;
    append_raa_section5_permutation_helper_witness(&mut values, &u1, None, challenges)?;
    append_raa_section5_permutation_helper_witness(
        &mut values,
        u4,
        Some(&permutation.permutation2),
        challenges,
    )?;
    append_raa_section5_permutation_helper_witness(&mut values, u3, None, challenges)?;
    let expected_len = len * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT;
    if values.len() != expected_len {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper oracle length does not match its packed row layout".to_string(),
        ));
    }
    Ok(values)
}

fn append_raa_section5_permutation_helper_witness(
    values: &mut Vec<B128>,
    poly: &[B128],
    permutation: Option<&[usize]>,
    challenges: RaaSection5RelationChallenges,
) -> Result<(), Error> {
    let len = poly.len();
    if len == 0 || !len.is_power_of_two() {
        return Err(Error::InvalidPcsOpen(
            "RAA Section 5 helper witness expects a non-empty power-of-two domain".to_string(),
        ));
    }
    if let Some(permutation) = permutation {
        if permutation.len() != len {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 helper permutation length does not match the witness domain"
                    .to_string(),
            ));
        }
    }

    let mut leaves = Vec::with_capacity(len);
    match permutation {
        Some(permutation) => {
            for (index, value) in poly.iter().enumerate() {
                let permuted = permutation[index];
                if permuted >= len {
                    return Err(Error::InvalidPcsOpen(
                        "RAA Section 5 helper permutation index is outside the witness domain"
                            .to_string(),
                    ));
                }
                leaves.push(
                    challenges.alpha - (*value + challenges.beta * B128::from(permuted as u64)),
                );
            }
        }
        None => {
            for (index, value) in poly.iter().enumerate() {
                leaves
                    .push(challenges.alpha - (*value + challenges.beta * B128::from(index as u64)));
            }
        }
    }

    let tree = ProductTree::new(leaves)?;
    values.extend_from_slice(&tree.packed_witness_evals());
    Ok(())
}

fn raa_relation_auxiliary_len(auxiliary_oracle_len: usize) -> usize {
    auxiliary_oracle_len
}

fn section5_relation_residual_len_for_schedule(
    layout: &SystematicAugmentedRfcLayout,
    spec: &HolographicQueryScheduleSpec,
) -> usize {
    if spec.raa_relation_strategy == RaaRelationProofStrategy::Section5 {
        layout.systematic_len() * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT
    } else {
        0
    }
}

fn section5_permutation_helper_len_for_schedule(
    layout: &SystematicAugmentedRfcLayout,
    spec: &HolographicQueryScheduleSpec,
) -> usize {
    if spec.raa_relation_strategy == RaaRelationProofStrategy::Section5 {
        layout.systematic_len() * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT
    } else {
        0
    }
}

fn b128_to_u128(value: B128) -> u128 {
    u128::from(value.value[0]) | (u128::from(value.value[1]) << 64)
}

fn absorb_usize<H: Hash, S>(transcript: &mut CfriTranscript<H, S>, value: usize) {
    transcript.absorb(&(value as u64).to_le_bytes());
}

fn build_parity_fold_table(layout: &SystematicAugmentedRfcLayout) -> Vec<Vec<B128>> {
    let mut transcript = CfriTranscript::<Blake2s>::new();
    absorb_systematic_foldable_code_spec(&mut transcript, layout.spec());

    let num_levels = layout.num_rounds().max(1);
    let mut table = Vec::with_capacity(num_levels);
    let mut level_len = layout.parity_expansion_factor();
    for level_index in 0..num_levels {
        transcript.absorb("systematic-basefold-parity-fold-level");
        absorb_usize(&mut transcript, level_index);
        absorb_usize(&mut transcript, level_len);
        let mut level = vec![B128::ZERO; level_len];
        transcript.squeeze_into(&mut level);
        table.push(level);
        level_len <<= 1;
    }
    table
}

fn validate_parity_query_index(index: usize, len: usize) -> Result<(), Error> {
    if len == 0 {
        return Err(Error::InvalidPcsOpen(
            "compiler parity query domain is empty".to_string(),
        ));
    }
    if index >= len {
        return Err(Error::InvalidPcsOpen(format!(
            "compiler parity query index {index} is outside length {len}"
        )));
    }
    Ok(())
}

fn validate_auxiliary_query_index(index: usize, len: usize) -> Result<(), Error> {
    if len == 0 {
        return Err(Error::InvalidPcsOpen(
            "auxiliary oracle query domain is empty".to_string(),
        ));
    }
    if index >= len {
        return Err(Error::InvalidPcsOpen(format!(
            "auxiliary oracle query index {index} is outside length {len}"
        )));
    }
    Ok(())
}

fn expected_auxiliary_query_proof_count(schedule: &HolographicQuerySchedule) -> usize {
    schedule.expected_auxiliary_query_proof_count()
}

fn verify_auxiliary_query_proof<H: Hash>(
    public: Option<&AuxiliaryOraclePublicCommitment<H>>,
    proof: Option<&AuxiliaryOracleQueryProof<H>>,
    auxiliary_oracle_len: usize,
    schedule: &HolographicQuerySchedule,
    top_queries: &[TopQuery<B128>],
) -> Result<(), Error> {
    let expected_count = expected_auxiliary_query_proof_count(schedule);
    if auxiliary_oracle_len == 0 {
        if public.is_some() || proof.is_some() || expected_count != 0 {
            return Err(Error::InvalidPcsOpen(
                "backend has no auxiliary oracle but auxiliary proof data was supplied or scheduled"
                    .to_string(),
            ));
        }
        return Ok(());
    }

    let public = public.ok_or_else(|| {
        Error::InvalidPcsOpen(
            "backend spec expects an auxiliary oracle commitment, but it is absent".to_string(),
        )
    })?;
    if public.len != auxiliary_oracle_len {
        return Err(Error::InvalidPcsOpen(
            "auxiliary oracle commitment length does not match backend spec".to_string(),
        ));
    }
    if expected_count == 0 {
        if proof.map(|proof| proof.query_count() != 0).unwrap_or(false) {
            return Err(Error::InvalidPcsOpen(
                "auxiliary query proof was supplied even though schedule has no auxiliary queries"
                    .to_string(),
            ));
        }
        return Ok(());
    }
    let proof = proof.ok_or_else(|| {
        Error::InvalidPcsOpen(
            "backend schedule contains auxiliary queries, but auxiliary query proof is absent"
                .to_string(),
        )
    })?;
    proof.verify_schedule(public, auxiliary_oracle_len, schedule, top_queries)
}

fn verify_section5_permutation_helper_query_proof<H: Hash>(
    public: Option<&RaaSection5PermutationHelperPublicCommitment<H>>,
    proof: Option<&RaaSection5PermutationHelperQueryProof<H>>,
    schedule: &HolographicQuerySchedule,
) -> Result<(), Error> {
    let expected_count = schedule.section5_permutation_helper_proof_query_count();
    match (public, proof, expected_count) {
        (None, None, 0) => Ok(()),
        (None, Some(_), _) => Err(Error::InvalidPcsOpen(
            "Section 5 helper query proof was supplied without a helper commitment".to_string(),
        )),
        (Some(_), None, 0) => Ok(()),
        (Some(_), None, _) => Err(Error::InvalidPcsOpen(
            "backend schedule contains Section 5 helper queries, but helper query proof is absent"
                .to_string(),
        )),
        (Some(public), Some(proof), _) => proof.verify_schedule(public, schedule),
        (None, None, _) => Err(Error::InvalidPcsOpen(
            "backend schedule contains Section 5 helper queries, but no helper commitment is present"
                .to_string(),
        )),
    }
}

fn verify_section5_relation_residual_query_proof<H: Hash>(
    public: Option<&RaaSection5RelationResidualPublicCommitment<H>>,
    proof: Option<&RaaSection5RelationResidualQueryProof<H>>,
    schedule: &HolographicQuerySchedule,
) -> Result<(), Error> {
    let expected_count = schedule.section5_relation_residual_proof_query_count();
    match (public, proof, expected_count) {
        (None, None, 0) => Ok(()),
        (None, Some(_), _) => Err(Error::InvalidPcsOpen(
            "Section 5 residual query proof was supplied without a residual commitment".to_string(),
        )),
        (Some(_), None, 0) => Ok(()),
        (Some(_), None, _) => Err(Error::InvalidPcsOpen(
            "backend schedule contains Section 5 residual queries, but residual query proof is absent"
                .to_string(),
        )),
        (Some(public), Some(proof), _) => proof.verify_schedule(public, schedule),
        (None, None, _) => Err(Error::InvalidPcsOpen(
            "backend schedule contains Section 5 residual queries, but no residual commitment is present"
                .to_string(),
        )),
    }
}

fn verify_section5_relation_query_proof_matches_prequery<H: Hash>(
    prequery_relation: Option<&RaaSection5RelationProof>,
    auxiliary_proof: Option<&AuxiliaryOracleQueryProof<H>>,
    schedule: &HolographicQuerySchedule,
) -> Result<(), Error> {
    if schedule.raa_relation_strategy() != RaaRelationProofStrategy::Section5 {
        if prequery_relation.is_some() {
            return Err(Error::InvalidPcsOpen(
                "RAA Section 5 relation prequery was supplied for a non-Section 5 schedule"
                    .to_string(),
            ));
        }
        return Ok(());
    }
    let prequery_relation = prequery_relation.ok_or_else(|| {
        Error::InvalidPcsOpen("RAA Section 5 relation prequery is missing".to_string())
    })?;
    let auxiliary_proof = auxiliary_proof.ok_or_else(|| {
        Error::InvalidPcsOpen("RAA Section 5 auxiliary query proof is missing".to_string())
    })?;
    match &auxiliary_proof.raa_relation {
        RaaRelationProof::Section5(query_relation) if query_relation == prequery_relation => Ok(()),
        RaaRelationProof::Section5(_) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 relation query proof does not match the prequery transcript".to_string(),
        )),
        RaaRelationProof::LocalQueries(_) => Err(Error::InvalidPcsOpen(
            "RAA Section 5 schedule carried local relation proof data".to_string(),
        )),
    }
}

fn verify_raa_relation_openings<H: Hash>(
    section5_relation_residual_public: Option<&RaaSection5RelationResidualPublicCommitment<H>>,
    proof: Option<&AuxiliaryOracleQueryProof<H>>,
    section5_relation_residual: Option<&RaaSection5RelationResidualQueryProof<H>>,
    authentication: &BackendProofOracleAuthentication<H>,
    auxiliary_oracle_len: usize,
    schedule: &HolographicQuerySchedule,
    top_queries: &[TopQuery<B128>],
) -> Result<(), Error> {
    if schedule.raa_final_queries().is_empty()
        && schedule.raa_auxiliary_queries().is_empty()
        && schedule.raa_relation_strategy() != RaaRelationProofStrategy::Section5
    {
        return Ok(());
    }
    if auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA relation checks require an auxiliary oracle".to_string(),
        ));
    }
    let proof = proof.ok_or_else(|| {
        Error::InvalidPcsOpen("RAA relation checks require auxiliary query openings".to_string())
    })?;
    proof.raa_relation.verify_openings::<H>(
        &proof.relation_queries,
        section5_relation_residual_public,
        section5_relation_residual,
        authentication,
        auxiliary_oracle_len,
        schedule,
        top_queries,
    )
}

fn verify_section5_relation_residual_openings<H: Hash>(
    proof: Option<&RaaSection5RelationResidualQueryProof<H>>,
    schedule: &HolographicQuerySchedule,
) -> Result<(), Error> {
    if schedule.raa_relation_strategy() != RaaRelationProofStrategy::Section5 {
        if proof.is_some() {
            return Err(Error::InvalidPcsOpen(
                "Section 5 residual openings were supplied for a non-Section 5 schedule"
                    .to_string(),
            ));
        }
        return Ok(());
    }
    let expected_count = schedule.section5_relation_residual_proof_query_count();
    if expected_count == 0 {
        if proof.map(|proof| proof.query_count() != 0).unwrap_or(false) {
            return Err(Error::InvalidPcsOpen(
                "Section 5 residual openings were supplied even though none were scheduled"
                    .to_string(),
            ));
        }
        return Ok(());
    }
    let proof = proof.ok_or_else(|| {
        Error::InvalidPcsOpen(
            "Section 5 residual openings are missing for scheduled residual queries".to_string(),
        )
    })?;
    if proof.query_count() != expected_count {
        return Err(Error::InvalidPcsOpen(
            "Section 5 residual opening count does not match the schedule".to_string(),
        ));
    }
    for query in &proof.queries {
        if query.value != B128::ZERO {
            return Err(Error::InvalidPcsOpen(
                "Section 5 residual opening is nonzero".to_string(),
            ));
        }
    }
    Ok(())
}

fn merkelize_b128_padded<H: Hash>(values: &[B128]) -> Vec<Vec<Output<H>>> {
    debug_assert!(!values.is_empty());
    let padded_len = values.len().next_power_of_two();
    let mut leaves = Vec::with_capacity(padded_len);
    for value in values {
        leaves.push(hash_b128_leaf::<H>(value));
    }
    for _ in values.len()..padded_len {
        leaves.push(hash_b128_leaf::<H>(&B128::ZERO));
    }

    let mut tree = Vec::with_capacity(log2_strict(padded_len) + 1);
    tree.push(leaves);
    while tree[tree.len() - 1].len() > 1 {
        let level = &tree[tree.len() - 1];
        let mut parents = Vec::with_capacity(level.len() >> 1);
        for children in level.chunks_exact(2) {
            let mut hasher = H::new();
            let mut hash = Output::<H>::default();
            hasher.update(&children[0]);
            hasher.update(&children[1]);
            hasher.finalize_into_reset(&mut hash);
            parents.push(hash);
        }
        tree.push(parents);
    }
    tree
}

fn merkle_padded_sibling_path<H: Hash>(
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

fn merkle_b128_multiproof_nodes<H: Hash, I: IntoIterator<Item = (usize, B128)>>(
    tree: &[Vec<Output<H>>],
    queries: I,
) -> Result<Vec<Output<H>>, Error> {
    let mut known = BTreeMap::new();
    for (logical_index, value) in queries {
        validate_merkle_b128_query_index(logical_index, tree[0].len())?;
        let hash = hash_b128_leaf::<H>(&value);
        if let Some(existing) = known.insert(logical_index, hash.clone()) {
            if existing != hash {
                return Err(Error::InvalidPcsOpen(
                    "duplicate Merkle query index has conflicting values".to_string(),
                ));
            }
        }
    }
    if known.is_empty() {
        return Ok(Vec::new());
    }

    let mut authentication_nodes = Vec::new();
    for level in tree.iter().take(tree.len().saturating_sub(1)) {
        let mut next = BTreeMap::new();
        for (&index, hash) in known.iter() {
            let sibling_index = index ^ 1;
            if index & 1 == 1 && known.contains_key(&sibling_index) {
                continue;
            }
            let sibling = if let Some(sibling) = known.get(&sibling_index) {
                sibling.clone()
            } else {
                let sibling = level[sibling_index].clone();
                authentication_nodes.push(sibling.clone());
                sibling
            };
            let parent = if index & 1 == 0 {
                hash_pair::<H>(hash, &sibling)
            } else {
                hash_pair::<H>(&sibling, hash)
            };
            next.insert(index >> 1, parent);
        }
        known = next;
    }
    Ok(authentication_nodes)
}

fn verify_merkle_b128_multiproof<H: Hash, I: IntoIterator<Item = (usize, B128)>>(
    root: &Output<H>,
    len: usize,
    queries: I,
    authentication_nodes: &[Output<H>],
) -> Result<(), Error> {
    let mut known = BTreeMap::new();
    for (logical_index, value) in queries {
        validate_merkle_b128_query_index(logical_index, len)?;
        let hash = hash_b128_leaf::<H>(&value);
        if let Some(existing) = known.insert(logical_index, hash.clone()) {
            if existing != hash {
                return Err(Error::InvalidPcsOpen(
                    "duplicate Merkle query index has conflicting values".to_string(),
                ));
            }
        }
    }
    if known.is_empty() {
        if authentication_nodes.is_empty() {
            return Ok(());
        }
        return Err(Error::InvalidPcsOpen(
            "Merkle multiproof has authentication nodes but no queries".to_string(),
        ));
    }

    let padded_len = len.next_power_of_two();
    let expected_rounds = log2_strict(padded_len);
    let mut supplied = authentication_nodes.iter();
    for _ in 0..expected_rounds {
        let mut next = BTreeMap::new();
        for (&index, hash) in known.iter() {
            let sibling_index = index ^ 1;
            if index & 1 == 1 && known.contains_key(&sibling_index) {
                continue;
            }
            let sibling = if let Some(sibling) = known.get(&sibling_index) {
                sibling.clone()
            } else {
                supplied
                    .next()
                    .ok_or_else(|| {
                        Error::InvalidPcsOpen(
                            "Merkle multiproof is missing an authentication node".to_string(),
                        )
                    })?
                    .clone()
            };
            let parent = if index & 1 == 0 {
                hash_pair::<H>(hash, &sibling)
            } else {
                hash_pair::<H>(&sibling, hash)
            };
            next.insert(index >> 1, parent);
        }
        known = next;
    }
    if supplied.next().is_some() {
        return Err(Error::InvalidPcsOpen(
            "Merkle multiproof has too many authentication nodes".to_string(),
        ));
    }
    if known.len() != 1 || known.values().next().expect("known root exists") != root {
        return Err(Error::InvalidPcsOpen(
            "Merkle multiproof does not authenticate".to_string(),
        ));
    }
    Ok(())
}

fn validate_merkle_b128_query_index(index: usize, len: usize) -> Result<(), Error> {
    if len == 0 {
        return Err(Error::InvalidPcsOpen(
            "Merkle query domain is empty".to_string(),
        ));
    }
    if index >= len {
        return Err(Error::InvalidPcsOpen(format!(
            "Merkle query index {index} is outside length {len}"
        )));
    }
    Ok(())
}

fn hash_pair<H: Hash>(left: &Output<H>, right: &Output<H>) -> Output<H> {
    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize_into_reset(&mut hash);
    hash
}

fn hash_b128_leaf<H: Hash>(value: &B128) -> Output<H> {
    let mut hasher = H::new();
    let mut hash = Output::<H>::default();
    hasher.update_field_element(value);
    hasher.finalize_into_reset(&mut hash);
    hash
}

fn systematic_to_physical(
    logical_index: usize,
    message_len: usize,
    parity_expansion_factor: usize,
) -> usize {
    debug_assert!(message_len.is_power_of_two());
    debug_assert!(logical_index < message_len);
    if message_len == 1 {
        0
    } else {
        let half_message_len = message_len >> 1;
        let half_codeword_len = half_message_len * (parity_expansion_factor + 1);
        if logical_index < half_message_len {
            systematic_to_physical(logical_index, half_message_len, parity_expansion_factor)
        } else {
            half_codeword_len
                + systematic_to_physical(
                    logical_index - half_message_len,
                    half_message_len,
                    parity_expansion_factor,
                )
        }
    }
}

fn parity_to_physical(
    logical_index: usize,
    message_len: usize,
    parity_expansion_factor: usize,
) -> usize {
    debug_assert!(message_len.is_power_of_two());
    debug_assert!(logical_index < message_len * parity_expansion_factor);
    if message_len == 1 {
        1 + logical_index
    } else {
        let half_message_len = message_len >> 1;
        let half_parity_len = half_message_len * parity_expansion_factor;
        let half_codeword_len = half_message_len * (parity_expansion_factor + 1);
        if logical_index < half_parity_len {
            parity_to_physical(logical_index, half_message_len, parity_expansion_factor)
        } else {
            half_codeword_len
                + parity_to_physical(
                    logical_index - half_parity_len,
                    half_message_len,
                    parity_expansion_factor,
                )
        }
    }
}

fn physical_to_logical(
    physical_index: usize,
    message_len: usize,
    parity_expansion_factor: usize,
) -> CodewordAddress {
    debug_assert!(physical_index < message_len * (parity_expansion_factor + 1));
    if message_len == 1 {
        if physical_index == 0 {
            CodewordAddress {
                part: CodewordPart::Systematic,
                local_index: 0,
            }
        } else {
            CodewordAddress {
                part: CodewordPart::Parity,
                local_index: physical_index - 1,
            }
        }
    } else {
        let half_message_len = message_len >> 1;
        let half_codeword_len = half_message_len * (parity_expansion_factor + 1);
        if physical_index < half_codeword_len {
            physical_to_logical(physical_index, half_message_len, parity_expansion_factor)
        } else {
            let mut address = physical_to_logical(
                physical_index - half_codeword_len,
                half_message_len,
                parity_expansion_factor,
            );
            match address.part {
                CodewordPart::Systematic => address.local_index += half_message_len,
                CodewordPart::Parity => {
                    address.local_index += half_message_len * parity_expansion_factor
                }
            }
            address
        }
    }
}

fn log2_strict(value: usize) -> usize {
    debug_assert!(value.is_power_of_two());
    usize::BITS as usize - 1 - value.leading_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::blaze2::{
        build_raa_aux_trace, evaluate_multilinear, Blaze2CodeSeed, Blaze2FieldId, Blaze2HashId,
        Blaze2LeafLayout, Blaze2PackingLayout, RaaVariant,
    };
    use crate::backend::hash::Blake2s;

    fn layout(message_len: usize, parity_expansion_factor: usize) -> SystematicAugmentedRfcLayout {
        SystematicAugmentedRfcLayout::new(SystematicFoldableCodeSpec {
            version: 1,
            compiler_message_len: message_len,
            compiler_systematic_len: message_len,
            compiler_parity_len: message_len * parity_expansion_factor,
            compiler_codeword_len: message_len * (parity_expansion_factor + 1),
            parity_expansion_factor,
            seed: [7; 32],
        })
        .unwrap()
    }

    fn schedule_spec() -> HolographicQueryScheduleSpec {
        HolographicQueryScheduleSpec {
            q_raa_input: 6,
            q_backend_proof: 7,
            auxiliary_oracle_len: 16 * BLAZE2_BASEFOLD_AUXILIARY_ROW_COUNT,
            raa_relation_strategy: RaaRelationProofStrategy::LocalQueries,
        }
    }

    fn message(len: usize, offset: u64) -> Vec<B128> {
        (0..len)
            .map(|index| B128::from(offset + index as u64 * 13))
            .collect()
    }

    fn section5_relation_check_for_row(
        row: &[B128],
        challenges: &[B128],
    ) -> RaaRelationSumcheckCheck {
        RaaRelationSumcheckCheck {
            challenges: challenges.to_vec(),
            initial_sum: B128::ZERO,
            terminal_claim: fold_eval_sumcheck_vector(row.to_vec(), challenges).unwrap(),
        }
    }

    fn section5_terminal_binding_fixture() -> (RaaSection5RelationChecks, Vec<B128>, usize) {
        let domain_len = 8;
        let permutation_row = message(domain_len, 31);
        let first_accumulator_row = message(domain_len, 211);
        let second_accumulator_row = message(domain_len, 409);
        let permutation_challenges = [B128::from(3), B128::from(5), B128::from(7)];
        let first_accumulator_challenges = [B128::from(11), B128::from(13), B128::from(17)];
        let second_accumulator_challenges = [B128::from(19), B128::from(23), B128::from(29)];
        let checks = RaaSection5RelationChecks {
            permutation: section5_relation_check_for_row(&permutation_row, &permutation_challenges),
            first_accumulator: section5_relation_check_for_row(
                &first_accumulator_row,
                &first_accumulator_challenges,
            ),
            second_accumulator: section5_relation_check_for_row(
                &second_accumulator_row,
                &second_accumulator_challenges,
            ),
        };
        assert_ne!(checks.permutation.terminal_claim, B128::ZERO);
        assert_ne!(checks.first_accumulator.terminal_claim, B128::ZERO);
        assert_ne!(checks.second_accumulator.terminal_claim, B128::ZERO);
        let mut residual_values =
            Vec::with_capacity(domain_len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT);
        residual_values.extend_from_slice(&permutation_row);
        residual_values.extend_from_slice(&first_accumulator_row);
        residual_values.extend_from_slice(&second_accumulator_row);
        (checks, residual_values, domain_len)
    }

    #[test]
    fn product_tree_local_queries_match_packed_relation_residuals() {
        let leaves = vec![
            B128::from(3),
            B128::from(5),
            B128::from(7),
            B128::from(11),
            B128::from(13),
            B128::from(17),
            B128::from(19),
            B128::from(23),
        ];
        let tree = ProductTree::new(leaves).unwrap();
        let packed = tree.packed_witness_evals();
        let residuals = product_tree_packed_relation_residuals(&packed).unwrap();
        for (index, expected) in residuals.iter().copied().enumerate() {
            let query = product_tree_packed_relation_local_query(&packed, index).unwrap();
            assert_eq!(query.residual, expected);
            for (opening_index, value) in query.openings {
                assert_eq!(packed[opening_index], value);
            }
        }
    }

    #[test]
    fn section5_permutation_residual_local_queries_match_helper_oracle() {
        let mut spec = blaze2_backend_spec();
        spec.raa_relation_strategy = RaaRelationProofStrategy::Section5;
        spec.auxiliary_oracle_len =
            required_blaze2_basefold_section5_auxiliary_oracle_len(&spec.praa);
        let params = Blaze2BaseFoldBackendParams::new(spec).unwrap();
        let request_point = make_request_point(&params, 37);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 89, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        let challenges = squeeze_raa_section5_relation_challenges(
            params.spec(),
            &prequery.compiler_parity,
            prequery.auxiliary.as_ref(),
            &request,
        );
        let helper_values = state.section5_permutation_helper.as_ref().unwrap().values();
        let residuals = build_raa_section5_relation_residual_oracle(
            params.praa().packed(),
            &auxiliary,
            &folded_codeword,
            helper_values,
            challenges,
        )
        .unwrap();
        let len = params.praa().packed().codeword_len();
        for (index, expected) in residuals[..len].iter().copied().enumerate() {
            let query = raa_section5_permutation_residual_local_query(
                len,
                helper_values,
                challenges.gamma,
                index,
            )
            .unwrap();
            assert_eq!(query.residual, expected);
            assert!(!query.helper_openings.is_empty());
            for (opening_index, value) in query.helper_openings {
                assert_eq!(helper_values[opening_index], value);
            }
        }

        let mut tampered_helper = helper_values.to_vec();
        tampered_helper[0] += B128::ONE;
        let tampered_query = raa_section5_permutation_residual_local_query(
            len,
            &tampered_helper,
            challenges.gamma,
            0,
        )
        .unwrap();
        assert_ne!(tampered_query.residual, residuals[0]);
    }

    fn blaze2_backend_spec() -> Blaze2BaseFoldBackendSpec {
        let praa = Blaze2CodeSpec {
            version: 1,
            field_id: Blaze2FieldId::B128,
            hash_id: Blaze2HashId::Blake2s256,
            raa_variant: RaaVariant::PackedPrefixAccumulator,
            packing: Blaze2PackingLayout::PackedInterleavedRows,
            leaf_layout: Blaze2LeafLayout::InterleavedColumn,
            praa_message_len: 8,
            praa_expansion_factor: 4,
            praa_codeword_len: 32,
            seed: Blaze2CodeSeed([19; 32]),
        };
        Blaze2BaseFoldBackendSpec {
            praa: praa.clone(),
            compiler_code: SystematicFoldableCodeSpec {
                version: 1,
                compiler_message_len: 32,
                compiler_systematic_len: 32,
                compiler_parity_len: 32 * 3,
                compiler_codeword_len: 32 * 4,
                parity_expansion_factor: 3,
                seed: [23; 32],
            },
            q_raa_input: 6,
            q_backend_proof: 7,
            auxiliary_oracle_len: required_blaze2_basefold_auxiliary_oracle_len(&praa),
            raa_relation_strategy: RaaRelationProofStrategy::LocalQueries,
        }
    }

    fn folded_codeword_and_auxiliary(
        params: &Blaze2BaseFoldBackendParams,
        offset: u64,
        request_point: &[B128],
    ) -> (Vec<B128>, Vec<B128>, B128) {
        let folded_message = message(params.praa().packed().message_len(), offset);
        let mut scratch = vec![B128::ZERO; folded_message.len()];
        let folded_eval =
            evaluate_multilinear(&folded_message, request_point, &mut scratch).unwrap();
        let folded_codeword = params.praa().packed().encode_row(&folded_message);
        let trace = build_raa_aux_trace(params.praa().packed(), &folded_message).unwrap();
        let mut auxiliary = Vec::with_capacity(params.spec().auxiliary_oracle_len);
        auxiliary.extend_from_slice(&trace.u2);
        auxiliary.extend_from_slice(&trace.u3);
        auxiliary.extend_from_slice(&trace.u4);
        auxiliary.resize(params.spec().auxiliary_oracle_len, B128::ZERO);
        assert_eq!(auxiliary.len(), params.spec().auxiliary_oracle_len);
        (folded_codeword, auxiliary, folded_eval)
    }

    fn make_request_point(params: &Blaze2BaseFoldBackendParams, offset: u64) -> Vec<B128> {
        message(log2_strict(params.praa().packed().message_len()), offset)
    }

    fn open_request<'a>(col_point: &'a [B128]) -> Blaze2BaseFoldOpenRequest<'a> {
        Blaze2BaseFoldOpenRequest {
            col_point,
            folded_eval: B128::ONE,
        }
    }

    fn open_request_with_eval<'a>(
        col_point: &'a [B128],
        folded_eval: B128,
    ) -> Blaze2BaseFoldOpenRequest<'a> {
        Blaze2BaseFoldOpenRequest {
            col_point,
            folded_eval,
        }
    }

    fn explicit_schedule(layout: &SystematicAugmentedRfcLayout) -> HolographicQuerySchedule {
        HolographicQuerySchedule {
            input_queries: vec![
                SystematicInputQuery {
                    logical_index: 2,
                    physical_index: layout.systematic_to_physical(2).unwrap(),
                },
                SystematicInputQuery {
                    logical_index: 5,
                    physical_index: layout.systematic_to_physical(5).unwrap(),
                },
            ],
            proof_queries: vec![
                BackendProofQuery {
                    domain: BackendProofQueryDomain::CompilerParity,
                    index: 0,
                    physical_index: Some(layout.parity_to_physical(0).unwrap()),
                },
                BackendProofQuery {
                    domain: BackendProofQueryDomain::RelationAuxiliary,
                    index: 3,
                    physical_index: None,
                },
                BackendProofQuery {
                    domain: BackendProofQueryDomain::CompilerParity,
                    index: layout.parity_len() - 1,
                    physical_index: Some(
                        layout.parity_to_physical(layout.parity_len() - 1).unwrap(),
                    ),
                },
            ],
            raa_relation_strategy: RaaRelationProofStrategy::LocalQueries,
            raa_final_queries: Vec::new(),
            raa_auxiliary_queries: Vec::new(),
            raa_auxiliary_authentication_queries: Vec::new(),
        }
    }

    #[test]
    fn rejects_non_power_of_two_total_expansion() {
        let err = SystematicAugmentedRfcLayout::new(SystematicFoldableCodeSpec {
            version: 1,
            compiler_message_len: 8,
            compiler_systematic_len: 8,
            compiler_parity_len: 16,
            compiler_codeword_len: 24,
            parity_expansion_factor: 2,
            seed: [0; 32],
        })
        .unwrap_err();
        assert!(matches!(err, Error::InvalidPcsParam(_)));
    }

    #[test]
    fn expanded_code_is_deterministic_from_spec() {
        let spec = layout(8, 3).spec().clone();
        let lhs = SystematicAugmentedRfcCode::new(spec.clone()).unwrap();
        let rhs = SystematicAugmentedRfcCode::new(spec).unwrap();
        assert_eq!(lhs, rhs);

        let mut changed_spec = lhs.layout().spec().clone();
        changed_spec.seed[0] ^= 1;
        let changed = SystematicAugmentedRfcCode::new(changed_spec).unwrap();
        assert_ne!(lhs.parity_fold_table(), changed.parity_fold_table());
    }

    #[test]
    fn blaze2_backend_params_pin_praa_and_compiler_domains() {
        let spec = blaze2_backend_spec();
        let params = Blaze2BaseFoldBackendParams::new(spec.clone()).unwrap();

        assert_eq!(params.spec(), &spec);
        assert_eq!(
            params.praa().packed().codeword_len(),
            spec.praa.praa_codeword_len
        );
        assert_eq!(
            params.compiler_code().layout().message_len(),
            spec.praa.praa_codeword_len
        );
        assert_eq!(
            params.compiler_code().layout().systematic_len(),
            spec.praa.praa_codeword_len
        );
        assert_eq!(
            params.query_schedule_spec(),
            HolographicQueryScheduleSpec {
                q_raa_input: spec.q_raa_input,
                q_backend_proof: spec.q_backend_proof,
                auxiliary_oracle_len: spec.auxiliary_oracle_len,
                raa_relation_strategy: spec.raa_relation_strategy,
            }
        );
    }

    #[test]
    fn section5_terminal_binding_accepts_residual_row_evaluations() {
        let (checks, residual_values, domain_len) = section5_terminal_binding_fixture();

        verify_section5_relation_terminal_evaluations_from_residual_values(
            &checks,
            &residual_values,
            domain_len,
        )
        .unwrap();
    }

    #[test]
    fn section5_terminal_binding_rejects_tampered_terminal_claim() {
        let (mut checks, residual_values, domain_len) = section5_terminal_binding_fixture();
        checks.first_accumulator.terminal_claim += B128::ONE;

        let err = verify_section5_relation_terminal_evaluations_from_residual_values(
            &checks,
            &residual_values,
            domain_len,
        )
        .unwrap_err();
        assert!(format!("{err:?}")
            .contains("first accumulator terminal residual evaluation does not match"));
    }

    #[test]
    fn section5_terminal_binding_rejects_tampered_residual_row() {
        let (checks, mut residual_values, domain_len) = section5_terminal_binding_fixture();
        residual_values[domain_len * 2 + 3] += B128::ONE;

        let err = verify_section5_relation_terminal_evaluations_from_residual_values(
            &checks,
            &residual_values,
            domain_len,
        )
        .unwrap_err();
        assert!(format!("{err:?}")
            .contains("second accumulator terminal residual evaluation does not match"));
    }

    #[test]
    fn section5_terminal_binding_rejects_bad_shapes() {
        let (mut checks, mut residual_values, domain_len) = section5_terminal_binding_fixture();
        residual_values.pop();
        let err = verify_section5_relation_terminal_evaluations_from_residual_values(
            &checks,
            &residual_values,
            domain_len,
        )
        .unwrap_err();
        assert!(format!("{err:?}").contains("residual terminal values are not row-aligned"));

        let (_, residual_values, domain_len) = section5_terminal_binding_fixture();
        checks.permutation.challenges.pop();
        let err = verify_section5_relation_terminal_evaluations_from_residual_values(
            &checks,
            &residual_values,
            domain_len,
        )
        .unwrap_err();
        assert!(format!("{err:?}").contains("permutation terminal challenge length is invalid"));
    }

    #[test]
    fn section5_auxiliary_domain_separates_post_challenge_helper_oracles() {
        let mut spec = blaze2_backend_spec();
        let local_len = required_blaze2_basefold_auxiliary_oracle_len(&spec.praa);
        let section5_len = required_blaze2_basefold_section5_auxiliary_oracle_len(&spec.praa);
        let helper_len =
            required_blaze2_basefold_section5_permutation_helper_oracle_len(&spec.praa);
        assert_eq!(local_len, spec.praa.praa_codeword_len * RAA_AUX_ROW_COUNT);
        assert_eq!(section5_len, local_len);
        assert_eq!(
            helper_len,
            spec.praa.praa_codeword_len * raa_section5_permutation_helper_rows().len()
        );

        spec.raa_relation_strategy = RaaRelationProofStrategy::Section5;
        spec.auxiliary_oracle_len = section5_len;
        let params = Blaze2BaseFoldBackendParams::new(spec.clone()).unwrap();
        assert_eq!(params.spec().auxiliary_oracle_len, section5_len);

        let schedule_spec = params.query_schedule_spec();
        assert_eq!(
            schedule_spec.auxiliary_oracle_len,
            required_blaze2_basefold_auxiliary_oracle_len_for_strategy(
                &spec.praa,
                RaaRelationProofStrategy::Section5
            )
        );

        let request_point = make_request_point(&params, 17);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 19, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        assert_eq!(
            prequery.section5_permutation_helper.as_ref().unwrap().len,
            helper_len
        );
        assert_eq!(
            state.section5_permutation_helper.as_ref().unwrap().len(),
            helper_len
        );
        let challenges = squeeze_raa_section5_relation_challenges(
            params.spec(),
            &prequery.compiler_parity,
            prequery.auxiliary.as_ref(),
            &request,
        );
        let expected_helper = build_raa_section5_permutation_helper_oracle(
            params.praa().packed(),
            &auxiliary,
            challenges,
        )
        .unwrap();
        assert_eq!(
            state.section5_permutation_helper.as_ref().unwrap().values(),
            expected_helper
        );
        assert!(expected_helper.iter().any(|value| *value != B128::ZERO));
    }

    #[test]
    fn blaze2_backend_params_reject_domain_mismatches() {
        let mut spec = blaze2_backend_spec();
        spec.compiler_code.compiler_message_len >>= 1;
        spec.compiler_code.compiler_systematic_len >>= 1;
        spec.compiler_code.compiler_parity_len >>= 1;
        spec.compiler_code.compiler_codeword_len >>= 1;
        assert!(Blaze2BaseFoldBackendParams::new(spec).is_err());

        let mut spec = blaze2_backend_spec();
        spec.q_raa_input = 0;
        assert!(Blaze2BaseFoldBackendParams::new(spec).is_err());

        let mut spec = blaze2_backend_spec();
        spec.auxiliary_oracle_len = 0;
        assert!(Blaze2BaseFoldBackendParams::new(spec).is_err());
    }

    #[test]
    fn logical_physical_mapping_round_trips() {
        let layout = layout(8, 3);
        let mut seen = vec![false; layout.codeword_len()];

        for logical_index in 0..layout.systematic_len() {
            let physical = layout.systematic_to_physical(logical_index).unwrap();
            assert!(!seen[physical]);
            seen[physical] = true;
            assert_eq!(
                layout.physical_to_logical(physical).unwrap(),
                CodewordAddress {
                    part: CodewordPart::Systematic,
                    local_index: logical_index,
                }
            );
        }

        for logical_index in 0..layout.parity_len() {
            let physical = layout.parity_to_physical(logical_index).unwrap();
            assert!(!seen[physical]);
            seen[physical] = true;
            assert_eq!(
                layout.physical_to_logical(physical).unwrap(),
                CodewordAddress {
                    part: CodewordPart::Parity,
                    local_index: logical_index,
                }
            );
        }

        assert!(seen.into_iter().all(|was_seen| was_seen));
    }

    #[test]
    fn fold_pairs_preserve_part_and_local_halves() {
        let layout = layout(8, 3);
        for round in 0..layout.num_rounds() {
            let current_message_len = layout.message_len() >> round;
            let child_message_len = current_message_len >> 1;
            let current_codeword_len = current_message_len * (layout.parity_expansion_factor() + 1);
            for output_index in 0..(current_codeword_len >> 1) {
                let pair = layout.fold_pair(round, output_index).unwrap();
                let left = layout
                    .physical_to_logical_at_round(round, pair.left)
                    .unwrap();
                let right = layout
                    .physical_to_logical_at_round(round, pair.right)
                    .unwrap();
                let out = layout
                    .physical_to_logical_at_round(round + 1, pair.out)
                    .unwrap();

                assert_eq!(left.part, right.part);
                assert_eq!(left.part, out.part);
                match left.part {
                    CodewordPart::Systematic => {
                        assert!(left.local_index < child_message_len);
                        assert_eq!(right.local_index, left.local_index + child_message_len);
                        assert_eq!(out.local_index, left.local_index);
                    }
                    CodewordPart::Parity => {
                        let child_parity_len = child_message_len * layout.parity_expansion_factor();
                        assert!(left.local_index < child_parity_len);
                        assert_eq!(right.local_index, left.local_index + child_parity_len);
                        assert_eq!(out.local_index, left.local_index);
                    }
                }
            }
        }
    }

    #[test]
    fn systematic_and_parity_fold_rules_are_distinct() {
        let left = B128::from(3);
        let right = B128::from(7);
        let alpha = B128::from(5);

        assert_eq!(
            fold_systematic_pair(left, right, B128::ZERO),
            left,
            "systematic coordinates interpolate at T_sys = 0"
        );
        assert_eq!(
            fold_systematic_pair(left, right, B128::ONE),
            right,
            "systematic coordinates interpolate at T_sys' = 1"
        );
        assert_eq!(fold_rfc_parity_pair(left, right, B128::ZERO), left);
        assert_ne!(
            fold_systematic_pair(left, right, alpha),
            fold_rfc_parity_pair(left, right, alpha),
            "raw systematic coordinates must not silently reuse the RFC linear fold"
        );
    }

    #[test]
    fn parity_encoding_is_linear_and_seeded() {
        let spec = layout(8, 3).spec().clone();
        let code = SystematicAugmentedRfcCode::new(spec.clone()).unwrap();
        let lhs_message = message(code.layout().message_len(), 5);
        let rhs_message = message(code.layout().message_len(), 91);
        let sum_message = lhs_message
            .iter()
            .zip(&rhs_message)
            .map(|(&lhs, &rhs)| lhs + rhs)
            .collect::<Vec<_>>();

        let mut lhs = vec![B128::ZERO; code.layout().parity_len()];
        let mut rhs = vec![B128::ZERO; code.layout().parity_len()];
        let mut sum = vec![B128::ZERO; code.layout().parity_len()];
        code.encode_parity_into(&lhs_message, &mut lhs).unwrap();
        code.encode_parity_into(&rhs_message, &mut rhs).unwrap();
        code.encode_parity_into(&sum_message, &mut sum).unwrap();

        for index in 0..sum.len() {
            assert_eq!(sum[index], lhs[index] + rhs[index]);
        }

        let mut changed_spec = spec;
        changed_spec.seed[0] ^= 1;
        let changed_code = SystematicAugmentedRfcCode::new(changed_spec).unwrap();
        let mut changed = vec![B128::ZERO; changed_code.layout().parity_len()];
        changed_code
            .encode_parity_into(&lhs_message, &mut changed)
            .unwrap();
        assert_ne!(lhs, changed);
    }

    #[test]
    fn encoded_codeword_is_closed_under_round_folds() {
        for (message_len, parity_expansion_factor) in [(1, 3), (2, 3), (8, 3), (16, 7)] {
            let code = SystematicAugmentedRfcCode::new(
                layout(message_len, parity_expansion_factor).spec().clone(),
            )
            .unwrap();
            let mut current_message = message(code.layout().message_len(), 11);
            let mut current_codeword = vec![B128::ZERO; code.layout().codeword_len()];
            let mut parity_scratch = vec![B128::ZERO; code.layout().parity_len()];
            code.encode_physical_codeword_into(
                &current_message,
                &mut current_codeword,
                &mut parity_scratch,
            )
            .unwrap();

            for round in 0..code.layout().num_rounds() {
                let alpha = B128::from(101 + round as u64 * 17);
                let half_message_len = current_message.len() >> 1;
                let next_message = (0..half_message_len)
                    .map(|index| {
                        fold_systematic_pair(
                            current_message[index],
                            current_message[index + half_message_len],
                            alpha,
                        )
                    })
                    .collect::<Vec<_>>();

                let mut folded_codeword = vec![B128::ZERO; current_codeword.len() >> 1];
                code.fold_physical_codeword_round_into(
                    round,
                    &current_codeword,
                    alpha,
                    &mut folded_codeword,
                )
                .unwrap();

                let next_parity_len = next_message.len() * code.layout().parity_expansion_factor();
                let mut expected = vec![B128::ZERO; folded_codeword.len()];
                let mut expected_parity = vec![B128::ZERO; next_parity_len];
                code.encode_physical_codeword_at_round_into(
                    round + 1,
                    &next_message,
                    &mut expected,
                    &mut expected_parity,
                )
                .unwrap();

                assert_eq!(
                    folded_codeword, expected,
                    "message_len={message_len} parity_expansion_factor={parity_expansion_factor} round={round} fold must produce the next encoded codeword"
                );
                current_message = next_message;
                current_codeword = folded_codeword;
            }
        }
    }

    #[test]
    fn logical_and_physical_codeword_encodings_agree() {
        let code = SystematicAugmentedRfcCode::new(layout(8, 3).spec().clone()).unwrap();
        let values = message(code.layout().message_len(), 17);
        let mut logical = vec![B128::ZERO; code.layout().codeword_len()];
        let mut physical = vec![B128::ZERO; code.layout().codeword_len()];
        let mut parity_scratch = vec![B128::ZERO; code.layout().parity_len()];

        code.encode_logical_codeword_into(&values, &mut logical)
            .unwrap();
        code.encode_physical_codeword_into(&values, &mut physical, &mut parity_scratch)
            .unwrap();

        assert_eq!(
            &logical[..code.layout().systematic_len()],
            values.as_slice()
        );
        assert_eq!(
            &logical[code.layout().systematic_len()..],
            parity_scratch.as_slice()
        );

        for (physical_index, &value) in physical.iter().enumerate() {
            let address = code.layout().physical_to_logical(physical_index).unwrap();
            let logical_index = match address.part {
                CodewordPart::Systematic => address.local_index,
                CodewordPart::Parity => code.layout().systematic_len() + address.local_index,
            };
            assert_eq!(value, logical[logical_index]);
        }
    }

    #[test]
    fn encoding_rejects_bad_shapes() {
        let code = SystematicAugmentedRfcCode::new(layout(8, 3).spec().clone()).unwrap();
        let values = message(code.layout().message_len(), 3);
        let mut parity = vec![B128::ZERO; code.layout().parity_len()];
        assert!(code
            .encode_parity_into(&values[..values.len() - 1], &mut parity)
            .is_err());
        parity.pop();
        assert!(code.encode_parity_into(&values, &mut parity).is_err());

        let mut codeword = vec![B128::ZERO; code.layout().codeword_len() - 1];
        assert!(code
            .encode_logical_codeword_into(&values, &mut codeword)
            .is_err());
    }

    #[test]
    fn compiler_parity_commitment_authenticates_queries() {
        let code = SystematicAugmentedRfcCode::new(layout(8, 3).spec().clone()).unwrap();
        let values = message(code.layout().message_len(), 31);
        let commitment = code.commit_parity::<Blake2s>(&values).unwrap();
        let public = commitment.public();

        assert_eq!(public.len, code.layout().parity_len());
        for index in [0usize, 1, 7, code.layout().parity_len() - 1] {
            let query = commitment.query(index).unwrap();
            assert_eq!(query.value, commitment.values()[index]);
            let authentication_nodes = merkle_b128_multiproof_nodes::<Blake2s, _>(
                &commitment.merkle_tree,
                [(query.logical_index, query.value)],
            )
            .unwrap();
            verify_merkle_b128_multiproof::<Blake2s, _>(
                &public.root,
                public.len,
                [(query.logical_index, query.value)],
                &authentication_nodes,
            )
            .unwrap();
        }
    }

    #[test]
    fn compiler_parity_query_proof_follows_typed_schedule_order() {
        let code = SystematicAugmentedRfcCode::new(layout(8, 3).spec().clone()).unwrap();
        let values = message(code.layout().message_len(), 37);
        let commitment = code.commit_parity::<Blake2s>(&values).unwrap();
        let public = commitment.public();
        let schedule = explicit_schedule(code.layout());
        let proof = commitment.prove_schedule(&schedule).unwrap();

        assert_eq!(proof.queries.len(), 2);
        assert_eq!(proof.queries[0].logical_index, 0);
        assert_eq!(
            proof.queries[1].logical_index,
            code.layout().parity_len() - 1
        );
        proof
            .verify_schedule(&public, code.layout(), &schedule)
            .unwrap();

        let mut swapped = proof.clone();
        swapped.queries.swap(0, 1);
        assert!(swapped
            .verify_schedule(&public, code.layout(), &schedule)
            .is_err());

        assert!(proof.authentication_nodes.is_empty());
    }

    #[test]
    fn backend_prequery_round_trips_through_scheduled_queries() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 7);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 41, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();

        assert_eq!(
            prequery.compiler_parity.len,
            params.compiler_code().layout().parity_len()
        );
        assert_eq!(
            prequery.folded_parity_layers.len(),
            params.compiler_code().layout().num_rounds()
        );
        for (round, public) in prequery.folded_parity_layers.iter().enumerate() {
            assert_eq!(
                public.len,
                params
                    .compiler_code()
                    .layout()
                    .message_len_at_round(round + 1)
                    .unwrap()
                    * params.compiler_code().layout().parity_expansion_factor()
            );
        }
        assert_eq!(
            prequery.auxiliary.as_ref().unwrap().len,
            params.spec().auxiliary_oracle_len
        );

        let mut transcript = CfriTranscript::<Blake2s>::new();
        let schedule = params
            .sample_query_schedule(&mut transcript, &prequery, &request)
            .unwrap();
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        let top_queries = schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: folded_codeword[query.logical_index],
            })
            .collect::<Vec<_>>();

        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &top_queries)
            .unwrap();

        let mut bad_top_queries = top_queries.clone();
        bad_top_queries[0].index ^= 1;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &bad_top_queries)
            .is_err());

        let mut bad_proof = proof;
        if let Some(query) = bad_proof.compiler_parity.queries.first_mut() {
            query.value += B128::ONE;
            assert!(params
                .verify_query_proof(&prequery, &request, &schedule, &bad_proof, &top_queries)
                .is_err());
        }
    }

    #[test]
    fn backend_compiler_parity_fold_paths_are_checked() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 11);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 149, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        let schedule = explicit_schedule(params.compiler_code().layout());
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        let top_queries = schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: folded_codeword[query.logical_index],
            })
            .collect::<Vec<_>>();

        assert_eq!(proof.compiler_parity_folds.paths.len(), 2);
        assert_eq!(
            proof.compiler_parity_folds.paths[0].steps.len(),
            params.compiler_code().layout().num_rounds()
        );
        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &top_queries)
            .unwrap();

        let changed_request_point = make_request_point(&params, 113);
        let changed_request = open_request(&changed_request_point);
        assert!(params
            .verify_query_proof(&prequery, &changed_request, &schedule, &proof, &top_queries)
            .is_err());

        let mut tampered_value = proof.clone();
        tampered_value.compiler_parity_folds.paths[0].steps[0].sibling_value += B128::ONE;
        assert!(params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_value,
                &top_queries
            )
            .is_err());

        let mut tampered_path = proof.clone();
        tampered_path.authentication.compiler_parity_layers[0].authentication_nodes[0][0] ^= 1;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered_path, &top_queries)
            .is_err());

        let mut tampered_root = prequery.clone();
        tampered_root.folded_parity_layers[0].root[0] ^= 1;
        assert!(params
            .verify_query_proof(&tampered_root, &request, &schedule, &proof, &top_queries)
            .is_err());

        let mut tampered_terminal = prequery;
        tampered_terminal.terminal_codeword[0] += B128::ONE;
        assert!(params
            .verify_query_proof(
                &tampered_terminal,
                &request,
                &schedule,
                &proof,
                &top_queries
            )
            .is_err());
    }

    #[test]
    fn backend_auxiliary_queries_are_authenticated() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 13);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 61, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        let schedule = explicit_schedule(params.compiler_code().layout());
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        let top_queries = schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: folded_codeword[query.logical_index],
            })
            .collect::<Vec<_>>();

        assert_eq!(proof.auxiliary.as_ref().unwrap().query_count(), 1);
        assert_eq!(
            proof.auxiliary.as_ref().unwrap().relation_queries[0].logical_index,
            3
        );
        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &top_queries)
            .unwrap();

        let mut tampered = proof.clone();
        tampered.auxiliary.as_mut().unwrap().relation_queries[0].value += B128::ONE;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered, &top_queries)
            .is_err());

        let mut tampered_authentication = proof.clone();
        tampered_authentication.authentication.auxiliary_nodes[0][0] ^= 1;
        assert!(params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_authentication,
                &top_queries
            )
            .is_err());

        let mut missing = proof;
        missing.auxiliary = None;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &missing, &top_queries)
            .is_err());
    }

    #[test]
    fn backend_auxiliary_local_relations_are_checked() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 17);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 89, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();

        let len = params.praa().packed().codeword_len();
        let mut schedule = HolographicQuerySchedule {
            input_queries: Vec::new(),
            proof_queries: vec![
                BackendProofQuery {
                    domain: BackendProofQueryDomain::RelationAuxiliary,
                    index: len + 3,
                    physical_index: None,
                },
                BackendProofQuery {
                    domain: BackendProofQueryDomain::RelationAuxiliary,
                    index: 2 * len + 5,
                    physical_index: None,
                },
            ],
            raa_relation_strategy: RaaRelationProofStrategy::LocalQueries,
            raa_final_queries: Vec::new(),
            raa_auxiliary_queries: Vec::new(),
            raa_auxiliary_authentication_queries: Vec::new(),
        };
        schedule
            .attach_raa_auxiliary_local_queries(params.praa().packed())
            .unwrap();

        assert_eq!(schedule.raa_auxiliary_queries().len(), 2);
        assert_eq!(
            schedule.raa_auxiliary_queries()[0].relation,
            RaaAuxiliaryRelationKind::FirstAccumulatorStep {
                previous_u3_auxiliary_index: len + 2,
                u2_auxiliary_index: 3,
            }
        );
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        assert_eq!(schedule.raa_auxiliary_authentication_query_count(), 3);
        assert_eq!(proof.auxiliary.as_ref().unwrap().query_count(), 5);
        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &[])
            .unwrap();

        let mut tampered_accumulator = proof.clone();
        if let RaaAuxiliaryLocalRelationProof::FirstAccumulatorStep { previous_value } =
            &mut tampered_accumulator
                .auxiliary
                .as_mut()
                .unwrap()
                .raa_relation
                .local_queries_mut()
                .unwrap()
                .local_relation_queries[0]
        {
            *previous_value += B128::ONE;
        } else {
            panic!("expected first accumulator witness");
        }
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered_accumulator, &[])
            .is_err());

        let mut tampered_schedule = schedule.clone();
        tampered_schedule.raa_auxiliary_authentication_queries[0].index += 1;
        assert!(params
            .verify_query_proof(&prequery, &request, &tampered_schedule, &proof, &[])
            .is_err());

        let mut tampered_permutation = proof;
        tampered_permutation.authentication.auxiliary_nodes[0][0] ^= 1;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered_permutation, &[])
            .is_err());
    }

    #[test]
    fn backend_section5_relation_strategy_checks_residual_oracles() {
        let mut spec = blaze2_backend_spec();
        spec.raa_relation_strategy = RaaRelationProofStrategy::Section5;
        spec.q_raa_input = 5;
        spec.auxiliary_oracle_len =
            required_blaze2_basefold_section5_auxiliary_oracle_len(&spec.praa);
        let params = Blaze2BaseFoldBackendParams::new(spec).unwrap();
        let request_point = make_request_point(&params, 17);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 89, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        assert_eq!(
            prequery.section5_permutation_helper.as_ref().unwrap().len,
            required_blaze2_basefold_section5_permutation_helper_oracle_len(&params.spec().praa)
        );
        assert_eq!(
            prequery.section5_relation_residual.as_ref().unwrap().len,
            params.spec().praa.praa_codeword_len * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT
        );
        assert!(prequery.section5_relation.is_some());
        let mut missing_helper = prequery.clone();
        missing_helper.section5_permutation_helper = None;
        let mut transcript = CfriTranscript::<Blake2s>::new();
        assert!(params
            .sample_query_schedule(&mut transcript, &missing_helper, &request)
            .is_err());
        let mut tampered_prequery_relation = prequery.clone();
        tampered_prequery_relation
            .section5_relation
            .as_mut()
            .unwrap()
            .permutation_sumcheck
            .round_polynomials[1][0] += B128::ONE;
        let mut transcript = CfriTranscript::<Blake2s>::new();
        let err = params
            .sample_query_schedule(&mut transcript, &tampered_prequery_relation, &request)
            .unwrap_err();
        assert!(format!("{err:?}").contains("sumcheck consistency"));
        let mut transcript = CfriTranscript::<Blake2s>::new();
        let schedule = params
            .sample_query_schedule(&mut transcript, &prequery, &request)
            .unwrap();

        assert_eq!(
            schedule.raa_relation_strategy(),
            RaaRelationProofStrategy::Section5
        );
        assert_eq!(schedule.input_queries().len(), 5);
        assert!(schedule.raa_final_queries().is_empty());
        assert!(schedule.raa_auxiliary_queries().is_empty());
        assert!(schedule.raa_auxiliary_authentication_queries().is_empty());
        assert_eq!(
            schedule.expected_auxiliary_query_proof_count(),
            schedule.relation_auxiliary_proof_query_count()
        );
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        let residual_terminal = proof
            .section5_relation_residual
            .as_ref()
            .and_then(|residual| residual.terminal_proof.as_ref())
            .expect("Section 5 residual proof carries terminal binding");
        assert_eq!(
            residual_terminal.rows.len(),
            RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT
        );
        assert!(residual_terminal.rows.iter().all(|row| row.paths.len()
            == schedule
                .section5_relation_residual_proof_query_count()
                .max(1)));
        let auxiliary = proof
            .auxiliary
            .as_ref()
            .expect("Section 5 still opens scheduled relation auxiliary proof queries");
        assert_eq!(
            auxiliary.query_count(),
            schedule.relation_auxiliary_proof_query_count()
        );
        assert!(matches!(
            auxiliary.raa_relation,
            RaaRelationProof::Section5(_)
        ));
        assert_eq!(
            auxiliary.raa_relation,
            RaaRelationProof::Section5(prequery.section5_relation.clone().unwrap())
        );
        if let RaaRelationProof::Section5(section5) = &auxiliary.raa_relation {
            let num_vars = log2_strict(params.spec().praa.praa_codeword_len);
            assert_eq!(
                section5.permutation_sumcheck.round_polynomials.len(),
                num_vars + 1
            );
            assert_eq!(
                section5.permutation_sumcheck.round_polynomials[0].len(),
                RAA_SECTION5_PERMUTATION_SUMCHECK_DEGREE + 1
            );
            assert_eq!(
                section5.first_accumulator_sumcheck.round_polynomials.len(),
                num_vars + 1
            );
            assert_eq!(
                section5.first_accumulator_sumcheck.round_polynomials[0].len(),
                RAA_SECTION5_ACCUMULATOR_SUMCHECK_DEGREE + 1
            );
            assert_eq!(
                section5.second_accumulator_sumcheck.round_polynomials.len(),
                num_vars + 1
            );
            assert!(section5.terminal_evaluations.is_empty());
        } else {
            panic!("expected Section 5 relation proof");
        }
        let mut helper_schedule = schedule.clone();
        helper_schedule.proof_queries.push(BackendProofQuery {
            domain: BackendProofQueryDomain::Section5RelationResidual,
            index: 0,
            physical_index: None,
        });
        helper_schedule.proof_queries.push(BackendProofQuery {
            domain: BackendProofQueryDomain::Section5PermutationHelper,
            index: 0,
            physical_index: None,
        });
        let helper_proof = params.open_query_proof(&state, &helper_schedule).unwrap();
        let helper = helper_proof
            .section5_permutation_helper
            .as_ref()
            .expect("explicit Section 5 helper query must be opened");
        assert_eq!(
            helper.query_count(),
            helper_schedule.section5_permutation_helper_proof_query_count()
        );
        let residual = helper_proof
            .section5_relation_residual
            .as_ref()
            .expect("explicit Section 5 residual query must be opened");
        assert_eq!(
            residual.query_count(),
            helper_schedule.section5_relation_residual_proof_query_count()
        );

        let top_queries = schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: folded_codeword[query.logical_index],
            })
            .collect::<Vec<_>>();
        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &top_queries)
            .unwrap();

        let mut tampered_sumcheck = proof.clone();
        if let RaaRelationProof::Section5(section5) =
            &mut tampered_sumcheck.auxiliary.as_mut().unwrap().raa_relation
        {
            section5.permutation_sumcheck.round_polynomials[1][0] += B128::ONE;
        } else {
            panic!("expected Section 5 relation proof");
        }
        let err = params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_sumcheck,
                &top_queries,
            )
            .unwrap_err();
        assert!(format!("{err:?}").contains("does not match the prequery transcript"));

        let mut missing_terminal = proof.clone();
        missing_terminal
            .section5_relation_residual
            .as_mut()
            .unwrap()
            .terminal_proof = None;
        let err = params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &missing_terminal,
                &top_queries,
            )
            .unwrap_err();
        assert!(format!("{err:?}").contains("terminal binding proof is missing"));

        let mut tampered_terminal_value = proof.clone();
        tampered_terminal_value
            .section5_relation_residual
            .as_mut()
            .unwrap()
            .terminal_proof
            .as_mut()
            .unwrap()
            .rows[0]
            .paths[0]
            .top_value += B128::ONE;
        let err = params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_terminal_value,
                &top_queries,
            )
            .unwrap_err();
        assert!(format!("{err:?}").contains("Merkle"));

        let mut tampered_terminal_fold_root = proof.clone();
        assert!(!tampered_terminal_fold_root
            .authentication
            .section5_relation_terminal_folded_layers
            .is_empty());
        tampered_terminal_fold_root
            .authentication
            .section5_relation_terminal_folded_layers[0]
            .root[0] ^= 1;
        let err = params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_terminal_fold_root,
                &top_queries,
            )
            .unwrap_err();
        assert!(format!("{err:?}").contains("Merkle"));

        let mut tampered_terminal_fold_auth = proof.clone();
        assert!(!tampered_terminal_fold_auth
            .authentication
            .section5_relation_terminal_layer_authentication
            .is_empty());
        assert!(!tampered_terminal_fold_auth
            .authentication
            .section5_relation_terminal_layer_authentication[0]
            .authentication_nodes
            .is_empty());
        tampered_terminal_fold_auth
            .authentication
            .section5_relation_terminal_layer_authentication[0]
            .authentication_nodes[0][0] ^= 1;
        let err = params
            .verify_query_proof(
                &prequery,
                &request,
                &schedule,
                &tampered_terminal_fold_auth,
                &top_queries,
            )
            .unwrap_err();
        assert!(format!("{err:?}").contains("Merkle"));

        let mut unbound_terminal_prequery = prequery.clone();
        unbound_terminal_prequery
            .section5_relation
            .as_mut()
            .unwrap()
            .terminal_evaluations
            .push(B128::ONE);
        let mut transcript = CfriTranscript::<Blake2s>::new();
        let err = params
            .sample_query_schedule(&mut transcript, &unbound_terminal_prequery, &request)
            .unwrap_err();
        assert!(format!("{err:?}").contains("unbound serialized RAA Section 5 terminal"));

        let helper_top_queries = helper_schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: folded_codeword[query.logical_index],
            })
            .collect::<Vec<_>>();
        params
            .verify_query_proof(
                &prequery,
                &request,
                &helper_schedule,
                &helper_proof,
                &helper_top_queries,
            )
            .unwrap();
        let query_set = helper_proof
            .collect_backend_query_set(
                params.compiler_code.layout(),
                &helper_schedule,
                &fold_challenges_from_prequery(params.spec(), &prequery, &request).unwrap(),
                Some(&prequery.terminal_codeword),
                prequery.section5_relation.as_ref(),
                prequery.section5_relation_residual.as_ref(),
                params.spec().auxiliary_oracle_len,
                &helper_top_queries,
            )
            .unwrap();
        assert_eq!(
            query_set.section5_permutation_helper_queries.len(),
            helper_schedule.section5_permutation_helper_proof_query_count()
        );
        let terminal_residual_query_count = helper_proof
            .section5_relation_residual
            .as_ref()
            .unwrap()
            .terminal_residual_authentication_queries(
                &prequery
                    .section5_relation
                    .as_ref()
                    .unwrap()
                    .verify_prequery::<Blake2s>(params.spec().auxiliary_oracle_len)
                    .unwrap(),
                prequery.section5_relation_residual.as_ref().unwrap(),
                &helper_schedule,
                &helper_proof
                    .authentication
                    .section5_relation_terminal_folded_layers,
            )
            .unwrap()
            .len();
        assert_eq!(
            query_set.section5_relation_residual_queries.len(),
            helper_schedule.section5_relation_residual_proof_query_count()
                + terminal_residual_query_count
        );
        assert!(query_set
            .section5_permutation_helper_queries
            .iter()
            .any(|(index, _)| *index == 0));
        assert!(query_set
            .section5_relation_residual_queries
            .iter()
            .any(|(index, _)| *index == 0));
        helper_proof
            .authentication
            .verify(&prequery, &query_set)
            .unwrap();

        let mut tampered_residual_value = helper_proof.clone();
        tampered_residual_value
            .section5_relation_residual
            .as_mut()
            .unwrap()
            .queries[0]
            .value += B128::ONE;
        let err = verify_section5_relation_residual_openings::<Blake2s>(
            tampered_residual_value.section5_relation_residual.as_ref(),
            &helper_schedule,
        )
        .unwrap_err();
        assert!(format!("{err:?}").contains("residual opening is nonzero"));
        let tampered_query_set = tampered_residual_value
            .collect_backend_query_set(
                params.compiler_code.layout(),
                &helper_schedule,
                &fold_challenges_from_prequery(params.spec(), &prequery, &request).unwrap(),
                Some(&prequery.terminal_codeword),
                prequery.section5_relation.as_ref(),
                prequery.section5_relation_residual.as_ref(),
                params.spec().auxiliary_oracle_len,
                &helper_top_queries,
            )
            .unwrap();
        assert!(tampered_residual_value
            .authentication
            .verify(&prequery, &tampered_query_set)
            .is_err());

        let mut tampered_residual_node = helper_proof.clone();
        tampered_residual_node
            .authentication
            .section5_relation_residual_nodes[0][0] ^= 1;
        assert!(tampered_residual_node
            .authentication
            .verify(&prequery, &query_set)
            .is_err());

        let mut tampered_helper_value = helper_proof.clone();
        tampered_helper_value
            .section5_permutation_helper
            .as_mut()
            .unwrap()
            .queries[0]
            .value += B128::ONE;
        let tampered_query_set = tampered_helper_value
            .collect_backend_query_set(
                params.compiler_code.layout(),
                &helper_schedule,
                &fold_challenges_from_prequery(params.spec(), &prequery, &request).unwrap(),
                Some(&prequery.terminal_codeword),
                prequery.section5_relation.as_ref(),
                prequery.section5_relation_residual.as_ref(),
                params.spec().auxiliary_oracle_len,
                &helper_top_queries,
            )
            .unwrap();
        assert!(tampered_helper_value
            .authentication
            .verify(&prequery, &tampered_query_set)
            .is_err());

        let mut tampered_helper_node = helper_proof;
        tampered_helper_node
            .authentication
            .section5_permutation_helper_nodes[0][0] ^= 1;
        assert!(tampered_helper_node
            .authentication
            .verify(&prequery, &query_set)
            .is_err());
    }

    #[test]
    fn backend_prequery_validates_auxiliary_shape() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 19);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 67, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, state) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();
        assert!(prequery.auxiliary.is_some());
        assert!(prequery.section5_permutation_helper.is_none());

        let mut transcript = CfriTranscript::<Blake2s>::new();
        let schedule = params
            .sample_query_schedule(&mut transcript, &prequery, &request)
            .unwrap();
        let proof = params.open_query_proof(&state, &schedule).unwrap();
        assert!(proof.auxiliary.is_some());

        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &[], &request)
            .is_err());
        let mut wrong_len = auxiliary.clone();
        wrong_len.pop();
        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &wrong_len, &request)
            .is_err());

        let mut wrong_u2 = auxiliary.clone();
        wrong_u2[0] += B128::ONE;
        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &wrong_u2, &request)
            .is_err());

        let len = params.praa().packed().codeword_len();
        let mut wrong_u3 = auxiliary.clone();
        wrong_u3[len] += B128::ONE;
        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &wrong_u3, &request)
            .is_err());

        let mut wrong_u4 = auxiliary.clone();
        wrong_u4[2 * len] += B128::ONE;
        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &wrong_u4, &request)
            .is_err());
    }

    #[test]
    fn backend_prequery_rejects_bad_u2_repetition_layer() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 23);
        let (mut codeword, mut auxiliary, _) =
            folded_codeword_and_auxiliary(&params, 71, &request_point);
        let code = params.praa().packed();
        let len = code.codeword_len();
        let index = (0..len)
            .find(|&candidate| raa_u2_repetition_peer(code, candidate).is_some())
            .expect("test code has a repeated RAA source");

        auxiliary[index] += B128::ONE;
        let mut first_accumulator = B128::ZERO;
        for row_index in 0..len {
            first_accumulator += auxiliary[row_index];
            auxiliary[len + row_index] = first_accumulator;
        }
        for row_index in 0..len {
            let permuted = code.permutation().permutation2[row_index];
            auxiliary[2 * len + row_index] = auxiliary[len + permuted];
        }
        let mut final_accumulator = B128::ZERO;
        for row_index in 0..len {
            final_accumulator += auxiliary[2 * len + row_index];
            codeword[row_index] = final_accumulator;
        }

        let weights = raa_codeword_eval_weights(code, &request_point).unwrap();
        let folded_eval = weights
            .iter()
            .zip(codeword.iter())
            .fold(B128::ZERO, |acc, (&weight, &value)| acc + weight * value);
        let request = open_request_with_eval(&request_point, folded_eval);

        assert!(params
            .prove_prequery::<Blake2s>(&codeword, &auxiliary, &request)
            .is_err());
    }

    #[test]
    fn backend_schedule_binds_spec_and_prequery_public() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let request_point = make_request_point(&params, 23);
        let (folded_codeword, auxiliary, folded_eval) =
            folded_codeword_and_auxiliary(&params, 43, &request_point);
        let request = open_request_with_eval(&request_point, folded_eval);
        let (prequery, _) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &request)
            .unwrap();

        let mut lhs = CfriTranscript::<Blake2s>::new();
        let lhs_schedule = params
            .sample_query_schedule(&mut lhs, &prequery, &request)
            .unwrap();

        let mut rhs = CfriTranscript::<Blake2s>::new();
        let rhs_schedule = params
            .sample_query_schedule(&mut rhs, &prequery, &request)
            .unwrap();
        assert_eq!(lhs_schedule, rhs_schedule);

        let changed_request_point = make_request_point(&params, 29);
        let (_, changed_request_auxiliary, changed_request_eval) =
            folded_codeword_and_auxiliary(&params, 43, &changed_request_point);
        let changed_request = open_request_with_eval(&changed_request_point, changed_request_eval);
        let (changed_request_prequery, _) = params
            .prove_prequery::<Blake2s>(
                &folded_codeword,
                &changed_request_auxiliary,
                &changed_request,
            )
            .unwrap();
        assert!(
            prequery.terminal_codeword != changed_request_prequery.terminal_codeword
                || prequery
                    .folded_parity_layers
                    .iter()
                    .zip(&changed_request_prequery.folded_parity_layers)
                    .any(|(lhs, rhs)| lhs.root.as_slice() != rhs.root.as_slice())
        );
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = params
            .sample_query_schedule(&mut changed, &prequery, &changed_request)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);

        let (changed_folded_codeword, changed_folded_auxiliary, changed_folded_eval) =
            folded_codeword_and_auxiliary(&params, 101, &request_point);
        let changed_folded_request = open_request_with_eval(&request_point, changed_folded_eval);
        let (changed_prequery, _) = params
            .prove_prequery::<Blake2s>(
                &changed_folded_codeword,
                &changed_folded_auxiliary,
                &changed_folded_request,
            )
            .unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = params
            .sample_query_schedule(&mut changed, &changed_prequery, &changed_folded_request)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);

        let changed_auxiliary = message(params.spec().auxiliary_oracle_len, 211);
        assert!(params
            .prove_prequery::<Blake2s>(&folded_codeword, &changed_auxiliary, &request)
            .is_err());

        let mut changed_spec = params.spec().clone();
        changed_spec.q_backend_proof += 1;
        let changed_params = Blaze2BaseFoldBackendParams::new(changed_spec).unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = changed_params
            .sample_query_schedule(&mut changed, &prequery, &request)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);
        assert_eq!(
            changed_schedule.proof_queries().len(),
            params.query_schedule_spec().q_backend_proof + 1
        );
    }

    #[test]
    fn query_schedule_has_fixed_typed_counts() {
        let layout = layout(16, 3);
        let spec = schedule_spec();
        let mut transcript = CfriTranscript::<Blake2s>::new();
        transcript.absorb("prequery-objects-are-bound");
        let schedule = HolographicQuerySchedule::sample(&mut transcript, &layout, spec).unwrap();

        assert_eq!(schedule.input_queries().len(), spec.q_raa_input);
        assert_eq!(schedule.proof_queries().len(), spec.q_backend_proof);
        assert_eq!(schedule.raa_final_queries().len(), spec.q_raa_input >> 1);
        assert_eq!(
            schedule.proof_queries()[0].domain,
            BackendProofQueryDomain::CompilerParity
        );
        for query in schedule.input_queries() {
            assert!(query.logical_index < layout.systematic_len());
            assert_eq!(
                layout.systematic_to_physical(query.logical_index).unwrap(),
                query.physical_index
            );
            assert_eq!(
                layout.physical_to_logical(query.physical_index).unwrap(),
                CodewordAddress {
                    part: CodewordPart::Systematic,
                    local_index: query.logical_index,
                }
            );
        }

        for query in schedule.proof_queries() {
            match query.domain {
                BackendProofQueryDomain::CompilerParity => {
                    let physical = query.physical_index.unwrap();
                    assert_eq!(layout.parity_to_physical(query.index).unwrap(), physical);
                    assert_eq!(
                        layout.physical_to_logical(physical).unwrap(),
                        CodewordAddress {
                            part: CodewordPart::Parity,
                            local_index: query.index,
                        }
                    );
                }
                BackendProofQueryDomain::RelationAuxiliary => {
                    assert!(query.index < spec.auxiliary_oracle_len);
                    assert_eq!(query.physical_index, None);
                }
                BackendProofQueryDomain::Section5RelationResidual => {
                    assert_eq!(
                        spec.raa_relation_strategy,
                        RaaRelationProofStrategy::Section5
                    );
                    assert!(
                        query.index
                            < layout.systematic_len() * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT
                    );
                    assert_eq!(query.physical_index, None);
                }
                BackendProofQueryDomain::Section5PermutationHelper => {
                    assert_eq!(
                        spec.raa_relation_strategy,
                        RaaRelationProofStrategy::Section5
                    );
                    assert!(
                        query.index
                            < layout.systematic_len() * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT
                    );
                    assert_eq!(query.physical_index, None);
                }
            }
        }
        for query in schedule.raa_final_queries() {
            assert_eq!(
                schedule.input_queries()[query.current_input_query].logical_index,
                query.index
            );
            assert_eq!(
                schedule.input_queries()[query.previous_input_query].logical_index + 1,
                query.index
            );
            let auxiliary_row_len = spec.auxiliary_oracle_len / RAA_AUX_ROW_COUNT;
            assert!(
                query.u4_auxiliary_index
                    >= raa_auxiliary_index(RAA_AUX_U4_ROW, 0, auxiliary_row_len)
            );
            assert_eq!(
                query.u4_auxiliary_index,
                raa_auxiliary_index(RAA_AUX_U4_ROW, query.index, auxiliary_row_len)
            );
        }
    }

    #[test]
    fn proof_query_classification_splits_parity_and_auxiliary_domains() {
        let layout = layout(16, 3);
        let parity =
            proof_query_from_sampled_index(&layout, 11, 0, 0, layout.parity_len() - 1).unwrap();
        assert_eq!(parity.domain, BackendProofQueryDomain::CompilerParity);
        assert_eq!(parity.index, layout.parity_len() - 1);
        assert_eq!(
            parity.physical_index,
            Some(layout.parity_to_physical(layout.parity_len() - 1).unwrap())
        );

        let auxiliary =
            proof_query_from_sampled_index(&layout, 11, 0, 0, layout.parity_len()).unwrap();
        assert_eq!(auxiliary.domain, BackendProofQueryDomain::RelationAuxiliary);
        assert_eq!(auxiliary.index, 0);
        assert_eq!(auxiliary.physical_index, None);

        let helper = proof_query_from_sampled_index(
            &layout,
            11,
            layout.systematic_len() * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT,
            layout.systematic_len() * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT,
            layout.parity_len() + 11,
        )
        .unwrap();
        assert_eq!(
            helper.domain,
            BackendProofQueryDomain::Section5RelationResidual
        );
        assert_eq!(helper.index, 0);
        assert_eq!(helper.physical_index, None);

        let helper = proof_query_from_sampled_index(
            &layout,
            11,
            layout.systematic_len() * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT,
            layout.systematic_len() * RAA_SECTION5_PERMUTATION_HELPER_ROW_COUNT,
            layout.parity_len()
                + 11
                + layout.systematic_len() * RAA_SECTION5_RELATION_RESIDUAL_ROW_COUNT,
        )
        .unwrap();
        assert_eq!(
            helper.domain,
            BackendProofQueryDomain::Section5PermutationHelper
        );
        assert_eq!(helper.index, 0);
        assert_eq!(helper.physical_index, None);
    }

    #[test]
    fn query_schedule_is_transcript_ordered_and_deterministic() {
        let layout = layout(16, 3);
        let spec = schedule_spec();
        let mut lhs = CfriTranscript::<Blake2s>::new();
        lhs.absorb("same-prequery-objects");
        let schedule_lhs = HolographicQuerySchedule::sample(&mut lhs, &layout, spec).unwrap();

        let mut rhs = CfriTranscript::<Blake2s>::new();
        rhs.absorb("same-prequery-objects");
        let schedule_rhs = HolographicQuerySchedule::sample(&mut rhs, &layout, spec).unwrap();
        assert_eq!(schedule_lhs, schedule_rhs);

        let mut different = CfriTranscript::<Blake2s>::new();
        different.absorb("different-prequery-objects");
        let schedule_different =
            HolographicQuerySchedule::sample(&mut different, &layout, spec).unwrap();
        assert_ne!(schedule_lhs, schedule_different);
    }

    #[test]
    fn top_queries_must_match_systematic_schedule() {
        let layout = layout(16, 3);
        let mut transcript = CfriTranscript::<Blake2s>::new();
        transcript.absorb("top-query-check");
        let schedule =
            HolographicQuerySchedule::sample(&mut transcript, &layout, schedule_spec()).unwrap();
        let mut top_queries = schedule
            .input_queries()
            .iter()
            .map(|query| TopQuery {
                index: query.logical_index,
                value: B128::from(query.logical_index as u64),
            })
            .collect::<Vec<_>>();

        schedule.validate_top_queries(&top_queries).unwrap();
        top_queries.swap(0, 1);
        assert!(schedule.validate_top_queries(&top_queries).is_err());
    }

    #[test]
    fn top_queries_are_derived_from_matching_blaze_columns() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let schedule = explicit_schedule(params.compiler_code().layout());
        let folding_challenges = vec![B128::from(3), B128::from(5), B128::from(7)];
        let columns = schedule
            .input_queries()
            .iter()
            .map(|query| Blaze2InterleavedColumnQuery::<Blake2s> {
                index: query.logical_index,
                values: vec![
                    B128::from(query.logical_index as u64 + 1),
                    B128::from(query.logical_index as u64 + 2),
                    B128::from(query.logical_index as u64 + 3),
                ],
                path: Vec::new(),
            })
            .collect::<Vec<_>>();

        let top_queries = params
            .top_queries_from_interleaved_columns(&schedule, &columns, &folding_challenges)
            .unwrap();

        assert_eq!(top_queries.len(), schedule.input_queries().len());
        for ((top_query, schedule_query), column) in top_queries
            .iter()
            .zip(schedule.input_queries())
            .zip(&columns)
        {
            assert_eq!(top_query.index, schedule_query.logical_index);
            assert_eq!(
                top_query.value,
                fold_interleaved_column(column, &folding_challenges).unwrap()
            );
        }
        schedule.validate_top_queries(&top_queries).unwrap();

        let mut wrong_index = columns.clone();
        wrong_index[0].index += 1;
        assert!(params
            .top_queries_from_interleaved_columns(&schedule, &wrong_index, &folding_challenges)
            .is_err());

        let mut wrong_width = columns;
        wrong_width[0].values.pop();
        assert!(params
            .top_queries_from_interleaved_columns(&schedule, &wrong_width, &folding_challenges)
            .is_err());
    }
}
