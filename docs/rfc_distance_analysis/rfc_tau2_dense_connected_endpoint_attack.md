# RFC Tau=2 Dense Connected Endpoint Attack

Scope: original non-systematic RFC only.

The product-style row is no longer the main endpoint threat:

```text
tau = 2
delta = 2
comp = 2
g = 0
endpoint_bound_logq = -4
```

Treat that row as benign once the product closed-form, exact-support refinement, and root
distribution checks are clean.  The live local endpoint attack is now the dense connected family:

```text
tau = 2
comp = 1
delta = 2
g >= 2
```

## Smallest Profile To Attack

The first target should be:

```text
tau = 2
comp = 1
delta = 2
g = 2
a = minimal support size where this profile appears
```

Do not start with larger `g`, larger `delta`, or disconnected rows.  The minimal connected `g=2`
profile is the cleanest local theorem test:

```text
comp = 1:
  no product decomposition should be available.

delta = 2:
  smallest tau=2 support/subcode increment.

g = 2:
  smallest nontrivial generic endpoint/root dimension.

minimal a:
  least room for unrelated support multiplicity to obscure the exponent.
```

If multiple exact profiles tie at the same `a`, prioritize:

```text
1. largest observed_endpoint_logq
2. largest root concentration
3. smallest automorphism orbit count / simplest support shape
4. nontrivial intersection with complete-stride planes or lines
```

## Endpoint Bound To Test

For each exact support profile, compute:

```text
component_endpoint_logq
generic_endpoint_logq
predicted_endpoint_logq = max(component_endpoint_logq, generic_endpoint_logq)
observed_endpoint_logq
endpoint_excess_logq = observed_endpoint_logq - predicted_endpoint_logq
```

The danger is that the connected support and the generic `g=2` endpoint supply independent degrees
of freedom, while the theorem charges only the maximum.

Primary counter-signal:

```text
endpoint_excess_logq > 0
```

Strong counter-signal:

```text
endpoint_excess_logq >= 1
```

Production counter-signal at `q=2^128`, `e=71`:

```text
128 * endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Interpretation:

```text
endpoint_excess_logq > 0:
  theorem exponent is too small or the profile is being grouped too coarsely.

endpoint_excess_logq >= 1:
  one full q-dimension is missing.

production-scaled > 41.83 bits:
  enough slack is lost to threaten the current e=71 target.
```

## Exact-Support Inversion Failure

Even if the aggregate connected row is safe, exact-support inversion can fail.

The bad pattern is:

```text
aggregate_endpoint_excess_logq <= 0
but max_exact_support_endpoint_excess_logq > 0
```

or:

```text
aggregate g = 2
but one exact support has effective g_exact > 2 or extra root concentration
```

Implementation should report both aggregate and exact-support views:

```text
aggregate_observed_endpoint_logq
aggregate_predicted_endpoint_logq
aggregate_endpoint_excess_logq
max_exact_support_observed_endpoint_logq
max_exact_support_predicted_endpoint_logq
max_exact_support_endpoint_excess_logq
number_of_positive_exact_supports
worst_exact_support_profile
```

Counter-signals:

```text
max_exact_support_endpoint_excess_logq > 0
max_exact_support_endpoint_excess_logq >= 1
128 * max_exact_support_endpoint_excess_logq
  + log2(exact_support_multiplicity) > 41.83
