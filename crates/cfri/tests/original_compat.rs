use blake2::Blake2s256;
use cfri::backend::{
    basefold::{BasefoldCommitment, BasefoldProverParams, BasefoldVerifierParams},
    blaze_transcript::BlazeBlake2sTranscript,
    halo2_curves::bn256::Fr,
    hash::Blake2s,
    pcs::PolynomialCommitmentScheme,
    transcript::{
        Blake2sTranscript, FieldTranscript, FieldTranscriptRead, FieldTranscriptWrite,
        InMemoryTranscript,
    },
};
use cfri::blaze::{blaze, Basefold, BasefoldExtParams, Blazeu64, MultilinearPolynomial};

#[derive(Debug)]
struct OriginalSmallRandomCode;

impl BasefoldExtParams for OriginalSmallRandomCode {
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

type BasefoldPcs = Basefold<Fr, Blake2s256, OriginalSmallRandomCode>;
type BasefoldFixture = (
    BasefoldProverParams<Fr>,
    BasefoldVerifierParams<Fr>,
    MultilinearPolynomial<Fr>,
    BasefoldCommitment<Fr, Blake2s256>,
    Vec<Fr>,
    Fr,
    Vec<u8>,
);

type BlazeCommitFixture = (
    blaze::BlazeProverParam,
    blaze::BlazeVerifierParam,
    Vec<Vec<Blazeu64>>,
    blaze::BlazeCommitment<Blazeu64, Blake2s>,
    Vec<u8>,
);

fn original_basefold_fixture() -> BasefoldFixture {
    bincode::deserialize(include_bytes!("fixtures/original_basefold_small.bin")).unwrap()
}

fn original_blaze_commit_fixture() -> BlazeCommitFixture {
    bincode::deserialize(include_bytes!("fixtures/original_blaze_commit_small.bin")).unwrap()
}

#[test]
fn basefold_matches_original_small_fixture() {
    let (pp, vp, poly, original_comm, original_point, original_eval, original_proof) =
        original_basefold_fixture();

    let current_comm = BasefoldPcs::commit(&pp, &poly).unwrap();
    assert_eq!(current_comm, original_comm);

    let mut transcript = Blake2sTranscript::new(());
    let written_comm = BasefoldPcs::commit_and_write(&pp, &poly, &mut transcript).unwrap();
    assert_eq!(written_comm, original_comm);
    let current_point = transcript.squeeze_challenges(original_point.len());
    assert_eq!(current_point, original_point);
    let current_eval = poly.evaluate(&current_point);
    assert_eq!(current_eval, original_eval);
    transcript.write_field_element(&current_eval).unwrap();
    BasefoldPcs::open(
        &pp,
        &poly,
        &written_comm,
        &current_point,
        &current_eval,
        &mut transcript,
    )
    .unwrap();
    assert_eq!(transcript.into_proof(), original_proof);

    let mut transcript = Blake2sTranscript::from_proof((), original_proof.as_slice());
    let read_comm = BasefoldPcs::read_commitment(&vp, &mut transcript).unwrap();
    let read_root: &[_] = read_comm.as_ref();
    let original_root: &[_] = original_comm.as_ref();
    assert_eq!(read_root, original_root);
    let proof_point = transcript.squeeze_challenges(original_point.len());
    assert_eq!(proof_point, original_point);
    let proof_eval = transcript.read_field_element().unwrap();
    assert_eq!(proof_eval, original_eval);
    BasefoldPcs::verify(&vp, &read_comm, &proof_point, &proof_eval, &mut transcript).unwrap();
}

#[test]
fn blaze_commit_matches_original_small_fixture() {
    let (pp, _vp, data, original_comm, original_commit_transcript) =
        original_blaze_commit_fixture();

    let mut transcript = BlazeBlake2sTranscript::new(());
    let current_comm = blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data, &mut transcript);

    assert_eq!(
        bincode::serialize(&current_comm).unwrap(),
        bincode::serialize(&original_comm).unwrap()
    );
    assert_eq!(transcript.into_proof(), original_commit_transcript);
}
