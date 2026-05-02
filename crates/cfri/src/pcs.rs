use ark_ff::UniformRand;
use ark_poly::{
    polynomial::{univariate::DensePolynomial as UnivariatePolynomial, Polynomial},
    DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
};
use rand::{rngs::StdRng, SeedableRng};

use crate::pip_fri::util::{
    fiat_shamir::RandomOracle,
    goldilocks::Goldilocks,
    helper::{Helper, MultilinearPolynomial},
    interpolate_vecs_value::{get_sub_variable_num, get_tensor, QueryVecsResult},
    merkle_tree::MERKLE_ROOT_SIZE,
    query_result::QueryResult,
    CODE_RATE, SECURITY_BITS,
};

pub type Field = Goldilocks;
pub type Multilinear = MultilinearPolynomial<Field>;
pub type Univariate = UnivariatePolynomial<Field>;

fn multilinear_cosets(variable_num: usize) -> Vec<GeneralEvaluationDomain<Field>> {
    let mut cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (variable_num + CODE_RATE),
        Field::from(1_u64),
    )
    .unwrap()];
    for i in 1..variable_num {
        cosets.push(Helper::pow(&cosets[i - 1], 2));
    }
    cosets
}

fn pip_fri_cosets(variable_num: usize, seed: u64) -> Vec<GeneralEvaluationDomain<Field>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (variable_num + CODE_RATE),
        Field::rand(&mut rng),
    )
    .unwrap()];
    for i in 1..variable_num {
        cosets.push(Helper::pow(&cosets[i - 1], 2));
    }
    cosets
}

#[derive(Clone)]
pub struct Params {
    pub variable_num: usize,
    pub seed: u64,
}

impl Params {
    pub fn new(variable_num: usize, seed: u64) -> Self {
        Self { variable_num, seed }
    }
}

pub mod fri {
    use super::*;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
        oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
        _cosets: Vec<GeneralEvaluationDomain<Field>>,
        _oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct Commitment(pub [u8; MERKLE_ROOT_SIZE]);

    pub struct ProverState {
        prover: crate::fri::prover::Prover<Field>,
    }

    #[derive(Clone)]
    pub struct Proof {
        proof: Vec<QueryResult<Field>>,
        verifier: crate::fri::verifier::Verifier<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        let cosets = multilinear_cosets(params.variable_num);
        let oracle = RandomOracle::new(params.variable_num, SECURITY_BITS / CODE_RATE);
        (
            ProverKey {
                params: params.clone(),
                cosets: cosets.clone(),
                oracle: oracle.clone(),
            },
            VerifierKey {
                _params: params.clone(),
                _cosets: cosets,
                _oracle: oracle,
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Univariate,
        _point: Field,
    ) -> (Commitment, ProverState) {
        let prover = crate::fri::prover::Prover::new(
            pk.params.variable_num,
            &pk.cosets,
            polynomial,
            &pk.oracle,
        );
        let commitment = Commitment(prover.commit_polynomial());
        (commitment, ProverState { prover })
    }

    pub fn open(
        pk: &ProverKey,
        mut state: ProverState,
        commitment: &Commitment,
        point: Field,
        value: Field,
    ) -> Proof {
        let mut verifier = crate::fri::verifier::Verifier::new(
            pk.params.variable_num,
            &pk.cosets,
            commitment.0,
            &pk.oracle,
            point,
        );
        let proof = state.prover.open(point, value, &mut verifier);
        Proof { proof, verifier }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: Field,
        value: Field,
        proof: &Proof,
    ) -> bool {
        proof.verifier.verify(&proof.proof, value)
    }

    pub fn rand_polynomial(variable_num: usize, seed: u64) -> Univariate {
        let mut rng = StdRng::seed_from_u64(seed);
        Univariate::rand((1 << variable_num) - 1, &mut rng)
    }
}

pub mod pip_fri {
    use super::*;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
    }

    #[derive(Clone)]
    pub struct Commitment(pub [u8; MERKLE_ROOT_SIZE]);

    pub struct ProverState {
        prover: crate::pip_fri::prover::Prover<Field>,
        sub_point: Vec<Field>,
        tensor: Vec<Field>,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
    }

