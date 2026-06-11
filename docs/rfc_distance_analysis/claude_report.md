# Claude Report: RFC Near-MDS Distance Push Review

Date: 2026-06-10
Reviewer: Claude (Fable 5), follow-up to `rfc_fable_audit_2026_06_10.md`.
Scope: review of `rfc_distance_resume_status.md` and the surrounding original/non-systematic RFC
distance-certificate program, grounded against the BaseFold paper.

## 1. Verdict Summary

```text
Statement (near-MDS distance, e=71, c=8, k=2048, q=2^128, lambda=80):
  very likely TRUE (~85-90%).

Proof program (finite-replica/flag first moment closing to a theorem-grade certificate):
  credible but expensive (~50-60% without further restructuring).

Resume status doc accuracy:
  broadly accurate and honest; four correctable issues listed in section 4.
```

The target is information-theoretically optimal in shape: even an ideal random linear code at
these parameters fails at `e=70` and crosses at `e=71`. "Do not claim exact MDS" is exactly
right.

## 2. Independent Verifications Performed

### 2.1 Crossing arithmetic

Recomputed by hand (entropy + Stirling correction):

```text
log2 binom(16384, 2119) ~= 9096
(e+1) log2 q = 72 * 128  = 9216
e=71 ideal moment        ~= -120 bits   (doc: -121.83; slack vs -80 ~= 40 bits, doc: 39.68)
e=70 ideal moment        ~= +5 bits     (unsafe, matches doc)
```

The headline numbers in the resume doc are correct.

### 2.2 Local lemma spot-checks

```text
rfc_u23_tau2_endpoint_lemma.md:
  VERIFIED. Any row dependency forces c_3 != 0, which forces all three projective lines
  equal; X_2 is the diagonal P^1; gamma_2 = 2; theta_2 = -2. Unconditional.

rfc_g1_first_drop_endpoint_lemma.md (h=2 layer):
  VERIFIED as far as it goes. Each constraint row depends on one coordinate line, so the
  r_gen x r_gen minor has multidegree <= 1 per coordinate; multiprojective Schwartz-Zippel
  gives |X_2| <= r_gen (q+1)^(|A|-1), hence gamma_2 >= 1. The a >= 5 floor follows from
  r_gen = 2delta - 1 = 5 <= a. BUT see issue 4.1: the h=3 full-kernel layer is conditional.

rfc_theta_minus_one_isolation_lemma.md, Lemma 2:
  VERIFIED. With s_i = a_i: z_V = p = z_{i+1}, so z_i = 2 z_{i+1} + a_i and
  z_0 = 2^m z_m + sum 2^i a_i. Outer-branch zero-budget burn is a clean identity.

Audit-correction propagation:
  VERIFIED. The product-of-first-moments retirement is consistent across the certificate
  theorem (L3c), proof obligations, manager board, and the --flag-bound best fix. The
  joint marked-line bound (q+1) * F_child((2,z),(1,z+1)) is correctly labeled
  diagnostic-only everywhere.
```

## 3. BaseFold Paper Context

The published distance proof (ZCF23, Theorem 1) is a per-level union bound with zero-budget
recursion:

```text
t_i = 2 t_{i-1} + l_i,
l_i ~= (2(d-1) log n_0 + lambda + 2.002 t_{d-1} + 0.6 n_d) / (log|F| - 1.001).
```

The `0.6 n_d / log|F|` term loses a constant fraction of the block length per level. The paper's
own rate-1/8 example achieves relative distance `0.728` against the Singleton bound `0.875`.
The published technique therefore structurally cannot give `distance >= N - k - O(lambda)`;
this program's pair-counting/flag approach is qualitatively stronger, not a constant-factor
tightening, and is correctly aimed at ideal-random-code behavior:

```text
B_d(1, k+e) <= C(d,N) * binom(N, k+e) * q^-(e+1).
```

The heuristic core supporting truth of the statement:

```text
paired zeros:    (l + t r, l - t r) = (0,0) forces l = r = 0 at the child coordinate;
                 dimension and zero budget halve together; relative gap preserved exactly.
singleton zeros: one root condition each, probability ~ 1/(q-1); the ideal random price.
```

Failure can only come from correlated degenerate families, and the falsification lanes
(seven-copy, GF(5) non-stride, complete-stride, paired-spine, tracked-kernel, dense-endpoint)
have all come back clean after exact-support counting and honest constants. Every apparent
counterexample so far has been an accounting artifact, not a structural one.

## 4. Issues Found

### 4.1 The g=1 row closure is conditional, and the resume doc hides the condition

`rfc_g1_first_drop_endpoint_lemma.md` bounds the full-kernel layer `h = delta = 3` by citing
"the connected full-rank kernel lemma" (`|X_3| <= q+1`, `gamma_3 = |A| - comp = 4`). That
general component lemma is still open by the program's own notes; the same file lists
"higher-drop layers kappa >= g+2" as remaining work, which for this row IS the h=3 layer.
If `gamma_3` were only 2, the layer would contribute `theta_2 = 0` and the row would not be
`-1`. The conclusion `theta_2 = max(-1, -2) = -1` is conditional.

