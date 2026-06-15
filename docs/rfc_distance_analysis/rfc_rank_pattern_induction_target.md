# RFC Rank-Pattern Induction Target

This note states the current proof target after the `r=2` root-compatibility check.

## Moment To Bound

For a depth-`d` RFC tree and a zero-set request pattern `Z`, define:

```text
M_d(r,Z)
  = E_T[# nonzero ordered r-tuples of messages that all vanish on Z].
```

The distance certificate needs:

```text
sum_{|Z|=k+e} M_d(1,Z) <= 2^-lambda.
```

The recursion sends an `r`-tuple of parent messages to a `2r`-tuple of child messages.

## Root Split

At a root, split `Z` into:

```text
P = child coordinates where both siblings are requested zero
S = child coordinates where exactly one sibling is requested zero
```

A paired request at `j in P` forces both child value vectors to be zero:

```text
X_j = 0, Y_j = 0.
```

A singleton request at `j in S` forces:

```text
X_j + T_j(Y_j-X_j) = 0.
```

For an `r`-tuple, this is possible only when:

```text
rank span(X_j,Y_j) <= 1.
```

If the rank is `0`, the singleton is automatic. If the rank is `1`, it costs one root value and
also imposes the rank-one compatibility equations on the child tuple. If the rank is `2`, the
request is impossible.

## Desired One-Step Bound

Let a child request pattern mark coordinates as:

```text
0: child 2r-tuple is common-zero
1: child 2r-tuple is rank-one compatible
*: unrestricted
```

The needed local theorem is an upper bound of the form:

```text
E[# child 2r-tuples satisfying a pattern with a zero labels and b rank-one labels]
  <= poly(n,r,d) q^(2r k_child - r0(a,b)),
```

where `r0(a,b)` should charge:

```text
common-zero label:       2r linear equations, up to child rank bottlenecks
rank-one label:          r-1 exterior/rank equations, plus the root equation above it
generic singleton total: r equations
```

The exact bookkeeping must be shape-sensitive, because paired-only requests recurse without adding
new root randomness.

## Minimal Sufficient Claim

Update: this scalar claim is now known to be too strong as stated. The depth-5 audit found
low-visible-rank child blocks where a shape-free `q^{-r|E|}` singleton charge is false before
tuple-span and child-flag savings are exposed. The corrected sufficient claim is the finite
exact-support flag recurrence in:

```text
docs/rfc_distance_analysis/rfc_depth5_finite_flag_recurrence_target.md
```

A sufficient theorem for the near-MDS estimate is:

```text
Rank-Pattern Induction.
For every depth d, replica count r=2^h arising from the recursion, and request pattern Z,
M_d(r,Z) is bounded by the optimistic rank-one recurrence computed by
rfc_replica_zero_moment.py --singleton-charge replica,
up to poly(d,N,r) factors.
```

This would immediately give the small-depth crossings:

```text
k=4:   e=1
k=8:   e=1
k=16:  e=2
k=32:  e=2
k=64:  e=3
```

and strongly suggests the production crossing should remain within a small additive slack of the
random-rank first-moment crossing.

## Main Risk

The risk is not the local `r=2` algebra; that is now explicit. The risk is accumulation of
rank-one labels in a child matroid with direct-sum structure. A proof must show that any such
bottleneck is exactly a paired-compression event already paid recursively, or else it must add a
new shape parameter to the recurrence.

The next attack should be the `r=2` induction step:

```text
Given a child generator and a set of rank-one labels S,
bound the number of ordered child quadruples whose 2 x 2 exterior determinant vanishes on S.
```

For represented matroids this is a determinantal-variety count. The first useful lemma should be a
large-field bound depending on the rank of the selected child columns and the number of parallel or
low-rank components in the restriction.

## Tau-2 Endpoint Correction

The first component correction was still incomplete. For a selected exact visible support `A`, let:

```text
delta = dim U_A
c     = number of connected components of U_A
g     = generic two-copy root-line kernel dimension
```

This correction is still incomplete. The actual local root-weight exponent for `tau=2` must be
layer-based. Define:

```text
X_h(A)      = { ell : dim K_A(ell) >= h }
gamma_h(A)  = codim X_h(A) in (P^1)^A
```

