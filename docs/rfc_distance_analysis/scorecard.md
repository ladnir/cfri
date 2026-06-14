# RFC Distance Proof Scorecard

Scope: original non-systematic RFC distance certificate unless marked otherwise.

Purpose: track which proof ideas are paying rent. This is a working scorecard, not a theorem
statement. Update it whenever an idea gets a real test: a deterministic recurrence run, a local
lemma, an audit, or a counterexample.

## Scoring

```text
win:       idea closed a blocker, tightened a bound, or survived a critical audit;
partial:   idea helped but left a real gap or needs another lemma;
loss:      idea was false, double-counted, anti-conservative, or failed the stress row;
open:      plausible but not tested enough.
```

The `score` column is:

```text
wins / (wins + partials + losses)
```

Open items are excluded from the denominator.

## Current Scorecard

| Idea | Wins | Partials | Losses | Score | Current Verdict | Evidence |
|---|---:|---:|---:|---:|---|---|
| Exact-support / finite-replica route instead of fixed-set MDS | 3 | 1 | 0 | 75% | Keep as main architecture | Removes marked-core overcount, handles shared child randomness, matches first-moment target; still needs final recurrence/constants. |
| Canonical diagram certificate theorem | 0 | 1 | 0 | 0% | New umbrella plan | `rfc_canonical_diagram_certificate_plan.md` packages the repeated local fixes into one block grammar: child diagrams and quotient/root data counted once, consumed kernels carried, unconsumed fibers covered, rank defects charged by incidence. This is the reset intended to stop row-by-row patching. |
| Simple global potential probe | 0 | 1 | 0 | 0% | Useful negative/locator | `rfc_block_potential_probe.py` shows a naive state-feature potential bottlenecks on `support3_stratified_to_3_6`; with positive zero reward it needs `level_weight=3.93994737` qdims. This justifies moving to block-specific ledger credits instead of treating `(level,dimension,zeros)` as enough. |
| Block-credit potential probe | 0 | 1 | 0 | 0% | Useful locator, top row unresolved | `--credit-profile local-incidence` moves the bottleneck from support-three to `top_tau1_to_2_15` and gives support-three `1.06800847` qdims of slack. The old `current-target` ceiling returns the tight row to support-three at `level_weight=1.93994737`, but the audited profile without top credits has `level_weight=3.00795584` and remains tight at the top row. This now says the block grammar needs a real top-row theorem or a row-specific boundary treatment. |
| Top tau-one/root-kernel carry contract | 0 | 1 | 1 | 0% | Top-edge credits rejected | `rfc_tau1_carry_kappa_audit.py` shows `top_tau1_to_2_15` has no descendant tau-one line, parent `charged_postroot_qdim=-1`, `kernel_dim=0`, and `kernel_lift_qdim=0`; do not spend `tau1_full_line_carry` or `kernel_fiber_cover` on that edge. The generic contract may still apply later, but it does not solve the current top bottleneck. |
| Top boundary finite seal | 0 | 1 | 0 | 0% | Adopt as hybrid route | `rfc_top_boundary_base_seal_decision.md` records the controlled exception: handle `top_tau1_to_2_15` with a finite depth-5 theorem `B_5(1,34)<=2^-80`, then use the reusable block grammar below. Skipping the top transition gives `level_weight=1.93994737`, bottlenecked by support-three, with support-two/base rows slack. |
| Base-seal calibration budget | 0 | 1 | 0 | 0% | High-risk, non-theorem baseline | `rfc_base_seal_budget.py` shows the optimistic scalar top boundary has only `35.10435419` bits, or `0.27425277` qdims, of uniform child-bound calibration slack. Dominant `u=0..5` child states tolerate only about `0.29..0.31` qdims of loss relative to scalar. Since scalar is retired as a theorem, this is a falsification threshold for the finite DP, not proof slack. |
| Base-seal child-gap table | 0 | 1 | 0 | 0% | New tail obstruction | `rfc_base_seal_child_gap.py` compares candidate depth-4 child values to the scalar one-`u` ceilings. The `component-uniform` child model passes low `u=0..12` but fails from `u=13` onward, so the finite DP must control the high-common-zero tail as well as the dominant scalar `u=0..5` rows. |
| Product of child first moments for shared child events | 0 | 0 | 1 | 0% | Retired | Fable/Claude audit found shared-randomness bug; replaced by joint flag states. |
| Joint multi-layer flag state `L <= V` | 3 | 1 | 0 | 75% | Keep | Corrects product bug, supports decomposable marked-line row, supports kernel-chain accounting; still globalizing. |
| Determinant-1 nonzero-root normalization | 2 | 0 | 0 | 100% | Closed construction constant | Binary-field compatible; singleton root cost is `q^-1 * q/(q-1)` and fits finite bucket. |
| Tau-two layer-codimension algebra | 2 | 2 | 0 | 50% | Keep, still local work | Found true `a=5,delta=3,g=1` penalty and fixed old shortcut; full-kernel endpoint/higher drops remain. |
| Old generic/component tau-two shortcut | 0 | 0 | 1 | 0% | Retired | False for connected `a=5,delta=3,comp=1,g=1`; misses first-drop layer. |
| Strict kernel-child coverage lemma | 2 | 1 | 0 | 67% | Keep | Proved `dim K>=D-2`, `L` zero on `P union S`, `K<=L+L`; gives strict child event. Needs global use. |
| Loose paired-spine `rho` rescue | 0 | 1 | 2 | 0% | Diagnostic only | Useful counter-signal, but unsafe for toy near-dimension rows and not a proof lower bound. |
| Charged strict hard-trace potential | 2 | 2 | 1 | 40% | Keep with caveats | Fixes `b=1` hard toy and observed slices; strict-only fails high-defect floor rows. |
| Adding `charge=9` on top of internal strict hard trace | 0 | 0 | 1 | 0% | Retired for internal segments | Audit says double-counting because `9H` already includes internal hard rows. Boundary-only use remains possible. |
| Boundary-plus hard-trace charge | 1 | 1 | 0 | 50% | Plausible with disjoint-boundary lemma | Closes reachable floor row through `b=13`; fails at `b=14`; valid only if boundary row is proved disjoint. |
| Production zero-floor reachability | 1 | 1 | 0 | 50% | Useful narrowing | Excludes `child_k=32,z=21` toy from production `e=71`; reachable `z=34` stress still has a gap. |
| Observed-shape defect routing | 2 | 0 | 0 | 100% | Looks safe | Child-k 8/16 observed slices are safe or strict-impossible with huge defect charge. |
| Reachable `child_k=32, zeros=(34,39,44)` stress row | 0 | 1 | 1 | 0% | Current blocker | Better than unreachable toy, but at `b=14` has a 3 q-dimension gap even with one boundary charge. |
| Raw one-step defect conservation for floor row | 0 | 0 | 1 | 0% | Retired for this blocker | `rfc_defect_conservation.py --child-k 16 --parent-dim 14 --parent-zeros 34` gives worst slack `-210`; raw flag counting is too loose. |
| Scalar `q^{-r|E|}` rank-pattern recurrence | 0 | 0 | 1 | 0% | Retired as theorem | Base repetition / low-visible-rank blocks lose `q^{(r-1)(s-1)}` locally; needs span/visible-support/flag state. |
| All-lift cover shortcut | 0 | 0 | 1 | 0% | Retired as theorem | `--cover-lift-mode all` moves depth-5 checkpoint to `z=35`, but it drops tau-positive quotient-line/plane incidence and is anti-conservative. |
| Quotient-incidence flag-lift recurrence | 0 | 2 | 0 | 0% | Keep as corrected candidate | Tau-zero duplicate-lift covering is safe; tau-positive branches must count quotient lines/planes via support-subcode/exterior incidence plus invisible-fiber dimensions. |
| Tau-one fixed-flag quotient-line incidence | 0 | 1 | 0 | 0% | Safe local brick, not a closer | `rfc_tau1_quotient_line_incidence_lemma.md` proves the post-root exponent `f_A + m_A - 1 - |A|`; `--report-tau1-incidence` shows the `z=34` tau-one trace rows have `support_saving=0`. |
| Tau-one carried quotient-line state | 0 | 0 | 0 | open | Current proof target | `rfc_tau1_carried_line_state.md` identifies the needed state: carry a marked root-compatible quotient line `R <= E_A(V/L)` through the child diagram. `rfc_tau1_full_line_carry_lemma.md` proves the generic conditional projective-fiber count `q^kappa_phi`. The first target is `(4,7)>=(2,8)`, whose dominant child-table row has `outer_tau1_charged_postroot_qdim=3`. Expanded diagnostics show this equals the invisible-fiber budget, so carrying only the root-visible image saves zero; next prove the transition map and `kappa_phi` for the full line. |
| Carried two-layer flag merge | 1 | 1 | 0 | 50% | Keep, exposes next diagram | `rfc_carried_flag_diagnostic.py` carries `F_3((4,7),(2,8))`, improves `(4,3)>=(2,5)` to `(4,4)>=(2,5)`, and saves `251.98` bits; next state is a two-marked-line diagram. |
| Two-marked-line plane diagram | 1 | 1 | 0 | 50% | Keep, local but insufficient | `rfc_two_marked_line_plane_lemma.md` replaces a coarse `q^4` ancestor choice by `q+1`, saving another `381.42` bits on the carried path. |
| Finite marked-plane state recurrence | 1 | 8 | 0 | 11% | Keep, frontier moved to quotient-incidence state | `rfc_marked_plane_state_recurrence.md`, `rfc_marked_plane_state_diagnostic.py`, `rfc_diagram_path_dp.py`, `rfc_flag_state_choice_diagnostic.py`, and `rfc_flag_bad_pair_classifier.py` turn the hand-expanded two-line diagram into a state scan/path diagnostic. Joint choices help at level 2; nested quotient/subspace/consumed-kernel diagnostics move depth-5 `z=34` from `1740.40` to `1232.89`, but do not close. |
| Dominant-row proof-obligation labels | 0 | 1 | 0 | 0% | Useful classifier, not a bound | `rfc_pair_flag_table_recurrence.py` now labels traced choices as `support2-quotient-diamond`, `tau1-full-line-carry`, `support2-line-quotient-impossible`, etc. `rfc_residual_trace_classification.md` records that the baseline level-3 pair sum is worse than scalar fallback, so the missing work is a joint quotient-diagram theorem and tau-one transition map, not more independent pair enumeration. |
| Scalar collapsed-active rerouting | 0 | 1 | 0 | 0% | Promising theorem plumbing | Applying the collapsed-active filter inside scalar lifts, not just pair tables, removes the dominant `(4,3)>=(4,4)` scalar overcount and tightens the `(4,7)>=(2,8)` baseline from `810.95` to `549.21`. Needs proof as exact-support rerouting. |
| Nested tau-zero equal-container collapse | 1 | 1 | 0 | 50% | Strong new exact-support filter, not final | The lemma in `rfc_nested_tau0_equal_container_filter.md` reroutes upper tau-positive rows when a lower tau-zero sibling has the same child projection container. It improves the strong `(4,7)>=(2,8)` table from `423.08` to `27.06` bits and the demanded depth-5 no-closure `z=34` checkpoint to `1080.36` bits. It does not close the certificate; support-two frame and tau-two rows take over. |
| Nested quotient/subspace/consumed-kernel counting | 0 | 1 | 0 | 0% | Strong diagnostic, conditional theorem target | The three modes `inner-in-outer`, `inner-in-outer`, and `tau0-inner-contained` lower the level-3 stress table to `303.44` and full depth-5 `z=34` to `1232.89`. `rfc_exact_support_quotient_state.md` now states the needed compatibility labels, especially `K_lower = W_lower cap K_upper` for upper-visible nested quotient counting. Decoded trace columns show the top stress row still has remaining quotient lifts `outer=8, inner=1`, so the residual is the outer support-two tau-two frame count. |
| Kernel-lift-only cover | 0 | 1 | 1 | 0% | Useful after richer state, still not enough | Earlier `--cover-kernel-lift` kept crossing at `z=137`; after nested diagnostics it lowers `z=34` to `924.69`, but crossing remains `z=133` and tau-two/tau-one quotient chains dominate. |
| Support-two tau-two quotient-frame bound | 1 | 2 | 0 | 33% | Keep, but old stress row is no longer dominant | The decoded `(4,7)>=(2,8)` structural trace has outer `a=2,tau=2,charge=4,lift=12` and adjusted qdim `8`, while the nested inner tau-one line is already reduced to one remaining quotient dimension. `--support2-diamond-mode child-only` lowers that level-3 pair sum from `303.44` to `61.01` bits and full depth-5 `z=34` from `1232.89` to `1095.86`, but the new blocker is `(4,2)>=(2,4)` with a strong bottom/kernel zero budget. |
| High-lift support-two component planes | 1 | 1 | 0 | 50% | Strong local improvement, next blocker exposed | `rfc_support_two_high_lift_component_plane_bound.md` proves the intended count shape for `parent_span=2,tau=2,a=2,K=0,dim V=4`: after `V` is fixed, the two rank-one components choose child 2-planes inside codimension-one slices, saving four q-dimensions. Diagnostic mode `--support2-component-plane-mode high-lift` moves the full-level-2 demanded checkpoint to `z=34` value `967.01` and crossing `z=130`; now `a=3,delta=3,comp=3` tau-two rows dominate. |
| Support-three component-plane stratification | 2 | 2 | 0 | 50% | Local route upgraded; constants/import remain | `rfc_support_three_component_plane_bound.md` gives a safe two-q-dimensional saving for the decomposable `a=3,delta=3,comp=3,K=0,dim V=4` row. The new `rfc_support_three_rank_defect_incidence.md` charges the rank-defect stratum by an extra-hyperplane incidence cost, supporting `--support3-component-plane-mode stratified` with the same exponent as the old rank-3 sensitivity. This moves the checkpoint to `200.01114103` bits and `crossing_z=72`; finite constants and recurrence import remain. |
| Strong-bottom positive-kernel containment | 0 | 1 | 0 | 0% | Local lemma plausible, not enough | `--consumed-kernel-mode inner-kernel-contained` charges a lower positive kernel inside the upper kernel. On `(4,2)>=(2,4)` it moves the table only from `932.90` to `931.48` bits before a codimension-one support-two line-quotient row takes over. |
| Support-two line-quotient impossibility | 0 | 4 | 0 | 0% | Valid local filter, small fair-baseline gain | `--support2-line-quotient-filter` removes decomposable tau-two support rows with `dim(V/L)=1`, where nonempty root-line support is impossible. `--exact-filtered-empty` proves the lower `(4,2)>=(2,4)` entry is empty and improves the strong-bottom table to `803.48`. In the fair `tau0-inner-contained` baseline it moves depth-5 `z=34` from `1095.86` to `1094.26`. The `1478.22` regression belongs to the stronger `inner-kernel-contained` diagnostic and is partly sparse-demand state loss; `--full-table-until 2` and `--demand-closure-passes 1` both improve that strong-mode report to `1352.53`, while a second closure pass does not move it. |
| Scalar kernel-lift container cover | 1 | 2 | 0 | 33% | Major theorem target, quotient incidence still counted | With exact-support filters, support-two high-lift, safe support-three, and `--cover-kernel-lift`, the demanded depth-5 checkpoint reaches `final_span_1_z_report,34,565.92392782` and `crossing_z=102`. This removes duplicate `K_parent <= L+L` fibers but keeps quotient line/plane incidence. It still leaves about `5.05` q-dimensions and exposes a marked-line-in-container row. |
| Root-kernel container cover probes | 0 | 1 | 0 | 0% | Promising diagnostic, not theorem-grade | New modes `--support2-root-kernel-cover-mode kernel`, `--support4-root-kernel-cover-mode kernel`, and `--tau1-root-kernel-cover-mode kernel` test counting root-compatible containers once after quotient/root data are fixed. With safe support-three, the full checkpoint improves to `424.70026934` bits and `crossing_z=99`; with rank-3 or stratified support-three it reaches `200.01114103` bits and `crossing_z=72`. The proof obligation is canonical unconsumed fibers with quotient incidence retained. |
| Tau-one child-line carry | 1 | 1 | 0 | 50% | Useful ceiling, exposes support-four blocker | `--tau1-child-line-carry-mode top` models conditioning a selected child tau-one row on the parent full quotient line. With current best structural modes it lowers `z=34` from `565.92` to `435.05` bits, but top mass moves to tau-zero rows feeding `(2,20)`. Needs a real marked-line state before theorem use. |
| Marked line inside fixed container | 0 | 1 | 0 | 0% | Useful but superseded as top blocker | The previous frontier was level-3 `(4,7)>=(1,8)`: outer tau-zero container plus inner connected tau-one row `a=2,delta=1,comp=1,K=0,V=2`, with one remaining q-dimension. The tau-one child-line carry diagnostic handles the selected-row ceiling and moves the frontier to support-four tau-two rows. |
| Support-four decomposable tau-two exterior row | 0 | 0 | 0 | open | Current frontier | After tau-one child-line carry, the top child state is `(2,20)`, dominated by `a=4,delta=4,comp=4,tau=2,K=0,dim V=4`, child `(4,8)`, term `413.29391501`. The local `q^4` family may be real (`[4 choose 2]_q`), so the next proof needs a marked-component child state or a different global argument. |
| One-layer shortened child flag ambient | 0 | 0 | 1 | 0% | Not useful for current base seal | Diagnostic `--flag-bound best-shortened` leaves the depth-5 checkpoint at `z=137` and the `z=34` trace unchanged. |
| High-kernel/local-incidence bonus | 0 | 0 | 0 | open | Next candidate | Need prove hard row carrying large shortened ambient pays more than uniform `q^-9`. |
| Joint nested shortened-rank profile charge | 0 | 0 | 0 | open | Next candidate | May recover missing few dimensions by charging the whole nested profile, not max one edge. |
| Finite `k<=32,e=2` base-case seal | 0 | 2 | 0 | 0% | Promising but needs flag refinement | Depth-5 replica calibration crosses at `z=34` with log2 moment `-115.10`; scalar rank-pattern recurrence failed audit; two-layer flag checkpoint crosses at `z=137`, so the finite seal now needs a tighter exact-support flag recurrence. |
| Systematic all-level adaptation | 0 | 1 | 0 | 0% | Deferred | Construction/correctness seems plausible; active proof push is original non-systematic. |