    #[derive(Clone)]
    pub struct Proof {
        polynomial: QueryVecsResult<Field>,
        folding: Vec<QueryResult<Field>>,
        function: Vec<QueryResult<Field>>,
        verifier: crate::pip_fri::verifier::Verifier<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        (
            ProverKey {
                params: params.clone(),
            },
            VerifierKey {
                _params: params.clone(),
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Multilinear,
        point: &[Field],
    ) -> (Commitment, ProverState) {
        let sub_variable_num = get_sub_variable_num(&polynomial);
        assert_eq!(point.len(), pk.params.variable_num);
        let (sub_point, remaining_var) = point.split_at(sub_variable_num);
        let tensor = get_tensor(&remaining_var.to_vec());
        let cosets = pip_fri_cosets(sub_variable_num, pk.params.seed);
        let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
        let mut prover = crate::pip_fri::prover::Prover::new(
            sub_variable_num,
            &cosets,
            polynomial,
            &oracle,
            &tensor,
        );
        let commitment = Commitment(prover.commit_polynomial());
        (
            commitment,
            ProverState {
                prover,
                sub_point: sub_point.to_vec(),
                tensor,
                cosets,
            },
        )
    }

    pub fn open(
        _pk: &ProverKey,
        mut state: ProverState,
        commitment: &Commitment,
        _point: &[Field],
        _value: Field,
    ) -> Proof {
        let oracle = state.prover.oracle.clone();
        let mut verifier = crate::pip_fri::verifier::Verifier::new(
            state.prover.total_round,
            commitment.0,
            &state.cosets,
            &oracle,
            &state.sub_point,
            &state.tensor,
        );
        let (polynomial, folding, function) = state.prover.open(&state.sub_point, &mut verifier);
        Proof {
            polynomial,
            folding,
            function,
            verifier,
        }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: &[Field],
        value: Field,
        proof: &Proof,
    ) -> bool {
        proof
            .verifier
            .verify(&proof.polynomial, &proof.folding, &proof.function, value)
    }
}

pub mod de_pip_fri {
    use super::*;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
    }

    #[derive(Clone)]
    pub struct Commitment(pub [u8; MERKLE_ROOT_SIZE]);

    pub struct ProverState {
        prover: crate::de_pip_fri::prover::Prover<Field>,
        sub_point: Vec<Field>,
        tensor: Vec<Field>,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
    }

    #[derive(Clone)]
    pub struct Proof {
        polynomial: QueryVecsResult<Field>,
        folding: Vec<QueryResult<Field>>,
        function: Vec<QueryResult<Field>>,
        verifier: crate::de_pip_fri::verifier::Verifier<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        (
            ProverKey {
                params: params.clone(),
            },
            VerifierKey {
                _params: params.clone(),
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Multilinear,
        point: &[Field],
    ) -> (Commitment, ProverState) {
        let sub_variable_num = get_sub_variable_num(&polynomial);
        assert_eq!(point.len(), pk.params.variable_num);
        let (sub_point, remaining_var) = point.split_at(sub_variable_num);
        let tensor = get_tensor(&remaining_var.to_vec());
        let cosets = pip_fri_cosets(sub_variable_num, pk.params.seed);
        let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
        let mut prover = crate::de_pip_fri::prover::Prover::new(
            sub_variable_num,
            &cosets,
            polynomial,
            &oracle,
            &tensor,
        );
        let commitment = Commitment(prover.commit_polynomial());
        (
            commitment,
            ProverState {
                prover,
                sub_point: sub_point.to_vec(),
                tensor,
                cosets,
            },
        )
    }

    pub fn open(
        _pk: &ProverKey,
        mut state: ProverState,
        commitment: &Commitment,
        _point: &[Field],
        _value: Field,
    ) -> Proof {
        let oracle = state.prover.oracle.clone();
        let mut verifier = crate::de_pip_fri::verifier::Verifier::new(
            state.prover.total_round,
            commitment.0,
            &state.cosets,
            &oracle,
            &state.sub_point,
            &state.tensor,
        );
        let (polynomial, folding, function) = state.prover.open(&state.sub_point, &mut verifier);
        Proof {
            polynomial,
            folding,
            function,
            verifier,
        }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: &[Field],
        value: Field,
        proof: &Proof,
    ) -> bool {
        proof
            .verifier
            .verify(&proof.polynomial, &proof.folding, &proof.function, value)
    }
}

pub mod polyfrim {
    use super::*;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
        oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
        _cosets: Vec<GeneralEvaluationDomain<Field>>,
        _oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct Commitment(pub [u8; MERKLE_ROOT_SIZE]);

