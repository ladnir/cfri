# RFC Residual Trace Classification

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: diagnostic note. This classifies the current proof residual; it is not a certificate
theorem.

## Purpose

The sparse pair-table recurrence now has enough moving parts that a numeric trace alone is easy to
misread. The important distinction is:

```text
1. baseline scalar fallback rows;
2. pair-table rows that expose a support-two quotient diamond;
3. stronger-mode rows where the remaining obstruction is recursive tau-one full-line carry.
```

These are different proof obligations. The recurrence should not treat any of them as generic
"more pair-table plumbing".

## Baseline Sparse Trace

Current checkout command:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --proof-shaped \
  --term-limit 300 \
  --demand-next-level \
  --report-final-z 34 \
  --trace-z 34 \
  --trace-follow coarse-bound
```

Current output:

```text
final_span_1_crossing_z,133
final_span_1_z_report,34,1478.66370843
```

The bound-following path is:

```text
level 5: tau=1, child (2,15), local charge exceeds the tau-one quotient count;
level 4: tau=1, child (4,7)>=(2,8), full-line carry row;
level 3: tau=2, support-two quotient-diamond scalar row;
level 2: tau=2, connected/support-three child row;
level 1: base tau-one row.
```

So the baseline residual is already structured. The level-4 row asks for recursive tau-one
full-line carrying, while the level-3 row asks for a represented support-two quotient-frame or
diamond theorem.

## Level-3 Stress Table

For the table state:

```text
(4,7)>=(2,8)
```

current checkout command:

```text
python -B scripts/rfc_distance_analysis/rfc_pair_flag_table_recurrence.py \
  --depth 5 \
  --stop-level 3 \
  --proof-shaped \
  --term-limit 300 \
  --report-flag-state 4,7,2,8 \
  --trace-table-state 3,4,7,2,8 \
  --trace-table-top 4 \
  --last-level-report-only
```

The pair-enumerated sum is worse than the scalar fallback:

```text
baseline/table value: 549.21247145
pair_sum:             943.43723477
```

The new trace labels classify the top pair rows as:

```text
outer_family:     support2-quotient-diamond
outer_obligation: joint-diamond-child-diagram

inner_family:     tau1-quotient-line
inner_obligation: count-quotient-line-incidence
```

This is the key diagnostic conclusion: the missing proof object is not a larger independent pair
sum. The pair sum is the wrong shape until the support-two quotient diamond is converted into a
joint child diagram and the lower quotient-line incidence is counted conditionally inside that
diagram.

## Relation To Stronger Diagnostics

The existing stronger diagnostics already follow this sequence:

```text
1. nested quotient/subspace/consumed-kernel counting improves the old stress table;
2. support2-diamond child-only routing improves the support-two row;
3. line-quotient impossibility and exact-empty handling expose strong-bottom rows;
4. the remaining high-lift row becomes tau-one full-line carry.
```

Therefore the next proof work should split cleanly:

```text
Support-two branch:
  Prove and propagate the represented support-two quotient-frame/diamond child-diagram theorem.

Tau-one branch:
  Define the actual transition map phi : E'_A -> E_A for the carried full line and compute
  kappa_phi on the dominant row.
```

Update: the first row below is now resolved by a different exact-support mechanism. It has a lower
tau-zero sibling whose projected child container has the same dimension as the upper tau-one
container. Nested witnesses therefore force the upper active support to be zero in that container.
See `rfc_nested_tau0_equal_container_filter.md`.

The old row was:

```text
state: (4,7)>=(2,8)
dominant descendant tau-one row:
  child flag (4,5)>=(3,4)
  independent charged post-root dimension = 3
target:
  replace that dimension by kappa_phi, not by zero unless kappa_phi=0 is proved.
```

After enabling `--nested-tau0-equal-container-filter`, the strong table value for `(4,7)>=(2,8)`
moves to `27.05765334` bits. Later tau-one rows may still need a full-line transition map, but this
specific top row should be rerouted before a `kappa_phi` calculation.

## Diagnostic Code Update

`rfc_pair_flag_table_recurrence.py` now labels traced choices with:

```text
choice_family
proof_obligation
```

and labels pair-table rows with:

```text
outer_family, inner_family
outer_obligation, inner_obligation
```

These labels do not change any recurrence value. They are an audit aid for making sure future
numeric improvements correspond to theorem-grade proof obligations.
