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
| High-kernel/local-incidence bonus | 0 | 0 | 0 | open | Next candidate | Need prove hard row carrying large shortened ambient pays more than uniform `q^-9`. |
| Joint nested shortened-rank profile charge | 0 | 0 | 0 | open | Next candidate | May recover missing few dimensions by charging the whole nested profile, not max one edge. |
| Finite `k<=32,e=2` base-case seal | 0 | 1 | 0 | 0% | Promising partial | Depth-5 replica calibration crosses at `z=34` with log2 moment `-115.10`; needs rank-pattern theorem, since component-uniform mode fails. |
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

Raw one-step defect conservation tested on the floor row and retired for this blocker:
`child_k=16,parent_dim=14,parent_zeros=34` has worst slack `-210`, confirming that raw flag
counting is far too loose here.
