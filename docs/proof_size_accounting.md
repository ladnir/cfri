# Proof Size Accounting: BaseFold and Blaze

This note is deliberately paper-first. It ignores the current implementation and asks what an
argument proof should contain after compiling the IOPs with Fiat-Shamir and Merkle commitments.

Sources:

- BaseFold: <https://eprint.iacr.org/2023/1705.pdf>
- Blaze: <https://eprint.iacr.org/2024/1609.pdf>
- Remaining implementation plan: `docs/blaze2_required_proof_size_plan.md`

## Shared Byte Model

| Symbol | Meaning | Typical paper value |
| --- | --- | --- |
| `field_bytes` | bytes per field element in proof | `8` for 64-bit field comparisons, `16` for GF(2^128), `32` for 256-bit fields |
| `hash_bytes` | bytes per Merkle digest | `32` for Blake2s256 |
| `lambda` | target security bits | `100` in the paper comparisons |
| `D` | number of BaseFold folding rounds | roughly number of multilinear variables after any `k0` base case |
| `N_i` | BaseFold oracle length at layer `i` | `N_i = expansion * k0 * 2^i` |
| `Q_BF` | BaseFold query repetitions | chosen so `(1 - delta + gamma * D)^Q_BF` is at most `2^-lambda` |
| `Q_RAA` | Blaze RAA/interleaving query repetitions | `ceil(lambda / -log2(1 - delta_RAA / 3))` |
| `k_praa` | per-row PRAA message length | Blaze Section 8.2 `k` |
| `n_praa` | per-row PRAA codeword length and compiler systematic length | Blaze Section 8.2 `n/t` |
| `N_comp` | compiler-code codeword length for `C_sys(c_star)` | `n_praa + parity_len` |

The formulas below separate "field payload" from "hash authentication". For these transparent
schemes the Merkle authentication paths are usually the dominant term.

## Holographic/Systematic BaseFold Model

The target BaseFold backend is holographic. It runs over a systematic compiler code:

```text
C_sys(y) = (y || parity(y))
```

The systematic part `y` may be authenticated outside the backend. The parity/proof-oracle parts are
authenticated by the BaseFold backend proof. This is the accounting model used by Blaze's
MLIOP-to-IOPP compiler: `y = c_star` is supplied by Blaze column openings, while the
non-systematic compiler-code part and auxiliary oracles live in the backend proof.

For Phase 1, the accepted concrete compiler code is systematic-augmented RFC:

```text
C_sys(m) = (m, E_RFC(m))
N_comp = k_comp + n_rfc
c_parity = n_rfc / k_comp
c_parity in {1, 3, 7, 15, ...}
rate_parity = 1 / c_parity
rate_sys = k_comp / N_comp = 1 / (1 + c_parity)
delta_sys >= (n_rfc / N_comp) * delta_rfc
          = (c_parity / (1 + c_parity)) * delta_rfc
```

The `c_parity` restriction keeps `N_comp = (1 + c_parity) * k_comp` power-of-two when `k_comp` is
power-of-two, which lets the first implementation reuse the existing power-of-two folding
infrastructure. The distance penalty is real. Query counts for the backend must use `delta_sys`,
not the old RFC distance alone.

## BaseFold PCS Proof Contents

From BaseFold Protocol 4, a single evaluation proof contains:

1. Sumcheck messages `h_D, ..., h_1`.
2. Commitments to the folded proof oracles `pi_{D-1}, ..., pi_0`.
3. Query openings proving folding consistency between adjacent oracle layers, split by whether the
   queried entry is systematic input, parity, or auxiliary proof-oracle data.
4. A terminal check that `pi_0` is a valid base-code codeword.

The initial root for `pi_D = Enc_D(f)` is the polynomial commitment, so it is public input rather
than part of the opening proof.

### BaseFold Spreadsheet Rows

