use crate::backend::basefold::{
    build_eq_x_r_vec, sum_check_challenge_round, sum_check_first_round,
};
pub(crate) mod permutation_check;
use crate::backend::basefold::{
    multilinear_evaluation_ztoa, BasefoldParams, BasefoldProverParams, BasefoldVerifierParams,
};
use crate::backend::basefold::{Basefold, BasefoldCommitment, Type1Polynomial, Type2Polynomial};
use crate::backend::blaze_transcript::BlazeBlake2sTranscript;
use crate::backend::code::encode_bits_ser;
use crate::backend::transcript::Blake2sTranscript;
use crate::backend::transcript::FieldTranscriptRead;
use crate::backend::transcript::FieldTranscriptWrite;
use crate::backend::transcript::{
    FieldTranscript, InMemoryTranscript, TranscriptRead, TranscriptWrite,
};
use itertools::chain;
use num_traits::Zero;
use std::array::IntoIter;
use std::iter::Chain;

use crate::backend::code::{repetition_code_long, serial_accumulator_long};
use crate::backend::hash::Blake2s;

use crate::backend::basefold::BasefoldExtParams;
use crate::backend::binary_extension_fields::B128;

//use crate::backend::test::rand_vec;
use crate::backend::avx_int_types::{u64::Blazeu64, BlazeField};
use crate::backend::{avx_int_types::u64, hash::Keccak256};
use rand::rngs::OsRng;

use crate::backend::pcs::Commitment;
use crate::backend::piop::sum_check::{
    classic::{ClassicSumCheck, CoefficientsProver},
    eq_xy_eval, SumCheck as _, VirtualPolynomial,
};

use crate::backend::{
    arithmetic::{div_ceil, horner, inner_product, steps, BatchInvert, Field, PrimeField},
    code::{encode_bits, encode_bits_long, Permutation},
    expression::{Expression, Query, Rotation},
    hash::{Hash, Output},
    new_fields::{Mersenne127, Mersenne61},
    parallel::{num_threads, parallelize, parallelize_iter},
    pcs::{
        multilinear::{additive, validate_input},
        AdditiveCommitment, Evaluation, Point, PolynomialCommitmentScheme,
    },
    poly::{multilinear::MultilinearPolynomial, Polynomial},
    BigUint, Deserialize, DeserializeOwned, Error, Itertools, Serialize,
};
use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use core::fmt::Debug;
use core::ptr::addr_of;
use ctr;
use ff::BatchInverter;
use generic_array::GenericArray;
use halo2_curves::bn256::{Bn256, Fr};
use itertools::izip;

use rayon::iter::IntoParallelIterator;
use std::time::Duration;
use std::{collections::HashMap, iter, ops::Deref, time::Instant};

use crate::plonky2_util::{reverse_bits, reverse_index_bits_in_place};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha12Rng, ChaCha8Rng,
};
use rayon::current_num_threads;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
    ParallelSlice, ParallelSliceMut,
};
use std::{borrow::Cow, marker::PhantomData, mem::size_of, slice};

pub type CommitmentChunk<H: Hash> = Output<H>;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlazeParams {
    permutation: Permutation,
    log_rate: usize,
    num_vars: usize,
    num_rows: usize,
    rng: ChaCha8Rng,
    split_basefold_params: BasefoldParams<B128>,
    reg_basefold_params: BasefoldParams<B128>,
    num_queries: usize,
    log_num_chunks: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlazeProverParam {
    log_rate: usize,
    num_vars: usize,
    permutation: Permutation,
    rng: ChaCha8Rng,
    split_basefold_prover_param: BasefoldProverParams<B128>,
    reg_basefold_prover_param: BasefoldProverParams<B128>,
    num_queries: usize,
    log_num_chunks: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlazeVerifierParam {
    log_rate: usize,
    num_vars: usize,
    rng: ChaCha8Rng,
    permutation: Permutation,
    split_basefold_verifier_param: BasefoldVerifierParams<B128>,
    reg_basefold_verifier_param: BasefoldVerifierParams<B128>,
    num_queries: usize,
    num_rows: usize,
    log_num_chunks: usize,
}

const BASEFOLD_RATE: usize = 1;
const DEFAULT_BLAZE_NUM_QUERIES: usize = 1059;

#[derive(Clone, Debug)]
pub struct BlazeBasefoldParams;

impl BasefoldExtParams for BlazeBasefoldParams {
    fn get_reps() -> usize {
        402
    }

    fn get_rate() -> usize {
        BASEFOLD_RATE
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

pub type BlazeBasefoldPcs<H> = Basefold<B128, H, BlazeBasefoldParams>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "F: Serialize", deserialize = "F: DeserializeOwned"))]
pub struct BlazeCommitment<F: BlazeField, H: Hash> {
    codeword: Vec<Vec<F>>,
    codeword_tree: Vec<Vec<Output<H>>>,
    bh_evals: Vec<Vec<F>>,
}
impl<F: Field + BlazeField, H: Hash> AsRef<[Output<H>]> for BlazeCommitment<F, H> {
    fn as_ref(&self) -> &[Output<H>] {
        let root = &self.codeword_tree[self.codeword_tree.len() - 1][0];
        slice::from_ref(root)
    }
}

impl<F: BlazeField, H: Hash> AsRef<Output<H>> for BlazeCommitment<F, H> {
    fn as_ref(&self) -> &Output<H> {
        let root = &self.codeword_tree[self.codeword_tree.len() - 1][0];
        &root
    }
}

impl<F: BlazeField, H: Hash> Default for BlazeCommitment<F, H> {
    fn default() -> Self {
        Self {
            codeword: Vec::new(),
            codeword_tree: vec![vec![Output::<H>::default()]],
            bh_evals: Vec::new(),
        }
    }
}

impl<F: BlazeField, H: Hash> BlazeCommitment<F, H> {
    pub fn public(&self) -> Self {
        Self {
            codeword: Vec::new(),
            codeword_tree: vec![vec![
                self.codeword_tree[self.codeword_tree.len() - 1][0].clone()
            ]],
            bh_evals: Vec::new(),
        }
    }
}

//*** STAND ALONE FUNCTIONS **//
pub fn setup<H: Hash>(
    poly_size: usize,
    _: usize,
    rng: impl RngCore,
    num_rows: Option<usize>,
    num_queries: Option<usize>,
) -> BlazeParams {
    let num_queries = match num_queries {
        None => DEFAULT_BLAZE_NUM_QUERIES,
        Some(x) => x,
    };
    let num_rows = match num_rows {
        None => 64,
        Some(x) => x,
    };
    let log_rate = 2;
    let num_vars = log2_strict(poly_size);
    let mut rng: ChaCha8Rng = ChaCha8Rng::from_entropy(); //TODO - use RngCore instead so it can be passed in
    let permutation = Permutation::create(&mut rng, (poly_size * (1 << log_rate)));
    let log_num_chunks = 1;
    //TODO: how is poly size determined?
    let split_params = {
        let mut rng = OsRng;
        let poly_size = 1 << (num_vars + 3 - log_num_chunks);
        BlazeBasefoldPcs::<H>::setup(poly_size, 1, &mut rng).unwrap()
    };

    let reg_basefold_params = {
        let mut rng = OsRng;
        let poly_size = 1 << (num_vars + 2);
        BlazeBasefoldPcs::<H>::setup(poly_size, 1, &mut rng).unwrap()
    };
    BlazeParams {
        split_basefold_params: split_params,
        reg_basefold_params,
        permutation,
        log_rate,
        num_vars,
        num_rows,
        rng,
        num_queries,
        log_num_chunks,
    }
}

pub fn setup_with_hiding<H: Hash>(
    poly_size: usize,
    batch_size: usize,
    rng: impl RngCore,
    num_rows: Option<usize>,
    num_queries: Option<usize>,
) -> BlazeParams {
    setup::<H>(poly_size << 1, batch_size, rng, num_rows, num_queries)
}

pub fn trim<H: Hash>(
    param: &BlazeParams,
    poly_size: usize,
    batch_size: usize,
) -> (BlazeProverParam, BlazeVerifierParam) {
    let log_num_rows = log2_strict(param.num_rows);
    let num_vars = log2_strict(poly_size);

    let (pp, vp) = BlazeBasefoldPcs::<H>::trim(
        &param.split_basefold_params,
        1 << (param.num_vars - param.log_num_chunks + 3),
        0,
    )
    .ok()
    .unwrap();

    let (ppp, pvp) =
        BlazeBasefoldPcs::<H>::trim(&param.reg_basefold_params, 1 << ((param.num_vars) + 2), 0)
            .ok()
            .unwrap();
    (
        BlazeProverParam {
            split_basefold_prover_param: pp,
            reg_basefold_prover_param: ppp,
            log_rate: param.log_rate,
            num_vars: param.num_vars,
            rng: param.rng.clone(),
            permutation: param.permutation.clone(), //temporary measure, need to make this a shared reference,
            num_queries: param.num_queries,
            log_num_chunks: param.log_num_chunks,
        },
        BlazeVerifierParam {
            log_rate: param.log_rate,
            num_vars: param.num_vars,
            rng: param.rng.clone(),
            permutation: param.permutation.clone(), //temporary measure
            split_basefold_verifier_param: vp,
            reg_basefold_verifier_param: pvp,
            num_queries: param.num_queries,
            log_num_chunks: param.log_num_chunks,
            num_rows: param.num_rows,
        },
    )
}

pub fn trim_with_hiding<H: Hash>(
    param: &BlazeParams,
    poly_size: usize,
    batch_size: usize,
) -> (BlazeProverParam, BlazeVerifierParam) {
    trim::<H>(param, poly_size << 1, batch_size)
}

pub fn commit<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    word: &Vec<Vec<F>>,
) -> BlazeCommitment<F, H> {
    let mut timer = Duration::new(0, 0);
    let now = Instant::now();
    let codeword: Vec<Vec<F>> = encode_bits_long(
        //TODO - do this with two distinct permutations
        &word,
        &pp.permutation,
        &pp.permutation,
        (1 << pp.log_rate),
        &mut timer,
    );
    println!(
        "degree {:?}, raa encode time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    //println!("encode time {:?}", now.elapsed());
    let now = Instant::now();
    let tree = merkelize_long::<H, F>(&codeword);
    println!(
        "degree {:?}, raa merkle time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    BlazeCommitment {
        codeword,
        codeword_tree: tree,
        bh_evals: word.to_vec(), //TODO: Fill this in -> this will be needed for sumcheck
    }
}
pub fn commit_2<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    word: &Vec<Vec<F>>,
) -> BlazeCommitment<F, H> {
    let mut timer = Duration::new(0, 0);
    let now = Instant::now();
    let codeword: Vec<Vec<F>> = word
        .par_iter()
        .map(|w| {
            encode_bits_ser(
                //TODO - do this with two distinct permutations
                w.to_vec(),
                &pp.permutation,
                (1 << pp.log_rate),
            )
        })
        .collect();
    println!(
        "degree {:?}, raa encode time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    // println!("encode time {:?}", now.elapsed());
    let now = Instant::now();
    let tree = merkelize_long::<H, F>(&codeword);
    println!(
        "degree {:?}, raa merkle time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    BlazeCommitment {
        codeword,
        codeword_tree: tree,
        bh_evals: word.to_vec(), //TODO: Fill this in -> this will be needed for sumcheck
    }
}
struct BlazeSumcheckTrace<F> {
    challenges: Vec<F>,
}

struct BlazeSumcheckCheck<F> {
    initial_sum: F,
    terminal_point: Vec<F>,
    terminal_claim: F,
}

fn sum_check<F: PrimeField, H: Hash>(
    poly: &Vec<F>,
    point: &Vec<F>,
    num_vars: usize,
    num_rounds: usize,
    eq: &Vec<F>,
    transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
) -> BlazeSumcheckTrace<F> {
    assert_eq!(poly.len(), eq.len());
    let mut eval = F::ZERO;
    let mut bh_evals = Type1Polynomial {
        poly: Vec::with_capacity(1 << num_vars),
    };
    for i in 0..eq.len() {
        eval = eval + poly[i] * eq[i];
        bh_evals.poly.push(poly[i]);
    }

    let mut eq = Type1Polynomial { poly: eq.to_vec() };
    let mut sum_check_oracles_vec = Vec::with_capacity(num_rounds + 1);
    let mut challenges = Vec::with_capacity(num_rounds);

    let mut sum_check_oracle = sum_check_first_round::<F>(&mut eq, &mut bh_evals);
    sum_check_oracles_vec.push(sum_check_oracle.clone());
    for i in 0..(num_rounds) {
        transcript.write_field_elements(&sum_check_oracle);
        let challenge: F = transcript.squeeze_challenge();
        challenges.push(challenge);

        sum_check_oracle = sum_check_challenge_round(&mut eq, &mut bh_evals, challenge);

        sum_check_oracles_vec.push(sum_check_oracle.clone());
    }

    let terminal_oracle = vec![bh_evals.poly[0] * eq.poly[0], F::ZERO, F::ZERO];
    transcript.write_field_elements(&terminal_oracle);
    BlazeSumcheckTrace { challenges }
}

fn polynomial_eval<F: PrimeField>(poly: &[F], point: F) -> F {
    poly.iter()
        .rev()
        .fold(F::ZERO, |acc, coeff| acc * point + *coeff)
}

fn polynomial_zero_plus_one<F: PrimeField>(poly: &[F]) -> F {
    poly[0] + polynomial_eval(poly, F::ONE)
}

fn verify_sumcheck_chain<F: PrimeField>(
    oracles: &[Vec<F>],
    challenges: &[F],
    claimed_sum: Option<F>,
    degree: usize,
) -> Result<BlazeSumcheckCheck<F>, Error> {
    let expected_oracle_len = degree + 1;
    if oracles.len() != challenges.len() + 1
        || oracles.iter().any(|poly| poly.len() != expected_oracle_len)
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze sumcheck transcript has invalid shape".to_string(),
        ));
    }

    let observed_sum = polynomial_zero_plus_one(&oracles[0]);
    if let Some(claimed_sum) = claimed_sum {
        if observed_sum != claimed_sum {
            return Err(Error::InvalidPcsOpen(format!(
                "Blaze sumcheck initial claim check failed: expected {claimed_sum:?}, got {observed_sum:?}",
            )));
        }
    }

    for (round, challenge) in challenges.iter().take(challenges.len() - 1).enumerate() {
        if polynomial_eval(&oracles[round], *challenge)
            != polynomial_zero_plus_one(&oracles[round + 1])
        {
            return Err(Error::InvalidPcsOpen(format!(
                "Blaze sumcheck consistency check failed at round {round}"
            )));
        }
    }
    let terminal_oracle = oracles.last().unwrap();
    let terminal_claim = polynomial_eval(
        &oracles[challenges.len() - 1],
        challenges[challenges.len() - 1],
    );
    if terminal_oracle[0] != terminal_claim || terminal_oracle[1..].iter().any(|v| *v != F::ZERO) {
        return Err(Error::InvalidPcsOpen(format!(
            "Blaze sumcheck terminal claim check failed: expected {terminal_claim:?}, got {:?}",
            terminal_oracle,
        )));
    }

    Ok(BlazeSumcheckCheck {
        initial_sum: observed_sum,
        terminal_point: challenges.to_vec(),
        terminal_claim,
    })
}

