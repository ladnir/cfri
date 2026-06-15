# RFC Tau-One Full-Line Carry Lemma

Scope: original non-systematic RFC, determinant-1 fold, `T` uniform in `F^*`.

Status: linear-algebra lemma and recurrence contract. This is a proof component, not the completed
distance certificate.

## Purpose

The current tau-one blocker is invisible-fiber mass. The diagnostic row:

```text
state (4,7)>=(2,8), rank-1 row:
outer_tau1_visible_image_qdim       = 2
outer_tau1_invisible_fiber_qdim     = 3
outer_tau1_charged_postroot_qdim    = 3
outer_tau1_visible_only_saving_qdim = 0
outer_tau1_full_line_saving_qdim    = 3
```

shows that remembering only the root-visible image of a quotient line does not save anything.
The recurrence must carry the full projective line in the tau-one ambient, or else pay the new
fiber dimension by which a descendant line differs.

This note states the exact projective-fiber counting lemma needed for that recurrence.

## Linear Setup

Let `E` be the parent tau-one quotient-line ambient after the child flag and support containment
constraints have been fixed. Let:

```text
rho : E -> A
```

be the visible/root projection to the active support coordinates. Let:

```text
R <= E
```

be the full projective line already paid for by the parent tau-one incidence count. Its visible
image is:

```text
r = rho(R) <= A.
```

Now let `E'` be a descendant tau-one quotient-line ambient in the child diagram. A compatibility
label must provide a linear transition map:

```text
phi : E' -> E
```

such that a descendant full line:

```text
R' <= E'
```

is the same carried line precisely when:

```text
phi(R') = R.
```

More generally, if only the visible image is carried, the relevant map is:

```text
rho' : E' -> A,
```

and the condition is only:

```text
rho'(R') = r.
```

## Lemma

Let `phi : E' -> E` be linear, and fix a line `R <= E`. Set:

```text
K_phi = ker(phi)
kappa_phi = dim K_phi.
```

Assume `phi^{-1}(R)` contains a vector mapping nontrivially to `R`; otherwise there are no
descendant lines with `phi(R')=R`. Then the number of projective lines `R' <= E'` satisfying:

```text
phi(R') = R
```

is exactly:

```text
q^kappa_phi
```

and in particular is at most:

```text
C_q q^kappa_phi
```

with the same finite projective constant convention as the local tau-one incidence lemma.

Consequently, if a recurrence has already paid for the full parent line `R`, the descendant line
family contributes only:

```text
kappa_phi
```

new q-dimensions, not the full descendant post-root quotient-line exponent.

## Proof

Let `U = phi^{-1}(R_hat)`, where `R_hat` is the one-dimensional vector subspace representing the
projective line `R`. The restriction:

```text
phi|_U : U -> R_hat
```

is surjective by assumption, with kernel `K_phi`. Therefore:

```text
dim U = kappa_phi + 1.
```

The lines `R' <= E'` satisfying `phi(R') = R` are exactly the one-dimensional subspaces of `U` not
contained in `K_phi`. The number is:

```text
((q^(kappa_phi+1)-1)/(q-1)) - ((q^kappa_phi-1)/(q-1))
  = q^kappa_phi.
```

This proves the claim.

## Visible-Only Corollary

Apply the lemma to the visible projection:

```text
rho' : E' -> A.
```

If a recurrence conditions only on the visible root image `r <= A`, then the new q-dimension is:

```text
dim ker(rho' on the contained-support ambient).
```

This is exactly the `tau1_invisible_fiber_qdim` diagnostic. For the first target row it is `3`,
so visible-only carrying saves zero q-dimensions:

```text
visible_fixed_cond_qdim = 3
visible_only_saving_qdim = 0
```

## Full-Line Corollary

If the compatibility label proves that the descendant tau-one line is the same carried full line,
then `phi` is injective on the relevant projective fiber:

```text
kappa_phi = 0.
```

The descendant line is determined, and the recurrence may remove the descendant post-root line
family already paid by the parent. For the first target row this is the possible:

```text
full_line_saving_qdim = 3.
```

If `kappa_phi > 0`, the proof may save only:

```text
charged_postroot_qdim - kappa_phi.
```

Thus the theorem never deletes quotient-line incidence blindly; it replaces an independent line
count by a conditional projective fiber count.

## RFC Compatibility Obligation

For every recurrence row that uses this lemma, the proof must specify:

```text
1. the parent full-line ambient E_A;
2. the descendant ambient E'_A;
3. the transition map phi : E'_A -> E_A;
4. the kernel dimension kappa_phi;
5. root/support compatibility: whether the descendant active support constraints are already
   implied by the carried line, disjoint from it, or incompatible.
```

The row is proof-safe only after these labels are part of the state. The original first row to
verify was:

```text
state: (4,7)>=(2,8)
dominant child-table row:
  outer choice = 3:1:1:1:4:3:3:1:1:1:0:-1:-1:-1:13
  child flag   = (4,5)>=(3,4)
target:
  replace independent outer_tau1_charged_postroot_qdim = 3
  by conditional kappa_phi.
```

The certificate only improves if this row has `kappa_phi < 3`; it fully realizes the diagnostic
saving only if `kappa_phi = 0`.

Update: this particular row is now superseded by the nested tau-zero equal-container filter in
`rfc_nested_tau0_equal_container_filter.md`. Its lower tau-zero sibling has the same child
projection dimension as the upper tau-one row, so exact-support nesting forces the upper active
coordinate to be zero in the upper child container. The row should be rerouted/excluded before any
`kappa_phi` calculation. The full-line carry lemma remains available for later tau-one rows that
survive this exact-support collapse test.
