# RFC Dense Endpoint Replay Contract

Scope: original non-systematic RFC only. This note defines how captured
`GF(11)` dense tau-2 endpoint supports may be replayed over larger fields.

The replay goal is to decide whether a finite `GF(11)` endpoint excess is only a
projective/Gaussian constant, or whether the same support family has positive
residual q-exponent over larger fields.

## Principle

A captured `GF(11)` support is only a coordinate/profile candidate. The replay
must recompute all algebraic data over the new field:

```text
delta
comp
g
exact-support status
endpoint count
finite constants
residual_endpoint_excess_logq
```

Do not inherit these values from the `GF(11)` capture. If the recomputed profile
changes, classify the original row as a small-field or capture-specific
degeneration, not as the same dense endpoint event.

## Data To Preserve From Capture

The capture artifact must preserve enough information to reconstruct the same
RFC local support over a new field.

Required coordinate/construction data:

```text
capture_id
capture_field = 11
construction = original_non_systematic_rfc
fold_algebra = determinant_1_rfc
root_distribution = independent_uniform_nonzero
depth
expansion_c
local_node_id
child_node_id
support_A_coordinates
coordinate_path_labels
row_or_message_coordinate_ids
output_coordinate_ids
active_copy_profile_key
exact_support_key
```

Required reproducibility data:

```text
capture_root_seed
capture_root_values_hash
replay_root_policy
replay_seed
matrix_construction_version
```

`replay_root_policy` must distinguish:

```text
fresh_nonzero_roots
lifted_integer_roots
symbolic_roots
```

The preferred proof-support mode is `fresh_nonzero_roots` or `symbolic_roots`.
Lifting literal `GF(11)` root values into a larger field is diagnostic only
unless the proof explicitly needs that specialization.

Do not preserve only a `GF(11)` row-reduced basis. Row-reduced bases are
field-dependent. The replay must preserve coordinate identities and the RFC
matrix construction needed to rebuild the local restricted matroid.

## Recomputing Delta, Comp, And G

For replay field `F_q`, rebuild the local restricted endpoint object on the same
coordinate support `A`.

Emit raw rank data for every subset:

```text
rank_A(B) for all B subset A
```

For small dense targets such as `a=4`, full subset enumeration is required.

Compute:

```text
delta = rank_before - rank_after
```

using the same support-drop convention as the endpoint counter. Also emit the
equivalent restricted rank when available:

```text
restricted_rank_A = rank_A(A)
```

The replay is admissible only if these agree with the expected profile.

Compute `comp` from the replay-field matroid, not from capture metadata. For
small supports, enumerate circuits or use the standard matroid connectivity
relation:

```text
i connected to j iff some circuit C subset A contains i and j.
```

Emit:

```text
component_partition_key
loop_count
parallel_class_profile
```

Compute:

```text
g = 2*delta - min_{B subset A} (a - |B| + 2*rank_A(B)).
```

Emit both the minimum value and the subset(s) attaining it:

```text
g_min_value
g_minimizer_keys
```

For the smallest dense target, the replay must confirm:

```text
a = 4
delta = 3
comp = 1
g = 2
loop_count = 0
```

If these fail, the row is not a replay of the dense connected target.

## Endpoint Count Over Replay Field

The endpoint counter should compute the exact tau-2 endpoint contribution over
`F_q`:

```text
exact_E_A(2)
```

after exact-support inversion and canonical de-duplication.

Then:

```text
observed_endpoint_logq =
  log_q(exact_E_A(2)) - a
```

under the theorem's `q^-a` root-factor convention.

If the implementation reports actual determinant-1 nonzero-root probabilities
instead, it must also report the conversion convention and the
`nonzero_root_normalization_logq` term below.

## Endpoint Bound

Recompute:

```text
endpoint_generic_logq   = 2*g - 4
endpoint_component_logq = comp + 2*delta - 4 - a
endpoint_bound_logq     = max(endpoint_generic_logq,
                              endpoint_component_logq)
```

For the target replay profile:

```text
a = 4
delta = 3
comp = 1
g = 2

endpoint_generic_logq   = 0
endpoint_component_logq = -1
endpoint_bound_logq     = 0
```

## Finite Constants To Subtract

For tau-2 endpoint rows:

```text
projective_line_factor_logq =
  a * log_q(1 + q^-1)
```

This accounts for:

```text
#P^1(F_q)^A = (q+1)^a
```

being approximated by `q^a` in exponent accounting.

For determinant-1 RFC roots sampled from `F_q^*`, when all required root values
are nonzero:

```text
nonzero_root_normalization_logq =
  a * log_q(q/(q-1)).
```

If any required root value is zero:

```text
classification = impossible_nonzero_root_row
observed_endpoint_logq = -inf
```

For generic kernel dimension `g`, track the Gaussian finite factor:

```text
gaussian_normalization_extra_logq =
  log_q(GaussianBinomial(g,2)_q / q^(2*g-4)).
```

For the target `g=2` row this is:

```text
gaussian_normalization_extra_logq = 0.
```

For exceptional higher-kernel strata, emit the replay-field distribution:

```text
kappa_distribution:
  count of ell with kappa_A(ell)=k
```

For the `a=4, g=2` dense target, define:

```text
exceptional_strata_extra_logq =
  log_q(1 + sum_{k>2} n_k *
              (GaussianBinomial(k,2)_q - 1) / (q+1)^a).
```

If the exact-support counter already subtracts some exceptional assignments,
use the exact-support `n_k` distribution. If only contained-support `n_k` is
available, mark the term as an upper-bound allowance.

Finally:

```text
finite_endpoint_constants_logq =
    projective_line_factor_logq
  + nonzero_root_normalization_logq
  + gaussian_normalization_extra_logq
  + exceptional_strata_extra_logq.
```

## Residual

The implementation must report:

```text
endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq

residual_endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq
  - finite_endpoint_constants_logq.
```

The replay is acceptable proof evidence if:

```text
residual_endpoint_excess_logq <= 0
```

up to declared numerical log-rounding tolerance, and the recomputed profile
matches the target profile over the replay field.

Suggested tolerance:

```text
log_rounding_tolerance = 1e-9
```

for exact integer counts converted to floating logs. Larger tolerances must be
justified by the implementation.

## Required Output Columns

Each replay row should emit:

```text
capture_id
capture_field
replay_field
construction
fold_algebra
root_distribution
depth
expansion_c
support_A_coordinates
coordinate_path_labels
replay_root_policy
replay_seed
a
tau
rank_A_subset_table_hash
rank_before
rank_after
restricted_rank_A
delta
component_partition_key
comp
loop_count
parallel_class_profile
g
g_min_value
g_minimizer_keys
exact_support_status
contained_E_A_2
exact_E_A_2
kappa_distribution
observed_endpoint_logq
endpoint_generic_logq
endpoint_component_logq
endpoint_bound_logq
endpoint_excess_logq
projective_line_factor_logq
nonzero_root_normalization_logq
gaussian_normalization_extra_logq
exceptional_strata_extra_logq
finite_endpoint_constants_logq
residual_endpoint_excess_logq
classification
proof_effect
notes
```

## Classification

Use:

```text
classification = replay_clean
```

when the target profile is reproduced and:

```text
residual_endpoint_excess_logq <= 0.
```

Use:

```text
classification = gf11_degenerate_profile
```

when the support no longer has the same `delta/comp/g` profile over the replay
field.

Use:

```text
classification = finite_constant_explains_excess
```

when bare `endpoint_excess_logq > 0` but residual excess is nonpositive.

Use:

```text
classification = residual_endpoint_excess
```

only when the target profile is reproduced and:

```text
residual_endpoint_excess_logq > 0
```

after exact-support inversion and finite-constant subtraction.

Only `residual_endpoint_excess` is a proof blocker for the asymptotic local
endpoint theorem. It should trigger local theorem repair before any global
`e=72` evaluation.
