# RFC Distance Manager Board

Scope: original non-systematic RFC unless explicitly stated otherwise.

## Current Objective

Build a credible near-MDS distance certificate for the original RFC at:

```text
c = 8
k = 2048
q = 2^128
lambda = 80
target excess e = 71
fallback excess e = 72
```

The certificate route is finite-replica/flag first moment, not fixed-set MDS and not a trivial
systematic-coordinate discount.

## Current Decision State

```text
e=70: unsafe in the current exact-support stress model.
e=71: still plausible; ideal final-shape moment is about -119.68 bits, while the best corrected
      scalar stress row is about -121.83 bits.
e=72: conservative fallback if constants or exact flag enumeration eat the e=71 margin.
```

The old seven-copy broad-family warning is no longer the live obstruction.  It used marked-core
labels; exact visible support/flag counting removes about:

```text
7 log2(1783) = 75.60063691 bits.
```

The live obstruction is now:

```text
tau=2 intermediate rank-drop layers, especially the connected
a=5, delta=3, comp=1, g=1, kappa>=2 layer.
```

## Blocker Review

Retired empirical blockers:

```text
marked-core seven-copy warning:
  exact visible-support counting removes the apparent 75.60-bit overcount.

GF(5) non-stride size-8 explosion:
  GF(65537) rank replay has no non-stride dim>=2 supports; currently classified as small-field.

complete-stride flag intersections:
  two-copy and three-copy GF(65537) checks show no V/L excess over generic intersections.

paired-spine lift:
  t=2 and resolved t=3 full-kernel gates show only finite Gaussian constants.

tau=2 local decomposable endpoint:
  closed-form local decomposable row has about two q-dimensions of slack. This does not justify
  multiplying child moments; shared child randomness is handled by the multi-layer flag theorem.

dense connected tau=2 endpoint row:
  GF(11) and GF(31) connected examples have positive raw finite-field excess but negative
  residual after explicit projective-line and nonzero-root constants for the `a=4, delta=3,
  comp=1, g=2` row.
```

Live blockers:

```text
1. Prove/finalize the formal shared-randomness-safe multi-layer flag transition theorem. The target
   note now covers both the neutral decomposable |A|=2, delta=2, comp=2 marked-line/frame row and
   the theta_2=-1 kernel-chain recurrence. The product-of-first-moments split is invalid because
   the child events share the same code randomness.
2. Prove the global finite-replica/flag recurrence with exact-support inversion, the tau-two
   exact-support Grassmann incidence cap, and polynomial factors.
3. Control possible chains of theta_2=-1 first-drop rows. The outer-branch accounting now has a
   zero-budget burn lemma; the kernel-branch route now has a three-layer nested-flag recurrence
   contract. Depth-6/7 best-transition diagnostics show max consecutive theta-chain length 1; the
   remaining proof target is a length-three truncation or length-four domination lemma.
4. Prove the delta=3 connected full-kernel/component endpoint used by the g=1 row, or mark the
   g=1 full-row theta_2=-1 conclusion as conditional until the general component endpoint is
   theorem-grade.
5. Stress higher-drop tau-two layers and mixed-fiber chain shapes beyond the retired
   complete-stride and paired-spine gates, especially tau sequences 1->2 and 2->1.
6. Produce the final theorem-driven certificate output for e=71/e=72 after constants are inserted.
   The determinant-1 nonzero-root normalization is now separated out: for `T in F^*`, singleton
   roots cost at most `q^-1 * q/(q-1)` each, so this construction constant is finite and
   binary-field compatible rather than a new q-dimensional loss.
```

Current honesty check:

```text
We are making real progress on the local obstruction list, but the certificate is not proved yet.
The main risk has shifted from local endpoint falsification to recurrence state: the tau-2 proof
needs layer codimensions, exact-support incidence caps, shared-randomness-safe multi-layer flag
states, and small-support layer accounting.  A shortcut using only `g` and `comp` is no longer a
candidate theorem.
```

## Active Lanes

### Proof-Support Lane

Owner artifact:

