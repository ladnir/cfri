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

Here is the intended local inequality after local cancellation charges have been paid.

Let:

```text
N = K/2,
R = L/2,
C = child projection of the selected parent residue,
A = |C \ (U union V)|,
S = U union V.
```

Let:

```text
O = |S \ C|.
```

Also let `c_C` be the number of vanished parent siblings over covered coordinates of `C`, and
`c_O` the number of vanished parent siblings over coordinates in `S\C`. The charged-tree theorem
should pay for all but at most one local sibling cancellation at this node, so the uncharged local
skeleton satisfies:

```text
c_C + c_O <= 1.
```

Then:

```text
parent holes inside the selected residue = 2A + c_C.
```

The `2A` term is because a missing child coordinate removes both sibling positions of the parent
residue. The `c_C` term is the possible sibling cancellation inside `C`.

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

In other words:

```text
O >= R + A.
```

For every coordinate in `S \ C`, the parent has support outside the selected residue. Before
cancellations, this gives two non-core parent outputs per coordinate. After the remaining uncharged
cancellations:

```text
outside non-core parent outputs >= 2O - c_O.
```

Thus the local output defect relative to the virtual parent core is:

```text
e_local
  = outside non-core parent outputs - parent holes
 >= (2O - c_O) - (2A + c_C)
 >= 2(R+A) - c_O - 2A - c_C
  = 2R - (c_C+c_O)
 >= 2R - 1.
```

The parent-position hole count is:

```text
h_local = 2A + c_C <= 2R + c_C.
```

For `R>=2`:

```text
h_local <= 2R + c_C <= 2R+1 <= 2(2R-1) <= 2 e_local.
```

If `A=R`, then `c_C=0` and the same inequality is even tighter:

```text
h_local = 2R <= 2(2R-1).
```

Therefore the local alpha-2 coupling holds for every `R>=2` once all but one sibling cancellation
has been paid by the cancellation-charge mechanism:

```text
parent holes <= 2 * local output defect.
```

This is the desired `alpha=2` local coupling.

### Small Cases

`R=1` means `L=2`: the parent residue has two positions over one child coordinate. If that child
coordinate is present, the residue has at most one cancellation hole; if it is absent, both holes
are offset by at least `|S|>=L=2` outside child coordinates, giving at least four non-core parent
outputs before paid cancellations. So the same alpha-2 inequality holds after cancellation charges.

`L=1` is the full-live endpoint: a parent core has one position, and any nonzero output position is
a valid core choice.

### Cancellation Charge Interface

The delicate assumption is the bound:

```text
c_C+c_O <= 1
```

for the uncharged local skeleton. This should not be read as saying many local cancellations are
deterministically impossible for arbitrary near supports. Rather, the charged-tree proof must first
emit cancellation records for all but one vanished sibling at this node. This is the same interface
as the balanced proof: no-early-gluing prevents multiple uncharged cancellations in an exact
skeleton, while additional cancellations are paid local defects.

## Status

One-child and balanced two-child nodes already preserve actual cores. This theorem is only needed
for unbalanced two-child nodes. It is the current main proof obligation for the ceiling-level
systematic distance certificate.
