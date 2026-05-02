use crate::pip_fri::{
    util::{
        fiat_shamir::RandomOracle,
        foldable_code::{FoldableCode, MultiplicativeFftCode},
        hiding::Masked,
        interpolate_vecs_value::QueryVecsResult,
        merkle_tree::MERKLE_ROOT_SIZE,
        query_result::QueryResult,
    },
    verifier::Verifier,
};
use ark_ff::PrimeField;
use ark_poly::GeneralEvaluationDomain;

#[derive(Clone, Debug)]
pub struct ZKVerifier<T: PrimeField, C: FoldableCode<T> = MultiplicativeFftCode<T>> {
    pub(crate) inner: Verifier<T, C, Masked>,
}

impl<T: PrimeField> ZKVerifier<T> {
    pub fn new(
        total_round: usize,
        commitment: [u8; MERKLE_ROOT_SIZE],
        coset: &Vec<GeneralEvaluationDomain<T>>,
        oracle: &RandomOracle<T>,
        open_point: &Vec<T>,
        combination: &Vec<T>,
    ) -> Self {
        Self {
            inner: Verifier::new_with_code_and_mode(
                total_round,
                commitment,
                MultiplicativeFftCode::new(coset),
                oracle,
                open_point,
                combination,
            ),
        }
    }
}

impl<T: PrimeField, C: FoldableCode<T>> ZKVerifier<T, C> {
    pub fn new_with_code(
        total_round: usize,
        commitment: [u8; MERKLE_ROOT_SIZE],
        code: C,
        oracle: &RandomOracle<T>,
        open_point: &Vec<T>,
        combination: &Vec<T>,
    ) -> Self {
        Self {
            inner: Verifier::new_with_code_and_mode(
                total_round,
                commitment,
                code,
                oracle,
                open_point,
                combination,
            ),
        }
    }

    pub fn verify(
        &self,
        polynomial_proof: &QueryVecsResult<T>,
        folding_proof: &Vec<QueryResult<T>>,
        function_proof: &Vec<QueryResult<T>>,
        evaluation: T,
    ) -> bool {
        self.inner
            .verify(polynomial_proof, folding_proof, function_proof, evaluation)
    }
}
