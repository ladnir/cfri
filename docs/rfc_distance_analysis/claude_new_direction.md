# RFC Distance — New Direction (ground-truth first)

Status: active clean-start plan. Supersedes the open-ended lemma grind recorded in
`rfc_distance_stop_report.md` and `rfc_distance_resume_status.md`. Those remain valid as a record of
what was tried; this note is the new working direction.

Owner intent (2026-06-13): "we just want a code with good distance. exact constants are flexible.
near-MDS, open to interpretation, but certainly better than the original BaseFold paper." So the
deliverable is a *theorem-grade* relative-distance lower bound that beats the paper, pushed as close
to MDS as the honest first moment allows.

## Current understanding (synthesis — read this first)

This section is the settled current view. The chronological "Progress log" below is the audit trail
(including one discarded guess); trust this section where they appear to differ.

**Settled conclusions:**

1. **We now have exact ground truth.** `rfc_brute_force_moment.py` computes the true first moment
   `B_d(R,z)` of the real code by enumeration (exact for depth<=2 / small q; Monte-Carlo for depth 3),
   and `rfc_lift_validator.py` feeds the recurrence EXACT children to isolate one-step error. These
   are the acceptance test for any bound. This is the asset the project previously lacked.

2. **The exact singleton charge law is `charge = r + extras - 1`** (validated vs oracle at r=1,2,4).
   First non-common singleton costs the full replica rate `r`; each additional costs only its root
   `q^-1`. It unifies with the exact r=1 charge (`= extras`) and does NOT depend on common-zero
   saturation.

3. **The `2^2016` base-seal artifact was a charge bug, not mathematics.** `component-uniform` drops
   the charge to 0 once common-zeros saturate the child dimension; the true charge never collapses.
   So all the saturation / credit-lemma machinery was fighting a bug.

4. **The proof constant is irrelevant.** Sweeping `log2 C(d,N)` over 0..8192 moves relative distance
   only 0.871->0.867. Stop optimizing constants and the `e=71` razor's edge.

5. **The recurrence framework is structurally too loose, and its looseness compounds with depth.**
   Even fed the exact charge, `B_d(r,z) = sum 2^s q^-charge binom(u,p) binom(n'-u,s-c) B_{d-1}(2r,u)`
   re-multiplies the child moment by free placement counts that the child already aggregated over —
   double-counting. Measured gap vs exact oracle: ~1.4 bits (depth 2) -> ~4 bits (depth 3); the
   production bound degrades (rel dist 0.40/0.19/0.05 at depth 5/6/7). **No per-term charge fix can
   repair this** — which is precisely why years of charge/credit-lemma work never closed.

**CORRECTION (encoder fidelity bug found and fixed).** The oracle originally used INDEPENDENT fold
challenges per subtree; the real encoder (`evaluate_over_foldable_domain`) indexes the challenge by
the position WITHIN a chunk, so the SAME challenge vector is shared across all sibling chunks at a
level (total `c*(2^d-1)` challenges, not `d*c*2^{d-1}`). Depth-1 is unaffected; depth>=2 ground truth
was for the wrong code and has been recomputed. On the corrected oracle the session's conclusions
SURVIVE and sharpen: charge law `r+extras-1` holds, `replica` is unsafe by 1.6 bits, and
`component-envelope` is safe and *tighter* at the saturated end.

**The open question is now substantially resolved — and the answer leans "statement TRUE, proof
hard," NOT "statement false."** The key realization is that relative distance is ROBUST to the
per-zero charge: at large q, `rel_dist ~ 1 - 1/(c*charge)`, so for c=8:
`charge=1.0 -> 0.875`, `0.9 -> 0.861`, `0.8 -> 0.844`, `0.5 -> 0.750`. Distance only collapses if
charge falls to ~0.15. Measured against the corrected exact oracle:
- True per-zero charge at z=k: **0.90 (depth 2, exact, q-stable across q=3,5,7,11) -> 0.80 (depth 3,
  MC).** That corresponds to relative distance ~0.84-0.86 — i.e. **good / near-MDS-ish.** The exact
  `e=71` (which needs charge=1, rel_dist 0.875) is mildly optimistic; the honest value looks like
  ~0.84-0.86.
