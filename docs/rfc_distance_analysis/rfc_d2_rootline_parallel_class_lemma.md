# RFC D=2 Root-Line Parallel-Class Lemma Candidate

Status: local theorem candidate for the fixed-survivor rank-tail route. This note records the
evidence that the first useful singleton-repair abstraction is not the scalar profile, and not even
the rank-increment histogram alone, but the projective parallel-class structure of `K_P|_T` with
left/right singleton side labels.

## Local Setup

After paired child positions `P`, let:

```text
K_P = ker(child evaluation on P)
D = dim K_P.
```

Singleton child positions `T` impose root-line equations on:

```text
K_P x K_P.
```

For a singleton position `j`, write the induced functional on `K_P` as:

```text
ell_j in K_P^*.
```

The parent singleton row has the form:

```text
ell_j(x) + alpha_j ell_j(y) = 0.
```

For left singletons, `alpha_j = t_j`; for right singletons, `alpha_j = t_j + 1`.
With `t_j in F_q^*`, this means:

```text
left domain:  alpha in F_q^*
right domain: alpha in F_q \ {1}.
```

## D=2 Signature

For `D=2`, each nonzero `ell_j` is a point of the projective line:

```text
P(K_P^*) ~= P^1.
```

The diagnostic groups singleton positions into projective parallel classes and records only:

```text
zero columns,
multiset of (left_count, right_count) over nonzero parallel classes.
```

This deliberately discards the actual projective locations and all cross-ratios.

## Evidence

The script:

```text
scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py
```

now supports:

```text
--quotient-signature
--rootline-exact-max 6
```

The `rootline_signature_ambiguity` summary checks whether the same side pattern plus rank/Hall
histogram has multiple exact root-line repair probabilities.

The `rootline_parallel_class_ambiguity` summary checks the stronger candidate abstraction: whether
the same projective parallel-class side-count signature has multiple exact root-line repair
probabilities.

### All-Left Top-Family Check

For:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family top --challenge-samples 200 --top-k 8 --hist-limit 8 \
  --seed 5 --quotient-signature --rootline-exact-max 6
```

the output includes:

```text
rootline_signature_ambiguity total_signatures=14 ambiguous_signatures=0
rootline_parallel_class_ambiguity total_signatures=8 ambiguous_signatures=0
```

So both the rank/Hall histogram and the parallel-class signature are sufficient in that all-left
sample.

### Mixed-Side Block Check

For:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family blocks --challenge-samples 80 --top-k 14 --hist-limit 8 \
  --seed 11 --quotient-signature --rootline-exact-max 6
```

the output includes:

```text
rootline_signature_ambiguity total_signatures=96 ambiguous_signatures=54
rootline_parallel_class_ambiguity total_signatures=105 ambiguous_signatures=0
```

This is a useful separation:

1. The rank/Hall histogram is too coarse once left/right side patterns mix.
2. The projective parallel-class side-count signature still predicts the exact finite-field
   root-line repair probability in this sample.

Examples from the mixed-side run:

```text
zero=0; classes=0:1/0:1/0:1/1:0/1:1  -> 87/4096
zero=0; classes=0:1/0:1/0:1/1:0/2:0  -> 45/2048
zero=0; classes=0:1/0:1/0:1/0:1/1:1  -> 25/1024
zero=0; classes=1:0/1:0/1:0/1:0/2:0  -> 7/256
```

The script now also implements the canonical finite-field formula from only the parallel-class
signature:

```text
parallel_class_formula_failure_d2
```

It assigns arbitrary distinct projective directions in `F_q^2`, expands each class into its left
and right singleton rows, enumerates the allowed alpha domains, and checks rank failure. This is
not using the actual RFC projective coordinates.

A smaller mixed-side check:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family blocks --challenge-samples 40 --top-k 8 --hist-limit 8 \
  --seed 11 --quotient-signature --rootline-exact-max 6
