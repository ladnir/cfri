# RFC Core Preservation Hall Condition

This note reformulates the remaining core-preservation problem as a combinatorial survival
condition.

## Goal

For one generic RFC tree, suppose:

```text
wt(x) = m
Y = supp(A_d x)
|Y| = k/m + e.
```

We need to prove:

```text
Y contains a matched stride class C of size k/m.
```

Equivalently, there is a row block `R` of size `m` and a residue `r mod m` such that every output
leaf in:

```text
C = { r, r+m, r+2m, ..., r+(k/m-1)m }
```

is present in `Y`.

## Recursive Survival View

In the exact proof, a matched core is selected by a path of choices:

```text
one-child descent choices above the live row block,
then glue continuation choices inside the full live block.
```

For near stability, define at each node a set of surviving local continuation coordinates:

```text
S(node) = coordinates that can still be extended to a full matched core contained in Y.
```

At a one-child descent node:

```text
j in S(child) survives to parent iff both sibling lifts of j are present in Y_parent.
```

At a balanced two-child glue node:

```text
j survives iff j survives in both children and one parent sibling over j is present after local
cancellation.
```

The core-preservation theorem is:

```text
S(root) is nonempty.
```

## Hall-Style Obstruction

If no matched core survives, then every exact candidate core is hit by at least one missing output
leaf. Let:

```text
M_missing = [k] \ Y.
```

For a fixed live block size `m`, the matched cores are the `m` stride classes modulo `m`. Destroying
all stride classes requires:

```text
M_missing intersects every stride class.
```

This costs at least `m` missing leaves.

But the support size is:

```text
|Y| = k/m + e.
```

This is not directly useful globally because most output leaves are missing when `m` is small. The
right version is local: after one-child descent has restricted to the candidate live block, missing
both sibling lifts of a child coordinate destroys that child continuation. Destroying all
continuations at a node should force enough local defect to exceed the available `e`.

## Local Hall Target

At a node of length `K`, live weight `M`, and target core size:

```text
L = K/M,
```

let `S` be the set of child continuation coordinates that survive child recursion. The local Hall
target is:

```text
If every parent continuation is destroyed, then the local defect charge is at least |S|.
```

For one-child descent this is clear: destroying a child continuation means at least one of its two
sibling lifts is missing from `Y_parent`, while any present output outside the lifted surviving set
is an extra charged leaf.

For balanced two-child glue, destroying all continuations means one of:

```text
1. child surviving sets do not overlap;
2. overlap exists but every overlapping coordinate fails parent cancellation/sibling survival.
```

The overlap and cancellation charges in:

```text
docs/rfc_near_defect_charge_injection.md
```

are designed to pay for exactly these failures.

## Use In Counted Theorem

If the local Hall target is true at every node, then the counted charged-tree encoder can choose a
surviving continuation whenever the accumulated charge is at most the global defect `e`. All other
present leaves outside the selected exact skeleton receive residual labels.

Thus the remaining theorem can be phrased as:

```text
local defect charges satisfy a Hall condition for the survival of at least one exact matched core.
```

This is the next proof target.

## Minimal Counterexample Form

A clean proof should use a minimal counterexample.

Choose a smallest node, by output length `K`, for which there is an actual output support `Y` from
some live vector of weight `M` such that:

```text
|Y| = K/M + E
Y contains no matched core of size K/M.
```

Every active child node of this minimal counterexample satisfies core preservation by minimality.
So child supports contain child matched cores after deleting their child defects.

Then split into cases.

### Case 1: One Live Child

If only one child is live, then:

```text
Y_parent = lift(Y_child).
```

Any child matched core contained in `Y_child` lifts to a parent matched core contained in
`Y_parent`, because both sibling lifts are present for every child output coordinate. This
contradicts minimality.

Therefore a minimal counterexample cannot be a one-child node.

### Case 2: Balanced Two-Child Node

Let:

```text
U = supp(Ax_0)
V = supp(Ax_1)
L = K/M
p = |U|
q = |V|
h = |U cap V|.
```

By minimality, both children have contained matched cores:

```text
C_0 subset U
C_1 subset V.
```

If these child cores expose a common continuation coordinate and the parent has a surviving sibling
over that coordinate, then the parent has a matched core, contradiction.

Thus every possible child-core continuation is killed by either:

```text
1. child-core mismatch / overlap failure;
2. parent sibling cancellation failure.
```

The balanced defect identity gives budget for exactly these failures:

```text
E_parent >= E_0 + E_1 + (p-h) + (q-h) + max(0,h-1).
```

The local Hall lemma needed here is:

```text
If all common child-core continuations are killed, then
(p-h) + (q-h) + max(0,h-1)
is at least the number of child-core continuations that must be hit.
```

This separates into two sublemmas.

#### Core Alignment

First, the intersection:

```text
U cap V
```

must contain a child matched core. It is not enough for `U` and `V` to contain possibly different
cores. If no child core is contained in `U cap V`, then every child core contained in `U` has at
least one hole in `V`, or every child core contained in `V` has at least one hole in `U`.