## Active Frontier

The current pressure point is:

```text
depth 5, z=34:
baseline sparse pair-table value:              1740.39750674 bits
with nested quotient/subspace/kernel modes:    1232.88847175 bits
with additional scalar kernel-cover diagnostic: 924.69069031 bits
with support2/support3 component planes
  plus scalar kernel-cover diagnostic:          565.92392782 bits
with tau-one child-line carry diagnostic:       435.05329836 bits
with root-kernel cover probes, safe support3:   424.70026934 bits
with root-kernel cover probes, stratified support3: 200.01114103 bits
target:                                         -80 bits
```

The old level-3 pair-table obstruction is no longer the only frontier. After scalar
collapsed-active filtering, nested parent-flag diagnostics, component-plane counts, kernel-fiber
covering, and the latest root-kernel cover probes, the remaining safe-mode trace is dominated by
the decomposable support-three tau-two rank-stratification row.

Three plausible ways to turn this into a win:

```text
1. formalize scalar kernel-lift container covering while keeping quotient incidence counted;
2. replace the tau-one child-line carry diagnostic by an explicit marked-line state;
3. import the support-three rank-defect incidence constants and then attack the remaining
   root-kernel/tau-one/container-state gap.
```

The current best theorem-safer trace is:

```text
level 5, state (1,34), after root-kernel cover probes:
  top row tau=0, child (2,19), term 424.48634741

level 4, state (2,19):
  top row tau=2,a=3,delta=3,comp=3, child (4,8), term 408.56599456

level 3, state (4,8):
  value -121.19264508
```

