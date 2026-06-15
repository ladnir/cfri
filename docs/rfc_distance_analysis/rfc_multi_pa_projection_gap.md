# RFC Multi-PA Projection Gap

Status: falsifies the naive multi-coordinate extension of the PA projection lemma.

## What We Hoped

The one-coordinate PA lemma says:

```text
if a complementary mixed sibling lies in span(P),
then its child column lies in the projection span of the other P columns.
```

A tempting extension was:

```text
rank(A_mixed modulo P)
  >= rank(child mixed columns modulo projection(P_other)).
```

If true, all mixed `PA` flat witnesses could be routed directly to child incremental rank tails.

## Result

The extension is false.

The diagnostic:

```text
scripts/rfc_distance_analysis/rfc_multi_pa_projection_gap_search.py
```

samples random root-line configurations and compares:

```text
child_rank  = rank increment of mixed child columns modulo projections of P_other,
marked_rank = rank increment of complementary mixed A siblings modulo P_other plus mixed P siblings.
```

It finds `marked_rank < child_rank` in small fields:

```text
GF(5), k=4, n=12, p_other=3, mixed=2, trials=20000:
  failures = 89
  first gap: child_rank=1, marked_rank=0

GF(5), k=4, n=12, p_other=3, mixed=3, trials=20000:
  failures = 230
  first gap: child_rank=1, marked_rank=0

GF(7), k=4, n=12, p_other=3, mixed=2, trials=20000:
  failures = 44
  first gap: child_rank=1, marked_rank=0
```

So the one-coordinate PA lemma is real, but it does not tensor into a simple rank comparison.

## Why The Naive Extension Fails

Let `U` be the span of the non-mixed `P` columns. For each mixed coordinate `j`, write the `P`
sibling as:

```text
p_j = (h_j, alpha_j h_j)
```

and the complementary `A` sibling as:

```text
a_j = p_j + (0, epsilon_j h_j)
```

where `epsilon_j = +/-1`.

Modulo the mixed `P` siblings, the marked columns are represented by:

```text
(0, epsilon_j h_j).
```

But relations among several marked columns are allowed to use arbitrary combinations of the mixed
`P` siblings:

```text
sum_j c_j (0, epsilon_j h_j)
  in U + span_j (h_j, alpha_j h_j).
```

Equivalently, there can exist coefficients `mu_j` and a vector `(u_1,u_2) in U` such that:

```text
sum_j mu_j h_j       = -u_1,
sum_j (c_j epsilon_j - mu_j alpha_j) h_j = u_2.
```

The first equation may hold for a nonzero combination of mixed child columns modulo `pi_1(U)`.
The second equation then uses the root-line slopes `alpha_j` to create a relation among marked
`A` siblings even when the child mixed columns are not all in the projection span of `U`.

For one mixed coordinate, this mechanism collapses: a nonzero scalar multiple of `h_j` in one
projection already forces `h_j` into the projection span. For several coordinates, combinations can
hide the dependency.

## Meaning

The mixed `PA` category is not a free child projection event. It is a two-projection root-line
incidence problem.

This is not necessarily bad numerically. The observed gap frequency decreases with field size, and
the one-coordinate case still has enormous slack. But the proof needs a genuine multi-PA local
lemma, not just the one-coordinate projection lemma.

## New Theorem Target

Let:

```text
U <= H plus H
J = set of mixed PA child positions
M_J = span{h_j : j in J}
```

Define two projection matroids from:

```text
pi_1(U), pi_2(U) <= H.
```

The local theorem should bound the probability over root slopes `alpha_j` that:

```text
rank{a_j mod (U + span p_j)} <= r.
```

The expected exponent should be controlled by:

1. true child projection rank defects of `J` modulo `pi_1(U)+pi_2(U)`;
2. root-line incidence equations of the form:

```text
sum mu_j h_j in pi_1(U),
sum mu_j alpha_j h_j in pi_2(U) + low-rank A span;
```

3. recursive marked incremental rank tails for lower-dimensional witness subsets.

In short:

```text
multi-PA requires a two-projection/root-line incidence lemma.
```

This is the current precise blocker for extending the small-flat charge beyond `a=1`.
