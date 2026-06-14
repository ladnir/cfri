# RFC Distance Stop Report

Scope: original non-systematic RFC/BaseFold-style distance certificate work in this worktree.

Status: stop/handoff document. This is not a completed proof. It records the current proof
architecture, the progress made, the evidence gathered, the mistakes found, and the precise
remaining blockers.

## Executive Summary

The active goal was to prove a near-MDS first-moment distance certificate for the original
non-systematic RFC, with default target:

```text
c = 8
k = 2048
N = c*k = 16384
q = 2^128
lambda = 80
target excess e = 71
```

The intended certificate statement is:

```text
B_d(1, k+e) <= 2^-lambda.
```

This implies no nonzero codeword has `k+e` or more zero coordinates, and therefore:

```text
distance >= N - (k+e) + 1.
```

For `k=2048,e=71`, the zero target is:

```text
k+e = 2119.
```

The key current conclusion is:

```text
We do not have a theorem-grade distance certificate yet.
```

But the work made real progress. The proof problem is now much better localized:

1. The original scalar/replica recurrence matches the ideal first-moment scale and reaches the
   desired crossing, but it is not theorem-safe.
2. The top reusable-block credit initially proposed for the transition `(1,34)->(2,15)` is illegal.
3. The viable architecture is now hybrid:
   - seal the top boundary by a finite depth-5 theorem,
   - use the reusable canonical block/diagram grammar below that boundary.
4. The finite top seal is tight. It must prove:

   ```text
   B_5(1,34) <= 2^-80.
   ```

5. The optimistic scalar calibration gives:

   ```text
   log2 B_5(1,34) = -115.10435419.
   ```

   This gives only:

   ```text
   35.10435419 bits = 0.27425277 qdims
   ```

   of uniform calibration slack against the `2^-80` target.

6. The latest correction exposed a high-common-zero tail obstruction in the depth-4 child state.
   With the coarse `component-uniform` model, child rows `u=0..8` match scalar, but `u>=9` exceed
   the scalar one-`u` ceilings.
7. The high-tail mechanism is now identified: an all-singleton/common-zero staircase where the
   coarse model stops charging once the common-zero child support saturates child dimension.
8. The tail is not obviously fatal. The recovery needed is much smaller than full scalar recovery:
   about `0.70` qdims at `u=9` and `16.38` qdims at `u=17`, while the trace has much larger missing
   scalar charge available.

The next proof target, if work resumes, is:

```text
prove a high-common-zero tail lemma for the finite depth-5 base seal,
especially controlling B_4(2,u) for u >= 9 and the all-paired top branch
B_5(1,34) -> B_4(2,17).
```

If that tail lemma cannot be made theorem-grade, the hybrid base-seal route should be abandoned and
the proof should pivot back toward more explicit row-by-row/top-specific analysis.

## Construction Assumptions

The active proof is for the original non-systematic RFC, not the systematic all-level variant.
Systematic notes remain in the folder as deferred reference material.

The fold algebra used for the current theorem statements is the determinant-1 binary-compatible
form:

```text
left + T * right,
left + (T+1) * right,
```

with:

```text
T uniform nonzero.
```

For `T in F^*`, singleton root compatibility is injective and costs at most:

```text
(q-1)^-1 = q^-1 * q/(q-1).
```

The factor `q/(q-1)` is negligible for `q=2^128` and is kept in the finite-constant bucket.

This avoids the paper-style `T'=-T` issue that is problematic over binary fields.

## Why This Is Not An MDS Claim

The target is near-MDS, not exact MDS. The intended statement is not that every `k` coordinates are
independent. It is a first-moment bound ruling out a nonzero codeword with at least `k+e` zeros.

The BaseFold paper's statement that the code is not MDS does not immediately contradict this goal.
Non-MDS means some `k`-sized information-set/minor property fails. The current target allows an
excess:

```text
e = 71.
```

