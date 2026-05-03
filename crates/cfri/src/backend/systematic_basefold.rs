use crate::backend::{
    arithmetic::Field,
    binary_extension_fields::B128,
    blaze2::{absorb_blaze2_code_spec, Blaze2Code, Blaze2CodeSpec},
    hash::{Blake2s, Hash, Output},
    Error,
};
use crate::transcript::Transcript as CfriTranscript;

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
    Auxiliary,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldPrequeryPublic<H: Hash> {
    pub compiler_parity: CompilerParityPublicCommitment<H>,
}

#[derive(Clone, Debug)]
pub struct Blaze2BaseFoldProverState<H: Hash> {
    compiler_parity: CompilerParityCommitment<H>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blaze2BaseFoldQueryProof<H: Hash> {
    pub compiler_parity: CompilerParityQueryProof<H>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HolographicQuerySchedule {
    input_queries: Vec<SystematicInputQuery>,
    proof_queries: Vec<BackendProofQuery>,
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
    ) -> Result<
        (
            Blaze2BaseFoldPrequeryPublic<H>,
            Blaze2BaseFoldProverState<H>,
        ),
        Error,
    > {
        let compiler_parity = self.compiler_code.commit_parity(folded_codeword)?;
        let public = Blaze2BaseFoldPrequeryPublic {
            compiler_parity: compiler_parity.public(),
        };
        Ok((public, Blaze2BaseFoldProverState { compiler_parity }))
    }

    pub fn sample_query_schedule<H: Hash, S>(
        &self,
        transcript: &mut CfriTranscript<H, S>,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
    ) -> Result<HolographicQuerySchedule, Error> {
        absorb_blaze2_basefold_backend_spec(transcript, self.spec());
        absorb_blaze2_basefold_prequery_public(transcript, prequery);
        HolographicQuerySchedule::sample(
            transcript,
            self.compiler_code.layout(),
            self.query_schedule_spec(),
        )
    }

    pub fn open_query_proof<H: Hash>(
        &self,
        state: &Blaze2BaseFoldProverState<H>,
        schedule: &HolographicQuerySchedule,
    ) -> Result<Blaze2BaseFoldQueryProof<H>, Error> {
        Ok(Blaze2BaseFoldQueryProof {
            compiler_parity: state.compiler_parity.prove_schedule(schedule)?,
        })
    }

    pub fn verify_query_proof<H: Hash>(
        &self,
        prequery: &Blaze2BaseFoldPrequeryPublic<H>,
        schedule: &HolographicQuerySchedule,
        proof: &Blaze2BaseFoldQueryProof<H>,
        top_queries: &[TopQuery<B128>],
    ) -> Result<(), Error> {
        schedule.validate_top_queries(top_queries)?;
        proof.compiler_parity.verify(
            &prequery.compiler_parity,
            self.compiler_code.layout(),
            schedule,
        )
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
        validate_message_and_parity_output(self.layout(), message, out)?;

        let parity_expansion_factor = self.layout.parity_expansion_factor();
        for (chunk, &value) in out.chunks_exact_mut(parity_expansion_factor).zip(message) {
            chunk.fill(value);
        }

        let mut chunk_len = parity_expansion_factor;
        for level in &self.parity_fold_table {
            let half_chunk_len = chunk_len;
            chunk_len <<= 1;
            debug_assert_eq!(level.len(), half_chunk_len);
            for chunk in out.chunks_exact_mut(chunk_len) {
                for j in 0..half_chunk_len {
                    let left = chunk[j];
                    let right = chunk[j + half_chunk_len];
                    let t = level[j];
                    chunk[j + half_chunk_len] = left + right * (t + B128::ONE);
                    chunk[j] = left + right * t;
                }
            }
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
        if out.len() != self.layout.codeword_len() {
            return Err(Error::InvalidPcsOpen(format!(
                "physical codeword output has length {}, expected {}",
                out.len(),
                self.layout.codeword_len()
            )));
        }
        self.encode_parity_into(message, parity_scratch)?;

        for (logical_index, &value) in message.iter().enumerate() {
            let physical_index = self.layout.systematic_to_physical(logical_index)?;
            out[physical_index] = value;
        }
        for (logical_index, &value) in parity_scratch.iter().enumerate() {
            let physical_index = self.layout.parity_to_physical(logical_index)?;
            out[physical_index] = value;
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
        for _ in 0..spec.q_raa_input {
            let logical_index = squeeze_bounded_index(transcript, layout.systematic_len())?;
            input_queries.push(SystematicInputQuery {
                logical_index,
                physical_index: layout.systematic_to_physical(logical_index)?,
            });
        }

        let proof_domain_len = layout.parity_len() + spec.auxiliary_oracle_len;
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
        })
    }

    pub fn input_queries(&self) -> &[SystematicInputQuery] {
        &self.input_queries
    }

    pub fn proof_queries(&self) -> &[BackendProofQuery] {
        &self.proof_queries
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
    Ok(())
}

fn validate_message_and_parity_output(
    layout: &SystematicAugmentedRfcLayout,
    message: &[B128],
    out: &[B128],
) -> Result<(), Error> {
    if message.len() != layout.message_len() {
        return Err(Error::InvalidPcsOpen(format!(
            "systematic RFC message has length {}, expected {}",
            message.len(),
            layout.message_len()
        )));
    }
    if out.len() != layout.parity_len() {
        return Err(Error::InvalidPcsOpen(format!(
            "systematic RFC parity output has length {}, expected {}",
            out.len(),
            layout.parity_len()
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
    if spec.q_backend_proof == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic BaseFold schedule needs at least one backend proof query".to_string(),
        ));
    }
    if layout.systematic_len() == 0 {
        return Err(Error::InvalidPcsParam(
            "systematic query domain must be non-empty".to_string(),
        ));
    }
    if layout.parity_len() + spec.auxiliary_oracle_len == 0 {
        return Err(Error::InvalidPcsParam(
            "backend proof query domain must be non-empty".to_string(),
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
    let proof_domain_len = layout.parity_len() + auxiliary_oracle_len;
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
            domain: BackendProofQueryDomain::Auxiliary,
            index: sampled_index - layout.parity_len(),
            physical_index: None,
        })
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

    let mut table = Vec::with_capacity(layout.num_rounds());
    let mut level_len = layout.parity_expansion_factor();
    for level_index in 0..layout.num_rounds() {
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
        Blaze2CodeSeed, Blaze2FieldId, Blaze2HashId, Blaze2LeafLayout, Blaze2PackingLayout,
        RaaVariant,
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
            q_raa_input: 5,
            q_backend_proof: 7,
            auxiliary_oracle_len: 11,
        }
    }

    fn message(len: usize, offset: u64) -> Vec<B128> {
        (0..len)
            .map(|index| B128::from(offset + index as u64 * 13))
            .collect()
    }

    fn blaze2_backend_spec() -> Blaze2BaseFoldBackendSpec {
        Blaze2BaseFoldBackendSpec {
            praa: Blaze2CodeSpec {
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
            },
            compiler_code: SystematicFoldableCodeSpec {
                version: 1,
                compiler_message_len: 32,
                compiler_systematic_len: 32,
                compiler_parity_len: 32 * 3,
                compiler_codeword_len: 32 * 4,
                parity_expansion_factor: 3,
                seed: [23; 32],
            },
            q_raa_input: 5,
            q_backend_proof: 7,
            auxiliary_oracle_len: 11,
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
                    domain: BackendProofQueryDomain::Auxiliary,
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
        let folded_codeword = message(params.compiler_code().layout().message_len(), 41);
        let (prequery, state) = params.prove_prequery::<Blake2s>(&folded_codeword).unwrap();

        assert_eq!(
            prequery.compiler_parity.len,
            params.compiler_code().layout().parity_len()
        );

        let mut transcript = CfriTranscript::<Blake2s>::new();
        let schedule = params
            .sample_query_schedule(&mut transcript, &prequery)
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
            .verify_query_proof(&prequery, &schedule, &proof, &top_queries)
            .unwrap();

        let mut bad_top_queries = top_queries.clone();
        bad_top_queries[0].index ^= 1;
        assert!(params
            .verify_query_proof(&prequery, &schedule, &proof, &bad_top_queries)
            .is_err());

        let mut bad_proof = proof;
        if let Some(query) = bad_proof.compiler_parity.queries.first_mut() {
            query.value += B128::ONE;
            assert!(params
                .verify_query_proof(&prequery, &schedule, &bad_proof, &top_queries)
                .is_err());
        }
    }

    #[test]
    fn backend_schedule_binds_spec_and_prequery_public() {
        let params = Blaze2BaseFoldBackendParams::new(blaze2_backend_spec()).unwrap();
        let folded_codeword = message(params.compiler_code().layout().message_len(), 43);
        let (prequery, _) = params.prove_prequery::<Blake2s>(&folded_codeword).unwrap();

        let mut lhs = CfriTranscript::<Blake2s>::new();
        let lhs_schedule = params.sample_query_schedule(&mut lhs, &prequery).unwrap();

        let mut rhs = CfriTranscript::<Blake2s>::new();
        let rhs_schedule = params.sample_query_schedule(&mut rhs, &prequery).unwrap();
        assert_eq!(lhs_schedule, rhs_schedule);

        let changed_folded_codeword = message(params.compiler_code().layout().message_len(), 101);
        let (changed_prequery, _) = params
            .prove_prequery::<Blake2s>(&changed_folded_codeword)
            .unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = params
            .sample_query_schedule(&mut changed, &changed_prequery)
            .unwrap();
        assert_ne!(lhs_schedule, changed_schedule);

        let mut changed_spec = params.spec().clone();
        changed_spec.q_backend_proof += 1;
        let changed_params = Blaze2BaseFoldBackendParams::new(changed_spec).unwrap();
        let mut changed = CfriTranscript::<Blake2s>::new();
        let changed_schedule = changed_params
            .sample_query_schedule(&mut changed, &prequery)
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
                BackendProofQueryDomain::Auxiliary => {
                    assert!(query.index < spec.auxiliary_oracle_len);
                    assert_eq!(query.physical_index, None);
                }
            }
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
        assert_eq!(auxiliary.domain, BackendProofQueryDomain::Auxiliary);
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
}
