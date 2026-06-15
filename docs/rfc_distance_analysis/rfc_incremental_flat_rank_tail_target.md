# RFC Incremental Flat-Rank Tail Target

Status: replacement for the too-coarse aggregate short-set target.

## Problem With `B_d(z,s)`

The note `rfc_short_set_rank_tail_target.md` proposed the aggregate short-set rank-tail:

```text
B_d(z,s)
  = E[# {Z : |Z|=z and rank(G_d[Z]) <= min(k_d,z)-s}].
```

The first recurrence diagnostic:

```text
scripts/rfc_distance_analysis/rfc_short_set_rank_tail_recurrence.py
```

shows that this aggregate object is too coarse for small-flat charging. For the dominant
small-flat shape:

```text
d=10, k=1024, n=8192, z=138, s=1,
```

the random-matrix finite-root model still gives:

```text
log2 B_10(138,1) ~= 828.343072.
```

The dominant trace is not the intended flat event. It starts:

```text
depth 10: z=138,s=1
  child witness p=1,t=136,a=45 -> child z=46,s=23.
```

This means the union set `Z` has a large hidden internally bad subset. But a flat-excess witness at
the top has more structure:

```text
Z = P union A,
```

where `P` is the paired core and `A` is the marked singleton flat witness. We do not merely need
`P union A` to be rank-deficient. We need `A` to have small rank modulo `P`.

The aggregate `B_d(z,s)` forgot the marking and therefore counts many dependencies inside `P` that
do not prove the flat witness is bad.

## Correct Object

Define an incremental marked rank-tail:

```text
I_d(p,a,r)
  = E[# {(P,A) :
          P,A disjoint subsets of [N_d],
          |P|=p,
          |A|=a,
          rank(P union A) - rank(P) <= r }].
```

Equivalently, after quotienting by `span(P)`, the marked set `A` has quotient rank at most `r`.

For the flat-excess witness:

```text
a = |A| = 2r + F,
quotient rank <= r.
```

The top dominant profile has:

```text
p = |P| = 137,
D = k_child - rank(P) ~= 887.
```

For a random code, conditioned on `P` having rank `p`, the quotient columns of `A` live in an
ambient space of dimension:

```text
D = k-p.
```

The probability that `a` random quotient columns have rank at most `r` has q-exponent:

```text
(a-r)(D-r).
```

Substituting `a=2r+F` gives:

```text
(r+F)(D-r),
```

which is exactly the small-flat exponent from `rfc_small_flat_subcode_charge.md`.

Examples for the dominant profile:

```text
(r,F)=(0,1): exponent = 887
(r,F)=(1,1): exponent = 1772
(r,F)=(0,8): exponent = 7096
(r,F)=(8,1): exponent = 7911
```

The helper:

```text
scripts/rfc_distance_analysis/rfc_incremental_flat_rank_scale.py
```

also includes the marked pair entropy. At `q=2^128`:

```text
(r,F)=(0,1): log2 first moment ~= -112523.330980
(r,F)=(1,1): log2 first moment ~= -225779.965142
(r,F)=(0,8): log2 first moment ~= -907199.805522
(r,F)=(8,1): log2 first moment ~= -1011436.082256
```

This is the sharp contrast with the failed aggregate recurrence:

```text
B_10(138,1) random-matrix diagnostic ~= +828.343072 bits.
```

The aggregate object is swamped by unmarked internal dependencies. The marked incremental object
has the intended enormous slack.

## Why This Is Better Than `B_d(z,s)`

For `B_d(138,1)`, a dependence entirely inside the 137-core `P` is counted as a bad 138-set. But
for `I_d(137,1,0)`, a dependence inside `P` is not enough. The marked extra coordinate must lie in
the span of `P`.

If `P` itself has rank deficit, that is not free:

```text
rank(P) = p-u
```

only makes the quotient ambient larger:

```text
D = k-rank(P) = k-p+u.
```

The event that `A` has small quotient rank is then at least as expensive, and the rank defect of
`P` can be charged separately. Thus the incremental object avoids the false dominance from
unmarked internal bad subsets.

## Recurrence Shape Needed

The next theorem target should recurse on marked pairs `(P,A)`, not unmarked unions.

At one fold, the coordinates in `P` and `A` split into marked local categories:

```text
P-pair:   both siblings in P
A-pair:   both siblings in A
mixed:    one sibling in P, one sibling in A
P-single: exactly one sibling in P
A-single: exactly one sibling in A
empty
```

Paired categories compress exactly to lower marked coordinates. Singleton categories create
root-line quotient-rank equations. The target is a recurrence for:

```text
I_d(p,a,r)
```

or a refined version that also tracks the rank defect of `P`.

The recurrence should have:

1. exact paired compression for `P-pair`, `A-pair`, and `mixed` coordinates;
2. a finite-root quotient-rank tail for marked `A` singletons modulo the span of marked `P`;
3. recursive charging when a lower marked subset of `A` already has small quotient rank;
4. a separate child-rank-tail charge for rank defects inside `P`.

## Current Meaning For The Proof

This is a notable change in understanding:

```text
short-set rank tail was too broad;
incremental marked quotient-rank tail is the right small-flat object.
```

The good news is that the random-code exponent for the incremental object is exactly the exponent
we need. The bad news is that the recurrence state is richer: it must preserve the distinction
between the large paired core `P` and the marked flat witness `A`.

This is still a plausible route, but the next blocker is now precise:

```text
prove or falsify an RFC incremental quotient-rank tail for marked pairs (P,A).
```

If this theorem holds with the random-code exponent up to small shape-recursive losses, the
small-flat part of the surplus proof should close. If it fails, the flat-excess route has a genuine
structural obstruction.
