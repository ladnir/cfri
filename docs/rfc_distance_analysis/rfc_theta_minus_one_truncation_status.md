# RFC Theta Minus One Truncation Status

Scope: original non-systematic RFC distance recurrence.

Status: diagnostic plus proof target. This note records the current evidence for truncating
`theta_2=-1` kernel-chain states at length three.

## Question

The local first-drop row:

```text
theta_2 = -1,
a >= 5,
local_charge >= 9
```

is locally quantified. The remaining global concern is whether many such rows can stack through
kernel children:

```text
V_0 >= V_1 >= V_2 >= ...
```

without being seen by the two-layer flag recurrence.

The intended proof architecture is:

```text
outer branch:
  controlled by zero-budget burn;

kernel branch:
  promote to nested flags.
```

The truncation question is whether the certificate needs arbitrary flag length, or whether a
three-layer state is enough.

## Diagnostic Chain Search

The checkpoint script now has:

```text
--report-theta-chains -1
```

This interprets the best-transition choices as a directed graph. Each state points to its outer
child and, when nonzero, its kernel child. The report gives the longest consecutive chain of
best-transition tau-two rows with `theta=-1`.

Depth 6, pruned to the final distance line:

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

Output summary:

```text
crossing_z=342 crossing_excess=278
theta_chain_len,start_level,span,z,log2_state,path
1,4,4,37,2736.49785592,
1,4,2,69,-608.34578355,
1,5,2,197,-2512.28444012,
```

Depth 7, same pruned path:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py \
  --depth 7 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2-layer-incidence \
  --print-window 1 \
  --max-visible-tau 2 \
  --prune-to-final-span 1 \
  --report-theta-chains -1 \
  --report-limit 20 \
  --allow-large
```

Output summary:

```text
crossing_z=801 crossing_excess=673
theta_chain_len,start_level,span,z,log2_state,path
1,5,4,103,6495.70492252,
1,5,4,133,4516.86221357,
1,4,4,37,2736.49785592,
1,5,3,135,2732.21193921,
1,4,2,69,-608.34578355,
1,5,2,197,-2512.28444012,
1,6,2,453,-4416.16906397,
```

Thus, in the current best-transition model through depth 7, no consecutive `theta_2=-1` chain
appears. Every best first-drop burst is followed by a non-theta transition on both available child
edges.

## Interpretation

This is not yet a theorem about all subdominant transitions. It does say that the feared chain is
not a dominant feature of the corrected checkpoint recurrence. The proof can therefore aim for a
targeted truncation lemma instead of a general arbitrary-length flag theorem.

The proposed truncation statement is:

```text
Any length-four kernel chain of theta_2=-1 rows is dominated by the length-three nested-flag
recurrence after either:

1. applying the outer-branch zero-budget burn when any edge exits through V; or
2. applying the inner-first/nested-flag count when all edges follow kernel children.
```

In case 2, each additional first-drop level contributes:

```text
local_charge >= 9,
a >= 5,
one more nested flag layer,
```

but no independent child moment. The candidate proof is to count the deepest child event first and
then choose ancestor flags by Gaussian superspaces, replacing the loose `k_child` ancestor factor
with the exact nested transition whenever needed for tightness.

## Current Proof Target

Prove the following domination lemma.

Let:

```text
V_0 >= V_1 >= V_2 >= V_3
```

be a four-layer child flag arising from three consecutive kernel-following `theta_2=-1`
first-drop rows. Then its contribution is bounded by the three-layer recurrence plus an additional
negative exponent from the third first-drop row:

```text
extra <= q^{-9} * split(a>=5) * ancestor_flag_factor.
```

The theorem needs to show that the binary split factor and ancestor flag factor are dominated by
the extra local charge and the inherited zero-budget constraints for the target depths/rate.

If this lemma is too tight globally, the fallback is still finite: implement length-four flags in
the certificate driver and rerun the same state-counting argument. The depth-7 best-transition
diagnostic suggests this fallback will not be on the dominant path.
