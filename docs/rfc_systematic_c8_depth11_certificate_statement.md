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
2. at least e+1 zeros across the remaining six parity copies.
```

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

Thus, within the matched-plus-extra model:

```text
Pr[d_sys < 12384] <= 2^-97.14825096.
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

The main remaining proof obligation is the near-stability theorem:

```text
wt(x)=m, wt(Ax)<=k/m+e
  => support/output pair contains a matched stride core plus e extras
```

or a sufficiently close upper count. The exact depth-4 checks match the strongest form:

```text
k * binom(k-k/m, e).
```

## Interpretation

This would certify the all-level systematic construction essentially at its known collapse ceiling:

```text
delta_sys = 0.755859375
```

for the depth-11, c=8 parameter point, with about `97` bits of modeled large-field slack. The
asymptotic target remains:

```text
delta_sys -> 1 - 2/c = 0.75.
```
