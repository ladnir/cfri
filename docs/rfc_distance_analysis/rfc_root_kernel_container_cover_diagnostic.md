# RFC Root-Kernel Container Cover Diagnostic

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: diagnostic and proof-target locator, not a completed certificate lemma.

## Purpose

The current depth-5 base-seal recurrence already keeps quotient-line and quotient-plane incidence
visible. The remaining large scalar factors are no longer generic all-lift factors; they occur in
small exact-support rows where, after the child container and root data are fixed, the recurrence
still counts many parent subspaces inside the same root-compatible kernel/container.

This diagnostic adds three opt-in cover probes to `rfc_pair_flag_table_recurrence.py`:

```text
--support2-root-kernel-cover-mode kernel
--support4-root-kernel-cover-mode kernel
--tau1-root-kernel-cover-mode kernel
```

These probes are intentionally narrow. They do not implement the retired all-lift shortcut, and
they do not erase quotient-line or quotient-plane incidence. They only test whether the remaining
post-root container family can be counted once after the quotient/root datum has already been
charged.

## Local Shapes

The support-four probe targets exactly the row exposed after tau-one child-line carrying:

```text
parent span = 2
tau = 2
|A| = 4
delta(A) = 4
comp(A) = 4
K = 0
dim V = 4
```

The scalar recurrence pays the four-q-dimensional family of parent two-planes inside the fixed
root-compatible four-dimensional container. The diagnostic subtracts those four q-dimensions only
for this exact shape.

The support-two probe targets decomposable support-two tau-two quotient-frame rows:

```text
parent span = 2
tau = 2
|A| = 2
delta(A) = 2
comp(A) = 2
K = 0
dim V in {3,4}
```

It subtracts only the positive post-root lift q-dimension left after the existing component-plane
and local root charges have been applied.

The tau-one probe is broader and therefore riskier. It subtracts the saturated post-root quotient
line family:

```text
max(0, 2 dim(V) - parent_span - local_charge)
```

for tau-one rows. This is best read as a full-line carry/root-container sensitivity test. A theorem
version must define the transition map for the full ambient quotient line and prove which fiber
dimensions are already conditioned by the carried state.

## Checkpoints

Baseline sparse proof-shaped run in the current checkout:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --proof-shaped \
  --term-limit 300 \
  --demand-next-level \
  --report-final-z 34
```

reports:

```text
final_span_1_crossing_z,133
final_span_1_z_report,34,1478.66370843
```

With the current best theorem-shaped filters, safe support-three component planes, and the new
root-kernel probes:

```text
--cover-kernel-lift
--nested-quotient-mode inner-in-outer
--nested-subspace-mode inner-in-outer
--consumed-kernel-mode inner-kernel-contained
--support2-diamond-mode child-only
--support2-line-quotient-filter
--exact-filtered-empty
--nested-tau0-equal-container-filter
--support2-component-plane-mode high-lift
--support2-root-kernel-cover-mode kernel
--support3-component-plane-mode safe
--tau1-child-line-carry-mode top
--tau1-root-kernel-cover-mode kernel
--support4-root-kernel-cover-mode kernel
```

the checkpoint is:

```text
final_span_1_crossing_z,99
final_span_1_z_report,34,424.70026934
```

The top final rows are tau-zero container rows feeding child states `(2,19)`, `(2,18)`, and
`(2,17)`. Tracing `(2,19)` shows the remaining safe-mode blocker:

```text
level 4, state (2,19):
  p=8, s=3, a=3, tau=2
  delta=3, comp=3
  K=0, dim V=4
  child (4,8)
  term 408.56599456
```

This is the decomposable support-three tau-two layer-codimension row. The safe support-three bound
saves two q-dimensions, but the row still carries a large local placement cost.

If the support-three diagnostic is upgraded to the rank-3 sensitivity mode, or to the stratified
rank-defect-incidence target:

```text
--support3-component-plane-mode rank3
--support3-component-plane-mode stratified
```

the same root-kernel-cover checkpoint becomes:

```text
final_span_1_crossing_z,72
final_span_1_z_report,34,200.01114103
```

The dominant path then is:

```text
level 5: tau-one row into child (2,15), term 200.01072750
level 4: support-three tau-two row into child (3,6), term 301.39238054
level 3: support-two quotient-diamond row into child flag (3,2)>=(1,4), term 26.47911711
```

The child table for `(3,2)>=(1,4)` is already strong:

```text
baseline_log2 =   15.71424552
table_log2    = -240.28575448
pair_sum_log2 = -240.28575448
```

So this run localizes the remaining gap upstream: the next theorem-grade improvement should target
the support-three tau-two rank stratification or an equivalent local charge, not more lower
pair-table plumbing.

## Proof Interpretation

The current evidence supports the following division:

```text
1. root-kernel container covers are useful for removing duplicate post-root fibers;
2. quotient-line and quotient-plane incidence must remain explicit event data;
3. safe support-three accounting is still too weak;
4. the support-three rank-defect incidence lemma gives the same exponent as the rank-3 sensitivity,
   provided `delta=3` is enforced by exact-support normalization.
```

The most plausible theorem upgrade is to refine `rfc_support_three_component_plane_bound.md`.
After fixing `V`, the three active coordinate restrictions on `V` either have rank three, where
the component planes are fixed up to constants, or they have a rank defect. The rank-defect
incidence lemma charges that defect by the extra hyperplane condition on `V`; see:

```text
docs/rfc_distance_analysis/rfc_support_three_rank_defect_incidence.md
```

Until the root-kernel covers and finite constants are fully imported into the certificate
recurrence, this remains a diagnostic checkpoint rather than a final distance certificate.
