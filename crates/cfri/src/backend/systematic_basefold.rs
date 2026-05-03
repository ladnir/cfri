use crate::backend::{
    arithmetic::Field,
    binary_extension_fields::B128,
    blaze2::{
        absorb_blaze2_code_spec, fold_interleaved_column, raa_codeword_eval_weights, Blaze2Code,
        Blaze2CodeSpec, Blaze2InterleavedColumnQuery,
    },
    code::PackedRaaCode,
    hash::{Blake2s, Hash, Output},
    Error,
};
use crate::transcript::Transcript as CfriTranscript;

const RAA_AUX_U2_ROW: usize = 0;
const RAA_AUX_U3_ROW: usize = 1;
const RAA_AUX_U4_ROW: usize = 2;
const RAA_AUX_EVAL_ROW: usize = 3;
const RAA_AUX_RELATION_ROW_COUNT: usize = 3;
const RAA_AUX_EVAL_BINDING_ROW_COUNT: usize = 1;
const RAA_AUX_ROW_COUNT: usize = 4;

pub const BLAZE2_BASEFOLD_RELATION_AUXILIARY_ROW_COUNT: usize = RAA_AUX_RELATION_ROW_COUNT;
pub const BLAZE2_BASEFOLD_EVAL_BINDING_ROW_COUNT: usize = RAA_AUX_EVAL_BINDING_ROW_COUNT;
pub const BLAZE2_BASEFOLD_AUXILIARY_ROW_COUNT: usize = RAA_AUX_ROW_COUNT;

pub fn required_blaze2_basefold_auxiliary_oracle_len(spec: &Blaze2CodeSpec) -> usize {
    required_blaze2_basefold_relation_auxiliary_len(spec)
        + required_blaze2_basefold_eval_binding_len(spec)
}

pub fn required_blaze2_basefold_relation_auxiliary_len(spec: &Blaze2CodeSpec) -> usize {
    spec.praa_codeword_len * BLAZE2_BASEFOLD_RELATION_AUXILIARY_ROW_COUNT
}

