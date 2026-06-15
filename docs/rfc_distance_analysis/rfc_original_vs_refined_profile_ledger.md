# Original Vs Refined Profile Ledger

Status: honest incremental-upgrade accounting for the original proof.

## Purpose

The previous local flat-excess lemmas are useful only if they can improve a full original-proof
row. This note records the first ledger that compares:

```text
original one-minor row
vs
explicit refined h=1 flat-excess bucket
```

at the same survivor profile.

The guiding rule is:

```text
No fake max. No replacing the proof everywhere.
Only refine an explicitly selected subevent, and keep track of the complement obligation.
```

## Script

Added:

```text
scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py
```

For a fixed top survivor profile `(p,t,D)`, it reports:

```text
old_full_profile_log2:
  profile entropy for the whole (p,t,D) row.

old_full_log2:
  original one-minor contribution for the whole row:
    profile + log2(D) - log2(q).

bucket_profile_log2:
  entropy after explicitly choosing a refined flat witness A.

old_same_bucket_log2:
  what the old one-minor proof would cost if it also enumerated the same A.
  This is an apples-to-apples local comparison, not the original proof.

refined_bucket_log2:
  h=1 refined bucket cost:
    choose P,A,T\A and sides
    + child line-zero on P union A
    + residual root repair.
```

The script deliberately distinguishes:

```text
local_bucket_status:
  does refined_bucket beat old_same_bucket?

global_refinement_status:
  is the bucket cheap enough to be worth pursuing, and does it still need a complement bound?
```

## Target Run

Command:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --flat-excess 1 --flat-excess 16 --flat-excess 72
```

The target profile is:

```text
p = 137
t = 1845
D = 887
S = 72.
```

The original one-minor accounting for the same profile is:

```text
old_full_profile_log2 = 9091.646238
old_local_log2        = -118.207210
old_full_log2         = 8973.439028.
```

The full top-deficit sum from the original one-minor tolerance script is:

```text
log2_bad_top_deficit = 8978.114943.
```

The same profile is the dominant term.

## h=1 Refined Buckets

Ledger rows:

```text
F=1:
  a = 1773
  z_child = 1910
  total_q_exp = 958
  old_same_bucket_log2 = 9407.798267
  refined_bucket_log2  = -113097.994523
  gain_vs_old_same     = 122505.792790 bits
  refined-old_full     = -122071.433551 bits

F=16:
  a = 1788
  z_child = 1925
  total_q_exp = 958
  old_same_bucket_log2 = 9336.108775
  refined_bucket_log2  = -113169.684015
  gain_vs_old_same     = 122505.792790 bits
  refined-old_full     = -122143.123043 bits

F=72:
  a = 1844
  z_child = 1981
  total_q_exp = 958
  old_same_bucket_log2 = 8984.288433
  refined_bucket_log2  = -113521.504357
  gain_vs_old_same     = 122505.792790 bits
  refined-old_full     = -122494.943385 bits
```

So the `h=1` refined bucket is a huge local win after the explicit `A`-counting overhead.

## What This Means

The refined `h=1` bucket is not the obstruction. It is far below both:

```text
1. the old proof applied to the same enumerated bucket;
2. the old proof applied to the whole survivor profile.
```

However, this is not yet a global improvement to the original proof. If we simply write:

```text
bad <= old_whole_profile + refined_h1_bucket,
```

we have made the bound slightly worse.

To get a real upgrade, we need a disjoint or canonical decomposition:

```text
bad-root event
  = h=1-refined bucket
    disjoint union
    complement.
```

Then the complement must be bounded by a restricted version of the original proof, not by the old
whole-profile bound. The h=1 bucket is cheap enough that almost any nontrivial complement saving
would be a net win, but the complement saving still has to be proved.

## Canonical-Witness Requirement

A proof-compatible refinement should choose a canonical flat witness from each bad-root witness
when the `h=1` condition holds. For example:

```text
1. from a bad root witness (x,y), define its common-zero singleton set A(x,y);
2. if A(x,y) contains an h=1 flat-excess witness, select a canonical minimal/lexicographic one;
3. route that event to the refined h=1 bucket;
4. route all other bad-root witnesses to the complement bucket.
```

The complement bucket must then have a local proof that uses the absence of selected `h=1`
witnesses. Without that restricted-complement theorem, the ledger is only a local comparison.

## Verdict

For the target profile, `h=1` is worth refining:

```text
local h=1 bucket: win
global proof upgrade: needs complement theorem
```

This changes the next proof target. We should not spend more time proving that the h=1 bucket is
small. It already is. The next useful question is:

```text
Can the original bad-root witness proof be restricted to the complement of canonical h=1 witnesses
and gain anything?
```

If yes, h=1 gives a real incremental proof upgrade. If no, h=1 remains a true local lemma but not a
finite-certificate improvement over the original proof.

## All-Bucket Replacement Update

The ledger now supports a full sweep:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --all-h --min-flat-excess 1 --max-flat-excess 72 \
  --summary-only --top 8
```

This scans:

```text
h = 1..887
F = 1..72
valid buckets = 63864.
```

The all-bucket refined sum is:

```text
old_full_log2           = 8973.439028
old_same_bucket_logsum  = 10823.608953
refined_bucket_logsum   = -113097.934786
gain_vs_old_same_logsum = 123921.543739
refined_sum-old_full    = -122071.373814.
```

This changes the diagnosis. The right replacement is not:

```text
h=1 bucket plus complement.
```

It is:

```text
all canonical common-zero buckets, charged by child subcode-zero events.
```

That theorem target is now recorded in:

```text
rfc_common_zero_subcode_replacement_theorem.md
```