Then the local exponent is:

```text
theta_2(A) = max_{2 <= h <= delta(A)} (2h - 4 - gamma_h(A)).
```

The older generic/component expression:

```text
max(2g - 4, 2delta + c - 4 - |A|)
```

is only a shortcut when all intermediate layers are dominated. It is false in general. For example
connected `a=5, delta=3, comp=1, g=1` supports have a codimension-one `h=2` layer, giving exponent
`-1` instead of the shortcut's `-2`.

The optimistic recurrence `rfc_replica_zero_moment.py --singleton-charge replica` is still useful
calibration, but neither it nor the scalar component-uniform correction is the theorem.

The component-codimension note and evidence are in:

```text
docs/rfc_distance_analysis/rfc_exterior_component_codimension.md
```

The induction target should therefore be updated to:

```text
Endpoint-Aware Rank-Pattern Induction.
For each root singleton exact support A and visible dimension tau=2, charge according to
`theta_2(A)`, with paired/common-zero coordinates recursing into the child. For every tau-two
quotient branch, also use the exact-support Grassmann cap `local_charge >= |A|` before applying any
independent quotient-lift factor. For the decomposable `|A|=2,delta=2,comp=2,K=L=0` boundary row,
split the contribution into two rank-one child moments with `outer_zeros+1` zeros each.
```

The remaining question is whether the endpoint-aware recurrence still sums to the near-MDS
crossing. This is now the next model to implement.

## Replica-Span Correction

The first component-aware scalar model has been implemented in:

```text
rfc_replica_zero_moment.py --singleton-charge component-uniform
```

It is too pessimistic and also uses the wrong local state:

```text
depth  k   optimistic-free e   component-uniform e
2      4   1                   3
3      8   1                   35
4      16  2                   98
```

The blow-up comes from high-replica tuples with small replica span dimension, while the local
tau-2 correction above adds the generic kernel dimension. The corrected state must track:

```text
t = dim span(m_1, ..., m_r).
```

The coordinate-side charge should use `t`, not just the ordered replica count `r`:

```text
|S| + t rank(S) - comp(S).
```

The companion note is:

```text
docs/rfc_distance_analysis/rfc_replica_span_state.md
```

The first subspace-span toy model shows that even global `t` is not sufficient. The induction also
needs the visible dimension on the singleton block:

```text
tau_S = dim image of the replica span after restriction to S.
```

Directions in the kernel of this restriction must be charged as vanishing/common-zero structure
elsewhere; they cannot be allowed to make singleton compatibility free.

The visible-span local profile shows that `tau_S` is still not the whole state. We also need:

```text
a_S = number of coordinates in S where the visible span has rank-1 projection.
```

This is the number of root equations. A support-refined pass shows that `a_S` alone is still too
coarse. If `A` is the visible root support inside `S`, the local line/subspace count is controlled
by:

```text
delta_S(A) = dim{u in U_S : supp(u) subset A}
           = rank(S) - rank(S \ A).
```

For `tau=1`, compatibility is automatic and the count is the projective line count in
`U_A + U_A`, so `delta_S(A)` is exactly the exponent-bearing support invariant. For full visible
rank, the exterior equations instead collapse the freedom to component-wise diagonal graphs. The
local note is:

```text
docs/rfc_distance_analysis/rfc_visible_span_local_state.md
```

The sharper formulation fixes projective root lines first. For `A subset S`, let:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for all j in A }
kappa_A(ell) = dim K_A(ell).
```

The exact-support local singleton factor is then:

```text
E_A(tau) * q^-|A|,
```

where `E_A(tau)` is obtained from:

```text
C_A(tau) = sum_{ell in (P^1)^A} GaussianBinomial(kappa_A(ell), tau)_q
C_A(tau) = sum_{B subset A} E_B(tau) * (q+1)^(|A|-|B|).
```

This root-line kernel state recovers the `r=2` component correction without brute-force subspace
enumeration. The current local target is to prove useful upper bounds for the distribution of
`kappa_A(ell)`, especially:

```text
# {ell : kappa_A(ell) = delta(A)} = (q+1)^comp(A)
```

for full-support restrictions. The companion note is:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_state.md
```
