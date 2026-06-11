# RFC Visible-Span Local State

This note refines the replica-span diagnosis to the actual local object.

## Local Object

Let `S` be a singleton block and let:

```text
U <= F^S
```

be the child image space restricted to `S`. For a parent message subspace `W`, the singleton block
sees a visible subspace:

```text
R <= U + U.
```

Write:

```text
tau = dim R.
```

The singleton compatibility condition is coordinatewise:

```text
for every j in S, dim projection_j(R) <= 1 in F^2.
```

Coordinates with projection rank `1` need one root value. Coordinates with projection rank `0` are
invisible/common-zero for this local block.

## Profiler

The diagnostic script is:

```text
scripts/rfc_distance_analysis/rfc_visible_span_profile.py
```

It enumerates `tau`-dimensional subspaces `R <= U+U`, tests the coordinatewise rank condition, and
groups compatible subspaces by:

```text
rank(U), comp(U), tau, root_count.
```

It also emits a support-refined summary. For `A = supp(R)`, this records:

```text
support_rank        = rank_M(A)
support_components  = comp(M|A)
complement_rank     = rank_M(S \ A)
support_kernel_dim  = dim{u in U : supp(u) subset A}
                    = rank_M(S) - rank_M(S \ A).
```

The last quantity is the one that controls how many child words can live entirely on the root
support.

Two non-enumerative support profilers now complement it:

```text
scripts/rfc_distance_analysis/rfc_support_profile_bound.py
scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py
```

The first computes the coarse Gaussian support-containment bound from `delta(A)`. The second fixes
the projective root line at every visible coordinate and counts the resulting kernel dimensions.
The root-line version matches the exact visible-span counts in the small checks and is the sharper
proof object.

## Exact Base Profiles

For the base repetition block with expansion `8`, the full set of base coordinates has
`rho=1` and one parallel component. There are no compatible `tau=2` exact-support visible
subspaces on all eight coordinates:

```text
python scripts/rfc_distance_analysis/rfc_visible_span_profile.py \
  --prime 5 \
  --child-depth 0 \
  --expansion 8 \
  --size 8 \
  --tau 2
```

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_base_c8_size8_tau2_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_base_c8_size8_tau2_support_summary.csv
```

This is the simplest local obstruction: a two-dimensional parent/replica span cannot be charged
against a rank-one parallel singleton block unless one direction is invisible on that block and is
paid elsewhere.

For `GF(5)`, child depth `1`, expansion `4`, selected size `3`, `tau=1`:

```text
rho comp tau root_count shapes logq_count
1   1    1   3          2      1.113283
2   1    1   2          24     1.795889
2   1    1   3          24     3.061475
2   2    1   1          30     1.113283
2   2    1   2          30     1.113283
2   2    1   3          30     3.087919
```

Every line is locally compatible; the root count is its support size on `S`.

For the same setup with `tau=2`:

```text
rho comp tau root_count shapes logq_count
2   1    2   3          24     1.113283
2   2    2   3          30     2.226566
```

Here `logq_count = log_5(q+1)` for one connected component and `log_5((q+1)^2)` for two
components. This matches the component-wise diagonal-graph picture.

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_child_depth1_c4_size3_tau1_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_child_depth1_c4_size3_tau1_support_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_child_depth1_c4_size3_tau2_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_child_depth1_c4_size3_tau2_support_summary.csv
```

The support-refined rows explain the small `q+1` line counts. For example, in the `rho=2`,
connected case, a line with root count `2` has `support_rank=2` but only:

```text
support_kernel_dim = 1.
```

So it is counted by projective lines in a two-dimensional space `(U_A)^2`, giving `q+1` lines. The
full-support line case has `support_kernel_dim=2`, giving projective lines in a four-dimensional
space.

## Depth-2 Sample

For `GF(5)`, child depth `2`, expansion `4`, selected size `4`, `tau=2`, sampled `20` subsets.
Seven rank-`4` local sets were skipped because the exact subspace enumeration is too large; the
script now prechecks this using the Gaussian binomial count.

```text
rho comp tau root_count shapes logq_count
2   2    2   4          1      2.226566
3   1    2   3          1      1.974636
3   1    2   4          1      4.470155
3   2    2   3          6      2.942766
3   2    2   4          6      4.618797
3   3    2   2          5      2.226566
3   3    2   3          5      2.657242
3   3    2   4          5      5.410263
```