The stratified support-three trace routes through `(2,15) -> (3,6) -> (3,2)>=(1,4)` and leaves
`200.01114103` bits. The lower child table `(3,2)>=(1,4)` is already strong
(`-240.28575448` bits), so the next proof should focus on importing the local incidence constants
and then on the remaining container/fiber theorem, not lower pair-table completion.

## Update Rule

When an idea is tested, update:

```text
Wins / Partials / Losses
Score
Current Verdict
Evidence
```

Also add a dated note below if the test changes the active frontier.

## Dated Notes

### 2026-06-12

Canonical diagram certificate reset added. The new plan in
`rfc_canonical_diagram_certificate_plan.md` records the pattern behind the successful local
repairs: count one child diagram over shared randomness, keep quotient/root incidence as event
data, cover only unconsumed duplicate fibers, and charge rank defects by incidence. This is now the
umbrella route; future diagnostics should be judged by whether they fit the block grammar or force
a genuinely new kind of certificate node.

Block-credit potential probe added. `rfc_block_potential_probe.py` still shows that global
`(level,dimension,zeros,node)` features alone bottleneck on `support3_stratified_to_3_6`. Once the
support-three incidence credit is treated as a reusable block, the bottleneck moves to
`top_tau1_to_2_15`; once support-two is also credited as a sensitivity check, the top row remains
tight. A broad `current-target` ceiling makes top/support-two/base rows slack and returns
support-three as the tight row at a lower per-level budget. This is progress if the top carry and
incidence credits can be formalized; it is not yet a distance certificate.

