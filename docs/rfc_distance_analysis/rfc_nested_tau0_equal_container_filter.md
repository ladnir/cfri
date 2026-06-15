# RFC Nested Tau-Zero Equal-Container Filter

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: local exact-support lemma and diagnostic rule. This is not a complete distance
certificate.

## Purpose

While trying to define the tau-one full-line transition map for the dominant row, the first target
row exposed a simpler exact-support obstruction. The row was:

```text
state: (4,7)>=(2,8)

outer row:
  tau = 1
  active support size = 1
  child projection dim = 4

inner row:
  tau = 0
  child projection dim = 4
```

The current two-layer state is nested: the lower parent layer is contained in the upper parent
layer, and the lower zero witness can be chosen to contain the upper zero witness. Therefore every
upper active singleton coordinate is also a lower zero coordinate. If the lower tau-zero child
projection has the same dimension as the upper child projection, containment forces the two child
containers to be equal. The upper container is then already zero on its claimed active support, so
the upper tau-positive exact-support row is impossible as stated.

## Lemma

Let:

```text
W_1 <= W_0
```

be two parent layers in one exact nested flag transition. Let the upper layer `W_0` have a
tau-positive singleton profile:

```text
A_0 nonempty,
tau_0 > 0,
K_0 = ker(visible singleton map on A_0),
V_0 = pi(W_0),
L_0 = pi(K_0).
```

Let the lower layer `W_1` have tau zero on a zero witness that contains the upper witness:

```text
tau_1 = 0,
K_1 = W_1,
V_1 = pi(W_1).
```

Assume the child projections satisfy:

```text
V_1 <= V_0
dim V_1 = dim V_0.
```

Then the upper active support `A_0` is not exact. Equivalently, this transition profile should be
rerouted to a smaller active support profile and excluded from the tau-positive exact-support sum.

## Proof

Because `W_1 <= W_0`, every coordinate where `W_0` vanishes is also a coordinate where `W_1`
vanishes. In the exact-support recurrence we may choose the lower witness set to contain the upper
witness set.

Take any active singleton coordinate:

```text
j in A_0.
```

The coordinate `j` is part of the upper zero witness, so it is part of the lower zero witness. Since
the lower row has `tau_1 = 0`, its child projection vanishes on every child coordinate under its
paired or singleton zero requests. Hence:

```text
V_1 is zero at j.
```

But `V_1 <= V_0` and `dim V_1 = dim V_0`, so:

```text
V_1 = V_0.
```

Therefore `V_0` is zero at `j`. This holds for every `j in A_0`, so the upper quotient datum has
zero projection on its claimed active support. That contradicts exact support on `A_0`.

The event is still counted safely after canonical exact-support rerouting: delete the collapsed
active coordinates from `A_0`, recompute `tau_0`, and charge the resulting smaller profile. The
tau-positive profile above should not also be counted.

## Diagnostic Rule

The script flag:

```text
--nested-tau0-equal-container-filter
```

excludes pair rows satisfying:

```text
outer tau > 0
outer active support size > 0
inner tau = 0
inner child projection dimension = outer child projection dimension
```

inside the pair-enumerated two-layer table. This is a diagnostic shadow of the lemma above. It is
proof-shaped only for a recurrence that carries nested zero witnesses, as specified in
`rfc_exact_support_quotient_state.md`.

## First Target Result

On the strong target table:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --stop-level 3 \
  --proof-shaped \
  --nested-quotient-mode inner-in-outer \
  --nested-subspace-mode inner-in-outer \
  --consumed-kernel-mode inner-kernel-contained \
  --support2-diamond-mode child-only \
  --support2-line-quotient-filter \
  --exact-filtered-empty \
  --nested-tau0-equal-container-filter \
  --term-limit 300 \
  --report-flag-state 4,7,2,8 \
  --trace-table-state 3,4,7,2,8 \
  --trace-table-top 8 \
  --last-level-report-only
```

the level-3 state improves from the previous strong-mode value:

```text
(4,7)>=(2,8): 423.08002115 bits
```

to:

```text
(4,7)>=(2,8): 27.05765334 bits.
```

The excluded old top row was the apparent tau-one full-line carry target:

```text
outer choice = 3:1:1:1:4:3:3:1:1:1:0:-1:-1:-1:13
inner choice = 3:2:0:0:4:4:5:0:0:0:0:-1:-1:0:0
```

It is better understood as a nested tau-zero equal-container active-support collapse, not as a row
whose descendant quotient line should be identified with a parent-carried full line.

## Full Checkpoint

With the same filter in the demanded depth-5 run without closure:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --proof-shaped \
  --nested-quotient-mode inner-in-outer \
  --nested-subspace-mode inner-in-outer \
  --consumed-kernel-mode inner-kernel-contained \
  --support2-diamond-mode child-only \
  --support2-line-quotient-filter \
  --exact-filtered-empty \
  --nested-tau0-equal-container-filter \
  --term-limit 300 \
  --demand-next-level \
  --report-final-z 34 \
  --trace-state-terms 4,2,15 \
  --trace-state-top 6
```

the current checkout reports:

```text
final_span_1_crossing_z,133
final_span_1_z_report,34,1080.36409460
```

This is a substantial diagnostic improvement, but it is still far from the target `-80` bits. The
next dominant competitors at state `(2,15)` are support-two quotient-frame and tau-two
layer-codimension rows, so the certificate is not closed by this filter alone.
