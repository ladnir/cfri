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

## Alpha-2 Local Skeleton

Here is the intended local inequality.

Let:

```text
N = K/2,
R = L/2,
C = child projection of the selected parent residue,
A = |C \ (U union V)|,
S = U union V.
```

Then:

```text
parent holes inside the selected residue <= 2A + 1.
```

The `2A` term is because a missing child coordinate removes both sibling positions of the parent
residue. The `+1` term is the possible single sibling cancellation inside `C`.

The child support lower bound should use the smaller child row weight:

```text
max(|U|, |V|) >= max(ceil(N/a), ceil(N/b)) >= L.
```

Since `|C|=R`, this gives:

```text
|S \ C| >= |S| - |S cap C|
        >= L - (R-A)
        = R + A.
```

For every coordinate in `S \ C`, the parent has support outside the selected residue. A one-sided
child coordinate gives two non-core parent outputs. A two-sided child coordinate gives two non-core
outputs except for a sibling cancellation, and no-early-gluing permits at most one such cancellation
over the selected local comparison. Therefore:

```text
outside non-core parent outputs >= 2(R+A) - 1.
```

Combining:

```text
local output defect
  = outside non-core parent outputs - parent holes
 >= (2R + 2A - 1) - (2A + 1)
  = 2R - 2.
```

This lower bound is already much larger than the hole count for all moderate `R`; the small cases
`R=1,2` should be checked directly. A sharper version keeps the actual `A` term and proves:

```text
parent holes <= 2 * local output defect.
```

This is the desired `alpha=2` local coupling.

The only delicate assumption in this skeleton is the cancellation bound outside `C`. It must be
stated with the same rational-function/no-early-gluing hypothesis used in the balanced proof: many
independent sibling cancellations at one node would impose incompatible fresh-random ratio
conditions.

## Status

One-child and balanced two-child nodes already preserve actual cores. This theorem is only needed
for unbalanced two-child nodes. It is the current main proof obligation for the ceiling-level
systematic distance certificate.
