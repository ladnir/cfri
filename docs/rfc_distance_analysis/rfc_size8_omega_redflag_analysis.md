# RFC Size-8 Omega Red-Flag Analysis

Scope: original non-systematic RFC only.

The implementation scan reported, over `GF(5)`, seed `1`:

```text
depth = 4
m = 4
row blocks = 4
Omega size = 8
total profiles = 51480 = 4 * binom(16,8)
complete-stride dim>=2 = 24 / 24
non-stride dim>=2 = 8534 / 51456
max non-stride dimension = 4
first examples include:
  row_block = 0:1:2:3
  Omega = 0:1:2:3:4:11:12:15
```

This was a real warning about the scan configuration, but the follow-up large-prime scan resolves
it for now.  The old artifacts and the first scan were measuring different universes, and the first
scan was also over a very small field where the RFC fold has many degenerate local multipliers.

Follow-up implementation result:

```text
prime = 65537
seed = 1
total_profiles = 51480
complete_stride_dim_ge2 = 24
nonstride_dim_ge2 = 0
max_complete_dim = 2
max_nonstride_dim = 1
status = ok
```

This is the expected generic-field outcome.  The GF(5) red flag should now be treated as likely
small-field degeneration, not production-field evidence of broader dimension growth.

## Why The Old Artifacts Saw Only 48

The relevant old files are:

```text
docs/rfc_distance_analysis/rfc_uncertainty_near_model_pairs_depth4_m4_e4.csv
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
docs/rfc_distance_analysis/rfc_near_core_stride_dimension_class_depth4_m4_e4.csv
docs/rfc_distance_analysis/rfc_core_containment_depth4_m4_e4_model.csv
```

They are not all-Omega scans.  The big pair CSV has:

```text
pairs = 7920 = 4 row blocks * 4 marked stride cores * binom(12,4) extras.
```

Every row was generated as:

```text
matched stride core C_j plus 4 extra output positions.
```

So every row has:

```text
contains_matched_core = 1
```

by construction.  It could never see a support such as:

```text
Omega = 0:1:2:3:4:11:12:15
```

because that set contains no complete stride class modulo `4`:

```text
C_0 = 0:4:8:12    missing 8
C_1 = 1:5:9:13    missing 5,9,13
C_2 = 2:6:10:14   missing 6,10,14
C_3 = 3:7:11:15   missing 7
```

The old kernel summary:

```text
kernel_dim = 1: 7872 marked rows
kernel_dim = 2:   48 marked rows
```

therefore means:

```text
Among matched-core-plus-4-extra rows over the large-prime diagnostic,
only the rows with a second complete stride core had dimension 2.
```

It does not say:

```text
Among all size-8 Omega, only complete-stride sets have dimension >= 2.
```

The `48` count is also marked.  It corresponds to:

```text
24 unmarked supports = 4 row blocks * binom(4 stride classes, 2)
48 ordered/marked flags = 24 supports * 2 contained stride cores.
```

## Why The New GF(5) Scan Is Suspicious

The RFC generator samples `T` uniformly nonzero.  The local fold used by the diagnostic has entries
including:

```text
1 - T
-T
T
T + 1
```

The one-child descent argument behind the matched-kernel/complete-stride picture assumes the
relevant lift multipliers are nonzero.  Over large fields, the exceptional values are rare:

```text
T = 1     makes 1-T = 0
T = -1    makes T+1 = 0
```

Over `GF(5)`, these are not rare.  A tiny seed check for `GF(5)`, seed `1`, gives:

```text
round 1: [2]                         T=1: 0, T=-1: 0
round 2: [1, 3]                      T=1: 1, T=-1: 0
round 3: [1, 4, 4, 4]                T=1: 1, T=-1: 3
round 4: [4, 2, 1, 4, 1, 4, 4, 1]    T=1: 3, T=-1: 4
```

So seed `1` has many zero lift multipliers.  This can create support collapses that are impossible
under the generic/nonzero-multiplier descent model.  In particular, `max non-stride dimension = 4`
is too extreme to treat as generic evidence: it means an entire four-dimensional row-block subspace
is supported inside some size-8 Omega.  That is exactly the kind of behavior zero local multipliers
can create.

