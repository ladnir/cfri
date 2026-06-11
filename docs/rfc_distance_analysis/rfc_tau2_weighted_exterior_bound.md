# RFC Tau-2 Weighted Exterior Bound

This note replaces the layer-by-layer `kappa` summation for `tau=2` with a cleaner endpoint-bound
target. The first component-only version was too optimistic for dense supports; the generic
two-copy matroid-union kernel must also be included.

## Root-Line Polynomial

For `A subset S`, define:

```text
C_A(2) = sum_{ell in (P^1)^A} GaussianBinomial(kappa_A(ell), 2)_q.
```

This is the contained-support root-line count for two-dimensional visible subspaces.

The exact-support count `E_A(2)` is obtained from the weighted support inversion:

```text
C_A(2) = sum_{B subset A} E_B(2) * (q+1)^(|A|-|B|).
```

For an upper bound:

```text
E_A(2) <= C_A(2).
```

## Ordered-Basis Reduction

The exterior reduction applies directly to `E_A(2)`, not to `C_A(2)`.

A two-dimensional subspace:

```text
R <= K_A(ell)
```

has:

```text
|GL_2(F_q)|
```

ordered bases. An ordered basis of `R` is a pair of vectors in `U_A + U_A`:

```text
(x,y), (z,w).
```

For an exact-support subspace, the condition that both basis vectors lie in the same root-line
kernel is exactly:

```text
x_j w_j - y_j z_j = 0  for every j in A.
```

If `R` has exact support `A`, then at every coordinate `j in A` the projection of `R` is a nonzero
line in `F^2`. Hence the root line `ell_j` is uniquely determined by `R`. This removes the
contained-support ambiguity where a zero coordinate could carry an arbitrary unused root line.

Thus ordered bases counted by `E_A(2)` inject into the exterior variety:

```text
V_2(U_A) = {
  (x,y,z,w) in U_A^4 :
  x_j w_j - y_j z_j = 0 for all j in A
}.
```

Hence:

```text
E_A(2) <= |V_2(U_A)| / |GL_2(F_q)|.
```

At exponent level:

```text
log_q E_A(2) <= log_q |V_2(U_A)| - 4 + O(1/log q).
```

The `O(1/log q)` term is the finite `|GL_2(F_q)|` constant. Tiny-field diagnostics should be read
as structural checks by `(rank, comp)` and root-line profile, not as literal constant-free exponent
certificates.

## Layer-Codimension Endpoint Bound

Let:

```text
a      = |A|
delta  = dim U_A
comp   = comp(U_A)
g      = generic dim K_A(ell)
```

where `g` is the two-copy matroid-union kernel dimension:

```text
g = 2 delta - min_{B subset A} (a - |B| + 2 rank_{U_A}(B)).
```

For each kernel layer define:

```text
X_h(A)      = { ell in (P^1)^A : dim K_A(ell) >= h }
gamma_h(A)  = codim X_h(A) inside (P^1)^A.
```

The proof-safe local endpoint target is the layer bound:

```text
log_q E_A(2)
  <= a + max_{2 <= h <= delta} (2h - 4 - gamma_h(A))
     + polylog_q(a),

log_q(E_A(2) q^-a)
  <= max_{2 <= h <= delta} (2h - 4 - gamma_h(A))
     + polylog_q(a).
```

Reason: the layer `dim K_A(ell) = h` contributes at most:

```text
|X_h(A)| * GaussianBinomial(h, 2)_q
  <= poly(delta) q^(a - gamma_h(A) + 2h - 4).
```

This layer form is the right local theorem statement. It exposes intermediate rank-drop layers
instead of forcing them into the generic or component endpoint.

## Two-Endpoint Shortcut And Its Gap

The previously proposed shortcut was:

```text
log_q(E_A(2) q^-a)
  <= max(
       2g - 4,                         if g >= 2,
       comp + 2delta - 4 - a
     )
     + polylog_q(a).
```

It is only a corollary of the layer theorem under the extra intermediate-layer condition:

```text
for every 2 <= h <= delta:
  gamma_h(A)
    >= 2h - 4
       - max(2g - 4 if g >= 2 else -infinity,
             comp + 2delta - 4 - a).
```

Equivalently, every intermediate rank-drop layer must be no larger than the larger of the generic
and full-rank/component endpoints. This condition is not automatic.

The component-only expression:

```text
comp + 2delta - 4 - a
```

is only the high-kernel endpoint. It misses dense supports where the generic kernel dimension
`g >= 2`, and it can also miss first contributing rank-drop layers when `g < 2`.

Example: if `U_A` is the rank-`3` hyperplane on `a=4` coordinates, then `delta=3`, `comp=1`, and
`g=2`. The component endpoint gives root-weight exponent `-1`, but the generic endpoint gives `0`,
which is what the exact root-line profiles approach as `q` grows.

New blocker example: the support-capture artifact contains connected exact-support rows with:

```text
a = 5
delta = 3
comp = 1
g = 1
kernel_profile = 1:226512;2:22308;3:12 over GF(11)
exact_root_line_count = 21744
endpoint_excess_logq = 1.1649413340457837 against the two-endpoint bound
residual_endpoint_excess_logq = 0.78477135985591195 after the old finite constants
```

This is not a finite-constant issue for the two-endpoint shortcut. It is the codimension-one layer:

```text
X_2(A) = { ell : dim K_A(ell) >= 2 }.
```

For this row, `gamma_2(A)` is empirically `1`, so the layer theorem gives root-weight exponent:

```text
2*2 - 4 - gamma_2(A) = -1,
```

while the old shortcut predicted `-2`. The shortcut is therefore false as a general local theorem.
The corrected proof tracks this layer. The local `g=1` first-drop bound is now isolated in:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
```

It proves `gamma_2 >= 1`, so this profile has local exponent `theta_2=-1`. The remaining
certificate question is whether such `theta_2=-1` rows can stack through the global recurrence.

## Generic Determinantal Layer Model

The layer theorem suggests a compact generic model. If `g` is the generic kernel dimension and
`h < delta`, the ordinary determinantal prediction for the first non-full-rank layers is:

```text
gamma_h(A) >= (h - g)^2        for h > g,
gamma_h(A) = 0                for h <= g.
```

The full-kernel layer is different and should be charged by the diagonal-endomorphism/component
lemma:

```text
gamma_delta(A) = a - comp(A).
```

This gives the local tau-2 exponent:

```text
theta_2(A)
  = max(
      max_{2 <= h < delta} (2h - 4 - max(0,h-g)^2),
      2delta - 4 - (a - comp(A))
    ).
```

Consequences:

```text
g = 0:
  the first contributing layer h=2 has theta_2 >= -4.

g = 1:
  the first contributing layer h=2 has theta_2 >= -1.

g >= 2 and delta > g:
  the first drop h=g+1 gives theta_2 >= 2g - 3,
  one q-dimension larger than the old generic endpoint 2g-4.

delta = g:
  the full-kernel/component endpoint controls the top layer instead.
```

Thus the old generic/component max can be wrong even when `g >= 2`, if the first drop above the
generic kernel dimension is an intermediate layer rather than the full-kernel component layer.

Implementation-facing charge conversion:

```text
local_charge_tau2(A)
  = 4delta(A) - 4 - theta_2(A).
```

The calibration scripts implement this as `--singleton-charge endpoint-tau2-layer`.

## Component/Full-Rank Lemma

The remaining component theorem is still useful, but only for the full-rank/high-kernel endpoint:

```text
# { ell : dim K_A(ell) = delta(A) } ~= q^comp(A).
```

Equivalently, the component family contributes:

```text
E_A(2) <= poly(a) q^(comp(A) + 2delta(A) - 4)
```

on this endpoint.

The stronger full-variety claim:

```text
dim V_2(U_A) <= 2delta(A) + comp(A)
```

is false for dense connected restrictions and should not be used as the `tau=2` proof. The
replacement proof obligation is to bound the layer codimensions `gamma_h(A)` and then show that
their weighted maximum is acceptable in the global recurrence.

For the full local endpoint theorem, the singleton factor is now:

```text
E_A(2) * q^-|A|
  <= poly(a) * q^theta_2(A),

theta_2(A) = max_{2 <= h <= delta(A)} (2h - 4 - gamma_h(A)).
```

The old value:

```text
max(2g(A)-4, comp(A)+2delta(A)-4-|A|)
```

is still usable only for supports satisfying the intermediate-layer condition above.

## Why Layer Bounds Are Necessary

The `kappa`-layer method must control:

```text
sum_k # {ell : kappa_A(ell)=k} * GaussianBinomial(k,2)_q.
```

Assignment-count bounds such as:

```text
# {ell : kappa_A(ell) >= 2} <= poly(a) q^(a-2)
```

are not enough when `kappa` can be larger, because the Gaussian term contributes:

```text
q^(2kappa-4).
```

The exterior variety still counts the weighted object directly, but its dimension has three visible
types of contributions:

```text
generic matroid-union kernel endpoint,
intermediate determinantal rank-drop layers,
high-kernel component endpoint.
```

The first new intermediate layer already appears in the `a=5, delta=3, comp=1, g=1` exact-support
rows above. Its local codimension-one behavior is now proved by the `g=1` first-drop lemma, but the
global recurrence still must charge repeated appearances.

## Relation To Existing Notes

The component endpoint is studied in:

```text
docs/rfc_distance_analysis/rfc_exterior_component_codimension.md
```

The root-line kernel profiles are still useful diagnostics, but for `tau=2` the proof should now
route through:

```text
exact-support root-line count
  -> ordered bases
  -> exterior variety V_2(U_A)
  -> layer-codimension endpoint bound.
```

The current state does not avoid the all-`h` problem. The `a=5, delta=3, comp=1, g=1` row shows
that the first contributing intermediate layer must be included. That first-drop layer is now
controlled for `g=1`; higher-drop layers and global stacking remain open before this can be a full
certificate theorem.