So the desired distance is below the exact MDS distance by an explicit finite excess. That said, the
paper's non-MDS construction should still be checked directly against any future final theorem, to
make sure it does not imply a larger bad-zero family near `k+71`.

## Proof Architecture

The proof route is a finite-replica first moment, not fixed-set MDS.

The object is:

```text
B_d(r,z)
```

which is the aggregate first moment over zero sets of size `z` of nonzero ordered `r`-tuples of
messages that all vanish on those zero coordinates.

At the top:

```text
B_d(1,k+e) <= 2^-lambda
```

is enough for the distance certificate.

The root recurrence doubles replica dimension in the child: a parent line becomes a two-dimensional
child span. A zero set splits into:

```text
p = paired child-coordinate groups
s = singleton child-coordinate groups
z = 2p+s.
```

For singleton groups, there is a visible root image. The corrected child flag state is:

```text
L = pi(K) <= V = pi(W),
z_V = p+s-a,
z_L = p+s,
```

where:

```text
W = parent witness subspace,
R = visible image of W on singleton support,
A = support of R,
a = |A|,
tau = dim R,
K = ker(W -> R).
```

The point of this state is to stop invisible kernel directions from passing through singleton
constraints for free. This is the main correction over the old scalar recurrence.

## Important Corrections And Retired Ideas

### Product Of First Moments

Retired. It is invalid to multiply two child first moments when both events live in the same child
code randomness.

This killed the old split:

```text
F_child(1,z+1)^2
```

for decomposable tau-two rows. The replacement must be a joint marked-line/frame state, such as:

```text
F_child((2,z_outer), (1,z_outer+1))
```

with the finite `(q+1)` frame-completion factor.

This correction came from the Fable/Claude audit and is now accepted.

### Scalar `q^{-r|E|}` Singleton Charge

Retired as a theorem. The optimistic scalar replica recurrence charges a generic singleton block by
`r` equations per non-common singleton coordinate. That is the right ideal/random-linear calibration
but false in low-visible-rank blocks.

The scalar recurrence remains useful only as a calibration baseline.

### All-Cover Lift Shortcut

Retired as a theorem. The diagnostic `all cover-lift` mode moves the depth-5 crossing close to the
target:

```text
crossing_z = 35.
```

But it erases tau-positive quotient-line/plane event data. This is anti-conservative. It is useful
only as a negative/sensitivity check.

### Top Tau-One / Root-Kernel Credit

Rejected for the immediate top transition:

```text
top_tau1_to_2_15: (1,34) -> (2,15).
```

The audit reports:

```text
charged_postroot_qdim = -1
kernel_dim = 0
kernel_lift_qdim = 0
```

Therefore neither:

```text
tau1_full_line_carry
kernel_fiber_cover
```

is legal on that immediate top edge.

This was the point that forced the hybrid architecture: finite top seal first, reusable block
grammar below.

## Hybrid Architecture

The current architecture is:

```text
top boundary:
  finite depth-5 base-seal theorem;

below the boundary:
  canonical diagram/block ledger with reusable local blocks.
```

The top boundary theorem is:

```text
B_5(1,34) <= 2^-80.
```

Why depth 5? It is the finite base case matching the production top trace after scaling the
recursion. At production scale, `e=71` maps to the finite boundary target `z=34`.

The reusable lower grammar is still promising after skipping the illegal top transition. The
potential probe:

```text
python -B scripts/rfc_distance_analysis/rfc_block_potential_probe.py \
  --zero-grid 0.2:2:0.05 \
  --credit-profile audited-top-kernel \
  --skip-transition top_tau1_to_2_15 \
  --top 1 --show-transitions
```

reports:

```text
level_weight = 1.93994737 qdims
```

with the bottleneck moved back to the expected support-three row:

```text
support3_stratified_to_3_6.
```

This is not a proof, but it is evidence that the lower grammar may be reusable once the boundary is
sealed.

## Base-Seal Calibration

