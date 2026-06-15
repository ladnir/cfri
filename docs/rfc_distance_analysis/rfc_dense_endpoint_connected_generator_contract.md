# RFC Dense Endpoint Connected Generator Contract

Scope: original non-systematic RFC only. This note defines the proof and usage
contract for a connectivity-biased dense tau-2 endpoint generator.

The generator may bias toward connected supports. That is acceptable for finding
candidate endpoint profiles. It is discovery-only unless it emits exact
multiplicity or valid importance weights.

## Status Of Generator Output

Default status:

```text
generator_mode = discovery
proof_effect = profile_discovery_only
```

In discovery mode, the generator may use any heuristic that helps find dense
connected endpoint rows:

```text
prefer connected coordinate sets
prefer high-rank supports
prefer target g minimizers
require comp=1
require delta=3
require g=2
```

The resulting hit rate is not a multiplicity estimate. It must not be used in
the global first moment.

The generator output becomes admissible for multiplicity only if:

```text
generator_mode = exact_enumeration
```

or:

```text
generator_mode = importance_weighted
proposal_probability is numeric
importance_weight is numeric
base_support_space is defined
all filters are included in the probability model
```

## Why Exact Rows Remain Valid

For endpoint-theorem testing, bias in how a support was found does not bias the
local endpoint count for that fixed support.

Once a row has:

```text
exact_support_status = exact
delta/comp/g recomputed over the target field
canonical endpoint de-duplication
exact endpoint count
finite constants subtracted
```

then it is valid evidence for the local statement:

```text
observed_endpoint_logq(A)
  <= endpoint_bound_logq(A) + finite_endpoint_constants_logq(A).
```

A positive residual on such a row is a real local counterexample candidate even
if the generator was biased. What remains biased is any claim about how many
such supports exist.

## Required Metadata

Each generated row must emit:

```text
generator_name
generator_version
generator_mode
proposal_description
base_support_space_description
proposal_probability
importance_weight
filter_list
filter_order
field_size
root_distribution
root_seed
fold_algebra
construction
depth
expansion_c
local_node_id
support_A_coordinates
coordinate_path_labels
support_A_key
active_copy_profile_key
matrix_construction_key
```

For discovery-only rows:

```text
proposal_probability = NA
importance_weight = NA
```

For weighted rows, both must be numeric and audited.

## Recomputing Delta, Comp, And G

The generator may propose supports using cached or heuristic labels, but the
reported proof row must recompute the profile over the target field.

Required rank data:

```text
rank_A(B) for all B subset A
rank_subset_table_hash
rank_before
rank_after
restricted_rank_A
```

Compute:

```text
delta = rank_before - rank_after
```

and also report whether:

```text
restricted_rank_A = delta
```

when the local endpoint convention identifies the two.

Compute `comp` from the replay-field matroid:

```text
component_partition_key
comp
loop_count
parallel_class_profile
circuit_witnesses
```

Compute:

```text
g = 2*delta - min_{B subset A} (a - |B| + 2*rank_A(B)).
```

Emit:

```text
g_min_value
g_minimizer_keys
```

For the smallest dense connected target, the desired profile is:

```text
a = 4
tau = 2
delta = 3
comp = 1
g = 2
loop_count = 0
exact_support_status = exact
```

Rows that do not recompute to the target profile can still be useful telemetry,
but they are not evidence for that dense connected endpoint target.

## Endpoint And Residual Columns

Each row intended for endpoint-theorem evidence must emit:

```text
contained_E_A_2
exact_E_A_2
kappa_distribution
canonical_endpoint_event_count
duplicate_certificate_count
observed_endpoint_logq
endpoint_generic_logq
endpoint_component_logq
endpoint_bound_logq
projective_line_factor_logq
nonzero_root_normalization_logq
gaussian_normalization_extra_logq
exceptional_strata_extra_logq
finite_endpoint_constants_logq
endpoint_excess_logq
residual_endpoint_excess_logq
classification
proof_effect
```

where:

```text
endpoint_generic_logq   = 2*g - 4
endpoint_component_logq = comp + 2*delta - 4 - a
endpoint_bound_logq     = max(endpoint_generic_logq,
                              endpoint_component_logq)
```

and:

```text
residual_endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq
  - finite_endpoint_constants_logq.
```

The acceptance threshold is:

```text
residual_endpoint_excess_logq <= 0
```

up to declared log-rounding tolerance.

Suggested tolerance for exact integer counts converted to logs:

```text
log_rounding_tolerance = 1e-9
```

If the residual is positive after exact-support inversion, canonical
de-duplication, and finite-constant subtraction, the row is a local endpoint
theorem blocker.

## Safe Use Of Filters

Filters such as:

```text
--require-components 1
--require-delta 3
--require-g 2
--require-support-size 4
```

are safe for discovery if they are evaluated after recomputing the profile over
the same field/root instance used for the endpoint count.

They are unsafe for multiplicity unless their effect on the sampling measure is
included in `proposal_probability` and `importance_weight`.

Do not use:

```text
accepted_rows / generated_rows
```

from a filtered generator as a global support frequency.

## Using GF31/GF101 Profiles For Later Replay

A found `GF(31)` or `GF(101)` dense profile can be used in two proof-valid ways.

### Larger-Field Profile Search Seed

Preserve the profile metadata:

```text
support_A_coordinates
coordinate_path_labels
rank_subset_table_hash
component_partition_key
circuit_witnesses
g_minimizer_keys
matrix_construction_key
filter_recipe
```

Then use it to guide a larger-field search. The larger-field row is valid only
if it recomputes to the target profile over the larger field.

The same coordinate IDs are helpful but not required. A new support with the
same recomputed profile is valid profile evidence.

### Symbolic Lift Candidate

If the implementation can preserve symbolic local-fold data, emit:

```text
symbolic_matrix_key
symbolic_minor_witnesses
symbolic_nonvanishing_conditions
specialization_values_hash
```

Then a later large-field replay may test the same algebraic stratum by
specializing the symbolic object into the larger field. The profile still must
be recomputed over the target field.

## Classification

Use:

```text
classification = discovery_profile_found
```

when a biased generator finds a row with the requested recomputed profile.

Use:

```text
classification = discovery_clean_residual
```

when that row has:

```text
residual_endpoint_excess_logq <= 0.
```

Use:

```text
classification = local_endpoint_counterexample
```

when that row has:

```text
residual_endpoint_excess_logq > 0.
```

Use:

```text
classification = multiplicity_not_admissible
```

when hit rates or counts are reported without exact enumeration or valid
importance weights.

Use:

```text
classification = replay_seed
```

for `GF(31)` or `GF(101)` profiles preserved for later large-field replay.

## Proof Use

The generator can accelerate the search for dense tau-2 endpoint stress rows. It
does not by itself estimate the contribution of those rows to the global
certificate.

For the proof lane, the useful outputs are:

```text
valid target profile found over GF31/GF101
residual_endpoint_excess_logq for that exact row
metadata sufficient to replay or search the same profile over larger fields
```

The key pass/fail signal remains the residual, not the biased sampler frequency.
