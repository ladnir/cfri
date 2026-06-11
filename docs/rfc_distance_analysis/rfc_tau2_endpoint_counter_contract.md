# RFC Tau-2 Endpoint Counter Contract

Scope: original non-systematic RFC only. This note specifies the exact endpoint
counter needed to turn `endpoint_excess_logq` from `NA` into a proof-usable
number for the `tau=2` paired-spine gate.

## Purpose

For a fixed exact endpoint support `A`, the tau-2 paired-spine proof needs a
number:

```text
observed_endpoint_logq(A)
```

that can be compared to the layer-codimension endpoint bound:

```text
theta_2(A)
  = max_{2 <= h <= delta(A)} (2h - 4 - gamma_h(A)),

gamma_h(A)
  = codim { ell : dim K_A(ell) >= h } inside (P^1)^A.
```

The older two-endpoint diagnostic bound:

```text
two_endpoint_bound_logq(A)
  = max(2*g(A)-4 if g(A) >= 2 else -infinity,
        comp(A)+2*delta(A)-4-|A|)
```

must still be reported for comparison, but it is not a valid proof bound for all supports.
Connected `a=5, delta=3, comp=1, g=1` rows violate it through a codimension-one `kappa>=2`
layer.

The counter must count canonical endpoint events, not construction certificates.
Contained supports, repeated bases, marked-core labels, and root-line
representatives must be separated from the event count.

## Inputs

Each endpoint counter row must fix:

```text
field
root_distribution
fold_algebra
A
support_mode
delta
comp
g
rank_data_for_delta
component_data_for_comp
rank_or_span_data_for_g
```

Required meanings:

- `field`: the finite field used for the count. For proof calibration this may
  be a large prime field, but the contract is field-agnostic.
- `root_distribution`: RFC root law. For the current construction this is
  independent uniform nonzero fold roots.
- `fold_algebra`: determinant-1 RFC fold algebra. Do not switch to the
  `T'=-T` algebra.
- `A`: canonical local endpoint support set.
- `support_mode`: must be `exact_support` for proof rows.
- `delta`: support subcode drop for `A`, with raw ranks reported.
- `comp`: connected-component count for the restricted local incidence object.
- `g`: generic visible-span parameter used in one layer of `theta_2(A)`.

Rows with `tau=2, delta<2` are impossible endpoint rows. The counter must reject
them before computing an exponent.

## Endpoint Event

For fixed `A`, an endpoint event is a canonical visible two-plane:

```text
V <= local visible span
dim V = 2
```

such that the endpoint constraints vanish exactly on support `A` under the given
root assignment model.

The event key is:

```text
endpoint_event_key =
  hash(
    A_key,
    V_canonical_basis,
    exact_zero_profile_key,
    active_copy_profile_key
  )
```

The `active_copy_profile_key` is included only when copy identity changes the
algebraic endpoint event. Copy labels that produce the same canonical span are
duplicate certificates.

## Exact Support vs Contained Support

Proof rows must use exact support:

```text
zero set at endpoint = A
```

or the exact-support object obtained by explicit inclusion-exclusion/Mobius
inversion from contained-support counts.

Contained-support rows:

```text
zero set contains A
```

are diagnostic only. They may be emitted, but must not be compared directly to
`endpoint_bound_logq`. If a contained-support count is used internally, the
counter must also emit the exact-support inversion data:

```text
contained_count(B) for B superset A
mobius_weight(A,B)
exact_count(A)
```

Any positive excess that disappears after exact-support inversion is classified
as contained-support overcount, not a theorem failure.

## Duplicate Certificates

Do not count these as endpoint events:

- different bases for the same two-plane `V`;
- scalar representatives of the same projective root line;
- generator orderings inside `V`;
- marked-core labels that collapse to the same unmarked support set `A`;
- contained-support witnesses for larger supports `B superset A`;
- repeated construction paths producing the same `endpoint_event_key`;
- active-copy labels that do not change the canonical endpoint span.

The counter should still report their multiplicity so that implementation can
diagnose where excess came from.

## Output Columns

The endpoint counter should emit at least:

```text
field
field_size
root_distribution
fold_algebra
seed
tau
A_key
support_size
support_mode
delta
delta_rank_before
delta_rank_after
comp
component_partition_key
g
g_rank_data
active_copy_profile_key
canonical_endpoint_event_count
duplicate_certificate_count
contained_support_certificate_count
exact_support_event_count
contained_to_exact_correction_logq
observed_endpoint_logq
endpoint_exponent_generic
endpoint_exponent_component
two_endpoint_bound_logq
theta_2_logq
dominant_layer_h
dominant_layer_codim_gamma
layer_profile
endpoint_excess_logq
classification
proof_effect
notes
```

where:

```text
endpoint_exponent_generic   = 2*g - 4
endpoint_exponent_component = comp + 2*delta - 4 - |A|
two_endpoint_bound_logq     = max(endpoint_exponent_generic if g >= 2 else -infinity,
                                  endpoint_exponent_component)
theta_2_logq                = max_h(2h - 4 - gamma_h)
endpoint_excess_logq        = observed_endpoint_logq - theta_2_logq
```

For zero exact events, `observed_endpoint_logq` should be reported as `-inf` or
with an explicit zero-count sentinel, not as `NA`.

## Classification Rules

Use:

```text
classification = accepted_endpoint
```

when `support_mode=exact_support`, `delta>=2`, canonical de-duplication has been
applied, and:

```text
endpoint_excess_logq <= 0
```

Use:

```text
classification = rejected_impossible_tau2_endpoint
```

when `delta<2`.

Use:

```text
classification = stale_two_endpoint_excess
```

when the row violates `two_endpoint_bound_logq` but satisfies `theta_2_logq`. This is not a theorem
failure; it means the row is charged by an intermediate layer.

Use:

```text
classification = contained_support_overcount
```

when a positive contained-support excess becomes nonpositive after
exact-support inversion.

Use:

```text
classification = duplicate_certificate_overcount
```

when the raw count is positive-excess but the canonical endpoint-event count is
nonpositive-excess.

Use:

```text
classification = local_endpoint_excess
```

only when the exact, canonical endpoint count satisfies:

```text
endpoint_excess_logq > 0
```

and the excess is not explained by contained supports, duplicate certificates,
or impossible `delta<2` rows.

## Proof Consequences

A positive `endpoint_excess_logq` forces local theorem repair if it persists
under:

- exact-support counting;
- canonical de-duplication;
- correct `delta`, `comp`, and `g` recomputation from raw data;
- large-field or symbolic-generic checks for the RFC nonzero-root law;
- the layer-codimension calculation `theta_2(A)`.

The likely repairs are a refined local invariant, an active-copy intersection
term, or a higher-layer local flag theorem. The paired-spine recurrence should
not absorb this as a Lift issue; the failure is already at the endpoint theorem.

A positive endpoint excess should trigger `e=72` evaluation only after local
repair options fail or after the production-scaled first moment shows that the
remaining excess consumes the reserved `e=71` slack. Until then, it is a proof
blocker, not evidence that the distance statement is false.

## Admissibility

An endpoint counter output is admissible for the tau-2 paired-spine gate only if:

```text
support_mode = exact_support
fold_algebra = determinant_1_rfc
root_distribution = independent_uniform_nonzero
tau = 2
delta >= 2
endpoint_excess_logq is numeric or -inf
```

Rows failing these conditions may be useful diagnostics, but they must not be
fed into the certificate recurrence as endpoint theorem evidence.
