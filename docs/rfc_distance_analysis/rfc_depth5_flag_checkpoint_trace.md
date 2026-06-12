# RFC Depth-5 Flag Checkpoint Trace

Scope: original non-systematic RFC, depth-5 base-seal diagnostics.

Status: diagnostic trace after correcting the cover-lift interpretation.

## Main Correction

The all-cover diagnostic:

```text
--cover-lift-mode all
```

is anti-conservative as a theorem. It removes tau-positive quotient-line/quotient-plane incidence.
The valid proof target is:

```text
count quotient incidence sharply,
then avoid duplicate extension multiplicity after quotient data are fixed.
```

Tau-zero duplicate lift covering is safe, but it does not close the depth-5 base checkpoint.

## No-Cover z=34 Trace

Using:

```text
endpoint-tau2-layer-incidence,
flag_bound=best,
max_visible_tau=2,
prune_to_final_span=1,
cover_lift_mode=none.
```

the `z=34` vector moment is:

```text
log2 vector moment = 2609.77218856.
```

Best trace:

```text
level span z   log2_state      p   s   a   tau outer inner outer_z inner_z charge lift_qdim
5     1    34  2481.77218856   14  6   0   0   2     2     20      20      0      3
4     2    20  2075.90011206   10  0   0   0   4     4     10      10      0      12
3     4    10  539.90011206    5   0   0   0   4     4     5       5       0      16
2     4    5   -1508.09988794  0   5   5   2   2     1     0       5       8      0
1     2    0   0.00000000      0   0   0   0   1     1     0       0       0      0
```

This trace is dominated by tau-zero/all-paired duplicate lift multiplicity at the upper levels.

## Safe Tau-Zero Cover z=34 Trace

Using:

```text
--cover-lift-mode tau0
```

the `z=34` vector moment is still:

```text
log2 vector moment = 2497.05321083.
```

Best bound-following trace:

```text
level span z   log2_state      p   s   a   tau outer inner outer_z inner_z charge delta lift_qdim child_bound
5     1    34  2369.05321083   15  4   4   1   2     0     15      19      4      1     3         outer-first
4     2    15  2470.43486387   7   1   1   1   4     2     7       8       1      1     9         inner-first
3     2    8   415.60197386    3   2   2   1   4     2     3       5       2      1     9         outer-first
2     4    3   -1003.06336206  0   3   3   2   2     1     0       3       6      2     0         inner-first
1     1    3   -247.19264508   0   3   3   1   1     0     0       3       3      1     1         outer-first
```

After removing the safe tau-zero duplicate lifts, the dominant path moves to tau-one
quotient-incidence rows with a tau-two boundary row at level two. These tau-positive quotient
choices are real event data; they cannot be zeroed the way the anti-conservative all-cover
diagnostic does.

The tau-one incidence report:

```text
--report-tau1-incidence
```

shows that the tau-one rows on this `z=34` trace have no hidden support-subcode saving:

```text
level  a  delta comp quotient_qdim universal_postroot support_saving charged_postroot
5      4  1     1    3             -1                 0              -1
4      1  1     1    6              5                 0               5
3      2  1     1    6              4                 0               4
1      3  1     1    1             -2                 0              -2
```

So the formal tau-one quotient-line lemma is needed for proof safety, but it is not the local
improvement that closes this checkpoint.

The same trace now reports which coarse child-flag relaxation is selected:

```text
level  child_bound_choice  child_bound_log2   outer_first_log2  inner_first_log2
5      outer-first          2470.43486387      2470.43486387     3584.00000000
4      inner-first          1439.60197386      1566.81698675     1439.60197386
3      outer-first          -491.06336206      -491.06336206     -244.60768258
2      inner-first          -247.19264508       128.00000000     -247.19264508
1      outer-first             0.00000000         0.00000000        0.00000000
```

After additionally removing duplicate kernel lifts, the upper trace switches to outer-first rows
but the crossing remains `z=135`. Thus neither the tau-one local count nor a single kernel-lift
factor is the missing lever. The next target is a true recursive joint child flag/diagram state.

## Carried-Flag Merge Check

The targeted diagnostic:

```text
python scripts/rfc_distance_analysis/rfc_carried_flag_diagnostic.py
```

reconstructs the corrected safe-tau-zero trace and carries the first inner-first flag:

```text
F_3((4,7),(2,8)).
```

The current one-layer collapse follows the inner layer into:

```text
(4,3) >= (2,5).
```

Carrying the outer layer's own child kernel zero budget merges the equal four-dimensional projected
layers and strengthens the child flag to:

```text
(4,4) >= (2,5).
```

Under the current coarse child-bound evaluator this saves:

```text
251.97763219 bits.
```

This is real but not enough by itself. The next expansion of `(4,4) >= (2,5)` exposes a non-chain
diagram:

```text
2-plane with marked lines (1,4) and (1,3).
```

The two marked lines both lie in the same child 2-plane, but they are not known comparable or equal.
So the next proof object is a small incidence diagram, not just a longer total chain.

The local two-marked-line plane bound in:

```text
rfc_two_marked_line_plane_lemma.md
```

replaces the coarse `q^4` ancestor choice for the second line by a `q+1` line choice inside the
fixed child plane, plus finite split constants. The diagnostic estimate is:

```text
carried merge saving:       251.97763219 bits
two-marked-line saving:     381.41503750 bits
combined local saving:      633.39266969 bits
adjusted vector moment:    1863.66054115 bits
residual to target:        1943.66054115 bits = 15.18484798 q-dim
```

This is material but still not enough to close `z=34`. It does, however, identify the next
recurrence object: a finite incidence-diagram state for a plane with multiple marked lines.

## Kernel-Lift Cover Check

The narrower diagnostic:

```text
--cover-kernel-lift
```

keeps quotient incidence and removes only `K <= L+L` multiplicity after the inner child container
is fixed. It has no effect on the depth-5 crossing:

```text
crossing_z = 137,
crossing_excess = 105.
```

The dominant rows either have no kernel lift or are controlled by quotient incidence instead.

Combining the safe tau-zero lift cover with the narrower kernel-lift cover:

```text
--cover-lift-mode tau0 --cover-kernel-lift
```

still leaves the checkpoint far from the production floor:

```text
crossing_z = 135,
crossing_excess = 103.
```

This is essentially the same crossing as tau-zero covering alone. The trace changes internally, but
the remaining mass is still carried by quotient-incidence and recursive child-flag looseness rather
than by duplicate kernel lifts.

## Shortened Child Flag Check

The diagnostic:

```text
--flag-bound best-shortened
```

replaces the inner-first flag ambient by the ideal shortened dimension after the outer zero witness.
It has no effect on the current depth-5 checkpoint:

```text
crossing_z = 137,
crossing_excess = 105.
```

At `z=34`, the best trace is unchanged from `flag_bound=best`. Thus the missing proof strength is
not a one-layer shortened-child ambient correction.

## Current Diagnosis

The base-seal proof is not blocked by the local tau-two root equation alone, and it is not rescued
by covering duplicate kernel lifts or by a one-layer shortened child flag ambient.

The current checkpoint is loose because it still routes quotient-incidence chains through coarse
one-layer child flag bounds. The next proof/implementation target is a finite exact-support
quotient-incidence DP that counts:

```text
child flag/container,
quotient line/plane datum,
invisible-fiber dimension,
root-line layer,
```

as one object across the depth-5 reachable states.
