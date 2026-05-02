use cfri::pcs;
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
