# RFC Distance Resume Status

Scope: original non-systematic RFC distance certificate work. Systematic RFC notes exist in this
folder, but the active proof push is original/non-systematic.

## Current Goal

Prove a near-MDS first-moment distance certificate for the original RFC, targeting:

```text
c = 8
k = 2048
q = 2^128
lambda = 80
target excess e = 71
fallback excess e = 72
```

The intended certificate statement is:

```text
B_d(1, k+e) <= 2^-lambda
```

which implies no nonzero codeword has `k+e` or more zeros, hence:

```text
distance >= N - (k+e) + 1.
```

Do not claim exact MDS.

## Main Proof Direction

The active route is a finite-replica / flag first moment, not fixed-set MDS and not the old scalar
span recurrence.

The recurrence tracks child flags created by singleton coordinates:

```text
L = pi(K) <= V = pi(W),
z_V = p + s - a,
z_L = p + s.
```

This is the state that keeps invisible kernel directions from passing through singleton blocks for
free.

## Important Corrections

1. Product-of-first-moments is invalid when two child events share the same child-code randomness.
   This broke the old `F_child(1,z+1)^2` split for the decomposable tau-two row.

2. The old tau-two generic/component endpoint shortcut is false as a theorem. The local tau-two
   object must use layer codimensions:

   ```text
   theta_2(A) = max_h(2h - 4 - gamma_h(A)).
   ```

3. The connected `a=5, delta=3, comp=1, g=1` row is real. Its first-drop layer has:

   ```text
   h=2,
   gamma_2 >= 1,
   first-drop exponent = -1.
   ```

   This is one q-dimension heavier than the old shortcut predicted. The full row conclusion
   `theta_2=-1` still depends on controlling the `h=delta=3` full-kernel layer by the connected
   component endpoint, or proving that endpoint directly in the `delta=3` case.

