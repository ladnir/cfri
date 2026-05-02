use super::verifier::Verifier;
use crate::pip_fri::util::helper::MultilinearPolynomial;
use crate::pip_fri::util::interpolate_vecs_value::{
    get_poly_num, InterpolateVecsValue, QueryVecsResult,
};
pub use crate::pip_fri::util::merkle_tree::MERKLE_ROOT_SIZE;
use crate::pip_fri::util::query_result::QueryResult;
use crate::pip_fri::util::{
    fiat_shamir::RandomOracle,
    foldable_code::{FoldableCode, MultiplicativeFftCode},
    helper::Helper,
    hiding::{HidingMode, ModeMarker, Transparent},
    merkle_tree::MerkleTreeProver,
    CODE_RATE,
};
use ark_ff::PrimeField;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use ark_std::{end_timer, start_timer};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
const MIN_PARALLEL_PROVER_ENCODING_WORK: usize = 1 << 14;

#[derive(Clone)]
struct InterpolateValue<T: PrimeField> {
    value: Vec<T>,
    merkle_tree: MerkleTreeProver,
}

impl<T: PrimeField> InterpolateValue<T> {
    fn new(value: Vec<T>) -> Self {
        let len = value.len() / 2;
        let merkle_tree = MerkleTreeProver::new(
            (0..len)
                .map(|i| Helper::to_bytes_vec(&[value[i], value[i + len]]))
                .collect(),
        );
        Self { value, merkle_tree }
    }

    fn leave_num(&self) -> usize {
        self.merkle_tree.leave_num()
    }

    fn commit(&self) -> [u8; MERKLE_ROOT_SIZE] {
        self.merkle_tree.commit()
    }

    fn query(&self, leaf_indices: &Vec<usize>) -> QueryResult<T> {
        let len = self.merkle_tree.leave_num();
        let proof_values = leaf_indices
            .iter()
            .flat_map(|j| [(*j, self.value[*j]), (*j + len, self.value[*j + len])])
            .collect();
        let proof_bytes = self.merkle_tree.open(&leaf_indices);
        QueryResult {
            proof_bytes,
            proof_values,
        }
    }
}

#[derive(Clone)]
pub struct Prover<
    T: PrimeField,
    C: FoldableCode<T> = MultiplicativeFftCode<T>,
    M: HidingMode<T> = Transparent,
> {
    // round number for sub_polynomials
    pub total_round: usize,
    pub code: C,
    pub interpolate_initial_polynomials: InterpolateVecsValue<T>,
    mode_state: M::ProverState,
    interpolate_rlc_polynomial: Vec<T>,
    interpolate_tensor_polynomial: Vec<T>,
    functions: Vec<InterpolateValue<T>>,
    foldings: Vec<InterpolateValue<T>>,
    pub oracle: RandomOracle<T>,
    final_value: Option<T>,
    _mode: ModeMarker<M>,
}

impl<T: PrimeField> Prover<T> {
    pub fn new(
        total_round: usize,
        interpolate_cosets: &Vec<GeneralEvaluationDomain<T>>,
        polynomial: MultilinearPolynomial<T>,
        // oracle has a random_linear_combination challenge
        oracle: &RandomOracle<T>,
        // the fixed combination to combine multiple sub polynomials
        tensor: &Vec<T>,
    ) -> Prover<T> {
        let poly_num = get_poly_num(&polynomial);
        let step = start_timer!(|| "NTT");

        let chunks = polynomial.chunks(poly_num);
        #[cfg(feature = "parallel")]
        let interpolation_sub_polynomials: Vec<Vec<T>> =
            if polynomial.coefficients().len() >= MIN_PARALLEL_PROVER_ENCODING_WORK {
                chunks
                    .par_iter()
                    .map(|chunk| interpolate_cosets[0].fft(chunk.coefficients()))
                    .collect()
            } else {
                chunks
                    .iter()
                    .map(|chunk| interpolate_cosets[0].fft(chunk.coefficients()))
                    .collect()
            };

        #[cfg(not(feature = "parallel"))]
        let interpolation_sub_polynomials: Vec<Vec<T>> = chunks
            .iter()
            .map(|chunk| interpolate_cosets[0].fft(chunk.coefficients()))
            .collect();

        end_timer!(step);

        let step = start_timer!(|| "rlc");
        let mut rlc_polynomial = interpolation_sub_polynomials[0].clone();
        for polynomial in interpolation_sub_polynomials.iter().skip(1) {
            for j in 0..rlc_polynomial.len() {
                rlc_polynomial[j] *= oracle.rlc;
                rlc_polynomial[j] += polynomial[j];
            }
        }
        let tensor_polynomial = Helper::linear_combine(tensor, &interpolation_sub_polynomials);
        end_timer!(step);

        let step = start_timer!(|| "Merkle tree");
        let interpolate_initial_polynomials =
            InterpolateVecsValue::new(interpolation_sub_polynomials);
        end_timer!(step);

        Prover {
            total_round,
            code: MultiplicativeFftCode::new(interpolate_cosets),
            interpolate_initial_polynomials,
            mode_state: (),
            interpolate_rlc_polynomial: rlc_polynomial,
            interpolate_tensor_polynomial: tensor_polynomial,
            functions: vec![],
            foldings: vec![],
            oracle: oracle.clone(),
            final_value: None,
            _mode: ModeMarker::default(),
        }
    }
}