Support-three rank-defect incidence lemma added. The lemma charges the proportional/dependent
active-restriction stratum by noting that a rank defect gives a nonzero projective relation among
the three active functionals; a fixed relation places the child four-container in one extra
hyperplane (`q^-4`), and there are only `q^2+q+1` relations. This recovers the two q-dimensions
between safe support-three and rank-3 sensitivity and is exposed as
`--support3-component-plane-mode stratified`.

Root-kernel container cover probes added. The modes
`--support2-root-kernel-cover-mode kernel`, `--support4-root-kernel-cover-mode kernel`, and
`--tau1-root-kernel-cover-mode kernel` count narrow root-compatible container fibers once after the
quotient/root data have been charged. With theorem-safer support-three mode, the demanded depth-5
checkpoint is `final_span_1_z_report,34,424.70026934` and `crossing_z=99`. With rank-3 or
stratified support-three it reaches `200.01114103` and `crossing_z=72`. The top safe-mode blocker
is the decomposable support-three tau-two row `a=3,delta=3,comp=3,K=0,dim V=4`; the rank-defect
incidence lemma supplies the local charge needed to interpret the rank-3 exponent theorem-wise.

Residual trace classifier added. The current proof-shaped sparse run reports
`final_span_1_z_report,34,1478.66370843` in this checkout. The bound-following path has a
level-4 tau-one full-line carry row and a level-3 support-two quotient-diamond row. A direct
level-3 trace for `(4,7)>=(2,8)` shows the pair-enumerated sum is worse than scalar fallback
(`943.43723477` versus `549.21247145`), while the top pair rows classify as
`support2-quotient-diamond` plus `tau1-quotient-line`. This is a partial win for problem
localization: the next proof work is a joint quotient-diagram theorem plus the tau-one
`phi : E'_A -> E_A` transition/kernel computation, not more broad pair enumeration.