Recommended fix: prove the `delta=3` connected full-kernel layer directly, U23-style, instead
of waiting on the general component theorem. Until then, the resume doc should say
"theta_2 = -1 conditional on the connected full-kernel layer bound".

### 4.2 "What Is Currently Closed" mixes proofs, contracts, and diagnostics

Of the six items listed in the resume doc, only U23 (unconditional) and g=1 (conditional per
4.1) are local proofs. The incidence-framing cap is sound modulo root-injectivity and
`(q-1)` vs `q` constants; the isolation lemma is a skeleton; the nested-flag recurrence is a
contract; the truncation status is a diagnostic. Retitle the section and tag each entry:

```text
proved / conditional / contract / diagnostic.
```

### 4.3 Strategic risk not surfaced: diagnostic crossings scale linearly in N

The corrected checkpoint crossings:

```text
depth 4: e = 49  / N = 128   (0.38)
depth 6: e = 278 / N = 512   (0.54)
depth 7: e = 673 / N = 1024  (0.66)
```

The runnable machinery, even diagnostically, lives in the same Theta(N)-deficit regime as the
published BaseFold bound. Everything between that and `e=71` rests on the unproven L4
recurrence being qualitatively tighter than every relaxation tried so far. No current
computation exhibits near-MDS scaling, even heuristically. The resume doc should state this
explicitly.

Cheap de-risking experiment: compute the exact `B_d(1,z)` (not a checkpoint bound) over a
small field at depths 3-4 and confirm the true moment tracks the ideal curve. This separates
"our bound is loose" (fine) from "the code is not near-MDS at small depth" (fatal).

### 4.4 Construction mismatch between certificate and shipped encoder

The certificate theorem fixes a "determinant-1 RFC fold ... do not switch to the T'=-T
algebra," but the BaseFold paper and the in-repo encoders
(`crates/cfri/src/backend/basefold.rs`, blaze backend) use the paper's fold
`(l + t o r, l - t o r)` with pair matrix `[[1,t],[1,-t]]`, determinant `-2t`, `t` uniform in
`F^*`. No note states the equivalence. Needed: a one-page change-of-variables lemma mapping
the `-T` algebra onto the determinant-1 algebra (with the root-challenge distribution mapping
to uniform-on-`F^*` up to tracked constants), or restate the certificate for the paper's fold.
Cheap now, embarrassing later.

## 5. Next-Step Recommendation

The resume doc nominates the length-four chain domination lemma (blocker 2). Mild pushback:
blockers 1 and 2 are both instances of the same missing object, the formal multi-layer flag
transition theorem

```text
F_h((t_0,z_0), ..., (t_m,z_m))
```

with shared-randomness-safe lifts. The `|A|=2, delta=2, comp=2` row sits at `theta_2 = 0` with
zero local slack and dominates small depth whenever the marked-line state is absent, so it is
load-bearing at every depth; the chain question only governs subdominant paths (depth-6/7
diagnostics show chain length 1 on dominant paths). The contract in
`rfc_kernel_branch_nested_flag_recurrence.md` is already most of the statement. Write the
general theorem once and discharge blockers 1 and 2 as corollaries.

## 6. Prognosis and Hedge

Why the statement is probably true: clean paired compression, ideal singleton root pricing,
extensive failed falsification, and a 40-bit margin that absorbs polynomial slop.

Why the proof is the risky half: the remaining work is a real paper's worth of machinery
(multi-layer flag recurrence with exact-support inversion, general tau-2 layer codimensions,
constants in bits), and the tau >= 3 question could force rebuilding the local theory one
level up if any dominant trace needs visible dimension 3.

Recommended hedge: target the intermediate theorem

```text
distance >= N - k - O(d * lambda / log q)
```

first. Allowing a small constant excess per level (e of a few hundred, relative distance still
~0.86 at rate 1/8) likely needs only the two-layer flag recurrence with crude lifts, no
chain-truncation lemma, and no sharp layer theory. It captures nearly all of the practical
payoff (better proven soundness per query, hence fewer queries and smaller proofs), validates
the recurrence skeleton end-to-end with constants, and turns the e=71 result into a tightening
of a working theorem rather than an all-or-nothing construction.

## 7. References

```text
docs/rfc_distance_analysis/rfc_distance_resume_status.md
docs/rfc_distance_analysis/rfc_distance_certificate_theorem.md
docs/rfc_distance_analysis/rfc_distance_manager_board.md
docs/rfc_distance_analysis/rfc_flag_recurrence_proof_obligations.md
docs/rfc_distance_analysis/rfc_fable_audit_2026_06_10.md
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
docs/rfc_distance_analysis/rfc_theta_minus_one_isolation_lemma.md
docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
docs/rfc_distance_analysis/rfc_theta_minus_one_truncation_status.md

BaseFold: Efficient Field-Agnostic Polynomial Commitment Schemes from Foldable Codes.
  Zeilberger, Chen, Fisch. ePrint 2023/1705, CRYPTO 2024.
  https://eprint.iacr.org/2023/1705
Secbit notes on BaseFold Part IV (RFC distance):
  https://github.com/sec-bit/mle-pcs/blob/main/basefold/basefold-04.md
```
