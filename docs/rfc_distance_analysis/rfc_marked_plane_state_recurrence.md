# RFC Marked-Plane State Recurrence

Scope: original non-systematic RFC. This note refines the local
`rfc_two_marked_line_plane_lemma.md` into a candidate finite recurrence state.

Status: proof target and diagnostic contract, not a complete distance certificate.

## Motivation

The corrected depth-5 safe-tau-zero trace exposes a child flag that is not a chain after one more
expansion. The carried flag

```text
F_3((4,4),(2,5))
```

expands into a child two-plane with two child lines:

```text
      P
     / \
 M(4) N(3)
```

The old chain relaxation pays for the lower line through an ambient ancestor factor. Once `P` is
already fixed, the lower line has at most `q+1` choices inside `P`. The current diagnostic saving is
about three q-dimensions for this one diagram, but the displayed top path still has a residual gap
of `15.18484798` q-dimensions. Therefore the real question is whether this local rule can be made
into a recursive state, not whether the single hand-expanded diagram closes the proof.

## State

For a child depth `h`, define a marked-plane state

```text
G_h(z_P; z_1, ..., z_m)
```

to count ordered tuples

```text
P, L_1, ..., L_m
```

such that:

```text
dim P = 2,
L_i <= P,
dim L_i = 1,
P has at least z_P common zeros,
L_i has at least z_i common zeros.
```

The tuple is ordered and line coincidences are allowed. This is deliberate: it overcounts, so it is
safe for a first-moment upper bound. If two requested lines are equal, the ordered model counts the
same event multiple times instead of missing it.

The minimal carrier inequality is:

```text
G_h(z_P; z_1, ..., z_m)
  <= min_i (q+1)^(m-1) F_h((2,z_P),(1,z_i)).
```

This ignores the zero requirements on the other `m-1` lines after choosing them inside `P`; ignoring
constraints only enlarges the event. Later refinements can use the extra line zero budgets, but the
safe base rule needs only the carrier line.

## Transition Pattern

Suppose a two-layer flag state

```text
V_0 >= V_1
```

is being bounded at one fold, and the selected one-step rows have:

```text
V_0 row: child plane P plus marked line M <= P,
V_1 row: tau = 0, child line N <= P.
```

The containment `N <= P` follows from `V_1 <= V_0` and functoriality of child projection. No
containment between `N` and `M` is implied, so the correct child object is the marked-plane diagram,
not a chain.

The safe replacement is:

```text
coarse child flag bound
  -> value of the V_0 carrier state
     + non-recursive local row cost for V_1
     + log2(q+1).
```

The current scripts use `log2(q)` for the q-dimensional part and leave the finite `+1` constant to
the finite-constant bucket. For `q=2^128`, this distinction is negligible for diagnostics but must
be accounted for in the final certificate constants.

## Compatibility Checks

This state is compatible with the determinant-1 RFC fold with `T` uniform nonzero:

```text
1. The line-count step is deterministic after conditioning on the child code and the carrier
   certificate.
2. It does not use the unavailable `T'=-T` symmetry, so it is binary-field compatible.
3. Nonzero-root normalization remains in the local row costs, not in the marked-plane line count.
4. Ordered tuples and allowed coincidences prevent undercounting.
```

The main proof hazard is not local algebra; it is state sufficiency. After several folds, the child
object can become a small inclusion diagram with multiple planes and marked lines. Collapsing that
diagram back to one chain loses exactly the information this note is trying to preserve.

The equality rule must also be explicit. If two marked line nodes in the same plane turn out to be
the same line, the diagram state merges them and keeps the strongest zero budget:

```text
L(z_a), L(z_b) -> L(max(z_a,z_b)).
```

Before equality is known, ordered duplicate nodes are safe because they overcount. After equality is
forced by containment plus equal dimension, failing to merge would incorrectly treat one line as two
independent directions in later folds.

## Diagnostic Contract

`scripts/rfc_distance_analysis/rfc_diagram_state.py` implements the diagnostic state skeleton:

```text
node: dim, zero budget
edge: child <= parent
canonicalization: equal-dimension containment forces node merge
line insertion: add a marked line under a fixed 2-plane for q+1 choices
transition builder: derive the child diagram of a two-layer flag transition
```

`scripts/rfc_distance_analysis/rfc_marked_plane_state_diagnostic.py` scans the current two-layer
flag recurrence for rows matching the transition pattern above and emits the carrier and successor
diagram keys. The exact carried-path row is:

```text
python -B scripts/rfc_distance_analysis/rfc_marked_plane_state_diagnostic.py \
  --layer-level 2 \
  --outer-state 4,4 \
  --inner-state 2,5
