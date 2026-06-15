# RFC Dense Endpoint Connectivity Sampler Attack

Scope: original non-systematic RFC only.

This note defines how to use connectivity-biased sampling adversarially for dense connected
`tau=2` endpoint profiles without turning the biased sample into biased evidence.

The target family remains:

```text
tau = 2
comp = 1
delta in {2,3} first
g >= 2
```

The sampler is a discovery tool.  It should bias toward connected supports and high-root
interaction, but every dirty candidate must be rechecked as a certified exact profile.

## Why Connectivity-Biased Sampling Is Useful

Uniform support search can waste most attempts on disconnected or product-like rows.  The current
live threat is connected:

```text
comp = 1
g >= 2
```

So a useful sampler should bias toward supports with:

```text
one component
many short dependency paths between support coordinates
rank(S) - rank(S \ A) >= 2
nontrivial root-line/root-plane interactions
possible overlap with complete-stride endpoint planes or lines
```

This is adversarial because it searches where the endpoint theorem is most likely to be stressed,
not where a natural random support distribution spends its mass.

## Discovery Target

First target:

```text
a = minimal discovered support size
tau = 2
delta = 2
comp = 1
g = 2
```

Second target:

```text
a = 4
tau = 2
delta = 3
comp = 1
g = 2
```

The second target is included because of the GF(11) scout row, but it should not replace the
minimal `delta=2` search.

## Serious Connected Profile

A sampled profile becomes serious only after exact certification.

Minimum certificate:

```text
tau = 2
comp = 1
delta >= 2
g >= 2
canonical support hash
component certificate
delta certificate
g certificate
root profile
finite-constant correction
residual_endpoint_excess_logq
```

Counter-signal:

```text
residual_endpoint_excess_logq > 0
```

Concerning:

```text
residual_endpoint_excess_logq >= 0.02
```

Serious:

```text
residual_endpoint_excess_logq >= 0.05
```

Full local-theorem failure:

```text
residual_endpoint_excess_logq >= 1
```

Production-threatening:

```text
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Secondary serious signals:

```text
root_concentration_excess_logq > 0
root_concentration_excess_logq >= 0.05
max_exact_support_residual_endpoint_excess_logq > 0
complete_stride_intersection_excess > 0
```

Highest-priority witness:

```text
comp = 1
delta = 2
g = 2
residual_endpoint_excess_logq >= 0.05
root_concentration_excess_logq > 0
stable over a large field
```

## Fields And Seeds

Use fields in two roles:

```text
small/mid fields:
  discovery and shape generation.

large fields:
  evidence for asymptotic residual exponent.
```

Recommended sequence:

```text
GF(31):
  cheap guarded discovery.

GF(101) or GF(1009):
  first stability check.

GF(65537):
  large-prime replay for credible endpoint evidence.
```

Seed policy:

```text
one dirty seed:
  candidate only.

two dirty seeds in the same field with matching certified profile type:
  meaningful warning.

same certified profile type dirty over a mid-size field and GF(65537):
  serious warning.

same exact support/root profile dirty over GF(65537):
  strongest evidence short of symbolic proof.
```

For negative evidence:

```text
one clean biased sample means little.
multiple clean seeds only say the sampler failed to find a counter-signal.
they do not prove the endpoint theorem.
```

A clean result becomes useful only if implementation reports coverage:

```text
number of attempted supports
number of connected supports found
number of certified comp=1,delta>=2,g>=2 profiles
deduplicated canonical profile count
acceptance/rejection reasons
```

## Preventing Biased Evidence

The sampler is allowed to be biased for discovery.  The reporting cannot hide that bias.

Implementation must separate:

```text
discovery distribution
certified profile verification
frequency/probability claims
```

Rules:

```text
1. Do not estimate global profile prevalence from biased samples unless importance weights are
   recorded.

2. Do not claim a clean theorem row from failing to find a dirty sample.

3. Do not compare residuals across rows unless delta, comp, g, support profile, and root profile
   certificates match.

4. Deduplicate by canonical support/root hash before reporting maxima.