fn interpolate_coefficients<F: PrimeField>(points: &[F], evals: &[F]) -> Vec<F> {
    assert_eq!(points.len(), evals.len());
    let mut coeffs = vec![F::ZERO; points.len()];

    for (i, (&x_i, &eval_i)) in points.iter().zip(evals.iter()).enumerate() {
        let mut basis = vec![F::ONE];
        let mut denom = F::ONE;
        for (j, &x_j) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut next = vec![F::ZERO; basis.len() + 1];
            for (degree, coeff) in basis.iter().enumerate() {
                next[degree] -= *coeff * x_j;
                next[degree + 1] += *coeff;
            }
            basis = next;
            denom *= x_i - x_j;
        }

        let scale = eval_i * denom.invert().unwrap();
        for (out, coeff) in coeffs.iter_mut().zip(basis.iter()) {
            *out += *coeff * scale;
        }
    }

    coeffs
}

fn boolean_suffix<F: PrimeField>(index: usize, len: usize) -> Vec<F> {
    (0..len)
        .map(|bit| F::from(((index >> bit) & 1) as u64))
        .collect()
}

fn permutation_product_expression(
    split_binding: &[MultilinearPolynomial<B128>],
    num_split_chunks: usize,
    coeffs: &[B128],
    point: &[B128],
) -> B128 {
    debug_assert_eq!(num_split_chunks, 2);
    debug_assert_eq!(split_binding.len(), coeffs.len() * num_split_chunks);
    let queries = permutation_check::terminal_queries(point);
    (0..coeffs.len())
        .map(|tree_idx| {
            let tree_offset = tree_idx * num_split_chunks;
            let tree_eval = queries
                .iter()
                .map(|query| {
                    let left =
                        split_binding[tree_offset + query.child_chunk].evaluate(&query.left_point);
                    let right =
                        split_binding[tree_offset + query.child_chunk].evaluate(&query.right_point);
                    let parent = split_binding[tree_offset + query.parent_chunk]
                        .evaluate(&query.parent_point);
                    query.selector * (parent - left * right)
                })
                .sum::<B128>();
            coeffs[tree_idx] * tree_eval
        })
        .sum()
}

fn permutation_product_sum_check<H: Hash>(
    split_binding: &[MultilinearPolynomial<B128>],
    num_split_chunks: usize,
    coeffs: &[B128],
    eq: &[B128],
    num_rounds: usize,
    transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, B128>,
) -> BlazeSumcheckTrace<B128> {
    let eq_poly = MultilinearPolynomial::new(eq.to_vec());
    let interpolation_points = [B128::ZERO, B128::ONE, B128::from(2), B128::from(3)];
    let mut challenges = Vec::with_capacity(num_rounds);

    for round in 0..num_rounds {
        let remaining = num_rounds - round - 1;
        let round_eval = |challenge: B128| {
            (0..(1usize << remaining))
                .into_par_iter()
                .map(|suffix_index| {
                    let mut point = Vec::with_capacity(num_rounds);
                    point.extend_from_slice(&challenges);
                    point.push(challenge);
                    point.extend(boolean_suffix::<B128>(suffix_index, remaining));
                    eq_poly.evaluate(&point)
                        * permutation_product_expression(
                            split_binding,
                            num_split_chunks,
                            coeffs,
                            &point,
                        )
                })
                .sum::<B128>()
        };

        let oracle_evals = interpolation_points
            .iter()
            .map(|point| round_eval(*point))
            .collect::<Vec<_>>();
        let oracle = interpolate_coefficients(&interpolation_points, &oracle_evals);
        let _ = transcript.write_field_elements(&oracle);
        challenges.push(transcript.squeeze_challenge());
    }

    let terminal_claim = eq_poly.evaluate(&challenges)
        * permutation_product_expression(split_binding, num_split_chunks, coeffs, &challenges);
    let _ =
        transcript.write_field_elements(&vec![terminal_claim, B128::ZERO, B128::ZERO, B128::ZERO]);
    BlazeSumcheckTrace { challenges }
}

pub fn commit_and_write<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    word: &Vec<Vec<F>>,
    transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
) -> BlazeCommitment<F, H> {
    let now = Instant::now();
    let codeword: Vec<Vec<F>> = word
        .par_iter()
        .map(|w| {
            encode_bits_ser(
                //TODO - do this with two distinct permutations
                w.to_vec(),
                &pp.permutation,
                (1 << pp.log_rate),
            )
        })
        .collect();
    println!(
        "degree {:?}, raa encode time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    //   println!("encode time {:?}", now.elapsed());
    let now = Instant::now();
    let tree = merkelize_long::<H, F>(&codeword);
    println!(
        "degree {:?}, raa merkle time {:?}",
        pp.num_vars,
        now.elapsed()
    );

    transcript.write_commitment(&tree[tree.len() - 1][0]);
    println!("one commitment written");
    BlazeCommitment {
        codeword,
        codeword_tree: tree,
        bh_evals: word.to_vec(),
    }
}

fn hide_blaze_rows<F: BlazeField>(word: &Vec<Vec<F>>) -> Vec<Vec<F>> {
    word.par_iter()
        .map(|row| {
            let mask = F::rand_vec(row.len());
            let mut hidden_row = Vec::with_capacity(row.len() * 2);
            hidden_row.extend_from_slice(row);
            hidden_row.extend(
                row.iter()
                    .zip(mask.iter())
                    .map(|(value, mask)| *value ^ *mask),
            );
            hidden_row
        })
        .collect()
}

fn extend_blaze_hiding_point(point: &Vec<B128>) -> Vec<B128> {
    let mut hidden_point = Vec::with_capacity(point.len() + 1);
    hidden_point.extend_from_slice(point);
    hidden_point.push(B128::ZERO);
    hidden_point
}

pub fn commit_with_hiding<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    word: &Vec<Vec<F>>,
) -> BlazeCommitment<F, H> {
    let hidden_word = hide_blaze_rows(word);
    commit(pp, &hidden_word)
}

pub fn commit_and_write_with_hiding<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    word: &Vec<Vec<F>>,
    transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
) -> BlazeCommitment<F, H> {
    let hidden_word = hide_blaze_rows(word);
    commit_and_write(pp, &hidden_word, transcript)
}

pub fn open_with_hiding<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    poly: &Vec<Vec<F>>,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    blazetranscript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    b128transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, B128>,
) -> Result<B128, Error> {
    let hidden_point = extend_blaze_hiding_point(point);
    open(
        pp,
        poly,
        comm,
        &hidden_point,
        eval,
        blazetranscript,
        b128transcript,
    )
}

pub fn verify_with_hiding<F: BlazeField, H: Hash>(
    vp: &BlazeVerifierParam,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    b128transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blazetranscript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> Result<(), Error> {
    let num_row_vars = log2_strict(vp.num_rows >> 1);
    let expected_public_vars = num_row_vars + vp.num_vars.saturating_sub(1);
    if vp.num_vars == 0 || point.len() != expected_public_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid hiding Blaze opening point: expected {} coordinates, got {}",
            expected_public_vars,
            point.len()
        )));
    }
    let transcript_point = b128transcript.squeeze_challenges(expected_public_vars);
    check_transcript_point(&transcript_point, point)?;
    let hidden_point = extend_blaze_hiding_point(point);
    verify_after_point_check(
        vp,
        comm,
        &hidden_point,
        eval,
        b128transcript,
        blazetranscript,
    )
}

struct BlazeProof<F: BlazeField> {
    row_evals: Vec<F>,
}
#[inline(always)]
fn one_dimension_eval<F: BlazeField>(poly: &Vec<Vec<F>>, point: Vec<B128>) -> Vec<B128> {
    poly.into_par_iter()
        .map(|evals| {
            let (mut coeffs, mut bh_evals) =
                interpolate_over_boolean_hypercube_with_copy(&BlazeType2Polynomial {
                    poly: evals.to_vec(),
                });
            //convert coeffs to vector of b128
            let mut b128_coeffs = Type2Polynomial {
                poly: coeffs.poly.iter().map(|c| c.to_b128()).collect::<Vec<_>>(),
            };
            multilinear_evaluation_ztoa(&mut b128_coeffs, &point.to_vec());
            b128_coeffs.poly[0]
        })
        .collect::<Vec<B128>>()
}
fn two_dimension_eval<F: BlazeField>(
    dim_one_poly: &mut Type2Polynomial<B128>,
    part_2_point: Vec<B128>,
) -> B128 {
    multilinear_evaluation_ztoa(dim_one_poly, &part_2_point);
    dim_one_poly.poly[0]
}
fn accumulation_mle(point: &Vec<B128>) -> Type1Polynomial<B128> {
    //get all evaluations of eq(X,point) on the boolean hypercube
    let eq = build_eq_x_r_vec::<B128>(&point).unwrap();
    let len = eq.len();
    //now for each element on the boolean hypercube compute the sum eq(x,z), x> b
    let mut result = vec![B128::zero(); len];
    result[len - 1] = eq[len - 1];
    for i in (0..len - 1).rev() {
        result[i] = result[i + 1] + eq[i];
    }
    Type1Polynomial { poly: result }
}
fn linear_combination<F: PrimeField>(vecs: Vec<Vec<F>>, coeffs: Vec<F>) -> Vec<F> {
    assert_eq!(coeffs.len(), vecs.len());

    let mut result = vec![F::ZERO; vecs[0].len()];
    result.par_iter_mut().enumerate().for_each(|(i, mut c)| {
        for j in 0..vecs.len() {
            *c += coeffs[j] * vecs[j][i];
        }
    });
    result
}

