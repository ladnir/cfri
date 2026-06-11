# RFC Theta Minus One Truncation Status

Scope: original non-systematic RFC distance recurrence.

Status: diagnostic plus proof target. This note records the current evidence for truncating
`theta_2=-1` kernel-chain states at length three.

## Question

The local first-drop layer:

```text
first-drop contribution = -1,
a >= 5,
local_charge >= 9
```

is locally quantified. The full-row statement `theta_2 <= -1` remains conditional on the
full-kernel/component endpoint for the same support row. The remaining global concern is whether
many first-drop layers can stack through kernel children:

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

## Hardest Case Reduction

For one first-drop row:

```text
z = 2p + s,
z_V = p + s - a,
z_L = p + s,
z_L - z_V = a >= 5.
```

If `s>a`, the outer child receives `s-a` explicit zero residue, so any chain that exits through
the outer edge is charged by zero-budget burn. The hard pure-kernel case is therefore:

```text
s_i = a_i = 5
```

for every consecutive first-drop row. Along a kernel-following path:

```text
z_{i+1} = (z_i + 5)/2,
z_m     = z_0/2^m + 5(1 - 2^-m).
```

Equivalently:

```text
z_0 = 2^m z_m - 5(2^m - 1).
```

So the scalar deepest event alone is not enough: for fixed `z_m`, a pure kernel chain asks for
fewer top-level zeros than all-paired compression. The certificate must keep the inserted nested
flag layers. The exploitable structure is that every kernel step creates a same-depth flag gap:

```text
upper layer zero budget z_V,
lower layer zero budget z_L >= z_V + 5.
```

Thus the length-four domination lemma should charge the third inserted flag gap and third local
first-drop charge against the ancestor flag multiplicity, rather than discarding the intermediate
layers.

The concrete ancestor tool is now the deterministic shortened-ambient lemma in:

```text
docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
```

For fixed nested zero witnesses `B_i`, it counts ancestors inside:

```text
H(B_i) = {messages vanishing on B_i}
```

and gives:

```text
prod_i GaussianBinomial(dim H(B_i)-t_{i+1}, t_i-t_{i+1})_q.
```

If `dim H(B_i)` is larger than the MDS value, that is itself a recursive rank-defect event and
must be charged as an added flag layer rather than hidden in the ancestor count.

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
extra <= q^{-9} * split(a>=5) * shortened_ancestor_factor.
```

For fixed zero witnesses, the shortened factor is:

```text
shortened_ancestor_factor
  <= Gamma_q^3
     q^sum_i (t_i-t_{i+1})(D_i-t_i),

D_i = dim H(B_i).
```

The theorem needs to show one of the following:

```text
normal slice:
  D_i <= max(k_child-|B_i|,0)+b_i
  makes q^-9 * split(a>=5) * shortened_ancestor_factor dominated by the three-layer recurrence;

defect slice:
  D_i > max(k_child-|B_i|,0)+b_i
  is charged recursively as a canonical shortened-kernel rank event R_child(D_i,|B_i|).
```

This is now a finite state question: choose the allowed `b_i` slices and prove the resulting
normal-slice inequality for the target rates/depths, while routing defect slices back into the same
rank recurrence.

Equivalently, for the extra fourth-layer transition define:

```text
E_anc = sum_i (t_i-t_{i+1})(D_i-t_i).
```

The normal slice is safe whenever:

```text
E_anc + log_q(Split_i * C_i) <= 9 - margin.
```

If not, expose the offending `D_i=dim H(B_i)` as a new recursive rank event rather than truncating
that state.

The rank-tail charge associated with exposing one such layer is:

```text
defect_charge(D_i, |B_i|)
  = D_i(|B_i|-k_child+D_i).
