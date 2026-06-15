# RFC Distance — Full Report for Review (2026-06-13)

Reviewer: please be adversarial. The headline conclusion is a *negative* result (a claimed structural
wall). The most valuable thing you can do is find a hole in it — especially in Section 9, where the
"no tractable bound" claim is stated most strongly and may overreach.

## 0. Verdict (one paragraph)

The original non-systematic RFC (BaseFold random foldable code; repetition base, determinant-1 fold,
`c=8, k=2048, q=2^128`) is **very likely near-MDS** — exact ground truth at small scale measures a
per-zero charge ~0.85, implying relative distance ~0.84–0.86. But we now have a **proof-grade
structural reason** that this is **not tractably provable by any first-moment / replica recurrence**:
the minimal per-column state closed under the fold is projective space `P^{D-1}(F_q)` (column
dimension `D=2^j`), size ~`q^{D-1}`, which is astronomically large at production. The first-moment
program is therefore closed. The only tractable route left is the paper's combinatorial min-distance
recursion, which is capped below near-MDS and offers limited advantage over the paper.

## 1. Setup and goal

- Code: depth-`d` foldable code. Base (depth 0) = repetition `[c,1]`, message symbol `x -> (x,...,x)`,
  length `c`. Fold (depth `i`): codeword `= [w_L[j] + T_j w_R[j]]_j ++ [w_L[j] + (T_j+1) w_R[j]]_j`,
  where `w_L=enc(m_L), w_R=enc(m_R)` (length `c*2^{i-1}`) and `T_j` is a per-coordinate challenge.
  Production target `c=8, k=2^11=2048, n=ck=16384, q=2^128, lambda=80`.
- Goal: a theorem-grade distance lower bound beating the BaseFold paper, ideally near-MDS, via the
  first moment `B_d(1,z) = E_code[ #(nonzero msg, z-subset of its zero coords) ]`. If
  `B_d(1,k+e) <= 2^-lambda` then whp distance `>= n-(k+e)+1`.

## 2. Diagnosis: why the prior effort stalled

Two facts, both computed:

1. **The proof constant is nearly irrelevant.** In `rfc_distance_certificate.py`, sweeping
   `log2 C(d,N)` from 0 to 8192 moves the certifiable relative distance only `0.871 -> 0.867`. The
   prior "0.27-qdim base-seal budget" anxiety was optimizing a non-issue.
2. **The real variable is the per-zero CHARGE** (the `q^-(e+1)` rank-tail exponent). The prior
   "safe" recurrence suppressed only ~2.4 bits/zero (vs idealized 128) and degraded with depth, so it
   was all-or-nothing and vacuous at production.

This reframed the task from "tune constants" to "measure and tightly bound the per-zero charge."

## 3. Ground-truth oracle (M1)

`rfc_brute_force_moment.py`: exact enumeration of the real code at small `(c,q,depth)`. Uses
`B_d(R,z) = E_T[ sum over nonzero R-tuples binom(common_zeros, z) ]`; exact (Fractions) for full
challenge enumeration, Monte-Carlo for depth>=3. Also emits the exact-support histogram
`A_d(R,w) = E_T[#tuples with exactly w common zeros]` (`--histogram`); identity
`B(z)=sum_w A(w) binom(w,z)` verified.

**Encoder-fidelity correction (important).** The first oracle used INDEPENDENT challenges per subtree.
The real encoder (`crates/cfri/src/backend/basefold.rs::evaluate_over_foldable_domain`) indexes the
challenge as `level[j - half_chunk]` (local position), so **the same challenge vector is shared across
all sibling chunks at a level** (total challenges `c*(2^d-1)`, not `d*c*2^{d-1}`). Depth-1 unaffected;
depth>=2 was recomputed. All numbers below are post-fix.

**Key measurements (corrected oracle):**
- True per-zero charge (units of `q`, at `z=k`): **0.90 (depth 2, exact, stable across q=3,5,7,11)
  -> 0.80 (depth 3, MC).**
- `replica` model (the optimistic one behind `e=71`) is concretely **unsafe**: undershoots the true
  moment by 1.6 bits at depth2,R2,z=6.
- `component-uniform` is a valid upper bound but its saturation rule is wrong (origin of the prior
  `2^2016` artifact).

## 4. The exact top relation (validated)

`rfc_exact_top_relation.py`. From the shared-challenge fold, a single message's codeword zero count is
exactly:

```
parent_zeros = 2u + X
  u = #child columns where both half-codewords vanish (common zeros of the child 2-tuple)
  X = sum over non-common columns jj of independent Bernoulli(g_jj/(q-1)),
      g_jj = [a_jj != 0] + [a_jj != -b_jj]  (when b_jj != 0; g in {1,2}); else 0.
      (a_jj,b_jj) = (W_L[jj], W_R[jj]).
B_d(1,z) = E_childcode[ sum_{m != 0} E_top[ binom(2u + X, z) ] ].
```

