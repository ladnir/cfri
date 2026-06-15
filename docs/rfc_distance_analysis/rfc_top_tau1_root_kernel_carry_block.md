# RFC Top Tau-One Root-Kernel Carry Block

Scope: original non-systematic RFC, determinant-1 fold, `T` uniform in `F^*`.

Status: theorem contract plus audit correction for the current top-row blocker. This is not yet a
completed distance certificate.

## Purpose

The block-credit potential probe says the next missing reusable block is the top tau-one/root-kernel
feed:

```text
top_tau1_to_2_15
```

With only the support-three incidence credit imported, the simple potential tightens at this row:

```text
level_weight = 3.00795584 qdims
```

If the top row is granted the two current target credits:

```text
tau1_full_line_carry = 1 qdim
kernel_fiber_cover   = 1 qdim
```

then the top row has positive slack in the same probe and the bottleneck returns to the
support-three incidence row. The proof obligation is therefore narrow: make those two q-dimensions
legal without deleting quotient-line incidence that has not actually been conditioned.

Audit update: `rfc_tau1_carry_kappa_audit.py` shows that the immediate `top_tau1_to_2_15` edge has
no descendant tau-one line and its parent tau-one row has:

```text
charged_postroot_qdim = -1.
kernel_dim = 0.
kernel_lift_qdim = 0.
```

So neither `tau1_full_line_carry` nor `kernel_fiber_cover` is a legal credit for this edge. The
audited potential profile `audited-top-kernel` removes both top credits; the top row remains tight
at `level_weight = 3.00795584`.

## Objects

Fix one tau-one scalar transition after the child flag and exact support have been selected.
The row contains:

```text
L <= V                         child flag
K                              child-visible kernel, dim K = parent_span - 1
Q = V / L                      child quotient
E_A                            contained-support quotient-line ambient
R <= E_A                       full parent quotient line
A                              active singleton support
rho : E_A -> F^A               root-visible projection
```

The tau-one incidence lemma pays for the possible full projective lines `R <= E_A` together with
the active root equations. The carry block is allowed to condition descendants on this already paid
full line. It is not allowed to condition only on `rho(R)`, because the live diagnostics show that
visible-only conditioning can save zero q-dimensions.

For any descendant tau-one row that claims to reuse the parent line, the state must include a
linear transition map:

```text
phi : E'_B -> E_A
```

from the descendant contained-support quotient-line ambient to the already paid parent ambient.
The conditional line count is then:

```text
# { R' <= E'_B : phi(R') = R } = q^dim ker(phi)
```

provided the preimage of `R` is nonempty. This is exactly the projective-fiber lemma in
`rfc_tau1_full_line_carry_lemma.md`.

## Safe Carry Rule

A recurrence row may replace an independent descendant tau-one line count by a carried-line count
only if it records the tuple:

```text
(R, E_A, E'_B, phi, kappa_phi, compatibility)
```

where:

```text
kappa_phi = dim ker(phi)
```

and `compatibility` says one of:

```text
same-line:       phi(R') = R, pay kappa_phi;
lower-contained: R' is already a lower marked line/plane, pay no fresh line count;
invisible-fiber: pay the stated kernel fiber dimension;
incompatible:    the row contributes zero.
```

If the unconditioned descendant row has charged post-root exponent `c_child`, the legal saving is:

```text
max(0, c_child - kappa_phi)
```

not automatically `c_child`. The diagnostic `tau1_full_line_saving_qdim` is theorem-grade only
when `kappa_phi = 0` or when the recurrence explicitly budgets the remaining `kappa_phi`.

## Root-Kernel Cover Rule

After `R` and the active roots are fixed, the singleton equations cut a root-compatible affine
kernel/container inside the quotient lift ambient. The scalar recurrence often counts many parent
subspaces inside that same container even though they induce the same child diagram and the same
root-line event.

The root-kernel cover may be spent only for the unconsumed part of that fiber. Formally, for a
fixed child diagram and fixed `R`, partition hidden directions into:

```text
consumed:   carried as a lower kernel/flag node in the child diagram;
unconsumed: not visible to any descendant event except through the fixed root-compatible container.
```

The recurrence may count the unconsumed container once and remove only the Gaussian multiplier for
choosing parent subspaces inside it. If a descendant event later uses the same hidden directions,
they must be represented by a consumed-kernel node; otherwise the same fiber has been charged twice.

This is the theorem version of the diagnostic `kernel_fiber_cover` block. It does not apply to the
immediate top edge because that row has no kernel lift. A carried-line or kernel-fiber credit can
only be used on a later row with the corresponding line map or positive unconsumed kernel lift.

## Top-Row Eligibility Test

Before the certificate can spend the two top-row q-dimensions, the row must pass all five checks:

```text
1. Full line recorded:
   the state carries R <= E_A, not only rho(R).

2. Transition map recorded:
   every descendant tau-one line using the carry has an explicit phi : E'_B -> E_A.

3. Conditional dimension bounded:
   the recurrence spends c_child - kappa_phi, with kappa_phi computed or upper-bounded.

4. Kernel fiber partitioned:
   every hidden direction is marked consumed or unconsumed.

5. No broad tau-one deletion:
   the root-kernel cover applies only to the eligible saturated row, not to arbitrary tau-one
   quotient-line incidence.
```

The current implementation diagnostics intentionally violate item 5 in the broad
`--tau1-root-kernel-cover-mode kernel` switch: it subtracts the saturated post-root quotient-line
family for all tau-one rows. That switch is a locator. The theorem-grade replacement should attach
an eligibility flag to the row and subtract only:

```text
carried_line_saving + unconsumed_kernel_fiber_saving.
```

## Proof Sketch

First expose the full quotient line `R` as event data and pay for it using the tau-one
quotient-line incidence lemma. This keeps all root probabilities and finite projective constants
where they belong.

Second, condition descendant tau-one rows on the same full line. The projective-fiber lemma gives
the exact conditional count `q^kappa_phi`, so no independence or product-of-child-moments argument
is used.

Third, after `(child diagram, R, roots)` are fixed, quotient by the consumed-kernel nodes. The
remaining unconsumed parent lifts lie in one root-compatible container and do not change any
descendant event. Count that container once. The only finite factors left are Gaussian/projective
constants for the fixed dimensions, which are polynomial in the local diagram size and can be
charged in the final finite-constant budget.

## Current Assessment

This is useful but negative for the current top edge. The good news is structural: the audit
prevented us from spending a fictional credit. The bad news is precise: neither full-line carry nor
kernel-fiber cover applies to `top_tau1_to_2_15`. The general block-ledger route now needs a new
top-row theorem, or this boundary transition should be handled row-by-row.

## Next Implementation Step

Add a narrow diagnostic mode that does not use the broad `--tau1-root-kernel-cover-mode kernel`.
For the top row, the audit now emits:

```text
charged_postroot_qdim
line_carry_applicable
kernel_dim
kernel_lift_qdim
unconsumed_kernel_fiber_qdim
legal_total_saving_qdim
```

For later tau-one rows that really do have a descendant line edge, also emit `transition_ambient_id`
and `kappa_phi`. Then rerun the block potential with measured legal credits rather than the old
`current-target` ceiling profile.
