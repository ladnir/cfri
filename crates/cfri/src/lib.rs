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
        interpolate_vecs_value::{get_poly_num, get_sub_variable_num, get_tensor, QueryVecsResult},
        merkle_tree::MERKLE_ROOT_SIZE,
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

        #[derive(Clone)]
        pub struct Params<F: PrimeField> {
            num_vars: usize,
            cosets: Vec<GeneralEvaluationDomain<F>>,
            oracle: ::deepfold::prover::RandomOracle<F>,
            config: PcsConfig,
        }

        pub struct Commitment<F: PrimeField>(::deepfold::Commit<F>);

        pub struct ProverData<F: PrimeField> {
            prover: ::deepfold::prover::Prover<F>,
        }

        pub struct Proof<F: PrimeField> {
            proof: ::deepfold::Proof<F>,
        }

        pub fn setup<F: PrimeField>(num_vars: usize, config: PcsConfig) -> Params<F> {
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
            Params {
                num_vars,
                cosets,
                oracle,
                config,
            }
        }

        pub fn commit<F: PrimeField>(
            params: &Params<F>,
            poly: MlPoly<F>,
        ) -> (Commitment<F>, ProverData<F>) {
            assert_eq!(params.num_vars, poly.num_vars());
            let prover = ::deepfold::prover::Prover::new(
                params.num_vars,
                &params.cosets,
                poly.as_pipfri_poly(),
                &params.oracle,
                params.config.step,
            );
            let commit = prover.commit_polynomial();
            (Commitment(commit), ProverData { prover })
        }

        pub fn open<F: PrimeField>(
            _params: &Params<F>,
            data: ProverData<F>,
            point: &[F],
        ) -> Proof<F> {
            let proof = data.prover.generate_proof(point.to_vec());
            Proof { proof }
        }

        pub fn verify<F: PrimeField>(
            params: &Params<F>,
            commitment: Commitment<F>,
            point: &[F],
            proof: Proof<F>,
        ) -> bool {
            let mut verifier = ::deepfold::verifier::Verifier::new(
                params.num_vars,
                &params.cosets,
                commitment.0,
                &params.oracle,
                params.config.step,
            );
            verifier.set_open_point(&point.to_vec());
            verifier.verify(proof.proof)
        }

        pub fn prove<F: PrimeField>(poly: &MlPoly<F>, point: &[F], config: &PcsConfig) -> Proof<F> {
            let params = setup(poly.num_vars(), config.clone());
            let (_commitment, data) = commit(&params, poly.clone());
            open(&params, data, point)
        }

        pub fn prove_and_verify<F: PrimeField>(
            poly: MlPoly<F>,
            point: &[F],
            config: &PcsConfig,
        ) -> bool {
            let params = setup(poly.num_vars(), config.clone());
            let (commitment, data) = commit(&params, poly);
            let proof = open(&params, data, point);
            verify(&params, commitment, point, proof)
        }
    }

    pub mod pipfri {
        use super::*;

        #[derive(Clone)]
        pub struct Params<F: PrimeField> {
            sub_num_vars: usize,
            cosets: Vec<GeneralEvaluationDomain<F>>,
            oracle: PipFriOracle<F>,
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Commitment {
            root: [u8; MERKLE_ROOT_SIZE],
        }

        pub struct ProverData<F: PrimeField> {
            poly: MlPoly<F>,
        }

        pub struct Proof<F: PrimeField> {
            verifier: ::pip_fri::verifier::Verifier<F>,
            polynomial_proof: QueryVecsResult<F>,
            folding_proof: Vec<QueryResult<F>>,
            function_proof: Vec<QueryResult<F>>,
            eval: F,
        }

        pub fn setup<F: PrimeField>(num_vars: usize, config: PcsConfig) -> Params<F> {
            let poly_len = 1 << num_vars;
            let poly_num = pipfri_utils::helper::nearest_power_of_two(num_vars * 4);
            assert!(poly_num <= poly_len);
            let sub_num_vars = (poly_len / poly_num).ilog2() as usize;

            let mut cosets = vec![GeneralEvaluationDomain::new_coset(
                1 << (sub_num_vars + config.rate_log),
                F::from(1_u64),
            )
            .unwrap()];
            for i in 1..sub_num_vars {
                cosets.push(Helper::pow(&cosets[i - 1], 2));
            }

            let oracle = PipFriOracle::new(sub_num_vars, config.security_bits / config.rate_log);
            Params {
                sub_num_vars,
                cosets,
                oracle,
            }
        }

        pub fn commit<F: PrimeField>(
            params: &Params<F>,
            poly: MlPoly<F>,
        ) -> (Commitment, ProverData<F>) {
            let polynomial = poly.as_pipfri_poly();
            assert_eq!(params.sub_num_vars, get_sub_variable_num(&polynomial));

            let tensor = vec![F::ZERO; get_poly_num(&polynomial)];
            let mut prover = ::pip_fri::prover::Prover::new(
                params.sub_num_vars,
                &params.cosets,
                polynomial,
                &params.oracle,
                &tensor,
            );
            let root = prover.commit_polynomial();
            (Commitment { root }, ProverData { poly })
        }

        pub fn open<F: PrimeField>(
            params: &Params<F>,
            data: ProverData<F>,
            commitment: &Commitment,
            point: &[F],
        ) -> Proof<F> {
            assert_eq!(
                data.poly.num_vars(),
                params.sub_num_vars + get_poly_num(&data.poly.as_pipfri_poly()).ilog2() as usize
            );
            assert_eq!(data.poly.num_vars(), point.len());

            let polynomial = data.poly.as_pipfri_poly();
            let eval = polynomial.evaluate(&point.to_vec());
            let (sub_open_point, remaining_var) = point.split_at(params.sub_num_vars);
            let tensor = get_tensor(&remaining_var.to_vec());

            let mut prover = ::pip_fri::prover::Prover::new(
                params.sub_num_vars,
                &params.cosets,
                polynomial,
                &params.oracle,
                &tensor,
            );
            assert_eq!(commitment.root, prover.commit_polynomial());
            let mut verifier = ::pip_fri::verifier::Verifier::new(
                params.sub_num_vars,
                commitment.root,
                &params.cosets,
                &params.oracle,
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

        pub fn prove<F: PrimeField>(poly: &MlPoly<F>, point: &[F], config: &PcsConfig) -> Proof<F> {
            let params = setup(poly.num_vars(), config.clone());
            let (commitment, data) = commit(&params, poly.clone());
            open(&params, data, &commitment, point)
        }
    }
}