5. Report rejected candidates and why they failed certification.
```

The output should include the sampler knobs:

```text
field
seed
proposal_method
connectivity_bias_parameters
support_size_distribution
target_delta_values
target_g_values
max_attempts
acceptance_count
rejection_count_by_reason
canonical_profile_count
duplicate_profile_count
```

If the sampler uses weights, report:

```text
proposal_probability_or_weight
importance_weight
weighted_profile_estimate
unweighted_profile_count
```

If weights are not available, mark all prevalence estimates as:

```text
discovery-only, not statistically calibrated
```

## Required Candidate Report

For every positive or near-positive candidate, save:

```text
ambient_depth
ambient_coordinate_count
ordered support coordinates
visible support A
singleton set S
paired-zero set P, if present
row/block labels
stride-class labels
component partition
rank(S)
rank(S \ A)
delta certificate
comp=1 certificate
g certificate
root-profile certificate
canonical support hash
canonical root hash
local evaluation matrix pattern
fold-parameter seed
T-values or symbolic fold labels
observed_endpoint_count
predicted_endpoint_count
finite_constant_logq
residual_endpoint_excess_logq
profile_multiplicity
root_concentration_excess_logq
complete_stride_intersection_excess
```

This support capture is mandatory before large-field replay.  Same coordinates alone are not
enough, because the GF31 mismatch showed that component structure can change across fields.

## Failure Modes

### GF-Specific Connectivity

Pattern:

```text
candidate has comp=1 over one field,
but the same captured profile cannot be certified over a larger field.
```

Interpretation:

```text
small-field or seed-specific artifact.
```

Action:

```text
archive as discovery-only and continue guarded search.
```

### Profile Does Not Replay

Pattern:

```text
same aggregate tuple appears,
but canonical support/root profile changes.
```

Interpretation:

```text
not the same endpoint object.
```

Action:

```text
do not compare residual exponents directly.
capture the new profile separately.
```

### Root Concentration Disappears

Pattern:

```text
root_concentration_excess_logq > 0 in discovery field,
but <= 0 over larger field.
```

Interpretation:

```text
finite-field collision or constant mass.
```

Action:

```text
do not route to paired-spine attack unless residual endpoint excess persists.
```

### Root Concentration Persists

Pattern:

```text
root_concentration_excess_logq > 0 over large field or multiple fields.
```

Interpretation:

```text
reusable bad directions may be more concentrated than the recurrence assumes.
```

Action:

```text
route to paired-spine composition even if total endpoint residual is small.
```

### Exact-Support Inversion Changes

Pattern:

```text
aggregate row clean,
but a certified exact support has positive residual.
```

Interpretation:

```text
aggregate endpoint bound is too coarse for recurrence use.
```

Action:

```text
promote exact-support row to replay target.
```

## Clean Sampler Result

A biased sampler run is clean only in the limited sense:

```text
no certified candidate with residual_endpoint_excess_logq > 0
no root_concentration_excess_logq > 0
no exact-support inversion failure
coverage and rejection statistics reported
```

Interpretation:

```text
the sampler did not find a counter-signal.
```

It should not be stated as:

```text
no counter-signal exists.
```

## Next Attack If A Serious Profile Is Found

If the sampler finds a serious connected profile, the next attack is targeted large-field replay:

```text
same certified support/root profile
GF(101) or GF(1009)
GF(65537)
residual_endpoint_excess_logq after finite constants
root concentration
exact-support inversion status
```

If the large-field residual persists, route it into:

```text
tau=2 paired-spine composition
depth 4 plus one paired lift
chain length = 2
t in {3,4}
endpoint seed = captured dense connected profile
```

Composition counter-signals:

```text
chain_excess_logq > 0
chain_excess_logq >= 1
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83
```

## Next Attack If Sampler Is Clean

If the sampler is clean with adequate reporting, move in this order:

```text
1. widen support size a by one for comp=1,delta=2,g=2;
2. test comp=1,delta=3,g=2 with large-field guarded search;
3. test complete-stride intersection with any dense connected endpoint rows found;
4. move to tau=2 paired-spine chain length 2 using clean endpoint bounds.
```

Do not jump from a clean biased sampler directly to a proof claim.  Use it to prioritize which
exact theorem rows need formal endpoint bounds.

## Feedback For Proof Agent

Connectivity-biased sampling can find candidate obstructions, but it cannot certify absence.  The
proof lane should only consume:

```text
certified exact profile rows,
large-field-stable residual exponents,
root-concentration summaries,
and explicit coverage/rejection data.
```

Any claim based only on biased non-discovery should be treated as heuristic, not proof support.
