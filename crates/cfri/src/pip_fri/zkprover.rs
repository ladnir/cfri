use crate::pip_fri::{
    prover::Prover,
    util::{
        fiat_shamir::RandomOracle,
        foldable_code::{FoldableCode, MultiplicativeFftCode},
        helper::MultilinearPolynomial,
        hiding::Masked,
        interpolate_vecs_value::QueryVecsResult,
        merkle_tree::MERKLE_ROOT_SIZE,
        query_result::QueryResult,
    },
    zkverifier::ZKVerifier,
};
use ark_ff::PrimeField;
use ark_poly::GeneralEvaluationDomain;

pub struct ZKProver<T: PrimeField, C: FoldableCode<T> = MultiplicativeFftCode<T>> {
    inner: Prover<T, C, Masked>,
}

impl<T: PrimeField> ZKProver<T> {
    pub fn new(
        total_round: usize,
        interpolate_cosets: &Vec<GeneralEvaluationDomain<T>>,
        polynomial: MultilinearPolynomial<T>,
        oracle: &RandomOracle<T>,
        tensor: &Vec<T>,
    ) -> Self {
        Self {
            inner: Prover::new_with_code_and_mode(
                total_round,
                MultiplicativeFftCode::new(interpolate_cosets),
                polynomial,
                oracle,
                tensor,
            ),
        }
    }
}

impl<T: PrimeField, C: FoldableCode<T>> ZKProver<T, C> {
    pub fn new_with_code(
        total_round: usize,
        code: C,
        polynomial: MultilinearPolynomial<T>,
        oracle: &RandomOracle<T>,
        tensor: &Vec<T>,
    ) -> Self {
        Self {
            inner: Prover::new_with_code_and_mode(total_round, code, polynomial, oracle, tensor),
        }
    }

    pub fn commit_polynomial(&mut self) -> [u8; MERKLE_ROOT_SIZE] {
        self.inner.commit_polynomial()
    }

    pub fn open(
        &mut self,
        sub_open_point: &Vec<T>,
        verifier: &mut ZKVerifier<T, C>,
    ) -> (QueryVecsResult<T>, Vec<QueryResult<T>>, Vec<QueryResult<T>>) {
        self.inner.open(sub_open_point, &mut verifier.inner)
    }
}