```text
docs/rfc_distance_analysis/rfc_flag_recurrence_proof_obligations.md
```

Mission:

```text
State the two-layer flag recurrence theorem, separate safe lemmas from open lemmas, and specify
the exact finite constants and enumerator contract.
```

Key risks:

```text
flag-lift constants,
exact-support inversion,
generic flag-intersection assumptions,
polynomial split/profile factors.
```

### Lower-Bound/Falsification Lane

Owner artifact:

```text
docs/rfc_distance_analysis/rfc_nested_kernel_attack_matrix.md
```

Mission:

```text
List the remaining obstruction families and define what exact small-depth evidence would count as
a real e=71 warning.
```

Key risks:

```text
complete-stride nested kernels,
dense tau-two generic endpoint supports,
multi-copy exact flag intersections.
```

### Implementation Lane

Owner artifacts:

```text
scripts/rfc_distance_analysis/rfc_flag_intersection_enum.py
docs/rfc_distance_analysis/rfc_flag_intersection_enum_status.md
```

Mission:

```text
Build a size-guarded exact/small-depth flag-intersection enumerator for nested flags L <= V.
```

Rules:

```text
No long benchmarks.
No concurrent benchmarks.
Only tiny smoke runs until the manager picks a run window.
```

## Integration Gates

1. Proof lane specifies the exact flag type and finite constants.
2. Lower-bound lane ranks the first exact cases to enumerate.
3. Implementation lane produces a smoke-tested enumerator.
4. Manager runs one selected exact small-depth check.
5. If no excess flag multiplicity appears, fold the flag recurrence into the certificate theorem.
6. If excess multiplicity appears, either add the missing state or move the certificate target to
   `e=72`.

## Current Next Bet

The best next bet is to formalize the exact-support incidence recurrence.  Every tau-two
exact-support quotient branch now has the uniform Grassmann charge:

```text
local_charge >= |A|.
```

External Fable audit found a serious correction: the attempted
`F_child(1, outer_zeros+1)^2` product split for the neutral `|A|=2,delta=2,comp=2` row multiplies
two first moments over the same child-code randomness and is not proof-safe. The correct target is
a joint marked-line/frame child state. The audit also found that `rfc_flag_span_moment.py
--flag-bound best` was anti-conservative because it included the same product-of-first-moments
relaxation; that has been removed from the default `best` mode.

A coarse joint marked-line diagnostic is now implemented for the `|A|=2` row:

```text
local_decomposable_row_contribution <= poly(N) * (q+1) * F_child((2,z_V), (1,z_V+1)).
```

At depth 4 this is already strong enough to move the dominant row to the connected
`|A|=3,delta=2,comp=1` endpoint. The joint marked-line recurrence is now drafted in the
multi-layer theorem note; the local marked-component certificate and uniform `(q+1)`
frame-completion count are written there. The remaining marked-line work is finite bookkeeping:
the determinant-1 root-fiber constant and exact-support inversion. The `|A|=3` endpoint itself now
has a direct `U_{2,3}` proof in:

```text
docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
```

The exact-support Grassmann cap still looks sound modulo root-injectivity constants. The `g=1`
first-drop local layer is now quantified by:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
```

which proves `gamma_2>=1` and hence first-drop exponent `-1` for the
`|A|=5,delta=3,comp=1,g=1,h=2` layer. The full-row conclusion `theta_2=-1` still depends on the
`h=delta=3` full-kernel/component endpoint. The row remains one q-dimension heavier than the old
shortcut, but it is no longer a mysterious first-drop gap. The remaining hard cases are now:

```text
1. global recurrence control for possible chains of theta_2=-1 first-drop rows;
2. direct delta=3 full-kernel/component endpoint for the g=1 row;
3. finite marked-line constants: determinant-1 root-fiber constant and exact-support inversion;
4. higher-drop local layers kappa >= g+2;
5. mixed-fiber quotient incidence when kernels persist across levels.
```

The old complete-stride enumerator target below is now historical context; those gates came back
clean.

Historical complete-stride case that was tested:

```text
depth 4,
t = 2,
tau = 1,
kappa = 1,
complete-stride extras around the known m=4, extra=4 dimension-growth class.
```

Concrete target family:

```text
B_b = {4b, 4b+1, 4b+2, 4b+3},        b in {0,1,2,3}
C_j = {j, j+4, j+8, j+12},            j in {0,1,2,3}
Omega = C_i union C_j,                0 <= i < j <= 3