pub fn required_blaze2_basefold_eval_binding_len(spec: &Blaze2CodeSpec) -> usize {
    spec.praa_codeword_len * BLAZE2_BASEFOLD_EVAL_BINDING_ROW_COUNT
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
    pub eval_current_auxiliary_index: usize,
    pub eval_previous_auxiliary_index: usize,
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
pub struct CompilerParityQuery<H: Hash> {
    pub logical_index: usize,
    pub value: B128,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityQueryProof<H: Hash> {
    pub queries: Vec<CompilerParityQuery<H>>,
}

#[derive(Clone, Debug)]
pub struct AuxiliaryOracleCommitment<H: Hash> {
    values: Vec<B128>,
    merkle_tree: Vec<Vec<Output<H>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOraclePublicCommitment<H: Hash> {
    pub root: Output<H>,
    pub len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOracleQuery<H: Hash> {
    pub logical_index: usize,
    pub value: B128,
    pub path: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuxiliaryOracleQueryProof<H: Hash> {
    pub relation_queries: Vec<AuxiliaryOracleQuery<H>>,
    pub final_accumulator_queries: Vec<RaaFinalAccumulatorQueryProof<H>>,
    pub eval_terminal_query: Option<AuxiliaryOracleQuery<H>>,
    pub local_relation_queries: Vec<AuxiliaryOracleQuery<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaFinalAccumulatorQueryProof<H: Hash> {
    pub u4: AuxiliaryOracleQuery<H>,
    pub eval_current: AuxiliaryOracleQuery<H>,
    pub eval_previous: AuxiliaryOracleQuery<H>,
}

#[derive(Clone, Copy, Debug)]
pub struct Blaze2BaseFoldOpenRequest<'a> {
    pub col_point: &'a [B128],
    pub folded_eval: B128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldPrequeryPublic<H: Hash> {
    pub compiler_parity: CompilerParityPublicCommitment<H>,
    pub folded_parity_layers: Vec<CompilerParityPublicCommitment<H>>,
    pub terminal_codeword: Vec<B128>,
    pub auxiliary: Option<AuxiliaryOraclePublicCommitment<H>>,
}

#[derive(Clone, Debug)]
pub struct Blaze2BaseFoldProverState<H: Hash> {
    compiler_parity: CompilerParityCommitment<H>,
    folded_parity_layers: Vec<CompilerParityCommitment<H>>,
    physical_layers: Vec<Vec<B128>>,
    fold_challenges: Vec<B128>,
    auxiliary: Option<AuxiliaryOracleCommitment<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldQueryProof<H: Hash> {
    pub compiler_parity: CompilerParityQueryProof<H>,
    pub compiler_parity_folds: CompilerParityFoldQueryProof<H>,
    pub auxiliary: Option<AuxiliaryOracleQueryProof<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldQueryProof<H: Hash> {
    pub paths: Vec<CompilerParityFoldPath<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldPath<H: Hash> {
    pub top_logical_index: usize,
    pub top_physical_index: usize,
    pub steps: Vec<CompilerParityFoldStep<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerParityFoldStep<H: Hash> {
    pub round: usize,
    pub output_physical_index: usize,
    pub left_physical_index: usize,
    pub left_logical_index: usize,
    pub left_value: B128,
    pub left_path: Vec<Output<H>>,
    pub right_physical_index: usize,
    pub right_logical_index: usize,
    pub right_value: B128,
    pub right_path: Vec<Output<H>>,
    pub folded_physical_index: usize,
    pub folded_logical_index: usize,
    pub folded_value: B128,
    pub folded_path: Vec<Output<H>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HolographicQuerySchedule {
    input_queries: Vec<SystematicInputQuery>,
    proof_queries: Vec<BackendProofQuery>,
    raa_final_queries: Vec<RaaFinalAccumulatorQuery>,
    raa_auxiliary_queries: Vec<RaaAuxiliaryLocalQuery>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HolographicQueryScheduleSpec {
    pub q_raa_input: usize,
    pub q_backend_proof: usize,
    pub auxiliary_oracle_len: usize,
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
        let compiler_parity = self.compiler_code.commit_parity(folded_codeword)?;
        let auxiliary = if self.spec.auxiliary_oracle_len == 0 {
            None
        } else {
            Some(AuxiliaryOracleCommitment::commit_values(
                auxiliary_oracle.to_vec(),
            )?)
        };
        let (folded_parity_layers, folded_parity_public, physical_layers, fold_challenges) = self
            .prove_folded_parity_layers::<H>(
            folded_codeword,
            &compiler_parity.public(),
            auxiliary.as_ref().map(AuxiliaryOracleCommitment::public),
            request,
        )?;
        let public = Blaze2BaseFoldPrequeryPublic {
            compiler_parity: compiler_parity.public(),
            folded_parity_layers: folded_parity_public,
            terminal_codeword: physical_layers
                .last()
                .expect("folded parity prover returns at least the top physical layer")
                .clone(),
            auxiliary: auxiliary.as_ref().map(AuxiliaryOracleCommitment::public),
        };
        Ok((
            public,
            Blaze2BaseFoldProverState {
                compiler_parity,
                folded_parity_layers,
                physical_layers,
                fold_challenges,
                auxiliary,
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
        absorb_blaze2_basefold_backend_spec(transcript, self.spec());
        absorb_blaze2_basefold_open_request(transcript, request);
        absorb_blaze2_basefold_prequery_public(transcript, prequery);
        let mut schedule = HolographicQuerySchedule::sample(
            transcript,
            self.compiler_code.layout(),
            self.query_schedule_spec(),
        )?;
        if self.spec.auxiliary_oracle_len != 0 {
            schedule.attach_raa_auxiliary_local_queries(self.praa.packed())?;
        }
        Ok(schedule)
    }

    pub fn open_query_proof<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<Blaze2BaseFoldQueryProof<H>, Error> {
        let auxiliary = match &state.auxiliary {
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
        Ok(Blaze2BaseFoldQueryProof {
            compiler_parity: state.compiler_parity.prove_schedule(schedule)?,
            compiler_parity_folds: self.open_compiler_parity_fold_paths(state, schedule)?,
            auxiliary,
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
        schedule.validate_top_queries(top_queries)?;
        proof.compiler_parity.verify(
            &prequery.compiler_parity,
            self.compiler_code.layout(),
            schedule,
        )?;
        proof.compiler_parity_folds.verify(
            prequery,
            self.compiler_code(),
            &fold_challenges_from_prequery(self.spec(), prequery, request)?,
            schedule,
        )?;
        verify_auxiliary_query_proof(
            prequery.auxiliary.as_ref(),
            proof.auxiliary.as_ref(),
            self.spec.auxiliary_oracle_len,
            schedule,
        )?;
        verify_raa_auxiliary_local_queries(
            proof.auxiliary.as_ref(),
            self.spec.auxiliary_oracle_len,
            schedule,
        )?;
        verify_raa_final_accumulator_queries(
            proof.auxiliary.as_ref(),
            self.spec.auxiliary_oracle_len,
            self.praa.packed(),
            request,
            schedule,
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
        auxiliary: Option<AuxiliaryOraclePublicCommitment<H>>,
        request: &Blaze2BaseFoldOpenRequest<'_>,
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
            auxiliary.as_ref(),
            request,
        );

        let mut physical_layers = Vec::with_capacity(layout.num_rounds() + 1);
        physical_layers.push(current.clone());
        let mut folded_parity_layers = Vec::with_capacity(layout.num_rounds());
        let mut folded_parity_public = Vec::with_capacity(layout.num_rounds());
        let mut fold_challenges = Vec::with_capacity(layout.num_rounds());

        for round in 0..layout.num_rounds() {
            transcript.absorb("systematic-basefold-fold-challenge-v1");
            absorb_usize(&mut transcript, round);
            let alpha: B128 = transcript.squeeze();
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
            paths.push(self.open_compiler_parity_fold_path(
                state,
                query.index,
                top_physical_index,
            )?);
        }
        Ok(CompilerParityFoldQueryProof { paths })
    }

    fn open_compiler_parity_fold_path<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        top_logical_index: usize,
        top_physical_index: usize,
    ) -> Result<CompilerParityFoldPath<H>, Error> {
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
            let left = parity_query_for_physical_index(
                layout,
                round,
                &state.compiler_parity,
                &state.folded_parity_layers,
                pair.left,
            )?;
            let right = parity_query_for_physical_index(
                layout,
                round,
                &state.compiler_parity,
                &state.folded_parity_layers,
                pair.right,
            )?;
            let folded = parity_query_for_physical_index(
                layout,
                round + 1,
                &state.compiler_parity,
                &state.folded_parity_layers,
                output_physical_index,
            )?;
            debug_assert_eq!(left.value, state.physical_layers[round][pair.left]);
            debug_assert_eq!(right.value, state.physical_layers[round][pair.right]);
            debug_assert_eq!(
                folded.value,
                state.physical_layers[round + 1][output_physical_index]
            );
            steps.push(CompilerParityFoldStep {
                round,
                output_physical_index,
                left_physical_index: pair.left,
                left_logical_index: left.logical_index,
                left_value: left.value,
                left_path: left.path,
                right_physical_index: pair.right,
                right_logical_index: right.logical_index,
                right_value: right.value,
                right_path: right.path,
                folded_physical_index: output_physical_index,
                folded_logical_index: folded.logical_index,
                folded_value: folded.value,
                folded_path: folded.path,
            });
            current_physical_index = output_physical_index;
        }

        Ok(CompilerParityFoldPath {
            top_logical_index,
            top_physical_index,
            steps,
        })
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

    pub fn query(&self, logical_index: usize) -> Result<CompilerParityQuery<H>, Error> {
        validate_parity_query_index(logical_index, self.len())?;
        Ok(CompilerParityQuery {
            logical_index,
            value: self.values[logical_index],
            path: merkle_padded_sibling_path::<H>(&self.merkle_tree, logical_index),
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
        Ok(CompilerParityQueryProof { queries })
    }
}

impl<H: Hash> CompilerParityQuery<H> {
    pub fn authenticate(&self, public: &CompilerParityPublicCommitment<H>) -> Result<(), Error> {
        validate_parity_query_index(self.logical_index, public.len)?;
        let padded_len = public.len.next_power_of_two();
        let expected_path_len = log2_strict(padded_len);
        if self.path.len() != expected_path_len {
            return Err(Error::InvalidPcsOpen(
                "compiler parity query path has incompatible length".to_string(),
            ));
        }

        let mut hash = hash_b128_leaf::<H>(&self.value);
        let mut query_index = self.logical_index;
        for sibling in &self.path {
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

        if hash != public.root {
            return Err(Error::InvalidPcsOpen(
                "compiler parity query does not authenticate".to_string(),
            ));
        }
        Ok(())
    }
}

impl<H: Hash> CompilerParityQueryProof<H> {
    pub fn verify(
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
            query.authenticate(public)?;
        }
        Ok(())
    }
}

impl<H: Hash> CompilerParityFoldQueryProof<H> {
    pub fn verify(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        code: &SystematicAugmentedRfcCode,
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

        let mut supplied = self.paths.iter();
        for expected in schedule.proof_queries() {
            if expected.domain != BackendProofQueryDomain::CompilerParity {
                continue;
            }
            let path = supplied.next().expect("path count checked above");
            let expected_physical = expected.physical_index.ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "compiler parity query is missing its physical index".to_string(),
                )
            })?;
            if path.top_logical_index != expected.index
                || path.top_physical_index != expected_physical
            {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path does not match schedule".to_string(),
                ));
            }
            path.verify(prequery, layout, fold_challenges)?;
        }
        Ok(())
    }
}

impl<H: Hash> CompilerParityFoldPath<H> {
    fn verify(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        layout: &SystematicAugmentedRfcLayout,
        fold_challenges: &[B128],
    ) -> Result<(), Error> {
        if self.steps.len() != layout.num_rounds() {
            return Err(Error::InvalidPcsOpen(
                "compiler parity fold path has wrong round count".to_string(),
            ));
        }
        let mut current_physical_index = self.top_physical_index;
        for (round, step) in self.steps.iter().enumerate() {
            if step.round != round {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path round is out of order".to_string(),
                ));
            }
            let current_len = layout.codeword_len() >> round;
            let half_len = current_len >> 1;
            let output_physical_index = current_physical_index & (half_len - 1);
            let pair = layout.fold_pair(round, output_physical_index)?;
            if step.output_physical_index != output_physical_index
                || step.left_physical_index != pair.left
                || step.right_physical_index != pair.right
                || step.folded_physical_index != output_physical_index
            {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path indices do not follow the fold pair".to_string(),
                ));
            }

            verify_parity_layer_opening(
                prequery,
                layout,
                round,
                pair.left,
                step.left_logical_index,
                step.left_value,
                &step.left_path,
            )?;
            verify_parity_layer_opening(
                prequery,
                layout,
                round,
                pair.right,
                step.right_logical_index,
                step.right_value,
                &step.right_path,
            )?;
            verify_parity_layer_opening(
                prequery,
                layout,
                round + 1,
                output_physical_index,
                step.folded_logical_index,
                step.folded_value,
                &step.folded_path,
            )?;

            let folded =
                fold_rfc_parity_pair(step.left_value, step.right_value, fold_challenges[round]);
            if folded != step.folded_value {
                return Err(Error::InvalidPcsOpen(
                    "compiler parity fold path value does not satisfy fold equation".to_string(),
                ));
            }
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

    pub fn query(&self, logical_index: usize) -> Result<AuxiliaryOracleQuery<H>, Error> {
        validate_auxiliary_query_index(logical_index, self.len())?;
        Ok(AuxiliaryOracleQuery {
            logical_index,
            value: self.values[logical_index],
            path: merkle_padded_sibling_path::<H>(&self.merkle_tree, logical_index),
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
        let mut final_accumulator_queries = Vec::with_capacity(schedule.raa_final_queries().len());
        for query in schedule.raa_final_queries() {
            final_accumulator_queries.push(RaaFinalAccumulatorQueryProof {
                u4: self.query(query.u4_auxiliary_index)?,
                eval_current: self.query(query.eval_current_auxiliary_index)?,
                eval_previous: self.query(query.eval_previous_auxiliary_index)?,
            });
        }
        let eval_terminal_query = if !schedule.raa_final_queries().is_empty() {
            let len = self.len() / RAA_AUX_ROW_COUNT;
            Some(self.query(raa_auxiliary_index(RAA_AUX_EVAL_ROW, len - 1, len))?)
        } else {
            None
        };
        let mut local_relation_queries = Vec::new();
        for query in schedule.raa_auxiliary_queries() {
            for index in 0..query.extra_count() {
                let auxiliary_index = query
                    .extra_index(index)
                    .expect("extra_count bounds extra_index");
                local_relation_queries.push(self.query(auxiliary_index)?);
            }
        }
        Ok(AuxiliaryOracleQueryProof {
            relation_queries,
            final_accumulator_queries,
            eval_terminal_query,
            local_relation_queries,
        })
    }
}

impl<H: Hash> AuxiliaryOracleQuery<H> {
    pub fn authenticate(&self, public: &AuxiliaryOraclePublicCommitment<H>) -> Result<(), Error> {
        validate_auxiliary_query_index(self.logical_index, public.len)?;
        let padded_len = public.len.next_power_of_two();
        let expected_path_len = log2_strict(padded_len);
        if self.path.len() != expected_path_len {
            return Err(Error::InvalidPcsOpen(
                "auxiliary oracle query path has incompatible length".to_string(),
            ));
        }

        let mut hash = hash_b128_leaf::<H>(&self.value);
        let mut query_index = self.logical_index;
        for sibling in &self.path {
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

        if hash != public.root {
            return Err(Error::InvalidPcsOpen(
                "auxiliary oracle query does not authenticate".to_string(),
            ));
        }
        Ok(())
    }
}

impl<H: Hash> AuxiliaryOracleQueryProof<H> {
    pub fn query_count(&self) -> usize {
        self.relation_queries.len()
            + 3 * self.final_accumulator_queries.len()
            + usize::from(self.eval_terminal_query.is_some())
            + self.local_relation_queries.len()
    }

    pub fn all_queries(&self) -> impl Iterator<Item = &AuxiliaryOracleQuery<H>> {
        self.relation_queries
            .iter()
            .chain(
                self.final_accumulator_queries
                    .iter()
                    .flat_map(RaaFinalAccumulatorQueryProof::queries),
            )
            .chain(self.eval_terminal_query.iter())
            .chain(self.local_relation_queries.iter())
    }

    pub fn verify(
        &self,
        public: &AuxiliaryOraclePublicCommitment<H>,
        auxiliary_oracle_len: usize,
        schedule: &HolographicQuerySchedule,
    ) -> Result<(), Error> {
        if public.len != auxiliary_oracle_len {
            return Err(Error::InvalidPcsOpen(
                "auxiliary oracle commitment length does not match backend spec".to_string(),
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
            query.authenticate(public)?;
        }
        if relation_queries.next().is_some() {
            return Err(Error::InvalidPcsOpen(
                "too many relation auxiliary query openings".to_string(),
            ));
        }

        if self.final_accumulator_queries.len() != schedule.raa_final_queries().len() {
            return Err(Error::InvalidPcsOpen(
                "RAA final accumulator query proof count does not match schedule".to_string(),
            ));
        }
        for (expected, supplied) in schedule
            .raa_final_queries()
            .iter()
            .zip(self.final_accumulator_queries.iter())
        {
            let u4 = &supplied.u4;
            if u4.logical_index != expected.u4_auxiliary_index {
                return Err(Error::InvalidPcsOpen(
                    "RAA final accumulator auxiliary query index does not match schedule"
                        .to_string(),
                ));
            }
            u4.authenticate(public)?;
            let eval_current = &supplied.eval_current;
            if eval_current.logical_index != expected.eval_current_auxiliary_index {
                return Err(Error::InvalidPcsOpen(
                    "RAA evaluation accumulator current query index does not match schedule"
                        .to_string(),
                ));
            }
            eval_current.authenticate(public)?;
            let eval_previous = &supplied.eval_previous;
            if eval_previous.logical_index != expected.eval_previous_auxiliary_index {
                return Err(Error::InvalidPcsOpen(
                    "RAA evaluation accumulator previous query index does not match schedule"
                        .to_string(),
                ));
            }
            eval_previous.authenticate(public)?;
        }
        if !schedule.raa_final_queries().is_empty() {
            let len = auxiliary_oracle_len / RAA_AUX_ROW_COUNT;
            let terminal = self.eval_terminal_query.as_ref().ok_or_else(|| {
                Error::InvalidPcsOpen(
                    "RAA evaluation accumulator terminal query is missing".to_string(),
                )
            })?;
            if terminal.logical_index != raa_auxiliary_index(RAA_AUX_EVAL_ROW, len - 1, len) {
                return Err(Error::InvalidPcsOpen(
                    "RAA evaluation accumulator terminal query index does not match schedule"
                        .to_string(),
                ));
            }
            terminal.authenticate(public)?;
        } else if self.eval_terminal_query.is_some() {
            return Err(Error::InvalidPcsOpen(
                "RAA evaluation accumulator terminal query was supplied without final checks"
                    .to_string(),
            ));
        }
        let mut local_relation_queries = self.local_relation_queries.iter();
        for expected in schedule.raa_auxiliary_queries() {
            for index in 0..expected.extra_count() {
                let expected_index = expected
                    .extra_index(index)
                    .expect("extra_count bounds extra_index");
                let query = local_relation_queries.next().ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA auxiliary local relation opening is missing".to_string(),
                    )
                })?;
                if query.logical_index != expected_index {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local relation opening index does not match schedule"
                            .to_string(),
                    ));
                }
                query.authenticate(public)?;
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

impl<H: Hash> RaaFinalAccumulatorQueryProof<H> {
    pub fn queries(&self) -> impl Iterator<Item = &AuxiliaryOracleQuery<H>> {
        [&self.u4, &self.eval_current, &self.eval_previous].into_iter()
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

        let mut input_queries = Vec::with_capacity(spec.q_raa_input);
        let mut raa_final_queries = Vec::new();
        if spec.auxiliary_oracle_len == 0 {
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
            let eval_offset = raa_auxiliary_index(RAA_AUX_EVAL_ROW, 0, len);
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
                    eval_current_auxiliary_index: eval_offset + index,
                    eval_previous_auxiliary_index: eval_offset + index - 1,
                });
            }
        }

        let auxiliary_proof_len = raa_relation_auxiliary_len(spec.auxiliary_oracle_len);
        let proof_domain_len = layout.parity_len() + auxiliary_proof_len;
        let mut proof_queries = Vec::with_capacity(spec.q_backend_proof);
        for _ in 0..spec.q_backend_proof {
            let sampled_index = squeeze_bounded_index(transcript, proof_domain_len)?;
            proof_queries.push(proof_query_from_sampled_index(
                layout,
                spec.auxiliary_oracle_len,
                sampled_index,
            )?);
        }

        Ok(Self {
            input_queries,
            proof_queries,
            raa_final_queries,
            raa_auxiliary_queries: Vec::new(),
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

    pub fn raa_auxiliary_queries(&self) -> &[RaaAuxiliaryLocalQuery] {
        &self.raa_auxiliary_queries
    }

    pub fn relation_auxiliary_proof_query_count(&self) -> usize {
        self.proof_queries
            .iter()
            .filter(|query| query.domain == BackendProofQueryDomain::RelationAuxiliary)
            .count()
    }

    pub fn expected_auxiliary_query_proof_count(&self) -> usize {
        self.relation_auxiliary_proof_query_count()
            + 3 * self.raa_final_queries.len()
            + usize::from(!self.raa_final_queries.is_empty())
            + self
                .raa_auxiliary_queries
                .iter()
                .map(RaaAuxiliaryLocalQuery::extra_count)
                .sum::<usize>()
    }

    pub fn attach_raa_auxiliary_local_queries(
        &mut self,
        code: &PackedRaaCode,
    ) -> Result<(), Error> {
        self.raa_auxiliary_queries = build_raa_auxiliary_local_queries(code, &self.proof_queries)?;
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
    let expected_eval_binding_len = required_blaze2_basefold_eval_binding_len(&spec.praa);
    let expected_auxiliary_len = expected_relation_auxiliary_len + expected_eval_binding_len;
    if spec.auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsParam(format!(
            "Blaze2 BaseFold backend needs {expected_relation_auxiliary_len} relation auxiliary entries plus {expected_eval_binding_len} eval-binding entries"
        )));
    }
    if spec.auxiliary_oracle_len != expected_auxiliary_len {
        return Err(Error::InvalidPcsParam(format!(
            "auxiliary oracle length must be {expected_auxiliary_len}: {expected_relation_auxiliary_len} relation auxiliary entries plus {expected_eval_binding_len} eval-binding entries"
        )));
    }
    if spec.q_raa_input & 1 != 0 {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled Blaze2 BaseFold backend needs an even RAA input query count"
                .to_string(),
        ));
    }
    Ok(())
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

fn validate_query_schedule_spec(
    layout: &SystematicAugmentedRfcLayout,
    spec: &HolographicQueryScheduleSpec,
) -> Result<(), Error> {
    if spec.q_raa_input == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic BaseFold schedule needs at least one input query".to_string(),
        ));
    }
    if spec.auxiliary_oracle_len != 0 && spec.q_raa_input & 1 != 0 {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled systematic BaseFold schedule needs an even RAA input query count"
                .to_string(),
        ));
    }
    if spec.auxiliary_oracle_len != 0
        && spec.auxiliary_oracle_len != RAA_AUX_ROW_COUNT * layout.systematic_len()
    {
        return Err(Error::InvalidPcsParam(
            "auxiliary-enabled systematic BaseFold schedule expects a flattened RAA auxiliary/evaluation trace"
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

fn parity_query_for_physical_index<H: Hash>(
    layout: &SystematicAugmentedRfcLayout,
    round: usize,
    top: &CompilerParityCommitment<H>,
    folded: &[CompilerParityCommitment<H>],
    physical_index: usize,
) -> Result<CompilerParityQuery<H>, Error> {
    let address = layout.physical_to_logical_at_round(round, physical_index)?;
    if address.part != CodewordPart::Parity {
        return Err(Error::InvalidPcsOpen(
            "compiler parity fold path tried to authenticate a systematic position as parity"
                .to_string(),
        ));
    }
    parity_commitment_at_round(top, folded, round)?.query(address.local_index)
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

fn verify_parity_layer_opening<H: Hash>(
    prequery: &Blaze2BaseFoldPrequeryPublic<H>,
    layout: &SystematicAugmentedRfcLayout,
    round: usize,
    physical_index: usize,
    logical_index: usize,
    value: B128,
    path: &[Output<H>],
) -> Result<(), Error> {
    let address = layout.physical_to_logical_at_round(round, physical_index)?;
    if address.part != CodewordPart::Parity || address.local_index != logical_index {
        return Err(Error::InvalidPcsOpen(
            "parity layer opening does not match the fold-path physical address".to_string(),
        ));
    }
    CompilerParityQuery {
        logical_index,
        value,
        path: path.to_vec(),
    }
    .authenticate(parity_public_at_round(prequery, round)?)
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
    sampled_index: usize,
) -> Result<BackendProofQuery, Error> {
    let proof_domain_len = layout.parity_len() + raa_relation_auxiliary_len(auxiliary_oracle_len);
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
        Ok(BackendProofQuery {
            domain: BackendProofQueryDomain::RelationAuxiliary,
            index: sampled_index - layout.parity_len(),
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

fn raa_relation_auxiliary_len(auxiliary_oracle_len: usize) -> usize {
    if auxiliary_oracle_len == 0 {
        0
    } else {
        auxiliary_oracle_len * RAA_AUX_RELATION_ROW_COUNT / RAA_AUX_ROW_COUNT
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
    proof.verify(public, auxiliary_oracle_len, schedule)
}

fn verify_raa_final_accumulator_queries<H: Hash>(
    proof: Option<&AuxiliaryOracleQueryProof<H>>,
    auxiliary_oracle_len: usize,
    code: &PackedRaaCode,
    request: &Blaze2BaseFoldOpenRequest<'_>,
    schedule: &HolographicQuerySchedule,
    top_queries: &[TopQuery<B128>],
) -> Result<(), Error> {
    if schedule.raa_final_queries().is_empty() {
        return Ok(());
    }
    if auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA final accumulator checks require an auxiliary oracle".to_string(),
        ));
    }
    let proof = proof.ok_or_else(|| {
        Error::InvalidPcsOpen(
            "RAA final accumulator checks require auxiliary query openings".to_string(),
        )
    })?;

    let eval_weights = raa_codeword_eval_weights(code, request.col_point)?;
    for (relative_index, expected) in schedule.raa_final_queries().iter().enumerate() {
        let current = top_queries
            .get(expected.current_input_query)
            .ok_or_else(|| Error::InvalidPcsOpen("missing current RAA input query".to_string()))?;
        let previous = top_queries
            .get(expected.previous_input_query)
            .ok_or_else(|| Error::InvalidPcsOpen("missing previous RAA input query".to_string()))?;
        if current.index != expected.index || previous.index + 1 != expected.index {
            return Err(Error::InvalidPcsOpen(
                "RAA final accumulator input queries do not match scheduled neighboring columns"
                    .to_string(),
            ));
        }
        let final_accumulator = proof
            .final_accumulator_queries
            .get(relative_index)
            .ok_or_else(|| {
                Error::InvalidPcsOpen("RAA final accumulator opening triple is missing".to_string())
            })?;
        let u4 = &final_accumulator.u4;
        if u4.logical_index != expected.u4_auxiliary_index {
            return Err(Error::InvalidPcsOpen(
                "RAA final accumulator auxiliary opening index does not match schedule".to_string(),
            ));
        }
        if current.value != previous.value + u4.value {
            return Err(Error::InvalidPcsOpen(
                "RAA final accumulator relation failed".to_string(),
            ));
        }
        let eval_current = &final_accumulator.eval_current;
        if eval_current.logical_index != expected.eval_current_auxiliary_index {
            return Err(Error::InvalidPcsOpen(
                "RAA evaluation accumulator current opening index does not match schedule"
                    .to_string(),
            ));
        }
        let eval_previous = &final_accumulator.eval_previous;
        if eval_previous.logical_index != expected.eval_previous_auxiliary_index {
            return Err(Error::InvalidPcsOpen(
                "RAA evaluation accumulator previous opening index does not match schedule"
                    .to_string(),
            ));
        }
        if eval_current.value != eval_previous.value + eval_weights[expected.index] * current.value
        {
            return Err(Error::InvalidPcsOpen(
                "RAA evaluation accumulator relation failed".to_string(),
            ));
        }
    }
    verify_raa_eval_accumulator_terminal(proof, auxiliary_oracle_len, code, request)?;
    Ok(())
}

fn verify_raa_eval_accumulator_terminal<H: Hash>(
    proof: &AuxiliaryOracleQueryProof<H>,
    auxiliary_oracle_len: usize,
    code: &PackedRaaCode,
    request: &Blaze2BaseFoldOpenRequest<'_>,
) -> Result<(), Error> {
    let len = code.codeword_len();
    if auxiliary_oracle_len != RAA_AUX_ROW_COUNT * len {
        return Err(Error::InvalidPcsOpen(
            "RAA evaluation accumulator terminal check has incompatible auxiliary length"
                .to_string(),
        ));
    }
    let terminal_index = raa_auxiliary_index(RAA_AUX_EVAL_ROW, len - 1, len);
    let terminal = proof.eval_terminal_query.as_ref().ok_or_else(|| {
        Error::InvalidPcsOpen("RAA evaluation accumulator terminal opening is missing".to_string())
    })?;
    if terminal.logical_index != terminal_index {
        return Err(Error::InvalidPcsOpen(
            "RAA evaluation accumulator terminal opening index does not match schedule".to_string(),
        ));
    }
    if terminal.value != request.folded_eval {
        return Err(Error::InvalidPcsOpen(
            "RAA evaluation accumulator terminal value does not match folded eval".to_string(),
        ));
    }
    Ok(())
}

fn verify_raa_auxiliary_local_queries<H: Hash>(
    proof: Option<&AuxiliaryOracleQueryProof<H>>,
    auxiliary_oracle_len: usize,
    schedule: &HolographicQuerySchedule,
) -> Result<(), Error> {
    if schedule.raa_auxiliary_queries().is_empty() {
        return Ok(());
    }
    if auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsOpen(
            "RAA auxiliary local checks require an auxiliary oracle".to_string(),
        ));
    }
    let proof = proof.ok_or_else(|| {
        Error::InvalidPcsOpen(
            "RAA auxiliary local checks require auxiliary query openings".to_string(),
        )
    })?;

    let mut local_relation_queries = proof.local_relation_queries.iter();
    for expected in schedule.raa_auxiliary_queries() {
        let main = proof
            .relation_queries
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
                let extra = local_relation_queries.next().ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA auxiliary local relation opening is missing".to_string(),
                    )
                })?;
                if extra.logical_index != expected.extra_index(0).expect("single extra relation") {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local relation opening index does not match schedule"
                            .to_string(),
                    ));
                }
                if main.value != extra.value {
                    return Err(Error::InvalidPcsOpen(
                        "RAA auxiliary local equality relation failed".to_string(),
                    ));
                }
            }
            RaaAuxiliaryRelationKind::FirstAccumulatorStep { .. } => {
                let previous = local_relation_queries.next().ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA first accumulator previous opening is missing".to_string(),
                    )
                })?;
                let u2 = local_relation_queries.next().ok_or_else(|| {
                    Error::InvalidPcsOpen(
                        "RAA first accumulator input opening is missing".to_string(),
                    )
                })?;
                if previous.logical_index != expected.extra_index(0).expect("previous u3")
                    || u2.logical_index != expected.extra_index(1).expect("u2")
                {
                    return Err(Error::InvalidPcsOpen(
                        "RAA first accumulator opening indices do not match schedule".to_string(),
                    ));
                }
                if main.value != previous.value + u2.value {
                    return Err(Error::InvalidPcsOpen(
                        "RAA first accumulator relation failed".to_string(),
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
        build_raa_aux_trace, build_raa_eval_accumulator, evaluate_multilinear, Blaze2CodeSeed,
        Blaze2FieldId, Blaze2HashId, Blaze2LeafLayout, Blaze2PackingLayout, RaaVariant,
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
        }
    }

    fn message(len: usize, offset: u64) -> Vec<B128> {
        (0..len)
            .map(|index| B128::from(offset + index as u64 * 13))
            .collect()
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
        let request = Blaze2BaseFoldOpenRequest {
            col_point: request_point,
            folded_eval,
        };
        let folded_codeword = params.praa().packed().encode_row(&folded_message);
        let trace = build_raa_aux_trace(params.praa().packed(), &folded_message).unwrap();
        let eval_accumulator =
            build_raa_eval_accumulator(params.praa().packed(), &folded_codeword, &request).unwrap();
        let mut auxiliary = Vec::with_capacity(params.spec().auxiliary_oracle_len);
        auxiliary.extend_from_slice(&trace.u2);
        auxiliary.extend_from_slice(&trace.u3);
        auxiliary.extend_from_slice(&trace.u4);
        auxiliary.extend_from_slice(&eval_accumulator);
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
            raa_final_queries: Vec::new(),
            raa_auxiliary_queries: Vec::new(),
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
            }
        );
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
            query.authenticate(&public).unwrap();
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
        proof.verify(&public, code.layout(), &schedule).unwrap();

        let mut swapped = proof.clone();
        swapped.queries.swap(0, 1);
        assert!(swapped.verify(&public, code.layout(), &schedule).is_err());

        let mut tampered_value = proof.clone();
        tampered_value.queries[0].value += B128::ONE;
        assert!(tampered_value
            .verify(&public, code.layout(), &schedule)
            .is_err());

        let mut tampered_path = proof;
        tampered_path.queries[0].path[0][0] ^= 1;
        assert!(tampered_path
            .verify(&public, code.layout(), &schedule)
            .is_err());
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
        let folded_codeword = message(params.compiler_code().layout().message_len(), 149);
        let auxiliary = message(params.spec().auxiliary_oracle_len, 151);
        let request_point = make_request_point(&params, 11);
        let request = open_request(&request_point);
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
        tampered_value.compiler_parity_folds.paths[0].steps[0].folded_value += B128::ONE;
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
        tampered_path.compiler_parity_folds.paths[0].steps[0].left_path[0][0] ^= 1;
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
        let folded_codeword = message(params.compiler_code().layout().message_len(), 61);
        let auxiliary = message(params.spec().auxiliary_oracle_len, 73);
        let request_point = make_request_point(&params, 13);
        let request = open_request(&request_point);
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
            raa_final_queries: Vec::new(),
            raa_auxiliary_queries: Vec::new(),
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
        assert_eq!(proof.auxiliary.as_ref().unwrap().query_count(), 5);
        params
            .verify_query_proof(&prequery, &request, &schedule, &proof, &[])
            .unwrap();

        let mut tampered_accumulator = proof.clone();
        tampered_accumulator
            .auxiliary
            .as_mut()
            .unwrap()
            .local_relation_queries[0]
            .value += B128::ONE;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered_accumulator, &[])
            .is_err());

        let mut tampered_permutation = proof;
        tampered_permutation
            .auxiliary
            .as_mut()
            .unwrap()
            .local_relation_queries[2]
            .value += B128::ONE;
        assert!(params
            .verify_query_proof(&prequery, &request, &schedule, &tampered_permutation, &[])
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
    }

    #[test]
    fn backend_schedule_binds_spec_and_prequery_public() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let folded_codeword = message(params.compiler_code().layout().message_len(), 43);
        let auxiliary = message(params.spec().auxiliary_oracle_len, 59);
        let request_point = make_request_point(&params, 23);
        let request = open_request(&request_point);
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
        let changed_request = open_request(&changed_request_point);
        let (changed_request_prequery, _) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &auxiliary, &changed_request)
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

        let changed_folded_codeword = message(params.compiler_code().layout().message_len(), 101);
        let (changed_prequery, _) = params
            .prove_prequery::<Blake2s>(&changed_folded_codeword, &auxiliary, &request)
            .unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = params
            .sample_query_schedule(&mut changed, &changed_prequery, &request)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);

        let changed_auxiliary = message(params.spec().auxiliary_oracle_len, 211);
        let (changed_prequery, _) = params
            .prove_prequery::<Blake2s>(&folded_codeword, &changed_auxiliary, &request)
            .unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = params
            .sample_query_schedule(&mut changed, &changed_prequery, &request)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);

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
            assert!(
                query.u4_auxiliary_index
                    < raa_auxiliary_index(RAA_AUX_EVAL_ROW, 0, auxiliary_row_len)
            );
            assert_eq!(
                query.eval_current_auxiliary_index,
                raa_auxiliary_index(RAA_AUX_EVAL_ROW, query.index, auxiliary_row_len)
            );
            assert_eq!(
                query.eval_previous_auxiliary_index,
                raa_auxiliary_index(RAA_AUX_EVAL_ROW, query.index - 1, auxiliary_row_len)
            );
        }
    }

    #[test]
    fn proof_query_classification_splits_parity_and_auxiliary_domains() {
        let layout = layout(16, 3);
        let parity = proof_query_from_sampled_index(&layout, 11, layout.parity_len() - 1).unwrap();
        assert_eq!(parity.domain, BackendProofQueryDomain::CompilerParity);
        assert_eq!(parity.index, layout.parity_len() - 1);
        assert_eq!(
            parity.physical_index,
            Some(layout.parity_to_physical(layout.parity_len() - 1).unwrap())
        );

        let auxiliary = proof_query_from_sampled_index(&layout, 11, layout.parity_len()).unwrap();
        assert_eq!(auxiliary.domain, BackendProofQueryDomain::RelationAuxiliary);
        assert_eq!(auxiliary.index, 0);
        assert_eq!(auxiliary.physical_index, None);
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