```

Why this matters:

```text
the recurrence consumes exact support conditions, not just aggregate endpoint counts.
```

An aggregate proof row is insufficient if the mass is concentrated in a small exact-support
subfamily that recurs through the global first moment.

## Root-Distribution Checks

For dense connected `g=2`, root distribution is part of the endpoint theorem, not an optional
diagnostic.

Report:

```text
distinct_root_count
max_supports_per_root
average_supports_per_root
root_entropy_logq
predicted_root_entropy_logq
root_concentration_excess_logq
worst_root_profile
```

Counter-signals:

```text
root_concentration_excess_logq > 0
root_concentration_excess_logq >= 1
same root line appears across many exact supports in the same connected profile
same root profile also intersects complete-stride endpoint flags
```

Interpretation:

```text
total endpoint count may be safe while reusable root directions are not.
```

This is especially relevant for the recursive certificate, because the bad event is driven by
reusable low-dimensional directions.

## Small-Field Evidence

Small-field evidence is useful for:

```text
finding support shapes,
checking component labels,
discovering candidate exact profiles,
testing whether a row is product-like or connected,
debugging root-distribution output.
```

Small-field evidence is misleading for endpoint exponents when:

```text
determinants vanish accidentally,
unrelated root lines collide,
projective line counts are too small to show asymptotic dimension,
field-specific algebra creates fake g values,
or a dirty row disappears over a large prime.
```

The earlier size-8 complete-stride red flag over `GF(5)` is the model warning: it looked broad
until the large-prime scan separated real structure from small-field degeneration.

Policy:

```text
dirty small-field row:
  candidate generator only; not a counter-signal until reproduced over a large prime or symbolic
  profile calculation.

clean small-field row:
  useful but not conclusive; it may miss an asymptotic q-dimensional fiber.

large-prime stable dirty row:
  serious falsification signal.
```

For this endpoint gate, small fields should guide which exact support profiles to inspect, while
`endpoint_excess_logq` claims should be based on large-prime-stable or symbolic dimension data.

## Required Implementation Output

The next query should emit one table grouped by exact support profile and one aggregate summary.

Exact-support rows:

```text
a
tau
delta
comp
g
support_profile
root_profile
component_endpoint_logq
generic_endpoint_logq
predicted_endpoint_logq
observed_endpoint_logq
endpoint_excess_logq
profile_multiplicity
production_scaled_endpoint_bits
distinct_root_count
max_supports_per_root
root_concentration_excess_logq
complete_stride_intersection_excess
```

Aggregate summary:

```text
minimal_a_for_comp1_delta2_g2
total_exact_profiles_at_minimal_a
max_endpoint_excess_logq
max_exact_support_endpoint_excess_logq
max_production_scaled_endpoint_bits
max_root_concentration_excess_logq
number_of_positive_endpoint_excess_profiles
number_of_positive_root_concentration_profiles
```

Emit every row satisfying:

```text
endpoint_excess_logq > 0
max_exact_support_endpoint_excess_logq > 0
root_concentration_excess_logq > 0
production_scaled_endpoint_bits > 41.83
complete_stride_intersection_excess > 0
```

## Clean Result

The dense connected endpoint gate is clean if:

```text
max endpoint_excess_logq <= 0
max exact_support_endpoint_excess_logq <= 0
max root_concentration_excess_logq <= 0
no production-scaled endpoint row exceeds 41.83 bits
no minimal comp=1, delta=2, g=2 row intersects complete-stride flags beyond generic position
```

Interpretation:

```text
the smallest dense connected tau=2 endpoint profile does not falsify the local theorem.
```

If clean, then widen only in controlled steps.

## Next Implementation Query

Run the minimal dense connected endpoint query:

```text
find minimal a with:
  tau = 2
  comp = 1
  delta = 2
  g = 2

for that minimal a:
  enumerate exact support profiles,
  compute observed_endpoint_logq,
  compare to max(component_endpoint_logq, generic_endpoint_logq),
  report exact-support inversion and root-distribution statistics.
```

If that query is clean, next widen in this order:

```text
1. same delta=2, comp=1, g=2, next larger a
2. delta=2, comp=1, g>2
3. delta>2, comp=1, g>=2
4. composition with complete-stride endpoint flags
5. tau=2 paired-spine chain length 2
```

The implementation should not jump directly to broad `tau=2` paired-spine enumeration until this
minimal connected endpoint row is classified.

## Feedback For Proof Agent

This gate tests the local claim:

```text
connected tau=2 endpoint profiles with g>=2 are bounded by the maximum of the component and
generic endpoint exponents, with no hidden additive q-dimension.
```

The proof must be pointwise enough for exact-support recurrence use and must control root
concentration.  A clean product row does not address this claim.
