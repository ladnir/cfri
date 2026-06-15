# RFC Lower-Bound / Falsification: Multi-Copy Stress

This note is adversarial.  It is for the original non-systematic RFC only.  It does not prove a
counterexample, but it records the strongest current way I can stress the target

```text
c = 8
k = 2048
N = 16384
q = 2^128
target excess e = 71
target zeros z = k + e = 2119
```

Third-iteration nested-kernel follow-up:

```text
docs/rfc_distance_analysis/rfc_lower_bound_nested_kernel_cascade.md
```

The first-iteration warning was that broad one-copy near-extremizer families can be required in
many independent copies at once.  With the old matched-core support count, this moved the worst
`e=71` row from about `-111.03` bits to about `-76.34` bits, missing an 80-bit first-moment target
by about `3.66` bits.

Second-iteration feedback from the upper-bound lane changes the right count.  The recurrence should
count child flags, not a scalar support state with a distinguished core.  In that flag state, the
dangerous seven-copy row pays a support-overlap penalty: for `core=1, support=1783`, the old
matched-core count labels each support `1783` times per active copy.  Across seven copies this is
about `75.60` fake bits.  Counting exact support sets once moves the seven-copy `e=71` row to about
`-151.94` bits.  The strongest exact-support row becomes the one-copy row at about `-121.83` bits.

This is not yet an explicit bad codeword family.  It is a union-bound / first-moment stress model
that the upper-bound proof must either absorb or rule out.

## Diagnostic Script

The small diagnostic added for this note is:

```text
scripts/rfc_distance_analysis/rfc_multicopy_falsification.py
```

It is a deterministic calculator, not a benchmark.  The default command for the headline rows is:

```text
python scripts/rfc_distance_analysis/rfc_multicopy_falsification.py \
  --depth 11 \
  --expansion 8 \
  --q-log2 128 \
  --target-excesses 70,71,72
```

Use `--by-active` to see the best row at each number of active copies.

Use `--support-count-model exact-size` for the flag-compatible count.  The default
`matched-core` mode preserves the old adversarial overcount for comparison.

## Stress Model

For `r` active copies, choose the same one-copy near-family shape in each active copy:

```text
core support b = k / live_rows
extra support h
one-copy support s = b + h
one-copy projective dimension D = floor(h / b)
```

The old matched-core model counts support-family choices as

```text
binom(c,r) * (k * binom(k-b,h))^r
```

This is intentionally adversarial but not flag-canonical.  The flag-compatible exact-support model
counts each visible support set once:

```text
binom(c,r) * binom(k,b+h)^r.
```

For the dangerous `b=1, h=1782` row:

```text
k * binom(k-1,h) = (b+h) * binom(k,b+h),
```

so the matched-core model overcounts by:

```text
r log2(b+h).
```

and estimates the projective common-line exponent by generic intersection in `P^(k-1)`:

```text
I(r,D) = rD - (r-1)(k-1).
```

The active copies deterministically contribute

```text
r(k-s)
```

zeros.  The remaining copies pay a random tail for the residual zero request:

```text
binom((c-r)k, z - r(k-s)) q^-(z - r(k-s)).
```

The modeled log2 expectation is therefore:

```text
log2 binom(c,r)
+ r(log2 support_family)
+ I(r,D) log2(q)
+ log2 binom((c-r)k, z-r(k-s))
- (z-r(k-s)) log2(q).
```

Rows with positive or near-positive values are warning signs.  They are not lower bounds until the
support-family overlap and exact-support constraints are controlled.

## Strongest Old Bad Family

Under the old matched-core count, the strongest `e=71` stress row is a seven-copy broad
near-family:

```text
active copies r = 7
live_rows = 2048
core support b = 1
extra support h = 1782
one-copy support s = 1783
one-copy projective dimension D = 1782
intersection exponent I = 7*1782 - 6*2047 = 192
deterministic zeros = 7*(2048-1783) = 1855
tail zeros needed in last copy = 2119 - 1855 = 264
modeled log2 expected = -76.33662864
```

Interpretation:

```text
This is not an explicit counterexample.
It is a generic-intersection stress model.
It is close enough to the 80-bit threshold that the proof cannot ignore multi-copy correlations.
```

The seven-copy row is better than the one-copy row because each broad support family still has a
large projective dimension.  Intersecting seven such families leaves a modeled `q^192` common-line
factor, while the final copy only has to supply `264` random zeros.

## Flag-State Recount

The upper-bound flag transition distinguishes:

```text
pi(W) vanishes on P union (S \ A),
pi(K) vanishes on P union S.
```

