use std::collections::HashMap;

use crate::pip_fri::util::fiat_shamir::RandomOracle;
use crate::pip_fri::util::foldable_code::{FoldableCode, MultiplicativeFftCode};
use crate::pip_fri::util::hiding::{HidingMode, ModeMarker, Transparent};
use crate::pip_fri::util::interpolate_vecs_value::*;
use crate::pip_fri::util::merkle_tree::MERKLE_ROOT_SIZE;
use crate::pip_fri::util::{merkle_tree::MerkleTreeVerifier, query_result::QueryResult};
use ark_ff::PrimeField;
use ark_poly::GeneralEvaluationDomain;

#[derive(Clone, Debug)]
pub struct Verifier<
    T: PrimeField,
    C: FoldableCode<T> = MultiplicativeFftCode<T>,
    M: HidingMode<T> = Transparent,
> {
    total_round: usize,
    code: C,
    initial_proof: MerkleTreeVerifier,
    function_root: Vec<MerkleTreeVerifier>,
    folding_root: Vec<MerkleTreeVerifier>,
    oracle: RandomOracle<T>,
    final_value: Option<T>,
    evaluation: Option<T>,
    mask_evaluation: Option<T>,
    open_point: Vec<T>,
    combination: Vec<T>,
    _mode: ModeMarker<M>,
}

impl<T: PrimeField> Verifier<T> {
    pub fn new(
        total_round: usize,
        // for rlc_polynomial
        commitment: [u8; MERKLE_ROOT_SIZE],
        coset: &Vec<GeneralEvaluationDomain<T>>,
        oracle: &RandomOracle<T>,
        open_point: &Vec<T>,
        combination: &Vec<T>,
    ) -> Self {
        Self::new_with_code(
            total_round,
            commitment,
            MultiplicativeFftCode::new(coset),
            oracle,
            open_point,
            combination,
        )
    }
}

impl<T: PrimeField, C: FoldableCode<T>> Verifier<T, C, Transparent> {
    pub fn new_with_code(
        total_round: usize,
        // for rlc_polynomial
        commitment: [u8; MERKLE_ROOT_SIZE],
        code: C,
        oracle: &RandomOracle<T>,
        open_point: &Vec<T>,
        combination: &Vec<T>,
    ) -> Verifier<T, C, Transparent> {
        Self::new_with_code_and_mode(
            total_round,
            commitment,
            code,
            oracle,
            open_point,
            combination,
        )
    }
}

impl<T: PrimeField, C: FoldableCode<T>, M: HidingMode<T>> Verifier<T, C, M> {
    pub fn new_with_code_and_mode(
        total_round: usize,
        // for rlc_polynomial
        commitment: [u8; MERKLE_ROOT_SIZE],
        code: C,
        oracle: &RandomOracle<T>,
        open_point: &Vec<T>,
        combination: &Vec<T>,
    ) -> Verifier<T, C, M> {
        let initial_leave_num = code.domain_size(0) / 2;
        Verifier {
            total_round,
            code,
            initial_proof: MerkleTreeVerifier::new(initial_leave_num, &commitment),
            function_root: vec![],
            folding_root: vec![],
            oracle: oracle.clone(),
            final_value: None,
            evaluation: None,
            mask_evaluation: None,
            open_point: open_point.clone(),
            combination: combination.clone(),
            _mode: ModeMarker::default(),
        }
    }

    pub fn get_combination(&self) -> Vec<T> {
        self.combination.clone()
    }

    pub fn set_evaluation(&mut self, evaluation: T) {
        self.evaluation = Some(evaluation);
    }

    pub fn set_mask_evaluation(&mut self, mask_evaluation: T) {
        self.mask_evaluation = Some(mask_evaluation);
    }

    pub fn set_function(&mut self, leave_number: usize, function_root: &[u8; MERKLE_ROOT_SIZE]) {
        self.function_root.push(MerkleTreeVerifier {
            merkle_root: function_root.clone(),
            leave_number,
        });
    }

    pub fn receive_folding_root(
        &mut self,
        leave_number: usize,
        folding_root: [u8; MERKLE_ROOT_SIZE],
    ) {
        self.folding_root.push(MerkleTreeVerifier {
            leave_number,
            merkle_root: folding_root,
        });
    }

    pub fn set_final_value(&mut self, value: T) {
        self.final_value = Some(value);
    }

    pub fn public_inputs_match(
        &self,
        commitment: &[u8; MERKLE_ROOT_SIZE],
        open_point: &[T],
        combination: &[T],
    ) -> bool {
        self.initial_proof.merkle_root == *commitment
            && self.open_point == open_point
            && self.combination == combination
    }