4. The in-repo encoder is not obviously using the paper's `T'=-T` pair matrix. The local encoder
   in `crates/cfri/src/backend/basefold.rs` uses:

   ```text
   left + T * right,
   left + (T+1) * right,
   ```

   whose pair matrix has determinant `1`. The remaining construction constant is the exact root
   distribution: the proof notes often assume `T` uniform nonzero, while the code path appears to
   sample field elements directly. This should be normalized in the theorem statement.

   Update: `rfc_distance_certificate_theorem.md` now states the determinant-1 nonzero-root
   normalization. For `T in F^*`, each singleton root map is injective and costs at most
   `(q-1)^-1 = q^-1 * q/(q-1)`. This is binary-field compatible and fits inside the finite
   constants bucket for the target `q=2^128`.

## Current Artifacts And Status

Local / recurrence pieces that now have usable notes:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
  Status: conditional local proof.
  Proves gamma_2 >= 1 for the g=1 first-drop layer. The full theta_2=-1 conclusion still uses
  the connected full-kernel endpoint for h=3.

docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
  Status: proved local row.
  Direct proof for |A|=3, delta=2, comp=1, giving theta_2=-2.

docs/rfc_distance_analysis/rfc_tau2_incidence_framing_lemma.md
  Status: proof skeleton / conditional lemma.
  Exact-support Grassmann cap and full-cover incidence framing.

docs/rfc_distance_analysis/rfc_theta_minus_one_isolation_lemma.md
  Status: proof skeleton.
  Outer-branch theta_2=-1 chains burn zero budget.

docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
  Status: recurrence contract.
  Kernel-branch theta_2=-1 chains must use nested flags, not independent moments.

docs/rfc_distance_analysis/rfc_theta_minus_one_truncation_status.md
  Status: diagnostic.
  Depth-6/7 diagnostics show max consecutive best-transition theta_2=-1 chain length 1.

docs/rfc_distance_analysis/rfc_depth5_rank_pattern_audit.md
  Status: audit / correction.
  The optimistic depth-5 scalar rank-pattern recurrence crosses at z=34, but the scalar
  `q^{-r|E|}` local charge is false as a theorem for low-visible-rank child blocks. The base-seal
  route now requires a finite exact-support flag recurrence.

docs/rfc_distance_analysis/rfc_depth5_finite_flag_recurrence_target.md
  Status: theorem/DP target.
  States the finite two-layer flag recurrence needed for the base seal, including tau=0/1/2
  branches and the depth-4 `B_4(2,u)` state table for `0<=u<=17`.

docs/rfc_distance_analysis/rfc_exact_support_quotient_state.md
  Status: canonical current theorem target.
  Defines the exact-support quotient state needed after the nested diagnostics. It separates the
  proof-safe subcases from diagnostic-only shortcuts: collapsed-active rerouting, upper-visible
  nested quotient counting, nested parent-subspace counting, consumed-kernel containment, and the
  remaining small-support tau-two / recursive tau-one quotient blockers.

docs/rfc_distance_analysis/rfc_support_two_tau2_quotient_frame_lemma.md
  Status: local theorem target.
  Isolates the decomposable `|A|=2,delta=2,comp=2` quotient-frame row now exposed by the decoded
  structural trace. The target child object is a diamond
  `V >= M_1,M_2 >= L`, not a product of child line moments. The first diagnostic
  `rfc_quotient_diamond_diagnostic.py` lowers the level-3 `(4,7)>=(2,8)` stress pair sum from
  `303.43723477` to `61.00503386` bits in the child-only mode.

docs/rfc_distance_analysis/rfc_covering_flag_lift_lemma.md
  Status: theorem target.
  States the covering/projectivization lemma suggested by the cover-lift diagnostics. This is now
  the key proof step for making the depth-5 base seal theorem-grade. Important correction:
  all-cover lift removal is anti-conservative for tau-positive branches because quotient
  lines/planes are event data. Tau-zero duplicate-lift covering is safe; tau-positive branches need
  quotient-incidence/fiber accounting. The current sharpened target is the unconsumed-kernel
  container cover: after the child container and canonical quotient/root datum are fixed, duplicate
  choices of `K_parent <= L+L` may be covered once; if a later profile consumes a hidden subspace
  inside `K_parent`, that subspace must be carried or charged.

docs/rfc_distance_analysis/rfc_depth5_flag_checkpoint_trace.md
  Status: diagnostic.
  Records the corrected depth-5 traces. Safe tau-zero covering leaves z=34 far too large and moves
  the dominant path into tau-one quotient-incidence chains. Kernel-lift-only covering has no
  crossing effect.

docs/rfc_distance_analysis/rfc_marked_plane_state_recurrence.md
  Status: active finite-diagram recurrence target.
  Records the carried-flag and marked-plane route. The local q+1 two-lines-in-plane brick is real
  and saves at level 2, but a naive level-3 sum over flag-state expansion choices is worse than the
  coarse table. Update: `rfc_pair_flag_table_recurrence.py` now builds pair-enumerated tables
  through level 2 and reports the depth-5-pruned level-3 stress state `(4,7)>=(2,8)` at
  `810.94626976` bits. Full unpruned level-3 pair-table construction timed out, so the next
  implementation blocker is a sparse/demand-driven pair table. Sparse demand mode now runs through
  depth 5 and moves the crossing from `z=137` to `z=133`. The new trace diagnostics show that the
  current `z=34` residual has zero scalar log-sum overhead and that the selected level-3 flag
  `(4,7)>=(2,8)` is still using the improved coarse outer-first bound `810.94626976`, because the
  proof-shaped pair sum is worse at `943.43723477`. Update: scalar collapsed-active filtering plus
  nested quotient/subspace/consumed-kernel diagnostics lower the level-3 stress state to
  `303.43723477` and full depth-5 `z=34` to `1232.88847175`; adding scalar kernel-lift cover gives
  `924.69069031`. The remaining trace is now a tau-two/tau-one quotient-incidence chain, not the
  old pair-table row alone.

docs/rfc_distance_analysis/rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8.csv
  Status: current obstruction classifier.
  The level-3 bad pair mass is concentrated in one tau-one high-lift outer witness:
  `p=3,s=1,a=1,tau=1,child=(4,4),z=3,charge=1,lift=19`, split as `kernel_lift=15` and
  `quotient_lift=4`. The top 12 pair products already equal the displayed truncated pair sum.

docs/rfc_distance_analysis/rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8_kernel_cover.csv
  Status: current best closure diagnostic.
  Keeps child table values fixed and subtracts only tau-positive kernel-lift factors. The level-3
  pair sum drops to `940.85227227` bits against the same `1187.19455102` bit coarse baseline,
  giving `1.92454905` q-dimensions of slack.
```