fn b128_rows<F: BlazeField>(rows: &[Vec<F>]) -> Result<Vec<Vec<B128>>, Error> {
    if rows.is_empty() || rows.len() % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze expects a non-empty even number of packed rows".to_string(),
        ));
    }
    let row_len = rows[0].len();
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(Error::InvalidPcsOpen(
            "Blaze rows must all have the same length".to_string(),
        ));
    }

    Ok(rows
        .chunks_exact(2)
        .map(|pair| {
            (0..row_len)
                .map(|col| F::to_b128_vec(vec![pair[0][col], pair[1][col]]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}

fn evaluate_b128_row(row: &[B128], point: &[B128]) -> Result<B128, Error> {
    if row.len() != (1usize << point.len()) {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze row evaluation point: row has {} entries but point has {} coordinates",
            row.len(),
            point.len()
        )));
    }
    let mut poly = Type2Polynomial { poly: row.to_vec() };
    multilinear_evaluation_ztoa(&mut poly, &point.to_vec());
    Ok(poly.poly[0])
}

fn evaluate_b128_rows(rows: &[Vec<B128>], point: &[B128]) -> Result<Vec<B128>, Error> {
    rows.iter()
        .map(|row| evaluate_b128_row(row, point))
        .collect()
}

fn b128_codeword_rows<F: BlazeField>(rows: &[Vec<F>]) -> Result<Vec<Vec<B128>>, Error> {
    b128_rows(rows)
}

fn index_to_hypercube_point(index: usize, num_vars: usize) -> Vec<B128> {
    (0..num_vars)
        .map(|bit| {
            if ((index >> bit) & 1) == 1 {
                B128::ONE
            } else {
                B128::ZERO
            }
        })
        .collect()
}

pub fn evaluate_commitment<F: BlazeField, H: Hash>(
    comm: &BlazeCommitment<F, H>,
    point: &[B128],
) -> Result<B128, Error> {
    let b128_rows = b128_rows(&comm.bh_evals)?;
    let row_size = b128_rows[0].len();
    let num_row_vars = log2_strict(b128_rows.len());
    let num_col_vars = log2_strict(row_size);
    if point.len() != num_row_vars + num_col_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze opening point length: expected {} row+column coordinates, got {}",
            num_row_vars + num_col_vars,
            point.len()
        )));
    }

    let (row_point, col_point) = point.split_at(num_row_vars);
    let row_evals = evaluate_b128_rows(&b128_rows, col_point)?;
    eval_folded_blaze_poly(&row_evals, row_point)
}

fn eval_folded_blaze_poly(folded_poly: &[B128], point: &[B128]) -> Result<B128, Error> {
    if folded_poly.len() != (1usize << point.len()) {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze opening point length: folded polynomial has {} evaluations but point has {} coordinates",
            folded_poly.len(),
            point.len()
        )));
    }

    let mut poly = Type2Polynomial {
        poly: folded_poly.to_vec(),
    };
    multilinear_evaluation_ztoa(&mut poly, &point.to_vec());
    Ok(poly.poly[0])
}

fn expected_folded_blaze_eval<F: BlazeField, H: Hash>(
    comm: &BlazeCommitment<F, H>,
    point: &[B128],
    challenges: &[B128],
) -> Result<B128, Error> {
    if comm.bh_evals.is_empty() {
        return Err(Error::InvalidPcsOpen(
            "Invalid Blaze commitment: empty evaluation matrix".to_string(),
        ));
    }

    let row_size = comm.bh_evals[0].len();
    let num_vars = log2_strict(row_size);
    if point.len() != num_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze opening point length: expected {num_vars}, got {}",
            point.len()
        )));
    }

    let folded_poly =
        blazefield_linear_combo_even_faster(&challenges.to_vec(), &comm.bh_evals, 128);
    eval_folded_blaze_poly(&folded_poly, point)
}

fn check_transcript_point(transcript_point: &[B128], claimed_point: &[B128]) -> Result<(), Error> {
    if transcript_point != claimed_point {
        return Err(Error::InvalidPcsOpen(
            "Blaze opening point does not match transcript challenge".to_string(),
        ));
    }
    Ok(())
}
#[cfg(feature = "upstream-tests")]
#[test]
fn test_split() {
    let poly = MultilinearPolynomial::new(vec![B128::ONE, B128::ONE, B128::ONE, B128::ONE]);
    let new_polys = poly.split(1);
    assert_eq!(new_polys.len(), 2);
    println!("new polys {:?}", new_polys);
}
pub fn open<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    poly: &Vec<Vec<F>>,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    blazetranscript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    b128transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, B128>,
) -> Result<B128, Error> {
    let b128_rows = b128_rows(&comm.bh_evals)?;
    let row_size = b128_rows[0].len();
    let codeword_len = comm.codeword[0].len();
    let num_row_vars = log2_strict(b128_rows.len());
    let num_col_vars = log2_strict(row_size);
    if point.len() != num_row_vars + num_col_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze opening point length: expected {} row+column coordinates, got {}",
            num_row_vars + num_col_vars,
            point.len()
        )));
    }
    let (row_point, col_point) = point.split_at(num_row_vars);

    let row_evals = evaluate_b128_rows(&b128_rows, col_point)?;
    b128transcript.write_field_elements(&row_evals);
    let claimed_eval = eval_folded_blaze_poly(&row_evals, row_point)?;
    if &claimed_eval != eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze claimed evaluation does not match row evaluations".to_string(),
        ));
    }

    // Fold the interleaved rows after the row evaluations are transcript-bound.
    let challenges: Vec<B128> = b128transcript.squeeze_challenges(b128_rows.len());
    println!("{:?} row folding challenges squeezed", b128_rows.len());
    let now = Instant::now();
    let folded_poly_b128 = linear_combination(b128_rows, challenges.clone());
    let folded_poly_blaze = b128_to_bf::<F>(&folded_poly_b128);
    println!("linear combo {:?}", now.elapsed());

    let now = Instant::now();
    let opening_eval = eval_folded_blaze_poly(&folded_poly_b128, col_point)?;
    let expected_folded_eval = inner_product(row_evals.iter(), &challenges);
    if opening_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze folded evaluation does not match row-evaluation fold".to_string(),
        ));
    }
    b128transcript.write_field_element(&opening_eval)?;

    println!("eval {:?}", now.elapsed());

    let now = Instant::now();
    let u1 = repetition_code_long(&folded_poly_blaze, (1 << pp.log_rate));
    let u2 = pp.permutation.interleave1_long(&u1);
    let mut u3 = u2.clone();
    serial_accumulator_long(&mut u3);
    let u4 = pp.permutation.interleave2_long(&u3);
    let mut u5 = u4.clone();
    serial_accumulator_long(&mut u5);

    println!("intermediate raa steps {:?}", now.elapsed());

    let now = Instant::now();

    let mut raa_words = vec![u1, u2, u3, u4];
    let mut raa_b128 = Vec::new();
    for word in raa_words {
        raa_b128.push(bf_to_b128_vec_long(&word));
    }

    let ml_polys = raa_b128
        .iter()
        .map(|v| {
            let mut poly = v.to_vec();
            poly.resize(v.len() * 2, B128::zero());
            MultilinearPolynomial::new(poly)
        })
        .collect::<Vec<_>>();

    println!("transforms {:?}", now.elapsed());
    let now = Instant::now();
    let split_polys = ml_polys
        .par_iter()
        .map(|p| p.split(pp.log_num_chunks))
        .flatten()
        .collect::<Vec<_>>();
    let num_split_chunks = 1 << pp.log_num_chunks;
    let live_raa_chunks = num_split_chunks >> 1;
    assert!(live_raa_chunks > 0);
    let split_raa_polys = split_polys
        .chunks_exact(num_split_chunks)
        .flat_map(|chunks| chunks.iter().take(live_raa_chunks).cloned())
        .collect::<Vec<_>>();
    println!("split {:?}", now.elapsed());

    assert_eq!(
        split_raa_polys[0].num_vars(),
        pp.split_basefold_prover_param.num_vars
    );
    // Commit only the live RAA chunks. The padded high chunks are identically zero.
    let now = Instant::now();

    let raa_commitments: Vec<BasefoldCommitment<B128, H>> =
        BlazeBasefoldPcs::batch_commit_and_write(
            &pp.split_basefold_prover_param,
            &split_raa_polys,
            b128transcript,
        )
        .unwrap(); //ONE TRNASCRIPT WRITE

    println!("commitments {:?}", now.elapsed());

    let now = Instant::now();
    let (alpha, beta) = (
        b128transcript.squeeze_challenge(),
        b128transcript.squeeze_challenge(),
    );
    let (f1, _f1_combo) = build_permutation_polynomials(
        Some(&pp.permutation.permutation1),
        &ml_polys[0],
        alpha,
        beta,
    );
    let (f2, _f2_combo) = build_permutation_polynomials(None, &ml_polys[1], alpha, beta);
    let (g1, _g1_combo) = build_permutation_polynomials(
        Some(&pp.permutation.permutation2),
        &ml_polys[2],
        alpha,
        beta,
    );
    let (g2, _g2_combo) = build_permutation_polynomials(None, &ml_polys[3], alpha, beta);
    let binding = vec![f1, f2, g1, g2];

    let split_binding = binding
        .par_iter()
        .map(|p| p.split(pp.log_num_chunks))
        .flatten()
        .collect::<Vec<_>>();

    assert_eq!(
        split_binding.len(),
        binding.len() * (1 << pp.log_num_chunks)
    );
    println!("build perms {:?}", now.elapsed());
    let now = Instant::now();
    let perm_commitments: Vec<BasefoldCommitment<B128, H>> =
        BlazeBasefoldPcs::batch_commit_and_write(
            &pp.split_basefold_prover_param,
            &split_binding,
            b128transcript,
        )
        .unwrap(); //TWO TRANSCRIPT WRITES
    println!("perm batch commitments {:?}", now.elapsed());

    let rand_point = b128transcript.squeeze_challenges(pp.reg_basefold_prover_param.num_vars);

    //compute accumulation matrix:
    let now = Instant::now();
    let accum = accumulation_mle(&rand_point);

    println!("accum {:?}", now.elapsed());

    // Zero-check the actual product-tree relation expression, not a
    // precomputed zero table.
    let now = Instant::now();
    let coeffs = b128transcript.squeeze_challenges(binding.len());
    let eq = build_eq_x_r_vec::<B128>(&rand_point).unwrap();
    assert_eq!(split_binding.len(), binding.len() * num_split_chunks);

    println!("perm relation setup {:?}", now.elapsed());

    let now = Instant::now();
    let perm_sumcheck = permutation_product_sum_check::<H>(
        &split_binding,
        num_split_chunks,
        &coeffs,
        &eq,
        pp.reg_basefold_prover_param.num_vars,
        b128transcript,
    );

    //reduce random eval on u5 to eval of u4
    //  assert_eq!(raa_b128[4].len(),accum.poly.len());
    let accum_sumcheck_2 = sum_check::<B128, H>(
        &raa_b128[3],
        &rand_point,
        pp.reg_basefold_prover_param.num_vars,
        pp.reg_basefold_prover_param.num_vars,
        &accum.poly,
        b128transcript,
    );
    //    assert_eq!(raa_b128[2].len(),accum.poly.len());
    let accum_sumcheck_1 = sum_check::<B128, H>(
        &raa_b128[1],
        &rand_point,
        pp.reg_basefold_prover_param.num_vars,
        pp.reg_basefold_prover_param.num_vars,
        &accum.poly,
        b128transcript,
    );
    // THESE ARE ALL TRANSCRIPT WRITES
    println!("sumchecks {:?}", now.elapsed());
    //fix this, this is for the opening
    let mut al_rand_point = &rand_point.clone()[0..pp.split_basefold_prover_param.num_vars];
    //evaluate f_combo -> ()
    // al_rand_point.push(B128::ONE); //this is mod - not really correct, f1,f0 need to be evaluated at different combinations of appending 1 and 0 to rand_point

    //combine bindind and ml polys
    let binding_offset = split_raa_polys.len();
    let mut polys = chain(split_raa_polys, split_binding).collect::<Vec<_>>();
    //combine perm commitments and raa commitments
    let mut commitments = chain(raa_commitments.iter(), perm_commitments.iter());
    let mut evals_al = Vec::new();
    let mut points = Vec::new();
    let mut push_eval = |poly: usize,
                         point: Vec<B128>,
                         evals: &mut Vec<Evaluation<B128>>,
                         points: &mut Vec<Vec<B128>>| {
        let point_index = points.len();
        let value = polys[poly].evaluate(&point);
        points.push(point);
        evals.push(Evaluation {
            poly,
            point: point_index,
            value,
        });
    };

    for poly in 0..polys.len() {
        push_eval(poly, al_rand_point.to_vec(), &mut evals_al, &mut points);
    }

    let perm_terminal_queries = permutation_check::terminal_queries(&perm_sumcheck.challenges);
    assert_eq!(num_split_chunks, 2);
    for tree_idx in 0..binding.len() {
        let tree_offset = binding_offset + tree_idx * num_split_chunks;
        for query in &perm_terminal_queries {
            push_eval(
                tree_offset + query.child_chunk,
                query.left_point.clone(),
                &mut evals_al,
                &mut points,
            );
            push_eval(
                tree_offset + query.child_chunk,
                query.right_point.clone(),
                &mut evals_al,
                &mut points,
            );
            push_eval(
                tree_offset + query.parent_chunk,
                query.parent_point.clone(),
                &mut evals_al,
                &mut points,
            );
        }
    }

    // Bind the two accumulation sumchecks to the committed RAA intermediates at
    // their terminal sumcheck points.
    push_eval(
        3,
        accum_sumcheck_2.challenges.clone(),
        &mut evals_al,
        &mut points,
    );
    push_eval(
        1,
        accum_sumcheck_1.challenges.clone(),
        &mut evals_al,
        &mut points,
    );

    let fevals = evals_al.iter().map(|e| e.value).collect::<Vec<_>>();

    b128transcript.write_field_elements(&fevals);

    let now = Instant::now();
    //now do batch opening THIS IS A TRANSCRIPT WRITE
    BlazeBasefoldPcs::batch_open(
        &pp.split_basefold_prover_param,
        &polys,
        commitments,
        &points,
        &evals_al,
        b128transcript,
    )?;

    println!("batch open time {:?}", now.elapsed());

    let folded_codeword_b128 = linear_combination(b128_codeword_rows(&comm.codeword)?, challenges);
    let folded_codeword_poly = MultilinearPolynomial::new(folded_codeword_b128.clone());
    let folded_codeword_comm = BlazeBasefoldPcs::<H>::commit_and_write(
        &pp.reg_basefold_prover_param,
        &folded_codeword_poly,
        b128transcript,
    )?;

    //query commitment merkle tree and write to transcript
    let mut queries = b128transcript.squeeze_challenges(pp.num_queries); //ned to fix this, it only works for prime fields

    let queries_usize: Vec<usize> = queries
        .par_iter()
        .map(|x_index| {
            let x_rep = (*x_index).to_repr();
            let mut x: &[u8] = x_rep.as_ref();
            let (int_bytes, rest) = x.split_at(std::mem::size_of::<u32>());
            let x_int: u32 = u32::from_be_bytes(int_bytes.try_into().unwrap());
            ((x_int as usize) % codeword_len).into()
        })
        .collect::<Vec<_>>();

    let mut query_points = Vec::with_capacity(pp.num_queries + 1);
    query_points.push(rand_point.clone());
    query_points.extend(
        queries_usize
            .iter()
            .map(|query| index_to_hypercube_point(*query, pp.reg_basefold_prover_param.num_vars)),
    );
    let mut folded_codeword_evals = Vec::with_capacity(pp.num_queries + 1);
    folded_codeword_evals.push(folded_codeword_poly.evaluate(&rand_point));
    folded_codeword_evals.extend(
        queries_usize
            .iter()
            .map(|query| folded_codeword_b128[*query]),
    );
    b128transcript.write_field_elements(&folded_codeword_evals)?;
    let folded_codeword_eval_claims = folded_codeword_evals
        .iter()
        .enumerate()
        .map(|(point, value)| Evaluation {
            poly: 0,
            point,
            value: *value,
        })
        .collect::<Vec<_>>();
    BlazeBasefoldPcs::batch_open(
        &pp.reg_basefold_prover_param,
        iter::once(&folded_codeword_poly),
        iter::once(&folded_codeword_comm),
        &query_points,
        &folded_codeword_eval_claims,
        b128transcript,
    )?;

    //write merkle paths

    queries_usize.iter().for_each(|i| {
        write_merkle_sibling_path::<H, F>(&comm.codeword_tree, *i, blazetranscript);
    });
    println!(
        "queries usize {:?} of 28 commitments written",
        queries_usize.len()
    );
    let now = Instant::now();

    //write leaves
    queries_usize.iter().for_each(|x_index| {
        let pair_start = x_index & !1;
        for row in &comm.codeword {
            blazetranscript.write_field_element(&row[pair_start]);
            blazetranscript.write_field_element(&row[pair_start + 1]);
        }
    });
    println!("collect queries {:?}", now.elapsed());
    Ok(claimed_eval)
}

