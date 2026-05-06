# Systematic RFC c=8 Depth-11 Certificate Statement

This note states the current certificate target for the all-level systematic RFC at:

```text
c = 8
c_p = 7
d = 11
k = 2048
N = 16384
q = 2^128.
```

## Distance Target

The generalized collapse family gives the upper bound:

```text
best live size m = 32 or 64
m + k/m = 96
d_sys <= (c_p - 1)k + 96
      = 6*2048 + 96
      = 12384.
```

Relative distance:

```text
12384 / 16384 = 0.755859375.
```

So the best possible certificate for this exact all-level systematic structure is:

```text
d_sys >= 12384
```

up to finite-field failure probability.

## Current Conditional Certificate

Under the exact stability theorem and the matched-plus-extra near-stability model, any codeword
with weight below `12384` must have:

```text
1. one parity copy with output support k/m + e around a matched stride core;
2. enough zeros across the remaining six parity copies to fall below the target.
```

At the optimal collapse live sizes `m=32` and `m=64`, this is `e+1` zeros. For other `m`, the
required number is larger.

The matched-plus-extra support-pair count is:

```text
7 * k * binom(k-k/m, e),
```

where the factor `7` chooses the sparse parity copy.

The aggregate remaining-copy zero tail is bounded by:

```text
binom(6k, e+1) q^{-(e+1)}.
```

Summing over:

```text
m in {2,4,...,2048}
0 <= e <= 128
```

gives:

```text
log2 failure bound = -97.14825096.
```

Artifact:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128.csv
```

The conservative variant using cumulative near counts:

```text
k * sum_{i<=e} binom(k-k/m, i)
```

is:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_cumulative.csv
```

and gives the same displayed total:

```text
log2 failure bound = -97.14825096.
```

A corrected version also charges:

```text
needed zeros = max(1, m + k/m + e - 96 + 1)
kernel dimension factor <= q^floor(e/(k/m))
```

where `96` is the collapse sparse-side target `m+k/m` at `m=32` or `m=64`. This is evaluated in:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_cumulative_stride_dim.csv
```

and gives:

```text
log2 failure bound = -99.60768258.
```

Thus, within the corrected matched-plus-extra model:

```text
Pr[d_sys < 12384] <= 2^-99.60768258.
```

## Lemma Status

The proof stack is:

```text
one-copy uncertainty:             drafted
exact matched construction:       drafted
no early gluing:                  local rational-function proof written
exact stability theorem:          drafted from the previous lemmas
near-stability/counting:          strategy plus exact depth-4 evidence
remaining-copy zero tail:         union bound written and tabulated
```

The near-stability evidence now includes:

```text
depth 4, m=4, e=1: exhaustive scan gives 192 = 16*binom(12,1)
depth 4, m=2, e=1: exhaustive scan gives 128 = 16*binom(8,1)
depth 4, m=2, e=2: exhaustive scan gives 448 = 16*binom(8,2)
depth 4, m=4, e=2: generated matched-core model gives 1056 = 16*binom(12,2)
depth 4, m=4, e=3: generated matched-core model gives 3520 = 16*binom(12,3)
depth 4, m=4, e=4: generated matched-core model gives 7920 = 16*binom(12,4)
```

The `e=4` generated model is the first point where kernel dimension grows:

```text
7872 pairs have kernel_dim = 1
48 pairs have kernel_dim = 2
```

Those dimension-`2` cases occur when the extras contain a complete additional stride class.
The classifier artifacts:

```text
docs/rfc_near_core_stride_dimension_class_depth4_m4_e2.csv
docs/rfc_near_core_stride_dimension_class_depth4_m4_e3.csv
docs/rfc_near_core_stride_dimension_class_depth4_m4_e4.csv
```

show `kernel_dim - 1` equals the number of complete extra stride classes for all generated
`m=4`, `e=2,3,4` pairs.

The main remaining proof obligation is the near-stability theorem:

```text
wt(x)=m, wt(Ax)<=k/m+e
  => support/output pair contains a matched stride core plus e extras
```

plus the near-core dimension lemma:

```text
dim {x supported on R : supp(Ax) subset C union E}
  <= 1 + floor(|E|/|C|)
```

for matched `R,C` and arbitrary extra set `E`.

These are tracked separately in:

```text
docs/rfc_near_defect_charge_injection.md
docs/rfc_near_kernel_line_lemma.md
```

The exact depth-4 checks match the strongest count:

```text
k * binom(k-k/m, e).
```

## Interpretation

This would certify the all-level systematic construction essentially at its known collapse ceiling:

```text
delta_sys = 0.755859375
```

for the depth-11, c=8 parameter point, with about `99` bits of modeled large-field slack. The
asymptotic target remains:

```text
delta_sys -> 1 - 2/c = 0.75.
```
