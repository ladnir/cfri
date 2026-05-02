use crate::pip_fri::util::{
    foldable_code::FoldableCode, helper::Helper, helper::MultilinearPolynomial,
};
use ark_ff::PrimeField;
use std::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone)]
pub struct EncodedInitial<T: PrimeField, S> {
    pub polynomials: Vec<Vec<T>>,
    pub rlc_polynomial: Vec<T>,
    pub tensor_polynomial: Vec<T>,
    pub state: S,
}

pub trait HidingMode<T: PrimeField>: Clone + Debug + Send + Sync + 'static {
    type ProverState: Clone + Send + Sync;

    fn encode_initial<C: FoldableCode<T>>(
        total_round: usize,
        code: &C,
        polynomial: &MultilinearPolynomial<T>,
        poly_num: usize,
        tensor: &[T],
        rlc: T,
    ) -> EncodedInitial<T, Self::ProverState>;

    fn mask_evaluation(state: &Self::ProverState, point: &[T]) -> Option<T>;

    fn initial_function_value(values: &[T], combination: &[T]) -> T;

    fn terminal_evaluation(public_eval: T, rlc: T, mask_eval: Option<T>) -> T;
}

#[derive(Clone, Debug)]
pub struct Transparent;

impl<T: PrimeField> HidingMode<T> for Transparent {
    type ProverState = ();

    fn encode_initial<C: FoldableCode<T>>(
        _total_round: usize,
        code: &C,
        polynomial: &MultilinearPolynomial<T>,
        poly_num: usize,
        tensor: &[T],
        rlc: T,
    ) -> EncodedInitial<T, Self::ProverState> {
        let polynomials = code.encode_multilinear_sub_polynomials(polynomial, poly_num);
        let rlc_polynomial = random_linear_combination(&polynomials, rlc);
        let tensor_polynomial = Helper::linear_combine(&tensor.to_vec(), &polynomials);
        EncodedInitial {
            polynomials,
            rlc_polynomial,
            tensor_polynomial,
            state: (),
        }
    }

    fn mask_evaluation(_state: &Self::ProverState, _point: &[T]) -> Option<T> {
        None
    }

    fn initial_function_value(values: &[T], combination: &[T]) -> T {
        assert_eq!(values.len(), combination.len());
        values
            .iter()
            .zip(combination.iter())
            .fold(T::zero(), |acc, (&value, &weight)| acc + value * weight)
    }

    fn terminal_evaluation(public_eval: T, _rlc: T, _mask_eval: Option<T>) -> T {
        public_eval
    }
}

#[derive(Clone, Debug)]
pub struct Masked;

impl<T: PrimeField> HidingMode<T> for Masked {
    type ProverState = MultilinearPolynomial<T>;

    fn encode_initial<C: FoldableCode<T>>(
        total_round: usize,
        code: &C,
        polynomial: &MultilinearPolynomial<T>,
        poly_num: usize,
        tensor: &[T],
        rlc: T,
    ) -> EncodedInitial<T, Self::ProverState> {
        #[cfg(feature = "parallel")]
        let mut polynomials: Vec<Vec<T>> = polynomial
            .chunks(poly_num)
            .par_iter()
            .map(|chunk| {
                let mut rng = rand::thread_rng();
                let coefficients = chunk.coefficients();
                let mut combined = Vec::with_capacity(coefficients.len() * 2);
                combined.extend_from_slice(coefficients);
                combined.extend((0..coefficients.len()).map(|_| T::rand(&mut rng)));
                code.encode_sub_polynomial(&combined)
            })
            .collect();

        #[cfg(not(feature = "parallel"))]
        let mut polynomials: Vec<Vec<T>> = polynomial
            .chunks(poly_num)
            .iter()
            .map(|chunk| {
                let mut rng = rand::thread_rng();
                let coefficients = chunk.coefficients();
                let mut combined = Vec::with_capacity(coefficients.len() * 2);
                combined.extend_from_slice(coefficients);
                combined.extend((0..coefficients.len()).map(|_| T::rand(&mut rng)));
                code.encode_sub_polynomial(&combined)
            })
            .collect();

        let mask_polynomial = MultilinearPolynomial::rand(total_round);
        let mask_evals = code.encode_sub_polynomial(mask_polynomial.coefficients());

        let mut rlc_polynomial = random_linear_combination(&polynomials, rlc);
        for (acc, mask) in rlc_polynomial.iter_mut().zip(mask_evals.iter()) {
            *acc *= rlc;
            *acc += mask;
        }

        let tensor_polynomial = Helper::linear_combine(&tensor.to_vec(), &polynomials)
            .iter()
            .zip(mask_evals.iter())
            .map(|(&value, &mask)| value + rlc * mask)
            .collect();

        polynomials.push(mask_evals);

        EncodedInitial {
            polynomials,
            rlc_polynomial,
            tensor_polynomial,
            state: mask_polynomial,
        }
    }

    fn mask_evaluation(state: &Self::ProverState, point: &[T]) -> Option<T> {
        Some(state.evaluate(&point.to_vec()))
    }

    fn initial_function_value(values: &[T], combination: &[T]) -> T {
        assert_eq!(values.len(), combination.len() + 1);
        values[..combination.len()]
            .iter()
            .zip(combination.iter())
            .fold(T::zero(), |acc, (&value, &weight)| acc + value * weight)
    }

    fn terminal_evaluation(public_eval: T, rlc: T, mask_eval: Option<T>) -> T {
        public_eval + rlc * mask_eval.expect("masked PiPFRI verifier requires mask evaluation")
    }
}

#[derive(Clone, Debug)]
pub struct ModeMarker<M>(PhantomData<M>);

impl<M> Default for ModeMarker<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

fn random_linear_combination<T: PrimeField>(polynomials: &[Vec<T>], rlc: T) -> Vec<T> {
    let mut rlc_polynomial = polynomials[0].clone();
    for polynomial in polynomials.iter().skip(1) {
        for (acc, value) in rlc_polynomial.iter_mut().zip(polynomial.iter()) {
            *acc *= rlc;
            *acc += value;
        }
    }
    rlc_polynomial
}
