# Blaze Rewrite Target

This is the target shape for replacing the current Blaze implementation. The current code should be
treated as an imported reference plus regression harness, not as the design to preserve.

Source: Blaze, Section 6 interleaving lift and Section 8 proof-size discussion.

## Paper-Shaped Protocol

Let the committed object be an interleaved codeword `c in F^(t x n)`, with message
`m in F^(t x k)`. The claimed evaluation point is split as:

```text
z = (z1, z2)
z1 in F^log(t)
z2 in F^log(k)
```

The opening proof should contain:

1. `u in F^t`, where `u_i = m_i(z2)`.
2. A check that `u(z1) == claimed_eval`.
3. A verifier challenge `r in F^t`.
4. The folded codeword `c_combo = r^T c`, accessed through queried columns of the original
   Merkle commitment.
5. A single inner IOPP/backend proof for the base relation:

```text
RMLE[C](z2, <u, r>, c_combo)
```

For our backend, this is the only place BaseFold should appear in the Blaze proof.

## Current Implementation Gap

The current implementation does the first three steps, but then diverges:

1. It creates RAA intermediate words `u1`, `u2`, `u4`.
2. It creates permutation/binding polynomials.
3. It commits to those auxiliary objects with BaseFold.
4. It batch-opens those auxiliary objects.
5. It separately commits to and batch-opens the folded final codeword at query positions.

This produces two large BaseFold batch openings. That is not the paper shape.

## Rewrite Rule

The new Blaze path should not BaseFold-open RAA intermediate words or permutation helper
polynomials.

The new prover/verifier should have:

```text
Blaze proof =
    row evaluation vector u
  + one backend proof for folded base relation
  + q_RAA queried columns from the interleaved commitment
  + q_RAA Merkle paths
  + small scalar/root overhead
```

Any term proportional to:

```text
q_RAA * number_of_auxiliary_polynomials * Merkle_path_length
```

means the implementation has drifted back into the old shape.