The external Fable audit was useful and found the product-of-first-moments bug. Its record is:

```text
docs/rfc_distance_analysis/rfc_fable_audit_2026_06_10.md
```

## Current Blockers

The live blockers are now narrow:

```text
1. Exact-support quotient-incidence state. The old level-3 carried flag `(4,7)>=(2,8)` is no
   longer the only active blocker. Scalar collapsed-active filtering changes its coarse baseline to
   `549.21247145`, and nested quotient/subspace/consumed-kernel diagnostics give table value
   `303.43723477`. End-to-end this lowers `z=34` from `1740.39750674` to `1232.88847175`.
   Adding scalar kernel-lift cover lowers it further to `924.69069031`, but crossing remains
   `z=133`. The theorem target is now a finite exact-support quotient state that proves:
   collapsed-active rerouting, nested quotient data inside outer quotient data, lower subspaces
   inside upper parent subspaces, and consumed-kernel containment for lower tau-zero rows. Quotient
   line/plane incidence must remain counted.
2. Sparse pair-enumerated table recurrence. The bounded diagnostic
   `rfc_pair_flag_table_recurrence.py --depth 5 --stop-level 3 --proof-shaped --term-limit 300
   --report-flag-state 4,7,2,8 --last-level-report-only` gives `810.94626976` bits for the
   level-3 stress flag after level-2 pair-table improvements. A full unpruned level-3 pair table
   timed out. The next implementation should compute only states demanded by the final trace and
   their recursive child flags. Update: `--demand-next-level` now runs end-to-end through depth 5,
   building 599 demanded level-2 table entries and 2076 demanded level-3 entries, and improves the
   crossing to `z=133`. At the production floor it reports `final_span_1_z_report,34,1740.39750674`,
   still `14.22185552` q-dimensions above the `2^-80` target. This is useful but not enough; the
   next recurrence needs a richer demanded diagram state or a joint local theorem. The latest
   structural modes show table plumbing can propagate real savings, but after the level-3 stress
   row improves the dominant path moves to scalar tau-two and tau-one quotient-incidence rows. More
   sparse plumbing alone is still not expected to close the gap.
3. Prove/finalize the shared-randomness-safe multi-layer flag transition theorem. The target note
   covers the joint marked-line/frame recurrence for the decomposable |A|=2,delta=2,comp=2 row,
   with the local marked-component certificate and uniform `(q+1)` frame-completion count written.
4. Kernel-branch nested-flag truncation:
   prove length three is enough, or prove length-four theta_2=-1 chains are dominated.
5. Direct proof of the delta=3 connected full-kernel endpoint used by the g=1 row, or a general
   component/full-kernel theorem with constants.
6. Higher-drop tau-two layers beyond the g=1 first-drop case.
7. Finite constants: marked-line root-fiber constants, exact-support inversion, split counts, and
   log-sum/state-count overhead. The determinant-1 nonzero-root normalization is now stated
   separately and is no longer a q-dimensional blocker at the target field size.
```

The best current next proof step is item 1, with the correction above:

Use `rfc_exact_support_quotient_state.md` as the canonical proof contract and turn its diagnostic
subcases into theorem statements:

```text
1. scalar collapsed-active exact-support rerouting;
2. nested quotient/subspace counting for W_inner <= W_outer;
3. consumed-kernel containment when a lower tau-zero layer sits inside an upper kernel;
4. small-support tau-two quotient-plane incidence;
5. recursive tau-one quotient-chain state.
```

The current best non-tau2-cover diagnostic still leaves
`(924.69069031 + 80) / 128 = 7.849145` q-dimensions, and the anti-conservative tau-two-cover
ceiling leaves `(567.15416718 + 80) / 128 = 5.055892` q-dimensions. So the next proof step must
handle both tau-two and tau-one quotient incidence; solving only the old level-3 pair row is not
enough.

Latest trace-budget refinement: `--trace-table-state` now decodes the residual q-dimensional
budget after structural covers. In the level-3 `(4,7)>=(2,8)` stress state, the top nested row has:

```text
outer adjusted lift-minus-charge: 8
inner adjusted lift-minus-charge: 2
remaining quotient lifts: outer 8, inner 1
```

So the nested tau-one quotient line is already mostly handled. The next local blocker is the outer
support-two tau-two quotient-frame placement, not another kernel cover or independent tau-one line
count.

Update: the quotient-diamond diagnostic now confirms a proof-shaped local route for that blocker.
Keeping quotient incidence counted but routing the decomposable support-two row through the joint
child diamond changes the level-3 stress pair sum:

```text
current pair sum:       303.43723477
child-only diamond:      61.00503386
```

The stronger sensitivity that also deletes the outer quotient lift reaches only `50.66146631` in
aggregate because non-candidate rows dominate after the child-diamond saving. So the next concrete
implementation step is to integrate the child-only diamond query into the demanded pair table and
then rerun the full depth-5 `z=34` trace.

Update after integration: `--support2-diamond-mode child-only` lowers the full demanded depth-5
`z=34` value from `1232.88847175` to `1095.85967660` bits, with crossing still `z=133`. The new
dominant path is:

```text
level 5: tau=1, child (2,15)
level 4: tau=2, a=3, child (4,6), charge 6, lift 12
level 3: tau=2, a=2, child flag (4,2)>=(2,4)
```

Tracing `(4,2)>=(2,4)` shows the top row has child flag `(4,0)>=(2,4)` and zero
`support2_diamond_saving_qdim`; the lower tau-one layer forces a much stronger bottom/kernel zero
budget than the outer support-two diamond supplies. The next blocker is a nested strong-bottom
quotient/kernel interaction, not the old `(4,7)>=(2,8)` row.

After that, return to the theta_2=-1 kernel-chain truncation and the connected full-kernel local
endpoint. Those remain real blockers, but they are not the first item on the active frontier.

Small diagnostic:

```text
scripts/rfc_distance_analysis/rfc_theta_chain_normal_slice.py
```

For the observed depth-6/7 level-local inner shapes, `b=0..3` normal slices are infeasible and
`b=4..6` remain within the third `q^-9` charge. A toy near-dimension slice fails at `b=1`, so the
defect-slice routing is genuinely needed. Under the generic-rank calibration, that toy `b=1` slice
would expose rank-tail charge `12`, giving combined margin `9+12-12=9`; this is now only a target,
not a certified RFC charge.

No-cycle status: defect routing is well-founded by depth, and the accumulated potential
`9*(first-drop rows)+sum defect_charge - sum E_anc` rules out an uncharged long path once the
recursive shortened-kernel rank theorem proves the expected cost for exposed events
`R_child(D,z)=Pr[dim H(B)>=D]`.

Important correction: the raw flag moment `F_child((D,z))` is too loose for exposed shortened
ambients. The deterministic diagnostic `rfc_defect_conservation.py` finds negative slack on the
toy event `k=32,D=12,z=21` under that naive flag-count induction. Defect routing must charge the
canonical rank event; ancestor flags inside `H(B)` are counted separately by `E_anc`.
The next recurrence must also account for all-paired compression of canonical rank events, which
can be cheaper than the generic-rank benchmark.

