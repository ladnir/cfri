# RFC Multi-PA Graph-Contraction Incidence

Status: replacement theorem target after the naive multi-PA projection reduction failed.

## Local Object

For a multi-`PA` top profile, condition on the non-mixed `P` span:

```text
U <= H plus H.
```

For each mixed child coordinate `j`, write:

```text
x_j = (h_j, 0),
y_j = (0, h_j).
```

The `P` sibling is a graph line:

```text
p_j = x_j + alpha_j y_j,
```

and the complementary marked `A` sibling differs from `p_j` by a nonzero multiple of `y_j`. Thus
the marked rank increment is:

```text
rank_A(alpha)
  = rank{ y_j : j in J } modulo U + span{ x_j + alpha_j y_j : j in J }.
```

This is the true multi-`PA` object.

## Relation To The Failed Shortcut

The naive projection shortcut tried to compare `rank_A(alpha)` to:

```text
rank{h_j : j in J} modulo pi_1(U)+pi_2(U).
```

That comparison is false. Several mixed coordinates can use the graph lines
`x_j + alpha_j y_j` together, producing rank loss even when no individual child column is in the
projection span. This is recorded in:

```text
rfc_multi_pa_projection_gap.md
```

The correct theorem should not force a direct child-projection rank comparison. It should bound the
root values `alpha_j` for which the graph-contraction rank drops.

## Incidence Form

A rank drop means that there is a nonzero coefficient vector `c` such that:

```text
sum_j c_j y_j in U + span_j (x_j + alpha_j y_j).
```

Equivalently, there are coefficients `mu_j` and some `(u_1,u_2) in U` with:

```text
sum_j mu_j h_j = -u_1,
sum_j (c_j - alpha_j mu_j) h_j = u_2.
```

For one mixed coordinate, this forces `h_j` into a projection of `U`, giving the deterministic PA
projection lemma. For several coordinates, `mu` can be supported on multiple child columns, and the
second equation becomes a root-line incidence condition in the variables `alpha_j`.

## Theorem Target

Let:

```text
m = |J|.
```

For a fixed local configuration `(U, h_J)`, define:

```text
g = generic rank_A(alpha) over F(alpha_j).
```

The needed finite-field theorem is:

```text
Pr_alpha[ rank_A(alpha) <= r ]
  <= poly(m,k) q^{-Gamma(U,h_J,r)}
```

where `Gamma` should be at least the minimum of:

1. a genuine child projection rank-tail term, when a low-dimensional subset of `h_J` already lies
   in the projections of `U`;
2. a root-line incidence term, when the drop only appears because of special `alpha_j` relations;
3. recursive marked incremental terms for lower-rank witness subsets.

For the distance proof we do not need an exact formula for every local matroid. We need enough
codimension after summing over marked profiles. Constant-factor polynomial losses are harmless at
`q=2^128`.

## Small-Field Profile

The profiler:

```text
scripts/rfc_distance_analysis/rfc_multi_pa_graph_contraction_profile.py
```

enumerates all mixed roots for small instances.

With generic random `U`, rank drops are extremely rare or absent in small samples. With graph-line
`U`, matching RFC geometry, drops appear but look finite-field/incidence-sized rather than
constant-sized.

Examples:

```text
GF(5), k=7, mixed=3, graph U rows=3, instances=500:
  total root assignments: 32000
  generic rank 3, observed rank 2 count: 291
  drop probability: 0.00909375

GF(7), k=7, mixed=3, graph U rows=3, instances=500:
  total root assignments: 108000
  generic rank 3, observed rank 2 count: 215
  drop probability: 0.00199074074074

GF(11), k=7, mixed=3, graph U rows=3, instances=100:
  total root assignments: 100000
  generic rank 3, observed rank <3 count: 0
```

This initially looked like finite-root incidence, but the updated profiler exposes a sharper
identity:

```text
rank_A(alpha) = Full(J) - P_alpha(J).
```

For the all-mixed low-rank event, roots are not the main enemy. Low marked rank occurs when the
full two-copy span `Full(J)` is already small modulo `U`; special roots can only reduce
`P_alpha(J)` and thereby increase `rank_A(alpha)`. The corrected all-mixed reduction is recorded
in:

```text
rfc_all_mixed_pa_full_span_reduction.md
```

Finite-root incidence remains relevant for non-pure profiles and for controlling the graph rank
`P_alpha`, but pure all-mixed PA should be charged first by full-span deficiency.

## Generic Rank Warning

One tempting way to compute generic rank is to treat:

```text
p_j = x_j + alpha_j y_j
```

as an arbitrary generic point on the line:

```text
span{x_j,y_j}.
```

Then a Rado/transversal formula gives a lower-looking value:

```text
generic(P+Y) - generic(P).
```

The profiler reports this as `formula_generic`. It can underestimate the exact enumerated generic
rank. For example, in the GF(5) graph-`U` profile, some instances have:

```text
formula_generic    = 2,
enumerated_generic = 3.
```

Reason: RFC graph points are not arbitrary points on the projective line. They live in the affine
chart with fixed nonzero `x_j` coefficient:

```text
x_j + alpha_j y_j,  alpha_j in F_q^*.
```

That restriction removes some degeneracies allowed by the projective-line Rado model. This is good
for distance, but it means the theorem should use the affine graph-line determinant structure, not
only abstract subspace-transversal rank.

## Meaning For The Current Distribution

The current linked `t,t+1` distribution remains viable. The multi-`PA` issue is not a collapse of
the code geometry; it is a local incidence theorem still needing proof.

The next useful attack for general mixed profiles is:

```text
classify rank_A drops by the minimal support of c,
then prove that each support either gives a child projection rank event or imposes at least one
nontrivial polynomial equation in the alpha_j, with surplus equations for higher drops.
```

This should plug into the marked incremental recurrence as the finite-root part of the mixed
category.
