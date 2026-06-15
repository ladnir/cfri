# RFC Root-Line Kernel State

This note upgrades the support-profile bound to the sharper local object.

## Setup

Let:

```text
U <= F^S
```

be the child image space on a singleton block. For a visible root support `A subset S`, define:

```text
U_A = {u in U : supp(u) subset A}
delta(A) = dim U_A = rank(S) - rank(S \ A).
```

For each `j in A`, choose a projective root line:

```text
ell_j <= F^2.
```

Define the root-line kernel:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for every j in A }.
```

Let:

```text
kappa_A(ell) = dim K_A(ell).
```

Then the local count for `tau`-dimensional visible subspaces is controlled by:

```text
C_A(tau) = sum_{ell in (P^1)^A} GaussianBinomial(kappa_A(ell), tau)_q.
```

This is much sharper than the coarse support bound:

```text
GaussianBinomial(2 delta(A), tau)_q.
```

## Exact-Support Inversion

`C_A(tau)` counts subspaces whose support is contained in `A`. If an exact-support subspace has
support `B subset A`, the unused coordinates in `A \ B` may carry arbitrary projective line labels.
Therefore:

```text
C_A(tau) = sum_{B subset A} E_B(tau) * (q+1)^(|A|-|B|),
```

where `E_B(tau)` is the exact-support root-line count. The script inverts this relation by
increasing support size.

This gives exact local compatible-subspace counts, up to the small orientation/root-existence
constant that distinguishes actual RFC left/right roots from all projective lines. That constant is
irrelevant to the q-exponent and can be tracked later as a `2^|A|` factor.

## Boundary Cases

For `tau=1`, every line is locally compatible. The exact-support root-line count agrees with the
support-profile line count.

For `tau=delta(A)`, full visible rank forces component-wise diagonal behavior. The high-kernel line
assignments are exactly the diagonal endomorphism choices:

```text
ell_j is constant on each connected component of M|A.
```

So the full-rank count is:

```text
(q+1)^comp(A).
```

This recovers the exterior component codimension from a linear kernel profile.

## Full-Rank Kernel Lemma

The component boundary can be stated as a concrete lemma.

Let `U <= F^A` have no zero coordinates on `A`, and decompose its represented matroid into connected
components:

```text
A = A_1 disjoint union ... disjoint union A_c.
```

For a projective line assignment `ell in (P^1)^A`:

```text
dim K_A(ell) = dim U
```

only if `ell` is constant on each connected component. Conversely, any component-wise constant
assignment has:

```text
dim K_A(ell) = dim U.
```

Therefore:

```text
# { ell : dim K_A(ell) = dim U } = (q+1)^c.
```

Proof sketch:

1. The converse is direct. On a component `A_i`, a constant projective relation

   ```text
   alpha_i x + beta_i y = 0
   ```

   leaves one free copy of `U|A_i`. Summing components gives `dim U`.

2. For the forward direction, restrict to one connected component. In any affine chart where the
   lines are finite slopes, `K_A(ell)` is:

   ```text
   { x in U : lambda * x in U }.
   ```

   Full dimension means the diagonal multiplier `lambda` preserves `U`. For a connected
   represented matroid, every diagonal endomorphism is scalar, so `lambda` is constant on the
   component.

3. The projective charts with vertical lines are the same argument after swapping or taking another
   nonzero linear combination of the two copies. If a component contains two genuinely different
   projective lines, the full-dimension kernel would produce a non-scalar diagonal endomorphism on
   that connected component, contradiction.

This is the linear-kernel version of:

```text
dim End_diag(U|A) = comp(A).
```

The remaining local proof work is not this boundary case; it is the intermediate profile
`dim K_A(ell) >= tau` for `1 < tau < delta(A)`.

For the first intermediate case `tau=2`, the current target is in:

```text
docs/rfc_distance_analysis/rfc_tau2_kernel_profile_target.md
```

The first rank-drop layer relative to the generic two-copy matroid-union baseline is in:

```text
docs/rfc_distance_analysis/rfc_h1_kernel_drop_bound.md
```

The second rank-drop layer is reduced to a cofactor/singular-locus statement in:

```text
docs/rfc_distance_analysis/rfc_h2_cofactor_bound.md
```

For the weighted `tau=2` count, the cleaner route is through the exterior variety:

```text
docs/rfc_distance_analysis/rfc_tau2_weighted_exterior_bound.md
```

## Profiler

The diagnostic script is:

```text
scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py
```

It enumerates projective root-line assignments, computes `kappa_A(ell)`, applies the exact-support
inversion, and groups by:

```text
rank(S), comp(S), tau, |A|, rank(A), comp(A), delta(A), rank(S \ A),
generic_kernel_dim.
```

Here `generic_kernel_dim` is computed from the two-copy matroid-union rank of the coordinate
matroid of `U_A`, not from the sampled minimum. The profiler also reports the older
two-endpoint diagnostic columns:

```text
generic_endpoint_logq   = |A| + tau * (generic_kernel_dim - tau)
component_endpoint_logq = comp(A) + tau * (delta(A) - tau)
endpoint_bound_logq     = max(generic_endpoint_logq, component_endpoint_logq)
```

For `tau=2`, after root weighting this diagnostic is:

```text
endpoint_root_weight_logq
  = max(2g(A)-4, comp(A)+2delta(A)-4-|A|).