Nested tau-zero equal-container collapse added. The old first tau-one full-line target row has an
inner tau-zero sibling with the same child projection dimension; exact nested witnesses therefore
force the upper active support to be zero in that container. With
`--nested-tau0-equal-container-filter`, the strong `(4,7)>=(2,8)` table improves from
`423.08002115` to `27.05765334` bits. The demanded depth-5 no-closure checkpoint improves to
`final_span_1_z_report,34,1080.36409460`. This is a real exact-support win, but the certificate is
still open; the next visible competitors are support-two quotient-frame and tau-two
layer-codimension rows at state `(2,15)`.

High-lift support-two component-plane diagnostic added. For the decomposable `a=2,delta=2,comp=2`
row with `K=0` and `dim V=4`, each rank-one component plane lies in a codimension-one slice of
the fixed `V`, saving four q-dimensions versus the scalar quotient placement. With
`--support2-component-plane-mode high-lift` and full lower tables through level 2, the demanded
depth-5 checkpoint reports `final_span_1_z_report,34,967.01172212` and `crossing_z=130`. The
state `(2,15)` is now topped by `p=6,s=3,a=3,tau=2,delta=3,comp=3`, so the next local blocker is a
tau-two layer-codimension row rather than support-two high-lift placement.

Support-three component-plane and scalar kernel-fiber checkpoint added. The safe support-three
mode saves two q-dimensions on the decomposable `a=3,delta=3,comp=3,K=0,dim V=4` row, moving state
`(2,15)` from `1068.39337516` to `1057.89054335` bits and exposing the adjacent tau-one full-line
row. Adding the scalar kernel-lift container cover gives the current best full depth-5 checkpoint:
`final_span_1_z_report,34,565.92392782` and `crossing_z=102`, about `5.0463` q-dimensions above
the `2^-80` target. The new frontier is level-3 `(4,7)>=(1,8)`, an outer tau-zero container plus
an inner connected tau-one quotient-line row with one remaining q-dimension; the next state should
carry a marked child line inside the fixed container.

Tau-one child-line carry diagnostic added. The mode
`--tau1-child-line-carry-mode top` conditions selected child tau-one rows on a parent full quotient
line. With the current best structural modes, the demanded depth-5 checkpoint improves to
`final_span_1_z_report,34,435.05329836`, about `4.0239` q-dimensions above target. The frontier
moves to a support-four decomposable tau-two exterior row in state `(2,20)`: `a=4,delta=4,comp=4`,
child `(4,8)`, term `413.29391501`.

