# RFC Near-Core Containment Theorem

This note states the remaining near-stability theorem in a proof-ready form.

## Target

Let `A_d` be one generic RFC transform, `k=2^d`, and let `x` be nonzero with:

```text
wt(x) = m
wt(A_d x) = k/m + e.
```

The target theorem is:

```text
supp(A_d x) contains a matched stride core C of size k/m,
and supp(A_d x) \ C has size e.
```

Equivalently, after deleting exactly `e` output leaves from `supp(A_d x)`, the remaining support is
the output support of an exact uncertainty extremizer.

This is the strongest structural form. The distance certificate can tolerate a weaker counted form:

```text
near support/output pairs with defect e
  <= k * binom(k-k/m, e) * 2^(B e)
```

for a moderate overhead `B`. The depth-11 certificate has enough field slack that even
`B=64` leaves the corrected union bound unchanged. Therefore the proof does not need the exact
binomial count if row-split or overlap pruning naturally introduces a polynomial-in-`k` charge
label.

The counted theorem is split out in:

```text
docs/rfc_counted_charged_tree_theorem.md
```

## Pruning Form

The clean induction is a pruning theorem.

At any node of output length `K`, live row weight `M`, and output support `Y`, define:

```text
E = |Y| - K/M.
```

The theorem says:

```text
There exists a set P subset Y with |P| <= E such that Y \ P is a matched core of size K/M.
```

At the root this is exactly the target statement. The set `P` is the set of charged extra leaves.

The counted fallback still needs this core-preservation statement. It relaxes the uniqueness and
exact charge injection, but not the requirement:

```text
matched core C subset Y.
```

Without `C subset Y`, the support count would have to include holes inside the core as well as
extras outside it.

## One-Child Step

If the live rows occupy only one child, then the parent output support is the two-sibling lift of
the child output support:

```text
Y = lift(Y_child)
|Y| = 2 |Y_child|
K/M = 2 K_child/M
E = 2 E_child.
```

By induction choose:

```text
P_child subset Y_child
|P_child| <= E_child
Y_child \ P_child = C_child.
```

Then set:

```text
P = lift(P_child).
```

This gives:

```text
|P| = 2 |P_child| <= E
Y \ P = lift(C_child),
```

and `lift(C_child)` is the matched stride core at the parent.

## Two-Child Balanced Step

Assume the live rows split evenly:

```text
M = 2a.
```

Let child output supports be:

```text
U, V
```

with:

```text
p = |U|
q = |V|
h = |U cap V|
L = K/M.
```

Write child defects:

```text
e_0 = p - L
e_1 = q - L.
```

The local defect decomposition gives:

```text
E >= e_0 + e_1 + (p-h) + (q-h) + max(0,h-1).
```

Inductively prune the children:

```text
P_0 subset U, |P_0| <= e_0, U \ P_0 = C_0
P_1 subset V, |P_1| <= e_1, V \ P_1 = C_1.
```

The remaining task at this node is to pay for the mismatch between `C_0` and `C_1`, and then pay
for all but one common continuation coordinate.

For the counted certificate, the core-preservation part of this step is now settled in the weaker
form we need. A child core contained in either `U` or `V` lifts to a parent residue core. The parent
residue geometry splits the child core into two sub-residues modulo `M`; each parent residue
candidate uses both siblings over one sub-residue. Quantitative no-early-gluing can destroy at most
one sibling over the whole child core, so at most one of the two parent residue candidates is lost.
The other candidate is fully contained in the actual parent output support.

Thus overlap and cancellation still create charged leaves for counting, but they no longer threaten
existence of an actual contained core in the balanced case.

## Overlap Pruning

The symmetric difference:

```text
(U \ V) union (V \ U).
```

A coordinate in `U triangle V` has only one active child value, so both parent siblings are nonzero
and it cannot participate in exact cancellation at this node. The local defect identity allocates:

```text
(p-h) + (q-h)
```

units to this mismatch.

The remaining overlap-pruning lemma is:

```text
After child pruning has exposed child cores, the parent leaves forced by U triangle V can be assigned
injectively to output leaves outside the final parent core, with total cost bounded by the overlap
charge plus the lifted child charges.
```

This is the first place where the proof is genuinely global rather than purely local. The local
identity says the budget exists; the pruning lemma must show the same leaves are not also needed for
child-defect or cancellation charges.

## Cancellation Pruning

