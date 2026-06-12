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

Thus the base seal is sensitive to covering/projectivization, but the `all` cover run is
anti-conservative. The valid proof target is to replace crude independent lift/local products by
quotient-incidence counts, not to erase quotient multiplicity.

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
Cert(W) = (P,S,A,tau,L<=V,full quotient datum R,ell_A,local layer labels),
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

The local quotient/root datum is still counted, including invisible-fiber dimensions outside the
visible singleton support. The claim is only that after this quotient datum is fixed, duplicate
extensions inside the same child container are not separate container events.

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
invisible-fiber dimensions for quotient data,
finite projective/frame constants,
```

but does not include duplicate Gaussian extension multiplicity after the quotient datum has been
fixed.

For a final projective distance event, the top line is counted projectively. If an auxiliary script
prints nonzero-vector moments, subtract one factor of `q` before comparing to `2^-lambda`.

## Safe Subcases And Correction

### Tau Zero

If:

```text
tau = 0,
A = empty,
K = W,
```

then the singleton block is completely invisible. The only child datum is:

```text
V = pi(W),
V zero on P union S.
```

Every parent subspace `W` inside `V+V` has the requested paired and singleton zeros, because both
child projections are zero on every requested child coordinate. Therefore for an existence bound:

```text
count V once,
do not multiply by # { W <= V+V }.
```

The only finite data left are the split choices and the child container event `F_{h-1}((dim V,
p+s))`. This proves tau-zero lift covering as a direct container argument.

### Tau One: Correction

The previous tempting statement was: fix the child flag and visible support, then count the parent
fiber once. That is too optimistic unless the full quotient line datum is also counted.

Assume:

```text
tau = 1,
K = ker(W -> R),
dim R = 1.
```

Fix the child flag:

```text
L = pi(K) <= V = pi(W),
```

and fix the full visible quotient datum:

```text
R <= (V+V)/(L+L)
```

with exact support `A` on the singleton block and root-line assignment `ell_A`.

The child zero budgets are:

```text
V zero on P union (S \ A),
L zero on P union S.
```

For every parent lift in this fiber:

```text
kernel directions vanish on all S through L,
visible quotient directions vanish on S \ A through V,
visible quotient directions vanish on A by the fixed root line ell_A.
```

Thus all parent lifts in the fiber are bad for the same witness. However, the quotient line `R`
is itself an event variable. For a fixed child container there can be many possible quotient lines,
and the probability that at least one is root-compatible grows with the quotient ambient
dimension. Therefore a proof may count the container tuple:

```text
(L <= V, R, ell_A)
```

once, but it may not omit the count of possible `R`.

The tau-one local count is therefore:

```text
root factor q^-a
times the number of exact-support quotient lines R in the represented quotient.
```

This is exactly the support-subcode line count from `delta(A)`, with finite projective constants,
plus any invisible-fiber dimension from quotient coordinates outside the singleton support. In
other words, tau-one covering removes duplicate extensions of a fixed quotient line, but it does
not remove quotient-line incidence.

### Tau Two

The same covering philosophy should apply to tau two, but this is the live proof obligation rather
than a closed subcase. The local datum must include:

```text
R <= (V+V)/(L+L),
dim R = 2,
exact support A,
root-line layer X_h(A),
```

and for the decomposable `a=2,delta=2,comp=2` row it must include the joint marked-line/frame child
state. The tau-two branch is where a proof can accidentally reintroduce either:

```text
1. a product of child moments over shared randomness, or
2. a Gaussian quotient-lift factor that the container map was supposed to remove.
```

Closing tau two means proving the weighted exterior/root-line count, including any invisible-fiber
dimension, is the right count of quotient data `R`. It should replace the independent product:

```text
Gaussian quotient lift * local charge computed in the wrong ambient,
```

but it does not justify setting the quotient-lift exponent to zero.

## Anti-Conservative Shortcut Retired

The diagnostic:

```text
--cover-lift-mode all
```

sets all lift exponents to zero. This is useful only as a sensitivity test. It is not a candidate
theorem.

Counterexample shape: take a fixed child container `V` and a singleton block `A` with `a` roots.
If the quotient ambient has dimension `m`, the event that some quotient line is compatible has
rough exponent:

```text
q^(m-1-a)
```

in the rare-event range. Counting the container once with only a root factor `q^-a` misses the
projective quotient-line family `q^(m-1)`.

For tau two the same issue is the family of quotient planes. The valid replacement is the
incidence/fiber expression:

```text
fiber_qdim(Q,A) + theta_2(Q|_A,A),
```

or the coarse exact-support Grassmann cap:

```text
2(m-2) - |A|.
```

Those terms are quotient data counts. They cannot be dropped.

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

## Level-3 Flag Stress Row

The marked-plane recurrence diagnostics now isolate a concrete stress row:

```text
level 3 flag: (4,7)>=(2,8)
baseline truncated pair sum: 2094.40570138 bits
coarse/table baseline:       1187.19455102 bits
naive loss:                   907.21115036 bits = 7.08758711 q-dim
```

The dominant bad outer witness is:

```text
p=3, s=1, a=1, tau=1,
child=(4,4), z=3,
charge=1, lift=19.
```

This `lift=19` splits as:

```text
kernel_lift   = kappa(2r0-kappa) = 3(8-3) = 15,
quotient_lift = tau(2r1-t)       = 1(8-4) = 4.
```

A diagnostic collapsed-active filter removes the exact-flag overcount where equal-dimensional child
containers force the active singleton support to vanish. That cuts the displayed loss to
`4.08510402` q-dimensions but does not close the row; the next bad row still has a tau-one
kernel-lift factor.

The decisive diagnostic is the fixed-table kernel-cover what-if:

```text
python -B scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py \
  --level 3 \
  --outer-state 4,7 \
  --inner-state 2,8 \
  --term-limit 300 \
  --posthoc-cover-kernel-lift
```

It gives:

```text
pair sum:              940.85227227 bits
coarse/table baseline: 1187.19455102 bits
saving:                246.34227875 bits = 1.92454905 q-dim
```

This is strong evidence for the intended division of labor:

```text
keep quotient incidence;
cover duplicate kernel lifts.
```

Concretely, for fixed child flag `L<=V` and fixed local quotient/root datum `R`, the recurrence
should count the container tuple once and not multiply by the Gaussian number of possible
`K <= L+L` kernel lifts. The proof still has to show that this container tuple is a valid
first-moment event at intermediate flag states, not only at the final projective-line event.