| Component | Count | Unit bytes | Total bytes | Notes |
| --- | ---: | ---: | ---: | --- |
| Sumcheck degree-2 polynomials | `3 * D` | `field_bytes` | `=3*D*field_bytes` | Each `h_i(X)` is degree 2, so three field elements. |
| Systematic input root | wrapper-dependent | `hash_bytes` | wrapper-dependent | Standalone BaseFold owns this; Blaze supplies it via the interleaved PRAA commitment instead. |
| Parity/proof-oracle Merkle roots | `D - 1` or `D` plus auxiliary roots | `hash_bytes` | `=(D-final_clear+aux_roots)*hash_bytes` | Roots for backend-authenticated parity and proof oracles. |
| Final base codeword | `N_0` | `field_bytes` | `=N_0*field_bytes` | Usually cheaper than committing when `N_0` is tiny. |
| Query leaf values, optimized path reuse | schedule-dependent | `field_bytes` | explicit schedule sum | Top systematic values may be externally authenticated; parity/proof-oracle values are backend-authenticated. Within each folding path, carried-value reuse still applies. |
| Query leaf values, literal no reuse | `2 * Q_BF * D` | `field_bytes` | `=2*Q_BF*D*field_bytes` | Useful as a red-flag upper bound. |
| Query Merkle authentication | `Q_BF * sum_{i=1..D} log2(N_i)` | `hash_bytes` | `=Q_BF*SUM(LOG2(N_i))*hash_bytes` | Paper verifier cost is `O(Q_BF * D)` Merkle path checks, each path length `O(D)`. |
| Optional transcript/check scalars | `O(D)` | `field_bytes` | small | Usually already covered by sumcheck or derived by Fiat-Shamir. |

For the common `k0 = 1`, `N_i = expansion * 2^i`. If the expansion is 2, then
`sum_{i=1..D} log2(N_i) = D*(D+3)/2`.

For the old monolithic standalone model, the dominant BaseFold size was:

```text
basefold_bytes ~= Q_BF * sum_{i=1..D} log2(N_i) * hash_bytes
               + Q_BF * (D + 1) * field_bytes
               + 3 * D * field_bytes
               + folded_roots
               + final_codeword
```

For the target holographic model, subtract systematic input authentication supplied by an external
commitment and add parity/proof-oracle authentication according to the typed query schedule:

```text
holographic_basefold_bytes ~=
    backend_roots
  + final_codeword
  + sumcheck_bytes
  + systematic_opening_bytes_owned_by_wrapper
  + parity_and_aux_query_values
  + parity_and_aux_merkle_paths
```

For Blaze, `systematic_opening_bytes_owned_by_wrapper` is zero inside the backend term because
systematic `c_star` openings are counted in the Blaze outer term.

The current Blaze2 holographic backend carries an eval-binding product sumcheck for
`sum_i eval_weight[i] * c_star[i] = folded_eval`. Its round messages contribute
`3 * log2(n_praa) * field_bytes` before serialization framing. This is a backend prequery term, not
an additional Blaze input-column opening.

The implementation regression test keeps an explicit byte decomposition for the current small
Blaze2/BaseFold fixture (`t = 2` packed rows, `n_praa = 32`, `Q_RAA = 4`,
`Q_backend = 9`). The implemented proof uses GF(2^128) elements, so the primary pinned total uses
`field_bytes = 16`:

| Component | Bytes |
|---|---:|
| Row evaluations | 32 |
| Backend prequery roots, eval sumcheck, terminal word | 560 |
| Blaze-authenticated column values | 128 |
| Blaze-authenticated column Merkle paths | 640 |
| Backend compiler-parity query values and multiproof nodes | 32 |
| Backend compiler-parity fold values and multiproof nodes | 544 |
| Backend auxiliary relation values and multiproof nodes | 960 |
| Section 5 residual values and shared residual multiproof nodes | 1,136 |
| Section 5 terminal folded-layer roots and authentication | 1,408 |
| Section 5 helper values and multiproof nodes | 928 |
| Total | 6,368 |

For comparison to the Blaze paper's 64-bit field-byte convention, the same proof structure with
`field_bytes = 8` projects to:

| Component | Bytes |
|---|---:|
| Row evaluations | 16 |
| Backend prequery roots, eval sumcheck, terminal word | 424 |
| Blaze-authenticated column values | 64 |
| Blaze-authenticated column Merkle paths | 640 |
| Backend compiler-parity query values and multiproof nodes | 16 |
| Backend compiler-parity fold values and multiproof nodes | 464 |
| Backend auxiliary relation values and multiproof nodes | 480 |
| Section 5 residual values and shared residual multiproof nodes | 984 |
| Section 5 terminal folded-layer roots and authentication | 1,408 |
| Section 5 helper values and multiproof nodes | 880 |
| Total | 5,376 |

