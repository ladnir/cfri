# RFC Flag-Intersection Enumerator Status

Scope: original non-systematic RFC only.

## Script

```text
scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py
```

The script is a bounded exact/small-depth diagnostic for nested child flags:

```text
L <= V
V zero on P union (S \ A)
L zero on P union S
```

It emits the proof-lane transition variables where available:

```text
h,t,z,p,s,a,tau,kappa,r1,r0,z_V,z_L,delta,comp,g
```

and two-copy/three-copy intersection diagnostics:

```text
common_v_dim, common_l_dim,
generic_v_dim, generic_l_dim,
v_excess, l_excess.
```

## Algorithm

For each independent RFC copy over a small prime:

1. Generate the original non-systematic RFC generator with an independent seed.
2. Enumerate exact split profiles `(P,S,A)` from either all inner zero sets, sampled inner zero
   sets, or a single explicit `--inner-columns` support.
3. Compute the zero kernels:

   ```text
   K_inner = ker(G_{P union S})
   K_outer = ker(G_{P union (S \ A)})
   ```

4. In default mode, treat `K_inner <= K_outer` as the maximal zero-induced flag.
5. In `--enumerate-subflags` mode, enumerate exact `r0/r1` subflags

   ```text
   L <= K_inner,
   V <= K_outer,
   L <= V,
   dim L = r0,
   dim V = r1.
   ```

   This is guarded by `--max-subflags-per-profile`; oversized Gaussian subflag profiles are counted
   as skipped instead of being enumerated.
6. Canonicalize every `L` and `V` by RREF bases, so repeated certificates for the same transition
   shape and same flag are counted once.
7. Compute local support metadata:

   ```text
   delta = rank(S) - rank(S \ A)
   comp  = matroid component count on A
   g     = exact two-copy/root-line generic-kernel dimension for U_A
   ```

8. Intersect exact flags across independent copies and compare observed dimensions against generic
   linear-subspace intersection dimensions.

The targeted complete-stride mode now has an earlier combinatorial acceptance gate:

```text
--targeted-near-stride-w --targeted-combinatorial-only --child-depth 4
```

This builds only:

```text
B_b = {4b,4b+1,4b+2,4b+3}
C_j = {j,j+4,j+8,j+12}
Omega = C_i union C_j
```

and emits the 48 ordered direct-support flags:

```text
ell_i <= span(ell_i, ell_j), visible quotient on C_j
ell_j <= span(ell_i, ell_j), visible quotient on C_i
```

It stops before field-algebra enumeration or multi-copy intersections.

The direct finite-field gate is:

```text
--targeted-near-stride-w --targeted-direct-check-only --child-depth 4 --prime 5
```

For all 24 supports it computes:

```text
K_Omega = {x supported on B_b : Ax is zero outside Omega}
ell_i   = {x supported on B_b : Ax is zero outside C_i}
ell_j   = {x supported on B_b : Ax is zero outside C_j}
```

and verifies:

```text
dim K_Omega = 2
dim ell_i = 1
dim ell_j = 1
K_Omega = ell_i direct_sum ell_j
```

It also canonicalizes the 48 ordered flags over the field and checks that no two collapse.

The cheap non-stride size-8 scan is:

```text
--targeted-near-stride-w --targeted-size8-omega-scan-only --child-depth 4 --prime 5
```

It scans all `4 * binom(16,8) = 51480` pairs `(B_b, Omega)`, separates complete-stride
`Omega = C_i union C_j` from all other size-8 supports, and reports only a compact summary plus
failure examples. The classified version also counts:

```text
contained_stride_line_count:
  number of complete C_j stride classes with dim ell_j = 1 and C_j subset Omega.

contained_kernel_line_count:
  number of canonical projective message lines in B_b whose output support is contained in Omega.
```

Stride metadata is diagnostic only:

```text
max_visible_complete_path_strides,
max_inner_complete_path_strides,
max_visible_full_sibling_pairs,
max_inner_full_sibling_pairs.
```

It is not multiplied as event count.

## Smoke Results

Complete-stride combinatorial acceptance gate:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --targeted-combinatorial-only \
  --child-depth 4 \
  --copies 2 \
  --near-live-rows 4 \
  --top-intersections 1
