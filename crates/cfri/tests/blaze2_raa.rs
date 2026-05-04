use blake2::Blake2s256;
use cfri::backend::{
    arithmetic::{Field, PrimeField},
    blaze2::{
        absorb_blaze2_opening_folded_eval, absorb_blaze2_opening_public,
        absorb_blaze2_opening_public_with_code_spec, absorb_blaze2_opening_row_evals,
        build_raa_aux_trace, build_raa_trace, build_raa_trace_into, check_raa_folded_codeword_link,
        check_raa_trace_at, evaluate_multilinear, evaluate_packed_matrix_at_point,
        evaluate_packed_matrix_at_point_into, evaluate_packed_rows_at_point_into,
        fold_interleaved_column, fold_packed_query_pair, fold_packed_rows_into,
        pack_interleaved_rows, pack_interleaved_rows_into, prove_blaze2_basefold_opening,
        prove_blaze2_opening, prove_blaze2_opening_with_code_spec, raa_codeword_eval_weights,
        squeeze_blaze2_opening_folding_challenges, squeeze_blaze2_opening_query_indices,
        verify_blaze2_basefold_opening, verify_blaze2_opening,
        verify_blaze2_opening_with_code_spec, verify_raa_aux_trace_spot_query,
        verify_raa_trace_spot_query, Blaze2BaseFoldOpeningProof, Blaze2Code, Blaze2CodeSeed,
        Blaze2CodeSpec, Blaze2FieldId, Blaze2FoldedMessageBackend, Blaze2FoldedMessageOpenRequest,
        Blaze2HashId, Blaze2InterleavedCodewordCommitment, Blaze2InterleavedColumnQuery,
        Blaze2LeafLayout, Blaze2OpeningClaim, Blaze2OpeningProof, Blaze2PackingLayout,
        Blaze2RaaAuxTrace, Blaze2RaaAuxTraceCommitment, Blaze2RaaCommitment, Blaze2RaaQuery,
        Blaze2RaaTrace, Blaze2RaaTraceCommitment, Blaze2RaaTraceSpotQuery, RaaVariant,
    },
    blaze_transcript::BlazeBlake2sTranscript,
    code::PackedRaaCode,
    hash::{Blake2s, Hash, Output},
    systematic_basefold::{
        required_blaze2_basefold_auxiliary_oracle_len, required_blaze2_basefold_eval_binding_len,
        required_blaze2_basefold_relation_auxiliary_len, BackendProofQueryDomain,
        Blaze2BaseFoldBackendParams, Blaze2BaseFoldBackendSpec, Blaze2BaseFoldOpenRequest,
        HolographicQuerySchedule, SystematicFoldableCodeSpec,
    },
    transcript::InMemoryTranscript as _,
    Error,
};
use cfri::blaze::{BlazeField, Blazeu64, B128};
use cfri::transcript::Transcript as CfriTranscript;
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExhaustiveFoldedMessageBackend;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExhaustiveFoldedMessageState {
    message: Vec<B128>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExhaustiveFoldedMessageProof {
    message: Vec<B128>,
}

impl Blaze2FoldedMessageBackend for ExhaustiveFoldedMessageBackend {
    type Commitment = Output<Blake2s256>;
    type ProverState = ExhaustiveFoldedMessageState;
    type Proof = ExhaustiveFoldedMessageProof;

    fn commit(message: &[B128]) -> Result<(Self::Commitment, Self::ProverState), Error> {
        Ok((
            exhaustive_folded_message_commitment(message),
            ExhaustiveFoldedMessageState {
                message: message.to_vec(),
            },
        ))
    }

    fn absorb_commitment<H: Hash, S>(
        transcript: &mut CfriTranscript<H, S>,
        commitment: &Self::Commitment,
    ) {
        transcript.absorb(commitment);
    }

    fn open(
        state: &Self::ProverState,
        request: &Blaze2FoldedMessageOpenRequest<'_>,
    ) -> Result<Self::Proof, Error> {
        let proof = ExhaustiveFoldedMessageProof {
            message: state.message.clone(),
        };
        let encoded = request.code.encode_row(&state.message);
        let input_values = request
            .input_indices
            .iter()
            .map(|&index| encoded[index])
            .collect::<Vec<_>>();
        Self::verify(
            &exhaustive_folded_message_commitment(&state.message),
            &proof,
            state.message.len(),
            request,
            &input_values,
        )?;
        Ok(proof)
    }

    fn verify(
        commitment: &Self::Commitment,
        proof: &Self::Proof,
        message_len: usize,
        request: &Blaze2FoldedMessageOpenRequest<'_>,
        input_values: &[B128],
    ) -> Result<(), Error> {
        if proof.message.len() != message_len {
            return Err(Error::InvalidPcsOpen(
                "test folded-message backend proof has wrong message length".to_string(),
            ));
        }
        if exhaustive_folded_message_commitment(&proof.message) != *commitment {
            return Err(Error::InvalidPcsOpen(
                "test folded-message backend commitment mismatch".to_string(),
            ));
        }
        let mut scratch = vec![B128::ZERO; message_len];
        let proof_eval = evaluate_multilinear(&proof.message, request.col_point, &mut scratch)?;
        if proof_eval != request.folded_eval {
            return Err(Error::InvalidPcsOpen(
                "test folded-message backend evaluation mismatch".to_string(),
            ));
        }
        if input_values.len() != request.input_indices.len() {
            return Err(Error::InvalidPcsOpen(
                "test folded-message backend input value count mismatch".to_string(),
            ));
        }
        let encoded = request.code.encode_row(&proof.message);
        for (&index, &value) in request.input_indices.iter().zip(input_values) {
            if encoded[index] != value {
                return Err(Error::InvalidPcsOpen(
                    "test folded-message backend input oracle mismatch".to_string(),
                ));
            }
        }
        Ok(())
    }
}

fn exhaustive_folded_message_commitment(message: &[B128]) -> Output<Blake2s256> {
    let mut bytes = Vec::with_capacity(16 + message.len() * 16);
    bytes.extend_from_slice(&(message.len() as u64).to_le_bytes());
    for value in message {
        bytes.extend_from_slice(value.to_repr().as_ref());
    }
    Blake2s256::digest(bytes)
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

fn paper_raa_fixture(
    seed: u8,
) -> (
    PackedRaaCode,
    Vec<Vec<B128>>,
    Vec<B128>,
    Vec<B128>,
    Blaze2RaaAuxTrace,
) {
    let mut rng = ChaCha8Rng::from_seed([seed; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let challenges = vec![B128::from(17 + seed as u64), B128::from(39 + seed as u64)];
    let mut folded_message = vec![B128::ZERO; code.message_len()];
    fold_packed_rows_into(&packed, &challenges, &mut folded_message).unwrap();
    let codeword_rows = code.encode_rows(&packed);
    let trace = build_raa_aux_trace(&code, &folded_message).unwrap();
    (code, codeword_rows, challenges, folded_message, trace)
}

fn paper_column_openings<H: Hash>(
    commitment: &Blaze2InterleavedCodewordCommitment<H>,
    index: usize,
) -> Vec<Blaze2InterleavedColumnQuery<H>> {
    let mut queries = Vec::with_capacity(2);
    queries.push(commitment.query(index).unwrap());
    if index > 0 {
        queries.push(commitment.query(index - 1).unwrap());
    }
    queries
}

fn opening_fixture(
    seed: u8,
) -> (
    PackedRaaCode,
    Blaze2CodeSeed,
    Vec<Vec<B128>>,
    Blaze2InterleavedCodewordCommitment<Blake2s256>,
    Blaze2OpeningClaim,
    usize,
) {
    let code_seed = Blaze2CodeSeed([seed; 32]);
    let mut rng = ChaCha8Rng::from_seed(code_seed.0);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let rows = rows(4, 8);
    let packed = pack_interleaved_rows(&rows).unwrap();
    let codeword_rows = code.encode_rows(&packed);
    let commitment =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let row_point = vec![B128::from(3 + seed as u64)];
    let col_point = vec![
        B128::from(5 + seed as u64),
        B128::from(9 + seed as u64),
        B128::from(13 + seed as u64),
    ];
    let value = evaluate_packed_matrix_at_point(&packed, &row_point, &col_point).unwrap();
    let claim = Blaze2OpeningClaim {
        row_point,
        col_point,
        value,
    };
    let num_queries = 4;
    (code, code_seed, packed, commitment, claim, num_queries)
}

fn blaze2_code_spec(seed: u8) -> Blaze2CodeSpec {
    Blaze2CodeSpec {
        version: 1,
        field_id: Blaze2FieldId::B128,
        hash_id: Blaze2HashId::Blake2s256,
        raa_variant: RaaVariant::PackedPrefixAccumulator,
        packing: Blaze2PackingLayout::PackedInterleavedRows,
        leaf_layout: Blaze2LeafLayout::InterleavedColumn,
        praa_message_len: 8,
        praa_expansion_factor: 4,
        praa_codeword_len: 32,
        seed: Blaze2CodeSeed([seed; 32]),
    }
}

fn blaze2_basefold_backend_params(
    seed: u8,
    q_raa_input: usize,
    q_backend_proof: usize,
) -> Blaze2BaseFoldBackendParams {
    let auxiliary_oracle_len =
        required_blaze2_basefold_auxiliary_oracle_len(&blaze2_code_spec(seed));
    blaze2_basefold_backend_params_with_auxiliary_len(
        seed,
        q_raa_input,
        q_backend_proof,
        auxiliary_oracle_len,
    )
}

fn blaze2_basefold_backend_params_with_auxiliary_len(
    seed: u8,
    q_raa_input: usize,
    q_backend_proof: usize,
    auxiliary_oracle_len: usize,
) -> Blaze2BaseFoldBackendParams {
    let praa = blaze2_code_spec(seed);
    let compiler_message_len = praa.praa_codeword_len;
    let parity_expansion_factor = 1;
    Blaze2BaseFoldBackendParams::new(Blaze2BaseFoldBackendSpec {
        praa,
        compiler_code: SystematicFoldableCodeSpec {
            version: 1,
            compiler_message_len,
            compiler_systematic_len: compiler_message_len,
            compiler_parity_len: compiler_message_len * parity_expansion_factor,
            compiler_codeword_len: compiler_message_len * (parity_expansion_factor + 1),
            parity_expansion_factor,
            seed: [seed ^ 0xa5; 32],
        },
        q_raa_input,
        q_backend_proof,
        auxiliary_oracle_len,
    })
    .unwrap()
}

fn audit_blaze2_opening_against_witness<H: Hash, B: Blaze2FoldedMessageBackend>(
    code: &PackedRaaCode,
    code_seed: &Blaze2CodeSeed,
    packed: &[Vec<B128>],
    commitment: &Blaze2InterleavedCodewordCommitment<H>,
    claim: &Blaze2OpeningClaim,
    proof: &Blaze2OpeningProof<H, B>,
    num_queries: usize,
) -> Result<(), Error> {
    if proof.queries.len() != num_queries {
        return Err(Error::InvalidPcsOpen(
            "test audit: proof query count mismatch".to_string(),
        ));
    }

    let row_len = code.message_len();
    let num_rows = packed.len();
    let mut row_evals = vec![B128::ZERO; num_rows];
    let mut scratch = vec![B128::ZERO; row_len.max(num_rows)];
    let value = evaluate_packed_matrix_at_point_into(
        packed,
        &claim.row_point,
        &claim.col_point,
        &mut row_evals,
        &mut scratch,
    )?;
    if value != claim.value {
        return Err(Error::InvalidPcsOpen(
            "test audit: claim does not match witness".to_string(),
        ));
    }
    if proof.row_evals != row_evals {
        return Err(Error::InvalidPcsOpen(
            "test audit: row evaluations do not match witness".to_string(),
        ));
    }

    let public = commitment.public();
    let mut transcript = CfriTranscript::<H>::new();
    absorb_blaze2_opening_public(
        &mut transcript,
        code,
        code_seed,
        &public,
        claim,
        num_queries,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &row_evals);

    let mut folding_challenges = vec![B128::ZERO; num_rows];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);

    let mut folded_message = vec![B128::ZERO; row_len];
    fold_packed_rows_into(packed, &folding_challenges, &mut folded_message)?;
    let folded_eval =
        evaluate_multilinear(&folded_message, &claim.col_point, &mut scratch[..row_len])?;
    B::absorb_commitment(&mut transcript, &proof.folded_message.commitment);
    absorb_blaze2_opening_folded_eval(&mut transcript, &folded_eval);

    let mut query_indices = vec![0usize; num_queries];
    squeeze_blaze2_opening_query_indices(&mut transcript, code.codeword_len(), &mut query_indices)?;

    let folded_codeword = code.encode_row(&folded_message);
    for (query, &expected_index) in proof.queries.iter().zip(&query_indices) {
        let current = &query.column_opening;
        current.authenticate(
            commitment.root(),
            commitment.num_rows(),
            code.codeword_len(),
        )?;
        if current.index != expected_index {
            return Err(Error::InvalidPcsOpen(
                "test audit: interleaved column index mismatch".to_string(),
            ));
        }
        if fold_interleaved_column(current, &folding_challenges)? != folded_codeword[expected_index]
        {
            return Err(Error::InvalidPcsOpen(
                "test audit: interleaved column does not match folded codeword".to_string(),
            ));
        }
    }

    Ok(())
}

fn exhaustive_backend_blaze2_outer_bytes(
    proof: &Blaze2OpeningProof<Blake2s256, ExhaustiveFoldedMessageBackend>,
) -> usize {
    exhaustive_backend_blaze2_outer_bytes_with_field_bytes(proof, 16)
}

fn exhaustive_backend_blaze2_outer_bytes_with_field_bytes(
    proof: &Blaze2OpeningProof<Blake2s256, ExhaustiveFoldedMessageBackend>,
    field_bytes: usize,
) -> usize {
    proof.row_evals.len() * field_bytes
        + proof.folded_message.commitment.len()
        + proof
            .queries
            .iter()
            .map(|query| {
                query.column_opening.values.len() * field_bytes
                    + query
                        .column_opening
                        .path
                        .iter()
                        .map(|digest| digest.len())
                        .sum::<usize>()
            })
            .sum::<usize>()
}

fn blaze2_basefold_outer_bytes_with_field_bytes(
    proof: &Blaze2BaseFoldOpeningProof<Blake2s256>,
    field_bytes: usize,
) -> usize {
    let backend_prequery_bytes = proof.backend_prequery.compiler_parity.root.len()
        + proof
            .backend_prequery
            .eval_sumcheck
            .as_ref()
            .map(|sumcheck| sumcheck.round_polynomials.len() * 3 * field_bytes)
            .unwrap_or(0)
        + proof
            .backend_prequery
            .folded_parity_layers
            .iter()
            .map(|layer| layer.root.len())
            .sum::<usize>()
        + proof.backend_prequery.terminal_codeword.len() * field_bytes
        + proof
            .backend_prequery
            .auxiliary
            .as_ref()
            .map(|auxiliary| auxiliary.root.len())
            .unwrap_or(0);
    proof.row_evals.len() * field_bytes
        + backend_prequery_bytes
        + proof
            .queries
            .iter()
            .map(|query| {
                query.column_opening.values.len() * field_bytes
                    + query
                        .column_opening
                        .path
                        .iter()
                        .map(|digest| digest.len())
                        .sum::<usize>()
            })
            .sum::<usize>()
}

fn replay_blaze2_basefold_schedule(
    params: &Blaze2BaseFoldBackendParams,
    commitment: &Blaze2InterleavedCodewordCommitment<Blake2s256>,
    claim: &Blaze2OpeningClaim,
    proof: &Blaze2BaseFoldOpeningProof<Blake2s256>,
) -> HolographicQuerySchedule {
    let mut transcript = CfriTranscript::<Blake2s256>::new();
    absorb_blaze2_opening_public_with_code_spec(
        &mut transcript,
        params.praa(),
        &commitment.public(),
        claim,
        params.spec().q_raa_input,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &proof.row_evals);
    let mut folding_challenges = vec![B128::ZERO; commitment.num_rows()];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);
    let folded_eval = proof
        .row_evals
        .iter()
        .zip(folding_challenges.iter())
        .fold(B128::ZERO, |acc, (&eval, &challenge)| {
            acc + eval * challenge
        });
    absorb_blaze2_opening_folded_eval(&mut transcript, &folded_eval);
    let request = Blaze2BaseFoldOpenRequest {
        col_point: &claim.col_point,
        folded_eval,
    };
    params
        .sample_query_schedule(&mut transcript, &proof.backend_prequery, &request)
        .unwrap()
}

fn blaze2_basefold_backend_query_bytes_with_field_bytes(
    proof: &Blaze2BaseFoldOpeningProof<Blake2s256>,
    field_bytes: usize,
) -> usize {
    proof
        .backend_proof
        .compiler_parity
        .queries
        .iter()
        .map(|query| field_bytes + query.path.iter().map(|digest| digest.len()).sum::<usize>())
        .sum::<usize>()
        + proof
            .backend_proof
            .compiler_parity_folds
            .paths
            .iter()
            .map(|path| {
                path.steps
                    .iter()
                    .map(|step| {
                        3 * field_bytes
                            + step
                                .left_path
                                .iter()
                                .map(|digest| digest.len())
                                .sum::<usize>()
                            + step
                                .right_path
                                .iter()
                                .map(|digest| digest.len())
                                .sum::<usize>()
                            + step
                                .folded_path
                                .iter()
                                .map(|digest| digest.len())
                                .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
        + proof
            .backend_proof
            .auxiliary
            .as_ref()
            .map(|auxiliary| {
                auxiliary
                    .all_queries()
                    .map(|query| {
                        field_bytes + query.path.iter().map(|digest| digest.len()).sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .unwrap_or(0)
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
fn blaze2_code_spec_deterministically_expands_to_praa_code() {
    let spec = blaze2_code_spec(44);
    let code = Blaze2Code::new(spec.clone()).unwrap();
    let same_code = Blaze2Code::new(spec.clone()).unwrap();

    assert_eq!(code.spec(), &spec);
    assert_eq!(code.packed().message_len(), spec.praa_message_len);
    assert_eq!(code.packed().rate(), spec.praa_expansion_factor);
    assert_eq!(code.packed().codeword_len(), spec.praa_codeword_len);

    let message = (0..spec.praa_message_len)
        .map(|index| B128::from(3 + index as u64 * 17))
        .collect::<Vec<_>>();
    assert_eq!(
        code.packed().encode_row(&message),
        same_code.packed().encode_row(&message)
    );

    let mut rng = ChaCha8Rng::from_seed(spec.seed.0);
    let expected = PackedRaaCode::new(spec.praa_message_len, spec.praa_expansion_factor, &mut rng);
    assert_eq!(
        code.packed().encode_row(&message),
        expected.encode_row(&message)
    );

    let changed_code = Blaze2Code::new(blaze2_code_spec(45)).unwrap();
    assert_ne!(
        code.packed().encode_row(&message),
        changed_code.packed().encode_row(&message)
    );
}

#[test]
fn blaze2_code_spec_rejects_inconsistent_lengths() {
    let mut spec = blaze2_code_spec(46);
    spec.praa_codeword_len += 1;
    assert!(Blaze2Code::new(spec).is_err());

    let mut spec = blaze2_code_spec(46);
    spec.praa_expansion_factor = 3;
    spec.praa_codeword_len = spec.praa_message_len * spec.praa_expansion_factor;
    assert!(Blaze2Code::new(spec).is_err());

    let mut spec = blaze2_code_spec(46);
    spec.version = 0;
    assert!(Blaze2Code::new(spec).is_err());
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
fn blaze2_raa_codeword_eval_weights_reconstruct_message_eval() {
    let mut rng = ChaCha8Rng::from_seed([63; 32]);
    let code = PackedRaaCode::new(8, 4, &mut rng);
    let message = (0..code.message_len())
        .map(|index| B128::from(19 + 7 * index as u64))
        .collect::<Vec<_>>();
    let point = vec![B128::from(3), B128::from(5), B128::from(11)];
    let mut scratch = vec![B128::ZERO; message.len()];
    let expected = evaluate_multilinear(&message, &point, &mut scratch).unwrap();
    let codeword = code.encode_row(&message);
    let weights = raa_codeword_eval_weights(&code, &point).unwrap();
    let actual = codeword
        .iter()
        .zip(weights.iter())
        .fold(B128::ZERO, |acc, (&value, &weight)| acc + value * weight);

    assert_eq!(actual, expected);
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
fn blaze2_paper_raa_aux_trace_links_to_committed_columns() {
    let (code, codeword_rows, challenges, folded_message, trace) = paper_raa_fixture(25);
    let column_comm =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let aux_comm = Blaze2RaaAuxTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let folded_codeword = code.encode_row(&folded_message);

    assert_eq!(column_comm.num_rows(), challenges.len());
    assert_eq!(column_comm.codeword_len(), code.codeword_len());
    assert_eq!(aux_comm.num_rows(), 3);
    assert_eq!(aux_comm.codeword_len(), code.codeword_len());

    for index in [0usize, 1, 6, 7, 17, 30, 31] {
        let current_column = column_comm.query(index).unwrap();
        assert_eq!(
            fold_interleaved_column(&current_column, &challenges).unwrap(),
            folded_codeword[index]
        );

        let opening = aux_comm.spot_query(&code, index).unwrap();
        let columns = paper_column_openings(&column_comm, index);
        opening
            .verify(
                &code,
                &folded_message,
                aux_comm.root(),
                column_comm.root(),
                &columns,
                &challenges,
            )
            .unwrap();
        verify_raa_aux_trace_spot_query(
            &code,
            &folded_message,
            aux_comm.root(),
            &opening,
            column_comm.root(),
            &columns,
            &challenges,
        )
        .unwrap();
    }
}

#[test]
fn blaze2_paper_raa_aux_trace_rejects_authenticated_bad_auxiliary_layer() {
    let (code, codeword_rows, challenges, folded_message, mut trace) = paper_raa_fixture(26);
    trace.u4[7] += B128::ONE;
    let column_comm =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let aux_comm = Blaze2RaaAuxTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = aux_comm.spot_query(&code, 7).unwrap();
    let columns = paper_column_openings(&column_comm, 7);

    assert!(opening
        .verify(
            &code,
            &folded_message,
            aux_comm.root(),
            column_comm.root(),
            &columns,
            &challenges,
        )
        .is_err());
}

#[test]
fn blaze2_paper_raa_aux_trace_rejects_authenticated_bad_committed_column() {
    let (code, mut codeword_rows, challenges, folded_message, trace) = paper_raa_fixture(27);
    codeword_rows[0][6] += B128::ONE;
    let column_comm =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let aux_comm = Blaze2RaaAuxTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = aux_comm.spot_query(&code, 6).unwrap();
    let columns = paper_column_openings(&column_comm, 6);

    assert!(opening
        .verify(
            &code,
            &folded_message,
            aux_comm.root(),
            column_comm.root(),
            &columns,
            &challenges,
        )
        .is_err());
}

#[test]
fn blaze2_paper_raa_aux_trace_rejects_tampered_column_opening() {
    let (code, codeword_rows, challenges, folded_message, trace) = paper_raa_fixture(28);
    let column_comm =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let aux_comm = Blaze2RaaAuxTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = aux_comm.spot_query(&code, 6).unwrap();
    let mut columns = paper_column_openings(&column_comm, 6);
    columns[0].values[0] += B128::ONE;

    assert!(opening
        .verify(
            &code,
            &folded_message,
            aux_comm.root(),
            column_comm.root(),
            &columns,
            &challenges,
        )
        .is_err());
}

#[test]
fn blaze2_paper_raa_aux_trace_rejects_missing_previous_column() {
    let (code, codeword_rows, challenges, folded_message, trace) = paper_raa_fixture(29);
    let column_comm =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&codeword_rows)
            .unwrap();
    let aux_comm = Blaze2RaaAuxTraceCommitment::<Blake2s256>::commit_trace(&trace).unwrap();
    let opening = aux_comm.spot_query(&code, 6).unwrap();
    let columns = vec![column_comm.query(6).unwrap()];

    assert!(opening
        .verify(
            &code,
            &folded_message,
            aux_comm.root(),
            column_comm.root(),
            &columns,
            &challenges,
        )
        .is_err());
}

#[test]
fn blaze2_opening_verifies_row_fold_and_raa_boundaries() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(30);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    let public = commitment.public();

    assert_eq!(proof.row_evals.len(), commitment.num_rows());
    assert_eq!(proof.queries.len(), num_queries);
    audit_blaze2_opening_against_witness::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        &proof,
        num_queries,
    )
    .unwrap();
    verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &public,
        &claim,
        &proof,
        num_queries,
    )
    .unwrap();
}

#[test]
fn blaze2_opening_with_code_spec_binds_compact_public_code() {
    let (expanded_code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(47);
    let compact_code = Blaze2Code::new(blaze2_code_spec(47)).unwrap();
    assert_eq!(
        compact_code.packed().encode_rows(&packed),
        expanded_code.encode_rows(&packed)
    );

    let proof = prove_blaze2_opening_with_code_spec::<_, ExhaustiveFoldedMessageBackend>(
        &compact_code,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    let public = commitment.public();

    verify_blaze2_opening_with_code_spec::<_, ExhaustiveFoldedMessageBackend>(
        &compact_code,
        &public,
        &claim,
        &proof,
        num_queries,
    )
    .unwrap();

    assert!(
        verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
            &expanded_code,
            &code_seed,
            &public,
            &claim,
            &proof,
            num_queries,
        )
        .is_err(),
        "expanded-code transcript must not verify a proof sampled from the compact-code transcript"
    );
}

#[test]
fn blaze2_opening_with_code_spec_rejects_hash_id_mismatch() {
    let (_, _, packed, commitment, claim, num_queries) = opening_fixture(48);
    let mut spec = blaze2_code_spec(48);
    spec.hash_id = Blaze2HashId::Blake2s;
    let compact_code = Blaze2Code::new(spec).unwrap();

    assert!(
        prove_blaze2_opening_with_code_spec::<_, ExhaustiveFoldedMessageBackend>(
            &compact_code,
            &packed,
            &commitment,
            &claim,
            num_queries,
        )
        .is_err()
    );

    let valid_code = Blaze2Code::new(blaze2_code_spec(48)).unwrap();
    let proof = prove_blaze2_opening_with_code_spec::<_, ExhaustiveFoldedMessageBackend>(
        &valid_code,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    assert!(
        verify_blaze2_opening_with_code_spec::<_, ExhaustiveFoldedMessageBackend>(
            &compact_code,
            &commitment.public(),
            &claim,
            &proof,
            num_queries,
        )
        .is_err()
    );
}

#[test]
fn blaze2_opening_outer_proof_size_matches_paper_accounting() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(43);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();

    let t = commitment.num_rows();
    let field_bytes = 16;
    let hash_bytes = 32;
    let path_len = code.codeword_len().trailing_zeros() as usize;
    let expected =
        t * field_bytes + hash_bytes + num_queries * (t * field_bytes + path_len * hash_bytes);

    assert_eq!(
        exhaustive_backend_blaze2_outer_bytes(&proof),
        expected,
        "Blaze2 outer proof must be u + folded commitment/eval + Q_RAA opened columns and paths"
    );
}

#[test]
fn blaze2_outer_proof_matches_paper_shape_after_field_byte_correction() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(49);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();

    let t = commitment.num_rows();
    let paper_field_bytes = 8;
    let hash_bytes = 32;
    let path_len = code.codeword_len().trailing_zeros() as usize;
    let backend_commitment_bytes = hash_bytes;
    let expected = t * paper_field_bytes
        + backend_commitment_bytes
        + num_queries * (t * paper_field_bytes + path_len * hash_bytes);

    assert_eq!(
        exhaustive_backend_blaze2_outer_bytes_with_field_bytes(&proof, paper_field_bytes),
        expected,
        "after the allowed 16-to-8 byte field correction, Blaze2 outer proof shape must be row evals + one backend commitment + Q_RAA opened columns and paths"
    );
}

#[test]
fn blaze2_basefold_opening_verifies_with_typed_backend_schedule() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(50);
    let q_raa_input = 4;
    let q_backend_proof = 7;
    let params = blaze2_basefold_backend_params(50, q_raa_input, q_backend_proof);
    let proof = prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();

    assert_eq!(proof.queries.len(), q_raa_input);
    assert!(proof.backend_prequery.auxiliary.is_some());
    assert!(proof.backend_proof.auxiliary.is_some());
    let schedule = replay_blaze2_basefold_schedule(&params, &commitment, &claim, &proof);
    let compiler_parity_query_count = schedule
        .proof_queries()
        .iter()
        .filter(|query| query.domain == BackendProofQueryDomain::CompilerParity)
        .count();
    assert_eq!(
        proof.backend_proof.compiler_parity.queries.len(),
        compiler_parity_query_count
    );
    assert_eq!(
        proof
            .backend_proof
            .auxiliary
            .as_ref()
            .unwrap()
            .query_count(),
        schedule.expected_auxiliary_query_proof_count()
    );
    verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &proof).unwrap();
}

#[test]
fn blaze2_basefold_outer_shape_matches_paper_after_field_byte_correction() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(51);
    let q_raa_input = 4;
    let q_backend_proof = 9;
    let params = blaze2_basefold_backend_params(51, q_raa_input, q_backend_proof);
    let proof = prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();

    let t = commitment.num_rows();
    let paper_field_bytes = 8;
    let hash_bytes = 32;
    let outer_path_len = params.praa().packed().codeword_len().trailing_zeros() as usize;
    let eval_sumcheck_bytes =
        params.praa().packed().codeword_len().trailing_zeros() as usize * 3 * paper_field_bytes;
    let backend_prequery_bytes = (1 + params.compiler_code().layout().num_rounds()) * hash_bytes
        + eval_sumcheck_bytes
        + hash_bytes
        + (params.compiler_code().layout().parity_expansion_factor() + 1) * paper_field_bytes;
    let expected_outer = t * paper_field_bytes
        + backend_prequery_bytes
        + q_raa_input * (t * paper_field_bytes + outer_path_len * hash_bytes);

    assert_eq!(
        blaze2_basefold_outer_bytes_with_field_bytes(&proof, paper_field_bytes),
        expected_outer,
        "systematic BaseFold integration must keep the Blaze outer proof at row evals + backend prequery roots + exactly Q_RAA opened columns"
    );
    assert_eq!(
        proof.queries.len(),
        q_raa_input,
        "backend proof queries must not create extra Blaze input-column openings"
    );
    let schedule = replay_blaze2_basefold_schedule(&params, &commitment, &claim, &proof);
    let compiler_parity_query_count = schedule
        .proof_queries()
        .iter()
        .filter(|query| query.domain == BackendProofQueryDomain::CompilerParity)
        .count();
    let relation_auxiliary_query_count = schedule.relation_auxiliary_proof_query_count();
    assert_eq!(
        compiler_parity_query_count + relation_auxiliary_query_count,
        q_backend_proof,
        "typed backend proof schedule has exactly Q_backend entries"
    );
    assert_eq!(
        proof.backend_proof.compiler_parity.queries.len(),
        compiler_parity_query_count,
        "compiler-parity openings match their subset of the backend proof schedule"
    );
    assert_eq!(
        proof
            .backend_proof
            .auxiliary
            .as_ref()
            .unwrap()
            .query_count(),
        schedule.expected_auxiliary_query_proof_count(),
        "auxiliary proof carries exactly the schedule-required relation and eval-accumulator openings"
    );
    assert_eq!(
        proof.backend_prequery.folded_parity_layers.len(),
        params.compiler_code().layout().num_rounds(),
        "folded-layer roots are backend prequery commitments, not extra Blaze input openings"
    );
    assert!(blaze2_basefold_backend_query_bytes_with_field_bytes(&proof, paper_field_bytes) > 0);
}

#[test]
fn blaze2_basefold_opening_rejects_tampered_backend_prequery() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(52);
    let params = blaze2_basefold_backend_params(52, 4, 7);
    let mut proof =
        prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();
    proof.backend_prequery.compiler_parity.root[0] ^= 1;

    assert!(verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &proof).is_err());
}

#[test]
fn blaze2_basefold_opening_rejects_bad_outer_column() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(53);
    let params = blaze2_basefold_backend_params(53, 4, 7);
    let mut proof =
        prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();
    proof.queries[0].column_opening.values[0] += B128::ONE;

    assert!(verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &proof).is_err());
}

#[test]
fn blaze2_basefold_opening_rejects_row_eval_folded_eval_mismatch() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(56);
    let params = blaze2_basefold_backend_params(56, 4, 7);
    let mut proof =
        prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();

    let delta = B128::ONE;
    proof.row_evals[0] += claim.row_point[0] * delta;
    proof.row_evals[1] += delta;

    let mut scratch = vec![B128::ZERO; proof.row_evals.len()];
    assert_eq!(
        evaluate_multilinear(&proof.row_evals, &claim.row_point, &mut scratch).unwrap(),
        claim.value,
        "test mutation must preserve the outer row-evaluation claim"
    );
    assert!(verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &proof).is_err());
}

