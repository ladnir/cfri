# RFC Targeted Flag Generator Proof Contract

Scope: original non-systematic RFC only. This contract is for the targeted depth-4 generator that
enumerates parent two-dimensional `W` spaces in the known complete-stride `m=4, extra=4` class. It
does not define a production certificate by itself; it defines what the depth-4 evidence must count
to be admissible for or against the two-layer flag recurrence.

## Purpose

The generic subflag enumerator is too broad at depth 4 because the ambient zero kernels contain too
many Gaussian subflags. The targeted generator should instead enumerate parent spaces:

```text
W <= H_h,
dim W = t = 2,
```

whose singleton visible quotient has:

```text
tau = 1,
kappa = dim ker(W -> R) = 1.
```

The child flag induced by such a parent space is:

```text
L = pi(K) <= V = pi(W),
r0 = dim L,
r1 = dim V.
```

The admissible evidence question is:

```text
Do complete-stride parent W spaces produce more distinct child flags or multi-copy intersections
than the upper-bound Lift(t,tau,r0,r1) factor and generic flag-intersection exponent already allow?
```

## Event Versus Certificate

The enumerator must count linear events, not labels explaining those events.

### Events To Count

Count each of the following once after canonicalization:

```text
1. exact visible support A subset S;
2. parent projective subspace W <= H_h;
3. kernel K = ker(W -> R);
4. induced child flag L <= V;
5. multi-copy intersection of canonical flags.
```

Canonical keys should be based on RREF bases over the chosen field:

```text
key(W), key(K), key(V), key(L), key(L<=V).
```

If two generation paths produce the same canonical `W`, or the same canonical child flag `L<=V`,
they are duplicate certificates, not two events.

### Duplicate Certificates

Record these as diagnostics only; do not multiply the event count by them:

```text
marked core coordinate,
matched block/stride witness,
complete extra stride class witness,
choice of a basis for W,
choice of a basis for K,
choice of a root-line representative,
choice of a larger contained support A' superset A.
```

For exact support counting, a line or flag with visible support `A` belongs to the exact `A` bucket,
not to every larger support bucket.

### Real Multiplicities

These are real union-bound multiplicities and must be counted:

```text
choice of exact paired set P,
choice of singleton set S,
choice of exact visible support A subset S,
singleton orientation,
choice of independent RFC copy subset in active-copy tests,
distinct canonical W spaces,
distinct canonical child flags L<=V.
```

## Relation To The Lift Factor

The upper-bound recurrence contains the safe lift factor:

```text
Lift(t,tau,r0,r1)
  <= Gamma_q^2 q^{kappa(2r0-kappa) + tau(2r1-t)},
kappa = t - tau.
```

For the targeted depth-4 case:

```text
t = 2,
tau = 1,
kappa = 1,
r0 = 1,
r1 = 2
```

the q-exponent is:

```text
1*(2*1-1) + 1*(2*2-2) = 3.
```

So, for each fixed child flag support/zero-budget profile, the parent `W` enumeration is admissible
only if the number of distinct canonical parent spaces lifting the same child flag profile is
bounded by the Gaussian lift allowance:

```text
<= Gamma_q^2 q^3
```

up to explicitly tracked split/profile constants.

The targeted generator is not required to prove this asymptotic bound. It must provide enough
exact data to decide whether complete-stride structure appears to add an extra reusable q-dimension
beyond this allowance.

## Minimal Output Columns

Each row should correspond to one grouped event profile. The minimal columns are:

```text
field_prime
copy_id
level_h
t
z
p
s
a
tau
kappa
r1
r0
z_V
z_L
delta
comp
g
exact_support_key
outer_zero_key
inner_zero_key
canonical_flag_count
canonical_parent_W_count
duplicate_certificate_count
max_certificates_per_flag
complete_extra_stride_count
observed_parent_lift_logq
lift_bound_logq
parent_lift_excess_logq
common_v_dim
common_l_dim
generic_v_dim
generic_l_dim
v_excess
l_excess
status
```

Definitions:

```text
canonical_flag_count:
  number of distinct canonical child flags L<=V in the group.

canonical_parent_W_count:
  number of distinct canonical parent W spaces producing those flags.

duplicate_certificate_count:
  number of discarded generation-path labels after canonicalization.

observed_parent_lift_logq:
  log_q(canonical_parent_W_count) after fixing the stated support/zero-budget profile.

lift_bound_logq:
  kappa(2r0-kappa) + tau(2r1-t), plus an annotation that Gamma_q^2 is a bit constant.

parent_lift_excess_logq:
  observed_parent_lift_logq - lift_bound_logq.

v_excess, l_excess:
  observed multi-copy common-dimension excess over generic intersection prediction.
```

If multi-copy intersections are not run for a row, set the common/generic/excess columns to `NA`;
do not omit them.

## Acceptance Criteria

A depth-4 targeted result is admissible supporting evidence for the two-layer flag recurrence if:

```text
1. exact supports are canonicalized and counted once;
2. marked-core/stride labels are reported only as duplicate_certificate_count;
3. parent W spaces are canonicalized before grouping;
4. child flags L<=V are canonicalized before grouping;
5. every emitted row reports lift_bound_logq and parent_lift_excess_logq;
6. impossible states are rejected, especially tau=2 with delta<2;
7. multi-copy rows report both observed and generic flag-intersection dimensions;
8. skipped or capped groups are reported with status=skipped, not silently absent.
```

The result is evidence in favor of the recurrence only if all completed targeted groups satisfy:

```text
parent_lift_excess_logq <= 0
v_excess <= 0
l_excess <= 0
```

up to finite-field rounding and explicitly logged constants.

## Rejection Criteria And Extra q-Dimension

An observed extra q-dimension is a serious counter-signal. Flag it as `status=excess_q_dimension`
if any completed group has:

```text
parent_lift_excess_logq >= 1 - epsilon
```

or, for multi-copy intersections:

```text
max(v_excess, l_excess) >= 1.
```

Use a small `epsilon` only for finite-field log rounding over small primes; the row must report the
raw counts so the manager can recompute.

Interpretation:

```text
extra lift dimension:
  complete-stride parent W spaces may have more multiplicity than Lift allows.

extra intersection dimension:
  independent-copy flags may intersect above the generic flag exponent.
```

Either signal does not immediately disprove the production `e=71` target. It does mean the current
two-layer recurrence is missing state or charge. The next proof action would be to identify whether
the excess is a duplicate certificate, a paired-compression structure, or a real reusable
nested-kernel family.

## Targeted Parent-W Enumeration Contract

The generator should start from the known depth-4 complete-stride class:

```text
h = 4,
m = 4,
extra outputs = 4,
t = 2,
tau = 1,
kappa = 1.
```

For each exact support profile:

```text
1. build candidate parent W spaces directly from the complete-stride construction;
2. compute R, A, K, V=pi(W), L=pi(K);
3. verify the zero budgets z_V=p+s-a and z_L=p+s by direct rank/kernel checks;
4. canonicalize W, K, V, and L;
5. group by exact event keys before counting;
6. separately report how many marked construction paths led to each event.
```

The enumeration should not enumerate all subflags inside the ambient zero kernels unless a separate
size guard says that is tiny. The contract is to count the flags actually generated by the targeted
parent `W` family and compare them to the generic lift allowance.

## Manager Decision Rule

If the targeted generator finds no extra q-dimension in the complete-stride depth-4 target, the
two-layer flag recurrence becomes credible enough to push into the certificate theorem, while still
leaving the tau-two endpoint theorem as an open local obligation.

If it finds a completed row with a full extra q-dimension, do not paper it over with constants:

```text
1. classify whether the excess is duplicate certificates or distinct canonical events;
2. if duplicate, fix the counting model;
3. if distinct but paired-compressive, add that state/charge to the recurrence;
4. if distinct and reusable, treat e=71 as fragile and evaluate e=72.
```

## Feedback For Falsification Agent

The sharpest break target is:

```text
A complete-stride depth-4 parent-W family with t=2,tau=1,kappa=1,r0=1,r1=2
that produces distinct canonical child flags or independent-copy intersections with one full
extra q-dimension beyond the Gaussian Lift/generic-intersection bound after exact support
canonicalization.
```

Please report the smallest explicit row that does this, including the canonical counts and the
duplicate certificate count. A row that only has many marked core or stride witnesses for the same
canonical flag does not break the recurrence.
