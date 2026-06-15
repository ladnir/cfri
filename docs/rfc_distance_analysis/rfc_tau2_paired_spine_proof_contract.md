# RFC Tau-2 Paired-Spine Proof Contract

Scope: original non-systematic RFC only. This note specifies what the
chain-length-2 `tau=2` paired-spine gate must count before it can be used as
evidence for the flag recurrence or the `e=71` distance certificate.

## Gate Shape

The gate has two linked levels:

```text
child endpoint/local tau=2 state
paired-spine parent lift
```

The purpose is to test whether a local visible two-plane creates an uncharged
paired-spine cascade. The endpoint theorem and the paired Lift are separate
charges; a diagnostic must report both.

## State Variables

Each row should fix an exact support/profile and report:

```text
level_child
level_parent
chain_length = 2
tau = 2
t_parent
full_kernel_dim = t_parent - tau
tracked_kernel_dim
kernel_mode
A_key
component_partition_key
delta
comp
g
V_child_key
W_parent_key
K_parent_key
active_copy_profile_key
chain_key
```

Definitions:

- `A_key`: canonical exact local support set for the endpoint event.
- `delta`: exact-support subcode drop for `A`; equivalently the quantity used by
  the local endpoint theorem to measure how many endpoint constraints are forced
  by exact support. Rows must include enough raw ranks to recompute it.
- `comp`: number of connected components in the local incidence/matroid object
  after restricting to `A`.
- `g`: generic visible-span parameter for `A`; rows must include enough raw
  rank/span data to recompute it.
- `V_child_key`: canonical visible two-plane at the child endpoint.
- `W_parent_key`: canonical parent subspace after one paired lift.
- `K_parent_key`: canonical full kernel of the parent projection.
- `active_copy_profile_key`: canonical profile of which independent fold copies
  are active. It must distinguish real copy choices from labels that are only
  duplicate certificates.

For `tau=2`, states with `delta < 2` are impossible endpoint states and must be
classified as rejected, not counted with a favorable exponent.

## Corrected Tau-2 Endpoint Theorem Test

For each exact support `A`, the corrected endpoint exponent is:

```text
E2(A) = max(2*g(A) - 4,
            comp(A) + 2*delta(A) - 4 - |A|)
```

The diagnostic should test the local endpoint theorem in this form:

```text
observed_endpoint_logq(A) <= E2(A)
```

after:

- exact-support inversion;
- canonical de-duplication of bases and root-line representatives;
- rejection of `tau=2, delta<2`;
- collapse of marked-core labels to unmarked support sets when the marks do not
  change the endpoint event;
- separation of event counts from certificate multiplicities.

Rows should report:

```text
support_size = |A|
delta
comp
g
endpoint_exponent_2g_minus_4 = 2*g - 4
endpoint_exponent_component = comp + 2*delta - 4 - |A|
endpoint_bound_logq = max(endpoint_exponent_2g_minus_4,
                          endpoint_exponent_component)
observed_endpoint_logq
endpoint_excess_logq = observed_endpoint_logq - endpoint_bound_logq
```

A positive `endpoint_excess_logq` is a local-theorem failure unless it is fully
explained by duplicate certificates, contained-support overcount, or small-field
degeneracy.

## Event vs Duplicate Certificate

Count as events:

- a canonical exact support `A`;
- a canonical local visible two-plane `V_child`;
- a canonical active-copy profile when copy identity changes the algebraic
  event;
- a canonical parent subspace `W_parent`;
- the full parent kernel `K_parent`;
- a tracked subkernel only when the recurrence state explicitly remembers it and
  a downstream constraint consumes it.

Count as duplicate certificates:

- different bases for the same two-plane, parent subspace, or kernel;
- ordering of endpoint generators inside the same two-plane;
- marked-core labels that collapse to the same exact support event;
- contained-support certificates already charged by exact-support inversion;
- repeated construction paths producing the same `chain_key`;
- active-copy labels that do not change the canonical span/intersection object;
- tracked kernel subspaces that are not part of the recurrence state.

The chain event key should be:

```text
chain_key =
  hash(
    A_key,
    component_partition_key,
    V_child_key,
    active_copy_profile_key,
    W_parent_key,
    K_parent_key,
    optional_tracked_kernel_key
  )
```