The overlap charge:

```text
(p-h) + (q-h) = |U \ V| + |V \ U|
```

is exactly the budget for these holes. Thus the core-alignment lemma should say:

```text
Either U cap V contains a child matched core, or the missing-core obstruction is encoded by overlap
charge labels.
```

For the exact theorem, the overlap charge is zero, so `U=V` and alignment is automatic.

The useful combinatorial fact is stronger because child stride cores are disjoint.

Let `A(U)` be the set of child stride residues whose full stride class is contained in `U`, and
define `A(V)` similarly. Then:

```text
child cores contained in U cap V are exactly A(U) cap A(V).
```

If a residue lies in:

```text
A(U) \ A(V),
```

then its whole stride class is contained in `U`, but at least one leaf of that stride class is
missing from `V`. Since distinct stride classes are disjoint, these missing leaves are distinct.
Therefore:

```text
|U \ V| >= |A(U) \ A(V)|.
```

Similarly:

```text
|V \ U| >= |A(V) \ A(U)|.
```

Hence:

```text
overlap_charge = |U \ V| + |V \ U|
  >= |A(U) triangle A(V)|.
```

This is the precise Hall alignment statement: overlap charge pays for the symmetric difference
between the sets of child cores available on the two sides. In particular, if both children have at
least one available core but no common core, then overlap charge is at least:

```text
|A(U)| + |A(V)| >= 2.
```

The remaining balanced-node problem is then not alignment of child core sets; it is choosing a
common residue from `A(U) cap A(V)` that also survives parent sibling selection.

#### Parent Sibling Survival

Second, once a common child core:

```text
C' subset U cap V
```

is found, parent gluing must choose one surviving sibling over every coordinate of `C'`. The
no-early-gluing algebra says at most one coordinate can have the exact sibling cancellation pattern
unless the node is the full-live glue point. Therefore killing all parent continuations inside
`C'` costs cancellation labels:

```text
max(0, |C'|-1).
```

At the full-live node `|C'|=1`, no cancellation charge is needed and the exact glue step survives.

There is an indexing constraint here. A parent matched core is not an arbitrary choice of one
sibling over each child coordinate. The chosen siblings must form one parent stride residue modulo
`M`. Therefore the survival set should be indexed by parent residues:

```text
rho in {0, ..., M-1}.
```

Each parent residue `rho` projects to a child residue:

```text
rho mod (M/2).
```

and prescribes a deterministic sibling pattern over the child stride class. The parent sibling
survival lemma should be stated as:

```text
If a child residue lies in A(U) cap A(V), then every parent residue above it either survives as an
actual parent stride core, or is paid for by a cancellation/residual label.
```

In the exact full-live glue point, the child core has size one, so this sibling pattern is just one
local sibling choice. Above that point, no-early-gluing says the exact cancellation pattern cannot
hold for all coordinates of a multi-coordinate child core. In the near theorem, those failed
coordinates are exactly what the cancellation/residual labels pay for.

Since an exact child core has size `L` before the full-live glue point, killing all continuations
should cost at least `L`. But a parent with output size `L+E_parent` can only afford this if the
extra support is large enough to label those killed continuations. This is precisely the charged
tree count; it is also why a positive-defect counterexample is not ruled out by size alone.

The certificate does not need to rule out such configurations uniquely. It needs to show that the
killed continuations can be labeled with bounded local data and charged leaves.

### Case 3: Unbalanced Two-Child Node

If the row split is unbalanced, the split-defect lower bound is positive:

```text
ceil(max((K/2)/a, (K/2)/b)) - ceil(K/(a+b)) > 0.
```

A minimal counterexample can include such a node only by spending that many units of defect. The
charged-tree theorem should label the row split:

```text
node id, a, b, selected skeleton side, charged leaf marker.
```

Then it recurses into the selected lower-defect side. The unselected side contributes only charged
or residual support.

The local proof obligation is to show that selecting the lower-defect side cannot increase the
number of required labels beyond the row-split charge plus residual output leaves.

## Hall Conclusion

The certificate-facing Hall conclusion remains:

```text
some matched core survives
```

The counted charged-tree theorem relaxes uniqueness and charge bookkeeping, but it does not remove
this requirement. If no actual core survives, the current binomial count:

```text
k * binom(k-k/m, e)
```

would be invalid, because the support would have holes inside the proposed core.

There is a possible fallback with virtual cores:

```text
choose a core, choose h holes inside it, choose e+h extras outside it.
```

This was evaluated as a stress model with:

```text
charge overhead B=64,
virtual core holes <= H.
```

Artifacts:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes1.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes4.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes16.csv
```

Results:

```text
H=1:  total log2 union = -83.06077305
H=4:  total log2 union = -42.01824475
H=16: total log2 union =  79.12109543
```

So a bounded-hole fallback exists, but unbounded holes are too expensive under this crude count.

So the next proof target is still the actual Hall survival statement:

```text
local defect charges cannot kill every exact continuation unless the defect budget is exceeded.
```

The counted theorem then labels the leaves outside the surviving core.