```

Result:

```text
unmarked_support_pairs=24
ordered_flags=48
status=ok
```

The ordered rows include:

```text
h,t,z,p,s,a,tau,kappa,r1,r0,z_V,z_L,delta,comp,g
```

with the direct-support target values:

```text
4,2,8,8,4,4,1,1,2,1,8,12,1,1,0
```

and intersection columns set to `NA`.

Complete-stride direct finite-field gate:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --targeted-direct-check-only \
  --child-depth 4 \
  --prime 5 \
  --seed 1 \
  --copies 2 \
  --near-live-rows 4 \
  --top-intersections 1
```

Result:

```text
unmarked_support_pairs=24
ordered_flags=48
canonical_ordered_flags=48
dim_K_Omega_2_supports=24
dim_ell_i_1_flags=48
dim_ell_j_1_flags=48
direct_sum_ok_supports=24
status=ok
```

Size-8 non-stride Omega scan over `GF(5)`:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --targeted-size8-omega-scan-only \
  --child-depth 4 \
  --prime 5 \
  --seed 1 \
  --copies 2 \
  --near-live-rows 4 \
  --omega-scan-max-omegas 60000 \
  --omega-scan-max-failures 20 \
  --top-intersections 1
```

Result:

```text
total_profiles=51480
complete_stride_omegas=24
complete_stride_dim_ge2=24
nonstride_omegas=51456
nonstride_dim_ge2=8534
max_complete_dim=2
max_nonstride_dim=4
status=bad-size8-omega-scan
```

This is a counter-signal to the broad version of the current story: over `GF(5)`, many non-stride
size-8 supports have `dim K_Omega >= 2`. The scan emitted the first 20 failure examples and exited
nonzero by design.

Classified rerun after making the red flag a reported status rather than a process failure:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --targeted-size8-omega-scan-only \
  --child-depth 4 \
  --prime 5 \
  --seed 1 \
  --copies 2 \
  --near-live-rows 4 \
  --omega-scan-max-omegas 60000 \
  --omega-scan-max-failures 20 \
  --top-intersections 1
```

Summary:

```text
complete_stride_omegas=24
complete_stride_dim_ge2=24
nonstride_omegas=51456
nonstride_dim_ge2=8534
max_complete_dim=2
max_nonstride_dim=4
status=red-flag-classified
```

Grouped classification:

```text
complete stride:
  dim=2, complete_stride_union=1, contained_stride_line_count=2,
  contained_kernel_line_count=6, profiles=24.

non-stride dim=2:
  contained_stride_line_count in {0,1},
  contained_kernel_line_count=6,
  profiles=7994.

non-stride dim=3:
  contained_stride_line_count in {0,1},
  contained_kernel_line_count=31,
  profiles=538.

non-stride dim=4:
  contained_stride_line_count=0,
  contained_kernel_line_count=156,
  profiles=2.
```

Interpretation: the non-stride failures are not merely `Omega = C_i union C_j` hidden under a
different label. They usually contain zero or one complete stride line, yet contain the full
projective line set of a higher-dimensional kernel subspace. Over `GF(5)` this is real non-stride
dimension growth in the broad size-8 support scan.

Large-prime replay after adding rank-only classification:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --targeted-size8-omega-scan-only \
  --omega-scan-fast-rank-only \
  --child-depth 4 \
  --prime 65537 \
  --seed 1 \
  --copies 2 \
  --near-live-rows 4 \
  --omega-scan-max-omegas 60000 \
  --omega-scan-max-failures 20 \
  --top-intersections 1
```

Summary:

```text
total_profiles=51480
complete_stride_dim_ge2=24
nonstride_dim_ge2=0
max_complete_dim=2
max_nonstride_dim=1
status=ok
```

Top rank-only classification groups:

```text
dim=0, complete=0, stride_lines=0, row_hits=2, contains_row_block=0, profiles=18168
dim=0, complete=0, stride_lines=0, row_hits=3, contains_row_block=0, profiles=10944
dim=0, complete=0, stride_lines=0, row_hits=1, contains_row_block=0, profiles=10656
dim=1, complete=0, stride_lines=1, row_hits=2, contains_row_block=0, profiles=3984
dim=1, complete=0, stride_lines=1, row_hits=1, contains_row_block=0, profiles=2016
dim=2, complete=1, stride_lines=2, row_hits=2, contains_row_block=0, profiles=24
```

Interpretation: the broad non-stride `dim K_Omega >= 2` red flag appears to be a `GF(5)`
small-field artifact for the seed-1 large-prime replay. At `GF(65537)`, the only size-8 profiles
with `dim K_Omega = 2` are the 24 complete-stride unions.

Two-copy exact complete-stride flag intersection diagnostic over `GF(65537)`:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --child-depth 4 \
  --prime 65537 \
  --seed 1 \
  --copies 2 \
  --near-live-rows 4 \
  --t 2 \
  --tau 1 \
  --exact-r0 1 \
  --exact-r1 2 \
  --stride-modulus 4 \
  --max-raw-profiles 100 \
  --max-flags-per-copy 100 \
  --max-intersections 3000 \
  --top-intersections 20
```