pub fn faster_open<F: BlazeField, H: Hash>(
    pp: &BlazeProverParam,
    poly: &Vec<Vec<F>>,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    blazetranscript: &mut impl TranscriptWrite<CommitmentChunk<H>, F>,
    b128transcript: &mut impl TranscriptWrite<CommitmentChunk<H>, B128>,
) -> Result<B128, Error> {
    let row_size = comm.bh_evals[0].len();
    let col_size = comm.bh_evals.len();
    let num_vars_per_row = log2_strict(row_size);
    let security_param = 128;
    //fold commitment
    let challenges: Vec<B128> = bf_to_b128_vec(&blazetranscript.squeeze_challenges(col_size >> 1));
    println!("{:?} commitments squeezed", col_size >> 1);
    let now = Instant::now();
    let folded_poly_b128 = blazefield_linear_combo_even_faster(&challenges, &comm.bh_evals, 128);
    let folded_poly_blaze = b128_to_bf::<F>(&folded_poly_b128);
    println!("linear combo {:?}", now.elapsed());

    let now = Instant::now();
    let opening_eval = eval_folded_blaze_poly(&folded_poly_b128, point)?;
    println!("eval {:?}", now.elapsed());

    //query commitment merkle tree and write to transcript
    let mut queries = b128transcript.squeeze_challenges(pp.num_queries); //ned to fix this, it only works for prime fields

    let queries_usize: Vec<usize> = queries
        .par_iter()
        .map(|x_index| {
            let x_rep = (*x_index).to_repr();
            let mut x: &[u8] = x_rep.as_ref();
            let (int_bytes, rest) = x.split_at(std::mem::size_of::<u32>());
            let x_int: u32 = u32::from_be_bytes(int_bytes.try_into().unwrap());
            ((x_int as usize) % row_size).into()
        })
        .collect::<Vec<_>>();

    //write merkle paths
    queries_usize.iter().for_each(|i| {
        write_merkle_path::<H, F>(&comm.codeword_tree, *i, blazetranscript);
    });
    println!(
        "queries usize {:?} of 28 commitments written",
        queries_usize.len()
    );
    let now = Instant::now();

    //write leaves
    queries_usize.iter().for_each(|x_index| {
        for row in &comm.codeword {
            blazetranscript.write_field_element(&row[*x_index]);
        }
    });
    println!("collect queries {:?}", now.elapsed());

    b128transcript.write_field_elements(&folded_poly_b128);
    Ok(opening_eval)
}

pub fn faster_verify<F: BlazeField, H: Hash>(
    vp: &BlazeVerifierParam,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    b128transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blazetranscript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> Result<(), Error> {
    Err(Error::InvalidPcsOpen(
        "faster_verify was an incomplete benchmark path; use verify".to_string(),
    ))
}

pub fn verify<F: BlazeField, H: Hash>(
    vp: &BlazeVerifierParam,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    b128transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blazetranscript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> Result<(), Error> {
    let num_row_vars = log2_strict(vp.num_rows >> 1);
    let expected_vars = num_row_vars + vp.num_vars;
    if point.len() != expected_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze opening point: expected {} coordinates, got {}",
            expected_vars,
            point.len()
        )));
    }
    let transcript_point = b128transcript.squeeze_challenges(expected_vars);
    check_transcript_point(&transcript_point, point)?;
    verify_after_point_check(vp, comm, point, eval, b128transcript, blazetranscript)
}

