# RFC Support-Three Component-Plane Bound

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: local theorem target and diagnostic rule. This is not a completed distance certificate.

## Purpose

After the support-two high-lift component-plane bound, the state `(2,15)` is topped by a tau-two
row:

```text
parent span = 2
tau = 2
|A| = 3
delta(A) = 3
comp(A) = 3
kernel dim = 0
child container dim = 4
```

The scalar recurrence pays quotient placement exponent `12` and local charge `6`, leaving a
six-q-dimensional post-root placement budget. The decomposable component structure shows that this
is loose, but only partly closes the current checkpoint by itself.

## Safe Component-Plane Count

Fix the child code and a child container:

```text
V, dim V = 4.
```

For the decomposable support-three row, the exact support decomposition has three rank-one
components. Let `P_i <= V` be the child projection plane associated with component `i`. Since
component `i` is inactive at the two other active coordinates:

```text
P_i <= V cap H_j cap H_k,  {i,j,k}=A.
```

Let the three active coordinate restrictions on `V` be nonzero linear forms
`lambda_1, lambda_2, lambda_3`. Exact support rules out `lambda_i=0`. If two of the restrictions
are proportional, at most one component sees a codimension-one slice; the other two still see
codimension-two slices. Thus, uniformly over the exact-support strata:

```text
one component plane costs at most [3 choose 2]_q <= C_q q^2,
the other two component planes cost O(1).
```

The local exact support two-plane inside the three component directions costs `q^2`. Therefore the
post-root component placement costs at most:

```text
C_q q^4,
```

instead of the scalar `q^6`. This gives a theorem-target saving of two q-dimensions.

## Rank-3 Stratum

If the three active coordinate restrictions on `V` are pairwise independent, then every slice
`V cap H_j cap H_k` has dimension at most two. The component planes are then fixed up to finite
constants after `V` is fixed, and only the local exact-support two-plane contributes `q^2`.

This gives a four-q-dimensional saving on the rank-3 stratum. The current diagnostic exposes this
as a sensitivity mode, but the safe theorem route uses only the two-q-dimensional saving until the
proportional-pair stratum is either charged or carried as extra state.

## Diagnostic Rule

The pair recurrence exposes this as:

```text
--support3-component-plane-mode safe
--support3-component-plane-mode rank3
```

The safe mode subtracts two q-dimensions only for:

```text
parent_span = 2
tau = 2
|A| = 3
delta = 3
comp = 3
K = 0
dim V = 4
```

The rank-3 mode subtracts four q-dimensions for the same row and should be read only as a
stratified sensitivity test.

## Checkpoint

With the previous exact-support filters and support-two high-lift mode, the top state `(2,15)` is:

```text
state (2,15): 1068.39337516 bits
top row:      a=3, tau=2, delta=3, comp=3, child (4,6)
```

Adding the safe support-three mode moves the top row to the adjacent tau-one full-line family:

```text
--support3-component-plane-mode safe

state (2,15): 1057.89054335 bits
top row:      tau=1, a=1, child flag (4,7)>=(2,8)
```

Adding the scalar kernel-lift container cover as well:

```text
--cover-kernel-lift
```

moves the full depth-5 checkpoint to:

```text
final_span_1_crossing_z,102
final_span_1_z_report,34,565.92392782
```

The remaining gap is about:

```text
(565.92392782 + 80) / 128 = 5.04628069
```

q-dimensions. The new top final row is a locally charged tau-one row feeding `(2,15)`, and the
next local/state blocker is the level-3 table:

```text
(4,7)>=(1,8)
```

whose dominant row is an outer tau-zero container together with an inner connected tau-one
quotient-line row. The missing state appears to be a marked child line inside a fixed container,
not another independent support-two or support-three component-plane count.

## Current Root-Kernel-Cover Checkpoint

The later root-kernel container-cover probes changed the active frontier. With:

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

the full demanded depth-5 checkpoint reports:

```text
final_span_1_crossing_z,99
final_span_1_z_report,34,424.70026934
```

The top safe-mode obstruction is now again a support-three row:

```text
level 4, state (2,19):
  p=8, s=3, a=3, tau=2
  delta=3, comp=3
  K=0, dim V=4
  child (4,8)
  term 408.56599456
```

Switching only this local rule to the rank-3 sensitivity mode gives:

```text
final_span_1_crossing_z,72
final_span_1_z_report,34,200.01114103
```

and the dominant path becomes:

```text
level 5: tau-one row into child (2,15), term 200.01072750
level 4: support-three tau-two row into child (3,6), term 301.39238054
level 3: support-two quotient-diamond row into child flag (3,2)>=(1,4), term 26.47911711
```

The child flag `(3,2)>=(1,4)` is already well controlled:

```text
baseline_log2 =   15.71424552
table_log2    = -240.28575448
pair_sum_log2 = -240.28575448
```

So the next local proof obligation is the rank-defect split for the support-three row. A theorem
version must show that the dependent/proportional active-restriction strata are not allowed to pay
the full safe-mode cost. Possible routes:

```text
1. exact-support collapse reroutes proportional restrictions to smaller support;
2. the dependent stratum forces a smaller effective child container than dim V = 4;
3. a marked proportional-pair state carries the defective relation and charges it recursively;
4. the rank defect pays an explicit incidence factor that recovers the two q-dimensions between
   safe and rank-3 mode.
```

Until one of these is proved, `rank3` remains a sensitivity ceiling rather than a certificate rule.