New diagnostic:

```text
scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py
```

This optimistic `rho_h(D,z)` recurrence includes exact depth-one root-line collisions and paired
compression. It finds:

```text
h=5,k=32,D=12,z=21,p=10,s=1: rho <= 1
h=4,k=16,D=7,z=21:             rho <= 2
h=4,k=16,D=8,z=21:             rho <= 2
```

So the generic-rank defect charge is not the right estimate for paired-spine shaped witnesses. The
remaining proof question is whether the nested kernel-chain state already charges these
paired-spine rank events through local first-drop rows, or whether this is a genuine distance-loss
family.

Combined theta-chain diagnostic after adding optimistic `rho` columns to
`rfc_theta_chain_normal_slice.py`:

```text
toy child_k=32, b=1: E_anc=12, max rho=1, rho margin=-2   (unsafe)
observed child_k=8, b=4: E_anc=3, max rho=7, rho margin=13 (safe)
observed child_k=16, b=4: E_anc=3, rho=inf               (cheap event impossible)
```

So the old generic-rank rescue is false for abstract near-dimension chains, but the observed
theta-chain shapes still survive the pessimistic paired-spine diagnostic. The next proof step is an
isolation lemma excluding the near-dimension paired-spine toy from the dominant `theta_2=-1`
recurrence path, or showing it pays an additional support/fiber charge.

The cheap toy `rho` trace has now been classified:

```text
h=5: s=1, hard_theta=0
h=4: s=8, hard_theta=0, residue_if_a5=3
h=3: s=0, hard_theta=0
h=2: s=0, hard_theta=0
h=1: s=2, hard_theta=0
```

So it is not a self-feeding hard `theta_2=-1` chain. A hard connected first-drop row needs
`s=a=5`; the cheap trace either has too few singletons, all-paired compression, or singleton
residue. The next proof step is narrower: prove the normal/rho margin only for hard `s=5` rank
traces, and route non-hard paired-spine traces to residue/all-paired cases.

Refinement: constraining the same toy to hard-compatible/all-paired splits:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 --expansion 8 --dim 12 --zeros 21 \
  --allowed-singletons 0,5 --ancestor-exponent 12 --trace
```

still gives `rho<=1`, but the trace has two `s=5` hard steps before all-paired compression. The
loose survivor-envelope hard-trace margin is therefore:

```text
2*9 + 1 - 12 = 7,
```

not the one-charge margin `9+1-12=-2`. This remains a conservative stress budget. The stricter
kernel-child propagation mode:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 --expansion 8 --dim 12 --zeros 21 \
  --allowed-singletons 0,5 --hard-force-all-singletons --ancestor-exponent 12 --trace
```

forces all five singleton zeros at each hard step and gives:

```text
4*9 + 5 - 12 = 29.
```

This is the proof-relevant toy margin if the coverage lemma can derive strict kernel-child
propagation from the minimal hard recurrence state.

With the conservative tau-two visible-quotient loss:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 --expansion 8 --dim 12 --zeros 21 \
  --allowed-singletons 0,5 --hard-force-all-singletons --hard-visible-dim-loss 2 \
  --ancestor-exponent 12 --trace