This confirms the structural acceptance point: the Blaze outer term is exactly row evaluations plus
backend prequery material plus `Q_RAA` opened columns and paths. The remaining large term is inside
the backend proof, especially the residual-terminal side chain and Section 5 helper/residual
authentication that is still outside the final shared BaseFold-core proof; these are the next targets for
serialization tightening.

The BaseFold paper gives the protocol and asymptotics, while its exact figure data for standalone
PCS proof size is plotted visually. The Blaze paper's Figure 4 reports BaseFold proof sizes of about
`1.2, 1.2, 1.4, 1.4, 1.4 MB` for `25..29` variables in its comparison setting.

## Blaze Proof Contents

Blaze commits to an interleaved packed RAA codeword and then reduces the large interleaved
claim to a smaller BaseFold claim.

From Blaze Section 8.2, for a polynomial with `t * k` coefficients:

1. The proof opens a constant number of `t`-length columns of the interleaved code.
2. The proof includes a BaseFold/compiler proof whose systematic input length is the smaller
   witness/input length, written in the paper as `n / t`.
3. The proof includes Merkle paths for the queried interleaved-code columns.

For the RAA query count, the paper sets distance `delta_RAA = 0.19` and targets 100 bits:

```text
100 / -log2(1 - 0.19 / 3) = 1059.407...
```

Strict ceiling from the rounded `0.19` value gives `1060`, while the paper text reports `1059`.
For reproducing the paper's Figure 4 accounting, use the paper's `1059`; for a strict parameter
calculator, use the ceiling rule on the exact distance value. The paper then states the dominant
extra column payload as:

```text
1059 * 8 * t bytes
```

That is the spreadsheet formula `Q_RAA * field_bytes * t` with `field_bytes = 8`. If we instantiate
the same proof over GF(2^128), this row becomes `1059 * 16 * t`.

### Blaze Spreadsheet Rows

| Component | Count | Unit bytes | Total bytes | Notes |
| --- | ---: | ---: | ---: | --- |
| Interleaving/evaluation vector | `t` or `2 * t` | `field_bytes` | `=t*field_bytes` or `=2*t*field_bytes` | Lemma 6.1 has `2t + cc(k)` non-oracle communication; Section 8.2 folds this into "constant number of columns". |
| RAA queried columns | `Q_RAA * t` | `field_bytes` | `=Q_RAA*t*field_bytes` | Explicit paper row: `1059 * 8 * t` bytes. |
| RAA column Merkle paths | `Q_RAA * log2(n / t)` | `hash_bytes` | `=Q_RAA*LOG2(n/t)*hash_bytes` | Section 8.2 says 1059 paths for a tree with `n/t` leaves. |
| Inner BaseFold proof | `1` | `holographic_basefold_bytes(k_comp=n/t, N_comp)` | `=holographic_basefold_bytes(...)` | Backend term for `C_sys(c_star)`, excluding Blaze-authenticated systematic openings. |
| Small scalar overhead | `O(log n)` | `field_bytes` | small | From the MLIOP/IOPP and batching reductions. |
| Commitment roots | constant plus inner roots | `hash_bytes` | small | Public commitment root is not counted as opening proof unless the benchmark includes it. |

Dominant Blaze size:

```text
blaze_bytes ~= holographic_basefold_bytes(k_comp = n / t, N_comp)
             + Q_RAA * t * field_bytes
             + Q_RAA * log2(n / t) * hash_bytes
             + O(t * field_bytes + log(n) * field_bytes + hash_bytes * log(n/t))
```

The important first-principles point is that Blaze should not include a full `t * n` row-combination
vector. That is exactly the object Blaze avoids by using the inner BaseFold proof. The price paid is
many RAA column queries plus their Merkle paths.

## Paper Figure 4 Values

Blaze Figure 4 reports proof sizes in MB:

| Variables | Input MB | BaseFold MB | Blaze MB | Interleaved Blaze MB |
| ---: | ---: | ---: | ---: | ---: |
| 25 | 256 | 1.2 | 1.3 | 17.6 |
| 26 | 512 | 1.2 | 1.4 | 17.7 |
| 27 | 1024 | 1.4 | 2.0 | 18.0 |
| 28 | 2048 | 1.4 | 2.5 | 18.5 |
| 29 | 4096 | 1.4 | 3.7 | 19.7 |
| 30 | 8192 | x | 3.8 | 21.8 |
| 31 | 16384 | x | 6.1 | 26.2 |

