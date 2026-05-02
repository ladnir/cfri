use ark_ff::UniformRand;
use ark_poly::{
    polynomial::{univariate::DensePolynomial as UnivariatePolynomial, Polynomial},
    DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
};
use cfri::imported::{de_pip_fri, fri, polyfrim, virgo};
use pipfri_utils::{
    fiat_shamir::RandomOracle,
    goldilocks::Goldilocks,
    helper::{Helper, MultilinearPolynomial},
    interpolate_vecs_value::{get_sub_variable_num, get_tensor},
    CODE_RATE, SECURITY_BITS,
};
use rand::{rngs::StdRng, SeedableRng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_rejects_or_panics(verify: impl FnOnce() -> bool) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| !valid).unwrap_or(true));
}

fn multilinear_cosets(variable_num: usize) -> Vec<GeneralEvaluationDomain<Goldilocks>> {
    let mut cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (variable_num + CODE_RATE),
        Goldilocks::from(1_u64),
    )
    .unwrap()];
    for i in 1..variable_num {
        cosets.push(Helper::pow(&cosets[i - 1], 2));
    }
    cosets
}

#[test]
fn fri_native_open_verify_small() {
    let variable_num = 4;
    let degree = (1 << variable_num) - 1;
    let mut rng = StdRng::seed_from_u64(11);
    let polynomial = UnivariatePolynomial::rand(degree, &mut rng);
    let point = Goldilocks::rand(&mut rng);
    let eval = polynomial.evaluate(&point);
    let interpolate_cosets = multilinear_cosets(variable_num);
    let oracle = RandomOracle::new(variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover =
        fri::prover::Prover::new(variable_num, &interpolate_cosets, polynomial, &oracle);
    let commitment = prover.commit_polynomial();
    let mut verifier = fri::verifier::Verifier::new(
        variable_num,
        &interpolate_cosets,
        commitment,
        &oracle,
        point,
    );

    let proof = prover.open(point, eval, &mut verifier);

    assert!(!proof.is_empty());
    assert!(verifier.verify(&proof, eval));
    assert_rejects_or_panics(|| verifier.verify(&proof, eval + Goldilocks::from(1_u64)));
}

#[test]
fn polyfrim_native_open_verify_small() {
    let variable_num = 4;
    let mut rng = StdRng::seed_from_u64(12);
    let polynomial = MultilinearPolynomial::<Goldilocks>::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| Goldilocks::rand(&mut rng))
        .collect::<Vec<_>>();
    let eval = polynomial.evaluate(&point);
    let interpolate_cosets = multilinear_cosets(variable_num);
    let oracle = RandomOracle::new(variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover = polyfrim::prover::One2ManyProver::new(
        variable_num,
        &interpolate_cosets,
        polynomial,
        &oracle,
    );
    let commitment = prover.commit_polynomial();
    let mut verifier = polyfrim::verifier::One2ManyVerifier::new(
        variable_num,
        &interpolate_cosets,
        commitment,
        &oracle,
        &point,
    );

    let (folding_proof, function_proof) = prover.open(&mut verifier, &point);

    assert!(!folding_proof.is_empty());
    assert!(!function_proof.is_empty());
    assert!(verifier.verify(&folding_proof, &function_proof, eval));
    assert_rejects_or_panics(|| {
        verifier.verify(
            &folding_proof,
            &function_proof,
            eval + Goldilocks::from(1_u64),
        )
    });
}

#[test]
fn de_pip_fri_native_open_verify_small() {
    let variable_num = 8;
    let mut rng = StdRng::seed_from_u64(13);
    let polynomial = MultilinearPolynomial::<Goldilocks>::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| Goldilocks::rand(&mut rng))
        .collect::<Vec<_>>();
    let eval = polynomial.evaluate(&point);
    let sub_variable_num = get_sub_variable_num(&polynomial);
    let (sub_open_point, remaining_var) = point.split_at(sub_variable_num);
    let tensor = get_tensor(&remaining_var.to_vec());
    let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (sub_variable_num + CODE_RATE),
        Goldilocks::rand(&mut rng),
    )
    .unwrap()];
    for i in 1..sub_variable_num {
        interpolate_cosets.push(Helper::pow(&interpolate_cosets[i - 1], 2));
    }
    let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover = de_pip_fri::prover::Prover::new(
        sub_variable_num,
        &interpolate_cosets,
        polynomial,
        &oracle,
        &tensor,
    );
    let commitment = prover.commit_polynomial();
    let mut verifier = de_pip_fri::verifier::Verifier::new(
        sub_variable_num,
        commitment,
        &interpolate_cosets,
        &oracle,
        &sub_open_point.to_vec(),
        &tensor,
    );

    let (polynomial_proof, folding_proof, function_proof) =
        prover.open(&sub_open_point.to_vec(), &mut verifier);

    assert!(!folding_proof.is_empty());
    assert!(!function_proof.is_empty());
    assert!(verifier.verify(&polynomial_proof, &folding_proof, &function_proof, eval));
    assert_rejects_or_panics(|| {
        verifier.verify(
            &polynomial_proof,
            &folding_proof,
            &function_proof,
            eval + Goldilocks::from(1_u64),
        )
    });
}

#[test]
fn virgo_native_open_verify_small() {
    let variable_num = 4;
    let mut rng = StdRng::seed_from_u64(14);
    let polynomial = MultilinearPolynomial::<Goldilocks>::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| Goldilocks::rand(&mut rng))
        .collect::<Vec<_>>();
    let eval = polynomial.evaluate(&point);
    let interpolate_cosets = multilinear_cosets(variable_num);
    let vector_interpolation_coset =
        GeneralEvaluationDomain::new_coset(1 << variable_num, Goldilocks::from(1_u64)).unwrap();
    let oracle = RandomOracle::new(variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover = virgo::prover::FriProver::new(
        variable_num,
        &interpolate_cosets,
        &vector_interpolation_coset,
        polynomial,
        &oracle,
    );
    let commitment = prover.commit_first_polynomial();
    let mut verifier = virgo::verifier::FriVerifier::new(
        variable_num,
        &interpolate_cosets,
        &vector_interpolation_coset,
        commitment,
        &oracle,
    );

    verifier.get_open_point(&point);
    prover.commit_functions(&mut verifier, &point);
    prover.prove();
    prover.commit_foldings(&mut verifier);
    let (folding_proofs, function_proofs, v_values) = prover.query();

    assert!(!folding_proofs.is_empty());
    assert!(!function_proofs.is_empty());
    assert!(verifier.verify(eval, &folding_proofs, &v_values, &function_proofs));
    assert_rejects_or_panics(|| {
        verifier.verify(
            eval + Goldilocks::from(1_u64),
            &folding_proofs,
            &v_values,
            &function_proofs,
        )
    });
}
