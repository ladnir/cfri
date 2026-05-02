use ark_ff::UniformRand;
use cfri::pcs;
use rand::{rngs::StdRng, SeedableRng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_rejects_or_panics(verify: impl FnOnce() -> bool) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| !valid).unwrap_or(true));
}

#[test]
fn fri_native_open_verify_small() {
    let variable_num = 4;
    let polynomial = pcs::fri::rand_polynomial(variable_num, 11);
    let mut rng = StdRng::seed_from_u64(11);
    let point = pcs::Field::rand(&mut rng);
    let value = pcs::evaluate_univariate(&polynomial, point);

    let params = pcs::fri::setup(variable_num, 11);
    let (pk, vk) = pcs::fri::trim(&params);
    let (commitment, state) = pcs::fri::commit(&pk, polynomial, point);
    let proof = pcs::fri::open(&pk, state, &commitment, point, value);

    assert!(pcs::fri::verify(&vk, &commitment, point, value, &proof));
    assert_rejects_or_panics(|| {
        pcs::fri::verify(
            &vk,
            &commitment,
            point,
            value + pcs::Field::from(1_u64),
            &proof,
        )
    });
}

#[test]
fn polyfrim_native_open_verify_small() {
    let variable_num = 4;
    let polynomial = pcs::random_multilinear(variable_num);
    let point = pcs::random_multilinear_point(variable_num, 12);
    let value = pcs::evaluate_multilinear(&polynomial, &point);

    let params = pcs::polyfrim::setup(variable_num, 12);
    let (pk, vk) = pcs::polyfrim::trim(&params);
    let (commitment, state) = pcs::polyfrim::commit(&pk, polynomial, &point);
    let proof = pcs::polyfrim::open(&pk, state, &commitment, &point, value);

    assert!(pcs::polyfrim::verify(
        &vk,
        &commitment,
        &point,
        value,
        &proof
    ));
    assert_rejects_or_panics(|| {
        pcs::polyfrim::verify(
            &vk,
            &commitment,
            &point,
            value + pcs::Field::from(1_u64),
            &proof,
        )
    });
}

#[test]
fn de_pip_fri_native_open_verify_small() {
    let variable_num = 8;
    let polynomial = pcs::random_multilinear(variable_num);
    let point = pcs::random_multilinear_point(variable_num, 13);
    let value = pcs::evaluate_multilinear(&polynomial, &point);

    let params = pcs::de_pip_fri::setup(variable_num, 13);
    let (pk, vk) = pcs::de_pip_fri::trim(&params);
    let (commitment, state) = pcs::de_pip_fri::commit(&pk, polynomial, &point);
    let proof = pcs::de_pip_fri::open(&pk, state, &commitment, &point, value);

    assert!(pcs::de_pip_fri::verify(
        &vk,
        &commitment,
        &point,
        value,
        &proof
    ));
    assert_rejects_or_panics(|| {
        pcs::de_pip_fri::verify(
            &vk,
            &commitment,
            &point,
            value + pcs::Field::from(1_u64),
            &proof,
        )
    });
}

#[test]
fn virgo_native_open_verify_small() {
    let variable_num = 4;
    let polynomial = pcs::random_multilinear(variable_num);
    let point = pcs::random_multilinear_point(variable_num, 14);
    let value = pcs::evaluate_multilinear(&polynomial, &point);

    let params = pcs::virgo::setup(variable_num, 14);
    let (pk, vk) = pcs::virgo::trim(&params);
    let (commitment, state) = pcs::virgo::commit(&pk, polynomial, &point);
    let proof = pcs::virgo::open(&pk, state, &commitment, &point, value);

    assert!(pcs::virgo::verify(&vk, &commitment, &point, value, &proof));
    assert_rejects_or_panics(|| {
        pcs::virgo::verify(
            &vk,
            &commitment,
            &point,
            value + pcs::Field::from(1_u64),
            &proof,
        )
    });
}
