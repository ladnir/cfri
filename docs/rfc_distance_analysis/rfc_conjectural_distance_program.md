# RFC Conjectural Distance Program

Status: relaxed goal / research program after the proof route became state-heavy.

## Purpose

The full near-MDS proof for original non-systematic RFC is still open. The current proof attempt
has made real structural progress, but the state needed to handle marked flats, mixed `PA`
coordinates, and full-span deficiencies is getting heavy.

This note reframes the goal:

```text
state plausible conjectures;
make them falsifiable;
collect experiments that distinguish true structure from proof artifact.
```

The aim is not to claim a theorem prematurely. The aim is to produce a coherent conjectural
distance certificate program that can guide proof work and experiments.

## Main Conjecture

For the original non-systematic RFC with determinant-1 fold:

```text
L_j = u_j + t_j w_j
R_j = u_j + (t_j+1) w_j
T_j uniform in F_q^*
```

and parameters:

```text
N = c k,
q large,
```

the expected number of bad zero sets satisfies a random-like paired-compression first moment:

```text
B_d(k+e) = E_T[# {Z : |Z|=k+e, rank(G_Z(T)) < k}]
        <= poly(N,d) * q^{-(e+1)}
```

after shape-sensitive recursion, where all-paired branches recurse exactly.

For the target:

```text
c = 8,
k = 2048,
q = 2^128,
```

the predicted crossing remains:

```text
e ~= 71
distance >= N-(k+71)+1 = 14266
MDS distance = 14337
gap to MDS = 71
```

For:

```text
c = 4,
k = 2048,
q = 2^128,
```

the predicted crossing is:

```text
e ~= 53
distance >= 6092
MDS distance = 6145
gap to MDS = 53
```

This is the optimistic conjecture. It should be treated as plausible, not proved.

## Weaker Conjecture A: Relaxed Near-MDS

There exists a modest constant/slack function `F(d,c)` such that:

```text
B_d(k+e+F) <= 1/2
```

where `e` is the random-like crossing.

For `c=8,k=2048,q=2^128`, even:

```text
F <= 16
```

would still give:

```text
distance >= 14250
```

versus MDS distance:

```text
14337.
```

This weaker conjecture may be much easier to justify because the flat-excess tolerance experiments
already show that small constant loss is acceptable.

## Weaker Conjecture B: Bounded Flat-Excess Dominance

The corrected surplus incidence formula says local repair codimension is:

```text
t - 2D + 1 - flat_excess.
```

Conjecture:

```text
dominant bad profiles have effective flat_excess <= O(1)
```

after recursively charging marked incremental rank tails and full-span deficiencies.

This conjecture does not require exact MDS-like behavior for every fixed set. It only requires that
large flat excess is itself rare enough to pay for its loss.

## Weaker Conjecture C: Marked Incremental Rank Tail

For marked pairs `(P,A)` in a child RFC code:

```text
rank(P union A) - rank(P) <= r
```

should have roughly random-code first-moment scale, up to shape-recursive paired losses:

```text
Pr/first-moment scale ~= q^{-(|A|-r)(k-rank(P)-r)}.
```

This is the right object for small flat witnesses. The aggregate short-set rank tail `B_d(z,s)` is
too coarse and is already known to be the wrong abstraction.

## Weaker Conjecture D: All-Mixed PA Full-Span Charge

For pure all-mixed `PA` blocks:

```text
A_alpha(J) = Full(J) - P_alpha(J).
```

Low marked rank:

```text
A_alpha(J) <= r
```

is controlled by deterministic full two-copy span deficiency:

```text
Full(J) <= |J| + r.
```

Conjecture:

```text
full two-copy span deficiencies have enough recursive first-moment cost
to charge pure all-mixed PA flat witnesses.
```

This is now the cleanest formulation of the previously scary multi-`PA` blocker.

## Falsification Criteria

Any of the following should make us lower confidence sharply:

1. Exact small-field first moments show bad-set crossings drifting away from random-like paired
   compression as depth increases.
2. Structured survivor families beat the predicted crossing by growing q-exponent, not just
   finite-field constants.
3. Marked incremental rank tails are dominated by an uncharged family whose cost does not grow with
   `|A|-r`.
4. All-mixed PA full-span deficiency occurs at much higher rate than random/two-copy span scale.
5. Independent-randomizer variant exhibits the same bad families, confirming the issue is structural
   to foldability rather than the linked `t,t+1` distribution.

