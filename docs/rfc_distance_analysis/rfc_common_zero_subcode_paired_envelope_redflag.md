# Common-Zero Subcode Paired-Envelope Red Flag

Status: diagnostic warning for the common-zero replacement theorem.

## Purpose

The full common-zero replacement theorem is promising under the random child subcode-zero exponent.
But RFC is recursive. A child zero set can be all-paired at the next level, causing kernel-dimension
tail events to compress:

```text
(h,z) -> (ceil(h/2), z/2).
```

This note records the first all-paired-envelope stress test. It shows that a uniform fixed-set
random exponent cannot be the theorem statement.

## Ledger Models

The profile ledger now has:

```text
--subcode-model random
--subcode-model paired_envelope
```

The random model uses:

```text
q_exp = h(z-k+h).
```

The paired envelope replaces this by the minimum exponent seen along the all-paired compression
chain:

```text
(d,z,h), (d-1,z/2,ceil(h/2)), ...
```

This is deliberately pessimistic because it applies the all-paired exponent while still using the
entropy of arbitrary zero sets. It is a red-flag test, not a valid upper bound.

## Target Run

Random model:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --subcode-model random \
  --all-h --min-flat-excess 1 --max-flat-excess 72 \
  --summary-only --top 5
```

gives:

```text
old_full_log2         = 8973.439028
refined_bucket_logsum = -113097.934786
dominant bucket       = h=1,F=1,total_q=958.
```

Paired-envelope model:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --subcode-model paired_envelope \
  --all-h --min-flat-excess 1 --max-flat-excess 72 \
  --summary-only --top 8
```

gives:

```text
old_full_log2         = 8973.439028
refined_bucket_logsum = 10672.925272
dominant bucket       = h=479,F=71,r=408,a=887,z_child=1024,total_q=2.
```

This loses to the original row by:

```text
1699.486244 bits.
```

## Interpretation

This does not refute the common-zero replacement theorem. It refutes a uniform statement of the
form:

```text
every fixed child zero set Z pays at least the random exponent,
or at least the all-paired envelope exponent while being counted with arbitrary-set entropy.
```

The dominant bad ledger row is only dangerous if many arbitrary `Z=P union C` sets of size `1024`
can behave like deeply all-paired sets. They cannot. The all-paired degeneration is a shape event
with much smaller entropy:

```text
Z all-paired at one level:
  choose z/2 child positions, not z arbitrary parent positions.

Z all-paired for m levels:
  choose z/2^m lower positions.
```

So the subcode-zero theorem must be shape-sensitive, just like the line-zero/rank-tail recurrence.

## The Correct Next Theorem Target

Replace the fixed-set subcode-zero bound:

```text
S_d(h,z) <= binom(n,z) q^{-h(z-k+h)}
```

by a recursive shape sum:

```text
S_d(h,z)
  <= sum over top split shapes
       split_entropy(shape)
       * child_subcode_tail(shape)
       * root/singleton repair factors.
```

The all-paired branch should contribute roughly:

```text
S_{d-1}(ceil(h/2), z/2)
```

with all-paired split entropy, not arbitrary `binom(n,z)` entropy.

Thus the next useful diagnostic is not another uniform envelope. It is a paired-shape ledger that
compares:

```text
arbitrary-set entropy + random exponent
vs
all-paired-shape entropy + compressed child exponent.
```

## Current Verdict

The common-zero replacement remains the best route, but the child subcode-zero ingredient must be
recursive and shape-sensitive. A random fixed-set theorem is false for RFC, and the paired-envelope
stress row shows exactly where it fails.