fn verify_after_point_check<F: BlazeField, H: Hash>(
    vp: &BlazeVerifierParam,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    eval: &B128,
    b128transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blazetranscript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> Result<(), Error> {
    //read the blaze commitment root
    let blaze_root = blazetranscript.read_commitment()?;
    let expected_root: &Output<H> = comm.as_ref();
    if &blaze_root != expected_root {
        return Err(Error::InvalidPcsOpen(
            "Blaze proof commitment root does not match public commitment".to_string(),
        ));
    }

    let num_packed_rows = vp.num_rows >> 1;
    let row_evals = b128transcript.read_field_elements(num_packed_rows)?;
    let num_row_vars = log2_strict(num_packed_rows);
    if point.len() < num_row_vars {
        return Err(Error::InvalidPcsOpen(
            "Blaze opening point is shorter than row dimension".to_string(),
        ));
    }
    let (row_point, col_point) = point.split_at(num_row_vars);
    if col_point.len() != vp.num_vars {
        return Err(Error::InvalidPcsOpen(format!(
            "Invalid Blaze column point length: expected {}, got {}",
            vp.num_vars,
            col_point.len()
        )));
    }
    let claimed_eval = eval_folded_blaze_poly(&row_evals, row_point)?;
    if &claimed_eval != eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze claimed evaluation does not match row evaluations".to_string(),
        ));
    }

    let challenges: Vec<B128> = b128transcript.squeeze_challenges(num_packed_rows);
    let proof_eval = b128transcript.read_field_element()?;
    let expected_folded_eval = inner_product(row_evals.iter(), &challenges);
    if proof_eval != expected_folded_eval {
        return Err(Error::InvalidPcsOpen(
            "Blaze folded evaluation does not match row-evaluation fold".to_string(),
        ));
    }

    //read basefold roots

    let num_split_chunks = 1 << vp.log_num_chunks;
    let live_raa_chunks = num_split_chunks >> 1;
    let num_raa_comms = 4 * live_raa_chunks;
    let num_perm_comms = 4 * num_split_chunks;

    let basefold_comms1 = BlazeBasefoldPcs::<H>::read_commitments(
        &vp.split_basefold_verifier_param,
        num_raa_comms,
        b128transcript,
    )
    .unwrap();

    let (perm_beta, perm_alpha) = (
        b128transcript.squeeze_challenge(),
        b128transcript.squeeze_challenge(),
    );

    let basefold_comms2 = BlazeBasefoldPcs::<H>::read_commitments(
        &vp.split_basefold_verifier_param,
        num_perm_comms,
        b128transcript,
    )
    .unwrap();

    let rand_point = b128transcript.squeeze_challenges(vp.reg_basefold_verifier_param.num_vars);

    let basefold_comms = chain(basefold_comms1.iter(), basefold_comms2.iter()).collect::<Vec<_>>();
    //read sumcheck transcript
    let coeffs = b128transcript.squeeze_challenges(4);

    let now = Instant::now();
    let mut sumcheck_checks = Vec::with_capacity(3);
    for sumcheck_idx in 0..3 {
        let mut sum_check_oracles = Vec::new();
        let mut sum_check_challenges = Vec::new();
        let sumcheck_degree = if sumcheck_idx == 0 { 3 } else { 2 };
        let oracle_len = sumcheck_degree + 1;
        for _ in 0..vp.reg_basefold_verifier_param.num_rounds {
            sum_check_oracles.push(b128transcript.read_field_elements(oracle_len).unwrap());
            sum_check_challenges.push(b128transcript.squeeze_challenge());
        }
        sum_check_oracles.push(b128transcript.read_field_elements(oracle_len).unwrap());
        let claimed_sum = if sumcheck_idx == 0 {
            Some(B128::ZERO)
        } else {
            None
        };
        sumcheck_checks.push(verify_sumcheck_chain(
            &sum_check_oracles,
            &sum_check_challenges,
            claimed_sum,
            sumcheck_degree,
        )?);
    }
    println!("verify sumchecks {:?}", now.elapsed());

    if live_raa_chunks != 1 || num_split_chunks != 2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze verifier currently expects one live RAA chunk and two split chunks".to_string(),
        ));
    }

    let perm_terminal_queries =
        permutation_check::terminal_queries(&sumcheck_checks[0].terminal_point);
    let num_perm_trees = coeffs.len();
    let extra_perm_evals = num_perm_trees * perm_terminal_queries.len() * 3;
    let extra_accum_evals = 2;
    let num_aux_evals = num_raa_comms + num_perm_comms + extra_accum_evals + extra_perm_evals;
    let evaluations: Vec<B128> = b128transcript.read_field_elements(num_aux_evals).unwrap();
    let mut evals_al = evaluations
        .iter()
        .enumerate()
        .map(|(i, v)| Evaluation {
            poly: if i < num_raa_comms + num_perm_comms {
                i
            } else if i < num_raa_comms + num_perm_comms + extra_perm_evals {
                let offset = i - (num_raa_comms + num_perm_comms);
                let evals_per_tree = perm_terminal_queries.len() * 3;
                let tree_idx = offset / evals_per_tree;
                let slot = offset % 3;
                let query = &perm_terminal_queries[(offset % evals_per_tree) / 3];
                let tree_offset = num_raa_comms + tree_idx * num_split_chunks;
                tree_offset
                    + if slot == 2 {
                        query.parent_chunk
                    } else {
                        query.child_chunk
                    }
            } else if i == num_raa_comms + num_perm_comms + extra_perm_evals {
                3
            } else if i == num_raa_comms + num_perm_comms + extra_perm_evals + 1 {
                1
            } else {
                unreachable!("unexpected Blaze auxiliary evaluation index")
            },
            point: i,
            value: v.clone(),
        })
        .collect::<Vec<_>>();

    let mut al_rand_point = &rand_point.clone()[0..vp.split_basefold_verifier_param.num_vars];

    let mut points: Vec<Vec<B128>> = (0..(num_raa_comms + num_perm_comms))
        .map(|_| al_rand_point.to_vec())
        .collect::<Vec<_>>();
    for _tree_idx in 0..num_perm_trees {
        for query in &perm_terminal_queries {
            points.push(query.left_point.clone());
            points.push(query.right_point.clone());
            points.push(query.parent_point.clone());
        }
    }
    points.push(sumcheck_checks[1].terminal_point.clone());
    points.push(sumcheck_checks[2].terminal_point.clone());

    //create points and create eval
    let now = Instant::now();
    BlazeBasefoldPcs::<H>::batch_verify(
        &vp.split_basefold_verifier_param,
        basefold_comms,
        &points,
        &evals_al,
        b128transcript,
    )?;
    println!("batch verify {:?}", now.elapsed());

    let u1_eval_at_rand_point = evaluations[0];
    let u2_eval_at_rand_point = evaluations[1];
    let u3_eval_at_rand_point = evaluations[2];
    let u4_eval_at_rand_point = evaluations[3];
    let f1_zero_at_rand_point = evaluations[num_raa_comms];
    let f2_zero_at_rand_point = evaluations[num_raa_comms + 2];
    let g1_zero_at_rand_point = evaluations[num_raa_comms + 4];
    let g2_zero_at_rand_point = evaluations[num_raa_comms + 6];
    let permutation1_at_rand_point =
        permutation_mle_eval(&vp.permutation.permutation1, &rand_point);
    let permutation2_at_rand_point =
        permutation_mle_eval(&vp.permutation.permutation2, &rand_point);
    let identity_at_rand_point = identity_mle_eval(&rand_point);
    if f1_zero_at_rand_point
        != perm_alpha - (u1_eval_at_rand_point + perm_beta * permutation1_at_rand_point)
        || f2_zero_at_rand_point
            != perm_alpha - (u2_eval_at_rand_point + perm_beta * identity_at_rand_point)
        || g1_zero_at_rand_point
            != perm_alpha - (u3_eval_at_rand_point + perm_beta * permutation2_at_rand_point)
        || g2_zero_at_rand_point
            != perm_alpha - (u4_eval_at_rand_point + perm_beta * identity_at_rand_point)
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation helper boundary check failed".to_string(),
        ));
    }

    let perm_terminal_eval_offset = num_raa_comms + num_perm_comms;
    let evals_per_tree = perm_terminal_queries.len() * 3;
    let mut expected_perm_terminal_claim = B128::ZERO;
    for tree_idx in 0..num_perm_trees {
        let tree_eval_offset = perm_terminal_eval_offset + tree_idx * evals_per_tree;
        let tree_relation_eval = perm_terminal_queries
            .iter()
            .enumerate()
            .map(|(query_idx, query)| {
                let offset = tree_eval_offset + query_idx * 3;
                let left = evaluations[offset];
                let right = evaluations[offset + 1];
                let parent = evaluations[offset + 2];
                query.selector * (parent - left * right)
            })
            .sum::<B128>();
        expected_perm_terminal_claim += coeffs[tree_idx] * tree_relation_eval;
    }
    let eq_at_perm_terminal =
        MultilinearPolynomial::new(build_eq_x_r_vec::<B128>(&rand_point).unwrap())
            .evaluate(&sumcheck_checks[0].terminal_point);
    if sumcheck_checks[0].terminal_claim != eq_at_perm_terminal * expected_perm_terminal_claim {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation sumcheck terminal opening check failed".to_string(),
        ));
    }

    let accum_poly = accumulation_mle(&rand_point);
    let u4_terminal_eval = evaluations[num_raa_comms + num_perm_comms + extra_perm_evals];
    let u2_terminal_eval = evaluations[num_raa_comms + num_perm_comms + extra_perm_evals + 1];
    if sumcheck_checks[1].terminal_claim
        != u4_terminal_eval
            * MultilinearPolynomial::new(accum_poly.poly.clone())
                .evaluate(&sumcheck_checks[1].terminal_point)
        || sumcheck_checks[2].terminal_claim
            != u2_terminal_eval
                * MultilinearPolynomial::new(accum_poly.poly)
                    .evaluate(&sumcheck_checks[2].terminal_point)
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze accumulation sumcheck terminal opening check failed".to_string(),
        ));
    }

    let folded_codeword_comm =
        BlazeBasefoldPcs::<H>::read_commitment(&vp.reg_basefold_verifier_param, b128transcript)?;

    let q_challenges = b128transcript.squeeze_challenges(vp.num_queries);
    let row_len = 1 << (vp.num_vars + vp.log_rate);
    let folded_codeword_evals = b128transcript.read_field_elements(vp.num_queries + 1)?;
    let mut folded_codeword_points = Vec::with_capacity(vp.num_queries + 1);
    folded_codeword_points.push(rand_point.clone());
    folded_codeword_points.extend(q_challenges.iter().map(|x_index| {
        let x_rep = (*x_index).to_repr();
        let x: &[u8] = x_rep.as_ref();
        let (int_bytes, _) = x.split_at(std::mem::size_of::<u32>());
        let x_int: u32 = u32::from_be_bytes(int_bytes.try_into().unwrap());
        index_to_hypercube_point(
            (x_int as usize) % row_len,
            vp.reg_basefold_verifier_param.num_vars,
        )
    }));
    let folded_codeword_eval_claims = folded_codeword_evals
        .iter()
        .enumerate()
        .map(|(point, value)| Evaluation {
            poly: 0,
            point,
            value: *value,
        })
        .collect::<Vec<_>>();
    BlazeBasefoldPcs::<H>::batch_verify(
        &vp.reg_basefold_verifier_param,
        iter::once(&folded_codeword_comm),
        &folded_codeword_points,
        &folded_codeword_eval_claims,
        b128transcript,
    )?;

    let u5_eval_at_rand_point = folded_codeword_evals[0];
    if sumcheck_checks[1].initial_sum != u5_eval_at_rand_point {
        return Err(Error::InvalidPcsOpen(
            "Blaze u4->u5 accumulation sumcheck does not match opened u5 value".to_string(),
        ));
    }
    if sumcheck_checks[2].initial_sum != u3_eval_at_rand_point {
        return Err(Error::InvalidPcsOpen(
            "Blaze u2->u3 accumulation sumcheck does not match opened u3 value".to_string(),
        ));
    }

    //read blaze query transcript
    let mut count = 0;
    let mut paths = Vec::new();

    (0..vp.num_queries).for_each(|i| {
        count = count + 1;
        paths.push(
            blazetranscript
                .read_commitments(vp.num_vars + vp.log_rate - 1)
                .unwrap(),
        );
    });
    let now = Instant::now();
    let queries: Vec<Vec<F>> = blazetranscript
        .read_field_elements(vp.num_queries * vp.num_rows * 2)
        .unwrap()
        .par_chunks_exact(vp.num_rows * 2)
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();
    /*  let queries: Vec<Vec<F>> = (0..vp.num_queries).map(|q|{
        let res = blazetranscript.read_field_elements(vp.num_rows >> 1).unwrap();
        res
    }).collect::<Vec<_>>();*/
    println!("read field element time {:?}", now.elapsed());
    let queries_usize: Vec<usize> = q_challenges
        .par_iter()
        .map(|x_index| {
            let x_rep = (*x_index).to_repr();
            let mut x: &[u8] = x_rep.as_ref();
            let (int_bytes, rest) = x.split_at(std::mem::size_of::<u32>());
            let x_int: u32 = u32::from_be_bytes(int_bytes.try_into().unwrap());
            ((x_int as usize) % row_len).into()
        })
        .collect::<Vec<_>>();

    //check the merkle path
    let now = Instant::now();
    (0..vp.num_queries).into_par_iter().try_for_each(|q| {
        authenticate_blaze_merkle_sibling_path::<H, F>(
            &paths[q],
            queries_usize[q],
            &queries[q],
            &blaze_root,
        )
    })?;
    println!("verify merkle paths {:?}", now.elapsed());

    let now = Instant::now();
    let sums = queries
        .iter()
        .enumerate()
        .map(|(q_idx, q)| {
            let b128vec = query_selected_b128_values::<F>(q, queries_usize[q_idx], vp.num_rows)?;
            Ok(inner_product(b128vec.iter(), &challenges))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    if sums != folded_codeword_evals[1..] {
        return Err(Error::InvalidPcsOpen(
            "Blaze folded query values do not match folded codeword openings".to_string(),
        ));
    }
    println!("linear combo {:?}", now.elapsed());
    Ok(())
}

fn write_merkle_path<H: Hash, F: BlazeField>(
    tree: &Vec<Vec<Output<H>>>,
    mut x_index: usize,
    transcript: &mut impl TranscriptWrite<Output<H>, F>,
) {
    x_index >>= 1;
    for oracle in tree {
        if (oracle.len() == 1) {
            transcript.write_commitment(&oracle[0]);
            break;
        }
        let mut p0 = x_index;
        let mut p1 = x_index ^ 1;
        if p1 < p0 {
            p0 = x_index ^ 1;
            p1 = x_index;
        }
        transcript.write_commitment(&oracle[p0]);
        transcript.write_commitment(&oracle[p1]);
        x_index >>= 1;
    }
}

fn write_merkle_sibling_path<H: Hash, F: BlazeField>(
    tree: &Vec<Vec<Output<H>>>,
    mut x_index: usize,
    transcript: &mut impl TranscriptWrite<Output<H>, F>,
) {
    x_index >>= 1;
    for oracle in tree {
        if oracle.len() == 1 {
            break;
        }
        transcript.write_commitment(&oracle[x_index ^ 1]);
        x_index >>= 1;
    }
}

fn authenticate_merkle_path<H: Hash>(
    path: &Vec<Vec<Output<H>>>,
    mut x_index: usize,
    root: &Output<H>,
) -> Result<(), Error> {
    x_index >>= 1;
    if path.is_empty() || path[path.len() - 1].len() != 1 || &path[path.len() - 1][0] != root {
        return Err(Error::InvalidPcsOpen(
            "Blaze Merkle path root mismatch".to_string(),
        ));
    }
    for i in 0..path.len() - 1 {
        if path[i].len() != 2 {
            return Err(Error::InvalidPcsOpen(
                "Blaze Merkle path layer has invalid width".to_string(),
            ));
        }
        let mut hasher = H::new();
        let mut hash = Output::<H>::default();
        hasher.update(&path[i][0]);
        hasher.update(&path[i][1]);
        hasher.finalize_into_reset(&mut hash);

        if hash != path[i + 1][(x_index >> 1) & 1] {
            return Err(Error::InvalidPcsOpen(
                "Blaze Merkle path hash mismatch".to_string(),
            ));
        }
        x_index >>= 1;
    }
    Ok(())
}

fn authenticate_blaze_merkle_sibling_path<H: Hash, F: BlazeField>(
    path: &[Output<H>],
    mut x_index: usize,
    leaf_values: &[F],
    root: &Output<H>,
) -> Result<(), Error> {
    if leaf_values.len() % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze query leaf has invalid width".to_string(),
        ));
    }

    let mut hasher = H::new();
    for row_pair in leaf_values.chunks_exact(2) {
        hasher.update_blaze_field(&row_pair[0]);
        hasher.update_blaze_field(&row_pair[1]);
    }
    let mut hash = hasher.finalize_fixed();

    x_index >>= 1;
    for sibling in path {
        let mut hasher = H::new();
        if x_index & 1 == 0 {
            hasher.update(&hash);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(&hash);
        }
        hash = hasher.finalize_fixed();
        x_index >>= 1;
    }

    if &hash != root {
        return Err(Error::InvalidPcsOpen(
            "Blaze Merkle path root mismatch".to_string(),
        ));
    }
    Ok(())
}

fn verify_blaze_query_leaf<F: BlazeField, H: Hash>(
    path: &Vec<Vec<Output<H>>>,
    x_index: usize,
    leaf_values: &[F],
) -> Result<(), Error> {
    if leaf_values.len() % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze query leaf has invalid width".to_string(),
        ));
    }
    if path.is_empty() || path[0].len() != 2 {
        return Err(Error::InvalidPcsOpen(
            "Blaze query path has no leaf pair".to_string(),
        ));
    }
    let selected_leaf_index = (x_index >> 1) & 1;
    let mut hasher = H::new();
    for row_pair in leaf_values.chunks_exact(2) {
        hasher.update_blaze_field(&row_pair[0]);
        hasher.update_blaze_field(&row_pair[1]);
    }
    let selected_leaf_hash = hasher.finalize_fixed();
    if selected_leaf_hash != path[0][selected_leaf_index] {
        return Err(Error::InvalidPcsOpen(
            "Blaze query leaf does not match Merkle path".to_string(),
        ));
    }
    Ok(())
}

