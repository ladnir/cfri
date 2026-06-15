# RFC Fixed-Survivor Rank Tail Status

Status: active diagnostic for the non-systematic RFC distance proof. This note records the first
evidence for the fixed-set rank/subspace-evasion route after the projective-column-state wall.

## Proof Object

For a fixed survivor coordinate set `S`, measure:

```text
Pr_T[rank(G_S(T)) < k]
E_T[q^(k-rank(G_S(T))) - 1].
```

This is the erasure/recoverability version of the distance problem. It avoids the old aggregate
zero-count recurrence by fixing the coordinate set first.

Script:

```text
scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py
```

The script uses the shared-challenge RFC encoder model from `rfc_brute_force_moment.py`. It supports
structured survivor families:

```text
--set-family structured
--set-family blocks,strides,top,recursive
```

and now emits a multi-level fold signature:

```text
P/S/E/T | P/S/E/T | ...
```

where each level records paired, singleton, empty, and touched child positions from top to bottom.
It also emits top-child ranks on paired positions `P` and touched positions `U`, plus the
diagonal-transversality proxy:

```text
singleton_count - 2 * (k_child - rank(P)) + 1.
```

It also splits the parent rank into a pair-only block plus singleton increment:

```text
rank(pair-only parent block),
rank(full S) - rank(pair-only parent block).
```

In all current exact/sampled checks, the pair block obeys the clean identity:

```text
rank(pair-only parent block) = 2 * rank_child(P).
```

So paired survivor coordinates compress exactly to the child rank-tail problem. The local work is
the singleton rank increment modulo that paired block.

## Initial Signal

Tiny exact check:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 2 --q 5 --survivors k+1,k+2 --top-k 4
```

Worst observed fixed-set tails:

```text
excess 1: fail = 1/4,  log_q ~= -0.861
excess 2: fail = 1/16, log_q ~= -1.723
```

The `excess 2` worst sets have multi-level profile:

```text
2/2/0/4 | 2/2/0/4
```

All-paired profiles are safe in this tiny case; the hard case is recursively mixed paired/singleton
structure.

For these exact rows the pair-rank identity has zero mismatches. Example for the worst `excess 2`
row:

```text
avg_child_rP = 1.75
avg_pair_block_rank = 3.50 = 2 * avg_child_rP
avg_singleton_inc = 0.4375
```

The row fails when the paired child rank drops and the singleton increment does not repair it.

Structured `c=4` sampled check:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+1,k+2,k+3 \
  --set-family structured --challenge-samples 2000 --top-k 8 --seed 5
```

Worst structured profiles:

```text
excess 1: log_q fail ~= -0.837, profile 2/1/5/3 | 0/5/3/5
excess 2: log_q fail ~= -1.179, profile 2/2/4/4 | 0/6/2/6
excess 3: log_q fail ~= -1.694, profile 3/1/4/4 | 0/7/1/7
```

These are not random-matrix tails `q^-(excess+1)`, but they are nontrivial and may still be enough
for a paper-beating theorem if a positive per-excess rate survives at larger `c,d`.

The pair-rank identity again has zero mismatches in the displayed worst rows. The observed tradeoff:

```text
more paired top positions -> higher pair-block baseline,
but fewer singleton opportunities to repair child rank drops.
```

For `excess 3`, the pair-heavy profile `3/1/4/4 | 0/7/1/7` has only one top singleton; its singleton
increment is tiny on average, so failures are mostly inherited from the paired child block.

Structured depth-3 sampled check:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 3 --expansion 2 --q 5 --survivors k+1,k+2 \
  --set-family top,blocks,recursive --challenge-samples 800 --top-k 8 --seed 7
```

Worst profiles:

```text
excess 1: log_q fail ~= -0.414, profile 3/3/2/6 | 3/3/2/6 | 3/3/2/6
excess 2: log_q fail ~= -0.880, profile 5/0/3/5 | 4/2/2/6 | 2/6/0/8
```

The `excess 1` hard family repeats the same balanced top profile at every level. The `excess 2`
hard family has an all-paired top (`5/0/3/5`) and then pushes the difficulty into the child. This
shows the extremizer is recursive, but not a single fixed top pattern for all excesses.

Structured `c=8` smoke check:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 8 --q 5 --survivors k+1,k+2,k+3 \
  --set-family structured --challenge-samples 1500 --top-k 6 --seed 8
```

