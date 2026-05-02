use cfri::blaze::{blaze, Blazeu64, B128};
use cfri::plonkish_backend::util::{
    blaze_transcript::BlazeBlake2sTranscript,
    hash::Blake2s,
    transcript::{Blake2sTranscript, FieldTranscript, InMemoryTranscript},
};
use num_traits::Zero;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_rejects_or_panics(
    verify: impl FnOnce() -> Result<(), cfri::plonkish_backend::Error>,
) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| valid.is_err()).unwrap_or(true));
}

fn small_blaze_proof() -> (
    blaze::BlazeVerifierParam,
    blaze::BlazeCommitment<Blazeu64, Blake2s>,
    Vec<B128>,
    B128,
    Vec<u8>,
    Vec<u8>,
) {
    let num_vars = 2;
    let num_rows = 2;
    let num_queries = 2;
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([9; 32]);
    let params = blaze::setup::<Blake2s>(poly_size, 1, &mut rng, Some(num_rows), Some(num_queries));
    let (pp, vp) = blaze::trim::<Blake2s>(&params, poly_size, 1);
    let data = (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (17 * row + 31 * col + 5) as u64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let comm = blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript);
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = b128_transcript.squeeze_challenges(num_vars);
    let eval = blaze::open(
        &pp,
        &data,
        &comm,
        &point,
        &B128::zero(),
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    (vp, comm, point, eval, blaze_proof, b128_proof)
}

#[test]
fn blaze_native_commit_open_verify_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_blaze_proof();
    let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
    let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());

    assert_eq!(
        blaze::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        ),
        Ok(())
    );
}

#[test]
fn blaze_native_rejects_wrong_eval_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_blaze_proof();
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        let wrong_eval = eval + B128::from(123u64);
        blaze::verify(
            &vp,
            &comm,
            &point,
            &wrong_eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}

#[test]
fn blaze_native_rejects_wrong_point_small() {
    let (vp, comm, mut point, eval, blaze_proof, b128_proof) = small_blaze_proof();
    point[0] += B128::from(1u64);
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        blaze::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}
