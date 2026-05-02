use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use cfri::pcs::Field as T;
use cfri::pip_fri::{
    prover::Prover,
    util::{
        fiat_shamir::RandomOracle,
        helper::{Helper, MultilinearPolynomial},
        interpolate_vecs_value::{get_sub_variable_num, get_tensor},
        merkle_tree::MERKLE_ROOT_SIZE,
        CODE_RATE, SECURITY_BITS,
    },
    verifier::Verifier,
};
use rand::{rngs::StdRng, SeedableRng};
use std::{
    hint::black_box,
    mem::size_of,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
struct Timed<TValue> {
    value: TValue,
    elapsed: Duration,
}

fn timed<TValue>(f: impl FnOnce() -> TValue) -> Timed<TValue> {
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

fn polynomial(num_vars: usize, seed: u8) -> MultilinearPolynomial<T> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let coefficients = (0..(1usize << num_vars))
        .map(|_| T::rand(&mut rng))
        .collect();
    MultilinearPolynomial::new(coefficients)
}

fn point(num_vars: usize, seed: u8) -> Vec<T> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    (0..num_vars).map(|_| T::rand(&mut rng)).collect()
}

fn print_perf_line(
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
        "PERF|impl=current|scheme=pipfri|mode=transparent|{params}|setup_ms={:.3}|trim_ms={:.3}|commit_ms={:.3}|eval_ms={:.3}|open_ms={:.3}|verify_ms={:.3}|proof_bytes={proof_bytes}",
        ms(setup),
        ms(trim),
        ms(commit),
        ms(eval),
        ms(open),
        ms(verify),
    );
}

fn run_pipfri(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let polynomial = polynomial(num_vars, 41);
    let point = point(num_vars, 42);
    let eval = timed(|| polynomial.evaluate(&point));

    let sub_variable_num = get_sub_variable_num(&polynomial);
    let (sub_open_point, remaining_var) = point.split_at(sub_variable_num);
    let sub_open_point = sub_open_point.to_vec();
    let tensor = get_tensor(&remaining_var.to_vec());

    let setup = timed(|| {
        let mut rng = StdRng::seed_from_u64(43);
        let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
            1 << (sub_variable_num + CODE_RATE),
            T::rand(&mut rng),
        )
        .unwrap()];
        for i in 1..sub_variable_num {
            interpolate_cosets.push(Helper::pow(&interpolate_cosets[i - 1], 2));
        }
        let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
        (interpolate_cosets, oracle)
    });
    let (interpolate_cosets, oracle) = setup.value;

    let commit = timed(|| {
        let mut prover = Prover::new(
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
        let mut verifier = Verifier::new(
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
        + (2 * sub_variable_num).saturating_sub(3) * MERKLE_ROOT_SIZE
        + 2 * size_of::<T>();

    print_perf_line(
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

fn main() {
    let num_vars = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(8);
    println!("PERF|kind=metadata|impl=current|crate=cfri|profile=release");
    run_pipfri(num_vars);
}
