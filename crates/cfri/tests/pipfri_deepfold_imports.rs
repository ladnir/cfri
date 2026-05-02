use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use cfri::pip_fri::util as owned_util;
use cfri::{imported::deepfold, pip_fri};
use pipfri_utils::{
    goldilocks::Goldilocks as UpstreamGoldilocks,
    helper::{Helper as UpstreamHelper, MultilinearPolynomial as UpstreamMultilinearPolynomial},
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
    let polynomial =
        owned_util::helper::MultilinearPolynomial::<owned_util::goldilocks::Goldilocks>::rand(
            variable_num,
        );
    let point = (0..variable_num)
        .map(|_| owned_util::goldilocks::Goldilocks::rand(&mut rng))
        .collect::<Vec<_>>();
    let eval = polynomial.evaluate(&point);

    let poly_num = owned_util::interpolate_vecs_value::get_poly_num(&polynomial);
    assert!(poly_num > 0);

    let sub_variable_num = owned_util::interpolate_vecs_value::get_sub_variable_num(&polynomial);
    let (sub_open_point, remaining_var) = point.split_at(sub_variable_num);
    let tensor = owned_util::interpolate_vecs_value::get_tensor(&remaining_var.to_vec());

    let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (sub_variable_num + owned_util::CODE_RATE),
        owned_util::goldilocks::Goldilocks::rand(&mut rng),
    )
    .unwrap()];
    for i in 1..sub_variable_num {
        interpolate_cosets.push(owned_util::helper::Helper::pow(
            &interpolate_cosets[i - 1],
            2,
        ));
    }

    let oracle = owned_util::fiat_shamir::RandomOracle::new(
        sub_variable_num,
        owned_util::SECURITY_BITS / owned_util::CODE_RATE,
    );
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
            eval + owned_util::goldilocks::Goldilocks::from(1_u64),
        )
    });
}

#[test]
fn deepfold_native_open_verify_small() {
    let variable_num = 6;
    let step = 1;
    let polynomial = UpstreamMultilinearPolynomial::<UpstreamGoldilocks>::rand(variable_num);
    let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (variable_num + pipfri_utils::CODE_RATE),
        UpstreamGoldilocks::from(1_u64),
    )
    .unwrap()];
    for i in 1..=variable_num {
        interpolate_cosets.push(UpstreamHelper::pow(&interpolate_cosets[i - 1], 2));
    }

    let oracle = deepfold::prover::RandomOracle::new(
        variable_num,
        pipfri_utils::SECURITY_BITS / pipfri_utils::CODE_RATE,
    );
    let prover =
        deepfold::prover::Prover::new(variable_num, &interpolate_cosets, polynomial, &oracle, step);
    let commit = prover.commit_polynomial();
    let verifier =
        deepfold::verifier::Verifier::new(variable_num, &interpolate_cosets, commit, &oracle, step);
    let point = verifier.get_open_point();
    let proof = prover.generate_proof(point);

    assert!(verifier.verify(proof));
}