fn query_selected_b128_values<F: BlazeField>(
    leaf_values: &[F],
    x_index: usize,
    num_rows: usize,
) -> Result<Vec<B128>, Error> {
    if leaf_values.len() != num_rows * 2 || num_rows % 2 != 0 {
        return Err(Error::InvalidPcsOpen(
            "Blaze query leaf has invalid row width".to_string(),
        ));
    }
    let pair_offset = x_index & 1;
    Ok(leaf_values
        .chunks_exact(4)
        .map(|rows| F::to_b128_vec(vec![rows[pair_offset], rows[2 + pair_offset]]))
        .collect())
}

//TODO: What should the type of u5_point be?
fn check_fold_u5<EF, F: BlazeField, H: Hash>(
    comm: &BlazeCommitment<F, H>,
    u5_eval_point: Vec<F>,
    transcript: &mut impl TranscriptRead<CommitmentChunk<H>, EF>,
) -> bool {
    //read merkle paths from transcript
    //check linear combination is equal to u5_point (this point should be on boolean hypercube)
    todo!()
}

fn b128_to_bf<F: BlazeField>(data: &Vec<B128>) -> Vec<Vec<F>> {
    let new_data = data
        .into_par_iter()
        .map(|d| F::from_b128(*d))
        .collect::<Vec<_>>();
    transpose(&new_data)
}
fn bf_to_b128_vec<F: BlazeField>(data: &Vec<F>) -> Vec<B128> {
    data.par_iter().map(|x| x.to_b128()).collect::<Vec<_>>()
}
fn bf_to_b128_vec_long<F: BlazeField>(data: &Vec<Vec<F>>) -> Vec<B128> {
    assert_eq!(data.len(), 2);
    let new_data = transpose(&data);
    new_data
        .par_iter()
        .map(|x| F::to_b128_vec(x.to_vec()))
        .collect::<Vec<B128>>()
}

fn bits_to_b128<F: BlazeField>(data: &Vec<Vec<F>>) -> Vec<B128> {
    assert_eq!(data.len(), 128);
    let new_data = transpose(&data);
    new_data
        .par_iter()
        .map(|x| F::to_b128_vec(x.to_vec()))
        .collect::<Vec<B128>>()
}

fn bf_to_long_b128_vec_long<F: BlazeField>(data: &Vec<Vec<F>>) -> Vec<Vec<B128>> {
    let new_data = transpose(&data);
    let nnd = new_data
        .par_iter()
        .map(|x| {
            x.chunks_exact(2)
                .map(|chunk| F::to_b128_vec(chunk.to_vec()))
                .collect::<Vec<B128>>()
        })
        .collect::<Vec<_>>();
    transpose(&nnd)
}

fn usize_to_b128_vec(data: &Vec<usize>) -> Vec<B128> {
    let reslt = data
        .par_iter()
        .map(|x| B128::from(*x as u64))
        .collect::<Vec<_>>();
    assert_eq!(data.len(), reslt.len());
    reslt
}

fn identity_mle_eval(point: &[B128]) -> B128 {
    MultilinearPolynomial::new(
        (0..(1 << point.len()))
            .map(|i| B128::from(i as u64))
            .collect::<Vec<_>>(),
    )
    .evaluate(point)
}

fn permutation_mle_eval(permutation: &[usize], point: &[B128]) -> B128 {
    assert_eq!(permutation.len(), 1 << point.len());
    MultilinearPolynomial::new(
        permutation
            .iter()
            .map(|i| B128::from(*i as u64))
            .collect::<Vec<_>>(),
    )
    .evaluate(point)
}

//while f has m+1 variables, f_combo has only m variables
//permutation should be an option - you should sometimes just replace beta *perm_poly with beta * 1 (TODO)
fn build_permutation_polynomials(
    permutation: Option<&Vec<usize>>,
    poly: &MultilinearPolynomial<B128>,
    beta: B128,
    alpha: B128,
) -> (MultilinearPolynomial<B128>, MultilinearPolynomial<B128>) {
    let len = poly.evals.len() >> 1;

    //construct f0,
    let mut f0 = vec![B128::zero(); len];
    match permutation {
        Some(permutation) => {
            assert_eq!(permutation.len(), len);
            f0.par_iter_mut().enumerate().for_each(|(i, x)| {
                *x = alpha - (poly.evals[i] + beta * B128::from(permutation[i] as u64));
            });
        }
        None => {
            f0.par_iter_mut().enumerate().for_each(|(i, x)| {
                *x = alpha - (poly.evals[i] + beta * B128::from(i as u64));
            });
        }
    }

    let tree = permutation_check::ProductTree::new(f0)
        .expect("Blaze permutation helper leaves are non-empty and power-of-two");
    (tree.packed_witness_poly(), tree.relation_poly())
}
#[cfg(feature = "upstream-tests")]
#[test]
fn test_perm() {
    let num_vars = 10;
    let psize: u64 = 1 << 10; //row_size = num_cols
    let col_size = 64; //num rows
    let mut rng = OsRng;
    let mut rng2 = ChaCha8Rng::from_entropy();
    let params = setup::<Blake2s>(psize as usize, 1, &mut rng, None, None);
    let (pp, vp) = trim::<Blake2s>(&params, psize as usize, 1);
    let polys = iter::repeat_with(|| MultilinearPolynomial::rand(num_vars, OsRng))
        .take(2)
        .collect_vec();
    let al = B128::random(&mut rng);
    let be = B128::random(&mut rng);
    build_permutation_polynomials(
        Some(&pp.permutation.permutation1),
        &polys[0].clone(),
        al,
        be,
    );
}
#[cfg(feature = "upstream-tests")]
#[test]
fn test_basefold_binary() {
    use crate::backend::binary_extension_fields::B128;
    use crate::backend::new_fields::Mersenne127;
    use crate::backend::pcs::PolynomialCommitmentScheme;
    use crate::backend::poly::multilinear::MultilinearPolynomial;
    use crate::backend::transcript::Blake2sTranscript;
    use blake2::Blake2s256;
    type Pcs = BlazeBasefoldPcs<Blake2s256>;

    // rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    let num_vars = 3;

    // Setup
    let (pp, vp) = {
        let mut rng = OsRng;
        let poly_size = 1 << num_vars;
        let param = Pcs::setup(poly_size, 1, &mut rng).unwrap();

        Pcs::trim(&param, poly_size, 1).unwrap()
    };

    let proof = {
        let mut transcript = Blake2sTranscript::new(());
        let poly = MultilinearPolynomial::rand(num_vars, OsRng);
        let now = Instant::now();

        let comm = Pcs::commit_and_write(&pp, &poly, &mut transcript).unwrap();
        println!("comm time {:?}", now.elapsed());
        let point = transcript.squeeze_challenges(num_vars);
        let eval = poly.evaluate(point.as_slice());
        transcript.write_field_element(&eval).unwrap();
        let now2 = Instant::now();
        Pcs::open(&pp, &poly, &comm, &point, &eval, &mut transcript).unwrap();
        println!("proximity time {:?}", now2.elapsed());

        transcript.into_proof()
    };
    let result = {
        let mut transcript = Blake2sTranscript::from_proof((), proof.as_slice());
        Pcs::verify(
            &vp,
            &Pcs::read_commitment(&vp, &mut transcript).unwrap(),
            &transcript.squeeze_challenges(num_vars),
            &transcript.read_field_element().unwrap(),
            &mut transcript,
        )
    };
    assert_eq!(result, Ok(()));
    //  println!("proof{:?}", proof);
}
#[cfg(feature = "upstream-tests")]
#[test]
fn test_batch_basefold_binary() {
    use crate::backend::binary_extension_fields::B128;
    use crate::backend::new_fields::Mersenne127;
    use crate::backend::pcs::multilinear::test::{
        run_batch_commit_open_verify, run_commit_open_verify,
    };
    use crate::backend::pcs::PolynomialCommitmentScheme;
    use crate::backend::poly::multilinear::MultilinearPolynomial;
    use crate::backend::transcript::Blake2sTranscript;
    use blake2::Blake2s256;
    type Pcs = BlazeBasefoldPcs<Blake2s256>;

    run_batch_commit_open_verify::<_, Pcs, Blake2sTranscript<_>>();
}
#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large transpose timing experiment; default tests should stay under one second"]
fn test_transpose() {
    let psize: u64 = 1 << 23;
    let mut rng2 = ChaCha8Rng::from_entropy();
    let col_size = 64;
    let mut data = Vec::new();
    for i in 0..col_size {
        data.push(Blazeu64::rand_vec(psize as usize));
    }
    let now = Instant::now();
    let transposed: Vec<Vec<_>> = transpose(&data);
    println!("time to transpose {:?}", now.elapsed());
}

fn transpose<F: std::marker::Send + std::marker::Sync + Copy>(data: &Vec<Vec<F>>) -> Vec<Vec<F>> {
    let rows = data.len();
    let cols = data[0].len();
    let now = Instant::now();
    (0..cols)
        .into_par_iter()
        .map(|col| (0..rows).map(|row| data[row][col]).collect())
        .collect()
}

#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large RAA timing sweep; default tests should stay under one second"]
fn bench_raa() {
    for k in 10..30 {
        bench_single_raa_aux(k);
    }
}
#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large Blaze commit timing sweep; default tests should stay under one second"]
fn bench_commit() {
    for k in 20..21 {
        bench_commit_aux(k, 64);
        bench_commit_aux(k, 128);
        bench_commit_aux(k, 256);
        bench_commit_aux(k, 512);
        bench_commit_aux(k, 1024);
        bench_commit_aux(k, 2048);
    }
}
#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large Blaze fast-commit timing sweep; default tests should stay under one second"]
fn bench_fast_commit() {
    for k in 10..11 {
        bench_fast_commit_aux(k, 64);
        bench_fast_commit_aux(k, 128);
        bench_fast_commit_aux(k, 256);
        bench_fast_commit_aux(k, 512);
        bench_fast_commit_aux(k, 1024);
        bench_fast_commit_aux(k, 2048);
    }
}

fn bench_single_raa_aux(k: usize) {
    let mut data = Vec::new();
    let psize = 1 << k;
    data = Blazeu64::rand_vec(psize as usize);

    let now = Instant::now();
    let poly_size = data.len();
    let log_rate = 2;
    let num_vars = log2_strict(poly_size);
    let mut rng: ChaCha8Rng = ChaCha8Rng::from_entropy(); //TODO - use RngCore instead so it can be passed in
    let permutation = Permutation::create(&mut rng, (poly_size * (1 << log_rate)));

    let now = Instant::now();
    let codeword = encode_bits_ser(
        //TODO - do this with two distinct permutations
        data,
        &permutation,
        (1 << log_rate),
    );
    println!("encoding time {:?}", now.elapsed());
    assert_eq!(codeword.len(), psize * (1 << log_rate));
}
fn bench_commit_aux(k: usize, col_size: usize) {
    let psize: u64 = 1 << (k - 6); //row_size = num_cols

    let mut rng2 = ChaCha8Rng::from_entropy();
    let params = setup::<Blake2s>(psize as usize, 1, &mut rng2, Some(col_size), Some(1004));
    let (pp, vp) = trim::<Blake2s>(&params, psize as usize, 1);

    let mut data = Vec::new();
    for i in 0..col_size {
        data.push(Blazeu64::rand_vec(psize as usize));
    }
    let now = Instant::now();
    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let com: BlazeCommitment<Blazeu64, Blake2s> =
        commit_and_write(&pp, &data, &mut blaze_transcript);
    println!("commit time {:?}", now.elapsed());
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = iter::repeat_with(|| b128_transcript.squeeze_challenges(pp.num_vars))
        .take(1)
        .collect_vec();
    let now = Instant::now();
    let eval = open(
        &pp,
        &data,
        &com,
        &point[0],
        &B128::zero(),
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    println!("open time {:?} for {:?} columns", now.elapsed(), col_size);

    let bl_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();
    let mut blazetranscript = BlazeBlake2sTranscript::from_proof((), bl_proof.as_slice());
    let mut b128transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());

    let now = Instant::now();
    verify(
        &vp,
        &com,
        &point[0],
        &eval,
        &mut b128transcript,
        &mut blazetranscript,
    );

    println!("verify time {:?}", now.elapsed());
}
fn bench_fast_commit_aux(k: usize, col_size: usize) {
    let psize: u64 = 1 << (k - 6); //row_size = num_cols

    let mut rng2 = ChaCha8Rng::from_entropy();
    let params = setup::<Blake2s>(psize as usize, 1, &mut rng2, Some(col_size), Some(1004));
    let (pp, vp) = trim::<Blake2s>(&params, psize as usize, 1);

    let mut data = Vec::new();
    for i in 0..col_size {
        data.push(Blazeu64::rand_vec(psize as usize));
    }
    let now = Instant::now();
    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let com: BlazeCommitment<Blazeu64, Blake2s> =
        commit_and_write(&pp, &data, &mut blaze_transcript);
    println!("commit time {:?}", now.elapsed());
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = iter::repeat_with(|| b128_transcript.squeeze_challenges(pp.num_vars))
        .take(1)
        .collect_vec();
    let now = Instant::now();
    let eval = faster_open(
        &pp,
        &data,
        &com,
        &point[0],
        &B128::zero(),
        &mut blaze_transcript,
        &mut b128_transcript,
    )
    .unwrap();

    println!("open time {:?} for {:?} columns", now.elapsed(), col_size);

    let bl_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();
    let mut blazetranscript = BlazeBlake2sTranscript::from_proof((), bl_proof.as_slice());
    let mut b128transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());

    let now = Instant::now();
    faster_verify(
        &vp,
        &com,
        &point[0],
        &eval,
        &mut b128transcript,
        &mut blazetranscript,
    );

    println!("verify time {:?}", now.elapsed());
}
// #[test]
// fn bench_commit() {
//     let psize: u64 = 1 << 15;
//     let mut rng = OsRng;
//     let mut rng2 = ChaCha8Rng::from_entropy();
//     let params = setup(psize as usize, 1, &mut rng);
//     let (pp, vp) = trim(&params, psize as usize, 1);

