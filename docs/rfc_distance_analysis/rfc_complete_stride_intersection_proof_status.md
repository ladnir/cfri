# RFC Complete-Stride Intersection Proof Status

Scope: original non-systematic RFC only.

This note records the proof meaning of the clean two-copy complete-stride intersection gate over
`GF(65537)`.

## Current Gate Result

Implementation reports the two-copy complete-stride gate as clean over:

```text
field = GF(65537)
copies = 2 independent RFC copies
target = depth-4 complete-stride flags
```

Interpreted under the contract in:

```text
docs/rfc_distance_analysis/rfc_complete_stride_intersection_proof_contract.md
```

this means:

```text
each copy has the expected 24 unmarked complete-stride supports;
each copy has the expected 48 ordered canonical flags;
the two-copy complete-stride flag intersections have no positive excess:
  max_v_excess = 0,
  max_l_excess = 0;
there is no real_extra_intersection row.
```

The generic dimensions being tested are:

```text
generic_v_dim = 0 for V_1 cap V_2, with dim V_i = 2;
generic_l_dim = 0 for L_1 cap L_2, with dim L_i = 1.
```

So the clean gate supports the claim that complete-stride flags from two independent copies
intersect generically at this depth and field size.

## What This Supports

The result supports the current two-layer flag recurrence in one specific place:

```text
active-copy complete-stride intersections for r = 2
do not require an additional shared-stride or shared-flag recurrence state.
```

It also supports the accounting separation:

```text
one-copy complete-stride flags are counted by exact support/canonical flag keys;
two-copy intersections are handled by the generic flag-intersection exponent;
duplicate stride/core certificates are not extra events.
```

This is evidence against the main complete-stride nested-kernel falsification route at `r=2`.

## What It Does Not Prove

The clean two-copy gate does not prove the full distance certificate.

It does not prove:

```text
1. the tau-two endpoint local theorem;
2. the full recursive two-layer flag moment F_h(r1,z1;r0,z0);
3. generic intersection for r >= 3 active independent copies;
4. generic intersection for arbitrary non-complete-stride exact flags;
5. production-depth behavior beyond the tested depth-4 complete-stride gate;
6. that all finite-field and split-count constants fit inside the e=71 slack.
```

It is also not a substitute for the parent lift bound:

```text
Lift(t=2,tau=1,r0=1,r1=2) <= Gamma_q^2 q^3.
```

The lift factor still belongs in the one-step recurrence. The intersection gate only checks that
the child flag objects produced by independent copies do not share an extra reusable dimension
beyond the generic intersection model.

## Remaining Obligation For r >= 3

The next proof obligation is a multi-copy version of the generic intersection claim.

For `r` independent copies, each complete-stride flag contributes:

```text
L_i <= V_i,
dim L_i = 1,
dim V_i = 2.
```

In ambient dimension `m`, the generic expected common dimensions are:

```text
generic_common_v_dim(r,m) = max(0, 2r - (r-1)m),
generic_common_l_dim(r,m) = max(0, r - (r-1)m).
```

For the relevant depth-4 ambient choices:

```text
m = 16 full ambient:
  generic_common_v_dim(r,16) = 0 for all r >= 2,
  generic_common_l_dim(r,16) = 0 for all r >= 2.

m = 4 conditioned row-block ambient:
  generic_common_v_dim(r,4) = 0 for r = 2,
  generic_common_v_dim(r,4) = max(0, 4-r) for r >= 3 if all V_i are forced into the same B_b;
  generic_common_l_dim(r,4) = 0 for all r >= 2.
```

The implementation and proof must therefore report which ambient convention is being used. A
claimed excess is only meaningful after this choice is explicit.

The `r >= 3` acceptance criterion should be:

```text
observed_common_v_dim <= generic_common_v_dim(r,m),
observed_common_l_dim <= generic_common_l_dim(r,m),
after canonical flag de-duplication and correct ambient selection.
```

If any large-prime or generic-symbolic `r >= 3` row has positive excess after this normalization,
the recurrence needs a new active-copy intersection state.

## Effect On The Two-Layer Flag Recurrence

The clean two-copy gate does not change the recurrence form.

Keep:

```text
B_h(t,z)
  <= sum Split * Local * Lift(t,tau,r0,r1)
       * F_{h-1}(r1,z_V;r0,z_L).
```

Keep the existing complete-stride one-copy parameters:

```text
t = 2,
tau = 1,
kappa = 1,
r1 = 2,
r0 = 1,
Lift exponent = 3.
```

The update is narrower:

```text
for r=2 complete-stride active-copy intersections,
no additional shared-complete-stride state is currently needed.
```

The recurrence still needs active-copy profile fields for audit:

```text
r_active,
ambient_dim,
support/flag profile,
observed_common_v_dim,
observed_common_l_dim,
generic_common_v_dim,
generic_common_l_dim.
```

## Fallback Trigger

The `e=72` fallback is not triggered by the clean two-copy gate.

It would be triggered or re-evaluated if:

```text
1. r >= 3 complete-stride intersections show positive large-prime/generic excess;
2. arbitrary exact flags show positive generic-intersection excess;
3. the rigorous recurrence closes e=71 with insufficient constant slack;
4. a real extra q-dimension persists after duplicate-certificate and ambient-mismatch checks.
```

Current status:

```text
r=2 complete-stride intersection gate: clean;
r>=3 complete-stride intersection theorem: still open;
arbitrary independent-copy flag intersection theorem: still open.
```
