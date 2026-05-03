use blake2::Blake2s256;
use cfri::backend::{
    arithmetic::Field,
    blaze2::{
        build_raa_trace, build_raa_trace_into, check_raa_folded_codeword_link, check_raa_trace_at,
        evaluate_multilinear, evaluate_packed_rows_at_point_into, fold_packed_query_pair,
        fold_packed_rows_into, pack_interleaved_rows, pack_interleaved_rows_into,
        verify_raa_trace_spot_query, Blaze2RaaCommitment, Blaze2RaaQuery, Blaze2RaaTrace,
        Blaze2RaaTraceCommitment, Blaze2RaaTraceSpotQuery,
    },
    blaze_transcript::BlazeBlake2sTranscript,
    code::PackedRaaCode,
    hash::{Blake2s, Hash},
    transcript::InMemoryTranscript as _,
};
use cfri::blaze::{BlazeField, Blazeu64, B128};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

fn rows(num_rows: usize, row_len: usize) -> Vec<Vec<Blazeu64>> {
    (0..num_rows)
        .map(|row| {
            (0..row_len)
                .map(|col| Blazeu64 {
                    value: ((row as u64 + 11) << 32) ^ (0x9e37_79b9u64 * (col as u64 + 1)),
                })
                .collect()
        })
        .collect()
}

