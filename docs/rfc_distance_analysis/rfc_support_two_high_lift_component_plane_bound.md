# RFC Support-Two High-Lift Component-Plane Bound

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: local theorem target and diagnostic rule. This is not a complete distance certificate.

## Purpose

After the nested tau-zero equal-container filter removes the old tau-one child-table obstruction,
the state `(2,15)` is dominated by support-two rows of the form:

```text
parent span = 2
tau = 2
|A| = 2
delta(A) = 2
comp(A) = 2
kernel dim = 0
child container dim = 4
```

These are not the low-lift quotient-diamond rows covered by
`rfc_support_two_tau2_quotient_frame_lemma.md`, where `dim V = dim L + 2`. Here `L=0` and
`dim V=4`: each rank-one support component may span a child 2-plane, so the child projection of
the two-dimensional parent quotient can have dimension four.

The scalar recurrence pays the full quotient-placement exponent:

```text
tau * (2 dim V - dim W) = 2 * (8 - 2) = 12
```

and then the local support-two root charge is `4`, leaving an eight-q-dimensional post-root
placement budget. The component-plane structure should cost only four q-dimensions after the child
container is fixed.

## Lemma Target

Fix a child code and a child container:

```text
V,  dim V = 4.
```

Consider the decomposable support-two tau-two row:

```text
A = {j_1, j_2},
delta(A) = 2,
comp(A) = 2,
K = 0.
```

The exact support decomposition gives two rank-one quotient components. Let `P_i <= V` be the child
projection span of component `i`. For the row above:

```text
dim P_i <= 2.
```

Since component `i` is inactive at the other active coordinate `j_{3-i}`, the child projection
plane satisfies:

```text
P_i <= V cap H_{j_{3-i}},
```

where `H_j` is the child hyperplane of messages zero at coordinate `j`.

If the active support is exact, then `V` is not already zero at `j_{3-i}`. Therefore:

```text
dim(V cap H_{j_{3-i}}) <= 3.
```

The number of possible two-dimensional component planes is bounded by:

```text
[3 choose 2]_q <= C_q q^2
```

for each component, so the two ordered component planes cost at most:

```text
C_q^2 q^4.
```

Thus, after `V` and the two root labels are fixed, the high-lift support-two component placement
contributes at most four q-dimensions, not eight.

## Proof Sketch

Condition on the child code and the child container `V`.

For each support component, take a representative parent quotient vector. Its two child halves span
a subspace `P_i <= V` of dimension at most two. Because the represented quotient support has two
rank-one components, component `i` has zero visible singleton projection at the other active
coordinate. The deterministic child-zero propagation for inactive singleton coordinates puts both
child halves of component `i` in the hyperplane `H_{j_{3-i}}`. Hence:

```text
P_i <= V cap H_{j_{3-i}}.
```

If `V <= H_{j_{3-i}}`, then the whole child container is already zero at an active coordinate, so
the support was not exact and the row is rerouted by exact-support canonicalization. In the exact
row, `V cap H_{j_{3-i}}` has codimension at least one inside `V`, so its dimension is at most
three.

Counting each ordered component plane independently inside its three-dimensional slice gives the
safe upper bound:

```text
[3 choose 2]_q^2 <= C_q^2 q^4.
```

This bound intentionally ignores further constraints: the two component planes must together span
`V` for the displayed high-lift row, and the root labels determine the parent quotient plane up to
finite projective constants. Ignoring those constraints only overcounts.

## Diagnostic Rule

The pair recurrence exposes this as:

```text
--support2-component-plane-mode high-lift
```

The mode is deliberately narrow. It subtracts four q-dimensions only when:

```text
parent_span = 2
tau = 2
|A| = 2
delta = 2
comp = 2
K = 0
dim V = 4
```

No other tau-two row is changed.

## Checkpoint

With the previous exact-support filters plus this mode:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --proof-shaped \
  --nested-quotient-mode inner-in-outer \
  --nested-subspace-mode inner-in-outer \
  --consumed-kernel-mode inner-kernel-contained \
  --support2-diamond-mode child-only \
  --support2-line-quotient-filter \
  --exact-filtered-empty \
  --nested-tau0-equal-container-filter \
  --support2-component-plane-mode high-lift \
  --term-limit 300 \
  --demand-next-level \
  --full-table-until 2 \
  --report-final-z 34 \
  --trace-state-terms 4,2,15 \
  --trace-state-top 12
```

the current checkout reports:

```text
final_span_1_crossing_z,130
final_span_1_z_report,34,967.01172212
```

At state `(2,15)`, the former support-two frame rows move below the top. The new dominant row is:

```text
p = 6
s = 3
a = 3
tau = 2
delta = 3
comp = 3
theta = 2
child flag = (4,6)
```

So this component-plane bound is a real improvement, but the certificate is still open. The next
blocker is now the `a=3, delta=3, comp=3` tau-two layer-codimension row.
