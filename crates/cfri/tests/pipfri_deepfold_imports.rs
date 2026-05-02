use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use cfri::imported::{deepfold, pip_fri};
use pipfri_utils::{
    fiat_shamir::RandomOracle as PipFriOracle,
    goldilocks::Goldilocks,
    helper::{Helper, MultilinearPolynomial},
    interpolate_vecs_value::{get_poly_num, get_sub_variable_num, get_tensor},
    CODE_RATE, SECURITY_BITS,
};
use rand::{rngs::StdRng, SeedableRng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_rejects_or_panics(verify: impl FnOnce() -> bool) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| !valid).unwrap_or(true));
}

#[test]
fn pipfri_native_open_verify_small() {
    let variable_num = 8;
    let mut rng = StdRng::seed_from_u64(0);
    let polynomial = MultilinearPolynomial::<Goldilocks>::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| Goldilocks::rand(&mut rng))
        .collect::<Vec<_>>();
    let eval = polynomial.evaluate(&point);

    let poly_num = get_poly_num(&polynomial);
    assert!(poly_num > 0);

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

    let oracle = PipFriOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover = pip_fri::prover::Prover::new(
        sub_variable_num,
        &interpolate_cosets,
        polynomial,
        &oracle,
        &tensor,
    );
    let commitment = prover.commit_polynomial();
    let mut verifier = pip_fri::verifier::Verifier::new(
        sub_variable_num,
        commitment,
        &interpolate_cosets,
        &oracle,
        &sub_open_point.to_vec(),
        &tensor,
    );

    let (polynomial_proof, folding_proof, function_proof) =
        prover.open(&sub_open_point.to_vec(), &mut verifier);
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
fn deepfold_native_open_verify_small() {
    let variable_num = 6;
    let step = 1;
    let polynomial = MultilinearPolynomial::<Goldilocks>::rand(variable_num);
    let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (variable_num + CODE_RATE),
        Goldilocks::from(1_u64),
    )
    .unwrap()];
    for i in 1..=variable_num {
        interpolate_cosets.push(Helper::pow(&interpolate_cosets[i - 1], 2));
    }

    let oracle = deepfold::prover::RandomOracle::new(variable_num, SECURITY_BITS / CODE_RATE);
    let prover =
        deepfold::prover::Prover::new(variable_num, &interpolate_cosets, polynomial, &oracle, step);
    let commit = prover.commit_polynomial();
    let verifier =
        deepfold::verifier::Verifier::new(variable_num, &interpolate_cosets, commit, &oracle, step);
    let point = verifier.get_open_point();
    let proof = prover.generate_proof(point);

    assert!(verifier.verify(proof));
}
