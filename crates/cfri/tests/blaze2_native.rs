use cfri::backend::{
    blaze_transcript::BlazeBlake2sTranscript,
    hash::Blake2s,
    transcript::{FieldTranscript as _, InMemoryTranscript as _},
};
use cfri::blaze::{blaze2, Blazeu64, B128};
use cfri::transcript::Blake2sTranscript;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn rows(num_rows: usize, row_len: usize) -> Vec<Vec<Blazeu64>> {
    (0..num_rows)
        .map(|row| {
            (0..row_len)
                .map(|col| Blazeu64 {
                    value: (0x9e37_79b9u64)
                        .wrapping_mul((row as u64) + 1)
                        .wrapping_add((0xd1b5_4a32u64).wrapping_mul((col as u64) + 3)),
                })
                .collect()
        })
        .collect()
}

fn small_proof(
    num_vars: usize,
    num_rows: usize,
    num_queries: usize,
) -> (
    blaze2::Blaze2VerifierParam,
    blaze2::Blaze2Commitment<Blazeu64, Blake2s>,
    Vec<B128>,
    B128,
    Vec<u8>,
    Vec<u8>,
) {
    let row_len = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([77; 32]);
    let params = blaze2::setup::<Blake2s>(row_len, 1, &mut rng, Some(num_rows), Some(num_queries));
    let (pp, vp) = blaze2::trim::<Blake2s>(&params, row_len, 1);
    let rows = rows(num_rows, row_len);

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let comm =
        blaze2::commit_and_write::<Blazeu64, Blake2s>(&pp, &rows, &mut blaze_transcript).unwrap();
    let mut b128_transcript = Blake2sTranscript::new();
    let point = b128_transcript.squeeze_challenges(num_vars + (num_rows >> 1).ilog2() as usize);
    let eval = blaze2::evaluate_commitment(&comm, &point).unwrap();
    let eval = blaze2::open(
        &pp,
        &rows,
        &comm,
        &point,
        &eval,
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    (
        vp,
        comm.public(),
        point,
        eval,
        blaze_transcript.into_proof(),
        b128_transcript.into_proof(),
    )
}

fn assert_rejects_or_panics(verify: impl FnOnce() -> Result<(), cfri::backend::Error>) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| valid.is_err()).unwrap_or(true));
}

#[test]
fn blaze2_commit_open_verify_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_proof(3, 4, 4);
    let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
    let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());

    assert_eq!(
        blaze2::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript
        ),
        Ok(())
    );
}

#[test]
fn blaze2_rejects_wrong_eval_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_proof(3, 4, 4);
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
        blaze2::verify(
            &vp,
            &comm,
            &point,
            &(eval + B128::from(1)),
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}

#[test]
fn blaze2_rejects_wrong_point_small() {
    let (vp, comm, mut point, eval, blaze_proof, b128_proof) = small_proof(3, 4, 4);
    point[0] += B128::from(1);
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
        blaze2::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}

#[test]
fn blaze2_rejects_tampered_b128_proof_small() {
    let (vp, comm, point, eval, blaze_proof, mut b128_proof) = small_proof(3, 4, 4);
    let idx = b128_proof.len() / 2;
    b128_proof[idx] ^= 1;
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
        blaze2::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}

#[test]
fn blaze2_rejects_tampered_blaze_proof_small() {
    let (vp, comm, point, eval, mut blaze_proof, b128_proof) = small_proof(3, 4, 4);
    let idx = blaze_proof.len() / 2;
    blaze_proof[idx] ^= 1;
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
        blaze2::verify(
            &vp,
            &comm,
            &point,
            &eval,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
}

#[test]
fn blaze2_small_benchmark_shape_stays_sub_100kb() {
    let (_vp, _comm, _point, _eval, blaze_proof, b128_proof) = small_proof(8, 64, 16);
    let proof_bytes = blaze_proof.len() + b128_proof.len();
    assert!(
        proof_bytes < 100_000,
        "Blaze2 proof shape regressed to {proof_bytes} bytes"
    );
}