Worst profiles:

```text
excess 1: log_q fail ~= -0.807, profile 2/1/13/3 | 0/5/11/5
excess 2: log_q fail ~= -1.165, profile 2/2/12/4 | 0/6/10/6
excess 3: log_q fail ~= -1.590, profile 2/3/11/5 | 0/7/9/7
```

Increasing expansion from `c=4` to `c=8` does not by itself produce random-rank tails in these
small-depth structured probes. The same balanced/pair-heavy obstruction survives with extra empty
pairs. This is a warning against a flat "larger c randomizes fixed sets" proof.

## Interpretation

The fixed-survivor route is making real progress as a diagnostic. It shows:

1. Generic random-matrix rank tails are too optimistic.
2. The bad survivor sets are structured and recursively visible.
3. The diagonal-singleton heuristic explains some rows, but not all. Pair-heavy/all-paired top
   patterns can defer the obstruction into the child rather than paying singleton transversality at
   the current level.
4. Larger expansion did not remove the obstruction at depth 2; the worst profiles simply acquire
   more empty pairs.
5. Paired coordinates have an exact rank compression:
   `rank(pair block)=2*rank_child(P)`. This turns the paired part into a child rank-tail problem,
   leaving singleton increments as the local random-transversality term.

The next theorem target should therefore be a recursive rank-tail recurrence over survivor-set
profiles, not a flat random-rank claim. The state should include at least:

```text
multi-level pair/singleton signature,
top child ranks on P and U,
paired-compression child tail,
singleton diagonal-transversality tail.
```

The recurrence should probably be organized as:

```text
rank_parent(S) = 2 * rank_child(P) + singleton_increment(T modulo paired block).
```

The missing theorem is a lower-tail bound for `singleton_increment` conditioned on the child code
and paired block, plus a recursive rank-tail bound for `rank_child(P)`.

The latest singleton-increment histograms make this more precise. Since `k_parent=2*k_child`, define
the child pair deficit:

```text
D = k_child - rank_child(P).
```

Then the paired block contributes rank `2*(k_child-D)`, so the parent is full rank exactly when:

```text
singleton_increment >= 2D.
```

Equivalently:

```text
parent rank deficit = max(0, 2D - singleton_increment).
```

Examples from the exact `d=2,c=2,q=5` rows:

```text
excess 2, profile 2/2/0/4 | 2/2/0/4:
  pair/inc histogram = 2/1:256; 2/2:768; 4/0:3072
  failures are exactly the 2/1 cases: child P rank drops by 1 and two singletons add only 1 rank.
```

Examples from the sampled `d=2,c=4,q=5` structured rows:

```text
excess 2, profile 2/2/4/4 | 0/6/2/6:
  pair/inc histogram ~= 2/0, 2/1, 2/2, 4/0
  failures are 2/0 and 2/1; 2/2 repairs the child pair deficit.

excess 3, profile 3/1/4/4 | 0/7/1/7:
  pair/inc histogram ~= 2/0, 2/1, 4/0
  only one singleton is available, so a child pair deficit D=1 cannot be fully repaired.
```

This suggests a clean local theorem target:

```text
Given child image/quotient after paired positions P, bound
Pr[singleton_increment < 2D | rank_child(P)=k_child-D].
```

The number of singleton positions gives an obvious cap, but the useful bound must exploit diagonal
randomness to show repair succeeds with high probability when enough singleton positions are present.

The script now also emits a grouped `singleton_repair_summary` over all evaluated `(survivor set,
challenge)` events, keyed by:

```text
(#paired top positions, #singleton top positions, child pair deficit D, rank_child(U)).
```

This is the first test of whether the singleton repair law is compressible by simple rank/profile
data. In the checked rows, parent failure equals repair failure for each group, as predicted by:

```text
rank_parent(S) = 2*rank_child(P) + singleton_increment.
```

Sampled `d=2,c=4,q=5` structured rows give:

```text
survivor size 6, excess 2:
  P=2,T=2,D=1,rU=2: repair_fail = 1478/5137 ~= q^-0.774
  P=0,T=6,D=2,rU=2: repair_fail = 39/700   ~= q^-1.794
  P=2,T=2,D=0,rU=2: repair_fail = 0

survivor size 7, excess 3:
  P=3,T=1,D=1,rU=2: repair_fail = 1
  P=2,T=3,D=1,rU=2: repair_fail = 53/297   ~= q^-1.071
  P=0,T=7,D=2,rU=2: repair_fail = 1691/87600 ~= q^-2.453
```

This is a useful sign: the local term is not obviously chaotic. It appears governed by how many
singleton equations are available to repair a child pair deficit. The missing proof is now closer to
a finite-field diagonal transversality statement:

```text
Given a D-dimensional paired-kernel quotient, T singleton coordinates repair 2D missing rank
except with probability decreasing in the singleton surplus over 2D.
```

The observed exponent is not simply `T-2D+1`, but it is monotone in these tested rows and far from
the flat random-rank fantasy. This is exactly the kind of local law a recursive certificate could
use.

However, the simple key is not tight enough by itself. The script now reports `avg_set_repair_fail`
and `worst_set_repair_fail` inside each repair key. For the sampled `d=2,c=4,q=5` structured rows:

```text
survivor size 6, P=2,T=2,D=1,rU=2:
  aggregate repair_fail = 1478/5137 ~= q^-0.774
  worst set repair_fail = 117/193   ~= q^-0.311

survivor size 7, P=2,T=3,D=1,rU=2:
  aggregate repair_fail = 53/297 ~= q^-1.071
  worst set repair_fail = 58/247 ~= q^-0.900
```

So `(P,T,D,rU)` captures the repair condition but not the full repair probability. Some child
quotient geometry, multilevel profile, or matroid data still matters. This is not a blocker yet:
the variation is visible and bounded in these tests, but a theorem cannot rely only on the simple
key.

## Refined Repair-State Check

The immediate follow-up was to refine `singleton_repair_summary` by multilevel fold profile. The
script now emits:

```text
singleton_repair_profile_summary
```

keyed by:

```text
(multilevel P/S/E/T profile, #paired top positions, #singleton top positions, D, rank_child(U)).
```

This refinement matters. In the sampled `d=2,c=4,q=5` structured run with 1200 challenge samples,
the previously coarse `P=2,T=2,D=1,rU=2` group splits into two qualitatively different profile
classes:

```text
2/2/4/4 | 0/6/2/6:
  repair_fail = 1065/1958 ~= q^-0.378
  worst-set repair_fail = 117/193 ~= q^-0.311

2/2/4/4 | 2/2/4/4:
  repair_fail = 1891/8316 ~= q^-0.920
  worst-set repair_fail = 95/319 ~= q^-0.753
```

So multilevel profile is real information, not just formatting. It partially explains the
worst-set gap inside the coarse repair key.

However, multilevel profile is still not a complete state. The profile-refined rows retain
set-to-set variation, and the worst profile can be much worse than a simple singleton-surplus
heuristic would predict. This strongly suggests the recurrence state needs one more layer of child
quotient geometry, such as a rank/matroid signature of the child columns seen by singleton
positions after quotienting by the paired block.

The depth-3 structured probe:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 3 --expansion 2 --q 5 --survivors k+1,k+2 \
  --set-family top,blocks,recursive --challenge-samples 600 --top-k 6 --seed 7
```

shows two hard modes:

```text
excess 1:
  profile 3/3/2/6 | 2/5/1/7 | 3/3/2/6
  fail ~= 311/600 ~= q^-0.408

excess 2:
  profile 5/0/3/5 | 4/2/2/6 | 2/6/0/8
  fail ~= 149/600 ~= q^-0.866