    pub struct ProverState {
        prover: crate::polyfrim::prover::One2ManyProver<Field>,
    }

    #[derive(Clone)]
    pub struct Proof {
        folding: Vec<QueryResult<Field>>,
        function: Vec<QueryResult<Field>>,
        verifier: crate::polyfrim::verifier::One2ManyVerifier<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        let cosets = multilinear_cosets(params.variable_num);
        let oracle = RandomOracle::new(params.variable_num, SECURITY_BITS / CODE_RATE);
        (
            ProverKey {
                params: params.clone(),
                cosets: cosets.clone(),
                oracle: oracle.clone(),
            },
            VerifierKey {
                _params: params.clone(),
                _cosets: cosets,
                _oracle: oracle,
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Multilinear,
        _point: &[Field],
    ) -> (Commitment, ProverState) {
        let prover = crate::polyfrim::prover::One2ManyProver::new(
            pk.params.variable_num,
            &pk.cosets,
            polynomial,
            &pk.oracle,
        );
        let commitment = Commitment(prover.commit_polynomial());
        (commitment, ProverState { prover })
    }

    pub fn open(
        pk: &ProverKey,
        mut state: ProverState,
        commitment: &Commitment,
        point: &[Field],
        _value: Field,
    ) -> Proof {
        let mut verifier = crate::polyfrim::verifier::One2ManyVerifier::new(
            pk.params.variable_num,
            &pk.cosets,
            commitment.0,
            &pk.oracle,
            &point.to_vec(),
        );
        let (folding, function) = state.prover.open(&mut verifier, &point.to_vec());
        Proof {
            folding,
            function,
            verifier,
        }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: &[Field],
        value: Field,
        proof: &Proof,
    ) -> bool {
        proof
            .verifier
            .verify(&proof.folding, &proof.function, value)
    }
}

pub mod virgo {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
        vector_coset: GeneralEvaluationDomain<Field>,
        oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
        _cosets: Vec<GeneralEvaluationDomain<Field>>,
        _vector_coset: GeneralEvaluationDomain<Field>,
        _oracle: RandomOracle<Field>,
    }

    #[derive(Clone)]
    pub struct Commitment(pub [u8; MERKLE_ROOT_SIZE]);

    pub struct ProverState {
        prover: crate::virgo::prover::FriProver<Field>,
    }

    #[derive(Clone)]
    pub struct Proof {
        folding: Vec<QueryResult<Field>>,
        function: Vec<QueryResult<Field>>,
        v_values: HashMap<usize, Field>,
        verifier: crate::virgo::verifier::FriVerifier<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        let cosets = multilinear_cosets(params.variable_num);
        let vector_coset =
            GeneralEvaluationDomain::new_coset(1 << params.variable_num, Field::from(1_u64))
                .unwrap();
        let oracle = RandomOracle::new(params.variable_num, SECURITY_BITS / CODE_RATE);
        (
            ProverKey {
                params: params.clone(),
                cosets: cosets.clone(),
                vector_coset: vector_coset.clone(),
                oracle: oracle.clone(),
            },
            VerifierKey {
                _params: params.clone(),
                _cosets: cosets,
                _vector_coset: vector_coset,
                _oracle: oracle,
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Multilinear,
        _point: &[Field],
    ) -> (Commitment, ProverState) {
        let prover = crate::virgo::prover::FriProver::new(
            pk.params.variable_num,
            &pk.cosets,
            &pk.vector_coset,
            polynomial,
            &pk.oracle,
        );
        let commitment = Commitment(prover.commit_first_polynomial());
        (commitment, ProverState { prover })
    }

    pub fn open(
        pk: &ProverKey,
        mut state: ProverState,
        commitment: &Commitment,
        point: &[Field],
        _value: Field,
    ) -> Proof {
        let mut verifier = crate::virgo::verifier::FriVerifier::new(
            pk.params.variable_num,
            &pk.cosets,
            &pk.vector_coset,
            commitment.0,
            &pk.oracle,
        );
        let point = point.to_vec();
        verifier.get_open_point(&point);
        state.prover.commit_functions(&mut verifier, &point);
        state.prover.prove();
        state.prover.commit_foldings(&mut verifier);
        let (folding, function, v_values) = state.prover.query();
        Proof {
            folding,
            function,
            v_values,
            verifier,
        }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: &[Field],
        value: Field,
        proof: &Proof,
    ) -> bool {
        proof
            .verifier
            .verify(value, &proof.folding, &proof.v_values, &proof.function)
    }
}

pub mod deepfold {
    use super::*;

