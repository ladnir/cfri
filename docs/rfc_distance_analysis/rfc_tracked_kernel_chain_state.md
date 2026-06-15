# RFC Tracked Kernel Chain State

Scope: original non-systematic RFC only. This note resolves the `t=3,tau=1`
paired-spine issue where the full recursive kernel has dimension 2 but the
diagnostic wants to track one hidden line.

## Blocker

At a paired-spine parent state, let

```text
pi : W -> visible quotient
dim W = t
dim pi(W) = tau
K = ker(pi|_W)
full_kernel_dim = dim K = t - tau
```

For the blocker case:

```text
t = 3
tau = 1
full_kernel_dim = 2
tracked_kappa = 1
```

So a tracked hidden line `ell` is not the full recursive kernel. It is a line
inside the 2-dimensional kernel `K`. Counting every `ell <= K` as a separate bad
event would double-count unless the recurrence state explicitly needs that line
for a future constraint.

## Coherent State Object

Use a two-mode chain state:

```text
StateFull =
  exact_support_profile,
  level,
  W_key,
  visible_key,
  K_key

StateTrackedLine =
  exact_support_profile,
  level,
  W_key,
  visible_key,
  K_key,
  ell_key,
  ell_role
```

`StateFull` is the current two-layer recurrence state. It remembers the parent
subspace `W`, its visible quotient, and the full recursive kernel `K`.

`StateTrackedLine` is a refined state. It remembers one active line
`ell <= K`, but only when `ell` is used by a later paired-spine constraint. The
line must have a declared role, for example:

```text
ell_role = child_active_kernel
ell_role = next_level_intersection_witness
ell_role = forced_zero_subline
```

A line with no declared downstream role is a duplicate certificate, not an
event.

## Event Keys

For `StateFull`, the canonical event key is:

```text
full_chain_key =
  hash(
    exact_support_profile,
    level,
    W_canonical_basis,
    visible_canonical_basis,
    K_canonical_basis
  )
```

For `StateTrackedLine`, the canonical event key is:

```text
tracked_chain_key =
  hash(
    full_chain_key,
    ell_canonical_basis,
    ell_role,
    downstream_constraint_key
  )
```

The `downstream_constraint_key` is mandatory. It identifies the next-level
constraint that actually consumes `ell`. Without it, different line choices in
the same `K` are alternative certificates for the same `StateFull` event.

Duplicate certificates include:

- different bases for the same `W`, `K`, visible line, or `ell`;
- different generation paths producing the same `full_chain_key`;
- selected lines `ell <= K` with no downstream consumer;
- multiple `ell` values that satisfy the same downstream constraint but collapse
  to the same `tracked_chain_key`;
- root representatives of the same projective line.

## Lift Accounting

The existing two-layer Lift bound must use the full kernel dimension:

```text
LiftFull(t,tau,r0,r1)
  <= Gamma_q^O(1)
     q^{full_kernel_dim*(2*r0-full_kernel_dim)
       +tau*(2*r1-t)}
```

where:

```text
full_kernel_dim = t - tau
```

For `t=3,tau=1`, this is a `full_kernel_dim=2` lift. A diagnostic row that tracks
only `tracked_kappa=1` cannot be charged as if the full kernel had dimension 1.

If the recurrence explicitly enters `StateTrackedLine`, it must pay the
projective line choice inside `K`:

```text
Line(K,1) <= Gamma_q q^{tracked_kappa*(full_kernel_dim-tracked_kappa)}
```

For the blocker:

```text
Line(K,1) <= Gamma_q q
```

Thus the tracked-line lift accounting is:

```text
LiftTrackedLine =
  LiftFull(t,tau,r0,r1)
  + line_choice_logq
  - downstream_forcing_saving_logq
```

The line is useful only if the downstream constraint gives back at least the line
choice cost, or if keeping the line prevents a larger overcount later.

## Does This Require A Three-Layer Flag State?

Yes, if the line persists across levels as an active object.

The current two-layer state has the form:

```text
K <= W
```

Tracking one hidden line inside a larger kernel changes the state to:

```text
ell <= K <= W
```

This is a three-layer flag state. It is not optional if implementation wants to
count `ell` as an event rather than as a duplicate certificate.

However, the recurrence does not need to globally switch to three-layer flags
unless the paired-spine diagnostic finds a real positive excess after
de-duplication. The default interpretation remains:

```text
ell choices inside K are duplicate certificates for StateFull
```

until a downstream constraint makes `ell` persistent and canonical.

## Meaningful Implementation Counts

Implementation can count tracked hidden lines meaningfully only under one of
these modes:

```text
kernel_mode = full_kernel
```

Count distinct `StateFull` objects. Lines inside `K` are not events. Report
selected-line multiplicity only as duplicate-certificate data.

```text
kernel_mode = tracked_line_with_consumer
```

Count distinct `StateTrackedLine` objects. Each row must include
`downstream_constraint_key` and must charge the line-choice exponent.

Rows with:

```text
kernel_mode = tracked_line
downstream_constraint_key = null
```

are not admissible recurrence evidence. They measure certificate multiplicity
only.

## Required Columns

A tracked-kernel-chain diagnostic should report:

```text
t
tau
full_kernel_dim
tracked_kappa
kernel_mode
level
exact_support_profile_key
W_key
visible_key
K_key
ell_key
ell_role
downstream_constraint_key
full_chain_key
tracked_chain_key
full_event_count
tracked_event_count
duplicate_certificate_count
line_choice_logq
lift_full_logq
downstream_forcing_saving_logq
lift_tracked_line_logq
observed_chain_logq
chain_excess_full_logq
chain_excess_tracked_logq
classification
proof_effect
```

For `kernel_mode=full_kernel`, `ell_key`, `ell_role`, and
`downstream_constraint_key` may be empty, but selected-line multiplicities should
still be summarized in `duplicate_certificate_count`.

For `kernel_mode=tracked_line_with_consumer`, those fields are mandatory.

## Acceptance Criteria

The `t=3` paired-spine blocker is defused for the two-layer recurrence if:

```text
chain_excess_full_logq <= 0
```

after canonical de-duplication, while any positive selected-line multiplicity is
classified as duplicate certificate data.

The blocker requires a three-layer recurrence if:

- distinct `ell <= K` lines have distinct downstream consumers;
- those consumers survive canonical de-duplication;
- the same line identity must be propagated across the paired lift; and
- `chain_excess_full_logq > 0` but `chain_excess_tracked_logq <= 0` after paying
  the line-choice cost.

The blocker threatens the `e=71` certificate and should trigger `e=72`
evaluation if:

- `chain_excess_tracked_logq > 0` after paying the full Lift and line-choice
  costs;
- the excess persists over large/generic fields;
- it is not explained by exact-support containment, duplicate certificates, or
  small-field degeneracy; and
- the global first-moment recurrence has no remaining slack to absorb it.

## Practical Interpretation

For the current proof lane, the safe default is:

```text
t=3,tau=1,tracked_kappa=1 means:
  full recursive state has K of dimension 2;
  the selected line ell is duplicate-certificate data unless it has a named
  downstream consumer;
  using ell as a counted event requires the three-layer flag ell <= K <= W.
```

This lets implementation test the proposed hidden-line cascade without
accidentally weakening the Lift accounting or double-counting line choices as
independent bad events.