```

This is no longer the full local theorem. The proof target is the layer-codimension value:

```text
theta_2(A) = max_h(2h - 4 - gamma_h(A)).
```

Profiler detail rows now include empirical dominant-layer diagnostics to expose rows where the
two-endpoint shortcut is stale.

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_base_c8_size8_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_base_c8_size8_tau2_supports.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_child_depth1_c4_size3_tau1_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_child_depth1_c4_size3_tau1_supports.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_child_depth1_c4_size3_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_child_depth1_c4_size3_tau2_supports.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf7_child_depth1_c4_size3_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf11_child_depth1_c4_size3_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf5_child_depth2_c4_size4_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf5_child_depth2_c4_size4_tau2_supports.csv
```

## Checks

For `GF(5)`, child depth `1`, expansion `4`, size `3`, `tau=2`, the root-line kernel profile is:

```text
rho comp tau |A| rank(A) comp(A) delta rank(S\A) shapes logq_count kernel_profile
2   1    2   3   2       1       2     0          24     1.113283  1:210;2:6
2   2    2   3   2       2       2     0          30     2.226566  1:180;2:36
```

This exactly matches the visible-span enumeration:

```text
connected rank 2:       q+1 compatible full-rank subspaces
two components rank 2:  (q+1)^2 compatible full-rank subspaces
```

For the sampled `GF(5)`, child depth `2`, expansion `4`, size `4`, `tau=2`, the exact root-line
counts also match the visible-span support summary. Example rows:

```text
rho comp tau |A| rank(A) comp(A) delta rank(S\A) logq_count kernel_profile
3   1    2   4   3       1       3     0          4.470155  2:1290;3:6
3   2    2   4   3       2       3     0          4.618797  2:1260;3:36
3   3    2   4   3       3       3     0          5.410263  2:1080;3:216
```

The high-kernel count scales with component count:

```text
6, 36, 216 = (q+1)^1, (q+1)^2, (q+1)^3.
```

The exact depth-1, size-3 checks over `GF(7)` and `GF(11)` show the same pattern:

```text
GF(7):  kernel profiles 1:504;2:8 and 1:448;2:64
GF(11): kernel profiles 1:1716;2:12 and 1:1584;2:144
```

The high-kernel layer is again `(q+1)^comp(A)`, while the root factor `q^-|A|` supplies the
singleton charge used by the weighted exterior theorem.

Dense supports show why the component endpoint is not enough. In the sampled `GF(5)`, child
depth-2, size-4 run, rows with `delta=3`, `|A|=4`, and `generic_kernel_dim=2` have:

```text
generic_endpoint_logq = 4
component_endpoint_logq = 4 or 5 depending on comp(A)
```

For connected rank-3 support with `comp(A)=1`, the component-only endpoint would be `3`, but the
generic endpoint is `4`; the exact finite-field count is slightly above `4` because of constants.
This closes that `g=2` row, but the later `a=5,delta=3,g=1` row shows that the full theorem must
track intermediate `kappa` layers by `theta_2`, not by the two-endpoint maximum alone.

## Proof Target

The next local theorem should bound the kernel-dimension profile:

```text
N_A(kappa >= t) = # { ell in (P^1)^A : dim K_A(ell) >= t }.
```

A plausible structure is a stratification by subcodes `V <= U_A` whose coordinatewise diagonal
multiplier space has a given component decomposition. The full-rank boundary is already:

```text
N_A(kappa = delta(A)) = (q+1)^comp(A).
```

The global recurrence should use the exact-support root-line polynomial:

```text
E_A(tau) * q^-|A|
```

as the local singleton factor, together with the recursive charge for kernel directions invisible
on the block.
