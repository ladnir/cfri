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
| Carried two-layer flag merge | 1 | 1 | 0 | 50% | Keep, exposes next diagram | `rfc_carried_flag_diagnostic.py` carries `F_3((4,7),(2,8))`, improves `(4,3)>=(2,5)` to `(4,4)>=(2,5)`, and saves `251.98` bits; next state is a two-marked-line diagram. |
| Two-marked-line plane diagram | 1 | 1 | 0 | 50% | Keep, local but insufficient | `rfc_two_marked_line_plane_lemma.md` replaces a coarse `q^4` ancestor choice by `q+1`, saving another `381.42` bits on the carried path. |
| Finite marked-plane state recurrence | 1 | 6 | 0 | 14% | Keep, needs kernel-lift container cover | `rfc_marked_plane_state_recurrence.md`, `rfc_marked_plane_state_diagnostic.py`, `rfc_diagram_path_dp.py`, `rfc_flag_state_choice_diagnostic.py`, and `rfc_flag_bad_pair_classifier.py` turn the hand-expanded two-line diagram into a state scan/path diagnostic. Joint choices help at level 2; level-3 pair-sum is worse, but the classifier localizes the bad mass and the fixed-table kernel-cover what-if closes it with `1.92` q-dim slack. |
| Kernel-lift-only cover | 0 | 0 | 1 | 0% | Not useful for current base seal | `--cover-kernel-lift` keeps quotient incidence and leaves depth-5 checkpoint at `z=137`; combined with safe tau-zero covering it still only reaches `z=135`. |
| One-layer shortened child flag ambient | 0 | 0 | 1 | 0% | Not useful for current base seal | Diagnostic `--flag-bound best-shortened` leaves the depth-5 checkpoint at `z=137` and the `z=34` trace unchanged. |
| High-kernel/local-incidence bonus | 0 | 0 | 0 | open | Next candidate | Need prove hard row carrying large shortened ambient pays more than uniform `q^-9`. |
| Joint nested shortened-rank profile charge | 0 | 0 | 0 | open | Next candidate | May recover missing few dimensions by charging the whole nested profile, not max one edge. |
| Finite `k<=32,e=2` base-case seal | 0 | 2 | 0 | 0% | Promising but needs flag refinement | Depth-5 replica calibration crosses at `z=34` with log2 moment `-115.10`; scalar rank-pattern recurrence failed audit; two-layer flag checkpoint crosses at `z=137`, so the finite seal now needs a tighter exact-support flag recurrence. |
| Systematic all-level adaptation | 0 | 1 | 0 | 0% | Deferred | Construction/correctness seems plausible; active proof push is original non-systematic. |

## Active Frontier

The current pressure point is:

```text
child_k=32,
zeros=(34,39,44),
b=14,
gap = 3 q-dimensions after one boundary charge.
```

Three plausible ways to turn this into a win:

```text
1. prove a disjoint boundary row for the failing high-defect state;
2. prove a high-kernel/local-incidence bonus of at least 3 q-dimensions;
3. seal the finite k<=32,e=2 base case directly via rank-pattern induction.
```

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