```

The first is a recursively mixed paired/singleton profile. The second has no top singletons and
therefore cannot repair a child pair deficit at the top; it simply compresses the failure to the
child. This is exactly the paired-spine obstruction:

```text
rank_parent(S) = 2 * rank_child(P)
```

when `T=0` at the top.

The depth-3 repair tables also expose a capacity obstruction. If the singleton count is smaller
than `2D`, repair is impossible regardless of randomness:

```text
singleton_increment <= #singletons < 2D
```

The recurrence must separate these deterministic capacity failures from probabilistic
transversality failures. This is a useful simplification: not every bad row needs an algebraic
anti-concentration theorem.

The next refinement is an opt-in quotient signature:

```text
--quotient-signature
```

For each child challenge, this records the rank-increment histogram of singleton child columns
modulo paired child columns:

```text
(|A|, rank(P union A)-rank(P)) for A subseteq T.
```

This is a first proxy for the real local object:

```text
K_P restricted to T,
where K_P = ker(child evaluation on P).
```

It is intentionally off by default because it is much slower: on the small `d=2,c=4,q=5` structured
probe it turned a roughly 6-second run into roughly 70 seconds.

The quotient signature explains another layer of the previous gap. For the bad
`2/2/4/4 | 0/6/2/6`, `P=2,T=2,D=1,rU=2` profile:

```text
quotient full=1 with one singleton invisible:
  signature m=2; full=1; hist=0/0:1;1/0:1;1/1:1;2/1:1
  repair_fail = 1

quotient full=1 with both singletons visible but parallel:
  signature m=2; full=1; hist=0/0:1;1/1:2;2/1:1
  repair_fail = 295/1188 ~= q^-0.866
```

This is very informative: the worst profile aggregate was not a mysterious sampling artifact. It
mixed deterministic invisibility with genuine probabilistic root-line repair. The state needed for
a theorem is therefore closer to a kernel-quotient matroid/root-line rank-drop invariant than to
the scalar tuple `(|P|, |T|, D, rank(U))`.

Explorer C's local-algebra framing matches this diagnostic. After paired coordinates, the remaining
parent kernel is essentially:

```text
K_P x K_P
```

and singleton coordinates impose root-line equations:

```text
x_j + alpha_j y_j = 0.
```

The next local theorem should be a kernel singleton repair lemma:

```text
If K_P|_T has rank < D, repair can fail deterministically.
If K_P|_T has rank D and satisfies the appropriate two-copy/root-line Hall condition, bound
Pr_alpha[rank L_alpha < 2D]
```

with the first safe target being a `q^-1` failure bound for the non-deterministic cases, and only
later a stronger surplus exponent such as `|T|-2D+1` where the quotient geometry supports it.

The quotient diagnostic now also reports the Hall obstruction:

```text
hall_min = min_A ( |T|-|A| + 2 rank(K_P|_A) )
hall_deficit = max(0, 2D - hall_min).
```

This is the generic two-copy/root-line obstruction for the singleton repair map on `K_P x K_P`.
The first targeted run:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family top --challenge-samples 400 --top-k 12 --hist-limit 8 \
  --seed 5 --quotient-signature --rootline-exact-max 3
```

gives a much cleaner split for the hard `2/2/4/4 | 0/6/2/6`, `P=2,T=2,D=1` row:

```text
Hall-deficient row:
  full=1, hall_min=1, hall_deficit=1
  exact_rootline_fail = 1
  observed repair_fail = 1

Hall-OK row:
  full=1, hall_min=2, hall_deficit=0
  exact_rootline_fail = 1/4 ~= q^-0.861
  observed repair_fail = 6/25 ~= q^-0.887
```

This is the first genuinely clean local abstraction signal in the fixed-survivor route. The bad
profile was not just hiding arbitrary projective data: in this small case, visibility/Hall separates
deterministic failure from probabilistic root-line failure, and exact root-line enumeration predicts
the sampled repair rate.

For larger singleton counts such as `T=6,D=2`, exact root-line enumeration was intentionally skipped
in this run. The quotient rank histograms still vary, and those rows are the next place to test
whether Hall plus first root-line rank-drop codimension gives a stable exponent.

Follow-up: exact root-line enumeration was enabled for `T<=6` using a cache keyed by the actual
`K_P|_T` evaluation matrix and singleton-side pattern:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family top --challenge-samples 200 --top-k 8 --hist-limit 8 \
  --seed 5 --quotient-signature --rootline-exact-max 6
