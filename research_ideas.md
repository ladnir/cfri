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