```

the trace still has margin:

```text
3*9 + 0 - 12 = 15.
```

This `15` margin is the safer current coverage target.

After this turn, the hard-trace potential is stated as Lemma 4 in
`rfc_theta_minus_one_isolation_lemma.md`. The remaining proof-grade obligations are:

1. derive the hard/non-hard split from recurrence state labels, not from an imposed diagnostic flag;
2. bound log_q(state constants) below the current toy margin 7, or refine the state count;
3. prove non-hard paired-spine traces either leave the theta-chain analysis or pay explicit residue.

New refinement: the state-label split is deterministic because `A subset S`, so `a<=s`, while a
connected `theta_2=-1` first-drop row has `a>=5`. Hence `s=5` forces the minimal hard label
`a=s=5`; `s<5` is impossible for such a row; `s>5,a=5` has outer residue; and `s>5,a>5` is a
larger-support local row that must be charged separately.

Constants budget at the target scale: `log_q N = 14/128`. The toy margin `7` can absorb up to
roughly `N^64` worth of fixed polynomial state count, minus any explicit `q+1` frame factors. The
new helper `scripts/rfc_distance_analysis/rfc_state_constant_budget.py` shows that the illustrative
canonical budget `N^32 * (q+1)^2 * 52^11` costs `5.98988154` q-dimensions, leaving `1.01011846`.
The stronger padded atomic target `N^32 * (q+1)^2 * 3328^11` costs `6.50550654`, leaving
`0.49449346`. The next proof must keep state labels canonical enough to stay within that budget
or reduce the residual `N^32` pad by using exact witness/support accounting more aggressively.

Update: `rfc_theta_minus_one_isolation_lemma.md` now contains the padded canonicalization lemma.
It injects minimal hard-trace labels into
`N^32 * (2*52*32)^11 * (q+1)^2` after exact witnesses/splits are fixed. The closure is conditional
on the shortened-kernel recurrence giving the stated `rho_term` and on the profile having at most
two decomposable marked-line boundary interfaces; extra decomposable interfaces must be routed to
the joint marked-line recurrence separately.

New boundary-separation refinement: the "at most two" condition is segmentwise. A maximal
minimal-hard segment can have one marked-line interface on entry and one on exit. If another
decomposable `a=2,delta=2,comp=2` row appears internally, the atomic labels force a cut and the
row is charged by the joint marked-line state `(2,z_V),(1,z_V+1)`, not by the hard-segment
constants budget.

Rank-recurrence correction: `rfc_multilayer_flag_transition_theorem.md` now treats
`rfc_shortened_rank_recurrence.py` as a survivor envelope, not as a standalone lower bound on
`rho_h`. A child shortened-rank event can force a parent survivor, so the diagnostic gives cheap
obstruction paths. The proof-safe target is a coverage lemma: every minimal hard-segment
contribution admits a hard-compatible survivor trace, and every `s=5` survivor step in that trace
pays its own local `q^-9` first-drop charge.

Both toy traces are recorded in the theorem note. The loose survivor trace has two `s=5` steps,
terminal survivor cost `1`, `E_anc=12`, and pre-constant margin `7`. The strict
kernel-child trace has four `s=5` steps, terminal survivor cost `5`, `E_anc=12`, and margin `29`.
The conservative strict trace with visible quotient loss has margin `15`. The local coverage lemma
is now written: for a kernel-following minimal hard row, `dim K>=D-2`, `L=pi(K)` is zero on
`P union S`, and `K<=L+L`, giving `D_child>=ceil((D-2)/2)` and `z_child=p+5`. The next proof step
is to globalize this over all hard-segment states, or find a strict kernel-child-compatible trace
with smaller `9H + rho_terminal - E_anc`.

Update: `scripts/rfc_distance_analysis/rfc_theta_chain_normal_slice.py` now reports strict
hard-trace columns using the conservative kernel-child recurrence above. On the known hard toy
`child_k=32,dims=(4,3,2,1),zeros=(21,26,31),b=(1,1,1)`, the old loose-rho route still fails
with margin `-2`, while the strict hard-trace route has margin `15`. The observed `child_k=8`
and `child_k=16` local slices with `b=(4,4,4)` also report strict hard margin `15`. This upgrades
the live blocker from "understand the toy" to "prove every minimal hard segment is covered by the
strict trace, or exits through a separately charged non-hard boundary."

Correction after charged-DP audit: the strict hard potential is now optimized directly with
`hard_step_charge=9`, not added after a rho-minimizing trace is chosen. This preserved the `b=1`
margin but exposed the next real issue. For the same toy family with `b=(b,b,b)`, strict-only
margin is positive through `b=8`, zero at `b=9`, and negative from `b=10`; at `b=10` the margin is
`36-39=-3`. The boundary-only comparison column gives `9+36-39=6`, but the audit convention is
that this extra `9` cannot be used for an internal hard segment unless a disjoint boundary row is
proved. Therefore the next proof split must be:

```text
low-defect hard segment -> strict hard-trace potential;
high-defect shortened ambient -> proven boundary charge, extra rank-defect/incidence/high-kernel
                                  charge, or obstruction.
