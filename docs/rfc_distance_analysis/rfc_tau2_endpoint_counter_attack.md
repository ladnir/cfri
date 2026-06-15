# RFC Tau=2 Endpoint Counter Attack

Scope: original non-systematic RFC only.

This note defines the local `tau=2` endpoint-counter falsification gate.  It is upstream of the
paired-spine cascade test: before asking whether a `tau=2` endpoint composes through recursion, we
first need to know whether the endpoint count itself is being bounded with the right exponent.

The main target is dense connected support profiles with:

```text
tau = 2
comp = 1
delta >= 2
g >= 2
```

The secondary target is aggregate rows that look harmless by `(delta, comp, g)` but may hide
bad exact-support subrows, especially the observed aggregate shape:

```text
delta = 2
comp = 2
g = 0
```

## Endpoint Quantities

For every exact support profile, record:

```text
a
tau
delta
comp
g
support_profile
root_profile
observed_endpoint_logq
component_endpoint_logq
generic_endpoint_logq
predicted_endpoint_logq
endpoint_excess_logq
```

The predicted exponent should be the proof-side endpoint bound for the same exact profile.  In the
current intended model this is usually:

```text
predicted_endpoint_logq =
  max(component_endpoint_logq, generic_endpoint_logq)
```

unless a sharper theorem row gives a smaller explicit value.  Do not add component and generic
freedom unless the theorem explicitly allows them to be independent.

Define:

```text
endpoint_excess_logq =
  observed_endpoint_logq - predicted_endpoint_logq
```

## Primary Attack: Dense Connected `g >= 2`

Dense connected profiles are the highest-risk endpoint family:

```text
comp = 1
delta >= 2
g >= 2
tau = 2
```

Reason:

```text
connected support can create component/exterior freedom,
while g >= 2 says the generic root-line endpoint also has nontrivial dimension.
```

The proof needs these to be alternative descriptions of the same algebraic degrees of freedom, or
else it must charge both.  The falsification risk is that the actual endpoint variety has larger
dimension than either endpoint model alone.

Concrete counter-signals:

```text
endpoint_excess_logq > 0
endpoint_excess_logq >= 1
128 * endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Interpretation:

```text
endpoint_excess_logq > 0:
  the theorem exponent is too small for this exact endpoint profile.

endpoint_excess_logq >= 1:
  at least one full q-dimension is missing.

production-scaled > 41.83 bits:
  enough endpoint slack is lost to threaten the current e=71 target.
```

The `41.83` bit threshold is the approximate margin between the current best exact-support scalar
stress at `e=71` and the `2^-80` target.

Highest-priority witness:

```text
tau = 2
comp = 1
delta >= 2
g >= 2
endpoint_excess_logq >= 1
```

This is a direct local-theorem blocker even before recursive composition.

## Secondary Attack: The Aggregate `delta=2, comp=2, g=0` Row

The row:

```text
delta = 2
comp = 2
g = 0
```

should not be treated as the same threat as dense connected `g >= 2`.

Expected interpretation:

```text
comp = 2:
  the support decomposes into two local components.

delta = 2:
  the total support/subcode increment is two, plausibly one from each component.

g = 0:
  there is no generic root-line endpoint dimension in the aggregate row.
```

So the benign model is:

```text
two mostly independent tau=1-like component choices,
not one connected tau=2 endpoint with a reusable generic root family.
```

This row becomes dangerous only if aggregation is hiding structure.

Bad interpretations to test:

```text
1. exact-support inversion failure:
   the aggregate row is harmless, but some exact support inside it has endpoint_excess_logq > 0.

2. hidden connected subprofile:
   the row is labeled comp=2 only after aggregation, while individual exact supports contain a
   connected stratum with effective comp=1 or g>0.

3. root-distribution mismatch:
   g=0 on average, but root lines concentrate on a small set of exact supports and can recur in
   the global recurrence.

4. product-row overcount:
   the row is counted as two independent components, but their endpoint roots are correlated.
