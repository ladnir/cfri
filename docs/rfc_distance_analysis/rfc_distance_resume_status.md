# RFC Distance Resume Status

Scope: original non-systematic RFC distance certificate work. Systematic RFC notes exist in this
folder, but the active proof push is original/non-systematic.

## Current Goal

Prove a near-MDS first-moment distance certificate for the original RFC, targeting:

```text
c = 8
k = 2048
q = 2^128
lambda = 80
target excess e = 71
fallback excess e = 72
```

The intended certificate statement is:

```text
B_d(1, k+e) <= 2^-lambda
```

which implies no nonzero codeword has `k+e` or more zeros, hence:

```text
distance >= N - (k+e) + 1.
```

Do not claim exact MDS.

## Main Proof Direction

The active route is a finite-replica / flag first moment, not fixed-set MDS and not the old scalar
span recurrence.

The recurrence tracks child flags created by singleton coordinates:

```text
L = pi(K) <= V = pi(W),
z_V = p + s - a,
z_L = p + s.
```

This is the state that keeps invisible kernel directions from passing through singleton blocks for
free.

## Important Corrections

1. Product-of-first-moments is invalid when two child events share the same child-code randomness.
   This broke the old `F_child(1,z+1)^2` split for the decomposable tau-two row.

2. The old tau-two generic/component endpoint shortcut is false as a theorem. The local tau-two
   object must use layer codimensions:

   ```text
   theta_2(A) = max_h(2h - 4 - gamma_h(A)).
   ```

3. The connected `a=5, delta=3, comp=1, g=1` row is real and has:

   ```text
   theta_2 = -1.
   ```

   It is one q-dimension heavier than the old shortcut predicted, but the local algebra is now
   quantified by a first-drop/minor argument.

## What Is Currently Closed

Local / recurrence pieces that now have usable notes:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
  Proves gamma_2 >= 1 for g=1, hence theta_2=-1 for the first concrete blocker.

docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
  Direct proof for |A|=3, delta=2, comp=1, giving theta_2=-2.

docs/rfc_distance_analysis/rfc_tau2_incidence_framing_lemma.md
  Exact-support Grassmann cap and full-cover incidence framing.

docs/rfc_distance_analysis/rfc_theta_minus_one_isolation_lemma.md
  Outer-branch theta_2=-1 chains burn zero budget.

docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
  Kernel-branch theta_2=-1 chains must use nested flags, not independent moments.

docs/rfc_distance_analysis/rfc_theta_minus_one_truncation_status.md
  Depth-6/7 diagnostics show max consecutive best-transition theta_2=-1 chain length 1.
```

The external Fable audit was useful and found the product-of-first-moments bug. Its record is:

```text
docs/rfc_distance_analysis/rfc_fable_audit_2026_06_10.md
```

## Current Blockers

The live blockers are now narrow:

```text
1. Formal joint marked-line/frame recurrence for the decomposable |A|=2,delta=2,comp=2 row.
2. Kernel-branch nested-flag truncation:
   prove length three is enough, or prove length-four theta_2=-1 chains are dominated.
3. Higher-drop tau-two layers beyond the g=1 first-drop case.
4. Finite constants: nonzero determinant-1 roots, exact-support inversion, split counts, and
   log-sum/state-count overhead.
```

The best current next proof step is item 2:

```text
Prove a length-four domination lemma for kernel-following theta_2=-1 chains.
```

## Diagnostic Script

Main checkpoint:

```text
scripts/rfc_distance_analysis/rfc_flag_span_moment.py
```

Useful commands:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py \
  --depth 6 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2-layer-incidence \
  --print-window 1 \
  --max-visible-tau 2 \
  --prune-to-final-span 1 \
  --report-local-theta -1 \
  --report-theta-chains -1 \
  --report-limit 20
```

Depth 6 result:

```text
crossing_z=342 crossing_excess=278
max consecutive theta_2=-1 chain length = 1
```

Depth 7 result:

```text
crossing_z=801 crossing_excess=673
max consecutive theta_2=-1 chain length = 1
```

These are diagnostics over the corrected best-transition checkpoint, not theorem statements.

## Where To Look First

Start with:

```text
docs/rfc_distance_analysis/rfc_distance_manager_board.md
docs/rfc_distance_analysis/rfc_distance_certificate_theorem.md
docs/rfc_distance_analysis/rfc_flag_recurrence_proof_obligations.md
docs/rfc_distance_analysis/rfc_theta_minus_one_truncation_status.md
```

Then use:

```text
scripts/rfc_distance_analysis/README.md
docs/rfc_distance_analysis/README.md
```

for the hierarchy of scripts and notes.
