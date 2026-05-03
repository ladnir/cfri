# Proof Size: Paper Budget vs Current Implementation

This compares the paper-first proof-size accounting in `docs/proof_size_accounting.md` to the
implementation transcripts.

Current measured command:

```powershell
cargo test --release -p cfri --test perf_baseline current_release_performance_baseline -- --ignored --nocapture --test-threads=1
```

Measured legacy transcript proof sizes:

| Scheme | Parameters | Current bytes |
| --- | --- | ---: |
| BaseFold transparent | `num_vars=10`, `poly_len=1024`, `Q=5`, `log_rate=1` | 12,096 |
| BaseFold hiding | `num_vars=10`, `poly_len=1024`, `Q=5`, `log_rate=1` | 14,144 |
| Legacy Blaze transparent | `num_vars=8`, `poly_len=256`, `rows=64`, `Q_RAA=16` | 5,404,240 |
| Legacy Blaze hiding | `num_vars=8`, `poly_len=256`, `rows=64`, `Q_RAA=16` | 6,048,352 |

## BaseFold

For the current transparent BaseFold baseline, the implementation accounting exactly matches
`12,096` bytes:

| Component | Formula | Bytes |
| --- | ---: | ---: |
| Public commitment root written into same proof buffer | `1 * 32` | 32 |
| Claimed eval written by caller | `1 * 32` | 32 |
| Folded oracle roots | `10 * 32` | 320 |
| Sumcheck messages | `11 * 3 * 32` | 1,056 |
| Prover recomputed eval | `1 * 32` | 32 |
| Final base oracle | `2 * 32` | 64 |
| Query field values | `5 * (10 + 1) * 32` | 1,760 |
| Query Merkle auth | `5 * 55 * 32` | 8,800 |
| Total |  | 12,096 |

The current BaseFold transcript now uses the paper-style carrying convention for query values:
the first layer sends both leaf values, and each lower folded layer sends only the sibling value.
Merkle openings are also sibling-only and do not send the public/final root back to the verifier.

## Legacy Blaze

For the legacy transparent Blaze baseline, the implementation accounting also matches the measured
`5,404,240` bytes:

| Component | Bytes |
| --- | ---: |
| Blaze commitment root in outer transcript | 32 |
| Row evaluation vector and folded eval in B128 transcript | 528 |
| RAA and permutation BaseFold roots before batch opening | 352 |
| Three custom sumchecks for RAA/permutation checks | 1,584 |
| Evaluations for split RAA/permutation polynomials | 176 |
| First BaseFold batch opening: 11 eval claims | 2,336,544 |
| Folded codeword commitment root | 32 |
| Folded codeword evals for 16 query positions | 256 |
| Second BaseFold batch opening: 16 eval claims | 3,043,744 |
| Outer Blaze Merkle paths | 4,608 |
| Outer Blaze queried leaves | 16,384 |
| Total | 5,404,240 |

The two BaseFold batch openings dominate:

| Subcomponent | First batch open | Second batch open |
| --- | ---: | ---: |
| Batch-open source roots | 352 | 32 |
| Batch sumcheck | 480 | 480 |
| Inner BaseFold roots and sumcheck | 848 | 848 |
| Individual queried source field values | 141,504 | 205,824 |
| Individual queried source Merkle paths | 1,415,040 | 2,058,240 |
| Inner folded query values | 70,752 | 70,752 |
| Inner folded query Merkle paths | 707,520 | 707,520 |
| Eval and final oracle | 48 | 48 |
| Total | 2,336,544 | 3,043,744 |

The verifier also checks the internal degree-2 sumcheck transition consistency for the three Blaze
sumcheck transcripts. This is a correctness check, not a proof-size component.

## Conclusion

BaseFold is now in the right proof-size family for this transcript shape.

Legacy Blaze is not in the paper proof-size family. The paper budget is:

```text
Blaze ~= one smaller BaseFold proof
       + Q_RAA * t * field_bytes
       + Q_RAA * log2(n/t) * hash_bytes
       + small scalar/vector overhead
```

The legacy implementation instead does:

```text
Blaze ~= two large BaseFold batch openings
       + per-evaluation Merkle paths for 14 committed polynomials
       + per-evaluation Merkle paths for 16 folded-codeword evaluation claims
       + outer RAA column paths/leaves
```

For the small baseline above, a paper-shaped proof with the same outer `Q_RAA=16` should be on
the order of one BaseFold backend proof plus tens of KB of Blaze column data. The current proof is
5.4 MB because batch opening still multiplies Merkle paths by the number of committed/evaluated
polynomials. The immediate duplication inside each BaseFold opening and inside the outer Blaze
Merkle openings is fixed, and the padded zero RAA chunks are no longer committed/opened. The
remaining cleanup target is Blaze's use of two large BaseFold batch openings instead of the paper's
single reduced claim plus RAA column openings.

## Blaze2

`crates/cfri/src/backend/blaze2.rs` is the paper-shaped rewrite path. Its production opening proof
contains only:

| Component | Count |
| --- | ---: |
| Row-evaluation vector `u` | `t` field elements |
| Folded-message backend commitment/evaluation/proof | delegated to the backend |
| Interleaved RAA queried columns | `Q_RAA` columns, each `t` field elements |
| Interleaved column Merkle paths | `Q_RAA * log2(n / t)` hashes |

The public RAA code description is not proof data. The transcript binds the RAA shape and a compact
32-byte code seed, so the verifier and prover agree on the public random code without hashing or
sending the full permutation tables in the proof.

For `B128`, the Blaze-specific byte budget before the backend proof is:

```text
blaze2_outer_bytes =
    t * 16
  + Q_RAA * (t * 16 + log2(n / t) * 32)
  + backend_commitment_bytes
```

The folded evaluation is derived by the verifier from the row-evaluation vector and folding
challenges. It is transcript-bound before query sampling, but it is not serialized proof data. The
backend proof itself must separately match its own paper accounting. Test-only exhaustive backends
are intentionally not proof-size meaningful.
