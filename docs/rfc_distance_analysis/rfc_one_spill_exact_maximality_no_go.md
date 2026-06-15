# One-Spill Exact-Maximality Local No-Go

Status: local no-go for recovering the missing one-spill q-dimensions from exact maximality alone.

## Purpose

The common-zero replacement theorem uses the full set:

```text
C(x,y) = {j in T : x_j = y_j = 0}.
```

The one-spill ledger currently upper-bounds exact buckets by conditioning on:

```text
C subset C(x,y).
```

It is natural to ask whether enforcing exact maximality:

```text
C(x,y) = C
```

recovers the missing `30.196314` q-dimensions in the dominant one-spill row.

Conclusion:

```text
Not locally. Exact maximality is already what justifies the residual root exponent;
the remaining maximality conditions are inequalities and do not add q-codimension.
```

## Dominant Row

The row is:

```text
h = 864
a = |C| = 87
t = 1845
T\C size = 1758.
```

The projective witness space inside:

```text
H_Z plus H_Z
```

has q-dimension:

```text
2h - 1 = 1727.
```

For a fixed projective witness line, every coordinate in `T\C` that is not common-zero imposes one
root equation. The residual root exponent is therefore:

```text
(t-a) - (2h-1)
  = 1758 - 1727
  = 31.
```

This is exactly the `root_residual_q=31` already charged in the one-spill ledger.

## What Exact Maximality Adds

Exact maximality additionally says:

```text
for every j in T\C, not (x_j = 0 and y_j = 0).
```

For fixed `Z=P union C`, these are nonvanishing inequalities on the witness pair `(x,y)`.
Nonvanishing inequalities can remove lower-order or constant factors, but they do not generally
reduce the algebraic q-dimension of the witness family.

In particular, the complement of a union of coordinate kernels inside a large vector space still
has the same q-dimension as the vector space unless the kernels cover the whole space. Therefore
there is no local q-codimension budget here comparable to the missing:

```text
30.196314 q-dimensions.
```

## Diagnostic

The bookkeeping script:

```text
scripts/rfc_distance_analysis/rfc_one_spill_maximality_budget.py
```

prints:

```text
outside_singletons_t_minus_a,1758
projective_witness_qdim_2h_minus_1,1727
residual_root_q_already_charged,31
maximality_inequality_extra_qdim,0
missing_qdim_after_structural_ledger,30.196314
remaining_after_local_maximality,30.196314
```

## Consequence

The exact-maximality condition is important, but it is not an extra local saving for this row.
It prevents us from overcounting outside common-zero coordinates and supports the existing
residual-root charge. It does not close the remaining gap.

The remaining viable sources are:

```text
1. survivor-profile coupling:
   count P, C, and T\C jointly with the lower one-spill structure;

2. witness de-duplication:
   count canonical full common-zero sets or witness subspaces once instead of by many labels;

3. stronger lower tensor-rank theorem:
   improve the Segre-line lower event for the exact canonical triple stratum;

4. a nonlocal theorem tying minority labels to outside non-common-zero coordinates.
```

This rules out the simplest exact-maximality recovery. The next push should target global
survivor-profile coupling or witness de-duplication.
