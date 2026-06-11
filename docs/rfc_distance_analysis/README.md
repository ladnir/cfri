# RFC Distance Analysis

This folder contains research notes and generated artifacts for the original non-systematic RFC
distance-certificate push. Systematic notes are retained as deferred/reference material.

Useful entry points:

```text
rfc_distance_manager_board.md                 current manager board and lane ownership
rfc_distance_resume_status.md                 concise resume note for restarting the proof thread
scorecard.md                                  success/failure scorecard for proof ideas
rfc_fable_audit_2026_06_10.md                 external Fable audit and corrections
claude_report.md                              follow-up external review of the resume status
rfc_claude_report_response.md                 accepted/corrected actions from Claude report
rfc_distance_certificate_theorem.md           canonical conditional theorem and certificate statement
rfc_distance_certificate_c8_k2048_q128.json   generated default original c=8/k=2048/q=2^128 certificate
rfc_distance_certificate_c8_k2048_q128.csv    same certificate in CSV form
rfc_original_falsification_status.md          active bad-family/falsification status
rfc_lower_bound_falsification_multicopy.md    multi-copy broad-family falsification stress
rfc_upper_bound_response_to_multicopy.md      exact-support response to seven-copy stress
rfc_upper_bound_flag_lemma_target.md          exact-support and flag-lift lemma targets
rfc_multilayer_flag_transition_theorem.md     shared flag transition target for marked-line/kernel rows
rfc_flag_recurrence_proof_obligations.md      manager-facing two-layer proof map
rfc_lower_bound_nested_kernel_cascade.md      nested-kernel falsification status
rfc_nested_kernel_attack_matrix.md            manager-facing obstruction attack matrix
rfc_flag_intersection_enum_status.md          exact flag-intersection enumerator status
rfc_complete_stride_target_spec.md            depth-4 complete-stride target family
rfc_targeted_flag_generator_proof_contract.md proof contract for targeted flag generator
rfc_complete_stride_intersection_proof_contract.md proof contract for complete-stride intersections
rfc_complete_stride_intersection_proof_status.md proof status after clean two-copy gate
rfc_complete_stride_intersection_attack.md    falsification criteria for complete-stride intersections
rfc_post_complete_stride_attack_plan.md       next attack plan after clean complete-stride gates
rfc_paired_spine_cascade_proof_contract.md    proof contract for paired-spine cascade gate
rfc_paired_spine_cascade_attack.md            falsification criteria for paired-spine cascades
rfc_tracked_kernel_chain_state.md             proof contract for tracked kernel lines
rfc_tracked_kernel_chain_attack.md            falsification criteria for tracked kernel lines
rfc_tau2_paired_spine_proof_contract.md       proof contract for tau-2 paired-spine gate
rfc_tau2_paired_spine_attack.md               falsification criteria for tau-2 paired-spine gate
rfc_tau2_endpoint_counter_contract.md         proof contract for tau-2 endpoint counter
rfc_tau2_endpoint_counter_attack.md           falsification criteria for tau-2 endpoint counter
rfc_tau2_endpoint_remaining_risks.md          remaining endpoint risks after product row
rfc_tau2_product_endpoint_formula.md          product-row closed-form tau-2 endpoint formula
rfc_tau2_dense_connected_endpoint_target.md   dense connected tau-2 endpoint target
rfc_tau2_dense_connected_endpoint_attack.md   falsification criteria for dense connected endpoint
rfc_tau2_dense_connected_formula_target.md    finite-constant formula for dense connected endpoint
rfc_tau2_dense_connected_gf11_signal.md       GF(11) dense endpoint signal interpretation
rfc_tau2_incidence_framing_lemma.md           quotient-incidence and exact-support Grassmann cap
rfc_dense_endpoint_support_capture_attack.md  support-capture attack criteria for dense endpoint
rfc_dense_endpoint_replay_contract.md         larger-field dense endpoint replay contract
rfc_dense_endpoint_replay_field_model.md      field-model choice for endpoint replay
rfc_dense_endpoint_replay_fork_attack.md      falsification choice for endpoint replay fork
rfc_dense_endpoint_connectivity_sampler_contract.md proof contract for connectivity-biased sampling
rfc_dense_endpoint_connectivity_sampler_attack.md attack criteria for connectivity-biased sampling
rfc_dense_endpoint_connected_generator_contract.md proof contract for connected-support discovery
rfc_dense_endpoint_connected_generator_attack.md attack criteria for connected-support discovery
systematic_rfc_distance_proof.md              deferred systematic construction/reference note
rfc_systematic_c8_depth11_certificate_statement.md deferred systematic certificate snapshot
rfc_systematic_distance_proof_status.md       deferred systematic proof-status snapshot
rfc_defect_coupled_virtual_core_theorem.md    current virtual-core theorem target
rfc_global_rank_budget_pivot.md               current global-rank-budget pivot
rfc_rank_active_cancellation_independence.md  current cancellation-independence target
rfc_original_mds_certificate.md               original/non-systematic MDS-style result notes
rfc_mds_feasibility.md                        sampled-MDS feasibility and first-moment scale
rfc_near_mds_first_moment_proof.md            near-MDS theorem target and rank-tail lemma
rfc_replica_root_compatibility.md             local rank-one singleton condition for replica moments
rfc_exterior_component_codimension.md         component/full-rank endpoint, not the whole tau-2 theorem
rfc_replica_span_state.md                     component recurrence failure and replica-span fix
rfc_flag_recurrence_checkpoint.md             two-layer flag recurrence checkpoint
rfc_visible_span_local_state.md               local visible-span/root-support/support-kernel state
rfc_root_line_kernel_state.md                 root-line kernel profile and exact-support inversion
rfc_tau2_kernel_profile_target.md             tau-2 intermediate kappa-profile target
rfc_h1_kernel_drop_bound.md                   first root-line rank-drop bound
rfc_g1_first_drop_endpoint_lemma.md           g=1 first-drop tau-two endpoint closure
rfc_h2_cofactor_bound.md                      second rank-drop/cofactor route
rfc_tau2_weighted_exterior_bound.md           tau-2 layer-codimension/root-line endpoint target
rfc_u23_tau2_endpoint_lemma.md                direct |A|=3,delta=2,comp=1 endpoint proof
rfc_theta_minus_one_chain_recurrence_target.md recurrence target for theta_2=-1 chains
rfc_theta_minus_one_isolation_lemma.md        outer-chain isolation and kernel nested-flag target
rfc_kernel_branch_nested_flag_recurrence.md   minimal nested-flag recurrence for kernel chains
rfc_theta_minus_one_truncation_status.md      depth-6/7 chain diagnostics and truncation target
rfc_high_defect_hard_segment_gap.md           current high-defect hard-segment gap and closure routes
rfc_depth5_base_seal_candidate.md             finite k=32/e=2 base-seal candidate for the gap
scripts/rfc_distance_analysis/rfc_defect_conservation.py fixed-witness defect-conservation diagnostic
scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py optimistic shortened-kernel rank recurrence
rfc_rank_pattern_induction_target.md          current finite-replica induction target
```

