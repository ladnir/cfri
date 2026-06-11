# RFC Paired-Spine Cascade Proof Contract

Scope: original non-systematic RFC only. This note specifies what the paired-spine
cascade diagnostic must count for its output to be admissible evidence for the
two-layer flag recurrence.

## Target Shape

The diagnostic is seeded by complete-stride flags at depth 4 and then applies
one paired lift along the all-paired spine. The requested stress shape is:

```text
seed depth: 4
extra paired lifts: 1
chain length: 2
t in {2,3}
tau = 1
tracked kappa = 1
```

The diagnostic must distinguish two kernel dimensions:

```text
full_kernel_dim = dim ker(W -> R) = t - tau
tracked_kappa   = dimension of the kernel object explicitly marked by the diagnostic
```

Thus for `t=2,tau=1`, `tracked_kappa=1` is the full kernel. For `t=3,tau=1`,
the full kernel has dimension 2, so `tracked_kappa=1` is only a selected kernel
line inside the full kernel unless the recurrence is extended to remember that
line. This distinction is mandatory for interpreting any extra dimension.

## Chain Object

A canonical chain is the de-duplicated algebraic object:

```text
seed exact support/profile
seed complete-stride Omega
seed flag L0 <= V0
paired-lift parent W
parent visible line or flag data
parent full kernel K = ker(W -> R)
optional tracked kernel line ell <= K
```

For chain length 2, the chain key must bind the seed flag and the first lifted
parent flag/kernel object. Repeated construction paths producing the same chain
key are duplicate certificates, not extra events.

## Events vs Duplicate Certificates

Count as events:

- An exact zero-request support/profile at the seed level.
- An unmarked complete-stride seed support set, after quotienting by stride-label
  symmetries that do not change the support.
- A canonical seed flag `L0 <= V0`.
- A canonical lifted parent subspace `W`.
- The full parent kernel `K = ker(W -> R)` when the recurrence state remembers
  full kernels.
- A selected kernel line `ell <= K` only in rows explicitly marked
  `kernel_mode=tracked_line`.
- Independent copy labels when the diagnostic is intentionally testing
  multi-copy intersections.

Count as duplicate certificates:

- Different bases for the same `L`, `V`, `W`, `K`, or `ell`.
- Different representatives of the same root line.
- Orderings of the two complete-stride classes in the same `Omega`.
- Paired-lift construction labels that produce the same canonical parent `W`.
- Contained-support or non-stride `Omega` labels that collapse to the same exact
  seed flag.
- Same-copy sanity comparisons.
- Selected lines inside a higher-dimensional kernel when the claimed recurrence
  row only tracks the full kernel.

The implementation should report both the event count and the duplicate
certificate count. A large duplicate count is not a proof obstruction by itself.

## `chain_excess_logq`

The diagnostic should compute `chain_excess_logq` after canonical de-duplication
and under a fixed exact support/profile. Combinatorial split and support counts
must be reported separately in bits; they are not part of the local q-dimension
excess.

Use:

```text
observed_chain_logq =
    log_q(number of distinct canonical chains for the fixed profile)

recurrence_allowed_logq =
    seed_flag_bound_logq
  + child_flag_bound_logq
  + sum_j lift_bound_logq(j)
  + generic_intersection_bound_logq

chain_excess_logq =
    observed_chain_logq - recurrence_allowed_logq
```

If the seed flag is fixed by the row, `seed_flag_bound_logq=0`. If the row ranges
over all seed flags, use the exact complete-stride flag bound from the two-layer
flag recurrence.

For a full-kernel lift, the local Lift exponent must be computed with the full
kernel dimension:

```text
lift_bound_logq =
    full_kernel_dim * (2*r0 - full_kernel_dim)
  + tau             * (2*r1 - t)
```

The complete-stride base lift has:

```text
t=2, tau=1, full_kernel_dim=1, r0=1, r1=2
lift_bound_logq = 1*(2*1-1) + 1*(2*2-2) = 3
```

For `t=3,tau=1`, a row with `tracked_kappa=1` must also report the full-kernel
Lift accounting with `full_kernel_dim=2`. A selected-line excess cannot certify
the existing two-layer recurrence unless it is shown to be a duplicate
certificate or unless the recurrence state is extended to a longer flag.

## Required Output Columns

The diagnostic output should include at least:

```text
field_prime
seed
depth_seed
depth_parent
chain_length
t
tau
full_kernel_dim
tracked_kappa
kernel_mode
r0
r1
seed_support_key
seed_omega_key
seed_L_key
seed_V_key
seed_flag_key
parent_W_key
parent_K_key
tracked_kernel_line_key
parent_visible_key
chain_key
canonical_chain_count
duplicate_certificate_count
selected_kernel_line_count
split_log2
support_log2
observed_chain_logq
seed_flag_bound_logq
child_flag_bound_logq
lift_bound_logq
generic_intersection_bound_logq
recurrence_allowed_logq
chain_excess_logq
classification
proof_effect
notes
```

If the diagnostic emits one row per lift step, add `lift_step_index` and make the
final row or summary row aggregate the chain-level accounting.

## Acceptance Criteria

The paired-spine cascade supports the current two-layer flag recurrence if every
completed fixed-profile row satisfies:

```text
chain_excess_logq <= 0
```

up to explicitly reported finite-field rounding, and all of the following hold:

- Exact-support and canonical-chain de-duplication are applied before counting.
- `kernel_mode=full_kernel` is used for any claim about the current recurrence.
- Rows with `kernel_mode=tracked_line` are interpreted only as evidence about a
  possible longer-flag refinement.
- No capped, skipped, or failed groups are hidden inside an accepted summary.
- Large-prime or symbolic-generic checks rule out small-field-only degeneracy for
  any observed positive excess.

## Escalation Criteria

A result forces a longer-flag recurrence state if:

- `t=3,tau=1` only closes after selecting a kernel line inside a 2-dimensional
  full kernel;
- selected kernel lines remain distinct canonical events after de-duplication;
- the selected line is reusable across the paired lift; and
- `chain_excess_logq >= 1` relative to the full-kernel Lift accounting.

This would mean the proof must remember a longer flag such as:

```text
ell <= K <= W
```

or an equivalent nested-kernel state. The diagnostic should not report this as a
failure of distance; it is a failure of the current two-layer state to charge all
visible structure.

A result should trigger `e=72` evaluation if a real extra q-dimension survives
canonical de-duplication, large-field genericity checks, and the longer-flag
interpretation, and if the production-scaled contribution consumes the reserved
slack for the `e=71` certificate.

## Proof Use

Clean paired-spine output does not prove the global certificate by itself. It
discharges one local interaction needed by the flag recurrence: complete-stride
flags do not create an uncharged nested-kernel cascade under the first paired
lift. The remaining recurrence proof must still sum exact split/support profiles
and verify the global first-moment margin at `c=8,k=2048,q=2^128,e=71`.

