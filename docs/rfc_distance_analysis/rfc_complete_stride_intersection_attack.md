# RFC Complete-Stride Two-Copy Intersection Attack

Scope: original non-systematic RFC only.

This note defines the falsification criteria for the next gate after the clean large-prime
size-8 Omega scan:

```text
GF(65537), seed1
total_profiles = 51480
complete_stride_dim_ge2 = 24
nonstride_dim_ge2 = 0
max_complete_dim = 2
max_nonstride_dim = 1
status = ok
```

That scan says complete-stride supports are the only depth-4, `m=4`, size-8 local dimension-growth
exception over a generic-sized field.  The remaining question is whether those exact local flags
intersect non-generically across independent RFC copies.

## Local Object

Use the depth-4 complete-stride target:

```text
B_b = {4b, 4b+1, 4b+2, 4b+3},  b in {0,1,2,3}
C_j = {j, j+4, j+8, j+12},      j in {0,1,2,3}
Omega = C_i union C_j,          i < j
```

For each unmarked support pair `(B_b, Omega)`:

```text
dim K_Omega = 2
K_Omega = ell_i direct_sum ell_j
```

Emit two ordered flags:

```text
ell_i <= K_Omega, visible quotient on C_j
ell_j <= K_Omega, visible quotient on C_i
```

Expected local counts:

```text
24 unmarked supports
48 ordered flags
```

For the two-copy attack, each flag is a pair:

```text
L <= V
dim L = 1
dim V = 2
```

inside the same ambient message space.

## Intersection Variables

For two independent RFC copies, and flags:

```text
L_1 <= V_1
L_2 <= V_2
```

record:

```text
vv_dim = dim(V_1 cap V_2)
ll_dim = dim(L_1 cap L_2)
lv12_dim = dim(L_1 cap V_2)
lv21_dim = dim(L_2 cap V_1)
```

Generic vector-space baselines in ambient dimension `k` are:

```text
vv_generic = max(0, dim V_1 + dim V_2 - k)
ll_generic = max(0, dim L_1 + dim L_2 - k)
lv_generic = max(0, dim L + dim V - k)
```

At depth 4, `k=16`, so all three generic values are `0`.

Define excesses:

```text
v_excess = vv_dim - vv_generic
l_excess = ll_dim - ll_generic
lv12_excess = lv12_dim - lv_generic
lv21_excess = lv21_dim - lv_generic
```

For this gate, any positive excess over a large field is serious.

## Real Counter-Signals

### 1. Plane-Plane Excess

Counter-signal:

```text
v_excess >= 1
```

Meaning:

```text
Two complete-stride 2-planes from independent copies share a nonzero message direction.
```

Why this matters:

```text
One reusable q-dimension is 128 bits at q=2^128.
```

That is larger than the current exact-support `e=71` slack from the scalar stress model.

### 2. Repeated Kernel Line

Counter-signal:

```text
l_excess >= 1
```

Meaning:

```text
The same complete-stride kernel line appears in two independent copies.
```

This is stronger than plane-plane excess.  It gives a direct repeated bad message line and should
be treated as a high-priority falsification signal.

### 3. Line-In-Plane Incidence

Counter-signal:

```text
lv12_excess >= 1 or lv21_excess >= 1
```

Meaning:

```text
A kernel line from one copy lies inside a complete-stride 2-plane from another copy.
```

This may be enough to build active-copy multiplicity even if exact kernel lines do not repeat.

### 4. Support-Pair Correlation

Counter-signal:

```text
positive excess concentrated on specific row-block / stride-pair profiles
```

Record the profile:

```text
copy_id
row block B_b
unordered stride pair {i,j}
ordered kernel stride i or j
vv_dim, ll_dim, lv12_dim, lv21_dim
```

A single accidental incidence over a large prime is already unlikely.  A profile-stable incidence
across support shapes, seeds, or copy pairs is structural.

### 5. Active-Copy Multiplicity

Counter-signal:

```text
one canonical line or plane participates in complete-stride flags for r >= 2 copies
```

Report histograms:

```text
line_active_copy_count
plane_active_copy_count
support_profile_active_copy_count
```

This is the production-relevant version of the attack.  If the same line survives `r` copies, the
generic exponent should pay roughly one ambient intersection cost per additional copy.  Any
systematic failure of that payment is a real lower-bound obstruction.

## Expected Clean Result

A clean two-copy complete-stride scan should report:

```text
max_v_excess = 0
max_l_excess = 0
max_lv_excess = 0
no repeated L lines
no repeated V planes
no support-pair profile with positive excess
```

Interpretation:

```text
The complete-stride exception is local to one copy.
It does not create multi-copy amplification.
The exact-support scalar model remains the active lower-bound model.
```

This would not prove the distance certificate, but it would close the current complete-stride
intersection gate.

## Dirty Result Interpretation

If any excess is positive over `GF(65537)`:

```text
1. Emit the first examples.
2. Resample a second large-prime seed.
3. Check whether the same support profile, not necessarily the same literal vector, repeats.
```

Classify:

```text
one-off determinant zero:
  disappears under seed/prime resampling.

profile-stable excess:
  persists by row-block / stride-pair type.
  This is a proof blocker.

literal repeated line:
  same canonical line across copies.
  This is the strongest counter-signal.
```

## Implementation Output Requirements

For each copy pair and flag pair, output only positive-excess rows plus aggregate maxima:

```text
copy_a
copy_b
row_block_a
row_block_b
stride_pair_a
stride_pair_b
kernel_stride_a
kernel_stride_b
vv_dim
ll_dim
lv12_dim
lv21_dim
v_excess
l_excess
lv12_excess
lv21_excess
canonical_L_hash_a
canonical_L_hash_b
canonical_V_hash_a
canonical_V_hash_b
```

Aggregate:

```text
total_flag_pairs
positive_v_excess_count
positive_l_excess_count
positive_lv_excess_count
max_v_excess
max_l_excess
max_lv_excess
line_active_copy_histogram
plane_active_copy_histogram
```

## If This Gate Passes

If the two-copy complete-stride scan is clean, re-rank the next falsification targets as:

1. **Longer nested flags along the paired-compression spine.**

   Test chains:

   ```text
   V_0 >= V_1 >= V_2
   ```

   where each local step is complete-stride-controlled, but the chain might have more multiplicity
   than the two-layer recurrence counts.

2. **Dense connected tau=2 profiles with generic-kernel endpoint.**

   This remains the main local-algebra risk unrelated to complete-stride size-8 Omega.

3. **Production-depth scalar stress with exact constants.**

   Revisit the exact-support `e=71` scalar row once finite orientation, polynomial, and flag-lift
   constants are explicit.

## Feedback For Proof Agent

The proof claim most exposed by this gate is:

```text
Independent complete-stride flags intersect generically after exact support de-duplication.
```

If the implementation finds:

```text
v_excess >= 1
l_excess >= 1
or repeated active-copy L lines
```

then the proof needs an additional structural charge for complete-stride flags.  Exact support
counting alone would not be enough.