#[test]
fn blaze2_basefold_opening_rejects_terminal_only_eval_binding_defect() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(57);
    let params = blaze2_basefold_backend_params(57, 4, 7);
    let code = params.praa().packed();
    let row_len = code.message_len();
    let num_rows = packed.len();
    let mut row_evals = vec![B128::ZERO; num_rows];
    let mut scratch = vec![B128::ZERO; row_len.max(num_rows)];
    let claim_value = evaluate_packed_matrix_at_point_into(
        &packed,
        &claim.row_point,
        &claim.col_point,
        &mut row_evals,
        &mut scratch,
    )
    .unwrap();
    assert_eq!(claim_value, claim.value);

    let delta = B128::ONE;
    row_evals[0] += claim.row_point[0] * delta;
    row_evals[1] += delta;
    assert_eq!(
        evaluate_multilinear(&row_evals, &claim.row_point, &mut scratch[..num_rows]).unwrap(),
        claim.value,
        "test mutation must preserve the outer row-evaluation claim"
    );

    let public_commitment = commitment.public();
    let mut transcript = CfriTranscript::<Blake2s256>::new();
    absorb_blaze2_opening_public_with_code_spec(
        &mut transcript,
        params.praa(),
        &public_commitment,
        &claim,
        params.spec().q_raa_input,
    );
    absorb_blaze2_opening_row_evals(&mut transcript, &row_evals);
    let mut folding_challenges = vec![B128::ZERO; num_rows];
    squeeze_blaze2_opening_folding_challenges(&mut transcript, &mut folding_challenges);

    let claimed_folded_eval = row_evals
        .iter()
        .zip(folding_challenges.iter())
        .fold(B128::ZERO, |acc, (&eval, &challenge)| {
            acc + eval * challenge
        });
    let mut folded_message = vec![B128::ZERO; row_len];
    fold_packed_rows_into(&packed, &folding_challenges, &mut folded_message).unwrap();
    let actual_folded_eval =
        evaluate_multilinear(&folded_message, &claim.col_point, &mut scratch[..row_len]).unwrap();
    assert_ne!(
        claimed_folded_eval, actual_folded_eval,
        "test mutation must create a folded-eval mismatch"
    );
    absorb_blaze2_opening_folded_eval(&mut transcript, &claimed_folded_eval);

    let backend_request = Blaze2BaseFoldOpenRequest {
        col_point: &claim.col_point,
        folded_eval: claimed_folded_eval,
    };
    let folded_codeword = code.encode_row(&folded_message);
    let trace = build_raa_aux_trace(code, &folded_message).unwrap();
    let mut auxiliary = Vec::with_capacity(params.spec().auxiliary_oracle_len);
    auxiliary.extend_from_slice(&trace.u2);
    auxiliary.extend_from_slice(&trace.u3);
    auxiliary.extend_from_slice(&trace.u4);
    assert!(
        params
            .prove_prequery::<Blake2s256>(&folded_codeword, &auxiliary, &backend_request)
            .is_err(),
        "eval sumcheck generation must reject a terminal-only folded-eval defect"
    );
}

