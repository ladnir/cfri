# RFC Rank-Active Cancellation Independence Target

This note translates the global rank-cancellation probe into a proof target.

## Linear-Algebra Setup

Fix:

```text
row support R,
final virtual support Y = (C \ H) union E,
```

and let:

```text
V(Y) = { x supported on R : supp(Ax) subset Y }.
```

For a zero output coordinate `z notin Y`, write `ell_z` for the output coordinate functional:

```text
ell_z(x) = (Ax)_z.
```

If `J` is a set of zero coordinates, then:

```text
V(Y) = V(Y union J) cap ker({ell_z : z in J}).
```

Thus the rank imposed by the zero coordinates `J` is:

```text
dim V(Y union J) - dim V(Y).
```

This is exactly what the probe calls `imposed_rank`.

## Active Excess Cancellations

A raw tree sibling absence is not automatically a cancellation equation. There are two reasons:

```text
1. the selected exact matched core has its own baseline cancellation pattern;
2. some absent siblings are automatic zeros for the fixed row/support geometry.
```

The proof should therefore use only rank-active excess cancellations:

```text
z is rank-active relative to Y
  iff z is a non-core, non-hole sibling absence and dim V(Y union {z}) > dim V(Y).
```

Equivalently, `ell_z` is not redundant among the zero constraints defining `V(Y)`.

## Target Lemma

Let `J_active` be the set of rank-active excess cancellation zeros for a fixed final virtual
support `Y`. The target is:

```text
dim V(Y union J_active) - dim V(Y)
  >= |J_active| - (dim V(Y)-1) - 1.
```

The two subtracted terms are:

```text
dim V(Y)-1: global projective admissible budget already paid by the first moment;
1:          no-early-gluing relative-scalar allowance.
```

Using the near-kernel dimension lemma:

```text
dim V(Y)-1 <= floor((e+h)/|C|),
```

this is the global-rank-budget form needed by the certificate.

## Proof Shape

The exact no-early-gluing proof proves the special case:

```text
dim V(Y)=1,
|J_active|>=2
  => two active cancellation equations are independent.
```

It does this by assigning each cancellation equation a fresh parent challenge ratio:

```text
lambda = L_j(T_j) * U_j/V_j.
```

For the near case, `V(Y)` has projective dimension `s=dim V(Y)-1`. A nontrivial dependence among
many active cancellation equations would mean that more than `s+1` fresh sibling-ratio equations
are absorbed by the same `s` projective degrees of freedom plus one relative scalar.

The desired rational-function proof should show that this cannot happen generically because each
rank-active excess cancellation exposes a fresh parent challenge variable that is not eliminated by
the final support constraints.

## What Must Be Proved Carefully

The phrase "rank-active" hides the hard part. A coordinate can be a raw sibling absence but still
inactive if its zero follows from higher-level one-child descent or from the chosen support
geometry. The proof needs a canonical way to discard exactly those inactive zeros.

The likely route is:

```text
1. Work in V(Y union J_active).
2. Choose a basis adapted to V(Y).
3. For each active cancellation z, choose a witness vector whose z-coordinate is nonzero.
4. Show the matrix of active cancellation functionals has generic rank at least
   |J_active| - dim_projective(V(Y)) - 1.
```

The fresh-challenge part should be proved by a leading-variable/leading-monomial argument: order
each active cancellation by its highest tree node where it is exposed, then use a fresh `T_j` at
that node as a pivot variable unless the equation is one of the globally absorbed projective
directions or the single scalar allowance.

## Evidence

The probe:

```text
scripts/rfc_distance_analysis/rfc_global_rank_cancellation_probe.py
```

currently supports this statement at the checked small depths, including randomized placements:

```text
depth=3, max_extra=2, max_holes=1: checked=1240 failures=0
depth=4, max_extra=2, max_holes=1: checked=8512 failures=0
depth=5, max_extra=1, max_holes=1: checked=6624 failures=0
depth=5, live_bits=4, random, max_extra=2, max_holes=1: checked=1860 failures=0
depth=6, live_bits=5, random, max_extra=2, max_holes=1: checked=1717 failures=0
```

This is evidence only. The proof still needs the rational-function independence argument.
