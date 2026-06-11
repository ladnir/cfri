# RFC Complete-Stride Intersection Proof Contract

Scope: original non-systematic RFC only.

This contract defines the two-copy complete-stride flag-intersection gate. It is a proof-support
contract, not a benchmark plan. The goal is to decide whether exact complete-stride child flags from
two independent RFC copies intersect with the generic dimensions assumed by the two-layer flag
recurrence.

## Complete-Stride Flag Object

At depth `4`, with `k=16`, the complete-stride target uses row blocks:

```text
B_b = {4b, 4b+1, 4b+2, 4b+3},  b in {0,1,2,3},
```

and output stride classes:

```text
C_j = {j, j+4, j+8, j+12},  j in {0,1,2,3}.
```

For an unordered pair `{i,j}`, define:

```text
Omega = C_i union C_j.
```

The complete-stride gate expects:

```text
dim K_Omega = 2,
K_Omega = ell_i direct_sum ell_j,
```

where:

```text
K_Omega = {x supported on B_b : output outside Omega is zero},
ell_i   = {x supported on B_b : output outside C_i is zero},
ell_j   = {x supported on B_b : output outside C_j is zero}.
```

Each support produces two ordered flags:

```text
ell_i <= K_Omega, visible quotient on C_j,
ell_j <= K_Omega, visible quotient on C_i.
```

Expected single-copy complete-stride counts:

```text
unmarked supports = 4 * binom(4,2) = 24,
ordered flags     = 48.
```

Each ordered flag has:

```text
r0 = dim L = 1,
r1 = dim V = 2,
tau = 1,
kappa = 1.
```

## Event Versus Duplicate Certificate

The gate must count canonical linear events.

### Events

These are real events:

```text
1. an independent RFC copy label;
2. an unmarked support pair (B_b, Omega);
3. an ordered exact flag L <= V with a distinct visible quotient choice;
4. a canonical pair of flags, one from each independent copy;
5. the vector-space intersection V_1 cap V_2 or L_1 cap L_2 for such a canonical pair.
```

### Duplicate Certificates

These are duplicate certificates and must not multiply event counts:

```text
basis choices for L or V,
ordering of the unordered support Omega = C_i union C_j,
repeated construction paths for the same RREF flag key,
contained non-stride Omega labels that produce the same exact flag,
root-line representatives for the same projective line,
same-copy comparisons used as sanity checks.
```

The implementation should emit duplicate counts as diagnostics, but the proof gate uses canonical
RREF keys:

```text
canonical_L_key,
canonical_V_key,
canonical_flag_key = (canonical_L_key, canonical_V_key).
```

## Generic Intersection Dimensions

For two independent copies, compare flags in the same ambient message space `H`, with dimension:

```text
ambient_dim = k = 16
```

unless the implementation explicitly conditions both flags to lie in the same row block `B_b`.
If it does, it must report:

```text
conditioned_ambient_dim = dim span(B_b) = 4.
```

The generic vector-space intersection dimension for subspaces of dimensions `a` and `b` in ambient
dimension `m` is:

```text
generic_dim(a,b;m) = max(0, a + b - m).
```

For the complete-stride flags:

```text
generic_v_dim = generic_dim(2,2;16) = 0,
generic_l_dim = generic_dim(1,1;16) = 0.
```

If both flags are intentionally conditioned to the same `B_b`, then:

```text
generic_v_dim = generic_dim(2,2;4) = 0,
generic_l_dim = generic_dim(1,1;4) = 0.
```

So the expected generic dimensions are zero in either allowed ambient convention.

Observed dimensions:

```text
common_v_dim = dim(V_1 cap V_2),
common_l_dim = dim(L_1 cap L_2).
```

Excess dimensions:

```text
v_excess = common_v_dim - generic_v_dim,
l_excess = common_l_dim - generic_l_dim.
```

## Evidence Supporting Generic Intersection

The gate supports the current two-layer flag recurrence if all completed two-copy complete-stride
rows satisfy:

```text
common_v_dim = 0,
common_l_dim = 0,
v_excess = 0,
l_excess = 0.
```

Additional required conditions:

```text
1. both copies are independent, with distinct seeds or independently sampled fold randomness;
2. each copy individually passes the complete-stride direct-sum gate;
3. each copy emits exactly 24 unmarked supports and 48 ordered canonical flags;
4. duplicate certificates are reported separately and not multiplied;
5. skipped/capped intersections are reported explicitly, not omitted.
```

This is admissible evidence that active-copy complete-stride flags do not need a new recurrence
state beyond:

```text
active copy profile,
exact support profile,
flag dimensions r0=1,r1=2,
two-layer child flag moment.
```