- The recurrence's apparent "rel_dist -> 0.05 by depth 7" is its EFFECTIVE charge collapsing to
  ~0.13 — a pure looseness artifact: the recurrence-vs-oracle gap grows 0.84 bits (depth 2) -> 2.38
  bits (depth 3), and its z=k charge falls 0.78 -> 0.62 while the true charge falls only 0.90 -> 0.80.
  The recurrence understates the true distance; the true code is fine.
- Consistent with the BaseFold paper (which proves these foldable codes have good relative distance):
  the charge plausibly stabilizes at a positive constant.

**Remaining genuine uncertainty:** only two clean depth points (d2, d3) for the oracle charge trend,
declining ~0.1/level; d4+ is out of brute-force reach (q=3,d4 needs 3^16 messages). If that decline
were linear-to-zero rather than converging, distance would eventually suffer — but robustness +
the BaseFold theorem make convergence to a positive constant the strong bet. So: **near-MDS in the
loose sense (rel_dist ~0.84-0.86) is plausibly true; exact `e=71` is slightly optimistic; the
in-house recurrence cannot prove either because it is too loose and compounds.**

**Path forward (what would actually settle it):** a first-moment handle that is both rigorous and
tight enough to match the oracle within o(1) bits per level (not the current ~2.5+ that compounds).
Two candidates:
- (a) an exact-support / inclusion-exclusion recurrence that does NOT re-multiply placement counts
  already inside the child moment (kill the double-counting at the source); or
- (b) a direct first-moment computation at production scale by a smarter-than-brute-force method
  (e.g. exploiting the self-similar fold structure to evaluate `E_T[...]` without enumerating T).
Either way, validate every step against the oracle. Do NOT resume charge/credit-lemma refinement on
the existing recurrence — that layer is solved (`r+extras-1`) and is not where the loss is.

## Diagnosis of why the old direction stalled

The whole effort is a first-moment certificate `B_d(1, k+e) <= 2^-lambda`. Two facts, both now
checked by computation, reframe it:

1. **The proof constant is a near-total red herring.** In `rfc_distance_certificate.py`, sweeping
   `--log2-poly-factor` (= `log2 C(d,N)`) from 0 to 8192 moves the certifiable excess only
   `e = 71 -> 137` and relative distance `0.871 -> 0.867`. So the "0.27 qdim base-seal budget"
   panic in the stop report is optimizing a quantity that barely matters. Stop optimizing constants.

2. **The real variable is the per-zero CHARGE (the `q^-(e+1)` exponent), not the constant.** The
   safe two-layer flag bound suppresses only ~2.4 bits per zero versus the idealized 128, so it only
   crosses `2^-80` at base `z=137` (rel dist 0.47) and *degrades with depth* (documented safe
   crossings `z/n`: depth5 0.535, depth6 0.668, depth7 0.782 -> rel dist ~0 at production depth 11).

Consequence: the in-house replica/flag machinery is **all-or-nothing**. It yields near-MDS only if a
tight base seal closes; otherwise it is vacuous at production scale. There is no partial credit to
harvest from it as currently built.

The deeper methodological problem: **the team has been bounding a quantity nobody has ever
measured.** The optimistic `e=71` rests on a `q^{-r|E|}` charge that is documented as false; the
safe bound rests on coarse relaxations (`component-uniform`) that manufacture huge artifacts (e.g.
`B_4(2,17)` at `2^2016` vs scalar `2^-187`, pure saturation artifact). Neither is the truth.

## New direction: measure first, then prove against the measurement

Three milestones, each producing a checkable artifact.

### M1 — Ground-truth oracle (this is the foundation)

Brute-force the *exact* first moment of the real construction at small `(c, q, depth)` by
enumerating the code. The construction (confirmed from `crates/cfri/src/backend/basefold.rs`):

- base code (depth 0): repetition `[c, 1]`, message symbol `x -> (x,...,x)` length `c`;
- fold (depth `i`): `w = encode(m_L), w' = encode(m_R)` each length `c*2^{i-1}`; draw an independent
  challenge `T_j` per child coordinate `j`; codeword `= [w_j + T_j*w'_j]_j ++ [w_j + (T_j+1)*w'_j]_j`,
  length `c*2^i` (determinant-1 fold, matches the proof model).

The exact first moment uses the identity (no need to enumerate zero-sets):

```text
B_d(1, z) = E_T[ sum_{m != 0} binom( zeros(codeword(m)), z ) ].
```

