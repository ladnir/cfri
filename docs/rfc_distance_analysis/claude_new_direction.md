# RFC Distance — New Direction (ground-truth first)

Status: active clean-start plan. Supersedes the open-ended lemma grind recorded in
`rfc_distance_stop_report.md` and `rfc_distance_resume_status.md`. Those remain valid as a record of
what was tried; this note is the new working direction.

Owner intent (2026-06-13): "we just want a code with good distance. exact constants are flexible.
near-MDS, open to interpretation, but certainly better than the original BaseFold paper." So the
deliverable is a *theorem-grade* relative-distance lower bound that beats the paper, pushed as close
to MDS as the honest first moment allows.

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

  - **`replica` (the optimistic model behind e=71) is concretely UNSAFE**: at depth2,q=3,z=6 it
    gives log2 0.830 < truth 1.210 — it undershoots the real moment. So e=71 rests on a model now
    shown to violate the very bound it claims. Treat e=71 as a lower target, not a provable one.

  - **`component-uniform` is a VALID upper bound at this scale** (>= truth everywhere, ~1 bit loose
    at depth2,q=3). The catastrophic `2^2016` artifact is therefore NOT intrinsic to it; it erupts
    only in the high-`u` saturation regime (large depth, replica r=2), which is unreachable by
    direct r=1 brute force. Deciding whether that blow-up is real bad-codeword mass or pure artifact
    requires either replica-r=2 ground truth at depth>=4 (infeasible by brute force) or the M2 DP.

  Net: ground truth supports near-MDS (charge ~1), kills the optimistic model's safety, and confirms
  component-uniform is safe-but-loose. Next (M2): build the exact-support flag DP, validate its
  per-shape transitions against this oracle at depth<=3, then evaluate the honest charge at the
  production base to get the theorem-grade relative distance.
