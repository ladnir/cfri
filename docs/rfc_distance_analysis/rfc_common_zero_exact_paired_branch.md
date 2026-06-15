# Common-Zero Exact Paired Branch

Status: theorem brick for the common-zero subcode replacement program.

## Purpose

The paired-envelope red flag was caused by mixing two different events:

```text
canonical bucket:      dim ker(ev_Z) = h
relaxed subcode event: dim ker(ev_Z) >= h.
```

For a pure all-paired zero set, the canonical bucket has an exact divisibility law. Enforcing that
law makes the pure all-paired branch safe for the target profile.

## Exact Lift Lemma

Let `Z` be all-paired for one RFC fold, with compressed child set `U`. The local paired transform is
invertible coordinatewise, so:

```text
rank_d(Z) = 2 rank_{d-1}(U).
```

Since the message dimension also doubles:

```text
k_d = 2 k_{d-1},
```

the kernel dimension doubles exactly:

```text
dim ker_d(ev_Z) = 2 dim ker_{d-1}(ev_U).
```

Iterating for `m` paired levels gives:

```text
z = 2^m z'
h = 2^m h'.
```

Therefore a canonical exact-`h` common-zero bucket can contain a pure `m`-paired spine only if
`2^m` divides `h`.

## Why The Old Positive Row Was Spurious

The relaxed paired-shape diagnostic found:

```text
h = 735
F = 71
z = 512
paired_levels = 8
compressed h' = 3
log2_term = 7169.731801.
```

But a compressed `h'=3` pure 8-level lift has actual kernel dimension:

```text
2^8 * 3 = 768,
```

not `735`. It belongs to a different canonical bucket with a different flat-excess and
root-residual exponent. Charging it in the `h=735,F=71` bucket was therefore an overcount.

## Corrected Target Run

With exact kernel lifting enforced:

```text
python scripts/rfc_distance_analysis/rfc_common_zero_paired_shape_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --min-h 1 --min-flat-excess 1 --max-flat-excess 72 --top 12
```

gives:

```text
log2_sum = -912.890257
dominant:
  h = 768
  F = 9
  z = 384
  paired_levels = 7
  compressed depth = 3
  compressed z = 3
  compressed h = 6
  total_q = 69.
```

So the pure all-paired branch is safe by about:

```text
912.89 bits
```

against the 80-bit target in the top-profile stress model.

## Proof Use

Inside the common-zero replacement theorem, the pure all-paired branch should be routed by:

```text
1. check divisibility h = 2^m h';
2. compress Z to U;
3. charge the lower exact bucket (h', |U|);
4. use reduced all-paired shape entropy;
5. apply the top residual root exponent for the actual lifted h.
```

This closes the pure paired-spine red flag. It does not close mixed shapes, where a spill level
contains singleton zero requests and exact kernel dimension is not a pure power-of-two lift.

## Next Frontier

The next diagnostic is:

```text
scripts/rfc_distance_analysis/rfc_common_zero_one_spill_ledger.py
```

It tests shapes that are paired below one spill level. The first deep-spill sweep finds a positive
row needing about `19.24` q-dimensions of additional saving, so the active blocker has moved from
pure all-paired compression to mixed paired/singleton spill recurrence.