The important lower-bound consequence is that the child object is the actual visible support/flag,
not a choice of matched core inside that support.  For the seven-copy broad row:

```text
b = 1
h = 1782
b+h = 1783
r = 7
support-overlap penalty = 7 log2(1783) = 75.60063691 bits
```

The old row needed only:

```text
3.66337136 bits
```

to reach an 80-bit target.  Exact support-set counting therefore recovers far more than the missing
slack:

```text
old matched-core row:  -76.33662864
exact support row:    -151.93726555
```

The exact-support `e=71` by-active profile is:

```text
r=1: -121.83277193
r=2: -125.16543131
r=3: -129.28513672
r=4: -134.05348839
r=5: -139.41918392
r=6: -145.37809480
r=7: -151.93726555
r=8: -280.18621420
```

So the seven-copy broad near-family does not currently survive the flag/exact-support recount as a
near-counterexample.  The best exact-support stress is back to one active copy.

This is not a proof that `e=71` is safe.  It says this specific multi-copy objection was mostly a
support-label overcount.  A real falsification now needs nested-kernel entropy that is not collapsed
by passing from matched-core labels to child flags.

## Modeled Expectations

Best overall rows from the old matched-core multi-copy stress diagnostic:

```text
e=70: active copies 7, modeled log2 expected =   48.90605712
e=71: active copies 7, modeled log2 expected =  -76.33662864
e=72: active copies 8, modeled log2 expected = -193.78548630
```

For comparison, the one-copy-only stress model gives:

```text
e=70:  14.21707662
e=71: -111.03256536
e=72: -236.28232292
```

So the old multi-copy stress model makes `e=71` about `34.70` bits worse than the one-copy stress
model.  It still leaves `e=72` comfortably negative.

Best overall rows from the exact-support / flag-compatible diagnostic:

```text
e=70: active copies 1, modeled log2 expected =    3.41617672
e=71: active copies 1, modeled log2 expected = -121.83277193
e=72: active copies 1, modeled log2 expected = -247.08241391
```

This keeps the same qualitative crossing:

```text
e=70 is still unsafe in this stress model.
e=71 is safe against this exact-support multi-copy model with about 41.83 bits of slack to an
80-bit target.
```

Under the old matched-core model, the `e=71` best rows by active-copy count were:

```text
r=1: -111.03256536
r=2: -103.56524933
r=3:  -96.88486376
r=4:  -90.85312444
r=5:  -85.41872898
r=6:  -80.57754887
r=7:  -76.33662864
r=8: -193.78548630
```

This monotone rise through `r=7` was the first-iteration suspicious signal.  After exact-support
recounting it is classified as a support-label overcount, not as the current live obstruction.

## Status Of Explicitness

Current classification:

```text
exact one-copy extremizer:
  explicit/sampled at small depth;
  reaches k-1 zeros but does not threaten e=71.

wide one-copy near-family:
  heuristic/union-bound stress model;
  crosses between e=70 and e=71.

multi-copy broad near-family with matched-core labels:
  heuristic/generic-intersection stress model;
  now classified as an overcount for the flag recurrence;
  old e=71 row was -76.34.

multi-copy broad near-family with exact support sets:
  heuristic/generic-intersection stress model;
  current best e=71 row is -121.83;
  seven-copy row drops to -151.94.
```

I do not currently have an explicit scalable bad family that proves `e=71` false.  The strongest
remaining badness is not the seven-copy broad-support row; it is the possibility of a nested-kernel
cascade whose entropy survives the flag quotient.

## Existing Small-Depth Evidence

Existing exact/small-depth artifacts support two limited facts:

```text
docs/rfc_distance_analysis/rfc_extremizer_second_copy_check_depth4_m4.csv
```

Exact one-copy extremizer lines become full-weight in an independent second copy at depth 4.  This
argues against exact extremizers being the falsification route.

```text
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e2_model.csv
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e3_model.csv
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
```

For live block size `m=4` at depth 4, extra outputs `2` and `3` have kernel dimension `1` in all
saved pairs.  Extra outputs `4` has a small exceptional class:

```text
kernel_dim=1: 7872 pairs
kernel_dim=2:   48 pairs
```

The exceptional class lines up with complete extra stride structure.  This is not enough to prove
or disprove the production-scale stress model, but it confirms that broad near-families have real
dimension structure that must be tracked.

## Multi-Copy Correlation Attack Plan

The exact small-depth check that would falsify naive product-tail assumptions is:

