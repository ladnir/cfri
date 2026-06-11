# RFC Dense Endpoint Connectivity Sampler Contract

Scope: original non-systematic RFC only. This note defines how a
connectivity-biased sampler may be used for dense tau-2 endpoint work without
biasing the endpoint theorem evidence.

## Core Rule

A biased sampler may be used to discover supports and stress-test fixed-profile
endpoint formulas. It must not be used to estimate global multiplicity unless
the proposal distribution and all conditioning filters are explicitly accounted
for.

In short:

```text
biased discovery is admissible for finding candidates;
biased hit rates are not admissible as multiplicity estimates.
```

## Two Separate Uses

### Profile Discovery

Profile discovery asks:

```text
does there exist a support A over this field/instance with the target profile?
```

For this use, the sampler may bias toward connected supports, high rank, high
`g`, or any other desired profile. It may use filters such as:

```text
--require-components 1
--require-delta 3
--require-g 2
--require-support-size 4
```

provided every accepted row recomputes the profile over the sampled field from
raw rank/connectivity data.

The endpoint evidence from a discovered support is conditional:

```text
for this exact support/profile, observed_endpoint_logq is ...
```

This is enough to find a local theorem counterexample. A positive residual on a
valid exact support remains a real obstruction even if the support was found by
a biased sampler.

### Multiplicity Estimation

Multiplicity estimation asks:

```text
how many supports/profile rows of this type contribute to the global first moment?
```

For this use, connectivity-biased hit rates are not admissible by default.
Filters change the sampling measure. A row frequency after
`--require-components 1` is a conditional frequency among accepted proposals,
not a frequency among all supports.

To use biased sampling for multiplicity, implementation must provide one of:

- exact enumeration of the support/profile class;
- an upper bound on the number of supports in the filtered class;
- an unbiased estimator over the original support space;
- an importance-weighted estimator with known proposal probability for every
  emitted support;
- a rejection-sampling accounting that reports both proposal and base-space
  denominators.

Without this accounting, filtered sampler output must be labeled:

```text
proof_effect = profile_discovery_only
```

## Safe Use Of `--require-components`

`--require-components` is safe when used as a target predicate after rebuilding
the replay-field local matroid.

Safe workflow:

```text
sample or propose support A
recompute rank_A(B) for all required B subset A
compute component partition over the target field
apply --require-components filter
compute delta and g from the same replay-field data
if all target predicates pass, run exact endpoint counter
compute residual_endpoint_excess_logq
```

Unsafe workflow:

```text
filter by a cached GF(11) component label
replay the same coordinate IDs over GF(31)
assume the component count is preserved
use the filtered hit rate as a GF(31) multiplicity estimate
```

The component filter must be evaluated over the same field and root
specialization as the endpoint count.

## Safe Filter Semantics

Filters are admissible as predicates defining a conditional target class:

```text
P(A) =
  support_size(A) = a
  delta(A) = delta0
  comp(A) = comp0
  g(A) = g0
  exact_support(A) = true
```

For endpoint-theorem testing, the sampler may output only supports satisfying
`P(A)`. The proof statement then concerns each emitted exact support:

```text
observed_endpoint_logq(A)
  <= endpoint_bound_logq(A) + finite_constants(A).
```

This statement is not biased by the way `A` was found.

What is biased is any claim of the form:

```text
fraction of supports satisfying P(A) is ...
number of global bad supports is ...
production first-moment contribution is ...
```

unless the sampler provides multiplicity accounting.

## Required Columns

A connectivity-biased sampler row should emit:

```text
sampler_mode                    # discovery, exact_enumeration, importance_weighted
proposal_description
base_support_space_description
proposal_probability
importance_weight
filter_list
filter_evaluated_over_field
field_size
root_seed
support_A_key
support_A_coordinates
support_size
rank_subset_table_hash
delta
comp
component_partition_key
g
g_min_value
g_minimizer_keys
exact_support_status
endpoint_bound_logq
observed_endpoint_logq
finite_endpoint_constants_logq
residual_endpoint_excess_logq
accepted_by_filters
classification
proof_effect
notes
```

For pure discovery mode:

```text
proposal_probability = NA
importance_weight = NA
proof_effect = profile_discovery_only
```

For multiplicity mode, `proposal_probability` and `importance_weight` must be
numeric or the row is not admissible for global first-moment estimates.

## Residual Acceptance

For any discovered dense endpoint support, local theorem evidence is judged by:

```text
residual_endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq
  - finite_endpoint_constants_logq.
```

Accepted local row:

```text
residual_endpoint_excess_logq <= 0
```

Local theorem obstruction:

```text
residual_endpoint_excess_logq > 0
```

after exact-support inversion, canonical de-duplication, and recomputation of
`delta`, `comp`, and `g` over the sampled field.

A biased sampler can validly find the obstruction. It cannot, by itself, say how
large the obstruction is globally.

## Classification

Use:

```text
classification = discovery_clean
```

when a filtered discovery row has nonpositive residual.

Use:

```text
classification = local_endpoint_counterexample
```

when a filtered discovery row has positive residual after all recomputation and
finite constants.

Use:

```text
classification = multiplicity_estimate_admissible
```

only when the sampler reports exact enumeration or valid proposal/importance
weights.

Use:

```text
classification = multiplicity_estimate_biased
```

when filtered hit rates are reported without base-space accounting.

## Practical Guidance

For dense tau-2 work, use connectivity-biased sampling first to find exact
supports with:

```text
a = 4
delta = 3
comp = 1
g = 2
```

Then run the endpoint counter on those supports and judge the residual. Treat
the sampler hit rate as operational telemetry only. Global first-moment
multiplicity must come from exact enumeration, a combinatorial upper bound, or a
properly weighted estimator.

