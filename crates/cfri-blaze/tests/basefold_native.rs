use blake2::Blake2s256;
use cfri_blaze::imported::{Basefold, BasefoldExtParams, MultilinearPolynomial};
use plonkish_backend::{
    halo2_curves::bn256::Fr,
    pcs::PolynomialCommitmentScheme,
    util::transcript::{
        Blake2sTranscript, FieldTranscript, FieldTranscriptRead, FieldTranscriptWrite,
        InMemoryTranscript,
    },
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

#[derive(Debug)]
struct SmallRandomCode;

impl BasefoldExtParams for SmallRandomCode {
    fn get_reps() -> usize {
        5
    }

    fn get_rate() -> usize {
        1
    }

    fn get_basecode_rounds() -> usize {
        0
    }

    fn get_rs_basecode() -> bool {
        false
    }

    fn get_code_type() -> String {
        "random".to_string()
    }
}

type Pcs = Basefold<Fr, Blake2s256, SmallRandomCode>;

#[test]
fn basefold_native_commit_open_verify_small() {
    let num_vars = 3;
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([7; 32]);
    let param = Pcs::setup(poly_size, 1, &mut rng).unwrap();
    let (pp, vp) = Pcs::trim(&param, poly_size, 1).unwrap();

    let poly = MultilinearPolynomial::rand(num_vars, &mut rng);
    let proof = {
        let mut transcript = Blake2sTranscript::new(());
        let comm = Pcs::commit_and_write(&pp, &poly, &mut transcript).unwrap();
        let point = transcript.squeeze_challenges(num_vars);
        let eval = poly.evaluate(&point);
        transcript.write_field_element(&eval).unwrap();
        Pcs::open(&pp, &poly, &comm, &point, &eval, &mut transcript).unwrap();
        transcript.into_proof()
    };

    let result = {
        let mut transcript = Blake2sTranscript::from_proof((), proof.as_slice());
        let comm = Pcs::read_commitment(&vp, &mut transcript).unwrap();
        let point = transcript.squeeze_challenges(num_vars);
        let eval = transcript.read_field_element().unwrap();
        Pcs::verify(&vp, &comm, &point, &eval, &mut transcript)
    };
    assert!(result.is_ok());
}