```

reports:

```text
rootline_parallel_class_ambiguity total_signatures=98 ambiguous_signatures=0
rootline_parallel_formula_mismatches=0 distinct_mismatches=0
```

So the canonical formula matched the actual exact root-line enumeration for every sampled `D=2`
parallel-class signature in that run.

These values differ even when coarse rank histograms agree, because side labels are distributed
differently across parallel classes.

## Lemma Candidate

For `D=2`, the exact root-line repair failure probability over the determinant-1 nonzero challenge
domain is determined by:

```text
the number of zero singleton columns in K_P|_T,
the multiset of (left_count, right_count) over projective parallel classes.
```

Equivalently, no projective cross-ratio data is needed for `D=2`; projective parallelism plus side
domains should be enough.

This should be provable by conditioning on each projective class. A class with multiple singleton
rows contributes one or two independent equations depending on whether the allowed `alpha` values
inside that class all coincide. The remaining rank calculation depends only on which projective
classes have one equation or two equations, not on the actual cross-ratio positions of the classes,
because the ambient quotient dimension is two.

The executable formula currently proves the same claim for tested finite instances by replacement:
replace the actual projective directions with any canonical set of distinct projective directions.
The exact failure probability is unchanged in the tested `q=5,D=2,T=6` cases.

## Proof Skeleton

Fix a projective class with representative `ell in K_P^*`. Rows from that class are:

```text
(ell, alpha ell) in K_P^* plus K_P^*.
```

If all sampled `alpha` values in the class are equal, the class contributes one row direction. If
at least two sampled `alpha` values differ, the class contributes:

```text
ell tensor F_q^2
```

which is a two-dimensional slice.

Thus each root assignment induces a pattern:

```text
active classes:    at least two alpha values in the class,
inactive classes:  exactly one alpha value in the class.
```

The probability of each class outcome is determined only by `(L_i,R_i)` and the two alpha domains:

```text
left:  F_q^*
right: F_q \ {1}.
```

For a fixed outcome pattern:

1. Two active classes with distinct projective directions immediately span all of
   `K_P^* tensor F_q^2`.
2. One active class contributes one projective direction times all alpha directions; full rank then
   depends only on whether the inactive classes collectively expose two alpha directions outside
   that active class.
3. With no active classes, every class contributes one decomposable tensor
   `ell_i tensor (1,alpha_i)`. The remaining point to prove is that the number of alpha assignments
   giving rank `<4` depends only on the multiset of class side counts, not on the cross-ratio of the
   distinct projective directions `ell_i`.

Item 3 is the only real algebraic gap in this note. The self-test below attacks exactly this gap.

For four inactive classes with finite affine coordinates, the determinant is the standard
Segre/P1xP1 determinant:

```text
det rows [1, x_i, alpha_i, x_i alpha_i] = 0
```

if and only if the four points `(x_i, alpha_i)` lie on a `(1,1)` divisor. Equivalently, after
choosing an order and avoiding degenerate equalities:

```text
cr(x_1,x_2;x_3,x_4) = cr(alpha_1,alpha_2;alpha_3,alpha_4).
```

So cross-ratio dependence is a real possible failure mode in principle. The reason the
parallel-class side-count lemma may still be true is subtler: the alpha variables range over the
special side domains `F_q^*` and `F_q \ {1}`, with multiplicities forced by left/right class
counts. The count of bad alpha assignments appears invariant under changing the projective
directions, even though the determinant equation itself changes by cross-ratio.

## Cross-Ratio Self-Test

The standalone script:

```text
scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py
```

tests the replacement claim outside RFC. For a chosen parallel-class side-count signature, it
compares the canonical formula against every choice of distinct projective directions in `P^1(F_q)`.

The suspicious case is several one-equation classes, because a determinant of rows

```text
(ell_i, alpha_i ell_i)
```

could have depended on the cross-ratio of the projective directions `ell_i`. The self-test checks
this directly.

Checked commands:

```text
python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 5 \
  --signature 1:0/1:0/1:0/1:0 \
  --signature 1:0/1:0/1:0/0:1 \
  --signature 1:0/1:0/0:1/0:1 \
  --signature 2:0/1:0/1:0 \
  --signature 1:0/1:0/1:0/1:0/0:1

python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 7 \
  --signature 1:0/1:0/1:0/1:0 \
  --signature 1:0/1:0/1:0/0:1 \
  --signature 1:0/1:0/0:1/0:1 \
  --signature 2:0/1:0/1:0
