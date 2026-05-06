# RFC Global Rank-Cancellation Probe

This note records the first small-model probe for the global rank-budget pivot.

## Diagnostic

For a fixed matched row block and a final virtual support:

```text
Y = (C \ H) union E,
```

the probe:

```text
1. computes the admissible dimension for support Y;
2. finds tree sibling absences that look like cancellation zeros;
3. subtracts the selected exact core's baseline cancellation pattern;
4. ignores holes inside C;
5. keeps only rank-active cancellation zeros, meaning adding the zero coordinate back increases the
   admissible dimension;
6. checks that those active zeros impose enough independent rank relative to the global projective
   excess dimension.
```

The checked inequality is:

```text
imposed_rank >= max(0, active_cancellations - projective_excess - 1).
```

The `-1` is the current scalar-convention allowance matching the no-early-gluing ratio proof.

## Why Rank-Active Matters

A naive version that counted every raw sibling absence failed already at depth 3. The failures were
exact matched cores: the selected core itself has a tree cancellation pattern, and those zeros are
not excess equations.

After subtracting the exact core baseline, depth 4 still had raw sibling absences that imposed no
rank when added back. These are automatic zeros for the fixed row/support geometry. They should not
be counted as cancellation equations in the global rank budget.

Thus the working algebraic target should count:

```text
rank-active excess cancellation equations,
```

not raw support-level sibling absences.

## Results

The current probe artifacts are:

```text
docs/rfc_global_rank_cancellation_probe_depth3_e2_h1.csv
docs/rfc_global_rank_cancellation_probe_depth4_e2_h1.csv
docs/rfc_global_rank_cancellation_probe_depth5_e1_h1.csv
docs/rfc_global_rank_cancellation_probe_depth5_live3_e2_h1_random.csv
docs/rfc_global_rank_cancellation_probe_depth5_live4_e2_h1_random.csv
docs/rfc_global_rank_cancellation_probe_depth6_live3_e2_h1_random.csv
docs/rfc_global_rank_cancellation_probe_depth6_live4_e2_h1_random.csv
docs/rfc_global_rank_cancellation_probe_depth6_live5_e2_h1_random.csv
```

Runs:

```text
depth=3, max_extra=2, max_holes=1: checked=1240 failures=0 worst_gap=0
depth=4, max_extra=2, max_holes=1: checked=8512 failures=0 worst_gap=0
depth=5, max_extra=1, max_holes=1: checked=6624 failures=0 worst_gap=0
depth=5, live_bits=3, random, max_extra=2, max_holes=1: checked=1888 failures=0 worst_gap=0
depth=5, live_bits=4, random, max_extra=2, max_holes=1: checked=1860 failures=0 worst_gap=0
depth=6, live_bits=3, random, max_extra=2, max_holes=1: checked=2368 failures=0 worst_gap=0
depth=6, live_bits=4, random, max_extra=2, max_holes=1: checked=2112 failures=0 worst_gap=0
depth=6, live_bits=5, random, max_extra=2, max_holes=1: checked=1717 failures=0 worst_gap=0
```

The depth-4 run included active cancellation counts up to `9`. The depth-5 narrow run included
active cancellation counts up to `10`. The randomized live-bits-4 and live-bits-5 runs included
nontrivial active cancellation counts up to `2`; some smaller-live randomized runs were clean but
had no active cancellation zeros.

## Interpretation

This does not prove the global rank-cancellation lemma. It does show that the immediate toy
counterexamples disappear once the lemma is phrased in terms of rank-active excess cancellation
equations.

The next formal target is:

```text
For fixed final virtual support Y, the rank-active excess cancellation zero functionals are
generically independent modulo the global admissible projective dimension, up to the one scalar
allowance from no-early-gluing.
```

This is stated more explicitly in:

```text
docs/rfc_rank_active_cancellation_independence.md
```

This target matches the first-moment count more closely than child-local rank spending.