#[test]
fn blaze2_basefold_opening_derives_configured_auxiliary_trace() {
    let (_, _, packed, commitment, claim, _) = opening_fixture(54);
    let auxiliary_len = required_blaze2_basefold_auxiliary_oracle_len(&blaze2_code_spec(54));
    let params = blaze2_basefold_backend_params_with_auxiliary_len(54, 4, 11, auxiliary_len);
    let proof = prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &[]).unwrap();

    assert_eq!(
        proof
            .backend_prequery
            .auxiliary
            .as_ref()
            .map(|public| public.len),
        Some(auxiliary_len)
    );
    verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &proof).unwrap();
    assert_eq!(
        proof
            .backend_prequery
            .eval_sumcheck
            .as_ref()
            .unwrap()
            .round_polynomials
            .len(),
        params.praa().packed().codeword_len().trailing_zeros() as usize
    );

    let mut bad_sumcheck = proof.clone();
    bad_sumcheck
        .backend_prequery
        .eval_sumcheck
        .as_mut()
        .unwrap()
        .round_polynomials[0][0] += B128::ONE;
    assert!(
        verify_blaze2_basefold_opening(&params, &commitment.public(), &claim, &bad_sumcheck)
            .is_err()
    );

    let mut missing_sumcheck = proof.clone();
    missing_sumcheck.backend_prequery.eval_sumcheck = None;
    assert!(verify_blaze2_basefold_opening(
        &params,
        &commitment.public(),
        &claim,
        &missing_sumcheck
    )
    .is_err());

    let mut bad_final_accumulator = proof.clone();
    bad_final_accumulator
        .backend_proof
        .auxiliary
        .as_mut()
        .unwrap()
        .final_accumulator_queries
        .first_mut()
        .unwrap()
        .u4
        .value += B128::ONE;
    assert!(verify_blaze2_basefold_opening(
        &params,
        &commitment.public(),
        &claim,
        &bad_final_accumulator
    )
    .is_err());

    let bad_auxiliary = vec![B128::ONE; auxiliary_len];
    assert!(
        prove_blaze2_basefold_opening(&params, &packed, &commitment, &claim, &bad_auxiliary)
            .is_err()
    );
}