This has **no binomial placement factor** — the structural double-count of the old recurrence is gone.
Validated: reproduces the oracle **exactly** (full Fraction equality) at depth 2 (q=3, q=5) and within
~0.002 bits (MC) at depth 3. At depths 2–3 this tightens the bound from the old recurrence's
0.84 / 2.38 bit gaps to **0.00**.

## 5. Why the old recurrence was loose (resolved)

`B_d(R,z) = sum_{p,s,c} 2^s q^{-charge} binom(u,p) binom(n'-u,s-c) B_{d-1}(2R,u)` multiplies the
(already binom-weighted) child moment by free placement factors `binom(u,p) binom(n'-u,s-c)`, so a
high-zero child tuple is re-weighted at every level. Measured gap vs exact oracle: ~1.4 bits (depth 2)
growing to ~4 bits (depth 3) on the (now-superseded) independent-challenge oracle; ~0.84 -> ~2.4 bits
on the corrected oracle. Either way the looseness **compounds with depth** and is independent of the
per-term charge — which is why years of charge/credit-lemma work never closed.

## 6. The exact local charge law

`charge = r + extras - 1` (for `extras >= 1`): the first non-common singleton in an `r`-replica state
costs the full replica rate `r` (rank-1 alignment `q^-(r-1)` times root `q^-1`); each additional one,
conditioned on that alignment, costs only its own root `q^-1`. Fit and validated against the oracle:
`(r,extras) = (2,1)->2.1, (2,2)->3.0, (4,1)->4.0, (4,2)->4.9`, all `= r+extras-1` up to the negligible
`(q-1)/q` finite factor. (Implemented as `component-envelope`.) This fully explains the `2^2016`
artifact: `component-uniform` wrongly sets the charge to 0 when common-zeros saturate child dimension;
the true charge never collapses.

## 7. Relative distance is robust to the charge

At large `q`, the crossing of `binom(n,z) q^k q^{-charge*z} = 2^{-lambda}` gives
`rel_dist ~ 1 - 1/(c*charge)`. For `c=8`:
`charge=1.0 -> 0.875; 0.9 -> 0.861; 0.8 -> 0.844; 0.5 -> 0.750; 0.13 -> 0.04`.
So the measured charge ~0.8–0.9 implies relative distance ~0.84–0.86 (near-MDS-ish), and distance only
collapses if charge falls near ~0.15. **The exact `e=71` (charge=1) is mildly optimistic; the honest
value looks like ~0.84–0.86.**

## 8. The tractable-bound attempt and its failure

`rfc_ub_recurrence.py`: collapse the state to the zero-count distribution `A_h(R,.)` and UPPER-bound the
hit term `X <= Bin(#non-common, 2/(q-1))` (every non-common column treated as max-hittable, `g=2`).
This is a valid upper bound (>= oracle, confirmed at depth 1–2), tractable, scales to depth 11.

**It degrades like the old recurrence** (q=2^128, c=8): rel_dist 0.0625 (d5), 0.0156 (d6), 0.0078 (d7).
Root cause (verified not a bug; depth-1 is near-tight): at replica `R`, a parent coordinate is
common-zero only if the child column `(a,b)` is proportional in `F^R` (prob ~`q^-(R-1)`); generic
rank-2 columns are NOT hittable (`g=0`). The `g=2`-for-all over-bound discards this rank-dependent
suppression — **which IS the charge.** So any scalar (zero-count-only) state loses the charge: by
double-counting (old recurrence) or by over-bounding (this UB).

## 9. The decisive closure test (the wall) — SCRUTINIZE THIS

If the charge lives in column rank/structure, what is the minimal sufficient per-column state, and does
it close under the fold?

**Sufficiency** (`rfc_sufficiency_test.py`, exact enumeration q=3, depths 1–2, R=1,2): the parent zero
distribution is fixed by the child column-type counts `(u, g1, g2)`. Tested coarse summaries:
- `u` (zero-count): INSUFFICIENT
- `(u, rank)`: **INSUFFICIENT** (rank is not the missing statistic)
- `(u, rank, #nonzero-cols)`: INSUFFICIENT
- full type-histogram `(common,g1,g2,dead)`: SUFFICIENT
The sufficient object is only 4 counts summing to `n` (≈`n^3` distinct, polynomial: 17 at n=4, 131 at n=8).

