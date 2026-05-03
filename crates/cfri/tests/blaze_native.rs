use cfri::backend::{
    blaze_transcript::BlazeBlake2sTranscript,
    hash::{Blake2s, Output},
    transcript::{
        FieldTranscript, FieldTranscriptWrite, InMemoryTranscript, Transcript, TranscriptWrite,
    },
    Error,
};
use cfri::blaze::{blaze, Blazeu64, B128};
use cfri::transcript::Blake2sTranscript;
use num_traits::Zero;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_rejects_or_panics(verify: impl FnOnce() -> Result<(), cfri::backend::Error>) {
    let result = catch_unwind(AssertUnwindSafe(verify));
    assert!(result.map(|valid| valid.is_err()).unwrap_or(true));
}

struct CorruptingB128Transcript {
    inner: Blake2sTranscript,
    target_field_write: usize,
    field_writes: usize,
}

impl CorruptingB128Transcript {
    fn new(target_field_write: usize) -> Self {
        Self {
            inner: Blake2sTranscript::new(),
            target_field_write,
            field_writes: 0,
        }
    }

    fn into_proof(self) -> Vec<u8> {
        self.inner.into_proof()
    }
}

impl FieldTranscript<B128> for CorruptingB128Transcript {
    fn squeeze_challenge(&mut self) -> B128 {
        self.inner.squeeze_challenge()
    }

    fn common_field_element(&mut self, fe: &B128) -> Result<(), Error> {
        self.inner.common_field_element(fe)
    }
}

impl FieldTranscriptWrite<B128> for CorruptingB128Transcript {
    fn write_field_element(&mut self, fe: &B128) -> Result<(), Error> {
        let mut value = *fe;
        if self.field_writes == self.target_field_write {
            value += B128::from(1);
        }
        self.field_writes += 1;
        self.inner.write_field_element(&value)
    }
}

impl Transcript<Output<Blake2s>, B128> for CorruptingB128Transcript {
    fn common_commitment(&mut self, comm: &Output<Blake2s>) -> Result<(), Error> {
        <Blake2sTranscript as Transcript<Output<Blake2s>, B128>>::common_commitment(
            &mut self.inner,
            comm,
        )
    }
}

impl TranscriptWrite<Output<Blake2s>, B128> for CorruptingB128Transcript {
    fn write_commitment(&mut self, comm: &Output<Blake2s>) -> Result<(), Error> {
        <Blake2sTranscript as TranscriptWrite<Output<Blake2s>, B128>>::write_commitment(
            &mut self.inner,
            comm,
        )
    }
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
    let mut b128_transcript = Blake2sTranscript::new();
    let point = b128_transcript.squeeze_challenges(num_vars);
    let eval = blaze::evaluate_commitment(&comm, &point).unwrap();
    let eval = blaze::open(
        &pp,
        &data,
        &comm,
        &point,
        &eval,
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    (vp, comm.public(), point, eval, blaze_proof, b128_proof)
}

fn small_blaze_proof_with_corrupted_b128_write(
    target_field_write: usize,
) -> (
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
    let mut b128_transcript = CorruptingB128Transcript::new(target_field_write);
    let point = b128_transcript.squeeze_challenges(num_vars);
    let eval = blaze::evaluate_commitment(&comm, &point).unwrap();
    let eval = blaze::open(
        &pp,
        &data,
        &comm,
        &point,
        &eval,
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    (vp, comm.public(), point, eval, blaze_proof, b128_proof)
}

fn small_hiding_blaze_proof() -> (
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
    let mut rng = ChaCha8Rng::from_seed([13; 32]);
    let params = blaze::setup_with_hiding::<Blake2s>(
        poly_size,
        1,
        &mut rng,
        Some(num_rows),
        Some(num_queries),
    );
    let (pp, vp) = blaze::trim_with_hiding::<Blake2s>(&params, poly_size, 1);
    let data = (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (29 * row + 37 * col + 11) as u64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let comm =
        blaze::commit_and_write_with_hiding::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript);
    let mut b128_transcript = Blake2sTranscript::new();
    let point = b128_transcript.squeeze_challenges(num_vars);
    let hidden_point = {
        let mut hidden_point = point.clone();
        hidden_point.push(B128::zero());
        hidden_point
    };
    let eval = blaze::evaluate_commitment(&comm, &hidden_point).unwrap();
    let eval = blaze::open_with_hiding(
        &pp,
        &data,
        &comm,
        &point,
        &eval,
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    (vp, comm.public(), point, eval, blaze_proof, b128_proof)
}

#[test]
fn blaze_native_commit_open_verify_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_blaze_proof();
    let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
    let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());

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
fn blaze_native_commit_open_verify_four_rows_small() {
    let num_vars = 2;
    let num_rows = 4;
    let num_queries = 2;
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([21; 32]);
    let params = blaze::setup::<Blake2s>(poly_size, 1, &mut rng, Some(num_rows), Some(num_queries));
    let (pp, vp) = blaze::trim::<Blake2s>(&params, poly_size, 1);
    let data = (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (43 * row + 19 * col + 7) as u64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let comm = blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript);
    let mut b128_transcript = Blake2sTranscript::new();
    let point = b128_transcript.squeeze_challenges(num_vars + 1);
    let eval = blaze::evaluate_commitment(&comm, &point).unwrap();
    let eval = blaze::open(
        &pp,
        &data,
        &comm,
        &point,
        &eval,
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();
    let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
    let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());

    assert_eq!(
        blaze::verify(
            &vp,
            &comm.public(),
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
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
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
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
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

#[test]
fn blaze_native_rejects_tampered_b128_proof_small() {
    let (vp, comm, point, eval, blaze_proof, mut b128_proof) = small_blaze_proof();
    let tamper_at = b128_proof.len() / 2;
    b128_proof[tamper_at] ^= 1;

    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
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

#[test]
fn blaze_native_rejects_corrupted_product_sumcheck_oracle_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) =
        small_blaze_proof_with_corrupted_b128_write(2);

    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
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

#[test]
fn blaze_native_rejects_corrupted_product_terminal_claim_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) =
        small_blaze_proof_with_corrupted_b128_write(18);

    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
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

#[test]
fn hiding_blaze_native_commit_open_verify_small() {
    let (vp, comm, point, eval, blaze_proof, b128_proof) = small_hiding_blaze_proof();
    let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
    let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());

    assert_eq!(
        blaze::verify_with_hiding(
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
fn hiding_blaze_native_rejects_wrong_point_small() {
    let (vp, comm, mut point, eval, blaze_proof, b128_proof) = small_hiding_blaze_proof();
    point[0] += B128::from(1u64);
    assert_rejects_or_panics(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof(b128_proof.as_slice());
        blaze::verify_with_hiding(
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
fn blaze_native_commitment_binds_second_row_small() {
    let num_vars = 2;
    let num_rows = 2;
    let num_queries = 2;
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([41; 32]);
    let params = blaze::setup::<Blake2s>(poly_size, 1, &mut rng, Some(num_rows), Some(num_queries));
    let (pp, _vp) = blaze::trim::<Blake2s>(&params, poly_size, 1);
    let data_a = (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (17 * row + 31 * col + 5) as u64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut data_b = data_a.clone();
    data_b[1][0].value ^= 1;

    let mut transcript_a = BlazeBlake2sTranscript::new(());
    blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data_a, &mut transcript_a);
    let mut transcript_b = BlazeBlake2sTranscript::new(());
    blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data_b, &mut transcript_b);

    assert_ne!(transcript_a.into_proof(), transcript_b.into_proof());
}
