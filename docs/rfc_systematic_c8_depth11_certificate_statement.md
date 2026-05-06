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

The same script now supports an additional charged-defect counting overhead:

```text
--charge-overhead-log2 B
```

For:

```text
B in {16,32,64}
```

the total remains:

```text
log2 failure bound = -99.60768258.
```

Artifacts:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead16.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead32.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64.csv
```

Additional stress artifacts:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead104.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead108.csv
```

give:

```text
B=104: log2 failure bound = -98.65570921
B=108: log2 failure bound = -67.37561327
```

This means the final proof can use a charged-tree count with substantial per-defect overhead; it
does not need the exact strongest binomial classification for every positive-defect support.

The counted charged-tree theorem is tracked in:

```text
docs/rfc_counted_charged_tree_theorem.md
```

A virtual-core-with-holes fallback was also stress-tested with `B=64`:

```text
H=1:  log2 failure bound = -83.06077305
H=4:  log2 failure bound = -42.01824475
H=5:  log2 failure bound = -29.84764089
H=8:  log2 failure bound =   3.75441819
H=16: log2 failure bound =  79.12109543
```

This confirms very small hole counts are affordable, but the proof should still target actual core
preservation. Under the present crude hole count, five global holes remain safe and eight do not.

A stronger and more relevant fallback was then checked: allow virtual core holes only when they are
coupled to output defect:

```text
holes <= extra.
```

The artifact:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_coupled_extra.csv
```

gives:

```text
log2 failure bound = -99.60768253
```

So the certificate does not need literal actual-core preservation if every virtual hole is charged
to a real extra output leaf. This is now the better target for unbalanced splits.

Thus, within the corrected matched-plus-extra model:

```text
Pr[d_sys < 12384] <= 2^-99.60768258.
```

Within the defect-coupled virtual-core model, the corresponding audited number is:

```text
Pr[d_sys < 12384] <= 2^-99.60768253.
```

## Lemma Status

The proof stack is:

```text
one-copy uncertainty:             drafted
exact matched construction:       drafted
no early gluing:                  local rational-function proof written
exact stability theorem:          drafted from the previous lemmas
near-core dimension bound:        direct stride-class proof written
near-stability/counting:          local defect decomposition plus exact depth-4 evidence
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

The core-containment artifacts:

```text
docs/rfc_core_containment_depth4_m4_w5.csv
docs/rfc_core_containment_depth4_m2_w9.csv
docs/rfc_core_containment_depth4_m2_w10.csv
docs/rfc_core_containment_depth4_m4_e2_model.csv
docs/rfc_core_containment_depth4_m4_e3_model.csv
docs/rfc_core_containment_depth4_m4_e4_model.csv
```

show every saved/generated near pair contains at least one actual matched core. The generated
`m=4,e=4` model has `48` pairs with two contained cores, exactly matching the complete-extra-stride
dimension-growth cases.

The main remaining proof obligation is now the unbalanced-row-split part of core preservation:

```text
wt(x)=m, wt(Ax)<=k/m+e
  => supp(Ax) contains an actual matched stride core C of size k/m
```

The local accounting for this theorem is now:

```text
parent defect >= child defects
               + row-split charge
               + overlap charge
               + cancellation charge.
```

After such a core is selected, the counted charged-tree theorem labels the `e` remaining output
leaves with bounded charge records. The proof no longer needs exact near-extremizer classification,
but it still needs the core to be contained in the actual sparse output support.

The one-child and balanced two-child core-preservation cases are now reduced:

```text
one-child:          child core lifts directly;
balanced two-child: child core lifts through one surviving parent residue by no-early-gluing.
```

The open case is:

```text
unbalanced two-child split, paid by integer split defect.
```

The near-core dimension lemma is:

```text
dim {x supported on R : supp(Ax) subset C union E}
  <= 1 + floor(|E|/|C|)
```

for matched `R,C` and arbitrary extra set `E`. It follows by descending to the live block: a local
coordinate survives the global zero constraints exactly when its whole stride class is allowed.

These are tracked separately in:

```text
docs/rfc_near_defect_charge_injection.md
docs/rfc_core_preservation_hall_condition.md
docs/rfc_counted_charged_tree_theorem.md
docs/rfc_near_core_containment_theorem.md
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