//     let data = Blazeu64::rand_vec(&mut rng2, psize as usize);
//     let now = Instant::now();
//     println!("data {:?}", data.len());
//     let com = commit::<Blazeu64, Keccak256>(&pp, data);
//     println!("commit time {:?}", now.elapsed());

//     assert_eq!(com.codeword.len(), (psize * 4) as usize);
// }

// #[test]
// fn test_stdout() {
//     use inline_c::assert_c;
//     (assert_c! {
//         #include <stdio.h>
//         #include <stdlib.h>

//         int main() {
//             int *p1 = malloc((1<<36)*sizeof(int));  // allocates enough for an array of 4 int
//             int *p2 = malloc(sizeof(int[4])); // same, naming the type directly
//             int *p3 = malloc(4*sizeof *p3);   // same, without repeating the type name
//         free(p1);
//         free(p2);
//         free(p3);
//         return 0;
//         }
//     })
//     .success();
// }

#[cfg(feature = "upstream-tests")]
#[test]
fn test_heap_allocation() {
    //  const psize:usize = 1 << 36 + 1 << 28;
    //let test_vec:Box<[u64]> = Box::new([0u64; psize]);
    // let test_vec_2:Box<[u64]> = Box::new([0u64;1 << 28]);
    // assert_eq!(test_vec[0], 1u64);
}

fn blazefield_linear_combo<F: BlazeField>(
    challenges: &Vec<Vec<F>>,
    rows: &Vec<Vec<F>>,
    security_param: usize,
) -> Vec<Vec<u32>> {
    let num_rows = rows.len(); //aka column length
    let num_columns = rows[0].len(); //aka row length
    assert_eq!(challenges.len(), security_param);
    assert_eq!(challenges[0].len(), num_rows);
    //for each inner vec of the challenges, we want a column of the RAA matrix
    challenges
        .iter()
        .map(|c| {
            (0..num_columns)
                .into_par_iter()
                .map(|el| {
                    let col = (0..num_rows).map(|i| rows[i][el]).collect_vec();
                    let now = Instant::now();
                    let r = bit_wise_inner_product(c, &col);
                    r
                })
                .collect::<Vec<_>>()
        })
        .collect_vec()
}

fn blazefield_linear_combo_faster<F: BlazeField>(
    challenges: &Vec<Vec<F>>,
    rows: &Vec<Vec<F>>,
    security_param: usize,
) -> Vec<Vec<u32>> {
    let num_rows = rows.len(); //aka column length
    let num_columns = rows[0].len(); //aka row length
    assert_eq!(challenges.len(), security_param);
    assert_eq!(challenges[0].len(), num_rows);
    //for each inner vec of the challenges, we want a column of the RAA matrix
    challenges
        .iter()
        .map(|c| {
            (0..num_columns)
                .into_par_iter()
                .map(|el| {
                    let len = num_rows;
                    let mut hadamard: Vec<F> = Vec::new();
                    for i in 0..len {
                        hadamard.push(c[i] & rows[i][el]);
                    }
                    let mut sum = 0;
                    for elh in hadamard {
                        let parity = elh.count_ones();
                        sum = sum ^ (parity & 1);
                    }
                    sum
                })
                .collect::<Vec<_>>()
        })
        .collect_vec()
}

fn blazefield_linear_combo_even_faster<F: BlazeField>(
    challenges: &Vec<B128>,
    rows: &Vec<Vec<F>>,
    security_param: usize,
) -> Vec<B128> {
    let data = bf_to_long_b128_vec_long(rows);
    linear_combination(data, challenges.to_vec())
}

/*
fn bit_wise_inner_product<F:BlazeField>(lhs:&Vec<F>, rhs:&Vec<F>) -> u32{
    assert_eq!(rhs.len(),lhs.len());
    let len = lhs.len();
    let mut hadamard:Vec<F> = Vec::new();
    for i in 0..len{
        hadamard.push(lhs[i] & rhs[i]);
    }
    let mut sum = 0;
    for el in hadamard{
        let parity = el.count_ones();
        sum = sum ^ (parity & 1);
    }

    sum
}
*/
fn blazefield_linear_combo_transpose<F: BlazeField>(
    challenges: &Vec<Vec<F>>,
    rows: &Vec<Vec<F>>,
    security_param: usize,
) -> Vec<Vec<u32>> {
    let num_rows = rows.len(); //aka column length
    let num_columns = rows[0].len(); //aka row length
    assert_eq!(challenges.len(), security_param);
    assert_eq!(challenges[0].len(), num_rows);
    //for each inner vec of the challenges, we want a column of the RAA matrix
    let transpose = transpose(&rows);
    challenges
        .iter()
        .map(|c| {
            transpose
                .iter()
                .map(|col| {
                    let n = bit_wise_inner_product(c, col);
                    n
                })
                .collect_vec()
        })
        .collect_vec()
}

fn linear_combo_to_b128<F: BlazeField>(data: &Vec<Vec<u32>>) -> (Vec<B128>, Vec<Vec<F>>) {
    let td = transpose(data);
    assert_eq!(td.len(), data[0].len());
    assert_eq!(td[0].len(), data.len());
    assert_eq!(td[0].len(), 128);
    let mut result: Vec<B128> = Vec::new();
    let b128_vec = td
        .par_iter()
        .map(|el| {
            let mut val1 = 0u64;
            //first u64
            for i in 0..64 {
                val1 = val1 + (el[i] as u64) * (1 << i);
            }
            let mut val2 = 0u64;
            for i in 64..128 {
                val2 = val2 + (el[i] as u64) * (1 << (i - 64));
            }
            B128 {
                value: [val1, val2],
            }
        })
        .collect::<Vec<_>>();
    let mut blazefield_vec = b128_vec
        .par_iter()
        .map(|v| F::from_b128(*v))
        .collect::<Vec<_>>();
    blazefield_vec = transpose(&blazefield_vec);
    assert_eq!(blazefield_vec.len(), 2);
    (b128_vec, blazefield_vec)
}

#[cfg(feature = "upstream-tests")]
#[test]
fn test_lc_to_b128() {
    let mut data = Vec::new();
    for i in 0..128 {
        data.push(vec![0, 1, 0]);
    }
    let td = linear_combo_to_b128::<Blazeu64>(&data);
    assert_eq!(
        td.0,
        vec![
            B128::zero(),
            B128 {
                value: [u64::MAX, u64::MAX]
            },
            B128::zero()
        ]
    );
}

#[cfg(feature = "upstream-tests")]
#[test]
fn blaze_robustness_linear_combo_matches_reference_small() {
    let rows = vec![
        (0..32)
            .map(|i| Blazeu64 {
                value: 0x9e37_79b9_7f4a_7c15u64.rotate_left(i as u32),
            })
            .collect_vec(),
        (0..32)
            .map(|i| Blazeu64 {
                value: 0xd1b5_4a32_d192_ed03u64.rotate_right(i as u32),
            })
            .collect_vec(),
    ];
    let bit_challenges = (0..128)
        .map(|i| {
            vec![
                Blazeu64 {
                    value: 0xfedc_ba98_7654_3210u64.rotate_left(i as u32),
                },
                Blazeu64 {
                    value: 0x0123_4567_89ab_cdefu64.rotate_right(i as u32),
                },
            ]
        })
        .collect_vec();

    let reference = blazefield_linear_combo(&bit_challenges, &rows, 128);
    assert_eq!(
        reference,
        blazefield_linear_combo_transpose(&bit_challenges, &rows, 128)
    );
    assert_eq!(
        reference,
        blazefield_linear_combo_faster(&bit_challenges, &rows, 128)
    );
    let packed_rows = bf_to_long_b128_vec_long(&rows);
    let packed_challenges = vec![B128 {
        value: [0xfedc_ba98_7654_3210, 0x0123_4567_89ab_cdef],
    }];
    assert_eq!(
        linear_combination(packed_rows, packed_challenges.clone()),
        blazefield_linear_combo_even_faster(&packed_challenges, &rows, 128)
    );
}

#[cfg(feature = "upstream-tests")]
#[test]
fn blaze_robustness_merkle_long_matches_single_row_small() {
    let data = (0..1024)
        .map(|i| Blazeu64 {
            value: (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15),
        })
        .collect_vec();
    let rows = vec![data.clone()];

    let tree = merkelize::<Blake2s, Blazeu64>(&data);
    assert_eq!(tree, merkelize_long::<Blake2s, Blazeu64>(&rows));
    assert_eq!(tree, merkelize_long_par::<Blake2s, Blazeu64>(&rows));
}

#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large linear-combination timing experiment; default tests should stay under one second"]
fn test_linear_combo() {
    let mut col_size: usize = 2;
    let psize: usize = 1 << 20;
    let mut data = Vec::new();
    let mut challenges = Vec::new();
    let mut rng2 = ChaCha8Rng::from_entropy();
    let mut challenges2 = Vec::new();
    for i in 0..col_size {
        data.push(Blazeu64::rand_vec(psize as usize));
    }

    for j in 0..(col_size >> 1) {
        challenges2.push(B128::random(&mut rng2));
    }
    for i in 0..128 {
        challenges.push(Blazeu64::rand_vec(col_size));
    }

    //data has dimensions [col_size times psize]
    //challenges has dimensions [128 times col_size]
    let now = Instant::now();
    let result1 = blazefield_linear_combo(&challenges, &data, 128);
    println!("orig {:?}", now.elapsed());

    assert_eq!(result1.len(), 128);
    assert_eq!(result1[0].len(), psize);
    let now = Instant::now();
    let result2 = blazefield_linear_combo_transpose(&challenges, &data, 128);
    println!("transpose {:?}", now.elapsed());
    let now = Instant::now();
    let result3 = blazefield_linear_combo_faster(&challenges, &data, 128);
    println!("new {:?}", now.elapsed());
    assert_eq!(result1, result3);

    let now = Instant::now();
    assert_eq!(data[0].len(), psize);
    let result4 = blazefield_linear_combo_even_faster(&challenges2, &data, 128);
    println!("even faster? {:?}", now.elapsed());
    //   assert_eq!(result4, linear_combo_to_b128::<Blazeu64>(&result1).0);
}

fn raa_lc_long_with_transpose<F: BlazeField<IntType = u64>, H: Hash>(
    comm: &BlazeCommitment<F, H>,
    transcript: &mut impl TranscriptWrite<Output<H>, F>,
    security_param: usize,
) -> Vec<Vec<u32>>
where
    F: Sync,
{
    let num_rows = comm.codeword.len(); //aka column length
    let num_columns = comm.codeword[0].len(); //aka row length
                                              //sample challenges (these will be interpreted as bits, and we will do many linear combinations)
                                              //Each set of (64) challenges - you fold each alphabet symbol according to that set
                                              //Then we have to do this for many sets of challenges - 128 for security
                                              //Next, we pack these into extension field elements
                                              //and we create a merkle tree of the final word
    transcript
        .write_commitment(&comm.codeword_tree[0][0])
        .unwrap();
    //make this a vec of vecs, to hold more than |F| bits in each column
    let challenges: Vec<Vec<F>> = transcript
        .squeeze_challenges(security_param * num_rows)
        .chunks_exact(num_rows)
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    assert_eq!(challenges.len(), security_param);
    assert_eq!(challenges[0].len(), num_rows);
    //for each inner vec of the challenges, we want a column of the RAA matrix
    let now = Instant::now();
    let transpose = transpose(&comm.bh_evals);
    println!("transpose time {:?}", now.elapsed());
    challenges
        .iter()
        .map(|c| {
            transpose
                .iter()
                .map(|col| bit_wise_inner_product(c, col))
                .collect_vec()
        })
        .collect_vec()
}

