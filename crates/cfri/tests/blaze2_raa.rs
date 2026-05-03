use blake2::Blake2s256;
use cfri::backend::{
    arithmetic::Field,
    blaze2::{
        fold_packed_query_pair, fold_packed_rows_into, pack_interleaved_rows,
        pack_interleaved_rows_into, Blaze2RaaCommitment, Blaze2RaaQuery,
    },
    blaze_transcript::BlazeBlake2sTranscript,
    code::PackedRaaCode,
    hash::Blake2s,
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
fn blaze2_raa_folded_query_matches_encoded_folded_rows() {
    let mut rng = ChaCha8Rng::from_seed([6; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let comm =
        Blaze2RaaCommitment::<Blazeu64, Blake2s256>::commit_rows(code.clone(), &rows).unwrap();
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17), B128::from(39)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let folded_codeword = code.encode_row(&folded_message);

    for index in [0usize, 1, 6, 17, 30, 31] {
        let query = comm.query(index).unwrap();
        let pair = fold_packed_query_pair(&query.values, &challenges).unwrap();
        let pair_start = index & !1;
        assert_eq!(
            pair,
            (folded_codeword[pair_start], folded_codeword[pair_start + 1])
        );
    }
}