Enumerate all challenge vectors `T` and all nonzero messages `m`, count zeros, accumulate. This is
the true value the entire project approximates. It gives, for the first time, the **measured
per-zero charge** at small scale and a check on which model (replica / component-uniform / safe) is
honest. Validate the oracle two ways: internal (`sum_z B/binom` identities) and against
`rfc_replica_zero_moment.py --q-exact` run at the same small `q`.

Script: `scripts/rfc_distance_analysis/rfc_brute_force_moment.py`.

### M2 — Validated exact-support flag DP

The exact-support flag DP (state `L = pi(K) <= V = pi(W)` with two zero budgets `z_V = p+s-a`,
`z_L = p+s`, per `rfc_depth5_finite_flag_recurrence_target.md`) is the piece every prior review said
was missing and never built. Build it, but **validate every transition against the M1 oracle at
small scale** before trusting it at the base. Structured branches (all-paired, high-support) are
exact Gaussian-binomial / flag counts with no relaxation — this is where the `component-uniform`
artifact is deleted, not patched. Genuinely-hard low-visible-rank branches fall back to the *proven*
local exponents (L1 support-subcode, L3b Grassmann `>= a` cap, `theta_2`), so every term is
exact-or-proven-upper-bound and the total is a rigorous upper bound.

### M3 — Theorem-grade relative distance

Run the validated DP at the production-mapped base and read off the smallest honest `z`, hence the
theorem-grade relative distance after paired compression. Two outcomes, both useful:

- hard branches negligible after exact structured counting -> near-MDS, provable;
- hard branches dominate -> the *true* (still better-than-paper) relative distance, plus the exact
  finite list of low-rank blocks to attack next.

Either way we stop chasing a number (`e=71`) that may not even be true for this code.

## Progress log

- 2026-06-13: Direction reset written. Diagnosis confirmed by computation (constant sweep;
  per-zero charge 2.4 vs 128; safe-crossing depth degradation). Confirmed base code = repetition and
  the det-1 per-coordinate fold from the encoder. Starting M1 brute-force oracle.

- 2026-06-13: **M1 oracle built and validated** (`rfc_brute_force_moment.py`). Exact enumeration of
  the real code; hand-checked (`B_1(1,1)=8`, `B_1(1,2)=3` at c=2,q=3). Monte-Carlo mode added for
  depth >= 3. First ground-truth measurements of the true first moment — the quantity the whole
  project has been approximating but never computed:

  - **The true per-zero charge is ~1.0 (units of q), i.e. ~q^-1 suppression per zero — the
    near-MDS / optimistic regime, NOT the safe bound's ~0.019 (2.4/128 bits).** This is direct
    evidence the code really is near-MDS and the safe-bound collapse is bound looseness, not reality.
    - depth 2 (k=4,n=8): charge ~1.00-1.04 across all z, FLAT in q (q=3,5,7 identical); the real
      moment even dips slightly *below* the idealized union bound at high z (charge > 1).
    - depth 3 (k=8,n=16, q=3 MC): charge ~1.00 at low z, easing to ~0.86 by z=12 (excess 4).
      Unresolved: depth-compounding vs small-q tail effect. This is the key number M2's DP must
      reproduce and then evaluate at production q/depth.

  - **`replica` (the optimistic model behind e=71) is concretely UNSAFE in replica-state lifts**:
    at depth2, parent replica R=2, q=3, z=6 it gives log2 2.567041 < exact 3.209453, undershooting
    the real moment by 0.642413 bits. So e=71 rests on a model now shown to violate the very bound
    it claims in the child states the recurrence uses. Treat e=71 as a target, not a provable one.

  - **`component-uniform` is a VALID upper bound at this scale** (>= truth everywhere, ~1 bit loose
    at depth2,q=3). The catastrophic `2^2016` artifact is therefore NOT intrinsic to it; it erupts
    only in the high-`u` saturation regime (large depth, replica r=2), which is unreachable by
    direct r=1 brute force. Deciding whether that blow-up is real bad-codeword mass or pure artifact
    requires either replica-r=2 ground truth at depth>=4 (infeasible by brute force) or the M2 DP.

  Net: ground truth supports near-MDS (charge ~1), kills the optimistic model's safety, and confirms
  component-uniform is safe-but-loose. Next (M2): build the exact-support flag DP, validate its
  per-shape transitions against this oracle at depth<=3, then evaluate the honest charge at the
  production base to get the theorem-grade relative distance.

