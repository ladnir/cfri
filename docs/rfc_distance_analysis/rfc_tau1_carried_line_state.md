# RFC Tau-One Carried-Line State Target

Scope: original non-systematic RFC, determinant-1 fold, `T` uniform in `F^*`.

Status: proof target and diagnostic interpretation. This is not yet a certificate theorem.

Update: the first dominant child-table row below is now better classified as a nested tau-zero
equal-container active-support collapse, not as a genuine full-line carry row. See
`rfc_nested_tau0_equal_container_filter.md`. The full-line carry lemma remains a valid proof
component for later tau-one chains, but the specific row with outer choice
`3:1:1:1:4:3:3:1:1:1:0:-1:-1:-1:13` and inner tau-zero choice
`3:2:0:0:4:4:5:0:0:0:0:-1:-1:0:0` should be rerouted/excluded by exact-support
canonicalization.

## Purpose

The current strong-mode depth-5 `z=34` blocker is a high-lift tau-one quotient-line family. The
important point is that this is real first-moment mass, not a bad trace choice:

```text
level 4 state (2,15):
rank 1: child (4,7)>=(2,8), local 1030.83289001, child 423.08002115, term 1453.91291117
rank 4: child (3,7)>=(2,8), local  774.83289001, child 420.80863940, term 1195.64152942
```

The cheaper child flag exists, but the span-4 tau-one branch has two extra q-dimensions of local
lift count and dominates the moment. A proof cannot choose the cheaper branch; it must count or
amortize the span-4 tau-one quotient-line family.

## Current Accounting

For a tau-one scalar row, the recurrence fixes a child flag:

```text
L <= V
```

and then counts a projective quotient line datum `R` in the parent visible quotient ambient. The
existing local tau-one lemma counts:

```text
quotient_qdim              = 2 dim(V) - dim(W_parent)
charged_postroot_qdim      = quotient_qdim - local_charge
```

For the level-4 top term:

```text
parent span = 2
child flag  = (4,7)>=(2,8)
tau          = 1
quotient_qdim = 6
charged_postroot_qdim = 5
```

The child table for `(4,7)>=(2,8)` is then dominated by another tau-one outer row:

```text
joint = 423.08002115
outer choice = 3:1:1:1:4:3:3:1:1:1:0:-1:-1:-1:13
child flag = (4,5)>=(3,4)
outer_tau1_quotient_qdim = 4
outer_tau1_charged_postroot_qdim = 3
```

Thus the current recurrence pays for a tau-one quotient-line family at the parent and then pays
again for an apparently related tau-one quotient-line family in the child table.

## Required State

The missing state is not just:

```text
L <= V.
```

It must carry a marked root-compatible quotient line:

```text
R <= E_A(V/L),
```

where `E_A` is the contained-support quotient-line ambient from
`rfc_tau1_quotient_line_incidence_lemma.md`. The carried event should include:

```text
V zero on P union (S \ A),
L zero on P union S,
R root-compatible on A,
```

plus enough compatibility labels to say how `R` projects into the next child diagram.

The parent row still pays for possible `R`; the theorem must not delete quotient-line incidence.
The hoped-for saving is only that descendants conditioned on the same carried `R` should not count
an independent fresh tau-one line family unless the next diagram genuinely introduces one.

## Lemma Target

For a carried profile:

```text
Phi = (L <= V, R, exact support data),
```

prove a one-step recurrence of the form:

```text
Contribution_h(Phi)
  <= Split(Phi)
     * RootIncidence(Phi)
     * ConditionalLift(Phi)
     * F_{h-1}(ChildDiagram(Phi, R)).
```

The proof has to split the image of `R` in the child diagram:

```text
1. R remains visible as the next tau-one quotient line;
2. R lands in an already fixed lower quotient line or plane;
3. R falls into an invisible/kernel fiber and only that fiber dimension is new;
4. R is incompatible with the exact support constraints, giving zero contribution.
```

The diagnostic saving target is the child-table `outer_tau1_charged_postroot_qdim = 3` in the
`(4,7)>=(2,8)` row above. A theorem may save less than three q-dimensions if the inherited line
does not determine the child tau-one line uniquely; the recurrence must expose that conditional
dimension rather than assume it away.

## Conditional Dimension Diagnostic

The expanded `--trace-table-state` columns show that the first target row is more delicate than a
root-visible-line reuse:

```text
state (4,7)>=(2,8), rank-1 row:
outer_tau1_quotient_qdim             = 4
outer_tau1_visible_image_qdim        = 2
outer_tau1_invisible_fiber_qdim      = 3
outer_tau1_universal_postroot_qdim   = 3
outer_tau1_support_saving_qdim       = 0
outer_tau1_charged_postroot_qdim     = 3
outer_tau1_visible_fixed_cond_qdim   = 3
outer_tau1_visible_only_saving_qdim  = 0
outer_tau1_full_line_saving_qdim     = 3
```

Thus the entire post-root budget in this child row is invisible-fiber budget. Carrying only the
root-visible image of the quotient line would leave the same three q-dimensions. In conditional
terms:

```text
condition on visible image only:  conditioned qdim = 3, saving = 0
condition on full ambient line R: conditioned qdim = 0, saving = 3
```

So the required state must carry the full ambient projective line `R <= E_A`, not merely its
projection to the active root coordinates. This is still compatible with the tau-one local lemma:
the parent row already paid for the full line family. The remaining theorem obligation is to show
that the child's dominant tau-one row uses the same carried full line, or else to charge exactly the
new fiber dimension by which it differs.

The parent level-4 row has the same structure:

```text
state (2,15), rank-1 scalar row:
tau1_quotient_qdim                 = 6
tau1_visible_image_qdim            = 2
tau1_invisible_fiber_qdim          = 5
tau1_charged_postroot_qdim         = 5
tau1_visible_fixed_cond_qdim       = 5
tau1_visible_only_saving_qdim      = 0
tau1_full_line_saving_qdim         = 5
```

So the line-carry theorem is not a small local patch. It must say that once the parent pays for the
full ambient line, the relevant descendant rows are conditioned on that same full line, not only on
the same visible root image.

The projective-fiber counting statement is now isolated in:

```text
docs/rfc_distance_analysis/rfc_tau1_full_line_carry_lemma.md
```

That lemma says an independent descendant tau-one line count can be replaced by a conditional count
`q^kappa_phi`, where `kappa_phi` is the kernel dimension of the transition map from the descendant
ambient to the already-carried parent ambient. The first target only improves if `kappa_phi < 3`,
and it realizes the full diagnostic saving only if `kappa_phi = 0`.

## Next Diagnostic

The next useful diagnostic should not globally set tau-one quotient cost to zero. Instead, it
should build a small conditional table for a flagged state with one marked quotient line and report:

```text
unconditioned child row qdim
conditioned-on-R child row qdim
kappa_phi for the transition map
root/support compatibility loss
```

The first target is:

```text
state: (4,7)>=(2,8)
dominant child row: outer tau-one, child flag (4,5)>=(3,4)
budget to explain: outer_tau1_charged_postroot_qdim = 3
needed next: compute/prove kappa_phi for this row
```