### 2026-06-11

Charged hard-trace audit completed. Internal hard segments cannot add the scenario `charge=9` on
top of `9H`; that is double-counting. Boundary-plus remains valid only with a disjoint-boundary
lemma. Production zero-floor reachability excludes the abstract `child_k=32,z=21` toy, but the
reachable `zeros=(34,39,44)` row still leaves a 3 q-dimension gap at `b=14`.

Depth-5 base-seal candidate tested. The replica rank-pattern calibration gives
`B_5(1,34)` log2 moment `-115.10435419`, exactly matching the production zero floor at
`child_k=32`. The component-uniform shortcut fails badly, so this is a rank-pattern theorem route,
not an old component-uniform route.

Depth-5 rank-pattern contract written. The top-level mass at `z=34` is dominated by `c=0` splits;
the first `c=1` term is more than `123` bits below the best. Remaining proof task is the
multi-coordinate rank-pattern induction through depth five.

Raw one-step defect conservation tested on the floor row and retired for this blocker:
`child_k=16,parent_dim=14,parent_zeros=34` has worst slack `-210`, confirming that raw flag
counting is far too loose here.

Depth-5 scalar rank-pattern recurrence audited and retired as a theorem. The local charge
`q^{-r|E|}` fails on low-visible-rank blocks; a base repetition block of size `s` misses
`q^{(r-1)(s-1)}` before span savings. The finite base seal remains plausible only as an
exact-support flag recurrence. Diagnostics now bracket the situation:

```text
optimistic scalar replica: z=34
span/subspace only:        z=249
two-layer flag checkpoint: z=137
```

So the active task is tightening the finite flag recurrence, not proving the old scalar contract.
The concrete target is now recorded in `rfc_depth5_finite_flag_recurrence_target.md`.

Covering/projective flag-lift diagnostic added to `rfc_flag_span_moment.py`. Results:

```text
none:       crossing_z=137
tau0:       crossing_z=135
tau1:       crossing_z=137
tau2:       crossing_z=129
tau0tau1:   crossing_z=133
tau0tau2:   crossing_z=61
tau1tau2:   crossing_z=129
all:        crossing_z=35, z34 vector log2=27.64399707
projective z34 heuristic: 27.64399707 - 128 = -100.35600293
```

This makes the next proof target sharper: construct a valid container/projectivization recurrence
for quotient lifts. If that is valid with small constants, the depth-5 base seal may close at the
production floor `z=34`.

Covering flag-lift lemma target written in `rfc_covering_flag_lift_lemma.md`.
Correction: the `all` cover mode is anti-conservative as a theorem. For tau-positive branches,
quotient lines/planes are event data; dropping their incidence count misses factors like
`q^(m-1)` for tau-one lines or the Grassmann/exterior family for tau-two planes. Tau-zero duplicate
lift covering remains safe. The corrected proof obligation is quotient-incidence/fiber accounting:
count the quotient datum sharply, then do not multiply again by duplicate extensions inside the
same child container.

Corrected depth-5 checkpoint trace written in `rfc_depth5_flag_checkpoint_trace.md`. Safe tau-zero
covering leaves the `z=34` vector log moment at `2497.05`, and the best trace moves into tau-one
quotient-incidence chains. Kernel-lift-only covering has no crossing effect (`z=137`). The next
candidate must be a finite quotient-incidence DP, not a lift-cover shortcut.

The ideal one-layer shortened-child flag diagnostic also has no effect: `--flag-bound
best-shortened` leaves the crossing at `z=137` and the `z=34` best trace unchanged. Combining safe
tau-zero covering with kernel-lift covering still only reaches `z=135`. These push against more
scalar/one-edge ambient tweaks and toward a joint finite quotient-incidence chain state.

Subagent audit agreed that the quotient-incidence DP is well-posed only if exact witness profiles
and quotient data are carried explicitly. It also warned that recursive child flags may form a small
inclusion diagram rather than a single chain. The first safe tau-positive brick is now written as
`rfc_tau1_quotient_line_incidence_lemma.md`: for fixed child flag and support `A`, the universal
post-root quotient-line exponent is `f_A + m_A - 1 - |A|`, with binary-field constants included.

Tau-one incidence diagnostics added to `rfc_flag_span_moment.py`. On the safe tau-zero `z=34`
trace, the tau-one rows have `support_saving_qdim=0`; the charged post-root exponents equal the
universal fixed-line exponents. Therefore the immediate blocker is not a missing tau-one local
support-subcode saving, but the recursive joint child-state/quotient-plane accounting after those
tau-one rows.

Child-bound trace diagnostics added. In the safe tau-zero `z=34` trace the top row is outer-first,
then the next rows are inner-first; with kernel-lift covering the upper rows switch to outer-first
but the crossing remains `z=135`. This reinforces that no single one-layer child relaxation choice
is the whole gap. The next candidate is an actual recursive joint child flag/diagram state.

Trace semantics corrected: the previous displayed safe-tau-zero trace followed the outer projection
spine even when the child-bound relaxation selected an inner-first child state. `--trace-z` now
defaults to the bound-following path, with `--trace-follow projection` available for the old view.
The corrected `z=34` path is tau-one at levels 5, 4, and 3, tau-two at level 2, then a base tau-one
row. This makes the immediate proof target sharper: a joint state must preserve the inner-first
child flag created at level 4 and its interaction with the later tau-two boundary row.

