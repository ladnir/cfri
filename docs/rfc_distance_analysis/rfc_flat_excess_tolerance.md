# RFC Flat-Excess Tolerance

Status: global-tolerance checkpoint for the corrected surplus repair exponent.

The incidence stratification corrected the desired local exponent to:

```text
t - 2D + 1 - flat_excess.
```

This note asks how much flat excess the near-MDS target can tolerate in the top-profile stress
model.

## Calculator

Script:

```text
scripts/rfc_distance_analysis/rfc_one_step_repair_tolerance.py
```

Model:

```text
--model flat_corrected_surplus --flat-excess F
```

This subtracts `F` q-factors from the ideal surplus exponent:

```text
q^{-(t-2D+1-F)}.
```

The calculation is still only a top-profile tolerance check, not a full certificate.

## Target Config

```text
depth = 11
expansion = 8
k = 2048
n = 16384
q = 2^128
target expected bad top-profile count <= 1/2
```

Sequential sweep:

```text
python -B scripts/rfc_distance_analysis/rfc_one_step_repair_tolerance.py \
  --depth 11 --expansion 8 --q-log2 128 \
  --model flat_corrected_surplus --flat-excess F \
  --min-excess 68 --max-excess 96
```

## Result

| flat_excess F | crossing excess e | distance lower bound | log2 bad at crossing |
|---:|---:|---:|---:|
| 0 | 71 | 14266 | -119.677892 |
| 1 | 72 | 14265 | -116.927548 |
| 2 | 73 | 14264 | -114.177986 |
| 3 | 74 | 14263 | -111.429205 |
| 4 | 75 | 14262 | -108.681205 |
| 5 | 76 | 14261 | -105.933985 |
| 6 | 77 | 14260 | -103.187546 |
| 7 | 78 | 14259 | -100.441887 |
| 8 | 79 | 14258 | -97.697007 |
| 9 | 80 | 14257 | -94.952907 |
| 10 | 81 | 14256 | -92.209585 |
| 11 | 82 | 14255 | -89.467043 |
| 12 | 83 | 14254 | -86.725278 |
| 13 | 84 | 14253 | -83.984292 |
| 14 | 85 | 14252 | -81.244084 |
| 15 | 86 | 14251 | -78.504653 |
| 16 | 87 | 14250 | -75.765999 |

## Interpretation

For the dominant top-profile stress, every unit of flat excess costs almost exactly one unit of
distance excess:

```text
e_crossing ~= 71 + flat_excess.
```

So exact `e=71` needs `flat_excess=0` in the dominant profile. But the broader near-MDS goal is
robust to small constant flat excess. For example, allowing `flat_excess <= 16` still gives:

```text
distance >= 14250
```

at success probability at least `1/2` in this top-profile model. The MDS distance for these
parameters is:

```text
n-k+1 = 14337.
```

So even `F=16` is still only `87` below MDS.

## Meaning

This makes the next proof target sharper:

```text
We do not need flat_excess=0 everywhere.
We need flat_excess bounded by a small constant, or profiles with larger flat_excess charged by
their combinatorial scarcity.
```

The fixed-survivor route remains viable if we can prove one of:

```text
1. dominant profiles have flat_excess = O(1);
2. the number of profiles with flat_excess = F pays roughly q^{-F} elsewhere;
3. the recurrence can carry flat_excess as a state and show e increases by about F.
```

This is better than the previous all-or-nothing blocker.