```text
1. Fix depth d in {4,5} and a small prime q in {5,7,11}.
2. Generate two or three independent original non-systematic RFC copies.
3. For each support family A in copy 1, compute the message subspace S_A of lines whose copy-1
   output is supported in A.
4. For each support family B in copy 2, compute S'_B and the projective intersection dimension
   dim P(S_A cap S'_B).
5. Group by:
      support sizes,
      stride/core class,
      dim P(S_A),
      dim P(S'_B),
      dim P(S_A cap S'_B).
6. Compare the observed intersection dimension/count to the generic exponent:
      dim P(S_A) + dim P(S'_B) - (k-1).
7. Repeat for triples where feasible, or sample triples with exact rank arithmetic.
```

This check falsifies the product-tail picture if there is a large structured class with either:

```text
observed intersection dimension > generic intersection dimension
```

or

```text
pair/triple family count * q^(observed intersection dimension)
```

above the generic-intersection stress model by a production-relevant margin.

The more direct weight-enumerator version is:

```text
For each first-copy near-family subspace S_A, compute the exact second-copy output-weight
enumerator of projective lines in S_A.
```

Product-tail behavior predicts roughly:

```text
# lines in S_A with j prescribed second-copy zeros
  around q^(dim P(S_A)-j) times polynomial/support factors.
```

A persistent excess over this prediction is direct evidence that the same message is sparse in
independent copies more often than the proof can afford.

For the flag-state version, the exact check should enumerate nested pairs:

```text
L <= V
```

where:

```text
V = pi(W) with zero budget P union (S \ A),
L = pi(K) with zero budget P union S.
```

The falsifying signal is not merely a large support-family count.  It is a large number of flags
with:

```text
dim L substantial,
dim V/L small visible quotient,
and the same L surviving independent copies or paired compression more often than generic
intersection predicts.
```

This is the remaining route by which a nested-kernel cascade could replace the matched-core
overcount that the flag state already removes.

## Feedback For Upper-Bound Agent

The upper-bound flag feedback appears to kill the specific seven-copy broad-support obstruction,
provided the recurrence really counts exact visible supports/flags rather than matched-core labels.
The proof should explicitly record this: broad support labels collapse from
`k * binom(k-b,h)` to `binom(k,b+h)` when the state is the child flag.

The multi-copy correlation lemma is still needed, but the sharp version is:

```text
For independent RFC copies and exact support families A_1,...,A_r, the number of projective
message lines lying in all visible families is bounded by the generic projective intersection
exponent

  sum_i D(A_i) - (r-1)(k-1)

plus only acceptable polynomial/constant factors, unless the supports have a recursively paired
compression structure that is charged separately.
```

The old seven-copy row needed only `3.66` bits of recovery; exact-support overlap supplies about
`75.60` bits.  That is enough for this row, but it does not settle nested-kernel cascades.

Specific facts to prove or refute:

```text
1. The recurrence must count exact child flags, not matched-core labels.  Otherwise it recreates
   the artificial seven-copy obstruction.

2. Broad core=1, support=1783 families do not have more common projective lines across 7 copies
   than the generic-intersection exponent predicts once counted by exact support sets.

3. Paired compression is the only structured exception to generic intersection, and it preserves
   relative gap instead of creating a worse high-copy family.

4. Nested-kernel cascades do not create a replacement for the matched-core overcount.  In the flag
   transition, every invisible kernel layer that vanishes on `P union S` must either lose dimension
   generically or be charged by a lower-level paired-compression structure.

5. The recurrence state must remember active-copy count and common visible projective dimension, or
   otherwise prove that exact-support flags imply the same generic intersection exponent.
```

The current upper-bound target `e=71` looks more plausible after the flag recount.  The exact-size
stress row has about `41.83` bits of slack to an 80-bit target before final constants.  That is
real room, but still not enough to ignore polynomial factors or nested-kernel cascades.

## Questions For Integrator

1. Is the production certificate allowed to land at `e=72` if `e=71` remains too tight after
   polynomial factors, or is `e=71` a hard target?

2. Should the next exact check prioritize depth 4 exhaustive pair/triple intersections over
   depth 5 sampled intersections?

3. Do we want the upper-bound recurrence to include active-copy count `r` explicitly, or should it
   be folded into the replica/span state?

4. How much slack must be reserved for finite-field constants and exact-support cleanup?  After
   exact-support recounting, the current multi-copy stress leaves about `41.83` bits at `e=71`,
   before unknown flag-lift constants.

5. Should the next falsification step build an exact depth-4 two-copy flag-intersection enumerator,
   or should we first model nested-kernel cascades along the all-paired spine?
