# RFC D=2 Root-Line PGL2 Envelope

Status: theorem brick for the fixed-survivor rank-tail route.

This note replaces the fragile exact cross-ratio-invariance goal by a cross-ratio-free upper
envelope for `D=2` singleton repair. The envelope is weaker than the exact local formula, but it is
much easier to prove and is likely the version that should feed a distance certificate.

## Setup

Let:

```text
K_P = ker(child evaluation on paired positions P)
dim K_P = 2.
```

Singleton positions give root-line rows on:

```text
K_P x K_P.
```

For a singleton in projective class `ell in P(K_P^*)`, the row is:

```text
(ell, alpha ell).
```

The determinant-1 nonzero-root RFC uses:

```text
left singleton:  alpha in A = F_q^*
right singleton: alpha in B = F_q \ {1}.
```

Group singleton rows by nonzero projective class. For class `i`, let:

```text
L_i = number of left singleton rows in class i
R_i = number of right singleton rows in class i
s_i = L_i + R_i.
```

Zero singleton columns in `K_P|_T` contribute no row and do not affect the repair probability; their
root choices cancel between numerator and denominator.

## Active And Inactive Classes

A projective class is **inactive** if all alpha values in that class are equal. It then contributes
one row direction:

```text
ell_i tensor (1, alpha_i).
```

It is **active** if at least two alpha values differ. It then contributes the full two-dimensional
slice:

```text
ell_i tensor F_q^2.
```

The total number of root assignments for class `i` is:

```text
(q-1)^s_i.
```

The number of inactive assignments is:

```text
h_i =
  q-1   if L_i>0 and R_i=0,
  q-1   if L_i=0 and R_i>0,
  q-2   if L_i>0 and R_i>0.
```

The number of active assignments is:

```text
a_i = (q-1)^s_i - h_i.
```

## Rank Cases

If at least two distinct projective classes are active, repair succeeds: two slices

```text
ell_i tensor F_q^2,
ell_j tensor F_q^2
```

with `ell_i` and `ell_j` distinct span all of:

```text
K_P^* tensor F_q^2.
```

Therefore failure can happen only in the following cases.

### No Active Classes

All classes contribute decomposable rows:

```text
ell_i tensor (1, alpha_i).
```

If there are at most three nonzero projective classes, rank is at most three and failure is
deterministic under the no-active pattern. The bad count is:

```text
prod_i h_i.
```

If there are at least four nonzero projective classes, rank failure means the points:

```text
(ell_i, alpha_i) in P^1 x P^1
```

lie on a divisor of bidegree `(1,1)`. Equivalently, the alpha values are either constant or are the
values of a fractional-linear map:

```text
f in PGL_2(F_q).
```

There are:

```text
|PGL_2(F_q)| = q(q^2-1)
```

nonconstant fractional-linear maps and at most `q` constant finite-alpha maps. Hence the no-active
bad count is bounded by:

```text
min(prod_i h_i, q(q^2-1)+q) = min(prod_i h_i, q^3).
```

This is independent of the cross-ratio of the projective classes.

### Exactly One Active Class

Suppose class `j` is the only active class. Its slice contributes:

```text
ell_j tensor F_q^2.
```

Modulo that slice, every inactive class contributes a row direction determined only by
`(1, alpha_i)`. Repair fails only if all inactive classes expose the same alpha direction, or if
there are fewer than two inactive classes. In either case the number of common finite alpha choices
is at most `q`.

Thus the bad count for active class `j` is bounded by:

```text
q a_j.
```

This deliberately overcounts small cases. The overcount is harmless and keeps the envelope
monotone.

## Envelope

Let `r` be the number of nonzero projective classes and:

```text
H = prod_i h_i,
Q = prod_i (q-1)^s_i.
```

Define:

```text
N_0 =
  H             if r <= 3,
  min(H, q^3)   if r >= 4.

N_1 = q sum_i a_i.
```

Then the D=2 repair failure probability is bounded by:

```text
Pr[rank root-line rows < 4]
  <= min(Q, N_0 + N_1) / Q.
```

This bound depends only on:

```text
the multiset of (L_i,R_i) over projective parallel classes.
```

It does not depend on the actual projective locations or their cross-ratios.

## Why This Helps

The exact D=2 formula still appears to be invariant under replacing the projective directions by
canonical directions, but proving that exact invariance may require delicate side-domain
inclusion-exclusion.

The envelope above avoids that problem. It proves the exponent needed by the certificate in the
main no-active case:

```text
four single-row inactive classes:  Pr <= q^3/(q-1)^4 = O(q^-1).
```

Additional singleton rows inside a class increase `s_i`, so they improve the denominator unless
they make the class active, in which case two active classes already repair.

## Diagnostic Implementation

The envelope is implemented as:

```text
parallel_class_envelope_failure_d2
```

in:

```text
scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py
```

The self-test:

```text
scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py
```

now prints both the exact canonical formula and this envelope, and flags `BOUND_FAIL` if any tested
projective direction set violates the envelope.

Checked commands:

```text
python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 5 --exhaustive-total 5

python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 7 \
  --signature 1:0/1:0/1:0/1:0 \
  --signature 1:0/1:0/1:0/0:1 \
  --signature 1:0/1:0/0:1/0:1 \
  --signature 2:0/1:0/1:0
```

Results:

```text
GF(5): 125 signatures, failures=0
GF(7): 4 selected signatures, failures=0
```

## Remaining Risk

This closes only the `D=2` root-line repair envelope. It does not prove:

```text
1. a D>2 analogue;
2. the global fixed-survivor recurrence;
3. the survivor-set union/counting grammar.
```

The next falsification test should be `D=3`: determine whether a comparable envelope exists or
whether full projective configuration data immediately leaks into the repair probability.