- 2026-06-13: **M2 in progress — oracle extended to replica states; one-step lift isolated; the
  `2^2016` artifact precisely diagnosed.**

  - Oracle now computes exact `B_d(R,z)` for ordered R-tuples (`--replica R`), giving ground truth
    for the replica child states the recurrence actually uses (e.g. exact `B_1(2,u)={80,32,12,0,0}`,
    `B_1(4,u)={6560,320,120,0,0}` at c=2,q=3).

  - New tool `rfc_lift_validator.py`: feeds the one-step lift the EXACT child `B_{d-1}(2r,u)` and
    compares to the EXACT parent `B_d(r,z)`, so any gap is the transition's fault, not compounding.

  - Findings (all at c=2,q=3, exact), with TRUE charge backed out per dominant split:
    | regime | extras | quotient_rank | TRUE charge | component-uniform | replica (r·extras) |
    |---|---|---|---|---|---|
    | r=1 (top) | any | - | = extras (each q^-1) | = extras (exact) | n/a |
    | r=2 non-saturated | 2 | 1 | 2.99 | 3 (exact) | 4 (unsafe) |
    | r=2 **saturated** | 2 | **0** | **3.60** | **0 (ARTIFACT)** | 4 (unsafe) |
    | r=2 | 1 | 1 | 2.10 | 1 (loose) | 2 (~exact) |

  - **Root cause of the `2^2016` artifact, now proven:** `component-uniform` sets the singleton
    charge to 0 once common-zeros saturate the child dimension (`quotient_rank=0`). But the measured
    TRUE charge in that saturated regime is ~3.6 — essentially the SAME as the non-saturated ~3.0,
    i.e. roughly the replica rate `r·extras` minus a small component correction. **The charge does
    NOT collapse in saturation; the model's saturation->0 rule is simply wrong.** It stays a valid
    upper bound (over-counts) but the slack compounds (depth2 r2: +1.6/+3.6/+5.1/+5.7 bits at
    z=3..6), which is the depth-5 `2^2016`.

  - `replica` is structurally UNSAFE in the one-step lift: with exact child moments, parent
    R=2, q=3, z=6 gives lift log2 2.567041 versus exact 3.209453. The safer
    `component-envelope` charge gives 4.152003 on the same row. This is the current reproducible
    unsafe/safe sanity check for the local charge layer.

  - The intermediate idea "charge near r*extras with a correction" is superseded by the exact law
    below. Do not build a new DP around the discarded intermediate charge.

- 2026-06-13: **M2/M3 — exact charge law found; but the recurrence framework is shown to be
  structurally too loose. This is the key result of the session and it redirects the whole effort.**

  - **Exact singleton charge law (validated against the oracle across r=1,2,4):**
    `charge = r + extras - 1` (for extras>=1). The first non-common singleton costs the full replica
    rate `r` (rank-1 alignment q^-(r-1) times root q^-1); each additional one, conditioned on that
    alignment, costs only its own root equation q^-1. Unifies with the exact r=1 charge (= extras)
    and is independent of common-zero saturation. Implemented as `component-envelope` in
    `rfc_replica_zero_moment.py`. (An earlier guess `(r-1)*extras+1` was UNSAFE at r=4 by 3.3 bits;
    discarded. Measured true charges: (r,extras)=(2,1)->2.1, (2,2)->3.0, (4,1)->4.0, (4,2)->4.9,
    all = r+extras-1 up to the negligible (q-1)/q finite factor.)

  - **The `2^2016` artifact is fully explained:** component-uniform charges 0 in saturation; the
    true charge is `r+extras-1`, which never collapses. All the saturation/credit-lemma work was
    fighting a charge bug.

  - **BUT: with the correct charge the recurrence still does not prove near-MDS — because it is a
    loose upper bound whose looseness COMPOUNDS with depth.** Recurrence vs exact oracle (c=2,q=3):
    depth 2 gap ~1.4 bits, depth 3 gap ~4 bits (same support, growing excess). Production trend
    (exp8, q=2^128, correct charge) then degrades: rel_dist 0.398 (d5) -> 0.187 (d6) -> 0.047 (d7).
    The earlier "stable 0.746" was an artifact of the UNSAFE overcharge, not real.

  - **Root cause (structural, not the charge):**
    `B_d(r,z) = sum_{p,s,c} 2^s q^-charge binom(u,p) binom(n'-u,s-c) B_{d-1}(2r,u)`
    multiplies the child moment by free placement factors `binom(u,p) binom(n'-u,s-c)`. But
    `B_{d-1}(2r,u)` already aggregates over all size-`u` common-zero placements, so re-multiplying by
    placement counts over-counts configurations. This union-bound looseness compounds over depth,
    independent of any per-term charge. **No charge refinement can fix it.** This explains why years
    of charge/credit-lemma work (component-uniform, theta_2 algebra, the credit lemmas) never closed:
    they refined the per-term charge of a recurrence whose aggregation looseness grows with depth.

  - **Redirect.** The true first moment (oracle) is well-behaved, so a tight bound exists — just not
    via this recursive-union-bound aggregation. Path to near-MDS = a TIGHTER first-moment handle:
    (a) a recurrence that does not re-multiply placement counts already inside the child moment
    (exact-support / inclusion-exclusion so common-zero placements are not double-counted), or
    (b) a direct first-moment computation at production scale by a smarter-than-brute-force method.
    The oracle + lift validator stay as ground truth: any new bound must match within o(1) bits per
    level, not the current ~2.5+ bits/level that compounds.