#[test]
fn blaze2_basefold_backend_rejects_arbitrary_auxiliary_length() {
    let praa = blaze2_code_spec(55);
    let compiler_message_len = praa.praa_codeword_len;
    let parity_expansion_factor = 1;
    let spec = Blaze2BaseFoldBackendSpec {
        praa,
        compiler_code: SystematicFoldableCodeSpec {
            version: 1,
            compiler_message_len,
            compiler_systematic_len: compiler_message_len,
            compiler_parity_len: compiler_message_len * parity_expansion_factor,
            compiler_codeword_len: compiler_message_len * (parity_expansion_factor + 1),
            parity_expansion_factor,
            seed: [55 ^ 0xa5; 32],
        },
        q_raa_input: 4,
        q_backend_proof: 7,
        auxiliary_oracle_len: 1,
    };

    assert!(Blaze2BaseFoldBackendParams::new(spec).is_err());
}

#[test]
fn blaze2_basefold_auxiliary_length_uses_relation_rows_only() {
    let spec = blaze2_code_spec(57);
    assert_eq!(
        required_blaze2_basefold_relation_auxiliary_len(&spec),
        3 * spec.praa_codeword_len
    );
    assert_eq!(required_blaze2_basefold_eval_binding_len(&spec), 0);
    assert_eq!(
        required_blaze2_basefold_auxiliary_oracle_len(&spec),
        required_blaze2_basefold_relation_auxiliary_len(&spec)
    );
}

