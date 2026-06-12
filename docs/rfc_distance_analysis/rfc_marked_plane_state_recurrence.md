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