- 2026-06-13: **Encoder-fidelity bug found and fixed; the open question is now substantially
  resolved toward "statement true, proof hard."** (Larger milestone.)

  - **Bug:** the oracle used independent per-subtree fold challenges; the real encoder
    (`evaluate_over_foldable_domain`) shares one challenge vector across all sibling chunks per level
    (`level[j-half_chunk]`, local index). Total challenges `c*(2^d-1)`, not `d*c*2^{d-1}`. Fixed the
    oracle's `encode` to match exactly (depth-1 unchanged; depth>=2 recomputed). Earlier depth>=2
    oracle numbers (and the gap measurements based on them) were for the wrong code.

  - **Re-validation on the corrected oracle:** charge law `r+extras-1` holds; `replica` unsafe by
    1.6 bits; `component-envelope` safe and tighter at the saturated end (z=6 essentially exact).

  - **Key insight — relative distance is robust to the charge:** at large q,
    `rel_dist ~ 1 - 1/(c*charge)`, so charge 0.8-1.0 all give rel_dist 0.84-0.875; distance only
    collapses if charge falls to ~0.15.

  - **Measured true charge (corrected oracle, at z=k):** 0.90 (depth 2, exact, q-stable across
    q=3,5,7,11) -> 0.80 (depth 3, MC) => true rel_dist ~0.84-0.86 (good / near-MDS-ish). Exact
    `e=71` (charge=1, rel_dist 0.875) is mildly optimistic; honest value ~0.84-0.86.

  - **The recurrence's degradation is a looseness artifact, now quantified against the CORRECTED
    oracle:** gap grows 0.84 (d2) -> 2.38 bits (d3); recurrence z=k charge falls 0.78->0.62 while the
    true charge falls only 0.90->0.80; the recurrence's "rel_dist -> 0.05 at d7" = effective charge
    ~0.13, pure looseness. The true code is fine; the recurrence cannot show it.

  - **Verdict:** near-MDS in the loose sense (rel_dist ~0.84-0.86) is plausibly TRUE and consistent
    with the BaseFold distance theorem; the in-house recurrence is just too loose to prove it. This
    is "proof hard," not "statement false." Caveat: only d2,d3 clean charge points (declining
    ~0.1/level); d4+ is beyond brute-force reach (q=3,d4 needs 3^16 messages).

  - **Implication for the build:** the inclusion-exclusion recurrence is still the target, and its
    bar is now concrete — reproduce the oracle's ~0.8-0.9 charge within o(1)/level (the current
    recurrence loses ~0.15 charge/level). The oracle now also emits `A_d(R,w)` as the per-`w`
    acceptance test.