These are consistent with the paper-level decomposition: BaseFold is dominated by `Q_BF` Merkle
paths across `D` shrinking layers, while Blaze is "smaller BaseFold plus RAA column openings".
The missing concrete parameter in the text is the exact `t` used per row of Figure 4; the formulas
above expose that as an explicit spreadsheet input.

## 5% Reproduction Model

The paper does not print every concrete knob needed to regenerate Figure 4 exactly. In particular,
it omits the exact interleaving schedule and the exact BaseFold query-count schedule used for the
plotted standalone sizes. With a 5% tolerance, the Figure 4 rows can be reproduced by fitting only
the hidden security/query knobs while keeping the proof components above fixed.

For BaseFold, using

```text
BF_MB = (roots + final + Q_BF * (SUM_i log2(N_i) * hash_bytes
        + (D + 1) * field_bytes)) / 1_000_000
```

with `field_bytes = 8`, `hash_bytes = 32`, `N_i = 2^(i+1)`, and the integer `Q_BF` implied by the
rounded paper table gives:

| Variables | Paper MB | Implied `Q_BF` | Model MB | Error |
| ---: | ---: | ---: | ---: | ---: |
| 25 | 1.2 | 105 | 1.199 | -0.06% |
| 26 | 1.2 | 98 | 1.205 | 0.41% |
| 27 | 1.4 | 106 | 1.399 | -0.07% |
| 28 | 1.4 | 99 | 1.399 | -0.04% |
| 29 | 1.4 | 93 | 1.405 | 0.34% |

The variation in implied `Q_BF` is almost certainly table rounding plus parameter tiers, not a deep
protocol effect. The useful conclusion is that the standalone BaseFold row is consistent with about
`100` query repetitions and normal Merkle path accounting.

For Blaze, the author's benchmark source uses a fixed row length `2^21` and varies the number of
rows. The paper formula says the incremental Blaze term is `Q_RAA * 8 * t` bytes, with `Q_RAA =
1059`, plus a holographic BaseFold/compiler component. Fitting that component as a constant `0.477 MB`
and solving for the effective opened-column count gives:

| Variables | Paper MB | Effective `t` | Model MB | Error |
| ---: | ---: | ---: | ---: | ---: |
| 25 | 1.3 | 97 | 1.299 | -0.07% |
| 26 | 1.4 | 109 | 1.400 | -0.02% |
| 27 | 2.0 | 180 | 2.002 | 0.10% |
| 28 | 2.5 | 239 | 2.502 | 0.07% |
| 29 | 3.7 | 380 | 3.696 | -0.10% |
| 30 | 3.8 | 392 | 3.798 | -0.05% |
| 31 | 6.1 | 664 | 6.102 | 0.03% |

These `t` values should be read as effective proof-size parameters, not necessarily literal powers
of two for an implementation. If we force power-of-two row counts and the printed `1059 * 8 * t`
term, the table cannot be matched within 5%; the max error is closer to 9%. That is a useful
warning: the printed Figure 4 sizes depend on an unstated implementation/rounding convention.

## Correctness Target For Our Implementation

For proof-size regression accounting, the implementation should be measured against these
component budgets:

1. BaseFold should have one chain of folded oracle commitments, sumcheck messages, and query
   decommitments over a systematic/holographic compiler code. It should not commit to a fresh
   full-size object inside each query.
2. Blaze should add `Q_RAA` column openings and `Q_RAA` column Merkle paths, then delegate the
   reduced claim to one smaller holographic BaseFold/compiler proof.
3. Any proof-size term proportional to `Q_RAA * t * log(n/t)` field elements, `Q_RAA * n`, or a full
   row-combination vector is not part of the paper Blaze proof shape.
4. The implementation must keep PRAA and compiler-code lengths separate:

```text
k_praa      = per-row PRAA message length
n_praa      = per-row PRAA codeword length = compiler systematic length
N_comp      = compiler systematic length + compiler parity length
Q_RAA       = exact number of Blaze-authenticated compiler-systematic input queries
Q_backend   = separate backend proof-query count
```