```

It reproduces:

```text
coarse bound:       -743.04099425 bits
marked-plane bound: -1124.45603175 bits
saving:              381.41503750 bits
carrier diagram:     M:d1:z4;P:d2:z0|M<=P
next diagram:        M:d1:z4;N:d1:z3;P:d2:z0|M<=P;N<=P
transition diagram:  I0:d1:z3;O0:d2:z0;O1:d1:z4|I0<=O0;O1<=O0
```

This agrees with `rfc_carried_flag_diagnostic.py` and makes the next recurrence requirement
explicit: replace special-case hand carrying by a finite diagram state that can propagate multiple
marked lines through recursive folds.

The scanner also has a grouping mode:

```text
python -B scripts/rfc_distance_analysis/rfc_marked_plane_state_diagnostic.py \
  --only-positive \
  --group-by-transition-diagram
```

On the current depth-5 defaults, the top positive groups are all of the form:

```text
I0:d1:z_a;O0:d2:z0;O1:d1:z_b|I0<=O0;O1<=O0
```

and the best rows save exactly `3.00000000` q-dimensions before finite constants. This is useful
evidence that the diagram recurrence has a small repeated state family, not just a single isolated
repair.

`scripts/rfc_distance_analysis/rfc_diagram_path_dp.py` follows the corrected bound trace and applies
the carried-flag and marked-plane transitions when they are exposed. On the default depth-5 `z=34`
path it recovers the known local accounting:

```text
carry merge saving:       251.97763219 bits
marked-plane saving:      381.41503750 bits
combined saving:          633.39266969 bits
remaining residual:        15.18484798 q-dimensions
```

As a separate scalar-recursive diagnostic, `rfc_flag_span_moment.py --flag-bound best-marked-plane`
allows the recurrence to use the q+1 marked-plane child replacement wherever the relevant child
choices already exist. This mode is selected on the depth-5 `z=34` trace (`dominant_h=-5`) and
improves the top vector moment to:

```text
2244.71357608 bits
```

but the crossing remains:

```text
crossing_z = 135.
```

This is a key negative signal. The q+1 brick is valid and useful, but a scalar child-flag tweak
cannot reproduce the full carried-path saving because it does not preserve the outer layer across an
inner-first collapse. The next recurrence must carry the multi-layer diagram state itself.

The next diagnostic table is:

```text
rfc_flag_span_moment.py --flag-bound best-two-layer-table
```

It builds a two-layer flag table after each scalar level and lets parent scalar rows query that
table for child flags. This is closer to the desired recurrence, but it still uses scalar-state
dominant choices when expanding a flag state. The result is another useful negative:

```text
depth 5, z=34 top vector moment: 2244.71357608 bits
depth 5 crossing:                z=135
depth 6 crossing:                z=305
```

The table report makes the failure mode visible:

```text
level 2, (4,4)>=(2,5): table saves 381.41503750 bits
level 3, (4,7)>=(2,8): table saves   0.00000000 bits
```

So a value table alone is not the missing recurrence. The recurrence must choose and remember
flag-state expansions, not just reuse scalar-state dominant choices.

`scripts/rfc_distance_analysis/rfc_flag_state_choice_diagnostic.py` then tests exactly that next
idea for one target flag at a time. It enumerates outer and inner scalar expansion candidates,
combines them into a carried child flag, and reports both:

```text
pair_sum:         log-sum over the enumerated pair products
optimistic_best:  minimum enumerated pair
dominant_i:       largest enumerated pair contributions
```

The signal splits by level:

```text
level 2, (4,4)>=(2,5):
  pair_sum saves 506.75207249 bits = 3.95900057 q-dim

level 3, (4,7)>=(2,8):
  pair_sum loses 907.21115036 bits = 7.08758711 q-dim
  optimistic_best saves thousands of bits, but is not safe by itself
```

This sharpens the blocker. A flag state choosing its own rows is still not enough if all row-pair
witnesses are summed naively. The next proof step must supply a canonical witness selection,
exact-support grouping, or charging lemma that removes the high-mass duplicate/incompatible pair
family. Without such a lemma, the marked-plane route does not close the level-3 carried flag.

The next classifier is:

```text
scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py
```

It groups the same pair products by child flag, outer choice, inner choice, support profile, tau
profile, and lift profile. The saved diagnostics are:

```text
docs/rfc_distance_analysis/rfc_flag_bad_pair_classifier_level2_4_4_ge_2_5.csv
docs/rfc_distance_analysis/rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8.csv
docs/rfc_distance_analysis/rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8_kernel_cover.csv
```

The important level-3 output is concentration, not improvement:

```text
level 3, (4,7)>=(2,8):
  truncated pair sum:          2094.40570138 bits
  coarse/table baseline:       1187.19455102 bits
  naive loss:                  907.21115036 bits = 7.08758711 q-dim
  top 12 pair products:        2094.40570138 bits
  dominant outer-choice group: 2094.40570138 bits