The optional tracked kernel key is admissible only when `kernel_mode` says the
recurrence is tracking that object.

## Paired Lift Accounting With Tau=2

The endpoint theorem charges the local visible two-plane. The paired Lift charges
the number of parent states above that two-plane.

For a full-kernel parent lift:

```text
Lift(t_parent, tau=2, r0, r1)
  <= Gamma_q^O(1)
     q^{full_kernel_dim*(2*r0-full_kernel_dim)
       +2*(2*r1-t_parent)}
```

where:

```text
full_kernel_dim = t_parent - 2
```

The chain-level allowance is:

```text
recurrence_allowed_logq =
    endpoint_bound_logq
  + lift_bound_logq
  + generic_intersection_bound_logq
  + tracked_kernel_choice_logq
  - forced_downstream_saving_logq
```

Use `tracked_kernel_choice_logq=0` for the current two-layer recurrence. If a
tracked subkernel is counted as a real event, the row must explicitly enter a
longer flag state and pay the relevant Grassmannian line/plane choice inside the
full kernel.

The observed chain excess is:

```text
chain_excess_logq =
    observed_chain_logq
  - recurrence_allowed_logq
```

Support and split factors should be reported separately in bits. They are part
of the global first moment, not part of the local q-dimensional endpoint/Lift
excess.

## Required Output Columns

The implementation-facing diagnostic should emit at least:

```text
field_prime
seed
level_child
level_parent
chain_length
tau
t_parent
full_kernel_dim
tracked_kernel_dim
kernel_mode
A_key
support_size
component_partition_key
delta
delta_rank_before
delta_rank_after
comp
g
g_rank_data
active_copy_profile_key
V_child_key
W_parent_key
K_parent_key
tracked_kernel_key
chain_key
canonical_endpoint_event_count
canonical_chain_event_count
duplicate_certificate_count
endpoint_exponent_2g_minus_4
endpoint_exponent_component
endpoint_bound_logq
observed_endpoint_logq
endpoint_excess_logq
lift_bound_logq
generic_intersection_bound_logq
tracked_kernel_choice_logq
forced_downstream_saving_logq
recurrence_allowed_logq
observed_chain_logq
chain_excess_logq
classification
proof_effect
notes
```

Rows with `delta < 2` should have:

```text
classification = rejected_impossible_tau2_endpoint
proof_effect = no_event
```

## Acceptance Criteria

The tau-2 paired-spine gate supports the current proof if every accepted
fixed-profile row satisfies:

```text
endpoint_excess_logq <= 0
chain_excess_logq <= 0
```

after exact-support inversion and canonical de-duplication, with no hidden
skipped/capped groups.

The output supports the corrected endpoint theorem only if the reported
`delta`, `comp`, and `g` are reproducible from raw rank/component data. A row
that gives only the final exponent is not sufficient for proof use.

## Escalation Criteria

A result forces a higher-tau or local-theorem fix if:

- `endpoint_excess_logq > 0` persists after de-duplication;
- the excess survives large/generic-field checks;
- `delta`, `comp`, and `g` are computed correctly from raw data; and
- the excess is not explained by exact-support containment or duplicate
  certificates.

In that case the corrected endpoint theorem is still missing structure. The
likely repairs are a refined local invariant, an active-copy intersection term,
or a higher-layer local flag theorem.

A result forces a longer recurrence state if:

- endpoint excess is nonpositive, but `chain_excess_logq > 0`;
- the positive chain excess comes from a persistent subkernel or active-copy
  intersection object not remembered by the two-layer state; and
- paying the explicit tracked-object choice closes the count.

A result should trigger `e=72` evaluation if:

- `chain_excess_logq > 0` remains after all local theorem and state refinements;
- the excess is generic rather than small-field behavior;
- the global first-moment slack for `e=71` cannot absorb the contribution.

## Proof Use

A clean tau-2 paired-spine gate discharges one specific local-to-recursive
interaction: visible two-plane endpoint events do not create uncharged
chain-length-2 paired-spine families. It does not by itself prove the global
distance certificate. The global proof still has to sum exact support/split
profiles and combine this gate with the tau-1, complete-stride, and tracked
kernel-chain obligations.
