# RFC Root-Line Schwartz-Zippel Envelope

Status: proposed escape hatch after the D=3 exact-table warning.

The D=2 PGL2 envelope gives a sharp local theorem brick. The first `D=3` probe shows that exact
repair probabilities can split even under a side-colored matroid signature over `GF(5)`. That makes
an exact finite table unattractive.

This note records the more robust alternative: once a root-line repair instance is generically
full-rank, failure over the RFC root domain is bounded by a determinant polynomial.

## Setup

Let:

```text
dim K_P = D.
```

For singleton restrictions `ell_i in K_P^*`, the root-line rows are:

```text
(ell_i, alpha_i ell_i) in K_P^* plus K_P^*.
```

Each alpha variable ranges over a side domain of size `q-1`:

```text
left:  F_q^*
right: F_q \ {1}.
```

Choose any `2D` singleton rows. Their `2D x 2D` determinant is a polynomial:

```text
Delta(alpha_1,...,alpha_{2D}).
```

Every monomial in this determinant uses exactly the `D` right-block columns, so:

```text
total degree(Delta) <= D,
individual degree in each alpha_i <= 1.
```

If this determinant polynomial is not identically zero, then by Schwartz-Zippel over product
domains of size `q-1`:

```text
Pr[Delta = 0] <= D / (q-1).
```

Therefore:

```text
Pr[root-line repair rank < 2D] <= D / (q-1)
```

whenever some `2D`-row minor has a nonzero determinant polynomial.

## Generic Full-Rank Condition

The remaining algebraic condition is to characterize when such a nonzero minor exists.

The expected condition is the generic matroid-union/Hall condition:

```text
rank(K_P|_T) = D
and
min_A ( |T|-|A| + 2 rank(K_P|_A) ) >= 2D.
```

This is the same Hall quantity already used by the fixed-survivor diagnostic. If this condition
holds, the two-copy root-line row system has generic rank `2D`; equivalently, at least one minor
polynomial is nonzero.

This implication is proved in:

```text
rfc_rootline_generic_rank_lemma.md
```

It is the standard matroid-union rank formula for two copies of the singleton matroid, realized by
the generic diagonal variables `alpha_i`.

## Generic-Rank Proof Sketch

Let `M` be the linear matroid on singleton indices represented by the functionals `ell_i`. Consider
the generic row vectors:

```text
v_i(alpha_i) = (ell_i, alpha_i ell_i).
```

Work over the rational function field:

```text
F_q(alpha_i : i in T).
```

For any subset `A` of rows, rows outside `A` contribute at most `|T|-|A|` rank. Rows inside `A`
live in:

```text
span(ell_A) plus span(ell_A),
```

which has dimension at most `2 rank_M(A)`. Therefore every specialization satisfies:

```text
rank(v_T) <= |T|-|A| + 2 rank_M(A).
```

Taking the minimum over `A` gives the Hall upper bound:

```text
rank_generic(v_T) <= min_A ( |T|-|A| + 2 rank_M(A) ).
```

The needed lower bound is the matroid-union direction. The theorem statement is:

```text
rank_generic(v_T) = min_A ( |T|-|A| + 2 rank_M(A) ).
```

This is the rank formula for the union of two copies of `M`, represented through the one-row family
`(ell_i, alpha_i ell_i)`.

A proof route:

1. Apply the matroid union theorem to two copies of `M`. If the Hall minimum is at least `2D`, then
   there are two disjoint independent sets whose union has size `2D` in the duplicated matroid.
2. Use the corresponding exchange/matching structure to choose `2D` original singleton rows.
3. Expand the corresponding root-line minor. The monomial for the selected second-copy rows is
   unique and has nonzero coefficient, so the minor is not identically zero.

This proves that Hall-OK means at least one minor polynomial is nonzero.

This proof does not need exact cross-ratio data. Cross-ratio can change the number of roots of a
nonzero determinant polynomial, but not the fact that the polynomial is nonzero.

## Meaning For D=3

The `GF(5)` D=3 probe found exact-probability splitting inside side-colored matroid signatures.
That is bad for exact table compression.

But if the Hall/generic-rank condition holds, the determinant envelope gives:

```text
Pr[repair failure] <= 3/(q-1).
```

This is cross-ratio-free and side-placement-free except through the existence of a nonzero minor.
It may be too weak for the final production distance, but it prevents the D=3 exact-table warning
from being an immediate no-go.

## Proof Obligations

To use this in the distance certificate, prove:

```text
1. Hall/generic rank:
   done locally in rfc_rootline_generic_rank_lemma.md.

2. Restricted-domain Schwartz-Zippel:
   for nonzero polynomial degree <=D over product sets of size q-1,
   the zero probability is <= D/(q-1).

3. Recurrence compatibility:
   deterministic Hall failures are charged structurally;
   Hall-OK failures pay one q^-1-type local random factor.
```

The third item is the risk. A uniform `D/(q-1)` factor may be too weak if many levels need repair
with little surplus. The D=2 PGL2 envelope is sharper in special cases, so the recurrence should
use the sharper D=2 brick when available and fall back to the determinant envelope for higher `D`.

## Current Verdict

This is a viable local fallback to chasing exact D=3 probability tables, but it is not strong
enough by itself for near-MDS.

It changes the next question from:

```text
Can exact D=3 repair be compressed by a finite signature?
```

to:

```text
Can the global recurrence tolerate a D/(q-1) repair factor whenever Hall passes?
```

The answer to that global-tolerance question appears to be no. The top-profile tolerance check in:

```text
rfc_surplus_repair_codimension_target.md
```

shows that the one-minor `D/(q-1)` bound is thousands of bits too weak near `e=71`. Near-MDS needs
a surplus-sensitive codimension bound, roughly:

```text
Pr[repair failure] <= poly(D,t) q^{-(t-2D+1)}.
```