**Closure / minimal alphabet** (`rfc_column_class_closure.py`): the coarsest fold-closed equivalence on
column values (a bisimulation: class of a scalar = zero/nonzero; class of `(a,b)` = the function
`t -> (class(a+t b), class(a+(t+1)b))`). Class counts by replica dim `D`:
```
q=3:  D=1:2, D=2:5, D=4:41, D=8:3281
q=5:  D=1:2, D=2:7, D=4:157, D=8:97657
q=7:  D=1:2, D=2:9, D=4:401, D=8:960801
```
These match **exactly** `#classes(D) = (q^D - 1)/(q-1) + 1 = |P^{D-1}(F_q)| + 1`. The minimal closed
alphabet **is projective space** (column directions) plus zero, and it does not coarsen — consistent
with `a + t b = 0` iff `(a,b)` projectively aligned with `(1,-t)`.

**Claimed consequence:** the minimal sufficient per-column state grows as ~`q^{D-1}`, `D=2^j`, so no
tractable exact (or exactly-tight) first-moment recurrence exists; at production `D=2048, q=2^128` this
is astronomically intractable.

**Reviewer, please attack here.** The bisimulation shows no finite *exact* small-state DP. It does NOT
by itself rule out: (i) a recurrence on *moments / generating functions* of the projective-class
distribution (the class state is huge, but its distribution under the random code may concentrate or
have exploitable algebraic structure); (ii) a tight one-sided *bound* that tracks a coarse functional
of the class distribution rather than the classes themselves. We argued informally that the charge
`q^-(R-1)` is exactly the projective-alignment probability and that coarse functionals lose it (Section
8 shows the obvious coarse bound fails), but we did not prove that *every* moment/GF scheme fails.
This is the gap in the "no tractable bound" claim.

## 10. Where this leaves a depth-11 distance

- **No provable depth-11 distance from the first-moment route.** Best tractable depth-11 distance
  remains the BaseFold paper's combinatorial constant.
- **near-MDS (~0.84–0.86) is very likely true** but, by the above, not tractably first-moment-provable.
- Remaining routes: (A) combinatorial min-distance recursion (paper-style; tractable; constant; capped
  below near-MDS; limited advantage over the paper); (B) a different/algebraic idea (none in hand).

## 11. Specific questions for the reviewer

1. Section 9: does the projective-class bisimulation genuinely preclude a tractable *moment/GF*
   recurrence or a tight coarse *bound*, or only an exact small-state DP? This is the crux.
2. Is the charge law `r+extras-1` (Section 6) correct asymptotically, and is the `2u+X` decomposition
   (Section 4) complete (we believe it is exact and validated, but a second derivation would help)?
3. Section 7's `rel_dist ~ 1 - 1/(c*charge)` is a leading-order large-q heuristic; is it sound enough
   to assert the true relative distance is ~0.84–0.86 from two depth points (d2,d3) of charge data?
4. Is the combinatorial recursion (route A) worth pursuing for a *modest* improvement over the paper,
   or is the paper already tight enough that there's no advantage?
5. Have we mis-modeled the code? (Encoder fidelity, Section 3, is the most likely place for an error.)

## 12. Reproduction

```
# ground truth + charge
python -B scripts/rfc_distance_analysis/rfc_brute_force_moment.py --depth 2 --expansion 2 --q 5 --histogram
# exact top relation vs oracle
python -B scripts/rfc_distance_analysis/rfc_exact_top_relation.py --depth 2 --expansion 2 --q 5
# tractable UB (degrades)
python -B scripts/rfc_distance_analysis/rfc_ub_recurrence.py --depth 7 --expansion 8 --q-log2 128 --security-bits 80 --w-cap 1024 --x-cap 1024
# sufficiency: (zeros,rank) insufficient
python -B scripts/rfc_distance_analysis/rfc_sufficiency_test.py --depth 2 --replica 1 --q 3
# closure: projective-space alphabet (the wall)
python -B scripts/rfc_distance_analysis/rfc_column_class_closure.py --q 5 --max-dim 8
```

## 13. Artifacts (commits, newest first)

```
3d04d19 closure test resolved — minimal column-class alphabet is projective space (WALL)
db3240a sufficiency test — (zeros,rank) INSUFFICIENT; type-histogram sufficient, polynomial
313a2ec tractable scalar UB built and shown to FAIL — charge lives in rank structure
04e9728 derive + validate exact top relation; identify state-explosion blocker
cfb89cc fix encoder-fidelity bug in oracle; resolve open question toward "true, proof-hard"
897d4e3 oracle emits exact-support histogram A_d(R,w); critically confirm diagnosis
964550a replica oracle + lift validator; diagnose 2^2016 artifact
3f3a8a7 clean start — ground-truth-first direction
```
Full narrative and dated progress log: `docs/rfc_distance_analysis/claude_new_direction.md`.
