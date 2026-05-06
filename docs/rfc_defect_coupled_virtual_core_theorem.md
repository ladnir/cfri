# RFC Defect-Coupled Virtual Core Theorem

This note states the replacement for literal near-core containment.

## Motivation

Literal containment:

```text
matched core C subset supp(Ax)
```

is probably too strong once an unbalanced two-child split occurs. A missing child coordinate under a
parent residue can remove both parent sibling positions from that residue. Small residue models show
that unit row-split charge can still create parent-residue holes.

The distance certificate does not require literal containment. It can tolerate virtual cores as long
as the holes are coupled to true output defect.

## Theorem Target

Let `A_d` be one RFC transform with `k=2^d`. Suppose:

```text
wt(x) = m,
wt(A_d x) = k/m + e.
```

The target theorem is that there exists:

```text
1. a matched stride core C of size k/m;
2. a hole set H subset C;
3. an outside set E subset [k]\C;
```

such that:

```text
supp(A_d x) = (C \ H) union E,
|E| = e + |H|,
|H| <= alpha e.
```

The proof target is:

```text
alpha = 2.
```

The current depth-11, `c=8`, `B=64` count remains safe through:

```text
alpha = 5.
```

and breaks at:

```text
alpha = 6.
```

So the proof has a small but real constant-factor margin.

## Counting Consequence

For fixed `m,e`, the support count becomes:

```text
k * sum_{h <= alpha e} binom(k/m, h) binom(k-k/m, e+h) * 2^(B e).
```

The near-kernel dimension factor must use outside-core extras:

```text
q^floor((e+h)/(k/m)).
```

The certificate calculator implements this with:

```text
--virtual-core-holes-coupled-to-extra
--virtual-core-hole-extra-factor alpha
```

The key artifacts are:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le2extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le5extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le6extra.csv
```

with:

```text
alpha=2: total log2 union = -99.60689084
alpha=5: total log2 union = -21.53925283
alpha=6: total log2 union =  25.89359792
```

## Local Proof Obligation

At one unbalanced node of output length `K`, live row split:

```text
M = a+b,     a != b,
L = K/M,
```

choose a parent residue `rho mod M`. Its child projection `C_child` has size `L/2` when `L` is even.
If:

```text
h_child = |C_child \ (U union V)|,
```

then these missing child coordinates create:

```text
2 h_child
```

parent-position holes. A sibling cancellation inside the residue can add at most one more hole.

The local theorem should show that the same unbalanced node creates enough non-core parent support
to satisfy:

```text
parent-position holes <= 2 * local output defect.
```

The proof should use:

```text
1. one-copy uncertainty for child support lower bounds;
2. local sibling-pair invertibility;
3. no-early-gluing to limit cancellations;
4. first-divergence accounting so lower nodes do not reuse the same defect.
```

The `L=1` case is harmless because a parent core has one position; any nonzero output position is a
valid core choice.

## Status

One-child and balanced two-child nodes already preserve actual cores. This theorem is only needed
for unbalanced two-child nodes. It is the current main proof obligation for the ceiling-level
systematic distance certificate.
