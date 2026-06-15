# RFC All-Mixed PA Full-Span Reduction

Status: corrected interpretation of the all-mixed PA event.

## Setup

For an all-mixed PA block, condition on:

```text
U <= H plus H
J = mixed child positions
|J| = m
x_j = (h_j,0)
y_j = (0,h_j)
p_j(alpha) = x_j + alpha_j y_j
```

The marked A columns are equivalent, modulo the mixed P graph columns, to the `y_j`.

Define:

```text
Full(J) = rank( U + span{x_j,y_j : j in J} ) - rank(U)
P_alpha(J) = rank( U + span{p_j(alpha) : j in J} ) - rank(U)
A_alpha(J) = rank( U + span{p_j(alpha),y_j : j in J} )
             - rank( U + span{p_j(alpha) : j in J} ).
```

Because:

```text
span{p_j(alpha), y_j} = span{x_j,y_j}
```

for every `alpha_j`, we have the exact identity:

```text
A_alpha(J) = Full(J) - P_alpha(J).
```

## Consequence

The feared event:

```text
A_alpha(J) <= r
```

is not primarily a finite-root drop. Since `P_alpha(J) <= m`, it implies:

```text
Full(J) <= m + r
```

unless `P_alpha(J)` is unusually large, which cannot happen beyond `m`.

In the common/generic case where:

```text
P_alpha(J) = m,
```

the bad event is exactly:

```text
Full(J) <= m + r.
```

Special root values can only reduce `P_alpha(J)`, which increases `A_alpha(J)` and therefore helps
against the low-rank flat event. This is the opposite of the earlier finite-root-drop intuition.

## Profiler Evidence

The updated profiler:

```text
scripts/rfc_distance_analysis/rfc_multi_pa_graph_contraction_profile.py
```

reports `full_rank`, `min_rank_a`, `max_rank_a`, and `max_p_rank`. For:

```text
GF(5), k=7, mixed=3, graph U rows=3, instances=500
```

the rows with low marked rank satisfy:

```text
full_rank = 5,
max_p_rank = 3,
min_rank_a = 2.
```

That is:

```text
min_rank_a = full_rank - max_p_rank = 5 - 3 = 2.
```

The low marked rank comes from deterministic full-span deficiency, not from roots making a generic
rank collapse. Rows with:

```text
full_rank = 6,
max_p_rank = 3
```

have:

```text
rank_A = 3
```

for all sampled/enumerated roots.

## New Charge Target

For all-mixed PA witnesses, the first charge should be:

```text
Full(J) <= m+r.
```

This says that the two-copy child span contributed by the mixed coordinates is deficient modulo
the existing relation `U`.

Equivalently, the `2m` vectors:

```text
(h_j,0), (0,h_j), j in J
```

have rank at most `m+r` modulo `U`.

For a flat witness with:

```text
m = a = 2r+F,
```

the full-span deficiency is at least:

```text
2m - (m+r) = m-r = r+F.
```

This is exactly the same deficit scale that appeared in the random-code small-flat exponent, but
now it is a deterministic lower-level relation event rather than a finite-root event.

## Meaning For The Proof

The multi-PA blocker is less scary after this correction. The route should be:

```text
all-mixed PA low rank
  -> full two-copy span deficiency modulo U
  -> charge by a marked/two-copy child relation tail.
```

Finite-root incidence is still needed for profiles mixing `A0`, `AA`, and `PA`, and for cases where
`P_alpha(J)` itself has non-generic behavior. But for the pure all-mixed stress profile, roots do
not create the bad event; they can only remove it.

The remaining theorem target is therefore a two-copy full-span deficiency bound for:

```text
rank( U + H_J plus H_J ) - rank(U) <= |J| + r.
```

This is closer to the existing paired-compression/child-rank machinery than the previous
multi-root incidence formulation.