## Experiments

### Experiment 1: Exact First Moments At Tiny Depth

Use:

```text
scripts/rfc_distance_analysis/rfc_brute_force_moment.py
```

Run exact:

```text
depth=1,2
q=3,5,7
c=2,4
replica=1
```

Track:

```text
B_d(1,z),
exact-support histogram A_d(1,w),
charge_per_zero_q.
```

Goal:

```text
verify whether the exact aggregate tail follows paired-compression/random-like charge
or develops a growing structural excess.
```

Falsify if:

```text
charge_per_zero_q decreases with depth in the target tail region after accounting for all-paired
compression.
```

### Experiment 2: Fixed Survivor Structured Families

Use:

```text
scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py
```

Probe:

```text
blocks,
strides,
top pair-heavy,
singleton-heavy,
balanced,
recursive.
```

For small fields/depths, compare:

```text
Pr[rank(G_S)<k],
E[q^(k-rank)-1],
top pair profile,
rootline signatures.
```

Goal:

```text
find whether bad sets concentrate in known paired/full-span structures or whether new families
appear.
```

Falsify if:

```text
a structured family has a rank-failure exponent below the conjectured recursive bound and the gap
grows with depth.
```

### Experiment 3: Marked Incremental Rank Tail

Use/extend:

```text
scripts/rfc_distance_analysis/rfc_incremental_flat_rank_scale.py
scripts/rfc_distance_analysis/rfc_marked_one_a_profile_scale.py
scripts/rfc_distance_analysis/rfc_multi_pa_graph_contraction_profile.py
```

The experiment driver:

```text
rfc_marked_incremental_rank_sampler.py
```

samples RFC generator instances and marked pairs `(P,A)`, then records:

```text
|P|, |A|, rank(P), rank(P union A)-rank(P),
top category profile PP/PA/A0/AA/P0,
full-span rank for all-mixed PA subblocks.
```

Goal:

```text
test whether marked rank deficits follow the random-code scale after conditioning on top category.
```

Falsify if:

```text
large marked rank deficits occur without either child rank deficiency or full-span PA deficiency.
```

### Experiment 4: All-Mixed PA Full-Span Deficiency

Use:

```text
scripts/rfc_distance_analysis/rfc_multi_pa_graph_contraction_profile.py
```

Run exact small-field profiles:

```text
q=5,7,11
k=6..10
mixed=2..5
U generated by graph rows
```

Track:

```text
Full(J),
P_alpha(J),
A_alpha(J),
distribution of Full(J) <= |J|+r.
```

Goal:

```text
verify that low A rank is explained by Full(J), not hidden root incidence.
```

Falsify if:

```text
A_alpha(J) <= r occurs frequently while Full(J) > |J|+r.
```

This should be impossible by the identity, so this is mainly a regression/sanity test.

### Experiment 5: Independent-Randomizer Comparison

Add independent mode to small-field oracles:

```text
fold law = linked_t_plus_1 | independent_distinct
```

For independent distinct:

```text
L = u + a w,
R = u + b w,
a != b.
```

Compare:

```text
exact first moments,
fixed survivor failures,
marked PA/A0 mixed profiles.
```

Goal:

```text
decide whether independent randomizers simplify non-pure mixed profiles enough to justify a pivot.
```

Falsify pivot if:

```text
linked and independent have the same dominant bad families and similar exponents.
```

## Confidence Levels

Current subjective confidence:

```text
Full e~=71 near-MDS conjecture:          40%
Relaxed e~=71+O(16) conjecture:          55%
Bounded flat-excess conjecture:          50%
Marked incremental rank-tail conjecture: 45%
All-mixed PA full-span charge:           60%
Independent-randomizer proof pivot:      35%
```

The best near-term way to raise or lower these numbers is not more proof prose. It is the marked
incremental sampler plus independent-randomizer comparison.

## Recommended Next Step

Run and extend:

```text
scripts/rfc_distance_analysis/rfc_marked_incremental_rank_sampler.py
```

It currently supports:

```text
linked,
independent_distinct.
```

It produces CSV grouped by:

```text
depth,
q,
fold_mode,
|P|,
|A|,
rank_increment,
top category profile,
full_span_defect.
```

That sampler would become the central experiment for deciding whether the conjectural program is
realistic or whether there is a hidden family we are still missing.