```

All reported `OK`. Example rows:

```text
q=5, 1:0/1:0/1:0/1:0 -> 15/64 for all 15 direction sets
q=7, 1:0/1:0/1:0/0:1 -> 65/432 for all 70 direction sets
```

This does not replace the proof, but it directly tests the likely cross-ratio failure mode.

The script also has an exhaustive small-signature mode. The command:

```text
python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 5 --exhaustive-total 5
```

checks every side-count signature with at most five singleton rows over all choices of distinct
projective directions in `P^1(F_5)`. It reports:

```text
summary signatures=125 failures=0
```

A larger pure cross-ratio spot check was also run:

```text
python -B scripts/rfc_distance_analysis/rfc_d2_rootline_formula_selftest.py \
  --q 11 --signature 1:0/1:0/1:0/1:0
```

It reports:

```text
"1:0/1:0/1:0/1:0",4,495,93/1000,"93/1000",OK
```

This tests all `495` four-point projective direction sets in `P^1(F_11)` for the all-left
four-inactive-class case.

## Safer Proof Route: Upper Envelope Instead Of Exact Invariance

The exact parallel-class formula may be more than the certificate needs. A weaker but more robust
local theorem is:

```text
once Hall/visibility allows D=2 repair and at least four inactive projective classes are needed
to create the remaining rank, the no-active failure contribution is O(q^-1), uniformly in the
projective cross-ratio of the ell_i.
```

This avoids proving that the exact count is cross-ratio independent. The proof should use the
standard `PGL_2` interpretation of the no-active determinant.

In the no-active case each class contributes one decomposable row:

```text
ell_i tensor (1,alpha_i).
```

For four distinct projective directions `ell_i`, rank `<4` iff the four points

```text
(ell_i, alpha_i) in P^1 x P^1
```

lie on a divisor of bidegree `(1,1)`. Equivalently, either:

1. all `alpha_i` are equal, giving a horizontal ruling; or
2. the `alpha_i` are the values of a projective fractional-linear map
   `f in PGL_2(F_q)` at the four points `ell_i`.

There are:

```text
|PGL_2(F_q)| = q(q^2-1)
```

nonconstant fractional-linear maps, and at most `q` constant finite-alpha horizontal rulings.
Therefore, for
`m >= 4` inactive classes, the number of bad common-alpha assignments is bounded by:

```text
q(q^2-1) + q = q^3
```

before applying side-domain restrictions. The side domains only reduce this numerator. If this is
viewed conditionally after the classes have already been declared inactive, the denominator is the
number of allowed common-alpha choices, roughly `q^m`, giving:

```text
Pr[rank failure | these m distinct classes are inactive] <= O(q^(3-m)).
```

Unconditionally, if class `i` contains `s_i=L_i+R_i` singleton rows, the denominator is instead the
full root assignment count, roughly `q^(sum_i s_i)`, while the same bad common-alpha numerator is
used. Thus duplicate singleton rows in a projective class only improve the exponent.

In particular, four single-row inactive classes give the worst `O(q^-1)` case, uniformly over the
cross-ratio of the projective directions. Cross-ratio can affect the exact lower-order
inclusion-exclusion terms after restricting to:

```text
F_q^*,  F_q \ {1},  F_q \ {0,1},
```

but it cannot destroy the `q^-1` exponent.

This is likely the right theorem for the distance certificate: prove the exact parallel-class
formula if it stays easy, but rely on the `PGL_2` envelope for robustness. The recurrence wants a
certified exponent with finite constants, not necessarily the exact local probability.

## Next Proof Task

Derive a closed finite-field formula:

```text
Fail_q({(L_i,R_i)}_i)
```

where class `i` has `L_i` left and `R_i` right singleton rows. The formula should use:

```text
left alpha domain  = F_q^*
right alpha domain = F_q \ {1}
```

and sum over class activation patterns. A class is inactive when all its sampled `alpha` values are
equal; otherwise it contributes two independent equations for that projective direction.

After the formula is derived, the recurrence can use a table of `D=2` local repair charges rather
than exact enumeration.

Parallel next task: formalize the `PGL_2` upper envelope above with explicit finite constants for
the nonzero-root side domains. This may be enough even if exact cross-ratio independence is never
proved.

That formalized envelope is now written in:

```text
rfc_d2_rootline_pgl2_envelope.md
```