```

The dominant outer choice is:

```text
p=3, s=1, a=1, tau=1,
child=(4,4), z=3,
charge=1, delta=1, comp=1,
lift=19.
```

The classifier now prints the lift split for each top grouped row. For this row:

```text
outer kernel lift:   15 q-dim
outer quotient lift:  4 q-dim
```

So the bad mass is not a broad failure of the marked-plane local brick. It is a high-lift tau-one
outer witness family that admits many inner refinements whose child diagrams collapse back to
`(4,4)` or a short two-layer flag. A proof now has a concrete target: either choose this witness
canonically once per parent flag, or prove that the high-lift multiplicity is already charged by
the exact zero/support data and should not be summed independently across these refinements.

Two finer diagnostics sharpen that target:

```text
--exclude-collapsed-active:
  pair sum = 1710.08786604 bits
  remaining loss = 522.89331502 bits = 4.08510402 q-dim

--posthoc-cover-kernel-lift:
  pair sum = 940.85227227 bits
  saving over coarse = 246.34227875 bits = 1.92454905 q-dim
```

The old `--posthoc-cover-kernel-lift` flag is now also exposed as:

```text
--kernel-cover-mode unconsumed-container
```

which records the intended theorem guard: quotient incidence stays counted, and only duplicate
kernel lifts are covered under the unconsumed-kernel condition.

The classifier now also emits:

```text
top_outer_kernel_dim
top_inner_kernel_dim
top_outer_kernel_unconsumed_by_inner
```

For the dominant un-covered pair, these are:

```text
top_outer_kernel_lift_qdim = 15
top_outer_kernel_dim = 3
top_inner_kernel_dim = 0
top_outer_kernel_unconsumed_by_inner = yes
```

The next two un-covered pairs have `top_inner_kernel_dim = 1` and are marked `no`, which is the
expected consumed-kernel warning. After `--kernel-cover-mode unconsumed-container`, the top covered
pairs again have `top_inner_kernel_dim = 0` and are marked `yes`. This does not prove descendant
unconsumption, but it cleanly separates the easy sibling-unconsumed family from rows that must be
carried or charged.

The stricter diagnostic is:

```text
--kernel-cover-mode sibling-unconsumed
```

It covers only an upper-layer kernel lift when the displayed lower layer is fully visible; it does
not cover the lower layer's own kernel lift because that requires a descendant audit. By itself it
does not close the row:

```text
pair sum = 2092.28967759 bits
remaining loss = 905.09512656 bits = 7.07105568 q-dim
top row has top_inner_kernel_dim = 1 and is marked no
```

However, combining the exact-flag collapse routing with the sibling-unconsumed cover gives:

```text
--exclude-collapsed-active --kernel-cover-mode sibling-unconsumed:
  pair sum = 1071.43723477 bits
  saving over coarse = 115.75731625 bits = 0.90435403 q-dim
```

This is the current proof-shaped closure of the level-3 stress row. It uses two separate
statements: collapsed-active rows are rerouted as exact-flag support collapses, and only
sibling-unconsumed kernel fibers are covered. Consumed-kernel rows remain in the sum, but after the
collapsed-active rerouting they are below the coarse/table baseline.

The first option removes rows where a tau-positive exact flag would have equal-dimensional child
containers and the lower container carries the active singleton zero. That alone removes the
`lift=19` row, but leaves a `lift=13` tau-one row. The second option keeps the child table fixed and
subtracts only the kernel-lift part of tau-positive rows:

```text
kappa(2 r_0 - kappa),  kappa = t - tau.
```

This closes the level-3 stress row with about `1.92` q-dimensions of slack. The current best proof
target is therefore not to erase quotient-line or quotient-plane incidence. It is to prove a
kernel-lift container-cover lemma: after fixing the child flag and the local quotient/root datum,
the Gaussian family of kernel lifts is duplicate certificate data for this existence recurrence.

## Audit Result

A side audit agreed that the local rule is promising provided it is integrated as an atomic diagram
transition:

```text
condition on the child code and already-carried child diagram,
sum actual child diagrams once,
multiply by explicit current-fold split/root/lift profile constants.
```

The audit's main warning is the same as above: no product of child moments, no scalar post-hoc
discount, and no collapse of incomparable marked lines into a chain. The next proof step is to turn
the marked-plane state into a small diagram recurrence with merge/equality rules.