Summary:

```text
copy0_exact_flags=48
copy1_exact_flags=48
intersections_checked=2304
common_v_dim=0
common_l_dim=0
generic_v_dim=0
generic_l_dim=0
v_excess=0
l_excess=0
exact_flag_tuples=2304
```

Interpretation: for the seed-1/two-copy large-prime complete-stride gate, all exact flag tuples
match the generic intersection dimensions. This run does not show a two-copy complete-stride
intersection excess.

Three-copy exact complete-stride flag intersection diagnostic over `GF(65537)`:

```text
python scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py \
  --targeted-near-stride-w \
  --child-depth 4 \
  --prime 65537 \
  --seed 1 \
  --copies 3 \
  --near-live-rows 4 \
  --t 2 \
  --tau 1 \
  --exact-r0 1 \
  --exact-r1 2 \
  --stride-modulus 4 \
  --max-raw-profiles 100 \
  --max-flags-per-copy 100 \
  --max-intersections 120000 \
  --top-intersections 20
```

Summary:

```text
copy0_exact_flags=48
copy1_exact_flags=48
copy2_exact_flags=48
intersections_checked=110592
max_common_v_dim=0
max_common_l_dim=0
max_v_excess=0
max_l_excess=0
positive_excess_witness_rows=0
```

Interpretation: for the seed-1/three-copy large-prime complete-stride gate, all exact flag tuples
again match the generic intersection dimensions. This run does not show complete-stride V/L
intersection excess beyond generic behavior.

Paired-spine cascade seeded by the 48 ordered complete-stride flags:

```text
python scripts/rfc_distance_analysis/rfc_paired_spine_cascade.py \
  --prime 65537 \
  --depth 4 \
  --near-live-rows 4 \
  --chain-length 2 \
  --t-values 3 \
  --tau 1 \
  --tracked-kappa 1 \
  --kernel-mode full_kernel
```

Summary:

```text
chain_count=3145824
canonical_chain_count=3145824
duplicate_certificate_count=206167867488
expected_lift_logq=1.0000000000
observed_chain_logq=1.0000013758
chain_excess_logq=0.0000013758
hidden_line_fiber_logq=1.0000013758
hidden_line_fiber_charged=0
positive_excess_witness_rows=1
q_dimension_excess_witness_rows=0
status=ok
```

Completed profile:

```text
h=5,t=3,z=20,p=8,s=4,a=4,tau=1,kappa=2,
full_kernel_dim=2,tracked_kappa=1,kernel_mode=full_kernel,
r1=2,r0=1,z_V=8,z_L=12,
chain_key=h=5|t=3|kernel_mode=full_kernel|tau=1|full_kernel_dim=2|tracked_kappa=1|r1=2|r0=1|zV=8|zL=12|seed_flags=48,
downstream_constraint_key=,
hidden_line_fiber_count=65538,
hidden_line_fiber_logq=1.0000013758,
hidden_line_fiber_charged=0,
expected_lift_logq=1,
observed_chain_logq=1.0000013758,
chain_excess_logq=0.0000013758
```

Interpretation: the resolved `t=3,tau=1` full-kernel lift has `full_kernel_dim=kappa=2` and matches
the proof-contract Lift exponent up to the finite Gaussian-binomial constant already covered by
Gamma factors. The tracked `kappa=1` line inside `K_full` has fiber size `65538`, but because no
`downstream_constraint_key` was supplied it is reported as uncharged duplicate metadata, not free
canonical chain multiplicity. This run does not show a q-dimension lift excess.

Tau-2 paired-spine gate seeded by aggregate complete-stride supports:

```text
python scripts/rfc_distance_analysis/rfc_paired_spine_cascade.py \
  --prime 65537 \
  --depth 4 \
  --near-live-rows 4 \
  --chain-length 2 \
  --t-values 2 \
  --tau 2 \
  --tracked-kappa 0 \
  --kernel-mode full_kernel \
  --seed-mode auto
```

Summary:

```text
tau=2
t_values=2
seed_mode=auto
chain_count=442755635900124758160
canonical_chain_count=442755635900124758160
duplicate_certificate_count=0
expected_lift_logq=4.0000000000
observed_chain_logq=4.0000013759
chain_excess_logq=0.0000013759
rank_S=2
rank_S_minus_A=0
delta=2
comp=2
g=0
endpoint_component_logq=-6
endpoint_generic_logq=-4
endpoint_bound_logq=-4
canonical_endpoint_event_count=4295229444
exact_support_event_count=4295229444
observed_endpoint_logq=-5.9999972483
endpoint_excess_logq=-1.9999972483
endpoint_status=closed_form_product_rank1_components
q_dimension_excess_witness_rows=0
blockers=none
```

Interpretation: the complete-stride aggregate support is admissible for `tau=2` because
`delta=2`. The paired lift count again matches the expected Lift exponent up to finite Gaussian
constants and shows no q-dimension chain excess. The product-style endpoint hook uses the
two-rank-one-component formula `E_A(2)=(q+1)^2`; one projective root line is chosen for each
component, and one-component sub-supports have `delta<2`, so exact-support inversion subtracts no
tau-2 events. Against the corrected bound `endpoint_bound_logq=-4`, the observed root-weight
exponent is `-5.9999972483`, giving `endpoint_excess_logq=-1.9999972483`.

Existing endpoint-profiler probe for the same size-8 tau-2 row:

```text
python scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py \
  --prime 65537 \
  --child-depth 4 \
  --expansion 1 \
  --size 8 \
  --tau 2 \
  --samples 1 \
  --seed 1 \
  --max-size 8 \
  --max-line-assignments 1000
```

Result:

```text
checked=1
skipped_supports=255
observed endpoint/count columns: none emitted
endpoint_excess_logq: not computed
comparison to endpoint_bound_logq=-4: unavailable
```

Interpretation: the existing root-line profiler has the right output schema for exact root-line
counts and endpoint columns, but it cannot compute the depth-4 size-8 `GF(65537)` target row as-is.
Its CLI cannot name the complete-stride support directly, and exact root-line enumeration is
exponential in the support size: the target full support would require `(65537+1)^8` line
assignments. The guarded probe skipped all nonempty masks before any observed count could be
computed. The closed-form product hook above is therefore the relevant non-enumerative endpoint
method for this specific `delta=2, comp=2, g=0` row.

Dense connected tau-2 endpoint artifact check:

```text
Import-Csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_summary.csv |
  Where-Object { [int]$_.root_count -eq 4 -and [int]$_.delta -eq 3 -and
                 [int]$_.support_components -eq 1 -and [int]$_.generic_kernel_dim -eq 2 }
```

Result:

```text
file=rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_summary.csv
field=GF(11)
child_depth=2
expansion=4
sampled_size=5
tau=2
a=root_count=4
rank_S=4
rank_S_minus_A=complement_rank=1
support_rank=3
delta=3
comp=1
g=2
shapes=2
exact_root_line_logq=4.1649413340
observed_endpoint_logq=exact_root_line_logq-a=0.1649413340
endpoint_component_logq=comp+2delta-4-a=-1
endpoint_generic_logq=2g-4=0
endpoint_bound_logq=0
endpoint_excess_logq=0.1649413340
kernel_profiles=2:20724;3:12
```

Interpretation: the corrected dense connected target does exist in current small-depth sampled
data. The smallest saved connected `delta>=3,g>=2` support found has `a=4`, exactly the proof-lane
target shape `delta=3, comp=1, g=2`. The observed endpoint excess is positive over `GF(11)` but
well below one q-dimension; this is a finite-field/sampled diagnostic row, not yet a large-prime
proof obstruction.

Larger-field reproduction assessment for that dense connected row:

```text
Get-ChildItem docs\rfc_distance_analysis -Filter '*gf11_child_depth2_c4_size5_tau2*'
Import-Csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_summary.csv |
  Where-Object { [int]$_.root_count -eq 4 -and [int]$_.delta -eq 3 -and
                 [int]$_.support_components -eq 1 -and [int]$_.generic_kernel_dim -eq 2 }
```

Result:

```text
support-detail artifact: missing
available artifact: rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_summary.csv
concrete S columns: not available
concrete A/support_columns: not available
larger-field endpoint_excess_logq: not computed
```

Blocker: the saved GF(11) artifact is summary-only. It has no matching `*_supports.csv`, so the
concrete sampled support cannot be extracted. Existing `rfc_root_line_kernel_profile.py` also has no
support-targeted CLI such as `--columns` plus `--support-columns`; it only iterates or samples
subsets by size. Reproducing this row at a larger field without broad enumeration needs one of:

```text
1. a saved support-detail CSV for the GF(11) sample, containing columns/support_columns; or
2. a support-targeted profiler API:
   --columns <S> --support-columns <A>
   with guards and endpoint columns; or
3. a non-enumerative closed-form endpoint hook for the connected delta=3, comp=1, g=2 profile.
```

No larger-field diagnostic was run, because without a concrete support or support-targeted hook the
only available path is broad sampling plus root-line assignment enumeration.

Support-debug output hook:

```text
python -m py_compile scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py
python scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py --help
```

Change: `rfc_root_line_kernel_profile.py --support-csv` now writes the sampled parent support
`columns`, exact endpoint `support_columns`, existing endpoint/count columns, and finite-constant
decomposition fields:

```text
endpoint_excess_logq
projective_line_factor_logq
nonzero_root_normalization_logq
exceptional_kappa3_count
residual_endpoint_excess_logq
```

`exceptional_kappa3_count` is currently `NA` because the existing row has only the contained
`kernel_profile`; converting that into an exact-support-adjusted exceptional count needs a separate
inversion rule. Existing summary columns and semantics are unchanged.

Depth-4 complete-stride target, `t=2`, `tau=1`, `kappa=1`, exact `r0=1`, `r1=2`:

```text
--child-depth 4 --h 4 --inner-columns 0:1:4:5:8:9:12:13
--paired-sizes 4 --visible-sizes 4 --z 12 --stride-modulus 4
--enumerate-subflags --exact-r0 1 --exact-r1 2
```

Result:

```text
raw_profiles=70 per copy
skipped_subflag_profiles=70 per copy
exact_flags=0 under the 10000 subflag cap
```

Interpretation: the requested depth-4 `r0=1,r1=2` subflag target is not absent; it is too broad for
naive exact Gaussian subflag enumeration even on one explicit complete-stride inner set. The new
targeted combinatorial mode avoids that broad path and first verifies the expected `24/48`
complete-stride family.

Tiny exact subflag sanity check at child-depth 2:

```text
raw_profiles=2 per copy
skipped_subflag_profiles=0
exact_flags=66 per copy
intersections_checked=4356
```

This validates the exact `r1=2,r0=1` subflag and intersection path on a genuinely small state.

## Next Limits

The first two gates are green: the combinatorial `24/48` target and the direct finite-field
dimension/direct-sum target over `GF(5)`. The broad all-size-8 non-stride scan is field-sensitive:
`8534` non-stride supports have `dim K_Omega >= 2` over `GF(5)`, but the rank-only replay over
`GF(65537)` has `nonstride_dim_ge2=0`. The first two-copy exact complete-stride intersection
diagnostic over `GF(65537)` also shows no common-dimension excess, and the three-copy seed-1 gate
still has no positive V/L excess over 110592 exact flag tuples. The paired-spine cascade has no
q-dimension lift excess for the resolved `t=3` full-kernel shape; the tracked line is uncharged
metadata unless a downstream constraint names it. The tau-2 paired-spine lift gate also has no
q-dimension chain excess on the aggregate complete-stride supports, and the product-style endpoint
hook gives negative endpoint excess for the `delta=2, comp=2, g=0` row.
Existing sampled root-line data also contains the corrected dense connected tau-2 target
`a=4, delta=3, comp=1, g=2` over `GF(11)`, with `endpoint_excess_logq=0.1649413340`.

Recommended next turn:

```text
Treat the GF(5) non-stride growth as a small-field artifact unless a second large-prime seed says
otherwise. Next diagnostic pressure should only charge the hidden tracked-line fiber if a concrete
downstream recurrence constraint supplies a canonical `downstream_constraint_key`; otherwise the
full-kernel lift accounting remains the relevant event count. For tau=2, the next missing piece is
not this product row; it is any non-product tau-2 exact support where no closed-form endpoint hook
is available. Existing `rfc_root_line_kernel_profile.py` is suitable for small-field/small-support
endpoint experiments, but not for `GF(65537)` size-8 rows without a non-enumerative or
support-targeted endpoint method. For the dense connected tau-2 row, the next pressure test is a
support-targeted computation at a larger field, but that first requires concrete support columns or
a `--columns/--support-columns` endpoint profiler mode.
```

Dense connected tau-2 support capture:

```text
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 11 --child-depth 2 --expansion 4 --size 5 --tau 2 --samples 20 --seed 1 --max-size 5 --max-line-assignments 300000 --summary-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_support_capture_summary.csv --support-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_support_capture_supports.csv
```