Carried-flag merge diagnostic added. Preserving the level-4 flag `F_3((4,7),(2,8))` through the
next transition strengthens the level-2 child flag from `(4,3)>=(2,5)` to `(4,4)>=(2,5)`, saving
`251.98` bits in the current coarse evaluator. The following expansion creates a 2-plane with two
marked lines `(1,4)` and `(1,3)`, which confirms the auditor's warning that the real state can be a
small incidence diagram rather than a total chain.

Two-marked-line plane lemma target written. In the exposed diagram, once the outer tau-two row fixes
the child 2-plane and one marked line, the inner tau-zero line costs at most `q+1` choices in that
plane rather than the coarse `q^4` ancestor choice. The diagnostic estimates another `381.42` bits
of saving, for `633.39` bits combined with the carried-flag merge. Applying both to the displayed
path leaves vector log2 moment `1863.66`, still `15.18` q-dimensions above the `2^-80` target. This
is real progress but still local; the full `z=34` gap remains much larger.

Finite marked-plane state target added. The new scanner enumerates rows where a carrier child plane
with one marked line can absorb an additional tau-zero child line for `q+1` choices. On the exact
carried-path row `(4,4)>=(2,5)` at layer level 2, it reproduces the `381.41503750` bit saving from
the hand calculation. The next required step is not another one-off local saving, but propagation of
ordered marked-plane diagram states through the recurrence. Side audit agreed this is a real
recurrence brick if used as a joint diagram transition, with explicit merge/equality rules and no
product of child moments.

First diagram-state skeleton implemented. `rfc_diagram_state.py` now has node/edge state,
same-dimension containment merging with max zero budget, and the ordered marked-line insertion
operation. It also derives the child diagram of a two-layer flag transition; for the carried row
this gives `I0:d1:z3;O0:d2:z0;O1:d1:z4|I0<=O0;O1<=O0`, matching the desired two-line-in-plane
shape. The marked-plane scanner now emits canonical carrier, successor, and transition diagram
keys. This is still not the full DP: it only gives the state representation and one transition
brick.

Marked-plane grouping mode added. Grouping positive rows by transition diagram shows a repeated
family `I0:d1:z_a;O0:d2:z0;O1:d1:z_b|I0<=O0;O1<=O0`, with best rows saving exactly `3` q-dimensions
before finite constants. This supports building a small finite diagram DP rather than chasing many
unrelated special cases.

Path-DP and inline-recursive diagnostics added. `rfc_diagram_path_dp.py` recovers the known default
depth-5 `z=34` carried-path savings exactly:

```text
carry merge:    251.97763219 bits
marked plane:   381.41503750 bits
combined:       633.39266969 bits
residual:        15.18484798 q-dimensions
```

Injecting only the marked-plane q+1 brick into `rfc_flag_span_moment.py` as
`--flag-bound best-marked-plane` is selected on the trace (`dominant_h=-5`) and improves the top
vector moment to `2244.71357608` bits, but it leaves the crossing at `z=135`. This is a useful
failure: the local q+1 brick is not enough unless the recurrence also carries the higher outer layer
through inner-first collapses.

Dominant-choice two-layer flag table added as `--flag-bound best-two-layer-table`. This is closer
to the desired DP because parent rows can query a table value for child flags, and
`--report-flag-state` exposes entries directly. It confirms the next blocker: level-2 marked-plane
states save the expected `381.41503750` bits, but the level-3 target `(4,7)>=(2,8)` reports zero
additional table saving because the table is still driven by scalar-state dominant choices. The
mode gives the same depth-5 top vector moment `2244.71357608`, same depth-5 crossing `z=135`, and
same depth-6 crossing `z=305`. Next version must store flag-state expansion choices, not just
flag-state values.

Targeted flag-state choice diagnostic added. For `(4,4)>=(2,5)` at level 2, the truncated pair sum
saves `506.75207249` bits (`3.95900057` q-dim), agreeing that the marked-plane brick has real
summed strength locally. For the level-3 carried target `(4,7)>=(2,8)`, the truncated pair sum loses
`907.21115036` bits (`7.08758711` q-dim) even though the optimistic best pair is extremely small.
This is the strongest warning so far: a proof needs canonical witness selection, exact-support
grouping, or a charging argument for the high-mass pair family. Naively summing joint expansion
choices is worse than the current baseline.

Bad-pair classifier added. The level-3 high-mass family is sharply concentrated: the top 12 pair
products already equal the displayed truncated pair sum, and the dominant outer-choice group alone
has log2 mass `2094.40570138`. Its outer row is the tau-one high-lift witness

```text
p=3, s=1, a=1, tau=1, child=(4,4), z=3, charge=1, lift=19.
```

This changes the next proof obligation. We no longer need to explain a diffuse pair-sum failure
first; we need to show that this high-lift tau-one witness is counted canonically once per parent
flag or that its lift multiplicity is charged by exact support/zero data before inner refinements
are summed.

