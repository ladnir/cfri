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
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le2extra.csv
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le5extra.csv
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le6extra.csv
```

with:

```text
alpha=2: total log2 union = -99.60689084
alpha=5: total log2 union = -21.53925283
alpha=6: total log2 union =  25.89359792
```

## Recursive Invariant

The recursive encoder should carry a virtual-core triple:

```text
C_node      matched stride core at the current node,
H_node      holes inside C_node,
E_node      actual output leaves outside C_node.
```

with:

```text
supp(Ax at node) = (C_node \ H_node) union E_node,
|E_node| = E_defect(node) + |H_node|,
|H_node| <= alpha * E_defect(node).
```

Here:

```text
E_defect(node) = wt(Ax at node) - |C_node|.
```

The transitions are:

```text
one-child:
  C, H, and E all lift to both parent siblings.
  Both |H| and E_defect double, so |H| <= alpha E_defect is preserved.

balanced two-child:
  after overlap/cancellation charges, an actual parent core survives.
  This contributes no new holes; extra leaves are paid by the usual charged-tree records.

unbalanced two-child:
  choose a parent residue core, allow local holes, and use the alpha-2 local lemma to charge those
  holes to local output defect.
```

First-divergence accounting is still needed to ensure that extra leaves used by an unbalanced node
are not reused by lower nodes. The natural rule is unchanged: a leaf outside the final virtual core
is charged at the highest node where its path diverges from the chosen core path. Holes are charged
to extra leaves created at the same highest divergence.

## Hole First-Divergence Injection

Fix the final virtual matched core `C_root`. Every final output position has a path through the
binary recursion tree. For a leaf `ell notin C_root`, define:

```text
first(ell) = highest node where ell leaves the selected core residue path.
```

For a hole `h in H_root`, define `birth(h)` as the highest node where the selected virtual core
path required a child coordinate or sibling that was not present in the actual support. In the
one-child and balanced cases, no new holes are born. Thus every hole birth is at an unbalanced
node.

At an unbalanced node, the alpha-2 local lemma gives a set of non-core parent outputs whose
cardinality exceeds the number of parent-position holes by at least half the number of holes. First
reserve one non-core output as a replacement for each hole. The remaining non-core outputs are true
local defect leaves. Choose the paying outputs from these true defect leaves before recursing below
the node. Their descendants have:

```text
first(ell) = that unbalanced node.
```

Therefore they cannot be used by lower nodes, whose charged leaves have strictly lower first
divergence. They also cannot collide with charges from sibling subtrees, because their path prefixes
already differ at the charging node.

This gives the global inequality:

```text
|H_root| <= 2 * D_root
```

where `D_root` is the number of true outside defect leaves after one replacement leaf has been
reserved per hole. Equivalently, if the final support is `(C\H) union E_out`, then:

```text
|E_out| = |H| + e,
|H| <= 2e.
```

The local paying outputs must be selected injectively within each unbalanced node after child
defects, complete extra stride-class dimensions, and paid cancellation charges are removed.

The proof obligation left here is finite and local:

```text
At each unbalanced node, partition the non-core parent outputs into replacement leaves and true
defect leaves so that every two local holes have one distinct true defect leaf.
```

Once this is done, first-divergence makes the global alpha-2 injection automatic.

The clean composition form is:

```text
E_v = final outside leaves whose first divergence is v,
R_v subset E_v replacement leaves,
D_v subset E_v true defect leaves,
R_v cap D_v = empty,
|R_v| = h_v,
h_v <= 2 |D_v|.
```

Summing over unbalanced nodes gives:

```text
|H_root| = sum_v h_v <= 2 sum_v |D_v| <= 2e.
```

Balanced-node charges are placed in the same first-divergence partition, but they only add true
defect leaves and create no holes. Dimension growth is paid separately by the
`q^floor((e+h)/|C|)` factor and does not require another leaf injection.

## Local Proof Obligation

At one unbalanced node of output length `K`, live row split:

```text
M = a+b,     a != b,
L = K/M,
```

For the systematic certificate strata under discussion, `M` is a power-of-two divisor of `K` at the
node where the parent virtual core is selected. Therefore, if `L>=2`, the parent residue projects to
one child residue class in each sibling half. Choose any parent residue `rho mod M`. Its child
projection `C_child` has size:

```text
R = L/2.
```

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

Here is the intended local inequality after child defects and local cancellation charges have been
paid.

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
`c_O` the number of vanished parent siblings over coordinates in `S\C`.

The cancellation input is:

```text
c_C + c_O <= 1.
```

This is not a statement about arbitrary child near-supports. The proof first fixes the final
virtual support and pays excess cancellation equations using the global admissible rank budget and
no-early-gluing loss. On the remaining local skeleton, the two child outputs have only one relative
scalar. The quantitative no-early-gluing lemma then says two different vanished siblings would
impose two incompatible fresh-random ratio equations.

The reduction to this one-scalar skeleton is tracked in:

```text
docs/rfc_distance_analysis/rfc_uncharged_skeleton_reduction.md
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

The last inequality is deterministic because `a+b=M`, so one of `a,b` is at most `M/2`; hence:

```text
max(N/a, N/b) >= 2N/M = K/M = L.
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

This holds for every parent residue `rho`, not just for a Hall-selected residue. The unbalanced
case therefore no longer needs an existence argument for a well-covered residue; arbitrary residues
may have holes, but the same size inequality supplies proportional outside mass.

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

Equivalently, after reserving one outside non-core parent output for each parent-position hole,
`e_local` is the number of true local defect leaves left over.

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

The integer inequality is checked by:

```text
scripts/rfc_distance_analysis/rfc_check_alpha2_local_inequality.py --max-r 64
```

which reports:

```text
checked_R=1..64 failures=0
```

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
charge child defects and local extra dimensions until the remaining comparison has one relative
scalar. This is the same interface as the balanced proof: no-early-gluing prevents multiple
uncharged cancellations in an exact skeleton, while additional degrees of freedom are paid local
defects.

Equivalently, excess cancellation is a rank/probability issue before it is a hole issue. The
current proof route does not pay fresh child-rank dimension units at every node. Instead, fix the
final virtual support, use its global admissible dimension budget, and pay any remaining excess
cancellations by no-early-gluing probability loss. After those paid cancellations are removed, the
alpha-2 size inequality only sees the residual case `c_C+c_O<=1`.

The isolated exchange lemma and the global-rank pivot are:

```text
docs/rfc_distance_analysis/rfc_rank_cancellation_exchange.md
docs/rfc_distance_analysis/rfc_global_rank_budget_pivot.md
```

## Status

One-child and balanced two-child nodes already preserve actual cores. This theorem is only needed
for unbalanced two-child nodes. The deterministic size part of the unbalanced local theorem is now
reduced to the alpha-2 inequality above. The remaining main proof obligation for the ceiling-level
systematic distance certificate is the global rank-cancellation reduction recorded in:

```text
docs/rfc_distance_analysis/rfc_uncharged_skeleton_reduction.md
```

namely, after globally paid excess cancellations are removed, each local comparison has only one
residual cancellation visible to the alpha-2 size lemma, so `c_C+c_O<=1`.
