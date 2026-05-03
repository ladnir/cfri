# Research Ideas

## Compress Interleaved Column Openings

In paper-Blaze, the committed object is the interleaved packed RAA codeword matrix

```text
C in F^{t x n}
C_i = PRAA(m_i)
```

with Merkle leaves/opened addresses shaped as columns:

```text
C[:, j] = (C_0[j], C_1[j], ..., C_{t-1}[j]) in F^t
```

During opening, the verifier samples row-folding challenges `rho in F^t` and needs queried values of the folded codeword:

```text
C_star[j] = sum_i rho_i * C_i[j]
```

Paper-Blaze proves this by literally opening each queried column `C[:, j]`. For `F = GF(2^128)`, each queried column costs `16 * t` bytes before Merkle path overhead. This is simple and fast when `t = Theta(log n)`, but it makes proof size scale linearly with `t`.

Potential improvement: do not open the whole column. Instead, prove the inner product

```text
C_star[j] = sum_i rho_i * C(i, j)
```

using a sumcheck or backend PCS over the row dimension. Better, batch many queried columns `j in Q` by sampling coefficients `eta_j` and proving one combined statement:

```text
sum_j eta_j * C_star[j]
  = sum_i rho_i * sum_j eta_j * C(i, j)
```

This would require a commitment layout that supports these row-dimension inner-product claims, e.g. a 2D commitment/table model for `C(i, j)` rather than only raw column leaves.

Why this may matter: larger `t` shrinks the folded RAA/backend instance size because

```text
K = t * k
n = r * k = r * K / t
```

So reducing the `O(q * t * sizeof(F))` column-opening term could let us choose larger `t`, making the folded BaseFold/PipFRI backend smaller. This may be a concrete win for Blaze2, especially when composing Blaze with PipFRI.

Tradeoffs to measure:

- extra prover work for column inner-product proofs
- extra verifier work and transcript complexity
- whether batching all queried columns amortizes well enough
- memory layout impact for large `t`
- whether the global commitment address space can support both raw column openings and compressed inner-product openings cleanly

## Proper Same-Rate Systematic RFC Distance Analysis

We want BaseFold's compiler code to be systematic so a caller can authenticate the systematic
oracle externally:

```text
C_sys(y) = (y || parity(y))
```

The conservative construction

```text
C_aug(y) = (y, E_RFC(y))
```

is easy to reason about, but it worsens the rate and dilutes relative distance. It is a baseline, not
the target. The target is a **same-total-rate systematic random foldable code**:

```text
C_sys : F^k -> F^(c k)
C_sys(y) = (y || P(y))
```

where `P(y)` has length `(c - 1)k`, the full code remains foldable in the BaseFold sense, and the
distance is analyzed for the whole systematic code, not inherited through a trivial append-only
bound.

### Why The Naive Argument Is Not Enough

If `P` is just an RFC parity encoder of expansion `c - 1`, then a tempting distance lower bound is:

```text
wt(C_sys(y)) = wt(y) + wt(P(y))
             >= wt(y) + delta_parity * (c - 1)k
```

for nonzero `y`, giving roughly:

```text
delta_sys >= ((c - 1) / c) * delta_parity
```

This keeps the overall rate `1/c`, unlike `C_aug`, but it does not by itself prove that the combined
systematic-plus-parity code is BaseFold-foldable with the right query algebra. It is only a distance
calculation assuming the construction exists.

### The Main Algebraic Obstruction

BaseFold's foldable-code recurrence encodes a message split `(l, r)` into coordinate pairs of the
form:

```text
left_coord  = l_encoded[j] + T[j]  * r_encoded[j]
right_coord = l_encoded[j] + T'[j] * r_encoded[j]
```

and a verifier folds the two queried values by interpolating between the two points `T[j]` and
`T'[j]`.

A literal systematic block wants the two top-layer systematic coordinates to be:

```text
left_coord  = l[j]
right_coord = r[j]
```

The first equality can be obtained with `T[j] = 0`, but the second would require:

```text
l[j] + T'[j] * r[j] = r[j]   for all l[j], r[j]
```

which is impossible for finite `T'[j]` independent of `l[j]`. This is the core reason that a generic
"make the generator systematic" transform is not acceptable: it may preserve the linear code as a
set, but destroy the message-coordinate fold equation that BaseFold and the holographic compiler
need.

### Candidate Directions

1. **Projective fold point for the systematic block.**
   Treat the systematic pair `(l[j], r[j])` as evaluations at `(0, infinity)`, so folding at
   challenge `alpha` gives:

   ```text
   l[j] + alpha * r[j]
   ```

   This matches the desired message fold exactly, but requires extending the BaseFold fold equation
   and verifier query logic to support an infinity point cleanly. We would need to prove the
   soundness/distance argument still follows, or isolate it as a valid foldable-code variant.

2. **Construct a foldable parity code around a systematic projective block.**
   Keep the systematic coordinates literal and use normal RFC-style finite fold points only for the
   parity block. The global query schedule would have typed fold rules:

   ```text
   systematic pair: projective fold
   parity pair:     RFC finite-point fold
   ```

   This is attractive for performance because systematic queries are exactly the external oracle
   values Blaze can open. It needs a formal definition of "mixed foldable code" and a distance
   proof.

3. **Find a row-basis construction that is systematic and foldable simultaneously.**
   Search for a recursive generator family `G_i = [I | P_i]` that satisfies a BaseFold-style
   recurrence in the natural message basis, not merely up to a row operation that changes the
   meaning of the systematic coordinates. If row operations are used, the verifier must still be
   able to treat the first `k` codeword coordinates as the original oracle `y`.

4. **Distance analysis for same-rate systematic RFC.**
   Once a valid construction exists, redo the RFC zero-count/union-bound analysis for:

   ```text
   C_sys(y) = (y || P(y))
   ```

   The analysis should stratify by `s = wt(y)`. The systematic block already contributes `s`
   nonzero positions, while the parity block contributes according to the random foldable parity
   distribution conditioned on that message weight. The goal is a concrete bound better than the
   trivial append-identity dilution and good enough to set `Q_backend` honestly.

### Acceptance Bar

Do not use a same-rate systematic RFC in the implementation until we have:

- a concrete recursive construction,
- exact fold equations for systematic and parity coordinates,
- a verifier query procedure with no hidden values,
- a distance bound for the resulting systematic code,
- query-count formulas using that bound,
- small exhaustive/algebraic tests for the fold equations,
- randomized distance spot checks for small parameter sets.
