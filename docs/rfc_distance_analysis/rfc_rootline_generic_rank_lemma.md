# RFC Root-Line Generic Rank Lemma

Status: local theorem for the fixed-survivor rank-tail route.

This proves the Hall/generic-rank statement needed by the root-line Schwartz-Zippel envelope.

## Statement

Let `E` be a matrix whose rows are singleton functionals:

```text
ell_i in K^*,  i in T.
```

Let `M` be the linear matroid on `T` represented by these rows, with rank function:

```text
r(A) = dim span{ell_i : i in A}.
```

Introduce algebraically independent variables `alpha_i`, one per singleton row, and form the
root-line matrix:

```text
R(alpha) = [ E | diag(alpha_i) E ].
```

Equivalently, row `i` is:

```text
(ell_i, alpha_i ell_i) in K^* plus K^*.
```

Then over the rational function field `F(alpha_i : i in T)`:

```text
rank R(alpha) = min_{A subseteq T} ( |T|-|A| + 2 r(A) ).
```

In particular, if:

```text
r(T) = D
and
min_A ( |T|-|A| + 2 r(A) ) >= 2D,
```

then some `2D x 2D` root-line minor is a nonzero polynomial. Therefore the root-line repair system
has generic rank `2D`.

## Upper Bound

For any `A subseteq T`, rows outside `A` contribute at most `|T|-|A|` rank. Rows inside `A` lie in:

```text
span(E_A) plus span(E_A),
```

which has dimension at most:

```text
2 r(A).
```

Therefore:

```text
rank R(alpha) <= |T|-|A| + 2 r(A)
```

for every `A`, giving:

```text
rank R(alpha) <= min_A ( |T|-|A| + 2 r(A) ).
```

## Lower Bound

Use the matroid-union theorem for two copies of `M`. The maximum size of a union of two independent
sets in `M` is:

```text
mu = min_A ( |T|-|A| + 2 r(A) ).
```

So there exist disjoint index sets:

```text
I, J subseteq T
```

such that:

```text
|I| + |J| = mu,
I is independent in M,
J is independent in M.
```

Choose column sets `B` and `C` in the two `K^*` copies such that:

```text
det E[I,B] != 0,
det E[J,C] != 0.
```

Now take the square minor of `R(alpha)` on rows:

```text
S = I union J
```

and columns:

```text
B in the left block,
C in the right block.
```

This minor has size `mu`. Expand its determinant by choosing which rows feed the right-block
columns. The term where exactly the rows `J` feed the right block contributes the monomial:

```text
(prod_{j in J} alpha_j) det E[I,B] det E[J,C]
```

up to sign.

This monomial is unique to the choice of right-block row set `J`; no other term in the determinant
expansion has the same product of alpha variables. Its coefficient is nonzero. Hence the minor is a
nonzero polynomial, and:

```text
rank R(alpha) >= mu.
```

Combined with the upper bound:

```text
rank R(alpha) = mu.
```

## Consequence For Repair

For `dim K_P = D`, full repair needs generic rank `2D`. The lemma says this is equivalent to:

```text
r(T) = D
and
min_A ( |T|-|A| + 2 r(A) ) >= 2D.
```

This exactly matches the Hall quantity already computed by:

```text
scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py
```

under `hall_min`.

Thus a Hall-OK root-line instance has at least one nonzero `2D x 2D` repair determinant. The
Schwartz-Zippel envelope can then bound the probability that the sampled RFC roots land on the
zero set of that determinant.

## Scope

This lemma only proves generic rank. It does not give the exact finite-field failure probability.
The `GF(5)` D=3 probe shows exact probabilities can still depend on projective geometry. That
dependence is harmless for this lemma: it can change how many roots a nonzero determinant has, but
not whether a nonzero determinant exists.