    pub fn verify(
        &self,
        polynomial_proof: &QueryVecsResult<T>,
        folding_proof: &Vec<QueryResult<T>>,
        function_proof: &Vec<QueryResult<T>>,
        evaluation: T,
    ) -> bool {
        let mut leaf_indices = self.oracle.query_list.clone();
        for i in 0..self.total_round {
            let domain_size = self.code.domain_size(i);
            leaf_indices = leaf_indices
                .iter_mut()
                .map(|v| *v % (domain_size >> 1))
                .collect();
            leaf_indices.sort();
            leaf_indices.dedup();

            if i == 0 {
                if !polynomial_proof.verify_merkle_tree(&leaf_indices, &self.initial_proof) {
                    return false;
                }
            } else {
                let Some(function_root) = self.function_root.get(i - 1) else {
                    return false;
                };
                let Some(folding_root) = self.folding_root.get(i - 1) else {
                    return false;
                };
                if !function_proof[i - 1].verify_merkle_tree(&leaf_indices, function_root) {
                    return false;
                }
                if !folding_proof[i - 1].verify_merkle_tree(&leaf_indices, folding_root) {
                    return false;
                }
            }

            let last_challenge = if i == 0 {
                None
            } else {
                Some(self.oracle.folding_challenges[i - 1])
            };
            let cur_challenge = self.oracle.folding_challenges[i];

            // rlc_polynomial, p_0, p_1, ..., p_{\mu-2}
            // p_{\mu - 1} is a constant
            // when i = 0, need to construct rlc polynomial
            let get_folding_value = if i == 0 {
                let values_map = &polynomial_proof.proof_values;
                let mut new_map: HashMap<usize, T> = HashMap::new();
                for j in &leaf_indices {
                    let Some(f_x_values) = values_map.get(j) else {
                        return false;
                    };
                    let Some(f_nx_values) = values_map.get(&(j + domain_size / 2)) else {
                        return false;
                    };

                    let mut f_x_final = f_x_values[0];
                    let mut f_nx_final = f_nx_values[0];
                    if f_x_values.len() != f_nx_values.len() {
                        return false;
                    }
                    for k in 1..f_x_values.len() {
                        f_x_final *= self.oracle.rlc;
                        f_x_final += f_x_values[k];

                        f_nx_final *= self.oracle.rlc;
                        f_nx_final += f_nx_values[k];
                    }
                    new_map.insert(*j, f_x_final);
                    new_map.insert(j + domain_size / 2, f_nx_final);
                }
                new_map
            } else {
                folding_proof[i - 1].proof_values.clone()
            };

            // f_0, f_1, ..., f_{\mu - 1}
            // f_{\mu} is a constant
            // function[0] is the virtual f_0
            let function_values = if i == 0 {
                let values_map = &polynomial_proof.proof_values;
                let mut new_map: HashMap<usize, T> = HashMap::new();
                for j in &leaf_indices {
                    let Some(f_x_values) = values_map.get(j) else {
                        return false;
                    };
                    let Some(f_nx_values) = values_map.get(&(j + domain_size / 2)) else {
                        return false;
                    };

                    let f_x_final = M::initial_function_value(f_x_values, &self.combination);
                    let f_nx_final = M::initial_function_value(f_nx_values, &self.combination);
                    new_map.insert(*j, f_x_final);
                    new_map.insert(j + domain_size / 2, f_nx_final);
                }
                new_map
            } else {
                function_proof[i - 1].proof_values.clone()
            };

            for j in &leaf_indices {
                // verifier folding proofs
                let Some(&f_x) = function_values.get(j) else {
                    return false;
                };
                let Some(&f_nx) = function_values.get(&(j + domain_size / 2)) else {
                    return false;
                };

                if i != 0 {
                    let Some(&p_x) = get_folding_value.get(j) else {
                        return false;
                    };
                    let Some(&p_nx) = get_folding_value.get(&(j + domain_size / 2)) else {
                        return false;
                    };

                    let last_challenge_square = last_challenge.unwrap().pow([2 as u64]);
                    let phi_x = p_x + last_challenge_square * f_x;
                    let phi_nx = p_nx + last_challenge_square * f_nx;

                    let new_v = (phi_x + phi_nx)
                        + cur_challenge * (phi_x - phi_nx) * self.code.fold_weight(i, *j);
                    if i == self.total_round - 1 {
                        let Some(final_value) = self.final_value else {
                            return false;
                        };
                        if new_v != final_value {
                            return false;
                        }
                    } else {
                        let Some(&expected) = folding_proof[i].proof_values.get(j) else {
                            return false;
                        };
                        if new_v != expected {
                            return false;
                        }
                    }
                } else {
                    let Some(&x) = get_folding_value.get(j) else {
                        return false;
                    };
                    let Some(&nx) = get_folding_value.get(&(j + domain_size / 2)) else {
                        return false;
                    };
                    let v = x + nx + cur_challenge * (x - nx) * self.code.fold_weight(i, *j);
                    let Some(&expected) = folding_proof[i].proof_values.get(j) else {
                        return false;
                    };
                    if v != expected {
                        return false;
                    }
                }

                // verify function_proofs
                let v =
                    (f_x + f_nx) + self.open_point[i] * (f_x - f_nx) * self.code.fold_weight(i, *j);
                if i < self.total_round - 1 {
                    if i != 0 {
                        let Some(&expected) = function_proof[i].proof_values.get(j) else {
                            return false;
                        };
                        if v != expected * T::from_u64(2 as u64).unwrap() {
                            return false;
                        }
                    }
                } else {
                    let expected_eval =
                        M::terminal_evaluation(evaluation, self.oracle.rlc, self.mask_evaluation);
                    if v != expected_eval * T::from_u64(2 as u64).unwrap() {
                        return false;
                    }
                    let Some(verifier_eval) = self.evaluation else {
                        return false;
                    };
                    if v != verifier_eval * T::from_u64(2 as u64).unwrap() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