expected unmarked supports = 4 * binom(4,2) = 24
expected ordered flags     = 48
```

Current implementation status:

```text
Combinatorial complete-stride gate is green:
  unmarked supports = 24
  ordered flags     = 48

The generic exact subflag enumerator works on tiny states, but the depth-4 complete-stride target
is too broad if it enumerates all Gaussian r0/r1 subflags inside the ambient zero kernels.

Next implementation gate: direct finite-field complete-stride check for all 24 supports:
  dim K_Omega = 2
  dim ell_i = dim ell_j = 1
  K_Omega = ell_i direct_sum ell_j

Direct finite-field complete-stride gate is green over GF(5):
  dim K_Omega = 2 for all 24 supports
  dim ell_i = dim ell_j = 1 for all 48 ordered flags
  direct-sum check passes for all 24 supports

Next cheap falsification gate:
  scan size-8 non-stride Omega supports for any dim K_Omega >= 2.

Status of that gate over GF(5), seed 1:
  total profiles = 51480
  complete-stride dim>=2 = 24 / 24
  non-stride dim>=2 = 8534 / 51456
  max non-stride dimension = 4
  classification shows many non-stride hits have 0 or 1 contained complete stride lines, but many
  contained kernel lines.

This is a live red flag until classified.  Possible explanations:
  small-field degeneracy,
  non-stride supports containing other exact kernel lines,
  mismatch between the targeted stride model and the saved depth-4 artifacts,
  or a real broader dimension-growth family.

Next decision gate:
  replay the classified all-Omega scan over a larger prime, starting with 65537.

Large-prime replay status:
  fast rank-only GF(65537), seed 1 scan is clean:
    total profiles = 51480
    complete-stride dim>=2 = 24 / 24
    non-stride dim>=2 = 0 / 51456
    max non-stride dimension = 1

Manager interpretation:
  GF(5) non-stride explosion is best classified as small-field degeneration for now.
  The production-relevant complete-stride story survives this gate.

Complete-stride independent-copy intersection status:
  two-copy GF(65537), seed 1 gate is clean:
    exact flags per copy = 48,48
    intersections checked = 2304
    max common_v_dim = 0
    max common_l_dim = 0
    max v_excess = 0
    max l_excess = 0

  three-copy GF(65537), seed 1 gate is clean:
    exact flags per copy = 48,48,48
    intersections checked = 110592
    max common_v_dim = 0
    max common_l_dim = 0
    max v_excess = 0
    max l_excess = 0

Manager interpretation:
  no new shared-stride/shared-flag recurrence state is needed for complete-stride flags at r<=3
  in this large-prime check.  This does not prove arbitrary r or the full recurrence.

Next primary falsification target:
  paired-spine cascades seeded by the 48 ordered complete-stride flags.
  first shape: depth 4 plus one paired lift, t in {2,3}, tau=1, kappa=1, chain length 2.

Paired-spine cascade status:
  t=2, tau=1, kappa=1 gate is effectively clean over GF(65537):
    chain_count = 13511829702574368
    canonical_chain_count = 13511829702574368
    duplicate_certificate_count = 0
    max_chain_excess_logq = 0.0000027517
    q_dimension_excess_witness_rows = 0

  Interpretation:
    the small positive excess is the finite Gaussian-binomial constant above the q-exponent,
    not an extra q-dimension.

  t=3 blocker:
    full recursive kernel dimension is t - tau = 2, but the proposed tracked kernel has
    tracked_kappa = 1.  Counting this requires a precise chain-state definition before the
    diagnostic is meaningful.

