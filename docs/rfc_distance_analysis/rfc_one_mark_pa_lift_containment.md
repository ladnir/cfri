# One-Mark PA Lift Containment

Status: Tier-2 containment lemma for the scalar closure recurrence.

## Purpose

The one-mark closure recurrence has two branches:

```text
A0: marked coordinate's sibling is not in P;
PA: marked coordinate's sibling is in P.
```

The direct `A0` branch is handled in:

```text
rfc_one_mark_closure_defect_crawl.md
```

This note proves the deterministic containment and lift count for the `PA` branch. The goal is to
keep Tier 2 scalar: recurse to total child one-mark closure `C_{d-1}(y,1)` rather than introducing
an exact parent defect state or a full projection-matroid state.

## Setup

At one parent fold, each child coordinate `j` has two parent columns:

```text
L_j = (h_j, T_j h_j)
R_j = (h_j, (T_j+1) h_j).
```

Let `(P,a)` be a parent one-mark closure witness:

```text
|P| = p,
a notin P,
a in cl_parent(P).
```

Assume the `PA` case: the sibling of `a` over child coordinate `j` lies in `P`.

Write:

```text
b  = sibling of a over j, so b in P
P' = P \ {b}
Q  = child projection support of P', excluding j.
```

Here `Q` contains a child coordinate `q` if at least one parent sibling over `q` appears in `P'`.

## Containment Lemma

If `(P,a)` is a `PA` one-mark closure witness, then:

```text
h_j in span_child(Q).
```

Equivalently:

```text
j in cl_child(Q).
```

Thus every parent `PA` witness maps to a child one-mark closure witness:

```text
(Q,j),      |Q| = y <= p-1.
```

## Proof

This is exactly the PA mixed projection lemma.

Let:

```text
U = span_parent(P') <= H plus H.
```

If `a in span(P)` and `b in P`, then:

```text
a in span(U,b).
```

The two sibling columns over `j` differ by a nonzero vector of the form:

```text
(alpha h_j, beta h_j) != (0,0).
```

Therefore `U` contains a nonzero vector in the two-dimensional copy of the child column `h_j`.
Projecting to a nonzero component gives:

```text
h_j in span_child(Q).
```

So `(Q,j)` is a child one-mark closure witness.

## Lift Count

Fix a child closure witness:

```text
(Q,j),    |Q|=y.
```

We upper-bound the number of parent `PA` witnesses `(P,a)` that can map to it.

1. Choose which sibling over `j` is marked:

```text
2 choices.
```

The other sibling is forced into `P`.

2. For each child coordinate `q in Q`, choose one parent sibling over `q` to witness that `q` is in
the projection support:

```text
2^y choices.
```

3. We have already chosen:

```text
1 + y
```

coordinates of `P`: the sibling over `j` plus one sibling over each `q in Q`.

Choose the remaining:

```text
p-1-y
```

coordinates of `P` from the pool of parent coordinates not over `j` and not already selected:

```text
2(n_child-1)-y.
```

This gives the lift bound:

```text
Lift(d,p,y)
  = 2^{1+y} binom(2(n_child-1)-y, p-1-y).
```

The count intentionally overcounts. The remaining coordinates may add more child projection
positions beyond `Q`, and the same parent witness may be counted through several smaller child
closure witnesses. Both are safe for an upper bound.

## Recurrence Consequence

Let:

```text
C_d(p,1)
```

be the total expected number of parent one-mark closure witnesses with core size `p`. Then the PA
branch satisfies:

```text
PA_d(p,1)
  <= sum_{y=0}^{min(p-1,n_child-1)}
       C_{d-1}(y,1)
       2^{1+y}
       binom(2(n_child-1)-y, p-1-y).
```

This is the PA term used by:

```text
scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_recurrence.py
scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_defect_scale.py
```

## Why This Avoids A Larger State

The containment forgets parent rank defect and exact projection matroid type. That is deliberate.
It maps every PA witness to a child closure witness and pays a loose lift factor for all possible
parent reconstructions.

So the scalar recurrence is proof-safe at the cost of entropy slack:

```text
parent PA closure -> child total one-mark closure x loose lift.
```

For the target, this loose PA chain is still harmless after residual repair:

```text
log2 C_10(137,1) - 71 log2(q) ~= -7698 bits.
```

## Remaining Tier-2 Obligations

The PA lift containment is clean. The remaining one-mark closure theorem needs:

```text
1. an RFC-safe direct A0 rank-tail bound;
2. finite constants for the lift sum;
3. integration with residual root-repair factor in the flat-excess endpoint.
```

No arbitrary projection-matroid state is needed for this PA containment.