    #[derive(Clone)]
    pub struct ProverKey {
        params: Params,
        cosets: Vec<GeneralEvaluationDomain<Field>>,
        oracle: crate::deepfold::prover::RandomOracle<Field>,
        step: usize,
    }

    #[derive(Clone)]
    pub struct VerifierKey {
        _params: Params,
        _cosets: Vec<GeneralEvaluationDomain<Field>>,
        _oracle: crate::deepfold::prover::RandomOracle<Field>,
        _step: usize,
    }

    #[derive(Clone)]
    pub struct Commitment(pub crate::deepfold::Commit<Field>);

    pub struct ProverState {
        prover: crate::deepfold::prover::Prover<Field>,
    }

    #[derive(Clone)]
    pub struct Proof {
        proof: crate::deepfold::Proof<Field>,
        verifier: crate::deepfold::verifier::Verifier<Field>,
        point: Vec<Field>,
    }

    pub fn setup(variable_num: usize, seed: u64) -> Params {
        Params::new(variable_num, seed)
    }

    pub fn trim(params: &Params) -> (ProverKey, VerifierKey) {
        let mut cosets = vec![GeneralEvaluationDomain::new_coset(
            1 << (params.variable_num + CODE_RATE),
            Field::from(1_u64),
        )
        .unwrap()];
        for i in 1..=params.variable_num {
            cosets.push(Helper::pow(&cosets[i - 1], 2));
        }
        let oracle = crate::deepfold::prover::RandomOracle::new(
            params.variable_num,
            SECURITY_BITS / CODE_RATE,
        );
        let step = 1;
        (
            ProverKey {
                params: params.clone(),
                cosets: cosets.clone(),
                oracle: oracle.clone(),
                step,
            },
            VerifierKey {
                _params: params.clone(),
                _cosets: cosets,
                _oracle: oracle,
                _step: step,
            },
        )
    }

    pub fn commit(
        pk: &ProverKey,
        polynomial: Multilinear,
        _point: &[Field],
    ) -> (Commitment, ProverState) {
        let prover = crate::deepfold::prover::Prover::new(
            pk.params.variable_num,
            &pk.cosets,
            polynomial,
            &pk.oracle,
            pk.step,
        );
        let commitment = Commitment(prover.commit_polynomial());
        (commitment, ProverState { prover })
    }

    pub fn open(
        pk: &ProverKey,
        state: ProverState,
        commitment: &Commitment,
        point: &[Field],
        _value: Field,
    ) -> Proof {
        let mut verifier = crate::deepfold::verifier::Verifier::new(
            pk.params.variable_num,
            &pk.cosets,
            commitment.0.clone(),
            &pk.oracle,
            pk.step,
        );
        let point = if point.is_empty() {
            verifier.get_open_point()
        } else {
            verifier.set_open_point(&point.to_vec());
            point.to_vec()
        };
        let proof = state.prover.generate_proof(point.clone());
        Proof {
            proof,
            verifier,
            point,
        }
    }

    pub fn verify(
        _vk: &VerifierKey,
        _commitment: &Commitment,
        _point: &[Field],
        _value: Field,
        proof: &Proof,
    ) -> bool {
        proof.verifier.clone().verify(proof.proof.clone())
    }

    pub fn proof_point(proof: &Proof) -> &[Field] {
        &proof.point
    }
}

pub fn random_multilinear(variable_num: usize) -> Multilinear {
    Multilinear::rand(variable_num)
}

pub fn random_multilinear_point(variable_num: usize, seed: u64) -> Vec<Field> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..variable_num).map(|_| Field::rand(&mut rng)).collect()
}

pub fn evaluate_multilinear(polynomial: &Multilinear, point: &[Field]) -> Field {
    polynomial.evaluate(&point.to_vec())
}

pub fn evaluate_univariate(polynomial: &Univariate, point: Field) -> Field {
    polynomial.evaluate(&point)
}
