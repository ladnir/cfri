# RFC Multi-PA Linear-Relation Form

Status: sharper invariant form of the multi-PA witness equation.

## Why Projection Spaces Are Not Enough

The previous notes sometimes phrase the multi-PA obstruction using:

```text
pi_1(U), pi_2(U) <= H.
```

Those projection spaces are useful for rough intuition, but they are not the true invariant. The
same vector in `U <= H plus H` couples the first and second projections. A witness must satisfy
both projections with the same element of `U`.

So the correct object is the linear relation:

```text
U <= H x H.
```

## Fiber Formulation

For:

```text
v in pi_1(U),
```

define the fiber:

```text
F_U(v) = { w in H : (v,w) in U }.
```

This is either empty or an affine coset of:

```text
K_2(U) = { w : (0,w) in U }.
```

For mixed PA support `S`, the witness equation:

```text
sum_{j in S} c_j y_j in U + span_j (x_j + alpha_j y_j)
```

means there are coefficients `mu_j` such that:

```text
v(mu) = - sum_j mu_j h_j in pi_1(U)
```

and:

```text
sum_j (c_j - alpha_j mu_j) h_j in F_U(v(mu)).
```

Equivalently, if `w_mu` is any chosen element of `F_U(v(mu))`, then:

```text
sum_j (c_j - alpha_j mu_j) h_j - w_mu in K_2(U).
```

This is the exact affine incidence condition in the roots `alpha_j`.

## Consequences

The naive projection shortcut loses information twice:

1. It replaces the condition `v(mu) in pi_1(U)` plus a compatible second fiber by separate
   projection membership.
2. It ignores the affine coset shift `w_mu`, which can make the root condition nontrivial even when
   the projection spans look permissive.

This explains why:

```text
rank(A_mixed mod P) >= rank(child mixed mod projection(P_other))
```

is false, while the graph-contraction rank still appears to have real finite-field codimension.

## Refined Theorem Target

For each support `S`, define the first-projection coefficient space:

```text
M_1(S) = { mu in F^S : -sum_j mu_j h_j in pi_1(U) }.
```

For each projective `mu in P(M_1(S))`, the possible roots must satisfy:

```text
diag(mu) alpha  mod K_2(U)+span(h_S with c freedom)
```

landing in an affine subspace determined by the fiber `F_U(v(mu))`.

The proof should split:

```text
small dim M_1(S):
  few projective mu choices; root incidence q-factor survives.

large dim M_1(S):
  child columns h_S have a projection rank defect relative to pi_1(U);
  charge this recursively/structurally.
```

This is the more accurate version of the minimal-support witness plan.

## Updated Proof Shape

For a rank drop by one:

1. choose minimal support `S`;
2. choose projective `mu in M_1(S)`;
3. use the fiber condition to get at least one nonzero affine equation in `alpha_S`, unless the
   whole image of `diag(mu) alpha` is absorbed by `K_2(U)+span(h_S with c freedom)`;
4. show the absorbed case is exactly a child projection/rank defect or a lower marked event.

The hard part is still step 4, but the relation/fiber language states it without losing the
coupling information in `U`.