Tracked-kernel resolution:
  for t=3,tau=1, a tracked line ell <= K_full is a duplicate certificate unless it has a
  downstream constraint.  If it is a real event, the state is three-layer:
    ell <= K_full <= W
  and it must pay the full-kernel lift plus an extra line-choice/fiber factor.

Next implementation gate:
  count the t=3 full-kernel event first.  Separately report the tracked-line fiber as charged
  metadata, not as free multiplicity.

t=3 full-kernel gate status:
  GF(65537), depth 4, chain length 2:
    chain_count = 3145824
    canonical_chain_count = 3145824
    duplicate_certificate_count = 206167867488
    expected_lift_logq = 1.0000000000
    observed_chain_logq = 1.0000013758
    chain_excess_logq = 0.0000013758
    hidden_line_fiber_logq = 1.0000013758
    hidden_line_fiber_charged = 0
    q_dimension_excess_witness_rows = 0

  Interpretation:
    no q-dimension excess appears.  The tracked-line fiber is real metadata, but without a
    downstream constraint it is duplicate certificate mass, not free event multiplicity.

Tau-2 paired-spine lift gate:
  GF(65537), depth 4, chain length 2, t=2, tau=2:
    rank_S = 2
    rank_S_minus_A = 0
    delta = 2
    comp = 2
    g = 0
    expected_lift_logq = 4.0000000000
    observed_chain_logq = 4.0000013759
    chain_excess_logq = 0.0000013759
    endpoint_component_logq = -6
    endpoint_generic_logq = -4
    endpoint_bound_logq = -4
    endpoint_excess_logq = NA
    q_dimension_excess_witness_rows = 0

  Interpretation:
    paired-spine lift accounting is clean up to finite Gaussian constants.
    Remaining blocker is the local tau-2 endpoint counter, not the lift.

Tau-2 endpoint-counter status:
  Existing root-line profiler could not compute the GF(65537), size-8 complete-stride endpoint:
    checked = 1
    skipped_supports = 255
    observed endpoint rows = none

  Reason:
    the current profiler cannot directly name the target support, and exact root-line assignment
    enumeration over GF(65537) for size 8 is infeasible.

  Next required method:
    a non-enumerative or support-targeted endpoint counter for the decomposable
    delta=2, comp=2, g=0 row.

Tau-2 local decomposable endpoint resolution:
  closed-form decomposable row over GF(65537):
    support_size = 8
    delta = 2
    comp = 2
    g = 0
    endpoint_bound_logq = -4
    canonical_endpoint_event_count = 4295229444
    exact_support_event_count = 4295229444
    observed_endpoint_logq = -5.9999972483
    endpoint_excess_logq = -1.9999972483

  Interpretation:
    the local decomposable row is benign.  It has roughly two q-dimensions of slack against the
    safe endpoint bound. This is a local endpoint statement only; it does not justify multiplying
    child moments. The next endpoint target is a dense connected tau-2 profile with comp=1,
    delta=2, and g>=2.

Dense connected tau-2 endpoint target correction:
  comp=1, delta=2, g>=2 is infeasible for connected no-loop supports.
  First genuine dense connected target:
    a = 4
    delta = 3
    comp = 1
    g = 2
    endpoint_bound_logq = max(2g-4, comp+2delta-4-a) = 0

  Existing small-field evidence:
  GF(11), child_depth=2, expansion=4 sampled artifact contains this profile:
    exact_root_line_logq = 4.1649413340
    observed_endpoint_logq = 0.1649413340
    endpoint_excess_logq = 0.1649413340

  Interpretation:
    positive finite-field excess over the asymptotic endpoint bound, but not a full q-dimension.
    Proof analysis says this is compatible with finite projective/Gaussian constants:
      (q+1)^4 / q^4 contributes about 0.145 log_q at q=11.
    Existing artifact is summary-only, so larger-field targeted replay needs saved support columns
    or a support-targeted profiler API.

  Next implementation requirement:
    endpoint tooling must emit finite-constant decomposition:
      projective_line_factor_logq,
      exceptional_kappa3_count,
      nonzero_root_normalization_logq,
      residual_endpoint_excess_logq.