```

Concrete counter-signals for this row:

```text
aggregate_endpoint_excess_logq <= 0
but max exact_support_endpoint_excess_logq > 0
```

or:

```text
aggregate g = 0
but some exact support has measured g_exact > 0
```

or:

```text
root_concentration_excess_logq > 0
```

This row is a good audit row because it can tell us whether the endpoint counter is grouping too
coarsely.

## Exact-Support Inversion Failure

The endpoint counter must be usable by the recurrence on exact supports, not only by a cumulative
or aggregated support count.

For each aggregate group:

```text
(a, tau, delta, comp, g)
```

implementation should report:

```text
aggregate_observed_endpoint_logq
aggregate_predicted_endpoint_logq
aggregate_endpoint_excess_logq
max_exact_support_observed_endpoint_logq
max_exact_support_predicted_endpoint_logq
max_exact_support_endpoint_excess_logq
number_of_positive_exact_supports
```

Counter-signals:

```text
max_exact_support_endpoint_excess_logq > 0
max_exact_support_endpoint_excess_logq >= 1
128 * max_exact_support_endpoint_excess_logq + log2(exact_support_multiplicity) > 41.83
```

Strong inversion failure:

```text
aggregate_endpoint_excess_logq <= 0
but max_exact_support_endpoint_excess_logq > 0
```

Interpretation:

```text
the aggregate theorem row is not fine enough for the recurrence.
```

This is especially important for `delta=2, comp=2, g=0`, because a product-looking aggregate row
could contain a small number of exact supports that dominate recursive bad events.

## Root-Distribution Mismatch

The endpoint theorem may be correct as a total count but still too coarse if roots are not
distributed generically across exact supports.

Record root-distribution statistics:

```text
distinct_root_count
max_supports_per_root
average_supports_per_root
max_root_profile_multiplicity
root_entropy_logq
predicted_root_entropy_logq
root_concentration_excess_logq
```

Define:

```text
root_concentration_excess_logq =
  observed_max_root_multiplicity_logq
  - predicted_max_root_multiplicity_logq
```

Counter-signals:

```text
root_concentration_excess_logq > 0
root_concentration_excess_logq >= 1
same root line appears across many exact supports in one aggregate row
same root line appears in both dense g>=2 rows and complete-stride-compatible rows
```

Why this matters:

```text
the global recurrence cares about reusable bad directions, not just total endpoint mass.
```

If roots concentrate, the endpoint count may look safe while recursive chain multiplicity is not.

## Required Implementation Output

Emit aggregate summaries and every positive row for:

```text
endpoint_excess_logq > 0
max_exact_support_endpoint_excess_logq > 0
root_concentration_excess_logq > 0
production_scaled_endpoint_bits > 41.83
```

Per exact support:

```text
a
tau
delta
comp
g
support_profile
root_profile
observed_endpoint_logq
component_endpoint_logq
generic_endpoint_logq
predicted_endpoint_logq
endpoint_excess_logq
profile_multiplicity
production_scaled_endpoint_bits
```

Per aggregate group:

```text
a
tau
delta
comp
g
aggregate_observed_endpoint_logq
aggregate_predicted_endpoint_logq
aggregate_endpoint_excess_logq
max_exact_support_endpoint_excess_logq
number_of_positive_exact_supports
distinct_root_count
max_supports_per_root
average_supports_per_root
root_concentration_excess_logq
```

For the special row, include a named summary:

```text
row_delta2_comp2_g0_status
row_delta2_comp2_g0_max_exact_excess
row_delta2_comp2_g0_root_concentration_excess
row_delta2_comp2_g0_inversion_failure
```

## Clean Result

The endpoint-counter gate is clean if:

```text
max endpoint_excess_logq <= 0
max exact_support_endpoint_excess_logq <= 0
max root_concentration_excess_logq <= 0
no production-scaled endpoint row exceeds 41.83 bits
dense connected g>=2 rows have no positive endpoint excess
the delta=2, comp=2, g=0 row has no exact-support inversion failure
```

Interpretation:

```text
the tau=2 endpoint theorem is likely structurally correct at this gate,
and remaining falsification pressure should move from local endpoint counts to recursive
composition.
```

## Dirty Result Triage

Classify dirty rows in this order:

```text
1. Dense connected theorem failure:
   comp = 1, g >= 2, endpoint_excess_logq > 0.

2. Full missing dimension:
   endpoint_excess_logq >= 1.

3. Production failure:
   128 * endpoint_excess_logq + log2(profile_multiplicity) > 41.83.

4. Exact-support inversion failure:
   aggregate safe, exact support unsafe.

5. Root-distribution mismatch:
   total count safe, reusable root concentration unsafe.
```

The first three threaten the local theorem directly.  The last two threaten how the theorem is fed
into the global recurrence.

## Next Attack If Clean

If the endpoint counter is clean, move to the paired-spine composition test:

```text
tau = 2 paired-spine gate
depth 4 plus one paired lift
chain length = 2
t in {3,4}
compose dense tau=2 endpoints with complete-stride flags
```

The next counter-signal there is:

```text
chain_excess_logq > 0
chain_excess_logq >= 1
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83
```

Reason:

```text
if local endpoint counts are clean, the remaining tau=2 risk is that individually valid endpoint
profiles compose through paired compression with too little entropy loss.
```

## Feedback For Proof Agent

This gate tests whether the tau=2 local endpoint theorem is fine enough for exact-support
recurrence use.

The proof should make explicit:

```text
1. dense connected g>=2 profiles are bounded by a single endpoint exponent;
2. component and generic endpoint freedoms are not double-counted;
3. aggregate rows can be refined to exact supports without losing exponent;
4. root directions are sufficiently distributed, or concentration is explicitly charged.
```

The most likely subtle failure is not the total endpoint count.  It is an aggregate row whose total
mass is safe but whose exact-support or root-conditioned distribution is too concentrated for the
recursive first moment.