The script:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_budget.py
```

uses the optimistic scalar replica recurrence as a calibration baseline. It reports:

```text
log2 B_5(1,34) calibration = -115.10435419
target log2                 =  -80
slack                       =   35.10435419 bits
uniform child-loss budget   =   35.10435419 bits
uniform child-loss budget   =    0.27425277 qdims
```

For individual dominant child states, the one-`u` loss budgets are:

```text
u=2: 36.82450305 bits = 0.28769143 qdims
u=1: 37.19373686 bits = 0.29057607 qdims
u=3: 37.24450680 bits = 0.29097271 qdims
u=0: 38.75573821 bits = 0.30277920 qdims
u=4: 38.29697422 bits = 0.29919511 qdims
u=5: 39.90396303 bits = 0.31174971 qdims
```

This must be interpreted carefully:

```text
The 35-bit slack is not theorem slack.
```

It is slack against a known non-theorem scalar calibration. The theorem-grade finite recurrence
must stay close to this scalar trace. If it loses whole q-dimensions on dominant child states, the
hybrid top seal fails.

## Corrected Child-Gap Diagnostic

The script:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_child_gap.py
```

compares candidate depth-4 child values against the scalar one-`u` ceilings from
`rfc_base_seal_budget.py`.

Important correction: this script originally built the candidate child using depth `4`, which used
the wrong replica schedule. It is now fixed to build the same depth-`5` schedule and extract the
depth-4 child value `B_4(2,u)`.

After the fix, the default `component-uniform` comparison says:

```text
u=0..8:   matches scalar / passes one-u ceilings
u>=9:     exceeds scalar one-u ceilings
```

The first failed row is:

```text
u = 9
candidate_minus_ceiling = 89.35784254 bits = 0.69810814 qdims.
```

The all-paired top child row is:

```text
u = 17
candidate_minus_ceiling = 2096.35288965 bits = 16.37775695 qdims.
```

So the finite base seal has two separate demands:

1. Low-`u` dominant scalar branches must remain almost scalar-sharp.
2. The high-common-zero tail `u>=9` must not re-enter the top moment.

## High-Common-Zero Tail Mechanism

The script:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_tail_trace.py --min-u 9 --max-u 17
```

traces the failed high-`u` rows.

The mechanism is an all-singleton/common-zero staircase inside the depth-4 child. A typical trace
uses split shapes like:

```text
p = 0,
s large,
c large,
extras = s-c small.
```

As the recurrence descends, the common-zero count saturates the child dimension. In the coarse
`component-uniform` model, once the quotient rank is zero, remaining singleton extras get zero
charge. That is why high-zero child rows become enormous compared to scalar.

Example endpoint:

```text
B_4(2,17)
  component-uniform child value:  2016.35288965 bits
  scalar child value:             -186.94093195 bits
  one-u ceiling:                   -80.00000000 bits
  excess above ceiling:           2096.35288965 bits = 16.37775695 qdims