Dense connected endpoint tooling status:
  rfc_root_line_kernel_profile.py support-detail output now includes:
    endpoint_excess_logq
    projective_line_factor_logq
    nonzero_root_normalization_logq
    exceptional_kappa3_count
    residual_endpoint_excess_logq

  Future larger-field replay requires saving support details via --support-csv so exact S/A
  columns are available for targeted replay.

Dense connected support capture:
  GF(11), child_depth=2, expansion=4, size=5, tau=2 captured four target rows with:
    a = 4
    delta = 3
    comp = 1
    g = 2

  Sample captured supports:
    S=0:4:7:11:14   A=4:7:11:14
    S=0:7:9:12:13   A=0:9:12:13
    S=0:7:11:14:15  A=0:11:14:15
    S=1:4:9:11:12   A=4:9:11:12

  Captured endpoint columns:
    exact_root_line_count = 21744
    exact_root_line_logq = 4.164941334045784
    endpoint_excess_logq = 0.16494133404578371
    projective_line_factor_logq = 0.14514625050840735
    nonzero_root_normalization_logq = 0.15898972884349005
    residual_endpoint_excess_logq = -0.1391946453061137

  Interpretation:
    after explicit finite constants, the GF(11) captured rows are benign.

  Next replay gate:
    add support-targeted mode (--columns S --support-columns A) and replay one captured support
    at a larger prime.

Support-targeted replay status:
  rfc_root_line_kernel_profile.py now supports:
    --columns <S>
    --support-columns <A>

  First larger-field replay:
    prime = 31
    S = 0:4:7:11:14
    A = 4:7:11:14
    rank_S = 4
    rank_S_minus_A = 1
    delta = 3
    comp = 2
    g = 2
    exact_root_line_count = 1965056
    endpoint_excess_logq = 0.21988506034045496
    residual_endpoint_excess_logq = 0.14470886742595956

  Interpretation:
    this does not replay the captured dense-connected profile, because comp changed from 1 to 2.
    The residual is not evidence against the comp=1 dense-connected target.

  Next decision:
    either search for a matching a=4, delta=3, comp=1, g=2 profile over a larger field,
    or define a field-lifted replay that preserves the original GF(11) local fold data.

GF31 fresh-profile search:
  command searched GF31, child_depth=2, expansion=4, size=5, tau=2, samples=2.
  target rows found:
    a=4, delta=3, comp=1, g=2: 0

  closest rows:
    a=4, delta=3, g=2, comp=4
    residual_endpoint_excess_logq = -2.0306156429112927
    kernel_profile = 2:1048544;3:32

  Interpretation:
    blind small sampling mostly hits disconnected rows.  Need connectivity-biased support
    generation before spending more endpoint budget.

Connectivity-filter tooling:
  rfc_root_line_kernel_profile.py now supports discovery filters:
    --require-root-count
    --require-delta
    --require-support-components
    --require-generic-kernel-dim

  GF31 seed2 filtered run:
    samples = 10
    filtered_supports = 310
    target summary rows = 0

  Interpretation:
    filters work, but the dense connected target is rare under blind singleton-set sampling.
    Next improvement should bias or enumerate candidate S/A structures, not just filter after
    random S selection.

Connected-support discovery result:
  GF31, seed2, connected-support discovery found one target row:
    S = 2:3:7:10:14
    A = 3:7:10:14
    delta = 3
    comp = 1
    g = 2
    kernel_profile = 2:1048544;3:32
    exact_root_line_count = 1076224
    endpoint_excess_logq = 0.04456055000320269
    residual_endpoint_excess_logq = -0.030615642911292706

  Interpretation:
    this is the desired dense connected profile over GF31, and the finite-constant-adjusted
    residual is negative.  No endpoint exponent failure is visible on this profile.

  Remaining cleanup:
    parse kernel_profile into exceptional_kappa3_count=32 and replay this exact support as a
    stable regression row.