```

This is recorded only as the expected codimension target; the proof-safe recurrence exposes the
canonical event `R_child(D_i,|B_i|)`.

## Normal-Slice Diagnostic

The deterministic checker:

```text
python scripts/rfc_distance_analysis/rfc_theta_chain_normal_slice.py
```

evaluates:

```text
E_anc = sum_i (t_i-t_{i+1})(D_i-t_i)
```

for candidate length-four flag shapes. Default output is saved in:

```text
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_default.csv
```

For the observed depth-6/7 theta-chain inner shapes, using the level-local child dimensions:

```text
level 4 inner shape: child_k=8,  dims=(4,3,2,1), zeros=(21,26,31)
level 5 inner shape: child_k=16, dims=(4,3,2,1), zeros=(101,106,111)
```

the normal slice with `b=0` is infeasible: the MDS shortened ambient has dimension `0`, so it
cannot contain the proposed ancestors. Sweeps with uniform `b=0..6` are saved in:

```text
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_observed_level4_sweep.csv
docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_observed_level5_sweep.csv
```

In both observed-shape sweeps:

```text
b=0,1,2,3: normal slice infeasible;
b=4:       E_anc=3, margin=6;
b=5:       E_anc=6, margin=3;
b=6:       E_anc=9, margin=0.
```

So the observed level-local shapes are controlled by the third `q^-9` charge on normal slices with
up to six extra shortened dimensions. After adding the optimistic `rho` diagnostic, these observed
shapes still look safe:

```text
level 4, b=4: E_anc=3, max optimistic rho=7, rho-routed margin=13;
level 5, b=4: E_anc=3, optimistic rho=inf for all three edges.
```

Here `rho=inf` means the cheap paired-spine survivor model cannot realize that shortened-kernel
dimension at the displayed zero budget.

A separate toy near-dimension slice:

```text
child_k=32, dims=(4,3,2,1), zeros=(21,26,31)
```

shows the warning case: `b=0` is infeasible, but `b=1` has `E_anc=12`, already exceeding the
single `q^-9` charge. Under the generic-rank calibration, that same `b=1` toy slice exposes a
rank-defect target charge of `12`, which would make the combined margin positive:

```text
9 + 12 - 12 = 9.
```

This supports the defect-slice rule rather than a blanket normal-slice truncation theorem, but the
charge `12` is not yet certified for RFC. The next rank-event recurrence must compute the actual
RFC shortened-kernel cost, including all-paired compression branches that may pay less than the
generic-rank benchmark.

Regenerating the same toy sweep with optimistic `rho` changes the conclusion:

```text
b=1: E_anc=12, max optimistic rho=1, rho-routed margin=-2;
b=2: E_anc=15, max optimistic rho=1, rho-routed margin=-5;
b=3: E_anc=18, max optimistic rho=1, rho-routed margin=-8;
...
```

Thus generic rank-tail routing is false for the near-dimension toy. The observed level-local
theta-chain shapes avoid this because their zero budgets are much larger relative to child
dimension, but a proof must include an isolation statement excluding the near-dimension
paired-spine pattern from the actual dominant theta_2=-1 chain.

## No-Cycle Interpretation

Defect routing cannot create a literal cycle because every exposed layer is a child-code event at
one smaller depth. The asymptotic concern is a long path that repeatedly avoids normal truncation.
The intended invariant is:

```text
Phi_path =
  9 * (# first-drop kernel rows)
  + sum exposed defect_charge(D_i, |B_i|)
  - sum ancestor E_anc
  - log_q(split/state constants).
```

A normal step keeps `Phi_path` nondecreasing by the normal-slice inequality. A defect step exposes
a positive-codimension layer:

```text
defect_charge(D,z)>0
```

and routes it into the recursive rank event. Thus the only paths that can stay close to dominant
are near-tight normal paths; the observed level-local paths are either infeasible on the normal
slice or have nonnegative margin through `b=6`.

Correction from the defect-conservation diagnostic: routing through the raw flag moment
`F_child((D,z))` is too loose for exposed shortened ambients. The diagnostic
`rfc_defect_conservation.py` finds large negative slack for the toy event `k=32,D=12,z=21` under
that naive flag-count induction. The defect branch must instead charge the canonical rank event
`dim H(B)>=D`; ancestor flags inside `H(B)` are already counted separately by `E_anc`.

Repro command:

```text
python scripts/rfc_distance_analysis/rfc_defect_conservation.py \
  --child-k 16 \
  --expansion 8 \
  --parent-dim 12 \
  --parent-zeros 21 \
  --limit 15
```

Current result:

```text
worst_slack = -76
worst profile: p=10, s=1, a=1, tau=1, rV=11, rK=11
```

The same diagnostic on the small one-layer dim-4 profile is positive:

```text
python scripts/rfc_distance_analysis/rfc_defect_conservation.py \
  --child-k 16 \
  --expansion 8 \
  --parent-dim 4 \
  --parent-zeros 21 \
  --limit 12

worst_slack = 8
```

## Shortened-Kernel Rank Recurrence Diagnostic

The optimistic canonical-rank diagnostic:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py
```

tracks deterministic survivor mechanisms for:

```text
rho_h(D,z) = -log_q Pr[dim H_h(B) >= D]
```

including exact all-paired compression and depth-one root-line collisions. It is not a certificate:
it gives cheap obstruction costs and currently tracks one surviving child subspace rather than sums
of independent defect components.

For the exposed toy event:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 \
  --expansion 8 \
  --dim 12 \
  --zeros 21 \
  --paired 10 \
  --singletons 1 \
  --trace
```

the optimistic cost is:

```text
rho <= 1
```

with trace:

```text
h=5: D=12,z=21,p=10,s=1,hard_theta=0 -> child D=7,z=10
h=4: D=7,z=10,p=1,s=8,forced=7,hard_theta=0,residue_if_a5=3 -> child D=4,z=8
h=3: D=4,z=8,p=4,s=0,hard_theta=0 -> child D=2,z=4
h=2: D=2,z=4,p=2,s=0,hard_theta=0 -> child D=1,z=2
h=1: D=1,z=2,p=0,s=2,hard_theta=0 -> root-line collision, cost 1
```

Thus the generic-rank charge `12` is not believable for RFC-shaped witnesses with paired-spine
cascades. For observed-style level-local defects:

```text
depth 4, k=16, z=21, D=7: rho <= 2
depth 4, k=16, z=21, D=8: rho <= 2
```

This is still positive, but much smaller than the generic-rank values `84` and `104`. The proof now
has to show that these low-cost paired-spine rank events either are already counted by the nested
kernel-chain state or still leave enough combined local charge to dominate `E_anc`.

The trace also gives the isolation foothold: the cheap rank path is not a hard theta-chain path.
A hard connected first-drop row needs `s=a=5`. The cheap trace has `s=1`, then `s=8` with
`residue_if_a5=3`, then all-paired or too-small splits. Therefore the remaining proof can separate:

```text
1. hard theta-compatible rank splits with s=5;
2. non-hard paired-spine rank splits, which either cannot host theta_2=-1 or pay singleton residue.
```

If the optimistic rank recurrence is constrained to hard-compatible or all-paired splits:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 \
  --expansion 8 \
  --dim 12 \
  --zeros 21 \
  --allowed-singletons 0,5 \
  --trace
```

the cheap cost remains `rho<=1`, but the trace contains two hard-compatible steps:

```text
h=5: s=5, hard_theta=1 -> child D=7,z=11
h=4: s=5, hard_theta=1 -> child D=4,z=8
h=3: s=0, all-paired
h=2: s=0, all-paired
```

Thus the earlier one-charge defect-routing test undercounts this constrained hard trace. If both
hard steps are represented as theta-chain rows, their local charge is:

```text
2 * 9.
```

For the toy `b=1` numbers this gives:

```text
2*9 + rho(=1) - E_anc(=12) = 7.
```

This is the current best proof route: non-hard cheap rank paths are isolated by residue/all-paired
classification, while hard-compatible cheap rank paths carry one `q^-9` charge per `s=5` step.

## Charged Strict Hard-Trace Update

The normal-slice checker now has strict hard-trace columns. These use the conservative
kernel-child recurrence:

```text
allowed singleton counts = {0,5},
all five hard singletons forced into the kernel child,
visible quotient loss = 2,
hard step charge = 9.
```

The charge is optimized inside the dynamic program. Thus:

```text
strict_hard_potential = 9 * hard_steps + rho_terminal
```

is the charged optimum, not a post-hoc charge added to a rho-minimizing trace.

For the near-dimension toy:

```text
child_k=32,
dims=(4,3,2,1),
zeros=(21,26,31),
b=(1,1,1),
```

the old loose-rho route is unsafe:

```text
rho-routed margin = -2.
```

The strict hard route is safe:

```text
strict_hard_margin = 15.
```

However, sweeping `b=(b,b,b)` shows the strict-only route is not enough by itself:

```text
b <= 8: strict_hard_margin > 0;
b = 9:  strict_hard_margin = 0;
b = 10: strict_hard_margin = 36 - 39 = -3.
```

The boundary-only comparison column:

```text
boundary_plus_strict_hard_margin
```

adds the scenario's explicit `charge=9`. At `b=10` this gives `6`. Audit convention: this extra
charge is not available for an internal hard segment because `strict_hard_potential` already
counts `9H`. It can be used only after proving a disjoint boundary row. Otherwise, high-defect
shortened ambients need an additional rank-defect, incidence, or high-kernel charge.

If this lemma is too tight globally, the fallback is still finite: implement length-four flags in
the certificate driver and rerun the same state-counting argument. The depth-7 best-transition
diagnostic suggests this fallback will not be on the dominant path.
