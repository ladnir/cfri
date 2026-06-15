# RFC Dense Endpoint Connected Generator Attack

Scope: original non-systematic RFC only.

This note defines adversarial use of a connectivity-biased generator for dense connected `tau=2`
endpoint profiles.  The generator is allowed to target suspicious supports directly.  That makes
it good for finding counter-signals, but bad for estimating how common they are.

Target family:

```text
tau = 2
comp = 1
delta in {2,3} first
g >= 2
```

Primary target:

```text
tau = 2
comp = 1
delta = 2
g = 2
minimal support size a
```

Secondary target:

```text
tau = 2
comp = 1
delta = 3
g = 2
a = 4
```

## Adversarial Generator Use

The generator should deliberately build supports with:

```text
one connected component
delta >= 2
high root interaction
many short dependency paths
possible overlap with complete-stride endpoint planes or lines
minimal support size first
```

Useful generator biases:

```text
force comp=1 candidates
force rank(S) - rank(S \ A) in {2,3}
prefer support shapes with multiple root incidences
prefer canonical shapes that survive row/stride relabeling
seed from known GF(11) scout profiles only as shape hints, not as evidence
```

The generator should not be used to claim prevalence.  It is a search tool for existence of a
bad exact profile.

## Meaningful Hit

A generated row is a meaningful hit only if it is certified after generation.

Minimum certificate:

```text
canonical support hash
ordered support coordinates
visible support A
singleton set S
component partition
rank(S)
rank(S \ A)
delta certificate
comp=1 certificate
g certificate
root-profile certificate
finite-constant correction
residual_endpoint_excess_logq
```

Meaningful hit:

```text
tau = 2
comp = 1
delta >= 2
g >= 2
residual_endpoint_excess_logq > 0
```

Concerning hit:

```text
residual_endpoint_excess_logq >= 0.02
```

Serious hit:

```text
residual_endpoint_excess_logq >= 0.05
```

Full endpoint-theorem failure:

```text
residual_endpoint_excess_logq >= 1
```

Production-threatening hit:

```text
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

## Biased Discovery Noise

A generated row is only discovery noise if:

```text
it is dirty in one small field only,
the exact support/root profile is not saved,
same coordinates change comp/delta/g over another field,
finite constants explain the raw endpoint excess,
root concentration disappears over larger fields,
or the row cannot be deduplicated by canonical support/root hash.
```

A generator can manufacture many near-misses.  Do not escalate rows that fail certification.

Noise examples:

```text
raw endpoint_excess_logq > 0 but residual_endpoint_excess_logq <= 0
comp=1 over GF(11) but not over GF(31)
same aggregate tuple but different exact support profile
positive root concentration in one small field only
duplicate generated rows counted as distinct evidence
```

## Required Evidence For A Serious Profile

### 1. Residual Endpoint Excess

The live metric is:

```text
residual_endpoint_excess_logq =
  endpoint_excess_logq - finite_constant_logq
```

Evidence levels:

```text
> 0:
  warning after certification.

>= 0.02:
  concerning if repeated in a large field or independent seed.

>= 0.05:
  serious endpoint warning.

>= 1:
  full missing q-dimension.
```

Production criterion:

```text
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

### 2. Root Concentration

Report:

```text
distinct_root_count
max_supports_per_root
average_supports_per_root
root_entropy_logq
predicted_root_entropy_logq
root_concentration_excess_logq
canonical worst_root_hash
```

Counter-signals:

```text
root_concentration_excess_logq > 0
root_concentration_excess_logq >= 0.05
same root profile appears across multiple generated exact supports
same root profile survives large-field replay
```

Root concentration can be serious even when total endpoint residual is small, because recursive bad
events care about reusable low-dimensional directions.

### 3. Field, Seed, And Profile Stability

Evidence ladder:

```text
one generated dirty row in one small field:
  candidate only.

same certified profile type dirty across two seeds in the same field:
  meaningful warning.

same certified profile type dirty over GF(31) and a mid-size field:
  strong warning.

same exact support/root profile dirty over GF(65537):
  serious endpoint evidence.

same residual exponent bounded away from zero across increasing fields:
  asymptotic counter-signal.
```

Do not compare rows unless these match:

```text
delta certificate
comp certificate
g certificate
canonical support profile
root profile
finite constant correction rule
```

Same coordinates alone are not enough.  The GF31 mismatch showed that coordinates can keep their
names while the component profile changes.

### 4. Exact-Support Inversion

Report:

```text
aggregate_residual_endpoint_excess_logq
max_exact_support_residual_endpoint_excess_logq
number_of_positive_exact_supports
worst_exact_support_hash
whether worst hash is stable across seeds/fields
```

Counter-signals:

```text
aggregate residual <= 0
but max exact-support residual > 0
```

or:

```text
worst exact-support residual persists over large fields
```

Interpretation:

```text
the aggregate endpoint theorem may be too coarse for recurrence use.
```

## Required Generator Report

Every generator run must report:

```text
field
seed
generator_version
target tuple
generation rules
connectivity bias parameters
number_of_candidates_generated
number_certified_comp1
number_certified_delta_ge_2
number_certified_g_ge_2
number_positive_residual
number_positive_root_concentration
deduplicated_support_profile_count
duplicate_count
rejection_count_by_reason
```

For every positive or near-positive row:

```text
canonical support hash
canonical root hash
support coordinates
visible support A
singleton set S
component partition
delta certificate
comp certificate
g certificate
observed_endpoint_count
predicted_endpoint_count
finite_constant_logq
residual_endpoint_excess_logq
profile_multiplicity
production_scaled_residual_bits
root_concentration_excess_logq
complete_stride_intersection_excess
```

If the generator cannot emit the certificates, the row is discovery-only.

## Clean Generator Result

A clean generator run means:

```text
no certified generated row has residual_endpoint_excess_logq > 0,
no certified generated row has root_concentration_excess_logq > 0,
no exact-support inversion failure appears,
and coverage/rejection/deduplication statistics are reported.
```

It does not mean:

```text
no bad connected endpoint profile exists.
```

Biased non-discovery is a prioritization signal, not proof evidence.

## Next Move If The Generator Finds A Serious Profile

Run targeted replay:

```text
same certified support/root profile
GF(101) or GF(1009)
GF(65537)
residual_endpoint_excess_logq
root concentration
exact-support inversion
```

If residual or root concentration persists, route to:

```text
tau=2 paired-spine composition
depth 4 plus one paired lift
chain length = 2
t in {3,4}
endpoint seed = generated dense connected profile
```

Composition counter-signals:

```text
chain_excess_logq > 0
chain_excess_logq >= 1
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83
```

## Next Move If The Generator Finds No Target Rows

If no target rows are found, do not treat this as a theorem result.  Move in this order:

```text
1. widen the generator target from delta=2,g=2 to delta=3,g=2;
2. increase support size a by one for comp=1,delta=2,g=2;
3. switch to guarded larger-field search for any certified comp=1,delta>=2,g>=2 profile;
4. if still clean, use the endpoint theorem as the working model and attack recursive composition
   through tau=2 paired-spine chains.
```

The most useful single next step after a no-hit run is:

```text
report why candidates failed:
  disconnected,
  wrong delta,
  g < 2,
  finite constants absorb residual,
  duplicate profile,
  certification missing.
```

Those failure counts tell the integrator whether the generator is missing the target family or the
target family is genuinely rare.

## Feedback For Proof Agent

The proof lane should only use generator output after certification and replay.  A dirty generated
row is not proof by itself, but a large-field-stable certified row with positive residual is a real
local endpoint obstruction.

The proof-relevant claim being attacked is:

```text
dense connected tau=2 endpoint profiles with g>=2 have no positive residual endpoint exponent
after finite constants and exact-support refinement.
```