```

New high-defect checkpoint:

```text
docs/rfc_distance_analysis/rfc_high_defect_hard_segment_gap.md
```

It records that the abstract `child_k=32,zeros=(21,26,31)` toy is below the production zero floor:
from `z=2119`, all-paired compression to `child_k=32` already gives at least `34` zeros. The
reachable floor stress row `zeros=(34,39,44)` is better but still has a small real gap: strict-only
fails from `b=10`, and even boundary-plus fails from `b=14` with deficit `3`. The next proof target
is a reachability-plus-boundary/high-kernel lemma for this narrowed row.

New base-seal candidate, now qualified:

```text
docs/rfc_distance_analysis/rfc_depth5_base_seal_candidate.md
docs/rfc_distance_analysis/rfc_depth5_rank_pattern_contract.md
docs/rfc_distance_analysis/rfc_depth5_rank_pattern_audit.md
docs/rfc_distance_analysis/rfc_depth5_finite_flag_recurrence_target.md
```

The depth-5 rank-pattern calibration with `--singleton-charge replica` crosses exactly at
`z=34 = k+2` with log2 moment `-115.10435419`. Since the production zero floor at `child_k=32` is
also `34`, a finite `B_5(1,34)<=2^-80` theorem would let the global recurrence stop before the
high-defect theta-chain gap.

Audit correction: the scalar rank-pattern induction theorem is too strong. The local
`q^{-r|E|}` singleton charge fails on low-visible-rank blocks. Span-only and subspace-only
diagnostics are much too pessimistic (`crossing_z=249`), while the existing two-layer flag
checkpoint improves this to `crossing_z=137`. A diagnostic all-cover run gives `crossing_z=35`,
but that run is anti-conservative because it erases tau-positive quotient incidence. The finite
base seal is still plausible, but only after implementing/proving the exact-support
container/flag recurrence that charges:

```text
pi(K) <= pi(W),
z_pi(W) = p + s - a,
z_pi(K) = p + s.
```

The explicit recurrence contract and top-term table are in:

```text
docs/rfc_distance_analysis/rfc_depth5_rank_pattern_contract.md
```
```

## Diagnostic Script

Main checkpoint:

```text
scripts/rfc_distance_analysis/rfc_flag_span_moment.py
```

Useful commands:

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

Depth 6 result:

```text
crossing_z=342 crossing_excess=278
max consecutive theta_2=-1 chain length = 1
```

Depth 7 result:

```text
crossing_z=801 crossing_excess=673
max consecutive theta_2=-1 chain length = 1
```

These are diagnostics over the corrected best-transition checkpoint, not theorem statements.
They are also pessimistic bounds: the depth-4/6/7 checkpoint crossings scale linearly in `N` and
do not themselves exhibit the final near-MDS `e=71` behavior. The production claim still rests on
the theorem package tightening the recurrence, not on these checkpoint crossings.

## Where To Look First

Start with:

```text
docs/rfc_distance_analysis/rfc_distance_manager_board.md
docs/rfc_distance_analysis/rfc_distance_certificate_theorem.md
docs/rfc_distance_analysis/rfc_flag_recurrence_proof_obligations.md
docs/rfc_distance_analysis/rfc_theta_minus_one_truncation_status.md
```

Then use:

```text
scripts/rfc_distance_analysis/README.md
docs/rfc_distance_analysis/README.md
```

for the hierarchy of scripts and notes.
