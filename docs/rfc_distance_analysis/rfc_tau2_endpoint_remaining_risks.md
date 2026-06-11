# RFC Tau=2 Endpoint Remaining Risks

Scope: original non-systematic RFC only.

This note assumes the product-style endpoint row has a closed-form bound:

```text
tau = 2
delta = 2
comp = 2
g = 0
endpoint_bound_logq = -4
```

The purpose is to state what can still be dangerous after that product bound is proven, and to
separate this row from the denser connected `g >= 2` endpoint profiles.

## Interpretation Of The Product Row

The benign interpretation is:

```text
delta = 2:
  total support/subcode increment two.

comp = 2:
  two independent local components.

g = 0:
  no generic root-line endpoint dimension.

endpoint_bound_logq = -4:
  the closed-form product row pays four q-exponents of endpoint loss.
```

In this model, the row is not a connected `tau=2` obstruction.  It behaves like two separate
component constraints whose endpoint choices multiply, and the product formula supplies the full
charge.

So this row should not be used as evidence against the dense connected `g >= 2` theorem unless one
of the remaining risks below fires.

## Remaining Dangerous Behavior After Product Closure

### 1. Exact-Support Inversion Failure

The product bound may close the aggregate row but still be too coarse for the recurrence if exact
supports inside the row are uneven.

Dangerous behavior:

```text
aggregate endpoint_excess_logq <= 0
but some exact support has endpoint_excess_logq > 0
```

Strong danger:

```text
max_exact_support_endpoint_excess_logq >= 1
```

Production danger:

```text
128 * max_exact_support_endpoint_excess_logq
  + log2(exact_support_multiplicity) > 41.83
```

Interpretation:

```text
the closed product count is true only after summing over supports,
but the global recurrence needs a pointwise exact-support bound.
```

Implementation should report:

```text
aggregate_endpoint_excess_logq
max_exact_support_endpoint_excess_logq
number_of_positive_exact_supports
exact_support_multiplicity
worst_support_profile
```

### 2. Root-Distribution Mismatch

Even if every exact support satisfies the product count, root directions may concentrate.

Dangerous behavior:

```text
same root line appears in many exact supports
```

or:

```text
root_concentration_excess_logq > 0
```

Strong danger:

```text
root_concentration_excess_logq >= 1
```

Production danger:

```text
128 * root_concentration_excess_logq
  + log2(root_profile_multiplicity) > 41.83
```

Interpretation:

```text
the total endpoint mass is safe,
but reusable bad directions are more concentrated than the recurrence assumes.
```

Implementation should report:

```text
distinct_root_count
max_supports_per_root
average_supports_per_root
root_entropy_logq
predicted_root_entropy_logq
root_concentration_excess_logq
max_root_profile
```

### 3. Hidden Connected Subprofile

The aggregate labels may be correct globally while a subfamily behaves connected after conditioning
on root or support shape.

Dangerous behavior:

```text
conditioned subprofile has effective comp = 1
```

or:

```text
conditioned subprofile has measured g_exact > 0
```

Counter-signal:

```text
conditioned_endpoint_excess_logq > 0
```

Interpretation:

```text
the product row is mixing multiple strata,
and at least one stratum should be routed to the connected endpoint theorem instead.
```

Implementation should group the row by:

```text
support isomorphism type
component partition
root profile
exact support profile
```

and emit any group whose conditioned exponent is worse than `-4`.

### 4. Product Factor Dependence

The closed form may assume two independent component factors.  The row is dangerous if the two
factors share algebraic choices.

Dangerous behavior:

```text
observed endpoint count > product of component endpoint counts
```

Counter-signal:

```text
product_dependence_excess_logq > 0
```

Strong danger:

```text
product_dependence_excess_logq >= 1
```

Interpretation:

```text
the row is labeled comp=2, but the endpoint roots couple the two components.
```

Implementation should report:

```text
component_1_endpoint_logq
component_2_endpoint_logq
closed_product_endpoint_logq
observed_endpoint_logq
product_dependence_excess_logq
```

## How This Differs From Dense Connected `g >= 2`

The product row:

```text
delta = 2
comp = 2
g = 0
endpoint_bound_logq = -4
```

is a decomposition/inversion audit.

The dense connected rows:

```text
comp = 1
delta >= 2
g >= 2
```

are endpoint theorem stress tests.

For the product row, a clean closed form plus no concentration means:

```text
the row is benign and should be charged by component product structure.
```

For dense connected `g >= 2`, the main question is different:

```text
does observed_endpoint_logq exceed max(component_endpoint_logq, generic_endpoint_logq)?
```

The dense connected counter-signal remains:

```text
endpoint_excess_logq > 0
endpoint_excess_logq >= 1
128 * endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Do not let a clean product row lower priority for dense connected `g >= 2`.  It closes a separate
failure mode.

## Clean Product-Row Result

The product row is clean if all hold:

```text
aggregate_endpoint_excess_logq <= 0
max_exact_support_endpoint_excess_logq <= 0
root_concentration_excess_logq <= 0
conditioned_endpoint_excess_logq <= 0
product_dependence_excess_logq <= 0
no production-scaled excess above 41.83 bits
```

Interpretation:

```text
delta=2, comp=2, g=0 is not the tau=2 endpoint obstruction.
```

At that point, it should become a regression/audit row, not the main falsification target.

## Recommended Next Endpoint Profile

After the product row is resolved, test the smallest dense connected `g >= 2` profile.

Recommended next profile:

```text
tau = 2
comp = 1
delta = 2
g >= 2
minimal support size a where g >= 2 appears
```

If several profiles tie, prioritize in this order:

```text
1. smallest a
2. largest observed endpoint mass
3. largest root concentration
4. profiles that intersect complete-stride endpoint planes or lines
```

Required report:

```text
a
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
root_concentration_excess_logq
complete_stride_intersection_excess
```

Primary counter-signal:

```text
endpoint_excess_logq > 0
```

Strong counter-signal:

```text
endpoint_excess_logq >= 1
```

Production counter-signal:

```text
128 * endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Reason for this recommendation:

```text
once the product row is closed, the live local endpoint risk is whether connected tau=2 supports
with generic root dimension g>=2 have extra algebraic freedom beyond the max endpoint bound.
```

## Feedback For Proof Agent

The proof should use the product row only as a component-product lemma:

```text
delta=2, comp=2, g=0 pays endpoint_bound_logq=-4 pointwise or after a certified refinement.
```

It should not be used as evidence that dense connected `g >= 2` rows are safe.  Those need a
separate endpoint theorem or stratification.

The proof-side requirement after product closure is:

```text
closed aggregate product bound
plus exact-support refinement
plus root-distribution control
```

Without the last two, a true aggregate product formula can still be too weak for the recursive
first-moment certificate.
