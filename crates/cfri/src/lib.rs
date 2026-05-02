pub mod imported {
    pub use deepfold;
    pub use pip_fri;
}

pub mod mlpcs {
    use ark_ff::PrimeField;
    use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
    use pipfri_utils::{
        fiat_shamir::RandomOracle as PipFriOracle,
        helper::{Helper, MultilinearPolynomial},
        interpolate_vecs_value::{get_sub_variable_num, get_tensor, QueryVecsResult},
        query_result::QueryResult,
        CODE_RATE, SECURITY_BITS,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct MlPoly<F: PrimeField> {
        coefficients: Vec<F>,
    }

    impl<F: PrimeField> MlPoly<F> {
        pub fn from_coefficients(coefficients: Vec<F>) -> Self {
            assert!(coefficients.len().is_power_of_two());
            Self { coefficients }
        }

        pub fn coefficients(&self) -> &[F] {
            &self.coefficients
        }

        pub fn num_vars(&self) -> usize {
            self.coefficients.len().ilog2() as usize
        }

        pub fn evaluate(&self, point: &[F]) -> F {
            self.as_pipfri_poly().evaluate(&point.to_vec())
        }

        fn as_pipfri_poly(&self) -> MultilinearPolynomial<F> {
            MultilinearPolynomial::new(self.coefficients.clone())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PcsConfig {
        pub security_bits: usize,
        pub rate_log: usize,
        pub step: usize,
    }

    impl Default for PcsConfig {
        fn default() -> Self {
            Self {
                security_bits: SECURITY_BITS,
                rate_log: CODE_RATE,
                step: 1,
            }
        }
    }

    pub mod deepfold {
        use super::*;

        pub struct Proof<F: PrimeField> {
            verifier: ::deepfold::verifier::Verifier<F>,
            proof: ::deepfold::Proof<F>,
        }

        pub fn prove<F: PrimeField>(poly: &MlPoly<F>, point: &[F], config: &PcsConfig) -> Proof<F> {
            assert_eq!(poly.num_vars(), point.len());
            let num_vars = poly.num_vars();
            let mut cosets = vec![GeneralEvaluationDomain::new_coset(
                1 << (num_vars + config.rate_log),
                F::from(1_u64),
            )
            .unwrap()];
            for i in 1..=num_vars {
                cosets.push(Helper::pow(&cosets[i - 1], 2));
            }

            let oracle = ::deepfold::prover::RandomOracle::new(
                num_vars,
                config.security_bits / config.rate_log,
            );
            let prover = ::deepfold::prover::Prover::new(
                num_vars,
                &cosets,
                poly.as_pipfri_poly(),
                &oracle,
                config.step,
            );
            let commit = prover.commit_polynomial();
            let mut verifier = ::deepfold::verifier::Verifier::new(
                num_vars,
                &cosets,
                commit,
                &oracle,
                config.step,
            );
            verifier.set_open_point(&point.to_vec());
            let proof = prover.generate_proof(point.to_vec());
            Proof { verifier, proof }
        }

        pub fn verify<F: PrimeField>(proof: Proof<F>) -> bool {
            proof.verifier.verify(proof.proof)
        }
    }

    pub mod pipfri {
        use super::*;

        pub struct Proof<F: PrimeField> {
            verifier: ::pip_fri::verifier::Verifier<F>,
            polynomial_proof: QueryVecsResult<F>,
            folding_proof: Vec<QueryResult<F>>,
            function_proof: Vec<QueryResult<F>>,
            eval: F,
        }

        pub fn prove<F: PrimeField>(poly: &MlPoly<F>, point: &[F], config: &PcsConfig) -> Proof<F> {
            assert_eq!(poly.num_vars(), point.len());
            let polynomial = poly.as_pipfri_poly();
            let eval = polynomial.evaluate(&point.to_vec());
            let sub_num_vars = get_sub_variable_num(&polynomial);
            let (sub_open_point, remaining_var) = point.split_at(sub_num_vars);
            let tensor = get_tensor(&remaining_var.to_vec());

            let mut cosets = vec![GeneralEvaluationDomain::new_coset(
                1 << (sub_num_vars + config.rate_log),
                F::from(1_u64),
            )
            .unwrap()];
            for i in 1..sub_num_vars {
                cosets.push(Helper::pow(&cosets[i - 1], 2));
            }

            let oracle = PipFriOracle::new(sub_num_vars, config.security_bits / config.rate_log);
            let mut prover =
                ::pip_fri::prover::Prover::new(sub_num_vars, &cosets, polynomial, &oracle, &tensor);
            let commitment = prover.commit_polynomial();
            let mut verifier = ::pip_fri::verifier::Verifier::new(
                sub_num_vars,
                commitment,
                &cosets,
                &oracle,
                &sub_open_point.to_vec(),
                &tensor,
            );
            let (polynomial_proof, folding_proof, function_proof) =
                prover.open(&sub_open_point.to_vec(), &mut verifier);

            Proof {
                verifier,
                polynomial_proof,
                folding_proof,
                function_proof,
                eval,
            }
        }

        pub fn verify<F: PrimeField>(proof: &Proof<F>) -> bool {
            proof.verifier.verify(
                &proof.polynomial_proof,
                &proof.folding_proof,
                &proof.function_proof,
                proof.eval,
            )
        }
    }
}
