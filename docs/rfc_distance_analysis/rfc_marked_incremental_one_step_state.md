# RFC Marked Incremental One-Step State

Status: state refinement for the incremental flat-rank route.

## Starting Point

The current small-flat target is the marked incremental rank event:

```text
I_d(p,a,r)
  = E[# {(P,A) :
          P,A disjoint,
          |P|=p,
          |A|=a,
          rank(P union A)-rank(P) <= r }].
```

This is the correct object for a flat-excess witness because `A` is the marked flat witness and
`P` is the paired core. The aggregate short-set rank-tail `B_d(z,s)` is too coarse because it counts
dependencies wholly inside `P union A` that do not make `A` low-rank modulo `P`.

However, `I_d(p,a,r)` is probably not closed under one RFC fold. The top split remembers more than
the cardinalities `(p,a)`.

## Top Pair Categories

At a top fold, each child position has two parent siblings. Since `P` and `A` are disjoint, each
sibling is in one of:

```text
empty, P, A.
```

The unordered top pair categories are:

```text
00  empty pair
PP  both siblings in P
AA  both siblings in A
PA  one sibling in P, one sibling in A
P0  exactly one sibling in P
A0  exactly one sibling in A
```

The side labels matter for `P0`, `A0`, and `PA` only through the root-line slope domain and finite
constants. For generic-rank structure, each nonempty category has a clean local interpretation.

## Local Linear Algebra

At one child coordinate with child column `h`, the two parent sibling columns are:

```text
left  = (h, t h)
right = (h, (t+1) h)
```

The pair `(left,right)` is an invertible coordinate transform of the two coordinate axes:

```text
(h,0), (0,h).
```

Therefore:

```text
PP:
  P contains both axes for h.
  Both copies of h are contracted by span(P).

AA:
  A contains both axes for h.
  The marked increment can be as large as two child-copy ranks.

PA:
  P contains one root-line through h and A contains the complementary root-line.
  After a local invertible transform, this is equivalent to:
      P contains (h,0), A contains (0,h),
  but the transform depends on the root t and on which side is marked.

P0:
  P contains one root-line (h,t h) or (h,(t+1)h).
  This contracts a graph line, not a coordinate axis.

A0:
  A contains one root-line.
  Its rank modulo P is governed by a root-line quotient-rank condition.
```

The `PP` category is the old paired compression. The `A0` category is the old root-line singleton
repair, but with the roles changed: we are measuring the rank of marked `A` modulo a marked core
`P`. The new category is `PA`, where `P` already contains one sibling at the same child position.

## Why `(p,a,r)` Is Not Closed

For unmarked survivor rank, a top profile only needs:

```text
paired count p,
singleton count t,
child matroid rank on paired positions.
```

For marked incremental rank, two profiles with the same `(p,a)` can behave differently:

```text
Profile 1:
  A is all A0 singletons, disjoint from the child positions touched by P.

Profile 2:
  A is all PA mixed siblings, each sharing a child position with P.
```

Both have the same `|P|` and `|A|`, but Profile 2 gives `A` access to child coordinates already
visible in `P`. This can lower the effective ambient dimension. The event is still nontrivial:
the complementary root-line is not automatically in `span(P)`, but the proof must account for it.

Thus the next recurrence state should be at least:

```text
J_d(pp, p0, aa, a0, pa, r)
```

or, more invariantly, a contracted root-line matroid state describing:

```text
P_axis    = child positions where P contracts both copies,
P_line    = child positions where P contracts one root-line,
A_axis    = child positions where A asks for both copies,
A_line    = child positions where A asks for one root-line,
mixed     = child positions with one P sibling and one A sibling.
```

The scalar `I_d(p,a,r)` can be recovered by summing over these profiles, but it should not be used
as the induction state.

## One-Step Theorem Target

Fix the lower child generator and a marked top profile. Let:

```text
Q_P = span of all P columns in H + H.
```

Let `A_profile` be the set of marked A columns in the quotient:

```text
(H+H) / Q_P.
```

The local theorem needed is:

```text
Pr_top[ rank(A_profile modulo Q_P) <= r ]
  <= q^{-codim(profile,r)}
```

where `codim(profile,r)` should match the random quotient-rank exponent except for recursive child
rank-tail losses forced by lower marked dependencies.

The natural generic-rank quantity is:

```text
rank_generic(A modulo P)
  = rank over F(t) of the A columns in (H+H)/Q_P.
```

Finite-field loss is then:

```text
rank_generic - actual_rank >= rank_generic-r.
```

The proof should mirror the existing root-line generic rank lemma:

1. contract the `P` columns first;
2. compute the generic rank of the marked `A` columns by matroid union / transversal expansion;
3. bound finite-root rank drops by incidence strata;
4. when an incidence stratum says a subset of `A` has low quotient rank, recurse on a lower marked
   incremental event rather than on an unmarked `B_d(z,s)` event.

## Important Special Case: One Marked A Coordinate

The first flat-excess stress case is:

```text
p = 137,
a = 1,
r = 0.
```

The event is:

```text
the marked A column lies in span(P).
```

There are two top shapes:

```text
A0:
  A is alone in its sibling pair.

PA:
  A's sibling is in P.
```

The `A0` case should cost roughly:

```text
q^{-(2k_child-rank(P))}
```

in a random quotient model.

The `PA` case can be cheaper because one root-line through the same child column is already in
`P`. But it is not free: the complementary root-line is in `span(P)` only if the rest of the
contracted root-line matroid reconstructs the missing direction. A random-code scale suggests a
cost comparable to a child quotient event, roughly:

```text
q^{-(k_child-rank(child projection of P'))}
```

up to root-line incidence constants.

For the dominant profile, this is still enormous:

```text
k_child = 512,
rank(child projection of P') <= 136,
cost at least around q^-376 by the PA projection lemma.
```

The helper:

```text
scripts/rfc_distance_analysis/rfc_marked_one_a_profile_scale.py
```

reports this crude scale at `q=2^128`:

```text
A0:
  q_exponent  = 887
  log2 moment = -112523.355314

PA crude:
  q_exponent  = 376
  log2 moment = -47121.232771
```

The deterministic projection lemma is recorded in:

```text
rfc_pa_mixed_projection_lemma.md
```

So the mixed case may reduce the exponent from the naive `887` q-dimensions to about `376`, but it
still appears far from dangerous after entropy.

## Current Blocker

The flat-excess route now has a precise local blocker:

```text
prove the marked root-line quotient-rank theorem for J_d profiles,
especially the PA mixed category.
```

This is progress over the previous blocker because it is no longer an unstructured complaint about
"bad flats." The obstruction is now localized to a concrete one-step algebra problem.
