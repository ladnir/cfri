# Canonical h=1 Bucket And Complement

Status: Phase-2 canonical-bucket theorem and complement audit.

## Purpose

The h=1 flat-excess bucket is extremely cheap after explicit witness counting, but that alone does
not improve the original proof. To get an honest proof upgrade, the refined bucket must be part of
a disjoint decomposition of the original bad-root event.

This note defines the canonical h=1 bucket and explains what remains in the complement.

## Original Bad-Root Witness

Fix a paired core `P` and singleton set `T`. Let:

```text
K_P = ker(ev_P)
D   = dim K_P.
```

A root-line repair failure has a nonzero projective witness:

```text
(x,y) in K_P plus K_P
```

such that every singleton row `i in T` satisfies:

```text
ell_i(x) + alpha_i ell_i(y) = 0.
```

Define the full common-zero set of the witness:

```text
C(x,y) = { i in T : ell_i(x)=0 and ell_i(y)=0 }.
```

Rows in `C(x,y)` leave their root variables free. Rows outside `C(x,y)` force at most one root
value.

## Canonical h=1 Bucket

The canonical h=1 bucket is:

```text
B_1 = { bad-root witnesses (x,y) : rank(C(x,y)) = D-1 }.
```

This is canonical because it uses the full common-zero set of the actual bad witness. No arbitrary
flat witness `A subset T` is selected.

The complement is:

```text
B_rest = { bad-root witnesses (x,y) : rank(C(x,y)) <= D-2 }.
```

The two buckets are disjoint and cover the bad-root event, since `rank(C(x,y))=D` would force
`x=y=0`.

## h=1 Bucket Count

If:

```text
rank(C)=D-1,
```

then:

```text
dim ker(ev_C|K_P) = 1.
```

Thus there is a nonzero child line vanishing on:

```text
P union C.
```

For `|C|=a`, the h=1 bucket is bounded by:

```text
choose P, C, T\C and singleton sides
times
child line-zero on P union C
times
root assignments outside C.
```

At the q-exponent level this is:

```text
child line-zero exponent:  |P| + a - k_child + 1
root residual exponent:   t - a - 1
combined exponent:        |P| + t - k_child.
```

For the target profile:

```text
p = 137
t = 1845
k_child = 1024
```

the combined exponent is:

```text
p+t-k_child = 958.
```

This agrees with the previous flat-excess parameterization:

```text
a = 2(D-1)+F
line-zero exponent = 886+F
root residual      = 72-F
total              = 958.
```

## Ledger Result

The profile ledger:

```text
scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py
```

reports, for the target profile:

```text
old whole-profile one-minor row = 8973.439028 bits

h=1 bucket F=1:  -113097.994523 bits
h=1 bucket F=16: -113169.684015 bits
h=1 bucket F=72: -113521.504357 bits
```

The h=1 bucket is therefore not the obstruction.

## Complement Audit

The complement is not automatically better than the original proof.

For a common-zero set `C` with:

```text
r = rank(C) <= D-2,
```

the incidence exponent is:

```text
t - |C| - 2D + 2r + 1.
```

Hall-OK gives:

```text
|C| <= t - 2D + 2r.
```

Therefore Hall-OK implies only:

```text
t - |C| - 2D + 2r + 1 >= 1.
```

This lower bound can still be tight for `r=D-2` if:

```text
|C| = t - 4.
```

So removing h=1 witnesses does not by itself improve the original one-q-factor local repair bound.

## Verdict

Phase 2 gives a clean disjoint bucket:

```text
bad-root event = canonical h=1 bucket disjoint union no-h1 complement.
```

The refined bucket is a huge local win. But the complement remains as hard as the original proof
under Hall-only information:

```text
no h=1 complement: no gain under Hall-only assumptions.
```

Therefore h=1 alone is not a finite-certificate upgrade over the original proof.

## h=2 And h=3 Ledger Check

The ledger now supports arbitrary `h` buckets and summary mode. Running:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --h 1 --h 2 --h 3 \
  --flat-excess 1 --flat-excess 16 --flat-excess 72
```

shows that `h=2` and `h=3` buckets are even cheaper than `h=1`.

Representative rows:

```text
h=2,F=1:
  total_q_exp = 1843
  refined_bucket_log2 = -226368.810662
  gain_vs_old_same_bucket = 235785.792790 bits

h=3,F=1:
  total_q_exp = 2726
  refined_bucket_log2 = -339383.707526
  gain_vs_old_same_bucket = 348809.792790 bits
```

So the high-rank buckets are not the finite-accounting problem.

The complement problem also persists. After removing `h=1,2,3`, Hall-only information still allows
a lower-rank common-zero flat with incidence exponent one. More generally, removing any fixed
finite set of high-rank buckets does not improve the complement unless we prove an additional
flat-sparsity or low-rank common-zero theorem.

## Full h Sweep

The full target sweep:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --all-h --min-flat-excess 1 --max-flat-excess 72 \
  --summary-only --top 12
```

scans:

```text
h = 1..887
F = 1..72
```

for `63864` valid buckets. The dominant refined bucket is:

```text
h = 1
F = 1
r = 886
a = 1773
z_child = 1910
total_q_exp = 958
refined_log2 = -113097.994523.
```

The top 12 buckets are all:

```text
h=1, F=1..12.
```

The Hall-tight boundary check:

```text
--all-h --min-flat-excess 71 --max-flat-excess 71 --summary-only
```

also has dominant bucket:

```text
h = 1
F = 71
refined_log2 = -113511.655734.
```

and higher `h` buckets rapidly become much cheaper.

Thus the refined common-zero buckets themselves are uniformly harmless at the target profile under
the subcode-zero scale. The obstacle is not finding a bad `h` bucket; it is proving a complement
or structural theorem that lets us replace the original whole-profile one-minor row by the refined
bucket sum plus a genuinely smaller remainder.

The stronger formulation is to remove the complement entirely by summing every canonical
common-zero bucket. That replacement theorem is recorded in:

```text
rfc_common_zero_subcode_replacement_theorem.md
```

The all-bucket ledger gives:

```text
refined_bucket_logsum = -113097.934786
```

for all `h=1..887` and `F=1..72`, so the full replacement remains numerically dominated by
`h=1,F=1`.

## Next Incremental Target

The next honest refinement must attack the complement, not another high-rank bucket in isolation.
The high-rank bucket ledger suggests:

```text
h=1,2,3,... buckets are individually cheap.
```

But the complement remains Hall-tight unless the refinement covers all ranks that can saturate:

```text
|C| = t - 2D + 2 rank(C).
```

Thus the next phase should be one of:

```text
1. a full canonical common-zero rank ledger over all h;
2. a structural theorem ruling out Hall-tight low-rank common-zero flats for RFC survivor profiles;
3. a proof that Hall-tight low-rank flats themselves are cheap when charged by child subcode-zero
   events.
```

The full ledger says the first and third options are numerically plausible. The second option is
what would turn the local bucket wins into a genuine incremental upgrade over the original proof.
This is essentially the refined, honest version of flat-excess control.
