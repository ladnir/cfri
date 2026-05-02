use crate::pip_fri::util::helper::MultilinearPolynomial;
use ark_ff::PrimeField;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
const MIN_PARALLEL_ENCODING_WORK: usize = 1 << 14;

pub trait FoldableCode<T: PrimeField>: Clone + Sync {
    fn encode_sub_polynomial(&self, coefficients: &[T]) -> Vec<T>;

    fn domain_size(&self, round: usize) -> usize;

    fn fold_weight(&self, round: usize, index: usize) -> T;

    fn encode_multilinear_sub_polynomials(
        &self,
        polynomial: &MultilinearPolynomial<T>,
        poly_num: usize,
    ) -> Vec<Vec<T>> {
        let chunks = polynomial.chunks(poly_num);

        #[cfg(feature = "parallel")]
        if polynomial.coefficients().len() >= MIN_PARALLEL_ENCODING_WORK {
            return chunks
                .par_iter()
                .map(|chunk| self.encode_sub_polynomial(chunk.coefficients()))
                .collect();
        }

        chunks
            .iter()
            .map(|chunk| self.encode_sub_polynomial(chunk.coefficients()))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct MultiplicativeFftCode<T: PrimeField> {
    cosets: Vec<GeneralEvaluationDomain<T>>,
}

impl<T: PrimeField> MultiplicativeFftCode<T> {
    pub fn new(cosets: &[GeneralEvaluationDomain<T>]) -> Self {
        Self {
            cosets: cosets.to_vec(),
        }
    }

    pub fn cosets(&self) -> &[GeneralEvaluationDomain<T>] {
        &self.cosets
    }
}

impl<T: PrimeField> FoldableCode<T> for MultiplicativeFftCode<T> {
    fn encode_sub_polynomial(&self, coefficients: &[T]) -> Vec<T> {
        self.cosets[0].fft(coefficients)
    }

    fn domain_size(&self, round: usize) -> usize {
        self.cosets[round].size()
    }

    fn fold_weight(&self, round: usize, index: usize) -> T {
        self.cosets[round].element(index).inverse().unwrap()
    }
}