Artifacts:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_support_capture_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf11_child_depth2_c4_size5_tau2_support_capture_supports.csv
```

Result:

```text
checked=20
support_detail_rows=277
target rows with a=4, delta=3, comp=1, g=2: 4
summary row:
  rho=4 components=2 tau=2 root_count=4 support_rank=3 support_components=1
  delta=3 complement_rank=1 generic_kernel_dim=2 endpoint_bound_logq=4
  endpoint_root_weight_logq=0 shapes=4
  exact_root_line_logq=4.164941334045784
  asymptotic_root_weight_logq=0.1649413340457837
  kernel_profiles=2:20724;3:12
```

Concrete target supports captured:

```text
subset_index=1  S=0:4:7:11:14   A=4:7:11:14
subset_index=13 S=0:7:9:12:13   A=0:9:12:13
subset_index=18 S=0:7:11:14:15  A=0:11:14:15
subset_index=19 S=1:4:9:11:12   A=4:9:11:12
```

Endpoint/support-detail columns after fixing the detail-row bookkeeping:

```text
generic_endpoint_logq=4
component_endpoint_logq=3
endpoint_bound_logq=4
endpoint_root_weight_logq=0
exact_root_line_count=21744
exact_root_line_logq=4.164941334045784
asymptotic_root_weight_logq=0.1649413340457837
endpoint_excess_logq=0.16494133404578371
projective_line_factor_logq=0.14514625050840735
nonzero_root_normalization_logq=0.15898972884349005
exceptional_kappa3_count=NA
residual_endpoint_excess_logq=-0.1391946453061137
kernel_profile=2:20724;3:12
```

The support rows unblock a larger-field targeted replay once the profiler gets a guarded
`--columns <S> --support-columns <A>` path. No broad rerun was performed.

Targeted support replay mode:

`rfc_root_line_kernel_profile.py` now accepts a narrow exact replay path:

```text
--columns <S>
--support-columns <A>
```

When both are supplied, the script constructs only the requested parent support `S`, only the
requested exact endpoint support `A`, and only the submasks of `A` needed for exact
inclusion-exclusion. It does not sample or enumerate unrelated supports. The existing
`--max-line-assignments` guard still blocks a target if `(p+1)^|A|` is too large.

Compile and one replay command:

```text
python -m py_compile scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --tau 2 --columns 0:4:7:11:14 --support-columns 4:7:11:14 --max-line-assignments 2000000
```

Result:

```text
mode=targeted_support
status=ok
prime=31
S=0:4:7:11:14
A=4:7:11:14
tau=2
a=root_count=4
rank_S=4
rank_S_minus_A=1
support_rank=3
delta=3
comp=2
g=2
generic_endpoint_logq=4
component_endpoint_logq=4
endpoint_bound_logq=4
endpoint_root_weight_logq=0
target_line_assignments=1048576
kernel_profile=2:1047552;3:1024
contained_root_line_count=2064384
contained_root_line_logq=4.234244791535479
exact_root_line_count=1965056
exact_root_line_logq=4.219885060340455
asymptotic_root_weight_logq=0.21988506034045496
endpoint_excess_logq=0.21988506034045496
projective_line_factor_logq=0.036981731641996696
nonzero_root_normalization_logq=0.0381944612724987
exceptional_kappa3_count=NA
residual_endpoint_excess_logq=0.14470886742595956
```

Blocker/interpretation: root-line enumeration was feasible at `GF(31)`, but this replay did not
preserve the `comp=1` dense-connected shape from the captured `GF(11)` sample; it returned
`comp=2` for the same coordinate pattern under the seeded `GF(31)` generator. A strict larger-field
reproduction of the `a=4, delta=3, comp=1, g=2` row now needs either a larger-field support search
for a matching connected support, or a field-lifted replay model that fixes/lifts the original
GF(11) diagonal data rather than regenerating a new seeded RFC instance over the replay prime.

Guarded GF31 dense endpoint search:

```text
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --size 5 --tau 2 --samples 2 --seed 1 --max-size 5 --max-line-assignments 2000000 --summary-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_search_summary.csv --support-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_search_supports.csv
```

Artifacts:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_search_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_search_supports.csv
```

Result:

```text
prime=31
child_depth=2
expansion=4
size=5
tau=2
checked=2
skipped_supports=2
support_detail_rows=30
target a=4, delta=3, comp=1, g=2 rows=0
```

No target certificate/hash is available because the sampled GF31 supports did not contain a target
row. The closest exact rows had `a=4, delta=3, g=2` but `comp=4`:

```text
summary:
  rho=4 components=1 tau=2 root_count=4 support_rank=4 support_components=4
  delta=3 complement_rank=1 generic_kernel_dim=2 endpoint_bound_logq=6
  endpoint_root_weight_logq=2 shapes=10
  exact_root_line_logq=4.044560550003203
  asymptotic_root_weight_logq=0.04456055000320269
  kernel_profiles=2:1048544;3:32

example rows:
  S=1:3:7:12:15   A=1:3:7:12
  S=1:3:7:12:15   A=1:3:7:15
  S=1:3:7:12:15   A=1:3:12:15
  S=0:6:12:13:14  A=0:6:12:13
```

For these non-target rows:

```text
exact_root_line_count=1076224
exact_root_line_logq=4.044560550003203
endpoint_excess_logq=-1.9554394499967973
projective_line_factor_logq=0.036981731641996696
nonzero_root_normalization_logq=0.0381944612724987
residual_endpoint_excess_logq=-2.0306156429112927
kernel_profile=2:1048544;3:32
```

Blocker: this small GF31 search did not find a fresh connected `comp=1` dense endpoint row. The
next bounded step is either another small sampled GF31 search with a different seed, or a
connectivity-biased support sampler so the diagnostic does not spend most of its exact endpoint
budget on disconnected `comp=4` supports.

Connectivity-biased endpoint filter scaffold:

`rfc_root_line_kernel_profile.py` now has cheap endpoint-shape filters:

```text
--require-root-count <a>
--require-delta <delta>
--require-support-components <comp>
--require-generic-kernel-dim <g>
```

These filters are applied before root-line assignment enumeration. For exactness, when a support
mask `A` passes the filters, the script still enumerates all required submasks of `A` so the exact
root-line inclusion-exclusion remains valid. Unrelated endpoint masks are skipped before the hot
line-assignment loop. The first stdout line now includes `filtered_supports=<count>`.

Smoke/compile only:

```text
python -m py_compile scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --help
```

Next exact guarded command to run for the GF31 dense-connected search:

```text
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --size 5 --tau 2 --samples 10 --seed 2 --max-size 5 --max-line-assignments 2000000 --require-root-count 4 --require-delta 3 --require-support-components 1 --require-generic-kernel-dim 2 --summary-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_seed2_summary.csv --support-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_seed2_supports.csv
```

This is still a sample, not a broad enumeration: it samples 10 parent supports `S`, enumerates exact
root-line assignments only for endpoint supports matching `a=4, delta=3, comp=1, g=2`, and keeps
the existing `(p+1)^a <= 2000000` guard.

Connected-support discovery mode and GF31 hit:

`rfc_root_line_kernel_profile.py` now also has:

```text
--connected-support-discovery
--discovery-attempts <count>
--discovery-extra-per-support <count>
--discovery-max-hits <count>
```

This mode is explicitly labeled `generator_mode=discovery` and `sampler_mode=discovery`. It samples
candidate endpoint supports `A` first, rejects supports whose local matroid component count is not
the requested connected count, then proposes parent supports `S=A union {extra}`. It only performs
exact root-line inclusion-exclusion after the cheap profile matches the requested tuple. Discovery
rows are not multiplicity estimates.

Compile and one guarded discovery pass:

```text
python -m py_compile scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --size 5 --tau 2 --seed 2 --max-size 5 --max-line-assignments 2000000 --connected-support-discovery --discovery-attempts 100 --discovery-extra-per-support 4 --discovery-max-hits 1 --require-root-count 4 --require-delta 3 --require-support-components 1 --require-generic-kernel-dim 2 --summary-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_discovery_seed2_summary.csv --support-csv docs\rfc_distance_analysis\rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_discovery_seed2_supports.csv
```

Artifacts:

```text
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_discovery_seed2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_sample_gf31_child_depth2_c4_size5_tau2_connected_discovery_seed2_supports.csv
```

Summary:

```text
generator_mode=discovery
sampler_mode=discovery
prime=31
child_depth=2
expansion=4
parent_size=5
tau=2
root_count=4
configured_attempt_cap=100
actual_attempts_to_first_hit=19
connected_accepts=1
component_rejects=18
proposals=2
profile_rejects=1
profile_accepts=1
target_rows=1
worst_residual_endpoint_excess_logq=-0.030615642911292706
```

Found dense connected row:

```text
S=2:3:7:10:14
A=3:7:10:14
extra_column=2
rank_S=4
rank_S_minus_A=1
support_rank=3
delta=3
comp=1
g=2
endpoint_bound_logq=4
endpoint_root_weight_logq=0
target_line_assignments=1048576
kernel_profile=2:1048544;3:32
contained_root_line_count=1080320
contained_root_line_logq=4.045666749533665
exact_root_line_count=1076224
exact_root_line_logq=4.044560550003203
asymptotic_root_weight_logq=0.04456055000320269
endpoint_excess_logq=0.04456055000320269
projective_line_factor_logq=0.036981731641996696
nonzero_root_normalization_logq=0.0381944612724987
residual_endpoint_excess_logq=-0.030615642911292706
support_certificate_hash=ce34e02a4710196c693887b62697a2034d5f14f4407f82ee13993412bc4eac0e
rank_subset_table_hash=916d15f8bffad5db70c6d99e3cf5dba6893a894144d755c82c890d9c4e5b5423
component_partition_key=3:7:10:14
circuit_witnesses=3:7:10:14
g_minimizer_keys=empty
```

The requested dense profile exists over `GF(31)` in this discovered support. Remaining missing
decomposition: `exceptional_kappa3_count` is still emitted as `NA`; the raw `kernel_profile`
contains the finite concentration signal `3:32`, but the dedicated exceptional-count column has
not yet been wired to parse that profile.

Exceptional kappa parser and targeted replay note:

`rfc_root_line_kernel_profile.py` now parses `exceptional_kappa3_count` directly from
`kernel_profile`. For example, `kernel_profile=2:1048544;3:32` maps to
`exceptional_kappa3_count=32`.

Compile:

```text
python -m py_compile scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py
```

One support-targeted replay command was run:

```text
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --tau 2 --columns 2:3:7:10:14 --support-columns 3:7:10:14 --max-line-assignments 2000000
```

Result:

```text
status=ok
prime=31
S=2:3:7:10:14
A=3:7:10:14
tau=2
a=root_count=4
rank_S=4
rank_S_minus_A=1
support_rank=4
delta=3
comp=4
g=2
endpoint_bound_logq=6
endpoint_root_weight_logq=2
target_line_assignments=1048576
kernel_profile=2:1047552;3:1024
exceptional_kappa3_count=1024
contained_root_line_count=2064384
contained_root_line_logq=4.234244791535479
exact_root_line_count=1965056
exact_root_line_logq=4.219885060340455
asymptotic_root_weight_logq=0.21988506034045496
endpoint_excess_logq=-1.780114939659545
projective_line_factor_logq=0.036981731641996696
nonzero_root_normalization_logq=0.0381944612724987
residual_endpoint_excess_logq=-1.8552911325740404
```

The parser wiring is confirmed and the residual in this replay is negative. However, this replay
used the script default seed, while the discovered dense connected row above came from the seed-2
generator. Therefore this command did not reproduce the discovered `comp=1` row or the expected
`exceptional_kappa3_count=32`. No second replay was run because of the one-diagnostic limit.

Correct seed-2 replay for the exact discovered row:

```text
python scripts\rfc_distance_analysis\rfc_root_line_kernel_profile.py --prime 31 --child-depth 2 --expansion 4 --tau 2 --seed 2 --columns 2:3:7:10:14 --support-columns 3:7:10:14 --max-line-assignments 2000000
```

Result:

```text
mode=targeted_support
status=ok
prime=31
S=2:3:7:10:14
A=3:7:10:14
tau=2
a=root_count=4
rank_S=4
rank_S_minus_A=1
support_rank=3
delta=3
comp=1
g=2
endpoint_bound_logq=4
endpoint_root_weight_logq=0
target_line_assignments=1048576
kernel_profile=2:1048544;3:32
exceptional_kappa3_count=32
contained_root_line_count=1080320
contained_root_line_logq=4.045666749533665
exact_root_line_count=1076224
exact_root_line_logq=4.044560550003203
asymptotic_root_weight_logq=0.04456055000320269
endpoint_excess_logq=0.04456055000320269
projective_line_factor_logq=0.036981731641996696
nonzero_root_normalization_logq=0.0381944612724987
residual_endpoint_excess_logq=-0.030615642911292706
```

Interpretation: the dense connected `a=4, delta=3, comp=1, g=2` endpoint profile is now reproduced
as a stable targeted regression over `GF(31)`. The raw endpoint excess remains positive but is
fully covered by the explicit finite projective-line and nonzero-root constants; the residual is
negative. This retires the current dense connected tau-2 endpoint row as an empirical blocker.
