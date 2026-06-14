# RFC Top Boundary Base-Seal Decision

Scope: original non-systematic RFC, `c=8`, target `k=2048,e=71,q=2^128`.

Status: architecture decision after the top-row carry/kernel audit.

## Summary

The current block-ledger route should not spend a reusable local credit on the transition:

```text
top_tau1_to_2_15: (1,34) -> (2,15).
```

The audit in `rfc_tau1_carry_kappa_audit.py` shows:

```text
charged_postroot_qdim = -1
kernel_dim = 0
kernel_lift_qdim = 0
```

so neither:

```text
tau1_full_line_carry
kernel_fiber_cover
```

is legal on this immediate edge.

The useful conclusion is not that the whole block grammar failed. It is that the top boundary must
be removed from the reusable local-block grammar and handled by a finite base-seal theorem.

## Boundary-Seal Probe

The potential probe now supports:

```text
--skip-transition top_tau1_to_2_15
```

This should be read as:

```text
the top boundary is handled by an external finite theorem, not by a reusable local credit.
```

Command:

```text
python -B scripts/rfc_distance_analysis/rfc_block_potential_probe.py \
  --zero-grid 0.2:2:0.05 \
  --credit-profile audited-top-kernel \
  --skip-transition top_tau1_to_2_15 \
  --top 1 --show-transitions
```

Output:

```text
support3_stratified_to_3_6, adjusted_local_qdim 0.13994737, margin 0.00000000
support2_diamond_to_3_2_ge_1_4, adjusted_local_qdim 0.08410056, margin 1.85584681
flag_tau1_tau0_to_base_flag, adjusted_local_qdim -1.93041492, margin 3.07036229
```

The best level weight is:

```text
level_weight = 1.93994737 qdims
```

So once the top boundary is externally sealed, the reusable lower grammar has the expected
support-three bottleneck, while support-two and the base flag row have slack.

## Required Boundary Theorem

The external theorem should be the finite depth-5 base seal:

```text
B_5(1,34) <= 2^-80.
```

This is already the target of:

```text
docs/rfc_distance_analysis/rfc_depth5_base_seal_candidate.md
docs/rfc_distance_analysis/rfc_depth5_finite_flag_recurrence_target.md
```

The old optimistic rank-pattern calibration gives:

```text
log2 B_5(1,34) = -115.10435419
```

but that scalar recurrence is not theorem-safe by itself. The theorem-grade route still needs the
finite exact-support/flag recurrence.

## Decision

Use a hybrid architecture:

```text
top boundary:
  finite base-seal theorem, row-specific/boundary-specific;

below the boundary:
  canonical diagram/block ledger with reusable support-three, support-two, tau-one, and container
  blocks.
```

This is not a full return to row-by-row proof. It is a controlled exception at the boundary where
the local block credits are provably inapplicable.

## Falsification Criteria

The hybrid route fails if either:

```text
1. the finite depth-5 base-seal recurrence cannot certify B_5(1,34) <= 2^-80; or
2. after importing the boundary theorem, the lower block grammar cannot make the support-three
   incidence row theorem-grade with finite constants.
```

If either failure happens, then the proof should pivot back toward explicit row-by-row treatment
of the remaining dominant trace.
