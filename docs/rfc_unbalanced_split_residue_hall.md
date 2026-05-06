# RFC Unbalanced Split Residue-Hall Target

This note isolates the remaining core-preservation gap after the one-child and balanced two-child
cases.

## Setup

At a node of output length `K`, let the live row weight split as:

```text
M = a + b,     a,b > 0,     a != b.
```

The parent target core size is:

```text
L = K/M.
```

Let the child output supports be:

```text
U, V subset {0, ..., K/2-1}.
```

In a balanced branch, the child live weights remain powers of two and the induction exposes child
stride cores. In an unbalanced branch, `a` and `b` need not be powers of two, so we should not assume
the exact matched-core theorem applies verbatim inside each child. The optimistic residue model is:

```text
C_a = { r_a mod a } in U,     |C_a| = K/(2a),
C_b = { r_b mod b } in V,     |C_b| = K/(2b),
```

with possible extra child coordinates outside those cores. When `a` or `b` is not compatible with
an exact child core, the row-split/rounding defect must pay for the deviation before this model can
be used.

The parent transform combines the two child values at the same child coordinate `j` into two
siblings. Therefore a parent stride residue `rho mod M` survives only if every child coordinate
appearing under that residue is present in `U union V`, and the selected sibling is not cancelled.
The cancellation part is already controlled by the no-early-gluing lemma. What remains here is the
deterministic child-coordinate coverage problem.

## Parent Residue Projections

For a parent residue `rho`, define the child-coordinate projections:

```text
P_0(rho) = { j :       j  = rho + t M         lies in the left sibling block },
P_1(rho) = { j : K/2 + j  = rho + t M         lies in the right sibling block }.
```

Equivalently, each `P_s(rho)` is an arithmetic progression in child coordinates with step `M`,
restricted to `0 <= j < K/2`.

In the balanced case `a=b=M/2`, each child core modulo `M/2` splits into two sub-residues modulo
`M`, and one of the two parent residue candidates survives. This is why balanced core preservation
is now reduced.

Even under this optimistic model, the unbalanced case has a real obstruction: a child residue modulo
`a` or `b` generally does not contain a full parent projection:

```text
M mod a = b mod a,
M mod b = a mod b.
```

So the exact balanced lift argument cannot be copied. This is the actual obstruction.

## Hall Form

For a candidate parent residue `rho`, define its deterministic hole count:

```text
h_det(rho)
  = |{ (s,j) : j in P_s(rho), j notin U union V }|.
```

This counts missing parent residue positions, not just distinct child coordinates. If both parent
siblings over the same child coordinate are used by the residue, a missing child coordinate creates
two holes. Sibling cancellations can add at most the already-charged cancellation holes. The unbalanced
residue-Hall target is:

```text
min_rho h_det(rho) = 0
```

or, for the virtual-core fallback,

```text
sum over unbalanced nodes of selected h_det(rho) <= H,
```

with `H <= 5` at the current depth-11, `c=8`, `B=64` certificate point.

The second statement is much weaker than exact core preservation, but still strong enough for the
current first-moment bound. The H=8 stress run already fails, so the hole bound must be genuinely
small.

## Charge Interface

The rounded row-split charge is:

```text
Delta_split(a,b;K)
  = ceil(max((K/2)/a, (K/2)/b)) - ceil(K/(a+b)).
```

This charge measures how much larger the child uncertainty lower bound is than the parent target.
It gives extra supported child coordinates beyond the ideal parent core size. The residue-Hall
proof must use those extra child coordinates to cover the projected parent residue positions.

A certificate-sufficient local statement would be:

```text
Either some parent residue has h_det(rho)=0,
or the uncovered positions for a minimizing rho are paid by row-split charge and coalesce globally
into at most five virtual core holes.
```

The first clause preserves the existing counted theorem with no holes. The second clause uses the
experimentally safe virtual-hole fallback.

## Why This Is The Right Remaining Object

The field algebra is already doing two jobs:

```text
1. one-copy uncertainty gives the child support lower bounds;
2. no-early-gluing prevents many sibling cancellations over a fixed child core.
```

Unbalanced splitting fails before either of those tools can finish the lift, because the child
stride moduli `a,b` are not the parent stride modulus `M`. The missing step is therefore a finite
residue covering/Hall lemma, not a new random-field identity.

This also explains why the proof cannot simply copy the exact RFC paper. The exact proof only uses
balanced two-child gluing at the full-live node; an unbalanced near-extremizer can be close in
weight while exposing child cores with incompatible moduli.
