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
