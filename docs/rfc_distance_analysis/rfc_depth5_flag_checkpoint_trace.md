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

Best trace:

```text
level span z   log2_state      p   s   a   tau outer inner outer_z inner_z charge lift_qdim
5     1    34  2369.05321083   15  4   4   1   2     0     15      19      4      3
4     2    15  2470.43486387   7   1   1   1   4     2     7       8       1      9
3     4    7   1054.81698675   3   1   1   1   4     4     3       4       1      19
2     4    3   -1003.06336206  0   3   3   2   2     1     0       3       6      0
1     2    0   0.00000000      0   0   0   0   1     1     0       0       0      0
```

After removing the safe tau-zero duplicate lifts, the dominant path moves to tau-one
quotient-incidence chains. These tau-one quotient choices are real event data; they cannot be
zeroed the way the anti-conservative all-cover diagnostic does.

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