Regression replay:
  GF31, seed2, targeted support replay:
    S = 2:3:7:10:14
    A = 3:7:10:14
    rank_S = 4
    rank_S_minus_A = 1
    support_rank = 3
    delta = 3
    comp = 1
    g = 2
    exceptional_kappa3_count = 32
    kernel_profile = 2:1048544;3:32
    exact_root_line_count = 1076224
    endpoint_excess_logq = 0.04456055000320269
    residual_endpoint_excess_logq = -0.030615642911292706

  Interpretation:
    targeted replay confirms the dense connected GF31 profile is benign after finite constants.

Tau-2 layer-codimension correction:
  The old generic/component endpoint max is false as a general local theorem.
  Existing GF11 exact-support row:
    S = A = 0:8:10:14:15
    a = 5
    delta = 3
    comp = 1
    g = 1
    old endpoint_root_weight_logq = -2
    exact_root_line_count = 21744
    exact_root_line_logq = 4.164941334045784
    observed root-weight exponent = -0.8350586659542163
    old endpoint_excess_logq = 1.1649413340457837
    old residual_endpoint_excess_logq = 0.784771359855912
    kernel_profile = 1:226512;2:22308;3:12
    dominant_contained_layer_h = 2
    dominant_contained_layer_root_weight_logq = -0.8243795085243608
    dominant_contained_layer_codim_logq = 0.8241552372678544

  Interpretation:
    this is not a finite-constant problem.  It is the codimension-one-ish first contributing
    layer X_2={ell: dim K_A(ell)>=2}.  The local theorem must use:
      theta_2(A)=max_h(2h-4-gamma_h(A))
    rather than only max(2g-4, comp+2delta-4-a).

  Broader generic-layer consequence:
    for g=0, the generic h=2 layer gives theta_2=-4;
    for g=1, the generic h=2 layer gives theta_2=-1;
    for g>=2 with delta>g, the first-drop h=g+1 layer can give theta_2=2g-3.
    The full-kernel layer still uses gamma_delta=a-comp from diagonal endomorphisms.

  Calibration script update:
    rfc_subspace_span_moment.py, rfc_replica_span_moment.py, and rfc_flag_span_moment.py now
    support --singleton-charge endpoint-tau2-layer.

  Small-depth flag-checkpoint calibration, q_log2=128, c=8:
    depth 4:
      old endpoint-tau2 crossing_excess = 49
      layer endpoint-tau2-layer crossing_excess = 51
    depth 5:
      old endpoint-tau2 crossing_excess = 97
      layer endpoint-tau2-layer crossing_excess = 107

  Interpretation:
    the layer charge is visible and can amplify in the coarse flag checkpoint.  The absolute
    crossings are too pessimistic to use as certificates, but the delta says the global proof must
    explicitly charge theta_2 layers through kernel-zero propagation.

  Enriched dominant trace:
    depth 5, z=138, endpoint-tau2-layer:
      level=4, span=2, z=69
      p=5, s=59, a=59, tau=2
      outer_span=4, inner_span=0
      outer_zeros=5, inner_zeros=64
      local_charge=12
      delta=3, comp=1, g=0
      theta=-4, dominant_h=2, gamma=4
      lift_qdim=12

  Interpretation:
    local tau-2 layer charge can exactly cancel the quotient lift.  The next proof obligation is
    an incidence/framing recurrence that counts the parent quotient and the root-line layer
    together; scalar multiplication of local endpoint count and generic lift is too coarse.

  Incidence-framing calibration:
    new note:
      docs/rfc_distance_analysis/rfc_tau2_incidence_framing_lemma.md
    new checkpoint mode:
      rfc_flag_span_moment.py --singleton-charge endpoint-tau2-layer-incidence
    depth 4 result after adding the full-cover exact-support Grassmann cap:
      crossing_excess = 49
      trace at z=65/e=49 moves to a partial-cover branch:
        child length at that level = 32
        p=2, s=29, a=29, so p+s=31 rather than 32
        local_charge=12, lift_qdim=8, net q^-4
    depth 4 result after extending the exact-support Grassmann cap to all tau-two quotient
    branches:
      crossing_excess = 49
      z=65/e=49 aggregate drops from -199.33 bits to -351.48 bits
      new dominant tau-two row:
        child length at that level = 32
        p=16, s=2, a=2
        local_charge=4, lift_qdim=4, theta_2=0
    safe rerun after removing product-of-first-moments from --flag-bound best:
      crossing_excess = 49
      z=65/e=49 log2_vector_moment = -351.48 bits
      dominant tau-two row remains:
        p=16, s=2, a=2
        local_charge=4, lift_qdim=4, theta_2=0
      interpretation before the marked-line diagnostic:
        the a=2 boundary row remains the live small-depth obstruction if no joint marked-line
        state is used.
    depth 4 result from the invalid squared-first-moment product split:
      crossing_excess = 49
      z=65/e=49 aggregate drops again to -596.36 bits
      new dominant tau-two row:
        child length at that level = 32
        p=16, s=3, a=3
        delta=2, comp=1, g=1
        local_charge=6, lift_qdim=4, theta_2=-2
      audit status:
        anti-conservative; do not use this result for proof guidance until replaced by a joint
        marked-line/frame state.
    depth 4 result after replacing the invalid split by the safe marked-line child bound:
      bound used for the a=2 product row:
        (q+1) * F_child((2, outer_zeros), (1, outer_zeros+1))
      crossing_excess = 49
      z=65/e=49 log2_vector_moment = -596.36 bits
      dominant tau-two row:
        p=16, s=3, a=3
        delta=2, comp=1, g=1
        local_charge=6, lift_qdim=4, theta_2=-2
    depth 5 status:
      current unpruned checkpoint mode timed out after 240 seconds, so the proof route should
      formalize the quotient-incidence lemma directly rather than waiting on this toy model.

  Current proof target:
    for fixed child flag L<=V, count the parent quotient W/K inside Q+Q with Q=V/L and compute
    theta_2(Q,A), not theta_2(full child code,A) times an independent Gaussian quotient lift.

  Exact-support incidence lemma status:
    docs/rfc_distance_analysis/rfc_tau2_incidence_framing_lemma.md now proves the uniform cap:
      local_charge >= |A|
    for every fixed tau-two quotient branch. In the full-cover K=L=0 subcase this specializes to:
      E_{V,A}(2) q^-a <= Gamma_q q^(4 dim(V)-4-a)
    The cap can be combined with theta_2(V,A) or theta_2(Q|A,A) when the layer theorem is
    available.
    The a=2,delta=2,comp=2,K=L=0 row is decomposable, but the squared first-moment split is
    invalid. The current safe route is a joint marked-line/frame state:
      local_decomposable_row_contribution <= poly(N) * (q+1) * F_child((2,z_V), (1,z_V+1))

  Remaining incidence generalization:
    sharpen L<=V with Q=V/L and partial-cover singleton blocks.  The expected transition is:
      kernel_lift_qdim + fiber_qdim(Q,A) + theta_tau(Q,A)
    where fiber_qdim counts quotient directions invisible on the singleton support.