impl<T: PrimeField, C: FoldableCode<T>> Prover<T, C, Transparent> {
    pub fn new_with_code(
        total_round: usize,
        code: C,
        polynomial: MultilinearPolynomial<T>,
        // oracle has a random_linear_combination challenge
        oracle: &RandomOracle<T>,
        // the fixed combination to combine multiple sub polynomials
        tensor: &Vec<T>,
    ) -> Prover<T, C, Transparent> {
        Self::new_with_code_and_mode(total_round, code, polynomial, oracle, tensor)
    }
}

impl<T: PrimeField, C: FoldableCode<T>, M: HidingMode<T>> Prover<T, C, M> {
    pub fn new_with_code_and_mode(
        total_round: usize,
        code: C,
        polynomial: MultilinearPolynomial<T>,
        // oracle has a random_linear_combination challenge
        oracle: &RandomOracle<T>,
        // the fixed combination to combine multiple sub polynomials
        tensor: &Vec<T>,
    ) -> Prover<T, C, M> {
        // Divide the polynomial into polynomials, and generate the rlc_poly and tensor_poly
        let poly_num = get_poly_num(&polynomial);
        let step = start_timer!(|| "NTT");

        let encoded = M::encode_initial(
            total_round,
            &code,
            &polynomial,
            poly_num,
            tensor,
            oracle.rlc,
        );

        end_timer!(step);

        // this step takes the majority of time
        let step = start_timer!(|| "Merkle tree");
        let interpolate_initial_polynomials = InterpolateVecsValue::new(encoded.polynomials);
        end_timer!(step);

        Prover {
            total_round,
            code,
            interpolate_initial_polynomials,
            mode_state: encoded.state,
            // interpolate_polynomials,
            interpolate_rlc_polynomial: encoded.rlc_polynomial,
            interpolate_tensor_polynomial: encoded.tensor_polynomial,
            functions: vec![],
            foldings: vec![],
            oracle: oracle.clone(),
            final_value: None,
            _mode: ModeMarker::default(),
        }
    }

    pub fn commit_polynomial(&mut self) -> [u8; MERKLE_ROOT_SIZE] {
        // let step = start_timer!(|| "Merkle tree");
        let com = self.interpolate_initial_polynomials.commit();
        // end_timer!(step);
        com
    }

    // used for function recursive
    fn fold(&self, values: &Vec<T>, parameter: T, round: usize) -> Vec<T> {
        let len = values.len() / 2;
        let res = (0..len)
            .into_iter()
            .map(|i| {
                let x = values[i];
                let nx = values[i + len];
                let new_v = (x + nx) + parameter * (x - nx) * self.code.fold_weight(round, i);
                new_v * T::from_u64(2 as u64).unwrap().inverse().unwrap()
            })
            .collect();
        res
    }

    // generate f_1, f_2, ..., f_{\mu - 1}, and f_\mu
    // function[0], function[1], ..., function[\mu - 2]
    // function[0] is f_1, and f_\mu is a constant
    // f_0 is a virtual polynomial as tensor_polynomial
    pub fn commit_functions(&mut self, sub_open_point: &Vec<T>, verifier: &mut Verifier<T, C, M>) {
        let mut evaluation = None;

        for round in 0..self.total_round {
            let next_evaluation = self.fold(
                if round == 0 {
                    &self.interpolate_tensor_polynomial
                } else {
                    &self.functions[round - 1].value
                },
                sub_open_point[round],
                round,
            );
            if round < self.total_round - 1 {
                self.functions.push(InterpolateValue::new(next_evaluation));
            } else {
                evaluation = Some(next_evaluation[0]);
            }
        }
        for i in 0..(self.total_round - 1) {
            let function = &self.functions[i];
            verifier.set_function(function.leave_num(), &function.commit());
        }
        if let Some(mask_evaluation) = M::mask_evaluation(&self.mode_state, sub_open_point) {
            verifier.set_mask_evaluation(mask_evaluation);
        }
        verifier.set_evaluation(evaluation.unwrap());
    }