- 2026-06-13: **Tight top relation derived and VALIDATED EXACTLY; exact propagation hits a
  state-explosion blocker.** (Goal: tighten the bound.)

  - **Exact top relation** (`rfc_exact_top_relation.py`), from the corrected shared-challenge fold:
    `parent_zeros = 2u + X`, where `u` = common zeros of the child 2-tuple and
    `X = sum over non-common child columns of independent Bernoulli(g/(q-1))`,
    `g = [a!=0] + [a!=-b]` for `b!=0` (g in {1,2}), else 0; `(a,b)=(W_L[jj],W_R[jj])`.
    Hence `B_d(1,z) = E_childcode[ sum_{m!=0} E_top[ binom(2u+X, z) ] ]` with NO binomial placement
    factor — the double-count is gone by construction.
  - **Validation:** reproduces the brute-force oracle EXACTLY (full Fraction equality) at depth 2 for
    q=3 and q=5, and matches the depth-3 MC oracle within ~0.002 bits. So at d2/d3 this tightens the
    bound from the old recurrence's 0.84 / 2.38 bit gap to **0.00**.
  - **BLOCKER for production scale:** turning this into a recurrence requires propagating the joint
    distribution of `(u, m1, m2)` for the child 2-tuple, which in turn needs the joint column-TYPE
    distribution of the codewords (zero-pattern AND the `a=-b` proportionality relation). Under the
    replica doubling (depth d, R=1 -> 2-tuple at d-1 -> 4-tuple at d-2 -> 2^j-tuple at d-j), the
    column type lives in F^{2^j} and its sufficient statistic grows in richness per level ->
    state explosion. Collapsing to a scalar zero-count (what the old recurrence does) is exactly what
    forces the binom double-count and the ~0.15 charge/level loss. So there is a real tension:
    tractable scalar state <=> double-count looseness; exact <=> exponential state.
  - **Most promising way through (next):** the first moment only needs certain MOMENTS of the
    column-type distribution (e.g. `E[binom(2u,z)]` needs the u-distribution; the `X` correction
    needs `E[(m1+2m2)*binom(2u,z-1)]/(q-1)`), not the full joint. A moment-closure / transfer-operator
    recurrence over a small fixed set of column-type moments may be both tractable and tight. Whether
    such a finite moment set closes under the fold is the open question. Build target: a recurrence on
    the exact zero-count distribution `A_d(R,.)` augmented with the minimal extra moments needed for
    the `X` term, validated against the oracle to confirm o(1)/level loss before scaling.

- 2026-06-13: **Tractable scalar-state bound BUILT and shown to FAIL — the blocker is fundamental.**
  (`rfc_ub_recurrence.py`.) Tried the tractable middle path: collapse to the zero-count distribution
  `A_h(R,.)` and UPPER-bound the hit term `X <= Bin(#non-common, 2/(q-1))` (every non-common column
  treated as max-hittable, g=2). Valid upper bound (>= oracle, confirmed depth 1-2), tractable, scales
  to depth 11.

  - **Result: it DEGRADES like the old recurrence** — q=2^128, exp 8: rel_dist 0.0625 (d5), 0.0156
    (d6), 0.0078 (d7). Worse than the BaseFold paper.
  - **Why (fundamental):** at replica R, a parent coordinate is common-zero only if the child column
    `(a,b)` is proportional in F^R (prob ~q^-(R-1)); generic rank-2 columns are NOT hittable (g=0).
    The `g=2`-for-all over-bound discards this rank-dependent suppression — which IS the charge.
  - **Definitive characterization:** the per-zero charge lives in the column RANK-TYPE structure. Any
    scalar (zero-count-only) recurrence loses it — by double-counting (old recurrence, ~0.15
    charge/level) or by over-bounding (this UB, total charge loss). The exact relation is tight but
    needs the full column-type distribution (state explosion). No tractable scalar bound exists; now
    demonstrated, not conjectured.
  - **Depth-11 distance? No.** The first-moment/replica route cannot give a tight bound without
    tracking rank structure. Best provable depth-11 distance remains the BaseFold paper's; the true
    ~0.84-0.86 (oracle charge extrapolation) is unproven.
  - **Redirect (next move):** abandon scalar first-moment recurrences. (1) RANK-MOMENT closure: track
    moments of the column rank-type distribution; minimal closing set open; validate vs oracle. Hard
    but the only first-moment route to near-MDS. (2) COMBINATORIAL min-distance recursion
    (BaseFold-paper style): bound `d_h` from `d_{h-1}` via codeword weights directly, sidestepping the
    first moment and rank-tracking; tighten to BEAT the paper's constant. Option (2) is the likelier
    path to an actual depth-11 theorem.
