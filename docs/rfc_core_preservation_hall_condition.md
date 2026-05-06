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
