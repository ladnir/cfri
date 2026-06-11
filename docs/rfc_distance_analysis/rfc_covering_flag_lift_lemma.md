# RFC Covering Flag-Lift Lemma Target

Scope: original non-systematic RFC, depth-5 base-seal route.

Status: theorem target motivated by the `--cover-lift-mode all` diagnostic.

## Motivation

The current two-layer flag checkpoint counts parent lift multiplicity with Gaussian factors. This
is safe for counting all parent subspaces, but it can be far too pessimistic for an existence
certificate.

For distance we need:

```text
there exists a bad parent line.
```

We do not need to count every bad line inside one higher-dimensional child container separately if
the same child container certifies all of them.

The diagnostic evidence is:

```text
flag checkpoint, no lift cover:       crossing_z = 137
cover tau0 lifts:                     crossing_z = 135
cover tau1 lifts:                     crossing_z = 137
cover tau2 lifts:                     crossing_z = 129
cover tau0+tau1 lifts:                crossing_z = 133
cover tau0+tau2 lifts:                crossing_z = 61
cover tau1+tau2 lifts:                crossing_z = 129
cover all lifts:                      crossing_z = 35
cover all, z=34 vector log2:           27.64399707
cover all, z=34 projective heuristic: -100.35600293
```

Thus the base seal is probably controlled by a covering/projectivization issue, not by a missing
local root equation.

## One-Layer Covering Map

Fix a parent zero witness split:

```text
P = paired child coordinates,
S = singleton child coordinates.
```

For a bad parent subspace:

```text
W <= H_h,
dim W = t,
```

define:

```text
R   = visible image of W on S,
A   = supp(R),
tau = dim R,
K   = ker(W -> R),
V   = pi(W),
L   = pi(K).
```

The deterministic zero propagation is:

```text
V vanishes on P union (S \ A),
L vanishes on P union S.
```

For `tau > 0`, also record the exact local quotient/root datum:

```text
Q       = W/K,
R       = image of Q on A,
ell_A   = projective root-line assignment on A,
local layer label h if tau=2.
```

The proposed canonical certificate of `W` is:

```text
Cert(W) = (P,S,A,tau,L<=V,R,ell_A,local layer labels),
```

with `L` omitted if `K=0`. Ties must be broken canonically when a line has extra zeros or a
smaller visible support than the chosen witness.

## Fiber Principle

For fixed child flag and fixed local quotient/root datum:

```text
L <= V,
R,
ell_A,
```

every parent subspace `W` in the fiber that maps to this datum satisfies the same requested zero
witness:

```text
paired zeros:
  because V is zero on P;

singleton zeros outside A:
  because V is zero on S \ A;

singleton zeros on A:
  because R is root-compatible with ell_A;

kernel directions:
  because L is zero on all of S.
```

Therefore an existence certificate should count this fiber once at the container level, not with
the Gaussian number of possible parent lifts:

```text
[2 dim V choose t]_q
```

or its two-layer analogue:

```text
q^{kappa(2 dim L-kappa) + tau(2 dim V-t)}.
```

The local quotient/root datum is still counted. The claim is only that the ambient lift
multiplicity inside a fixed child container is not an event multiplicity.

## Candidate Lemma

Let `Phi` be an atomic transition profile containing:

```text
P,S,A,tau,
child flag L <= V,
exact local root-line/quotient layer labels,
canonical tie-breaking labels.
```

Let `Child(Phi)` be the set of child flags with:

```text
dim V = r1,  V zero count >= p+s-a,
dim L = r0,  L zero count >= p+s.
```

Then the parent bad-line event contribution for profile `Phi` is bounded by:

```text
Contribution_h(Phi)
  <= Split(Phi)
     * LocalContainer(Phi)
     * F_{h-1}(ChildFlag(Phi)),
```

where `LocalContainer(Phi)` includes:

```text
root probabilities,
exact-support root-line counts,
visible quotient incidence counts,
finite projective/frame constants,
```

but does not include the full Gaussian parent-lift multiplicity.

For a final projective distance event, the top line is counted projectively. If an auxiliary script
prints nonzero-vector moments, subtract one factor of `q` before comparing to `2^-lambda`.

## Proof Obligations

1. Canonical covering: every bad parent line with a fixed exact zero witness maps to at least one
   canonical profile `Phi`.
2. No undercounting from fibers: for fixed `Phi`, counting one child container plus local quotient
   datum covers all bad parent lifts in that fiber.
3. Controlled overlap: if one bad line maps to many profiles because of extra zeros or multiple
   root supports, the tie-breaking/profile count is polynomial in `N,d` and fits the finite
   constants bucket.
4. Shared child randomness: the child event is one joint flag event, never a product of separate
   child moments.
5. Local quotient counts: for `tau=1`, use support-subcode line counts; for `tau=2`, use the
   weighted exterior/root-line theorem plus the exact-support Grassmann cap and marked-line row.

## Current Risk

The lemma is plausible but not automatic. The dangerous case is a mixed local profile where only
some parent lifts inside a child container satisfy the singleton roots. In that case the proof must
show that `LocalContainer(Phi)` counts the satisfying quotient/root data sharply enough, instead of
silently covering a larger fiber.

This is why the next implementation should emit both:

```text
container count,
local quotient/root datum count,
```

not just zero out lift factors globally.