```

The run reports:

```text
rootline_exact_cache_entries = 812
rootline_signature_ambiguity total_signatures=14 ambiguous_signatures=0
```

Here `rootline_signature_ambiguity` groups by:

```text
(singleton side pattern, K_P|_T rank/Hall signature)
```

and checks whether the same signature ever has two different exact root-line failure probabilities.
In this sample it does not. That is evidence against immediate cross-ratio/projective leakage in
the first larger Hall-OK rows.

For the `T=6,D=2` singleton-heavy rows, Hall always passes:

```text
full=2, hall_min=4, visibility_deficit=0, hall_deficit=0.
```

The exact finite-field root-line probabilities vary with the rank-function signature, and the
observed repair rates track them:

```text
rank signature with many rank-one pairs:
  exact_rootline_fail = 67/256 ~= q^-0.833
  observed repair_fail ~= q^-0.82 to q^-0.89 in the small sample

more uniformly rank-two pair structure:
  exact_rootline_fail = 11/128 ~= q^-1.525
  observed repair_fail ~= q^-1.60

strongest displayed rank signature:
  exact_rootline_fail = 7/256 ~= q^-2.236
  observed repair_fail ~= q^-2.23 in the 400-sample run
```

This suggests the local theorem should not use only Hall pass/fail. Hall detects deterministic
generic failure, but the useful exponent is the first root-line rank-drop codimension/finite-field
root count determined by the represented rank function of `K_P|_T` plus singleton-side domains.

This is a better situation than the previous projective-state wall: in the checked rows, the rank
function and side pattern are sufficient to predict the exact repair rate. This is still small-field
evidence, not a theorem.

A mixed-side block-family check sharpened this conclusion. With:

```text
python -B scripts/rfc_distance_analysis/rfc_fixed_survivor_rank_tail.py \
  --depth 2 --expansion 4 --q 5 --survivors k+2 \
  --set-family blocks --challenge-samples 80 --top-k 14 --hist-limit 8 \
  --seed 11 --quotient-signature --rootline-exact-max 6
```

the output reports:

```text
rootline_signature_ambiguity total_signatures=96 ambiguous_signatures=54
rootline_parallel_class_ambiguity total_signatures=105 ambiguous_signatures=0
```

So the rank/Hall histogram plus the ordered side pattern is too coarse when left and right
singleton domains mix. But the `D=2` projective parallel-class signature with side counts remains
single-valued in this sample. The current local lemma candidate is recorded in:

```text
docs/rfc_distance_analysis/rfc_d2_rootline_parallel_class_lemma.md
```

The candidate state for `D=2` is:

```text
zero singleton columns in K_P|_T,
multiset of (left_count, right_count) over projective parallel classes.
```

This is still compact and does not include cross-ratio data.

The script now implements an executable canonical formula for this state:

```text
parallel_class_formula_failure_d2
```

It replaces the actual projective directions by arbitrary distinct canonical directions in `F_q^2`
and enumerates the allowed left/right alpha domains. In a smaller mixed-side block check with 40
challenge samples, the formula matched actual exact enumeration for every observed signature:

```text
rootline_parallel_formula_mismatches=0 distinct_mismatches=0
```

This is stronger than the ambiguity check: it says the proposed local state not only grouped
observed exact values, but also predicted them by a coordinate-free canonical replacement.

## Next Check

The next useful check is not broad random sampling. It is to promote the Hall diagnostic into a
small root-line layer profiler:

```text
for K_P|_T:
  compute visibility deficit,
  compute Hall/generic-kernel deficit,
  compute first root-line rank-drop codimension eta when Hall passes.
```

Then run it on the observed worst profile families:

```text
balanced:     roughly 2/e? mixed pairs and singletons at every level,
paired-spine: all/mostly paired at the top, pushing the hard tail into the child.
```

If Hall plus a finite list of root-line layer codimensions predicts repair exponents, the route has
a plausible theorem shape. If rows with the same rank function/Hall/layer codimension still have
different exponents because of cross-ratio data, then the projective wall has reappeared.
