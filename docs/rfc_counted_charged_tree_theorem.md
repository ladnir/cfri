# RFC Counted Charged-Tree Theorem

This note states the weaker near-stability result that is sufficient for the systematic distance
certificate.

## Motivation

The strongest hoped-for theorem is exact containment:

```text
near support pair = matched core + e arbitrary extra output leaves.
```

That gives the count:

```text
k * binom(k-k/m, e).
```

The certificate does not actually need this exact count. It can tolerate a labeled charged-tree
count:

```text
k * binom(k-k/m, e) * 2^(B e)
```

for a large constant `B` at the target depth.

## Theorem Target

Let `A_d` be one generic RFC transform, `k=2^d`, and let:

```text
wt(x) = m
wt(A_d x) = k/m + e.
```

The counted charged-tree theorem should prove that the support pair of `x` can be encoded by:

```text
1. a matched core identifier;
2. a set of at most e charged final output leaves outside that core;
3. one local charge label per unit of charge, from an alphabet of size at most 2^B.
```

Therefore the number of possible near support/output certificates is at most:

```text
k * binom(k-k/m, <= e) * 2^(B e).
```

For exact defect `e`, this can be used as:

```text
k * binom(k-k/m, e) * 2^(B e).
```

## Natural Label Budget

A charge label only needs enough information to reconstruct where the local defect was paid. A very
conservative label can include:

```text
node id:             at most 2k choices
charge type:         row-split / overlap / cancellation / residual
side bit:            at most 2 choices
local selector:      at most k choices
depth/order marker:  at most d choices
charged-leaf marker: at most e_max choices
```

Thus:

```text
label count <= 12 d e_max k^2.
```

At the certificate point:

```text
d = 11
k = 2048
e_max = 128
```

this is:

```text
log2(12*d*e_max*k^2) < 37.
```

So `B=64` comfortably covers this crude encoding, including repeated charges assigned to the same
final leaf. The charged-leaf marker tells which leaf in the chosen charged-leaf set a given local
charge uses. If an injective final-leaf assignment is proved, this marker can be removed and `B=32`
already covers the natural label budget.

## Slack Check

The corrected depth-11 systematic certificate was evaluated with:

```text
near count multiplier 2^(B e)
```

using:

```text
scripts/rfc_near_extremizer_total_union.py --charge-overhead-log2 B
```

Artifacts:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead16.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead32.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead104.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead108.csv
```

Results:

```text
B=16:  total log2 union = -99.60768258
B=32:  total log2 union = -99.60768258
B=64:  total log2 union = -99.60768258
B=104: total log2 union = -98.65570921
B=108: total log2 union = -67.37561327
```

The natural label budget is below `B=32`, and the certificate remains strong even at `B=108`.

## Proof Obligations

The local accounting is already isolated:

```text
parent defect >= child defects
               + row-split charge
               + overlap charge
               + cancellation charge.
```

To prove the counted theorem, it is enough to show:

```text
Every unit of local charge can be assigned to one charged final output leaf and one bounded-size
charge label. Repeated use of a final leaf is allowed if the charge label carries a charged-leaf
marker.
```

Unlike exact containment, this does not require proving a canonical minimal deletion set. It only
requires an injective accounting into:

```text
(final charged leaf, bounded label).
```

This should be much easier than proving the exact strongest structural classification.

## Recursive Encoder

The counted theorem can be proved by a recursive encoder `Encode(node, x_node, Y_node)`.

At each node, compute:

```text
K = node output length
M = wt(x_node)
E = |Y_node| - K/M
```

The encoder outputs:

```text
matched skeleton choices,
charged final leaves,
bounded labels for each charged unit.
```

### One-Child Case

If only one child is live, recurse into that child. Every child charge lifts to two parent leaves,
but the final-leaf component of the charge record already identifies the lifted leaf. The label adds
only:

```text
side bit.
```

The defect doubles:

```text
E_parent = 2 E_child.
```

so the lifted charge budget is exact.

### Balanced Two-Child Case

If both children are live and the row split is balanced, recurse into both children. The local
defect identity gives:

```text
E_parent >= E_left + E_right
          + overlap_charge
          + cancellation_charge.
```

Child charges keep their labels and gain a side bit. Then emit local charge records for:

```text
overlap units:       one record per coordinate in U triangle V;
cancellation units:  one record for every non-continuation coordinate in U cap V.
```

Each local record contains:

```text
node id,
charge type,
local coordinate or final leaf suffix,
side / continuation choice,
charged-leaf marker if needed.
```

The local terms above are minimal witnesses that exactness has failed. The actual parent support
may contain additional leaves outside the selected skeleton. Label every such remaining leaf with:

```text
charge type = residual.
```

This is safe because the total number of non-skeleton output leaves is exactly the defect budget at
that node. The local defect identity is used to prove an exact skeleton can be selected; the final
support count is paid by labeling all leaves outside that skeleton.

After these records are emitted, the uncharged local branch is exactly the exact-stability branch:

```text
balanced rows,
matching child supports,
one continuation coordinate.
```

Thus the skeleton follows the matched glue recursion.

### Unbalanced Two-Child Case

If both children are live but the row split is unbalanced, emit:

```text
rounded_split_defect
```

row-split charge records at the current node. The label records:

```text
node id,
charge type = row-split,
(a,b),
selector for a charged final leaf,
charged-leaf marker if needed.
```

After paying this charge, the encoder proceeds by selecting a maximal lower-defect branch to define
the skeleton and treats the other local mass as charged. This is the least formal step, but the
counted theorem only needs the emitted records to be bounded, not canonical.

As in the balanced case, any support leaf outside the final selected skeleton that was not already
used as a row-split witness receives a residual label.

## Why The Encoder Count Is Enough

For the certificate, we do not need to reconstruct `x`, only to union bound all possible bad support
certificates. A bad certificate is described by:

```text
matched core id,
charged final leaf set,
charge labels, each pointing to one leaf in that set.
```

The number of such descriptions is at most:

```text
k * binom(k-k/m, <= e) * 2^(B e).
```

Even if the same support pair has many descriptions, this is fine for a union bound.

## Relation To The Certificate

The systematic distance certificate can cite this theorem instead of exact near containment. The
first-moment contribution becomes:

```text
c_p * k * binom(k-k/m, e) * 2^(B e)
  * q^floor(e/(k/m))
  * binom((c_p-1)k, needed_zeros) q^(-needed_zeros).
```

At `d=11`, `c=8`, `q=2^128`, and `B<=64`, the displayed bound remains:

```text
Pr[d_sys < 12384] <= 2^-99.60768258.
```
