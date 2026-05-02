use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use cfri::pcs;
use cfri::pip_fri::{
    prover::Prover,
    util::{
        fiat_shamir::RandomOracle,
        foldable_code::MultiplicativeFftCode,
        helper::{Helper, MultilinearPolynomial},
        interpolate_vecs_value::{get_sub_variable_num, get_tensor},
        CODE_RATE, SECURITY_BITS,
    },
    verifier::Verifier,
    zkprover::ZKProver,
    zkverifier::ZKVerifier,
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
    let polynomial = pcs::random_multilinear(variable_num);
    let point = pcs::random_multilinear_point(variable_num, 0);
    let value = pcs::evaluate_multilinear(&polynomial, &point);

    let params = pcs::pip_fri::setup(variable_num, 0);
    let (pk, vk) = pcs::pip_fri::trim(&params);
    let (commitment, state) = pcs::pip_fri::commit(&pk, polynomial, &point);
    let proof = pcs::pip_fri::open(&pk, state, &commitment, &point, value);

    assert!(pcs::pip_fri::verify(
        &vk,
        &commitment,
        &point,
        value,
        &proof
    ));
    assert_rejects_or_panics(|| {
        pcs::pip_fri::verify(
            &vk,
            &commitment,
            &point,
            value + pcs::Field::from(1_u64),
            &proof,
        )
    });
}

#[test]
fn pipfri_explicit_foldable_code_open_verify_small() {
    let variable_num = 8;
    let mut rng = StdRng::seed_from_u64(3);
    let polynomial = MultilinearPolynomial::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| pcs::Field::rand(&mut rng))
        .collect::<Vec<_>>();
    let value = polynomial.evaluate(&point);
    let sub_variable_num = get_sub_variable_num(&polynomial);
    let (sub_point, remaining_var) = point.split_at(sub_variable_num);
    let tensor = get_tensor(&remaining_var.to_vec());

    let mut cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (sub_variable_num + CODE_RATE),
        pcs::Field::rand(&mut rng),
    )
    .unwrap()];
    for round in 1..sub_variable_num {
        cosets.push(Helper::pow(&cosets[round - 1], 2));
    }

    let code = MultiplicativeFftCode::new(&cosets);
    let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
    let mut prover =
        Prover::new_with_code(sub_variable_num, code.clone(), polynomial, &oracle, &tensor);
    let commitment = prover.commit_polynomial();
    let mut verifier = Verifier::new_with_code(
        sub_variable_num,
        commitment,
        code,
        &oracle,
        &sub_point.to_vec(),
        &tensor,
    );
    let (polynomial_proof, folding_proof, function_proof) =
        prover.open(&sub_point.to_vec(), &mut verifier);

    assert!(verifier.verify(&polynomial_proof, &folding_proof, &function_proof, value));
}

#[test]
fn zk_pipfri_explicit_foldable_code_open_verify_small() {
    let variable_num = 8;
    let mut rng = StdRng::seed_from_u64(4);
    let polynomial = MultilinearPolynomial::rand(variable_num);
    let point = (0..variable_num)
        .map(|_| pcs::Field::rand(&mut rng))
        .collect::<Vec<_>>();
    let value = polynomial.evaluate(&point);
    let sub_variable_num = get_sub_variable_num(&polynomial);
    let (sub_point, remaining_var) = point.split_at(sub_variable_num);
    let mut sub_point = sub_point.to_vec();
    sub_point.push(pcs::Field::from(0_u64));
    let tensor = get_tensor(&remaining_var.to_vec());

    let total_round = sub_variable_num + 1;
    let mut cosets = vec![GeneralEvaluationDomain::new_coset(
        1 << (total_round + CODE_RATE),
        pcs::Field::rand(&mut rng),
    )
    .unwrap()];
    for round in 1..total_round {
        cosets.push(Helper::pow(&cosets[round - 1], 2));
    }

    let code = MultiplicativeFftCode::new(&cosets);
    let oracle = RandomOracle::new(total_round, SECURITY_BITS / CODE_RATE);
    let mut prover =
        ZKProver::new_with_code(total_round, code.clone(), polynomial, &oracle, &tensor);
    let commitment = prover.commit_polynomial();
    let mut verifier =
        ZKVerifier::new_with_code(total_round, commitment, code, &oracle, &sub_point, &tensor);
    let (polynomial_proof, folding_proof, function_proof) = prover.open(&sub_point, &mut verifier);

    assert!(verifier.verify(&polynomial_proof, &folding_proof, &function_proof, value));
}

#[test]
fn deepfold_native_open_verify_small() {
    let variable_num = 6;
    let polynomial = pcs::random_multilinear(variable_num);
    let point = pcs::random_multilinear_point(variable_num, 1);
    let value = pcs::evaluate_multilinear(&polynomial, &point);

    let params = pcs::deepfold::setup(variable_num, 1);
    let (pk, vk) = pcs::deepfold::trim(&params);
    let (commitment, state) = pcs::deepfold::commit(&pk, polynomial, &point);
    let proof = pcs::deepfold::open(&pk, state, &commitment, &point, value);

    assert!(pcs::deepfold::verify(
        &vk,
        &commitment,
        pcs::deepfold::proof_point(&proof),
        value,
        &proof
    ));
}