Saved artifact:

```text
docs/rfc_distance_analysis/rfc_visible_span_sample_gf5_child_depth2_c4_size4_tau2_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_sample_gf5_child_depth2_c4_size4_tau2_support_summary.csv
```

This shows that for `tau < rank(U)`, root support is a real state variable. The support summary
shows that `support_kernel_dim` is the sharper version of that variable: it determines the ambient
dimension available to visible subspaces supported on `A`.

## Lemma Target

The local theorem should bound compatible visible subspaces by support/rank data.

For a visible subspace `R <= U+U`, define:

```text
supp(R) = { j in S : projection_j(R) != 0 }
a       = |supp(R)|
tau     = dim R
delta   = dim{u in U : supp(u) subset A}
        = rank_M(S) - rank_M(S \ A)
```

The root charge is `a`. The algebraic compatibility count should be controlled by a matroid
support profile of `U`, not just by `rank(U)` and `comp(U)`. A first universal upper bound is:

```text
R <= U_A + U_A,  dim U_A = delta,

# tau-subspaces with support subset A
  <= GaussianBinomial(2 delta, tau)_q
  ~= q^(tau(2 delta - tau)).
```

This bound is tight for the automatic `tau=1` compatibility layer up to projectivization and
inclusion-exclusion by exact support. It is too loose at full visible rank, where the exterior
conditions force component-wise diagonal graphs.

## Support-Containment Lemma

This part is already proved and can be used safely.

Let:

```text
U_A = {u in U : supp(u) subset A}.
```

If `R <= U+U` has visible support contained in `A`, then every vector `(x,y) in R` has:

```text
x_j = y_j = 0 for all j notin A.
```

Since `x,y in U`, this means:

```text
x,y in U_A,
R <= U_A + U_A.
```

The dimension of `U_A` is:

```text
dim U_A = dim U - rank(projection of U to S \ A)
        = rank_M(S) - rank_M(S \ A)
        = delta(A).
```

Therefore the number of `tau`-dimensional subspaces with visible support contained in `A` is at
most:

```text
GaussianBinomial(2 delta(A), tau)_q.
```

For `tau=1`, compatibility is automatic for every projective line, so exact-support counts are just
the support-profile coefficients obtained by subtracting smaller supports. For `tau>1`, this is
only the ambient support bound; the determinant/rank-one constraints provide the additional saving.

## Root-Line Kernel Lemma

The sharper local object fixes the root lines first.

For each `j in A`, choose:

```text
ell_j <= F^2
```

and define:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for all j in A }.
kappa_A(ell) = dim K_A(ell).
```

Every compatible visible subspace with exact support `A` determines a unique line assignment
`ell`. Therefore exact-support compatible subspaces are counted by the weighted inversion of:

```text
C_A(tau) = sum_{ell in (P^1)^A} GaussianBinomial(kappa_A(ell), tau)_q.
```

The containment relation is:

```text
C_A(tau) = sum_{B subset A} E_B(tau) * (q+1)^(|A|-|B|),
```

where `E_B(tau)` is the exact-support root-line count. This is inverted by increasing support size.

For `tau = delta(A)`, the only high-kernel root-line assignments are component-wise diagonal:

```text
E_A(delta(A)) ~= (q+1)^comp(A)
```

in the full-support case. This recovers the exterior component correction from a linear-kernel
statement.

The root-line details are in:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_state.md
```

Two useful boundary cases are now clear:

```text
tau = 1:
  compatibility is automatic;
  count is governed by the line-support distribution of U_A + U_A,
  hence by delta(A).

tau = rank(U):
  compatible full-visible-rank subspaces are component-wise diagonal graphs;
  dimension is comp(U).
```

The induction state must therefore carry:

```text
visible dimension tau,
visible root support a,
coordinate matroid rank/components of that support,
support subcode dimension delta(A),
kernel dimension t - tau outside the block.
```

This is the first formulation that cleanly separates the three effects that kept being conflated:

```text
1. root equations on visible singleton coordinates,
2. exterior/rank-one algebra on visible subspaces,
3. invisible kernel directions that must be charged elsewhere.
```
