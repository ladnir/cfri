# RFC Tau-2 Kernel Profile Target

This note isolates the first intermediate root-line kernel case.

## Baselines

For `A subset S`, write:

```text
a     = |A|
delta = dim U_A
```

For any projective root-line assignment `ell`, the kernel is cut out inside `U_A + U_A` by at most
`a` linear equations. Therefore:

```text
kappa_A(ell) >= kappa0(A)
kappa0(A) = max(0, 2 delta - a).
```

This is a deterministic lower bound.

It is not always the correct generic value. The correct generic baseline is:

```text
kappa_gen(A) = 2delta - r_gen(A),
```

where:

```text
r_gen(A) = min_{B subset A} |A \ B| + 2 rank_{U_A}(B).
```

This is the two-copy matroid-union rank of the coordinate matroid of `U_A`. When the two-copy union
has full possible rank:

```text
kappa_gen(A) = kappa0(A).
```

The rest of this note uses `kappa_gen`; the displayed `kappa0` values in the small examples agree
with `kappa_gen`.

For `tau=2`:

```text
if kappa_gen >= 2:
  every root-line assignment contributes.

if kappa_gen < 2:
  only assignments with a rank drop above the baseline contribute.
```

The hard case starts at:

```text
delta = 3, a = 5, kappa_gen = 1.
```

## Observed Pattern

The first profiles suggested a determinantal envelope:

```text
# { ell : kappa_A(ell) >= kappa_gen + h } ~= q^(a - h^2)
```

but this is too aggressive as a theorem target before the component/subsupport structure is fully
isolated. The robust target is layer-by-layer:

```text
h=1: codimension at least 1 by a nonzero maximal minor.
h=2: codimension at least 2 by a cofactor/singular-locus argument,
     after splitting direct-sum/component factors.
```

The full-rank spike is already understood:

```text
# { ell : kappa_A(ell) = delta } = (q+1)^comp(A).
```

So the current tau-2 target is:

```text
N_h(A) = # { ell : kappa_A(ell) >= kappa_gen(A) + h }

N_1(A) <= poly(a) * q^(a-1).
N_2(A) <= poly(a) * q^(a-2) + component/subsupport terms.
```

For `tau=2`, the local factor is:

```text
E_A(2) * q^-a,
```

where `E_A(2)` is obtained by exact-support inversion from:

```text
sum_ell GaussianBinomial(kappa_A(ell), 2)_q.
```

## Evidence

### Rank 2, Size 4

Here `delta=2`, `a=4`, so:

```text
kappa_gen = 0.
```

At `GF(11)`, child depth `1`, expansion `4`, size `4`, `tau=2`:

```text
connected:     0:17424;1:3300;2:12
two components 0:17424;1:3168;2:144
```

The `kappa >= 1` mass is about `q^(a-1)`, and the full-rank mass is `(q+1)^comp`.

### Rank 3, Size 5

Here `delta=3`, `a=5`, so:

```text
kappa_gen = 1.
```

At `GF(7)`, child depth `2`, expansion `4`, size `5`, `tau=2`:

```text
connected:       1:28224;2:4536;3:8
three components 1:25088;2:7168;3:512
```

The `kappa >= 2` mass is about `q^(a-1)`, while the full-rank mass is `(q+1)^comp`.

### Rank 4, Size 5

Here `delta=4`, `a=5`, so:

```text
kappa_gen = 3.
```

All assignments already have `kappa >= 2`. The profile is baseline plus the full-rank spike:

```text
connected:       3:32760;4:8
two components:  3:32704;4:64
three components 3:32256;4:512
```

## Interpretation

The tau-2 local proof should not try to enumerate visible subspaces directly. The right route is:

```text
1. Use the two-copy matroid-union value `kappa_gen` for the generic kernel floor.
2. Prove a rank-drop tail for root-line assignments above `kappa_gen`.
3. Add direct-sum/component spikes recursively.
```

This mirrors the earlier finite-field lift-tail correction: global excess alone is not enough;
the correct exponent is a local bottleneck profile.

## Current Proof Steps

The preferred route for the weighted `tau=2` count is now:

```text
root-line kernel polynomial
  -> ordered bases of 2-planes
  -> exterior variety V_2(U_A)
  -> component codimension.
```

This is written in:

```text
docs/rfc_distance_analysis/rfc_tau2_weighted_exterior_bound.md
```

The `h=1` bound is written:

```text
# { ell : kappa_A(ell) >= kappa_gen(A)+1 } <= poly(a) * q^(a-1)
```

This controls the first nontrivial intermediate case and is enough for rows with:

```text
kappa_gen(A)=1.
```

This `h=1` bound is now written in:

```text
docs/rfc_distance_analysis/rfc_h1_kernel_drop_bound.md
```

The next case needed for `tau=2` is:

```text
kappa_gen(A)=0,
kappa_A(ell) >= 2.
```

This is the `h=2` layer. It is reduced to a cofactor/singular-locus bound in:

```text
docs/rfc_distance_analysis/rfc_h2_cofactor_bound.md
```

However, the `tau=2` local polynomial is weighted:

```text
sum_ell GaussianBinomial(kappa_A(ell),2)_q.
```

Therefore an assignment-count bound for `kappa>=2` closes the case `dim U_A<=2`, but not larger
`dim U_A`. For larger support subcodes we still need higher-layer bounds or a direct weighted
singularity estimate.

The exterior-variety reduction supplies that direct weighted estimate, assuming the component
codimension theorem:

```text
dim V_2(U_A) <= 2 delta(A) + comp(A).
```
