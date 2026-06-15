# RFC Dense Endpoint Replay Field Model

Scope: original non-systematic RFC only. This note interprets the support-targeted
replay where a `GF(11)` dense endpoint support was replayed over `GF(31)` and
the captured coordinates changed profile from:

```text
comp = 1
```

to:

```text
comp = 2
```

## Conclusion

Fixed coordinate IDs across independently sampled finite-field RFC instances are
not, by themselves, proof-valid replay evidence for the same dense connected
endpoint profile.

They are useful diagnostics for profile stability:

```text
same coordinates + fresh field/random roots changed comp from 1 to 2
```

means the `GF(11)` dense connected profile was not forced by those coordinates
alone. It may depend on the field, the root specialization, or a small-field
coincidence. But this does not prove that the dense connected profile is absent
over larger fields; it only says this captured coordinate set is not a stable
larger-field replay witness under fresh sampling.

For proof use, there are two valid models:

1. Search for a new matching profile over the larger field/random RFC instance.
2. Lift through a common symbolic local-fold model and then specialize.

The first is the right model for distributional production evidence. The second
is the right model for testing whether a specific algebraic stratum persists
across fields.

## Model A: Search In The Larger Field

This is proof-valid for production-distribution evidence.

Procedure:

```text
choose replay field F_q
sample determinant-1 RFC roots T uniform in F_q^*
construct the local RFC matrix over F_q
search supports A
recompute delta/comp/g over F_q
keep only rows matching the dense target profile
count exact tau-2 endpoint events
subtract finite constants
report residual_endpoint_excess_logq
```

For the dense connected target, an accepted replay row must satisfy over the
larger field:

```text
a = 4
delta = 3
comp = 1
g = 2
loop_count = 0
exact_support_status = exact
root_distribution = independent_uniform_nonzero
fold_algebra = determinant_1_rfc
```

This model does not require the same coordinate IDs as the `GF(11)` capture. It
asks whether the larger-field RFC distribution produces the same profile and,
if so, whether the endpoint residual is nonpositive.

Interpretation:

- If matching profiles exist and residuals are nonpositive, the finite `GF(11)`
  excess is likely a constant/small-field artifact.
- If matching profiles exist and residuals remain positive after constants,
  the local endpoint theorem needs repair.
- If matching profiles do not appear, the dense connected row may be irrelevant
  for that field/depth, but absence from a finite search is not a theorem.

## Model B: Symbolic Field-Lift

This is proof-valid only if the replay is built from a common symbolic object.

Required object:

```text
local RFC matrix M(T_1,...,T_m) over Z[T_1,...,T_m]
support coordinates A
explicit symbolic nonzero/minor conditions defining the target stratum
```

Then specialize the same symbolic data into each field:

```text
T_i -> t_i in F_q^*
```

and recompute:

```text
delta
comp
g
exact support
endpoint count
residual_endpoint_excess_logq
```

This is a valid persistence test when the same rank and connectivity conditions
are certified by nonzero symbolic minors or by direct recomputation over the
target field.

Important caveat: mapping `GF(11)` root values to integers and reducing them
modulo `31` is not a field embedding. It is not the same algebraic point in a
larger field, because the characteristics differ. Such an integer lift is only a
diagnostic specialization unless it is backed by a symbolic polynomial model and
the target-field recomputation verifies the desired profile.

## Invalid Replay Model

The following is not proof-valid:

```text
take coordinate IDs from GF(11)
sample a fresh independent GF(31) RFC instance
expect delta/comp/g to remain the same
use profile change as direct evidence about the original GF(11) endpoint count
```

This mixes two different random algebraic instances. Coordinate IDs identify
positions in the recursive code tree, but the local matroid also depends on the
field and sampled roots. If the roots change, the represented matroid can change.

The observed `comp=1 -> comp=2` transition should therefore be classified as:

```text
classification = coordinate_pattern_not_profile_stable
proof_effect = diagnostic_only
```

not as:

```text
classification = dense_profile_absent
```

and not as:

```text
classification = endpoint_theorem_verified
```

## Exact Requirements For Valid Larger-Field Replay

A larger-field replay row is admissible only if it emits:

```text
replay_model                         # larger_field_search or symbolic_field_lift
capture_id
capture_field
replay_field
construction
fold_algebra
root_distribution
root_policy
support_A_coordinates
coordinate_path_labels
local_matrix_construction_key
rank_A_subset_table_hash
rank_before
rank_after
delta
component_partition_key
comp
loop_count
parallel_class_profile
g
g_min_value
g_minimizer_keys
exact_support_status
endpoint_bound_logq
observed_endpoint_logq
projective_line_factor_logq
nonzero_root_normalization_logq
gaussian_normalization_extra_logq
exceptional_strata_extra_logq
finite_endpoint_constants_logq
residual_endpoint_excess_logq
classification
proof_effect
```

For `replay_model = larger_field_search`, `capture_id` may identify the search
campaign rather than a fixed support. The row is valid if the replay-field
profile itself matches:

```text
a = 4
delta = 3
comp = 1
g = 2
```

For `replay_model = symbolic_field_lift`, the row must also emit:

```text
symbolic_matrix_key
symbolic_condition_key
specialization_values_hash
nonzero_minor_witnesses
```

and the replay-field recomputation must match the target profile.

## Residual Acceptance

For either valid model, acceptance is:

```text
residual_endpoint_excess_logq <= 0
```

where:

```text
residual_endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq
  - finite_endpoint_constants_logq.
```

For the dense connected target:

```text
endpoint_bound_logq = 0
```

so the residual is simply:

```text
observed_endpoint_logq - finite_endpoint_constants_logq.
```

Bare finite-field excess over `0` is not a theorem failure unless the residual
remains positive after projective, nonzero-root, Gaussian, and exceptional-strata
normalizations.

## Practical Direction

For the current proof lane, the recommended next proof-valid replay path is:

```text
search GF(31) or larger fresh RFC instances for supports with
a=4, delta=3, comp=1, g=2;
then compute residual_endpoint_excess_logq on those supports.
```

Use fixed-coordinate replay only as a stability diagnostic unless a symbolic
field-lift model is available.
