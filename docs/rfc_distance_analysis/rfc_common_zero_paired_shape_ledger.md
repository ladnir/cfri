# Common-Zero Paired-Shape Ledger

Status: shape-sensitive stress test for the common-zero subcode replacement theorem.

## Purpose

The paired-envelope red flag mixed:

```text
compressed all-paired subcode exponent
```

with:

```text
arbitrary-set entropy.
```

This was intentionally unfair. The paired-shape ledger counts pure all-paired zero sets with their
reduced entropy:

```text
Z all-paired for m levels
  -> choose z/2^m lower positions.
```

The goal is to decide whether the red flag disappears after shape-sensitive counting.

## Script

Added:

```text
scripts/rfc_distance_analysis/rfc_common_zero_paired_shape_ledger.py
```

For each common-zero bucket `(h,F)` and paired depth `m`, it computes:

```text
r = D-h
a = 2r+F
z = p+a
```

and, if `z` is divisible by `2^m`, compresses:

```text
z -> z/2^m
h -> ceil(h/2^m).
```

The shape entropy is:

```text
choose compressed lower Z
choose which lifted z positions are C rather than P
choose remaining singleton coordinates outside Z
choose singleton sides.
```

For canonical common-zero buckets, `h` is an exact kernel dimension. Pure all-paired compression
doubles rank at each lifted level and therefore doubles kernel dimension:

```text
h = 2^m h_compressed.
```

The script enforces this exact divisibility by default. The old relaxed diagnostic
`ceil(h/2^m)` can still be run with `--relaxed-kernel-ceil`, but that counts a larger
`dim ker >= h` event and can mix the wrong root-residual exponent into an exact-`h` bucket.

This is still not a certificate because it only models pure all-paired shapes. Mixed shapes need
the full recurrence.

## Target Runs

### Exact-Kernel Pure All-Paired Sweep

Rerunning the full paired-depth sweep with exact all-paired kernel lifting gives:

```text
log2_sum = -912.890257
dominant:
  h = 768
  F = 9
  r = 119
  a = 247
  z = 384
  paired_levels = 7
  compressed depth = 3
  compressed z = 3
  compressed h = 6
  subcode_q = 6
  root_residual_q = 63
  total_q = 69
  log2_shape = 7919.109743
  log2_term = -912.890257
  extra q-dimensions needed for 80-bit target = 0.
```

Restricting paired depth to six levels remains safe:

```text
log2_sum = -1619.310830
dominant:
  h = 832
  F = 9
  z = 256
  paired_levels = 6
  compressed depth = 4
  compressed z = 4
  compressed h = 13
  total_q = 76
  log2_term = -1619.310830
  extra q-dimensions needed for 80-bit target = 0.
```

Restricting paired depth to four levels is also safe:

```text
log2_sum = -6002.510989
dominant:
  h = 16
  F = 57
  z = 1936
  paired_levels = 4
  compressed depth = 6
  compressed z = 121
  compressed h = 1
  total_q = 73
  log2_term = -6002.510989
  extra q-dimensions needed for 80-bit target = 0.
```

So the pure all-paired spine is no longer a blocker once the canonical exact-`h` condition is
enforced. This is a meaningful win for the common-zero route: the earlier positive rows were
artifacts of charging a `dim ker >= h` compressed event with the root-residual exponent for exact
dimension `h`.

### Relaxed-Kernel Warning

The relaxed diagnostic is still useful as a warning about what not to prove.

The first full sweep originally found a compressed depth-2 event with:

```text
compressed z = 1
compressed h = 4
compressed k = 4.
```

That row asks for a nonempty RFC column set to have rank zero. RFC columns are never zero, so the
script now rejects:

```text
compressed z > 0 and compressed h >= compressed k.
```

Rerunning the relaxed full paired-depth sweep after this feasibility check gives:

```text
log2_sum = 7169.731801
dominant:
  h = 735
  F = 71
  r = 152
  a = 375
  z = 512
  paired_levels = 8
  compressed depth = 2
  compressed z = 2
  compressed h = 3
  subcode_q = 3
  root_residual_q = 1
  total_q = 4
  log2_shape = 7681.731801
  log2_term = 7169.731801
  extra q-dimensions needed for 80-bit target = 56.638530.
```

This row is not a canonical exact-`h` bucket. The compressed event has `h_compressed=3`, so an
eight-level pure all-paired lift has actual kernel dimension:

```text
2^8 * 3 = 768,
```

not `735`. It therefore belongs, if at all, to an `h=768` bucket with a different flat-excess and
root-residual exponent. The positive relaxed row is an overcounting artifact, not a proof blocker.

Restricting paired depth to six levels in relaxed mode gives:

```text
log2_sum = 4575.098087
dominant:
  h = 831
  F = 71
  z = 320
  paired_levels = 6
  compressed depth = 4
  compressed z = 5
  compressed h = 13
  total_q = 27
  log2_term = 4575.098087
  extra q-dimensions needed for 80-bit target = 36.367954.
```

Restricting paired depth to four levels in relaxed mode is safe:

```text
log2_sum = -4452.791981
dominant:
  h = 15
  F = 71
  z = 1952
  paired_levels = 4
  compressed depth = 6
  compressed z = 122
  compressed h = 1
  total_q = 60
  log2_term = -4452.791981
  extra q-dimensions needed for 80-bit target = 0.
```

The old stress threshold between four and six paired levels was therefore a relaxed-kernel
threshold, not a canonical bucket threshold.

### Pre-Patch Baseline

Full paired-depth sweep:

```text
python scripts/rfc_distance_analysis/rfc_common_zero_paired_shape_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --min-h 1 --min-flat-excess 1 --max-flat-excess 72 --top 12
```

Output:

```text
log2_sum = 7450.342384
dominant:
  h = 863
  F = 71
  r = 24
  a = 119
  z = 256
  paired_levels = 8
  compressed depth = 2
  compressed z = 1
  compressed h = 4
  total_q = 5
  log2_shape = 8090.342384
  log2_term = 7450.342384.
```

Thus the red flag survives shape-sensitive all-paired counting if arbitrarily long paired spines
are allowed.

Restricting paired depth to six levels:

```text
--max-paired-levels 6
```

still gives:

```text
log2_sum = 4575.098087
dominant:
  h = 831
  F = 71
  z = 320
  paired_levels = 6
  compressed depth = 4
  compressed z = 5
  compressed h = 13
  total_q = 27
  log2_term = 4575.098087.
```

Restricting paired depth to four levels:

```text
--max-paired-levels 4
```

gives:

```text
log2_sum = -4452.791981
dominant:
  h = 15
  F = 71
  z = 1952
  paired_levels = 4
  compressed depth = 6
  compressed z = 122
  compressed h = 1
  total_q = 60
  log2_term = -4452.791981.
```

So the stress threshold lies between four and six paired levels for the target profile.

## Interpretation

The common-zero replacement theorem cannot simply use:

```text
child subcode-zero recurrence with all-paired branches
```

and expect the target profile to close automatically. It must distinguish exact kernel dimension
from lower-bound kernel events. Once that distinction is made, pure all-paired compressed events are
safe at the target profile.

The obstruction is now sharper in a different way:

```text
mixed paired/singleton shapes, where exact kernel dimensions need not be pure powers of two.
```

The pure all-paired case should become a closed lemma using the paired-compression rank identity.
The remaining recurrence still has to handle mixed shapes where only part of `P union C` compresses
and the rest contributes singleton/root-line constraints.

## What Might Still Save The Route

The paired-shape ledger is still conservative in several ways:

1. It counts `P` and `C` labels inside the lifted all-paired set by `binom(z,a)`, ignoring that
   `P` and `C` must arise from the parent survivor profile and common-zero witness equations.

2. It does not charge the paired spine as a recursive survivor-profile event at the parent proof
   level; it only applies child subcode compression.

3. It treats every deep all-paired `Z` as compatible with the fixed top `(P,T,D)` profile, while
   such a deeply paired set may force additional structure in `P` and `T`.

4. It ignores exact full-common-zero maximality outside `C`, using only `C subset C(x,y)`.

The next theorem target should therefore isolate deep paired spines as their own canonical bucket:

```text
common-zero bucket
  + paired-spine depth m
  + compressed lower support U.
```

Then ask whether the top survivor profile entropy drops enough once `P`, `C`, and `T\C` are
required to be compatible with the same paired spine.

## Current Verdict

The common-zero replacement route improved materially. The pure all-paired spine red flag was a
relaxed-kernel artifact: canonical exact-`h` lifting makes the full-depth target row safe by about
`912.89` bits. The next blocker is not pure all-paired compression; it is the mixed-shape
recurrence that interpolates between exact paired compression and singleton/root-line repair.
