use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use blake2::Blake2s256;
use cfri::backend::{
    blaze_transcript::BlazeBlake2sTranscript,
    halo2_curves::bn256::Fr,
    hash::Blake2s,
    pcs::PolynomialCommitmentScheme,
    transcript::{
        Blake2sTranscript, FieldTranscript, FieldTranscriptRead, FieldTranscriptWrite,
        InMemoryTranscript,
    },
};
use cfri::blaze::MultilinearPolynomial;
use cfri::blaze::{blaze, Basefold, BasefoldExtParams, Blazeu64, HidingBasefold, B128};
use cfri::pcs::Field as PipFriField;
use cfri::pip_fri::{
    prover::Prover as PipFriProver,
    util::{
        fiat_shamir::RandomOracle as PipFriRandomOracle,
        helper::{Helper as PipFriHelper, MultilinearPolynomial as PipFriMultilinearPolynomial},
        interpolate_vecs_value::{
            get_sub_variable_num as pipfri_get_sub_variable_num, get_tensor as pipfri_get_tensor,
        },
        merkle_tree::MERKLE_ROOT_SIZE as PIPFRI_MERKLE_ROOT_SIZE,
        CODE_RATE as PIPFRI_CODE_RATE, SECURITY_BITS as PIPFRI_SECURITY_BITS,
    },
    verifier::Verifier as PipFriVerifier,
};
use num_traits::Zero;
use rand::{rngs::StdRng, SeedableRng as _};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::{
    env,
    hint::black_box,
    mem::size_of,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct PerfRandomCode;

impl BasefoldExtParams for PerfRandomCode {
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

type BasefoldPcs = Basefold<Fr, Blake2s256, PerfRandomCode>;
type HidingBasefoldPcs = HidingBasefold<Fr, Blake2s256, PerfRandomCode>;

#[derive(Clone, Copy)]
struct Timed<T> {
    value: T,
    elapsed: Duration,
}

fn timed<T>(f: impl FnOnce() -> T) -> Timed<T> {
    let start = Instant::now();
    let value = f();
    Timed {
        value,
        elapsed: start.elapsed(),
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn print_perf_line(
    scheme: &str,
    mode: &str,
    params: &str,
    setup: Duration,
    trim: Duration,
    commit: Duration,
    eval: Duration,
    open: Duration,
    verify: Duration,
    proof_bytes: usize,
) {
    println!(
        "PERF|scheme={scheme}|mode={mode}|{params}|setup_ms={:.3}|trim_ms={:.3}|commit_ms={:.3}|eval_ms={:.3}|open_ms={:.3}|verify_ms={:.3}|proof_bytes={proof_bytes}",
        ms(setup),
        ms(trim),
        ms(commit),
        ms(eval),
        ms(open),
        ms(verify),
    );
}

fn run_basefold_transparent(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([21; 32]);
    let setup = timed(|| BasefoldPcs::setup(poly_size, 1, &mut rng).unwrap());
    let trim = timed(|| BasefoldPcs::trim(&setup.value, poly_size, 1).unwrap());
    let (pp, vp) = trim.value;
    let poly = MultilinearPolynomial::rand(num_vars, &mut rng);

    let mut transcript = Blake2sTranscript::new(());
    let commit = timed(|| BasefoldPcs::commit_and_write(&pp, &poly, &mut transcript).unwrap());
    black_box(&commit.value);
    let point = transcript.squeeze_challenges(num_vars);
    let eval = timed(|| poly.evaluate(&point));
    transcript.write_field_element(&eval.value).unwrap();
    let open = timed(|| {
        BasefoldPcs::open(
            &pp,
            &poly,
            &commit.value,
            &point,
            &eval.value,
            &mut transcript,
        )
        .unwrap()
    });
    black_box(open.value);
    let proof = transcript.into_proof();

    let verify = timed(|| {
        let mut transcript = Blake2sTranscript::from_proof((), proof.as_slice());
        let comm = BasefoldPcs::read_commitment(&vp, &mut transcript).unwrap();
        let point = transcript.squeeze_challenges(num_vars);
        let eval = transcript.read_field_element().unwrap();
        BasefoldPcs::verify(&vp, &comm, &point, &eval, &mut transcript).unwrap();
    });

    print_perf_line(
        "basefold",
        "transparent",
        &format!("num_vars={num_vars}|poly_len={poly_size}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        eval.elapsed,
        open.elapsed,
        verify.elapsed,
        proof.len(),
    );
}

fn run_basefold_hiding(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([22; 32]);
    let setup = timed(|| HidingBasefoldPcs::setup(poly_size, 1, &mut rng).unwrap());
    let trim = timed(|| HidingBasefoldPcs::trim(&setup.value, poly_size, 1).unwrap());
    let (pp, vp) = trim.value;
    let poly = MultilinearPolynomial::rand(num_vars, &mut rng);

    let mut transcript = Blake2sTranscript::new(());
    let commit =
        timed(|| HidingBasefoldPcs::commit_and_write(&pp, &poly, &mut transcript).unwrap());
    black_box(&commit.value);
    let point = transcript.squeeze_challenges(num_vars);
    let eval = timed(|| poly.evaluate(&point));
    transcript.write_field_element(&eval.value).unwrap();
    let open = timed(|| {
        HidingBasefoldPcs::open(
            &pp,
            &poly,
            &commit.value,
            &point,
            &eval.value,
            &mut transcript,
        )
        .unwrap()
    });
    black_box(open.value);
    let proof = transcript.into_proof();

    let verify = timed(|| {
        let mut transcript = Blake2sTranscript::from_proof((), proof.as_slice());
        let comm = HidingBasefoldPcs::read_commitment(&vp, &mut transcript).unwrap();
        let point = transcript.squeeze_challenges(num_vars);
        let eval = transcript.read_field_element().unwrap();
        HidingBasefoldPcs::verify(&vp, &comm, &point, &eval, &mut transcript).unwrap();
    });

    print_perf_line(
        "basefold",
        "hiding",
        &format!("num_vars={num_vars}|poly_len={poly_size}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        eval.elapsed,
        open.elapsed,
        verify.elapsed,
        proof.len(),
    );
}

fn blaze_input(num_rows: usize, poly_size: usize) -> Vec<Vec<Blazeu64>> {
    (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (0x9e37_79b9u64)
                        .wrapping_mul((row as u64) + 1)
                        .wrapping_add((0xd1b5_4a32u64).wrapping_mul((col as u64) + 3)),
                })
                .collect()
        })
        .collect()
}

fn run_blaze_transparent(num_vars: usize, num_rows: usize, num_queries: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([31; 32]);
    let setup = timed(|| {
        blaze::setup::<Blake2s>(poly_size, 1, &mut rng, Some(num_rows), Some(num_queries))
    });
    let trim = timed(|| blaze::trim::<Blake2s>(&setup.value, poly_size, 1));
    let (pp, vp) = trim.value;
    let data = blaze_input(num_rows, poly_size);

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let commit =
        timed(|| blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript));
    black_box(&commit.value);
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = b128_transcript.squeeze_challenges((num_rows >> 1).ilog2() as usize + num_vars);
    let eval_value = blaze::evaluate_commitment(&commit.value, &point).unwrap();
    let eval = timed(|| {
        blaze::open(
            &pp,
            &data,
            &commit.value,
            &point,
            &eval_value,
            &mut blaze_transcript,
            &mut b128_transcript,
        )
        .unwrap()
    });
    let open = eval.elapsed;
    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    let verify = timed(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        blaze::verify(
            &vp,
            &commit.value,
            &point,
            &eval.value,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
        .unwrap();
    });

    print_perf_line(
        "blaze",
        "transparent",
        &format!("num_vars={num_vars}|poly_len={poly_size}|rows={num_rows}|queries={num_queries}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        Duration::ZERO,
        open,
        verify.elapsed,
        blaze_proof.len() + b128_proof.len(),
    );
}

fn run_blaze_hiding(num_vars: usize, num_rows: usize, num_queries: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([32; 32]);
    let setup = timed(|| {
        blaze::setup_with_hiding::<Blake2s>(
            poly_size,
            1,
            &mut rng,
            Some(num_rows),
            Some(num_queries),
        )
    });
    let trim = timed(|| blaze::trim_with_hiding::<Blake2s>(&setup.value, poly_size, 1));
    let (pp, vp) = trim.value;
    let data = blaze_input(num_rows, poly_size);

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let commit = timed(|| {
        blaze::commit_and_write_with_hiding::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript)
    });
    black_box(&commit.value);
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = b128_transcript.squeeze_challenges((num_rows >> 1).ilog2() as usize + num_vars);
    let hidden_point = {
        let mut hidden_point = point.clone();
        hidden_point.push(B128::zero());
        hidden_point
    };
    let eval_value = blaze::evaluate_commitment(&commit.value, &hidden_point).unwrap();
    let eval = timed(|| {
        blaze::open_with_hiding(
            &pp,
            &data,
            &commit.value,
            &point,
            &eval_value,
            &mut blaze_transcript,
            &mut b128_transcript,
        )
        .unwrap()
    });
    let open = eval.elapsed;
    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    let verify = timed(|| {
        let mut blaze_transcript = BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        blaze::verify_with_hiding(
            &vp,
            &commit.value,
            &point,
            &eval.value,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
        .unwrap();
    });

    print_perf_line(
        "blaze",
        "hiding",
        &format!("num_vars={num_vars}|poly_len={poly_size}|rows={num_rows}|queries={num_queries}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        Duration::ZERO,
        open,
        verify.elapsed,
        blaze_proof.len() + b128_proof.len(),
    );
}

fn pipfri_polynomial(num_vars: usize, seed: u8) -> PipFriMultilinearPolynomial<PipFriField> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let coefficients = (0..(1usize << num_vars))
        .map(|_| PipFriField::rand(&mut rng))
        .collect();
    PipFriMultilinearPolynomial::new(coefficients)
}

fn pipfri_point(num_vars: usize, seed: u8) -> Vec<PipFriField> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    (0..num_vars).map(|_| PipFriField::rand(&mut rng)).collect()
}

fn run_pipfri_transparent(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let polynomial = pipfri_polynomial(num_vars, 41);
    let point = pipfri_point(num_vars, 42);
    let eval = timed(|| polynomial.evaluate(&point));

    let sub_variable_num = pipfri_get_sub_variable_num(&polynomial);
    let (sub_open_point, remaining_var) = point.split_at(sub_variable_num);
    let sub_open_point = sub_open_point.to_vec();
    let tensor = pipfri_get_tensor(&remaining_var.to_vec());

    let setup = timed(|| {
        let mut rng = StdRng::seed_from_u64(43);
        let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
            1 << (sub_variable_num + PIPFRI_CODE_RATE),
            PipFriField::rand(&mut rng),
        )
        .unwrap()];
        for i in 1..sub_variable_num {
            interpolate_cosets.push(PipFriHelper::pow(&interpolate_cosets[i - 1], 2));
        }
        let oracle =
            PipFriRandomOracle::new(sub_variable_num, PIPFRI_SECURITY_BITS / PIPFRI_CODE_RATE);
        (interpolate_cosets, oracle)
    });
    let (interpolate_cosets, oracle) = setup.value;

    let commit = timed(|| {
        let mut prover = PipFriProver::new(
            sub_variable_num,
            &interpolate_cosets,
            polynomial,
            &oracle,
            &tensor,
        );
        let commitment = prover.commit_polynomial();
        (prover, commitment)
    });
    let (mut prover, commitment) = commit.value;
    black_box(&commitment);

    let open = timed(|| {
        let mut verifier = PipFriVerifier::new(
            sub_variable_num,
            commitment,
            &interpolate_cosets,
            &oracle,
            &sub_open_point,
            &tensor,
        );
        let proofs = prover.open(&sub_open_point, &mut verifier);
        (verifier, proofs)
    });
    let (verifier, (polynomial_proof, folding_proof, function_proof)) = open.value;

    let verify = timed(|| {
        assert!(verifier.verify(
            &polynomial_proof,
            &folding_proof,
            &function_proof,
            eval.value,
        ));
    });

    let proof_bytes = folding_proof.iter().map(|x| x.proof_size()).sum::<usize>()
        + polynomial_proof.proof_size()
        + function_proof.iter().map(|x| x.proof_size()).sum::<usize>()
        + (2 * sub_variable_num).saturating_sub(3) * PIPFRI_MERKLE_ROOT_SIZE
        + 2 * size_of::<PipFriField>();

    print_perf_line(
        "pipfri",
        "transparent",
        &format!("num_vars={num_vars}|poly_len={poly_size}|sub_vars={sub_variable_num}"),
        setup.elapsed,
        Duration::ZERO,
        commit.elapsed,
        eval.elapsed,
        open.elapsed,
        verify.elapsed,
        proof_bytes,
    );
}

#[test]
#[ignore = "release-only performance baseline; run scripts/perf-baseline.ps1"]
fn current_release_performance_baseline() {
    println!(
        "PERF|kind=metadata|crate=cfri|profile=release|test=current_release_performance_baseline"
    );
    run_basefold_transparent(10);
    run_basefold_hiding(10);
    run_blaze_transparent(8, 64, 16);
    run_blaze_hiding(8, 64, 16);
}

#[test]
#[ignore = "release-only transparent performance baseline; run scripts/perf-ab.ps1"]
fn current_release_transparent_performance_baseline() {
    println!(
        "PERF|kind=metadata|crate=cfri|profile=release|test=current_release_transparent_performance_baseline"
    );
    run_basefold_transparent(10);
    run_blaze_transparent(8, 64, 16);
}

#[test]
#[ignore = "release-only transparent PiPFRI A/B baseline; run scripts/perf-pipfri-ab.ps1"]
fn current_release_pipfri_transparent_performance_baseline() {
    println!(
        "PERF|kind=metadata|crate=cfri|profile=release|test=current_release_pipfri_transparent_performance_baseline"
    );
    let num_vars = env::var("CFRI_PIPFRI_NUM_VARS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8);
    run_pipfri_transparent(num_vars);
}