```

The good news is that the trace also exposes a much larger missing scalar-charge reservoir. For
`u=17`, the per-level missing scalar charges include:

```text
12, 20, 32, 15 qdims
```

while only `16.38` qdims must be recovered to pass the one-`u` ceiling.

This suggests a plausible theorem target:

```text
Do not try to prove full scalar locally.
Prove a high-common-zero tail lemma that recovers enough saturated-tail charge
from the all-singleton/common-zero staircase.
```

## What Is Genuinely Proved Or Established

The following are solid current takeaways:

1. The determinant-1 fold with `T` uniform nonzero is binary-field compatible for the singleton root
   accounting.
2. Product-of-child-first-moments is invalid for shared child randomness.
3. The old scalar recurrence is not theorem-safe.
4. The all-cover lift shortcut is anti-conservative for tau-positive branches.
5. The immediate top transition cannot legally receive the tau-one full-line carry or root-kernel
   fiber cover credit.
6. The hybrid architecture is coherent: finite top boundary plus reusable lower block grammar.
7. The optimistic scalar calibration gives the correct target scale and crosses at `z=34`.
8. The top seal has very little calibration slack.
9. The corrected child-gap diagnostic identifies `u>=9` as the high-tail hazard.
10. The high-tail trace identifies the concrete all-singleton/common-zero staircase mechanism.

The following are not yet proved:

1. The finite base seal `B_5(1,34) <= 2^-80`.
2. A theorem-grade finite exact-support DP matching the scalar calibration.
3. The high-common-zero tail lemma needed to control `u>=9`.
4. The below-top canonical block grammar with all constants imported.
5. The full production near-MDS distance certificate.
6. Any systematic RFC distance certificate.

## External Review

Claude was asked for a critical second opinion. The review is recorded in:

```text
docs/rfc_distance_analysis/claude_second_opinion_2026_06_13.md
```

The key verdict was mixed:

```text
Real progress, but not yet a cracked blockade.
```

The review agreed that the hybrid architecture is mathematically coherent and not simply hiding the
illegal top-edge credit under another name. It also warned that the 35-bit budget is only slack
against the optimistic scalar calibration, not theorem slack.

The subsequent work addressed that warning by:

1. relabeling the budget as an optimistic calibration/falsification threshold;
2. adding `rfc_base_seal_budget.py`;
3. adding and then correcting `rfc_base_seal_child_gap.py`;
4. adding `rfc_base_seal_tail_trace.py`;
5. identifying the high-common-zero tail as the next concrete theorem target.

## Current Files To Read First

If resuming from scratch, read these in order:

```text
docs/rfc_distance_analysis/rfc_distance_stop_report.md
docs/rfc_distance_analysis/rfc_distance_resume_status.md
docs/rfc_distance_analysis/rfc_top_boundary_base_seal_decision.md
docs/rfc_distance_analysis/rfc_depth5_finite_flag_recurrence_target.md
docs/rfc_distance_analysis/scorecard.md
docs/rfc_distance_analysis/claude_second_opinion_2026_06_13.md
```

Then run:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_budget.py
python -B scripts/rfc_distance_analysis/rfc_base_seal_child_gap.py
python -B scripts/rfc_distance_analysis/rfc_base_seal_tail_trace.py --min-u 9 --max-u 17
```

These reproduce the latest top-boundary facts.

## Current Commit Trail

The most relevant recent commits are:

```text
99f0656 Trace RFC base-seal high-zero tail
02c6018 Audit RFC base-seal calibration budget
1ac1ae1 Adopt RFC top boundary base seal
47e544b Reject RFC top-row kernel credit
b42c52b Audit RFC tau1 carry credit
004a991 Document top RFC tau1 carry block
1b2f6cf Add RFC block-credit potential probe
e3dda06 Probe RFC canonical diagram potentials
420b028 Reset RFC proof around canonical diagrams
c49b4f7 Charge RFC support-three rank defects
```

## Recommendation If Work Resumes

Do not keep adding generic block credits first. The top boundary is the gating uncertainty.

The next concrete proof task should be:

```text
Formulate and test a high-common-zero tail lemma for the depth-4 child B_4(2,u), u>=9.
```

The lemma should target the all-singleton/common-zero staircase exposed by
`rfc_base_seal_tail_trace.py`. It only needs to recover enough charge to push each high-`u` row
below its scalar one-`u` ceiling, not to recover the full optimistic scalar recurrence.

Useful success criterion:

```text
For all u>=9, theorem-grade child_bound(u) <= child_plus_loss_ceiling_log2(u)
```

using the ceilings emitted by:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_budget.py
```

If this can be done with honest finite constants, keep the hybrid route and then return to the
below-top block grammar.

If it cannot be done, pivot back to explicit row-by-row/top-specific analysis. The finite base seal
would then be too brittle to serve as the clean boundary theorem.
