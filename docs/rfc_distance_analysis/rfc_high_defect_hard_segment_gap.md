# RFC High-Defect Hard-Segment Gap

Scope: original non-systematic RFC distance certificate, theta_2=-1 kernel-chain truncation.

Status: current blocker note after the charged hard-trace audit.

## Accounting Convention

For a maximal internal hard segment, the correct potential is:

```text
9H + rho_terminal - E_anc - constants.
```

The `9H` term already counts the local `q^-9` charge of every internal `s=5` hard row. Adding the
normal-slice scenario charge again is double-counting unless a separate boundary row has been
proved disjoint from the internal segment.

For a length-four truncation test where the new top row is explicitly separated from the exposed
child rank event, the boundary diagnostic is:

```text
boundary_charge + 9H_child + rho_terminal - E_anc - constants.
```

This is the meaning of `boundary_plus_strict_hard_margin_logq` in
`rfc_theta_chain_normal_slice.py`. It is not the default internal-segment margin.

## Production Zero Floor

For the target:

```text
k = 2048,
e = 71,
top zero request = 2119.
```

After `m` folds, every child branch has at least:

```text
ceil(2119 / 2^m)
```

zero requests. All-paired compression minimizes child zeros; kernel-following hard rows only add
zeros relative to all-paired compression.

The resulting floor is:

```text
m=0  child_k=2048  min_child_zeros=2119
m=1  child_k=1024  min_child_zeros=1060
m=2  child_k=512   min_child_zeros=530
m=3  child_k=256   min_child_zeros=265
m=4  child_k=128   min_child_zeros=133
m=5  child_k=64    min_child_zeros=67
m=6  child_k=32    min_child_zeros=34
m=7  child_k=16    min_child_zeros=17
m=8  child_k=8     min_child_zeros=9
m=9  child_k=4     min_child_zeros=5
m=10 child_k=2     min_child_zeros=3
m=11 child_k=1     min_child_zeros=2
```

Therefore the abstract near-dimension stress row:

```text
child_k=32,
zeros=(21,26,31)
```

is not directly reachable from the production distance event at the `child_k=32` level. It remains
a useful counter-signal for proof structures, but it is not itself a production trace.

## Regenerated Normal-Slice Tables

The deterministic tables with strict hard-trace columns are:

```text
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_default.csv
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_toy_child32_sweep.csv
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_reachable_child32_z34_sweep.csv
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_observed_level4_sweep.csv
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_observed_level5_sweep.csv
```

Key readings:

```text
toy child_k=32, zeros=(21,26,31):
  strict-only margin positive through b=8;
  zero at b=9;
  negative from b=10;
  boundary-plus margin positive through b=12 and negative from b=13.

reachable child_k=32, zeros=(34,39,44):
  strict-only margin positive through b=9;
  negative from b=10;
  boundary-plus margin positive through b=13 and negative from b=14.

observed child_k=8, zeros=(21,26,31):
  strict route finite and safe through b=6;
  strict route impossible from b=7 onward;
  defect charge is already huge.

observed child_k=16, zeros=(101,106,111):
  same pattern as child_k=8, with even larger defect charge.
```

Thus the observed checkpoint shapes are not the current problem. The live problem is the reachable
near-floor `child_k=32` stress row: high-defect normal slices with `b >= 14` are not closed even by
one separated boundary charge.

## Current Blocker

The failing reachable row at `b=14` has:

```text
child_k=32,
zeros=(34,39,44),
short_dims=(14,14,14),
E_anc=33,
strict_hard_potential=21,
boundary_plus_strict_hard_margin = 9 + 21 - 33 = -3.
```

The dominant strict trace for the first edge is:

```text
h=5 D=14 z=34: all-paired -> child D=7  z=17
h=4 D=7  z=17: hard s=5 -> child D=3  z=11
h=3 D=3  z=11: hard s=5 -> child D=1  z=8
terminal rho = 3
```

So this is not a long pure hard chain. It is an all-paired compression followed by two hard rows
and a small terminal rank event. The missing exponent is only three q-dimensions, but it is real
under the current charged recurrence.

## Plausible Closure Routes

1. High-kernel/local-incidence improvement:

```text
Prove that a hard row carrying a large shortened ambient pays more than the uniform q^-9
first-drop charge.
```

Even a small improvement would matter. For the reachable `b=14` row, three extra q-dimensions
close the boundary truncation.

2. Nested rank-profile charge:

```text
Replace the max-over-one-edge defect routing by a joint nested shortened-ambient profile.
```

The current script uses one exposed edge as a conservative diagnostic. The actual event requires a
nested profile of shortened ambients for `(34,39,44)`. A joint rank-profile theorem could supply
the missing few q-dimensions without assuming independence.

3. Boundary decomposition:

```text
Prove that the failing high-defect row is not internal to a maximal minimal-hard segment, but is
preceded or followed by a disjoint non-hard boundary row with its own charge.
```

This would justify using additional boundary charge, but it must be a separate state-label lemma.

4. Reachability sharpening:

```text
Use the full production zero-budget path, not just the all-paired floor, to show that a reachable
minimal hard segment at child_k=32 actually has more than zeros=(34,39,44), or smaller feasible b.
```

The all-paired floor is crude. Any preceding hard or residue row raises the child zero budget and
moves the row toward the observed safe regime.

## Next Concrete Lemma

The best next target is a reachability-plus-boundary lemma:

```text
At child_k=32 in the c=8,k=2048,e=71 distance event, any length-four theta_2=-1 truncation state
with zeros near the all-paired floor either:

1. is preceded by enough hard/residue rows to raise the local zero budget above the failing
   reachable stress row; or
2. has an explicit disjoint boundary row whose q^-9 charge may be added to the child strict
   potential; or
3. exposes a joint nested shortened-rank profile with at least three extra q-dimensions of charge.
```

This is narrower than the previous blocker and is the current proof frontier.