//this actually returns a bit - test this function! - write some things out by hand and test it
fn bit_wise_inner_product<F: BlazeField>(lhs: &Vec<F>, rhs: &Vec<F>) -> u32 {
    assert_eq!(rhs.len(), lhs.len());
    let len = lhs.len();
    let mut hadamard: Vec<F> = Vec::new();
    for i in 0..len {
        hadamard.push(lhs[i] & rhs[i]);
    }
    let mut sum = 0;
    for el in hadamard {
        let parity = el.count_ones();
        sum = sum ^ (parity & 1);
    }

    sum
}
#[cfg(feature = "upstream-tests")]
#[test]
fn test_bw_inner_product() {
    //<[1,0,0,0] , [0,0,0,1]> = 0
    let lhs = vec![Blazeu64 { value: 8u64 }];
    let rhs = vec![Blazeu64 { value: 1u64 }];
    assert_eq!(bit_wise_inner_product(&lhs, &rhs), 0);
    let lhs = vec![Blazeu64 { value: 7u64 }];
    let rhs = vec![Blazeu64 { value: 7u64 }];
    assert_eq!(bit_wise_inner_product(&lhs, &rhs), 1);
}

fn query_accumulate(row_point: Vec<B128>, column_point: Vec<B128>) -> B128 {
    todo!()
}
fn create_accumulate_extension(point: Vec<B128>) -> Type1Polynomial<B128> {
    todo!()
}
fn query_permutation() {
    todo!()
}
fn create_permutation_extension() {
    todo!()
}

#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large Merkle timing experiment; default tests should stay under one second"]
fn test_merkle_tree() {
    use blake2::Blake2s256;
    let mut rng = ChaCha8Rng::from_entropy();
    let data = Blazeu64::rand_vec(1 << 20 as usize);

    let now = Instant::now();
    let tree_new = merkelize::<Blake2s256, Blazeu64>(&data);
    let root_new = tree_new.last();
    println!("time {:?}", now.elapsed());

    // let now = Instant::now();
    // let tree_old = old_merkelize::<Blake2s256, Blazeu64>(&data);
    // let root_old = tree_old.last();
    // println!("time {:?}", now.elapsed());
    // assert_eq!(root_new, root_old);
}

#[cfg(feature = "benchmark")]
#[cfg(feature = "upstream-tests")]
#[test]
#[ignore = "large Merkle comparison over 2^20 elements; default tests should stay under one second"]
fn test_merkle_long() {
    use blake2::Blake2s256;
    let mut rng = ChaCha8Rng::from_entropy();
    let data = Blazeu64::rand_vec(1 << 20 as usize);

    let tree_reg = merkelize::<Blake2s256, Blazeu64>(&data);

    let new_data = vec![data];
    let tree_new = merkelize_long_par::<Blake2s256, Blazeu64>(&new_data);

    assert_eq!(tree_reg.last(), tree_new.last());
}

fn merkelize<H: Hash, F: BlazeField>(values: &Vec<F>) -> Vec<Vec<Output<H>>> {
    let n = values.len();
    let depth = log2_strict(n);
    let depth_per_thread = ((n / current_num_threads()).ilog2()) as usize;
    let depth_recombine = depth - depth_per_thread as usize;

    // Split codeword into chunks, hash upwards in parallel as far as possible.
    let mut output = values
        .par_chunks_exact(1 << depth_per_thread as usize)
        .map(|ys| {
            // Hash first layer.
            let mut leaves = vec![Output::<H>::default(); (ys.len() >> 1)];
            leaves.iter_mut().enumerate().for_each(|(i, mut hash)| {
                let mut hasher = H::new();
                hasher.update_blaze_field(&ys[i + i]);
                hasher.update_blaze_field(&ys[i + i + 1]);
                *hash = hasher.finalize_fixed();
            });
            let mut tree = Vec::with_capacity(depth_per_thread);
            tree.push(leaves);

            // Hash subsequent layers.
            for i in 1..(depth_per_thread) {
                tree.push(hash_one_layer::<H>(&tree[i - 1]));
            }
            tree
        })
        .collect::<Vec<_>>();

    // Combine trees output by threads into one single tree.
    let mut final_tree = vec![Vec::new(); depth];
    for i in 0..(depth_per_thread) {
        for subtree in &mut output {
            final_tree[i].append(&mut subtree[i]);
        }
    }

    // Compute the last log(number of threads used) layers of the tree sequentially.
    for i in 0..(depth_recombine) {
        final_tree[i + depth_per_thread] =
            hash_one_layer::<H>(&final_tree[depth_per_thread + i - 1]);
    }
    final_tree
}

fn hash_one_layer<H: Hash>(oracle: &Vec<Output<H>>) -> Vec<Output<H>> {
    let mut hashes = vec![Output::<H>::default(); (oracle.len() >> 1)];
    hashes.iter_mut().enumerate().for_each(|(i, mut hash)| {
        let mut hasher = H::new();
        hasher.update(&oracle[i + i]);
        hasher.update(&oracle[i + i + 1]);
        *hash = hasher.finalize_fixed();
    });
    hashes
}

// Creates a Merkle tree out of a vector of l codeword of length n. At the leaf-level,
// combines the l first vector element and l second vector elements into one hash, etc.
// The result is a tree of equal size to the "merkelize" function which hashes a single
// length n codeword.
fn merkelize_long_par<H: Hash, F: BlazeField>(values: &Vec<Vec<F>>) -> Vec<Vec<Output<H>>> {
    let n = values[0].len();
    let depth = log2_strict(n);
    println!("n {:?}", n);
    println!("curr num threads {:?}", current_num_threads());
    let depth_per_thread = ((n / current_num_threads()).ilog2()) as usize;
    let depth_recombine = depth - depth_per_thread as usize;

    // Split codeword into chunks, hash upwards in parallel as far as possible.
    let mut output = (0..n)
        .collect::<Vec<usize>>()
        .par_chunks_exact(1 << (depth_per_thread) as usize)
        .map(|vec_i| {
            let mut leaves = vec![Output::<H>::default(); (vec_i.len() >> 1)];
            leaves.iter_mut().enumerate().for_each(|(i, mut hash)| {
                let mut hasher = H::new();
                for j in 0..values.len() {
                    hasher.update_blaze_field(&values[j][vec_i[0] + i + i]);
                    hasher.update_blaze_field(&values[j][vec_i[0] + i + i + 1]);
                }
                *hash = hasher.finalize_fixed();
            });
            let mut tree = Vec::with_capacity(depth_per_thread);
            tree.push(leaves);

            // Hash subsequent layers.
            for i in 1..(depth_per_thread) {
                tree.push(hash_one_layer::<H>(&tree[i - 1]));
            }
            tree
        })
        .collect::<Vec<_>>();

    // Combine trees output by threads into one single tree.
    let mut final_tree = vec![Vec::new(); depth];
    for i in 0..(depth_per_thread) {
        for subtree in &mut output {
            final_tree[i].append(&mut subtree[i]);
        }
    }

    // Compute the last log(number of threads used) layers of the tree sequentially.
    for i in 0..(depth_recombine) {
        final_tree[i + depth_per_thread] =
            hash_one_layer::<H>(&final_tree[depth_per_thread + i - 1]);
    }
    final_tree
}

fn merkelize_long<H: Hash, F: BlazeField>(values: &Vec<Vec<F>>) -> Vec<Vec<Output<H>>> {
    let log_v = log2_strict(values[0].len());
    let mut tree = Vec::with_capacity(log_v);
    let mut hashes = vec![Output::<H>::default(); (values[0].len() >> 1)];

    hashes.par_iter_mut().enumerate().for_each(|(i, mut hash)| {
        let mut hasher = H::new();
        for row in values {
            hasher.update_blaze_field(&row[i + i]);
            hasher.update_blaze_field(&row[i + i + 1]);
        }
        *hash = hasher.finalize_fixed();
    });

    tree.push(hashes);

    let now = Instant::now();
    for i in 1..(log_v) {
        let oracle = tree[i - 1]
            .par_chunks_exact(2)
            .map(|ys| {
                let mut hasher = H::new();
                let mut hash = Output::<H>::default();
                hasher.update(&ys[0]);
                hasher.update(&ys[1]);
                hasher.finalize_fixed()
            })
            .collect::<Vec<_>>();

        tree.push(oracle);
    }
    tree
}
/*
impl<F,H> PolynomialCommitmentScheme for Blaze<F,H>{
    where
    F:PrimeField + Serialize + DeserializeOwned,
        H: Hash

    type Param = Blaze<F>;
    type ProverParam = BlazeProverParam<F>;
    type VerifierParam = BlazeVerifierParam<F>;
    type Polynomial = MultilinearPolynomial<F>;
    type Commitment = BlazeCommitment<F,H>;
    type CommitmentChunk = Output<H>;

    fn setup(poly_size:usize, _: usize, rng: impl RngCore) -> Result<Self::Param,Error>{
    Ok()
    }
    fn commit(pp: &Self::ProverParam, poly: &Self::Polynomial) -> Result<Self::Commitment, Error> {
    Ok()
    }
}

fn lin_check<F: PrimeField>(
    result: Vec<F>,
    op: Vec<F>,
    sparse_matrix: Vec<F>,
) -> Result<Self::Commitment, Error> {
    Ok()
}
*/
pub fn log2_strict(n: usize) -> usize {
    let res = n.trailing_zeros();
    assert!(n.wrapping_shr(res) == 1, "Not a power of two: {n}");
    // Tell the optimizer about the semantics of `log2_strict`. i.e. it can replace `n` with
    // `1 << res` and vice versa.

    res as usize
}

fn interpolate_over_boolean_hypercube_with_copy<F: BlazeField>(
    evals: &BlazeType2Polynomial<F>,
) -> (BlazeType2Polynomial<F>, BlazeType1Polynomial<F>) {
    //iterate over array, replacing even indices with (evals[i] - evals[(i+1)])
    let n = log2_strict(evals.poly.len());
    let mut coeffs = vec![F::zero(); evals.poly.len()];
    let mut new_evals = vec![F::zero(); evals.poly.len()];

    let mut j = 0;
    while (j < coeffs.len()) {
        new_evals[j] = evals.poly[j];
        new_evals[j + 1] = evals.poly[j + 1];

        coeffs[j + 1] = evals.poly[j + 1] ^ evals.poly[j];
        coeffs[j] = evals.poly[j];
        j += 2
    }

    for i in 2..n + 1 {
        let chunk_size = 1 << i;
        coeffs.par_chunks_mut(chunk_size).for_each(|chunk| {
            let half_chunk = chunk_size >> 1;
            for j in half_chunk..chunk_size {
                chunk[j] = chunk[j] ^ chunk[j - half_chunk];
            }
        });
    }
    reverse_index_bits_in_place(&mut new_evals);
    (
        BlazeType2Polynomial { poly: coeffs },
        BlazeType1Polynomial { poly: new_evals },
    )
}

//The most fundamental building block of basefold is the polynomial. Currently, polynomial's are just expressed as vectors of field elements. Sometimes, vectors are in coefficient form and sometimes they are in evaluation form. Additionally, many functions make assumptions on the order of evaluations of a polynomial. There are two orderings that are used. The first ordering (which we label Type1Polynomial) places "folding pairs" next to each other, for increased parallelizability. eg, the evaluations are as follows:

//Type1Polynomial: vec![P(x1,y1,z1), P(x1,y1,z2), P(x1,y2,z3), P(x1,y2,z4), P(x2,y3,z5), P(x2,y3,z6), P(x2,y4,z7), P(x2,y4,z8)]

//Type2Polynomial has it in the ordering that is described in the Basefold paper, which also lends itself well to fast encoding
//Type2Polynomial:  vec![P(x1,y1,z1), P(x2,y3,z5), P(x1,y2,z3), P(x2,y4,z7), P(x1,y1,z2), P(x2,y3,z6), P(x1,y2,z4), P(x2,y4,z8)]

//Type1Polynomial is a bit-reversal of Type2Polynomial, where the transformation can be done by the function `reverse_index_bits_in_place`, which was taken from `plonky2`.

//Finally, sometimes Type2Polynomial contains coefficients rather than evaluations, in that case it just means that when encoded, it yields evaluations in Type2 order.
//TODO: implement deserilaize for these types
#[derive(Clone, Debug, Default, Serialize, Eq, PartialEq)]
pub(crate) struct BlazeType1Polynomial<F: BlazeField> {
    pub poly: Vec<F>,
}

#[derive(Clone, Debug, Default, Serialize, Eq, PartialEq)]
pub(crate) struct BlazeType2Polynomial<F: BlazeField> {
    pub poly: Vec<F>,
}