#[test]
fn blaze2_opening_rejects_bad_claim_value() {
    let (code, code_seed, packed, commitment, mut claim, num_queries) = opening_fixture(31);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    claim.value += B128::ONE;

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_bad_row_eval_vector() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(32);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.row_evals[0] += B128::ONE;

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_bad_input_column_opening() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(33);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.queries[0].column_opening.values[0] += B128::ONE;

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_does_not_serialize_folded_message_eval() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(40);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();

    let serialized = format!("{proof:?}");
    assert!(
        !serialized.contains("eval:"),
        "folded_eval is derived by the verifier and must not be serialized as proof data"
    );
}

#[test]
fn blaze2_opening_rejects_bad_folded_message_backend_commitment() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(41);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.folded_message.commitment[0] ^= 1;

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_bad_folded_message_backend_proof() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(42);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.folded_message.backend_proof.message[0] += B128::ONE;

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_query_index_not_sampled_by_transcript() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(38);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.queries.swap(0, 1);

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_truncated_query_count() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(39);
    let mut proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();
    proof.queries.pop();

    assert!(verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries - 1,
    )
    .is_err());
}

#[test]
fn blaze2_opening_rejects_bad_committed_codeword_column() {
    let (code, code_seed, packed, commitment, claim, num_queries) = opening_fixture(34);
    let proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries,
    )
    .unwrap();

    let mut bad_codeword_rows = code.encode_rows(&packed);
    for row in &mut bad_codeword_rows {
        for value in row {
            *value = B128::ZERO;
        }
    }
    let bad_commitment =
        Blaze2InterleavedCodewordCommitment::<Blake2s256>::commit_codeword_rows(&bad_codeword_rows)
            .unwrap();
    let bad_proof = prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &bad_commitment,
        &claim,
        num_queries,
    )
    .unwrap();

    assert!(
        audit_blaze2_opening_against_witness::<_, ExhaustiveFoldedMessageBackend>(
            &code,
            &code_seed,
            &packed,
            &bad_commitment,
            &claim,
            &bad_proof,
            num_queries,
        )
        .is_err()
    );
    verify_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &commitment.public(),
        &claim,
        &proof,
        num_queries,
    )
    .unwrap();
}

#[test]
fn blaze2_opening_prover_rejects_claim_not_matching_rows() {
    let (code, code_seed, packed, commitment, mut claim, num_queries) = opening_fixture(35);
    claim.value += B128::ONE;

    assert!(prove_blaze2_opening::<_, ExhaustiveFoldedMessageBackend>(
        &code,
        &code_seed,
        &packed,
        &commitment,
        &claim,
        num_queries
    )
    .is_err());
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