Kernel-lift split tested on the same level-3 flag. The collapsed-active filter removes the
equal-dimension exact-flag overcount and lowers the loss from `7.08758711` to `4.08510402`
q-dimensions, but does not close the row. The fixed-table diagnostic
`--kernel-cover-mode unconsumed-container` subtracts only the tau-positive kernel-lift factors
while keeping child table values fixed; it changes the pair sum to `940.85227227` bits against the
same `1187.19455102` bit coarse baseline, giving `1.92454905` q-dimensions of slack. This says the
quotient-line/plane incidence should remain counted; the next theorem target is duplicate
kernel-lift covering after the child flag and local quotient/root datum are fixed. The dominant
original outer row splits as `kernel_lift=15`, `quotient_lift=4`, making the target very specific.

The unconsumed-kernel condition is now written into the covering lemma and the multi-layer
transition theorem. The safe rule is: after fixing the child container and canonical quotient/root
datum, duplicate choices of `K_parent <= L+L` may be covered once only if no later/sibling profile
marks hidden subspace data inside that kernel lift. Quotient incidence remains event data. This is
not yet scored as a win because the finite marked-plane recurrence still has to consume the covered
exponent directly.

Classifier sibling-consumption audit added. The dominant un-covered pair has outer kernel lift
`15`, outer kernel dimension `3`, inner kernel dimension `0`, and is marked
`top_outer_kernel_unconsumed_by_inner=yes`. The next two exact pairs have inner kernel dimension
`1` and are marked `no`, so they fall into the consumed-kernel exception. Under
`--kernel-cover-mode unconsumed-container`, the top covered rows are again marked `yes`. This is a
partial proof signal, not a win: descendants below the displayed pair still need their own
unconsumed/consumed split.

Stricter sibling-only cover tested. `--kernel-cover-mode sibling-unconsumed` alone does not close
the level-3 stress row; the pair sum is `2092.28967759` bits, still `7.07105568` q-dimensions over
the coarse/table baseline, because consumed-kernel rows become dominant. Combining it with
`--exclude-collapsed-active` gives `1071.43723477` bits against the `1187.19455102` baseline,
saving `0.90435403` q-dimensions. This is the current proof-shaped closure: prove exact-flag
collapsed-active rerouting, then cover only sibling-unconsumed kernel fibers.

Pair-enumerated table recurrence added. The bounded command
`rfc_pair_flag_table_recurrence.py --depth 5 --stop-level 3 --proof-shaped --term-limit 300
--report-flag-state 4,7,2,8 --last-level-report-only` builds full pair tables through level 2 and
reports the level-3 stress state as `810.94626976` bits. At that point the level-3 coarse flag
bound is already `810.94626976`, so the improvement is coming from the child level-2 pair table.
This is a partial win for the recurrence architecture. The new blocker is computational: full
unpruned level-3 pair-table construction timed out, so the next version must be sparse or
demand-driven.

Sparse pair-table demand mode added. `--demand-next-level` runs end-to-end through depth 5 with
proof-shaped filters and moves the crossing from `z=137` to `z=133`. It builds 599 demanded level-2
entries and 2076 demanded level-3 entries. This is real but modest progress: the architecture
propagates some pair-table savings, but it still does not approach the depth-5 base-seal floor
`z=34`. The same run reports `final_span_1_z_report,34,1740.39750674`, still `14.22185552`
q-dimensions above the `2^-80` target.

Sparse trace diagnostics added. `--trace-z 34 --trace-span 1` reconstructs the dominant scalar path
and shows zero log-sum overhead at every displayed state. `--trace-table-state 3,4,7,2,8` shows
that the level-3 flag table entry used by the level-4 row is not a pair-table win:

```text
baseline_log2 = table_log2 = 810.94626976
pair_sum_log2 = 943.43723477
row_count     = 4432
```

The top pair row has local logs `1037.25029842` and `906.66533592`, child flag
`(4,3)>=(2,5)` at `-1000.47839956`, and no sibling-unconsumed kernel cover. This shifts the
frontier from sparse pair-table plumbing to a nested tau-positive local theorem or richer diagram
state.

Nested quotient/subspace/kernel diagnostics added. The level-3 stress state now has a proof-target
ladder:

```text
original sparse proof-shaped table: 810.94626976
scalar collapsed-active baseline:  549.21247145
nested structural table value:     303.43723477
```

End-to-end depth-5 `z=34` moves as follows:

```text
original sparse proof-shaped:                   1740.39750674
with nested quotient/subspace/consumed-kernel:  1232.88847175
with additional scalar kernel-cover diagnostic:  924.69069031
with anti-conservative tau0tau2 lift cover:      567.15416718
```

The anti-conservative tau-two run reaches only `crossing_z=41`, so even an idealized tau-two lift
removal would not finish the certificate. The next theorem has to carry exact-support quotient
data recursively, including the remaining tau-one chain.

Canonical quotient-state target added in `rfc_exact_support_quotient_state.md`. It turns the latest
diagnostic modes into proof obligations and identifies where each one is safe: collapsed-active
rerouting is an exact-support canonicalization, nested quotient counting requires an upper-visible
compatibility label, nested subspace counting is a parent-flag containment count, and consumed
kernel containment requires witness nesting that places a lower tau-zero layer inside the upper
kernel.