## Current Interpretation

There are two separate effects:

```text
1. Scope mismatch:
   Old artifacts only scanned matched-core-plus-extra supports.
   The new scan scans all size-8 Omega.

2. Small-field degeneracy:
   The new scan is over GF(5), seed1, with many T in {1,-1}.
   These values violate the nonzero-multiplier condition used by the complete-stride heuristic.
```

Therefore the original GF(5) red flag is **not evidence of real broader production-field dimension
growth at this point**.  It is best interpreted as:

```text
GF(5) all-Omega scans are contaminated by local zero-multiplier degeneracies unless conditioned or
resampled.
```

However, the red flag is still useful because it exposed a blind spot in the old artifacts:

```text
the old 48-row result only controlled the matched-core near-model universe.
```

The large-prime all-Omega check has now supplied that generic-field sanity check at depth 4,
`m=4`, size-8 Omega.

## What Would Be Real

A genuine production-relevant counter-signal would be:

```text
Over a large prime such as 65537, or over many samples conditioned on T notin {1,-1},
there exists a non-stride Omega of size 8 with kernel_dim >= 2.
```

An even stronger signal would be:

```text
max non-stride dimension > 1
```

persisting over large-prime samples.  That would contradict the current near-kernel line:

```text
dimension growth comes only from complete extra stride classes.
```

The GF(65537), seed1 scan found no such example:

```text
nonstride_dim_ge2 = 0
max_nonstride_dim = 1
```

So the complete-stride charge survives this checkpoint.

## Recommended Follow-Up For Implementation

Do not compare the old 7920-row matched-core CSV directly against the new 51480-row all-Omega scan.
The first large-prime follow-up has now passed.  Remaining optional checks are:

```text
1. Repeat over GF(5) but reject/resample any local T in {1,-1}.
   Expected if the issue is zero-multiplier degeneracy:
     non-stride dim>=2 collapses dramatically, ideally to 0.

2. Repeat the large-prime all-Omega scan for a few more seeds only if we want extra diagnostic
   confidence.

3. If non-stride dim>=2 ever appears over large prime, emit the first 20 examples with:
     row_block
     Omega
     kernel_dim
     rank of outside-Omega restriction
     whether Omega contains any complete stride class
     local T values on the ancestor path of the row block.
```

The implementation worker can now treat the old complete-stride target generator as the correct
depth-4 size-8 local exception for generic fields.

## Re-Ranked Falsification Targets

After the GF(65537) all-Omega scan, complete-stride dimension growth survives as the only local
`m=4`, size-8 generic exception.  The next falsification targets should be:

1. **Two-copy intersections of exact complete-stride flags.**

   The local single-copy exception is controlled:

   ```text
   24 unmarked supports, 48 ordered flags, max dimension 2.
   ```

   The next question is whether these exact flags intersect generically across independent RFC
   copies.  Counter-signal:

   ```text
   observed common flag/intersection dimension > generic flag intersection dimension
   ```

   by even one `q`-dimension in a reusable profile.

2. **Longer nested flags along the paired-compression spine.**

   Since single-level size-8 Omega is clean, the remaining hidden-kernel risk is a chain:

   ```text
   V_0 >= V_1 >= V_2
   ```

   where each local step is complete-stride-controlled but the chain has more multiplicity than
   the two-layer flag recurrence counts.

3. **Dense connected tau=2 profiles with generic-kernel endpoint.**

   This remains a separate local-algebra risk.  It is no longer supported by the size-8 Omega red
   flag, but it is still the place where the old component-only theorem failed.

## Manager Summary

Updated judgment:

```text
Resolved cause: GF(5), seed1 zero-multiplier degeneration plus broader all-Omega scope.
Large-prime status: clean at GF(65537), seed1.
Complete-stride checkpoint: survives.
Next risk: multi-copy/longer-flag behavior, not single-copy non-stride size-8 Omega.
```

The original blocker is resolved for now.  It should be reopened only if a large-prime all-Omega
scan finds non-stride `dim>=2` or if two-copy complete-stride flag intersections exceed the generic
flag exponent.