```

Counter-signal:

```text
observed flag intersection dimension exceeds the generic flag dimension by at least one q-dimension,
or the production-scaled corrected e=71 bad-count exceeds -80 bits.
```

Current ranked obstruction families:

```text
1. recurrence audit for chains of theta_2=-1 first-drop rows, now focused on the normal/defect
   inequality after the deterministic shortened-ambient ancestor bound. Current diagnostic:
   observed level-local inner shapes are safe/infeasible for b<=6, but a toy near-dimension slice
   fails normal truncation at b=1 and is rescued only after exposing the enlarged shortened ambient.
   Correction: the exposed object must be the canonical shortened-kernel rank event
   `R_child(D,z)=Pr[dim H(B)>=D]`, not the raw flag moment `F_child((D,z))`; the latter has negative
   one-step slack in `rfc_defect_conservation.py`. Next proof step: build the RFC recurrence for
   this rank event, including all-paired compression, and prove it supplies enough codimension for
   the exposed defect slices. Current obstruction: `rfc_shortened_rank_recurrence.py` finds
   paired-spine rank events with optimistic costs `rho<=1` for the constrained
   `h=5,D=12,z=21,p=10,s=1` toy and `rho<=2` for observed-style `h=4,z=21,D=7,8` defects.
   Combined theta-chain check: the toy `child_k=32,b=1` has rho margin `-2`, while observed
   `child_k=8,b=4` has rho margin `13` and observed `child_k=16,b=4` has no cheap rho event.
   Next proof step: isolate/exclude the near-dimension paired-spine toy from dominant
   `theta_2=-1` paths, or find the missing support/fiber charge it pays. Current refinement:
   the cheap toy trace has no hard `s=5` step (`s=1`, then `s=8` with residue 3, then all-paired),
   so the live theorem can split hard theta-compatible rank traces from non-hard paired-spine
   residue/all-paired traces. New refinement: if the toy is constrained to hard/all-paired splits
   (`--allowed-singletons 0,5`), it has two hard `s=5` steps and margin `2*9+1-12=7`, so the
   corrected potential should count one local charge per hard rank-trace step. State-label split:
   `A subset S` and `a>=5`, so `s=5` forces the minimal hard label; larger `s,a` rows must be
   charged as larger-support local profiles, not collapsed into the minimal hard case. Constants
   checkpoint: a canonical budget `N^32 * (q+1)^2 * 52^11` costs `5.98988154` q-dimensions at the
   target scale, leaving about `1.01` q-dimensions inside the toy hard-trace margin. A padded
   atomic target `N^32 * (q+1)^2 * 3328^11` still leaves about `0.49`, but only if the canonical
   label alphabet is complete and the root/frame cost is globally at most two `q+1` factors.
   Coarser duplicate-certificate grouped states can still break the margin. Current refinement:
   the padded canonicalization lemma is now written; its remaining conditions are the stated
   `rho_term` rank recurrence and at most two decomposable marked-line boundary interfaces per
   maximal hard segment. Extra decomposable interfaces deterministically cut the segment and are
   charged by the joint marked-line recurrence. Important correction: the shortened-rank script is
   a survivor envelope, not a standalone lower bound on `rho_h`; the remaining theorem obligation
   is coverage of every minimal hard segment by a hard-compatible survivor trace whose `s=5` steps
   pay local `q^-9` charges. The loose recorded toy trace has pre-constant margin `7`; the stricter
   kernel-child propagation trace has margin `29`, and the conservative visible-quotient-loss
   strict trace has margin `15`. Local coverage is now reduced to the deterministic inequalities
   `dim K>=D-2`, `L zero on P union S`, and `K<=L+L`. The normal-slice checker now reports this
   strict hard-trace potential directly: the hard toy has loose-rho margin `-2` but strict hard
   margin `15`, and the observed child-k 8/16 slices with `b=4` also have strict hard margin `15`.
   Charged-DP audit preserved the b=1 margin but found the strict-only boundary: for the toy
   family with `b=(b,b,b)`, strict margin is positive through `b=8`, zero at `b=9`, and negative
   from `b=10` (`36-39=-3`). Audit convention: the truncation's local `charge=9` is not separate
   for an internal hard segment; a boundary-plus margin can be used only after proving a disjoint
   boundary row. Next attack is now a two-regime proof: low-defect hard segments use strict trace
   potential; high-defect shortened ambients must use a proven boundary charge or expose extra
   rank-defect, incidence, or high-kernel charge;
2. higher-drop local layers kappa >= g+2;
3. mixed-fiber chain shapes where the intermediate layer persists across levels;
4. mixed chain shapes tau=1->2 and tau=2->1 after the exact-support incidence cap is fixed;
5. finite marked-line root-fiber/exact-support constants.
```

The chain target is split out in:

```text
docs/rfc_distance_analysis/rfc_theta_minus_one_chain_recurrence_target.md
```
