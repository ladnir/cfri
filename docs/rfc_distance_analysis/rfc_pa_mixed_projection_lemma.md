# RFC PA Mixed Projection Lemma

Status: deterministic local lemma candidate for the marked incremental route.

## Setup

Work at one RFC fold over a child ambient `H`. At child coordinate `j`, write the two sibling
columns as:

```text
L_j = (h_j, t_j h_j),
R_j = (h_j, (t_j+1) h_j)
```

inside:

```text
H plus H.
```

Suppose we are in the mixed `PA` case:

```text
one sibling of j is in P,
the complementary sibling is the marked A column.
```

Let `P'` be all other `P` columns, excluding the one sibling at `j`, and let:

```text
U = span(P') <= H plus H.
```

Let `C(P') <= H` be the child projection span of `P'`, meaning the span of all child columns whose
parent siblings appear in `P'`. Equivalently:

```text
C(P') = span( pi_1(U), pi_2(U) )
```

where `pi_1,pi_2` are the two coordinate projections from `H plus H` to `H`.

## Lemma

If the complementary sibling at `j` is already in the span of `P`, then:

```text
h_j in C(P').
```

Equivalently:

```text
rank_child(C(P') union {j}) = rank_child(C(P')).
```

## Proof

Assume, without loss of generality, that:

```text
L_j in P,
R_j is the marked A column.
```

If:

```text
R_j in span(U, L_j),
```

then for some scalar `lambda`:

```text
R_j - lambda L_j in U.
```

Compute:

```text
R_j - lambda L_j
  = ((1-lambda) h_j, (t_j+1-lambda t_j) h_j).
```

The two scalar coefficients cannot both be zero. The first is zero only if `lambda=1`; then the
second is `1`. Thus `U` contains a nonzero vector of the form:

```text
(a h_j, b h_j)
```

with `(a,b) != (0,0)`.

Project this vector to a nonzero coordinate. If `a != 0`, then:

```text
h_j in pi_1(U) <= C(P').
```

If `a = 0`, then `b != 0` and:

```text
h_j in pi_2(U) <= C(P').
```

So in every case:

```text
h_j in C(P').
```

The same argument applies when `R_j in P` and `L_j` is marked.

## Consequence

The mixed `PA` event for one marked A coordinate is contained in a child incremental dependence
event:

```text
rank_child(P'_proj union {j}) - rank_child(P'_proj) = 0.
```

This is stronger than the previous heuristic. If `|P|=p`, then `P'` has at most `p-1` projected
child positions. Under a random child-code scale, the q-exponent is at least:

```text
k_child - (p-1).
```

For the dominant stress:

```text
k_child = 512,
p       = 137,
```

the mixed `PA` exponent is at least:

```text
512 - 136 = 376
```

q-dimensions, before finite constants and shape-recursive losses.

This differs slightly from the earlier crude `375` estimate, which allowed the mixed child column
itself to contribute to the projection span. The deterministic lemma shows the sibling in `P` at
the same child coordinate should be excluded from the projection witness.

## Meaning For The Proof

This local lemma significantly reduces the `PA` blocker. For `a=1`, the mixed case does not require
a new root-line incidence theorem; it can be charged by an ordinary child incremental rank event.

The sanity checker:

```text
scripts/rfc_distance_analysis/rfc_pa_mixed_projection_selftest.py
```

samples random finite-field configurations and verifies the implication. Current checks:

```text
GF(5), k=5, n=12, p_other=4, trials=20000:
  containments=253, failures=0

GF(7), k=6, n=14, p_other=5, trials=20000:
  containments=18, failures=0
```

The next proof target is the multi-`PA` version:

```text
if many complementary mixed siblings in A have quotient rank <= r modulo P,
then a corresponding set of child columns has rank <= r modulo the projected P' core,
up to the A-columns that are genuinely supplied by root-line singleton equations.
```

If this extension holds, the small-flat marked recurrence can route mixed categories to child
incremental rank tails rather than treating them as new local algebra.