## Evidence Forcing A New State

The proof lane must treat the current recurrence as missing state if any completed large-prime or
generic-symbolic two-copy row has:

```text
v_excess >= 1
```

or:

```text
l_excess >= 1.
```

For this gate, either condition means an independent-copy complete-stride flag intersection has one
extra reusable q-dimension. At production `q=2^128`, that is a 128-bit effect and cannot be hidden
inside constants.

Before declaring a new state, classify the row:

```text
duplicate_certificate:
  same canonical flag pair was counted through multiple labels; fix counting, no new state.

conditioned_ambient_mismatch:
  generic dimension used the wrong ambient space; recompute with reported ambient_dim.

small_field_degeneracy:
  excess appears only over small fields; track as finite-field diagnostic, no production state.

real_extra_intersection:
  excess persists over large/generic fields after canonicalization and correct ambient choice.
```

Only `real_extra_intersection` forces a new recurrence state or an `e=72` fallback evaluation.

## Plug-In To The Two-Layer Flag Recurrence

The one-copy complete-stride flag feeds the child flag term:

```text
F_{h-1}(r1,z_V;r0,z_L)
```

with:

```text
r1 = 2,
r0 = 1,
z_V = p + s - a,
z_L = p + s.
```

The parent lift is already budgeted by:

```text
Lift(t=2,tau=1,r0=1,r1=2)
  <= Gamma_q^2 q^3.
```

The two-copy intersection gate does not replace this Lift factor. It tests whether the child flag
moment for active copies can use the generic intersection exponent. If intersections are generic,
the recurrence only needs to track the existing flag dimensions and active-copy support profile. If
intersections are not generic, the recurrence needs a new state describing the shared structure,
for example:

```text
shared row block,
shared stride line,
shared complete-stride plane,
or another canonical flag-intersection class.
```

## Required Output Columns

Each two-copy intersection row should include:

```text
field_prime
depth
k
copy_a_seed
copy_b_seed
copy_a_id
copy_b_id
independent_copies
ambient_dim
conditioned_ambient_dim
b_a
omega_a
ordered_flag_role_a
canonical_L_key_a
canonical_V_key_a
canonical_flag_key_a
b_b
omega_b
ordered_flag_role_b
canonical_L_key_b
canonical_V_key_b
canonical_flag_key_b
same_B_block
same_Omega
same_flag_key
r0_a
r1_a
r0_b
r1_b
common_v_dim
common_l_dim
generic_v_dim
generic_l_dim
v_excess
l_excess
duplicate_certificate_count
intersection_pair_count
classification
proof_effect
status
```

Each run summary should include:

```text
field_prime
copy_a_seed
copy_b_seed
complete_stride_supports_a
complete_stride_supports_b
ordered_flags_a
ordered_flags_b
canonical_flags_a
canonical_flags_b
total_intersection_pairs
skipped_intersection_pairs
max_common_v_dim
max_common_l_dim
max_v_excess
max_l_excess
duplicate_certificate_pairs
real_extra_intersection_count
status
```

## Acceptance Criteria

Accept the two-copy complete-stride gate as supporting generic intersection if:

```text
status = ok,
independent_copies = true for all counted pairs,
complete_stride_supports_a = complete_stride_supports_b = 24,
ordered_flags_a = ordered_flags_b = 48,
canonical_flags_a = canonical_flags_b = 48,
skipped_intersection_pairs = 0,
max_v_excess = 0,
max_l_excess = 0,
real_extra_intersection_count = 0.
```

If the implementation uses a smaller targeted subset rather than all `48 * 48` pairs, then
`status` must be:

```text
partial_ok
```

and the row must report the exact subset rule. Partial evidence can support debugging, but it is
not a full proof gate.

## Rejection Criteria

Reject the gate as unresolved if:

```text
skipped_intersection_pairs > 0,
independent_copies is false for any counted pair,
ambient_dim is missing,
duplicate certificates are included in intersection_pair_count,
same-copy sanity rows are mixed with independent-copy rows.
```

Escalate to new recurrence-state analysis if:

```text
real_extra_intersection_count > 0
```

after duplicate certificates, ambient mismatch, and small-field degeneracy have been ruled out.

## Manager Interpretation

If this gate is clean over a large prime, the current proof route can treat complete-stride active
copy intersections as generic and continue with the two-layer flag recurrence plus the existing
Lift factor.

If this gate finds a real extra intersection dimension, the current recurrence is under-specified.
The next proof step would be to add the shared complete-stride intersection class as state or to
evaluate whether the conservative `e=72` target absorbs the corrected count.