    pub fn commit_foldings(&self, verifier: &mut Verifier<T, C, M>) {
        for i in 0..(self.total_round - 1) {
            let interpolation = &self.foldings[i];
            verifier.receive_folding_root(interpolation.leave_num(), interpolation.commit());
        }
        verifier.set_final_value(self.final_value.unwrap());
    }

    // generate and push foldings, p_0, p_1, ..., p_{\mu - 1}, where p_{\mu - 1} is a constant
    // p_0 = g_0(X) + \alpha_0 h_0(X), and f_0(X) = g_0(X^2) + X h_0(X^2)
    // \phi_1(X) = p_0 + \alpha_0^2 f_1,
    // p_1 = g_1 + alpha_1 h_1, and \phi_1(X) = g_1(X^2) + X h_1(X^2)
    // ....
    // folding[0] is p_0,
    fn evaluation_next_domain(
        &self,
        round: usize,
        last_challenge: Option<T>,
        cur_challenge: T,
    ) -> Vec<T> {
        let mut res = vec![];
        let len = self.code.domain_size(round);
        let get_folding_value = if round == 0 {
            &self.interpolate_rlc_polynomial
        } else {
            &self.foldings[round - 1].value
        };
        for i in 0..(len / 2) {
            if round == 0 {
                assert_eq!(last_challenge, None);
                let x = get_folding_value[i];
                let nx = get_folding_value[i + len / 2];
                let new_v = (x + nx) + cur_challenge * (x - nx) * self.code.fold_weight(round, i);
                res.push(new_v);
            } else {
                let fv = &self.functions[round - 1].value;
                assert_eq!(fv.len(), get_folding_value.len());
                let x = fv[i];
                let nx = fv[i + len / 2];
                let last_challenge_square = last_challenge.unwrap().pow([2 as u64]);
                let phi_x = get_folding_value[i] + last_challenge_square * x;
                let phi_nx = get_folding_value[i + len / 2] + last_challenge_square * nx;
                let new_v = (phi_x + phi_nx)
                    + cur_challenge * (phi_x - phi_nx) * self.code.fold_weight(round, i);
                res.push(new_v);
            }
        }
        res
    }

    // commit foldings
    pub fn prove(&mut self) {
        for i in 0..self.total_round {
            let cur_challenge = self.oracle.folding_challenges[i];
            let mut last_challenge = None;
            if i > 0 {
                last_challenge = Some(self.oracle.folding_challenges[i - 1]);
            }
            if i < self.total_round - 1 {
                let next_evalutation =
                    self.evaluation_next_domain(i, last_challenge, cur_challenge);
                self.foldings.push(InterpolateValue::new(next_evalutation));
            } else {
                let next_evalutation =
                    self.evaluation_next_domain(i, last_challenge, cur_challenge);
                assert_eq!(next_evalutation.len(), 1 << CODE_RATE);
                self.final_value = Some(next_evalutation[0]);
            }
        }
    }

    pub fn query(&self) -> (QueryVecsResult<T>, Vec<QueryResult<T>>, Vec<QueryResult<T>>) {
        let mut folding_res = vec![];
        let mut functions_res = vec![];
        let mut polynomial_res = None;
        let mut leaf_indices = self.oracle.query_list.clone();

        for i in 0..self.total_round {
            let len = self.code.domain_size(i);
            leaf_indices = leaf_indices.iter_mut().map(|v| *v % (len >> 1)).collect();
            leaf_indices.sort();
            leaf_indices.dedup();

            if i == 0 {
                polynomial_res = Some(self.interpolate_initial_polynomials.query(&leaf_indices));
            } else {
                functions_res.push(self.functions[i - 1].query(&leaf_indices));
                folding_res.push(self.foldings[i - 1].query(&leaf_indices));
            }
        }
        (polynomial_res.unwrap(), folding_res, functions_res)
    }

    pub fn open(
        &mut self,
        sub_open_point: &Vec<T>,
        verifier: &mut Verifier<T, C, M>,
    ) -> (QueryVecsResult<T>, Vec<QueryResult<T>>, Vec<QueryResult<T>>) {
        self.commit_functions(&sub_open_point.to_vec(), verifier);
        self.prove();
        self.commit_foldings(verifier);
        self.query()
    }
}