fn packed_rows(rows: &[Vec<Blazeu64>]) -> Vec<Vec<B128>> {
    rows.chunks_exact(2)
        .map(|pair| {
            pair[0]
                .iter()
                .zip(pair[1].iter())
                .map(|(lhs, rhs)| Blazeu64::pack_pair_to_b128(*lhs, *rhs))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn raa_trace_fixture(seed: u8) -> (PackedRaaCode, Vec<B128>, Blaze2RaaTrace) {
    let mut rng = ChaCha8Rng::from_seed([seed; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17 + seed as u64), B128::from(39 + seed as u64)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let trace = build_raa_trace(&code, &folded_message).unwrap();
    (code, folded_message, trace)
}

fn tamper_opened_trace_value<H: Hash>(
    opening: &mut Blaze2RaaTraceSpotQuery<H>,
    row: usize,
    index: usize,
) {
    let pair_start = index & !1;
    let value_offset = row * 2 + (index & 1);
    for query in &mut opening.queries {
        if query.index & !1 == pair_start {
            query.values[value_offset] += B128::ONE;
            return;
        }
    }
    panic!("missing opened value");
}

#[test]
fn blaze2_raa_queries_authenticate() {
    let mut rng = ChaCha8Rng::from_seed([1; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm = Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code, &rows).unwrap();
    let public = comm.public();

    for index in 0..comm.codeword_len() {
        let query = comm.query(index).unwrap();
        query.authenticate(public.root()).unwrap();
    }
}

#[test]
fn blaze2_raa_query_transcript_roundtrip() {
    let mut rng = ChaCha8Rng::from_seed([2; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm = Blaze2RaaCommitment::<Blazeu64, Blake2s>::commit_rows(code, &rows).unwrap();

    let index = 17;
    let expected = comm.query(index).unwrap();
    let proof = {
        let mut transcript = BlazeBlake2sTranscript::new(());
        expected.write(&mut transcript).unwrap();
        transcript.into_proof()
    };

    let actual = {
        let mut transcript = BlazeBlake2sTranscript::from_proof((), proof.as_slice());
        Blaze2RaaQuery::<Blazeu64, Blake2s>::read(
            index,
            comm.num_rows(),
            comm.codeword_len(),
            &mut transcript,
        )
        .unwrap()
    };

    assert_eq!(actual, expected);
    actual.authenticate(comm.root()).unwrap();
}

#[test]
fn blaze2_raa_rejects_tampered_query_value() {
    let mut rng = ChaCha8Rng::from_seed([3; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm = Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code, &rows).unwrap();
    let mut query = comm.query(9).unwrap();
    query.values[0].value ^= 1;

    assert!(query.authenticate(comm.root()).is_err());
}

#[test]
fn blaze2_raa_rejects_tampered_query_path() {
    let mut rng = ChaCha8Rng::from_seed([4; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm = Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code, &rows).unwrap();
    let mut query = comm.query(11).unwrap();
    query.path[0][0] ^= 1;

    assert!(query.authenticate(comm.root()).is_err());
}

#[test]
fn blaze2_raa_rejects_malformed_query_value_count() {
    let mut rng = ChaCha8Rng::from_seed([5; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm = Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code, &rows).unwrap();
    let mut query = comm.query(11).unwrap();
    query.values.push(Blazeu64 { value: 123 });

    assert!(query.authenticate(comm.root()).is_err());
}

#[test]
fn blaze2_interleaving_pack_into_matches_allocating_pack() {
    let rows = rows(4, 8);
    let expected = packed_rows(&rows);
    let actual = pack_interleaved_rows(&rows).unwrap();
    assert_eq!(actual, expected);

    let mut out = vec![vec![B128::ZERO; 8]; 2];
    pack_interleaved_rows_into(&rows, &mut out).unwrap();
    assert_eq!(out, expected);
}

#[test]
fn blaze2_interleaving_fold_into_matches_manual_linear_combination() {
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded = vec![B128::ZERO; 8];
    fold_packed_rows_into(&packed, &challenges, &mut folded).unwrap();

    for col in 0..8 {
        assert_eq!(
            folded[col],
            challenges[0] * packed[0][col] + challenges[1] * packed[1][col]
        );
    }
}

#[test]
fn blaze2_interleaving_rejects_bad_shapes() {
    let odd_rows = rows(3, 8);
    assert!(pack_interleaved_rows(&odd_rows).is_err());

    let mut ragged_rows = rows(4, 8);
    ragged_rows[2].pop();
    assert!(pack_interleaved_rows(&ragged_rows).is_err());

    let rows = rows(4, 8);
    let mut bad_out = vec![vec![B128::ZERO; 7]; 2];
    assert!(pack_interleaved_rows_into(&rows, &mut bad_out).is_err());

    let packed = pack_interleaved_rows(&rows).unwrap();
    let mut folded = vec![B128::ZERO; 8];
    assert!(fold_packed_rows_into(&packed, &[B128::from(1)], &mut folded).is_err());
}

#[test]
fn blaze2_multilinear_eval_matches_z_to_a_formula_small() {
    let values = vec![B128::from(3), B128::from(5), B128::from(7), B128::from(11)];
    let point = vec![B128::from(13), B128::from(17)];
    let mut scratch = vec![B128::ZERO; values.len()];
    let eval = evaluate_multilinear(&values, &point, &mut scratch).unwrap();

    assert_eq!(
        eval,
        values[0] + point[0] * values[2] + point[1] * values[1] + point[0] * point[1] * values[3]
    );
}

#[test]
fn blaze2_packed_row_evals_match_individual_multilinear_evals() {
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let point = vec![B128::from(2), B128::from(3), B128::from(5)];
    let mut row_evals = vec![B128::ZERO; packed.len()];
    let mut scratch = vec![B128::ZERO; packed[0].len()];
    evaluate_packed_rows_at_point_into(&packed, &point, &mut row_evals, &mut scratch).unwrap();

    let mut expected_scratch = vec![B128::ZERO; packed[0].len()];
    for row in 0..packed.len() {
        assert_eq!(
            row_evals[row],
            evaluate_multilinear(&packed[row], &point, &mut expected_scratch).unwrap()
        );
    }
}

#[test]
fn blaze2_multilinear_eval_rejects_bad_shapes() {
    let values = vec![B128::from(3), B128::from(5), B128::from(7)];
    let point = vec![B128::from(13)];
    let mut scratch = vec![B128::ZERO; values.len()];
    assert!(evaluate_multilinear(&values, &point, &mut scratch).is_err());

    let values = vec![B128::from(3), B128::from(5), B128::from(7), B128::from(11)];
    let mut short_scratch = vec![B128::ZERO; values.len() - 1];
    assert!(evaluate_multilinear(&values, &point, &mut short_scratch).is_err());

    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let mut row_evals = vec![B128::ZERO; packed.len()];
    let mut scratch = vec![B128::ZERO; packed[0].len()];
    assert!(
        evaluate_packed_rows_at_point_into(&packed, &point, &mut row_evals, &mut scratch).is_err()
    );
}

#[test]
fn blaze2_raa_trace_matches_encoder_and_spot_checks_all_positions() {
    let mut rng = ChaCha8Rng::from_seed([6; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();

    let trace = build_raa_trace(&code, &folded_message).unwrap();
    assert_eq!(trace.u5, code.encode_row(&folded_message));
    for index in 0..code.codeword_len() {
        check_raa_trace_at(&code, &folded_message, &trace, index).unwrap();
    }
}

#[test]
fn blaze2_raa_trace_into_matches_allocating_builder() {
    let mut rng = ChaCha8Rng::from_seed([7; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 11))
        .collect::<Vec<_>>();
    let expected = build_raa_trace(&code, &message).unwrap();
    let mut actual = expected.clone();
    actual.u2.fill(B128::ZERO);
    actual.u3.fill(B128::ZERO);
    actual.u4.fill(B128::ZERO);
    actual.u5.fill(B128::ZERO);

    build_raa_trace_into(&code, &message, &mut actual).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn blaze2_raa_spot_checks_reject_bad_repetition_layer() {
    let mut rng = ChaCha8Rng::from_seed([8; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 13))
        .collect::<Vec<_>>();
    let mut trace = build_raa_trace(&code, &message).unwrap();
    trace.u2[5] += B128::ONE;

    assert!(check_raa_trace_at(&code, &message, &trace, 5).is_err());
}

#[test]
fn blaze2_raa_spot_checks_reject_bad_first_accumulator_layer() {
    let mut rng = ChaCha8Rng::from_seed([9; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 17))
        .collect::<Vec<_>>();
    let mut trace = build_raa_trace(&code, &message).unwrap();
    trace.u3[6] += B128::ONE;

    assert!(check_raa_trace_at(&code, &message, &trace, 6).is_err());
}

#[test]
fn blaze2_raa_spot_checks_reject_bad_second_permutation_layer() {
    let mut rng = ChaCha8Rng::from_seed([10; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 19))
        .collect::<Vec<_>>();
    let mut trace = build_raa_trace(&code, &message).unwrap();
    trace.u4[7] += B128::ONE;

    assert!(check_raa_trace_at(&code, &message, &trace, 7).is_err());
}

#[test]
fn blaze2_raa_spot_checks_reject_bad_second_accumulator_layer() {
    let mut rng = ChaCha8Rng::from_seed([11; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 23))
        .collect::<Vec<_>>();
    let mut trace = build_raa_trace(&code, &message).unwrap();
    trace.u5[8] += B128::ONE;

    assert!(check_raa_trace_at(&code, &message, &trace, 8).is_err());
}

#[test]
fn blaze2_raa_trace_rejects_bad_shapes() {
    let mut rng = ChaCha8Rng::from_seed([12; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let mut message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 29))
        .collect::<Vec<_>>();
    message.pop();
    assert!(build_raa_trace(&code, &message).is_err());

    let message = (0..code.message_len())
        .map(|i| B128::from((i as u64 + 1) * 31))
        .collect::<Vec<_>>();
    let mut trace = build_raa_trace(&code, &message).unwrap();
    trace.u4.pop();
    assert!(check_raa_trace_at(&code, &message, &trace, 0).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_authenticate_and_verify_all_positions() {
    let (code, message, trace) = raa_trace_fixture(14);
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let public = comm.public();

    assert_eq!(public.codeword_len(), code.codeword_len());
    assert_eq!(public.num_rows(), 4);
    for index in 0..code.codeword_len() {
        let opening = comm.spot_query(&code, index).unwrap();
        assert!(!opening.queries.is_empty());
        assert!(opening.queries.len() <= 3);
        opening.verify(&code, &message, public.root()).unwrap();
        verify_raa_trace_spot_query(&code, &message, public.root(), &opening).unwrap();
    }
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_tampered_opening_value() {
    let (code, message, trace) = raa_trace_fixture(15);
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let mut opening = comm.spot_query(&code, 9).unwrap();
    tamper_opened_trace_value(&mut opening, 0, 9);

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_tampered_opening_path() {
    let (code, message, trace) = raa_trace_fixture(16);
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let mut opening = comm.spot_query(&code, 11).unwrap();
    opening.queries[0].path[0][0] ^= 1;

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_missing_required_opening() {
    let (code, message, trace) = raa_trace_fixture(17);
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let index = (1..code.codeword_len())
        .find(|&index| comm.spot_query(&code, index).unwrap().queries.len() == 3)
        .unwrap();
    let mut opening = comm.spot_query(&code, index).unwrap();
    opening.queries.pop();

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_authenticated_bad_repetition_layer() {
    let (code, message, mut trace) = raa_trace_fixture(18);
    trace.u2[5] += B128::ONE;
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = comm.spot_query(&code, 5).unwrap();

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_authenticated_bad_first_accumulator_layer() {
    let (code, message, mut trace) = raa_trace_fixture(19);
    trace.u3[6] += B128::ONE;
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = comm.spot_query(&code, 6).unwrap();

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_authenticated_bad_second_permutation_layer() {
    let (code, message, mut trace) = raa_trace_fixture(20);
    trace.u4[7] += B128::ONE;
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = comm.spot_query(&code, 7).unwrap();

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_trace_spot_queries_reject_authenticated_bad_second_accumulator_layer() {
    let (code, message, mut trace) = raa_trace_fixture(21);
    trace.u5[8] += B128::ONE;
    let comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = comm.spot_query(&code, 8).unwrap();

    assert!(opening.verify(&code, &message, comm.root()).is_err());
}

#[test]
fn blaze2_raa_folded_query_matches_encoded_folded_rows() {
    let mut rng = ChaCha8Rng::from_seed([13; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm =
        Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code.clone(), &rows).unwrap();
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let folded_codeword = code.encode_row(&folded_message);
    let trace = build_raa_trace(&code, &folded_message).unwrap();
    let trace_comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();

    for index in [0usize, 1, 6, 17, 30, 31] {
        let query = comm.query(index).unwrap();
        let pair = fold_packed_query_pair(&query.values, &challenges).unwrap();
        let pair_start = index & !1;
        assert_eq!(
            pair,
            (folded_codeword[pair_start], folded_codeword[pair_start + 1])
        );

        let trace_opening = trace_comm.spot_query(&code, index).unwrap();
        trace_opening
            .verify(&code, &folded_message, trace_comm.root())
            .unwrap();
        check_raa_folded_codeword_link(&query, &challenges, &trace_opening).unwrap();
    }
}

#[test]
fn blaze2_raa_folded_codeword_link_rejects_wrong_challenge() {
    let mut rng = ChaCha8Rng::from_seed([22; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm =
        Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code.clone(), &rows).unwrap();
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let trace = build_raa_trace(&code, &folded_message).unwrap();
    let trace_comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();

    let query = comm.query(6).unwrap();
    let trace_opening = trace_comm.spot_query(&code, 6).unwrap();
    let wrong_challenges = vec![B128::from(18), B128::from(39)];

    assert!(check_raa_folded_codeword_link(&query, &wrong_challenges, &trace_opening).is_err());
}

#[test]
fn blaze2_raa_folded_codeword_link_rejects_authenticated_bad_trace_codeword() {
    let mut rng = ChaCha8Rng::from_seed([23; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm =
        Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code.clone(), &rows).unwrap();
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let mut trace = build_raa_trace(&code, &folded_message).unwrap();
    trace.u5[6] += B128::ONE;
    let trace_comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();

    let query = comm.query(6).unwrap();
    let trace_opening = trace_comm.spot_query(&code, 6).unwrap();

    assert!(check_raa_folded_codeword_link(&query, &challenges, &trace_opening).is_err());
}

#[test]
fn blaze2_raa_folded_codeword_link_rejects_mismatched_query_pair() {
    let (code, folded_message, trace) = raa_trace_fixture(24);
    let rows = rows(4, 8);
    let comm =
        Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code.clone(), &rows).unwrap();
    let trace_comm = Blaze2RaaTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let query = comm.query(0).unwrap();
    let trace_opening = trace_comm.spot_query(&code, 2).unwrap();
    let challenges = vec![B128::from(41), B128::from(63)];

    assert_eq!(folded_message.len(), code.message_len());
    assert!(check_raa_folded_codeword_link(&query, &challenges, &trace_opening).is_err());
}
