use cfri::mlpcs::{deepfold, pipfri, MlPoly, PcsConfig};
use pipfri_utils::goldilocks::Goldilocks;

fn poly(num_vars: usize) -> MlPoly<Goldilocks> {
    MlPoly::from_coefficients(
        (0..(1 << num_vars))
            .map(|i| Goldilocks::from((17 * i + 9) as u64))
            .collect(),
    )
}

fn point(num_vars: usize) -> Vec<Goldilocks> {
    (0..num_vars)
        .map(|i| Goldilocks::from((3 * i + 5) as u64))
        .collect()
}

#[test]
fn deepfold_flat_shape_open_verify_small() {
    let poly = poly(6);
    let point = point(poly.num_vars());
    let proof = deepfold::prove(&poly, &point, &PcsConfig::default());

    assert!(deepfold::verify(proof));
}

#[test]
fn pipfri_flat_shape_open_verify_small() {
    let poly = poly(8);
    let point = point(poly.num_vars());
    let proof = pipfri::prove(&poly, &point, &PcsConfig::default());

    assert!(pipfri::verify(&proof));
}
