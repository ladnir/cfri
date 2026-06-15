# RFC Tau-One Child-Line Carry Diagnostic

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: diagnostic and proof-target locator. This is not a theorem-grade recurrence rule.

## Purpose

After support-two/support-three component-plane savings and scalar kernel-lift container covering,
the best depth-5 trace was:

```text
final_span_1_z_report,34,565.92392782
```

The top row was a tau-one parent row feeding child state `(2,15)`. That parent row already pays for
a full quotient line, while the child state then pays another independent tau-one quotient-line
family. The diagnostic here tests the ceiling of a marked-line state that carries the parent full
line into the selected child scalar tau-one row.

## Diagnostic Rule

The pair recurrence exposes this as:

```text
--tau1-child-line-carry-mode top
```

For a tau-one parent row, if the selected child scalar row is also tau-one, the diagnostic subtracts
the positive child post-root quotient-line q-dimension:

```text
max(0, child_tau1_charged_postroot_qdim).
```

It does not delete quotient incidence globally. It is a proxy for a future marked-line/container
state in which the child tau-one line is conditioned on the full line already selected by the
parent row. The mode is deliberately optimistic because it uses only the selected child scalar row;
a theorem must replace this by an explicit state and transition map.

## Checkpoint

With the current best structural modes:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --proof-shaped \
  --cover-kernel-lift \
  --nested-quotient-mode inner-in-outer \
  --nested-subspace-mode inner-in-outer \
  --consumed-kernel-mode inner-kernel-contained \
  --support2-diamond-mode child-only \
  --support2-line-quotient-filter \
  --exact-filtered-empty \
  --nested-tau0-equal-container-filter \
  --support2-component-plane-mode high-lift \
  --support3-component-plane-mode safe \
  --tau1-child-line-carry-mode top \
  --term-limit 300 \
  --demand-next-level \
  --full-table-until 2 \
  --report-final-z 34
```

the current checkout reports:

```text
final_span_1_crossing_z,102
final_span_1_z_report,34,435.05329836
```

The gap is now:

```text
(435.05329836 + 80) / 128 = 4.02385389
```

q-dimensions above the `2^-80` target.

## New Dominant Shape

The top final rows are no longer tau-one rows. They are tau-zero container rows feeding child
states around `(2,20)`:

```text
level 5, state (1,34), top row:
  tau = 0
  child = (2,20)
  term = 434.53619596
```

Tracing `(2,20)` shows the new top row:

```text
level 4, state (2,20), top row:
  p = 8
  s = 4
  a = 4
  tau = 2
  delta = 4
  comp = 4
  charge = 8
  lift_qdim = 12
  child = (4,8)
  local = 534.48656009
  child = -121.19264508
  term = 413.29391501
```

This is the decomposable support-four exterior row. Unlike the earlier support-two and
support-three high-lift rows, the remaining four q-dimensions are plausibly the true local family:
after fixing a child 4-container and four independent active restrictions, the four component
directions give a four-dimensional component space, and choosing the parent tau-two plane costs
`[4 choose 2]_q`, i.e. `q^4`.

So the next proof step is not another tau-one line carry. It is a support-four / marked-component
state question: either the child event `(4,8)` must be strengthened by the four component-line
structure it induces, or this row is evidence that the current base-seal target needs a different
global argument.