Generated CSV/TXT/PNG files live beside the notes that reference them. Helper scripts for
regenerating or probing these artifacts are in:

```text
scripts/rfc_distance_analysis/
```

Recent original-RFC near-MDS diagnostics:

```text
rfc_distance_certificate.py    conditional theorem-based distance certificate driver
rfc_original_falsification_report.py original-RFC bad-family/falsification report
rfc_near_mds_crossing.py        ideal random-rank first-moment crossing
rfc_matroid_lift_check.py       one-step matroid-union rank and local lift checks
rfc_recursive_alpha_profile.py  small exact/sampled recursive codimension profiler
rfc_exterior_constraint_profile.py r=2 exterior-constraint component profile
rfc_replica_rank1_profile.py    r=2 root-compatibility singleton profile
rfc_replica_span_moment.py      ordered-tuple span-aware diagnostic
rfc_subspace_span_moment.py     subspace-span diagnostic exposing visible-kernel state
rfc_flag_span_moment.py         two-layer flag diagnostic for kernel-zero propagation
rfc_theta_chain_normal_slice.py shortened-ambient and defect-slice checker for theta-chain truncation
rfc_visible_span_profile.py     exact local visible-span subspace profiler with support summaries
rfc_support_profile_bound.py    support-containment Gaussian bound from delta(A)
rfc_root_line_kernel_profile.py root-line kernel profile with exact-support inversion
rfc_replica_zero_moment.py      loose aggregate replica first-moment recurrence diagnostic
rfc_multicopy_falsification.py  multi-copy broad-family falsification stress model
rfc_paired_spine_cascade.py     paired-spine cascade diagnostic seeded by complete-stride flags
```

The intended hierarchy is:

```text
theorem note -> certificate script -> diagnostic profilers
```

The certificate output is conditional until the corrected tau-2 endpoint theorem, the finite-replica
recurrence, and the polynomial factor `C(d,N)` are fully bounded.
