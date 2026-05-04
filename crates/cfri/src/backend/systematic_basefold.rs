use crate::backend::{arithmetic::Field, binary_extension_fields::B128, hash::Hash, Error};
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
pub struct SystematicAugmentedRfcLayout {
    spec: SystematicFoldableCodeSpec,
    num_rounds: usize,
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

pub fn fold_systematic_pair<F: Field>(left: F, right: F, alpha: F) -> F {
    (F::ONE - alpha) * left + alpha * right
}

pub fn fold_rfc_parity_pair<F: Field>(base: F, direction: F, alpha: F) -> F {
    base + alpha * direction
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
        let direction = right - left;
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
        assert_eq!(fold_rfc_parity_pair(left, direction, B128::ZERO), left);
        assert_eq!(fold_rfc_parity_pair(left, direction, B128::ONE), right);
        assert_ne!(
            fold_systematic_pair(left, right, alpha),
            fold_rfc_parity_pair(left, right, alpha),
            "raw systematic coordinates must not silently reuse the RFC linear fold"
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
