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
    let params = deepfold::setup(poly.num_vars(), PcsConfig::default());
    let (commitment, data) = deepfold::commit(&params, poly);
    let proof = deepfold::open(&params, data, &point);

    assert!(deepfold::verify(&params, commitment, &point, proof));
}

#[test]
fn pipfri_flat_shape_open_verify_small() {
    let poly = poly(8);
    let point = point(poly.num_vars());
    let params = pipfri::setup(poly.num_vars(), PcsConfig::default());
    let (commitment, data) = pipfri::commit(&params, poly);
    let proof = pipfri::open(&params, data, &commitment, &point);

    assert!(pipfri::verify(&proof));
}