After overlap pruning, only the common coordinates remain. Quantitative no-early-gluing says at
most one common coordinate can participate in exact cancellation generically. Choose that coordinate
as the continuation coordinate.

For every other common coordinate, at least one sibling parent leaf is outside the exact skeleton.
Charge one such leaf. This uses:

```text
max(0,h-1)
```

charges.

After deleting these charged leaves, and after resolving overlap mismatch, the uncharged support
follows the exact two-child glue step:

```text
one continuation coordinate,
one surviving sibling choice,
balanced live rows.
```

Thus the uncharged node is the full-live matched glue skeleton.

## Unbalanced Row Split

If the row split is unbalanced, the split defect is positive:

```text
ceil(max((K/2)/a, (K/2)/b)) - ceil(K/(a+b)) > 0.
```

This defect must be charged before the balanced two-child pruning can apply. The intended pruning
rule is:

```text
delete arbitrary extra parent leaves outside a maximal child-supported exact skeleton
until the remaining uncharged support has either one live child or a balanced split.
```

The local lower bound in:

```text
docs/rfc_near_defect_charge_injection.md
```

shows enough slack exists. The remaining detail is to make this deletion canonical and disjoint from
the overlap/cancellation charges.

## First-Divergence Injection

Once an exact skeleton core `C` is selected, every charged output leaf `ell notin C` has a unique
highest node where its path first diverges from the core path.

Use:

```text
first(ell)
```

to assign charges. Charges from different nodes cannot collide, and charges from different child
coordinates at the same node have different suffixes.

This is the intended reason local pruning costs add globally:

```text
|P| <= total local defect <= E(root).
```

## Current Gap

The local defect decomposition is now explicit, and the one-child and balanced two-child
core-preservation cases are reduced:

```text
one-child:          child core lifts directly;
balanced two-child: child core lifts through one surviving parent residue by no-early-gluing.
```

The central remaining global lemma is therefore the unbalanced row-split part of core preservation:

```text
after charging local defects, at least one exact matched skeleton remains inside the actual support.
```

The remaining sublemma is:

```text
unbalanced row-split pruning:
  turn the positive split-defect lower bound into either an actual contained parent core or a
  globally tiny set of virtual holes.
```

This is now isolated as a finite residue-Hall problem in:

```text
docs/rfc_unbalanced_split_residue_hall.md
```

The current evidence says literal actual-core containment is likely too strong in this unbalanced
case. The certificate-sufficient replacement is defect-coupled virtual containment:

```text
number of virtual core holes <= number of real extra output leaves.
```

The union bound remains essentially unchanged under this replacement, because the dangerous hole
terms were the artificial `extra=0, holes>0` cases.

For the all-level systematic certificate, this gap is less dangerous than cancellation because
unbalanced splits have explicit integer cost and cannot occur in the uncharged skeleton. Still, the
hole fallback is narrow: with `B=64`, the depth-11 `c=8` bound stays negative through `H=5` virtual
core holes but fails by `H=8`. So the unbalanced proof should aim for actual core containment, using
the bounded-hole model only as a backstop.

## Counted Fallback

If exact pruning to:

```text
k * binom(k-k/m, e)
```

is awkward, it is enough to label each charged defect by:

```text
node,
charge type,
local coordinate,
optional side bit.
```

This costs at most a moderate factor per charge, for example:

```text
2^64
```

per charged defect is already far larger than the natural `O(d k)` label count at `d=11`,
`k=2048`. Even if the label must also carry a charged-leaf marker for repeated charges on the same
final leaf, the crude natural budget is below `2^37`, still well under the tested `B=64` model.
The label alphabet also includes a `residual` type for output leaves outside the selected skeleton
that are not minimal row-split/overlap/cancellation witnesses.

The script:

```text
scripts/rfc_near_extremizer_total_union.py
```

supports this via:

```text
--charge-overhead-log2
```

The artifacts:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead16.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead32.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead104.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead108.csv
```

give:

```text
B=16:  total log2 union = -99.60768258
B=32:  total log2 union = -99.60768258
B=64:  total log2 union = -99.60768258
B=104: total log2 union = -98.65570921
B=108: total log2 union = -67.37561327
```

So the proof strategy can prioritize a robust charged-tree count over the exact strongest
matched-plus-extra classification. The break point is between `B=108` and `B=112`, so there is far
more overhead budget than a natural charged-tree labeling should need.

The recursive encoder for this counted route is drafted in:

```text
docs/rfc_counted_charged_tree_theorem.md
```
