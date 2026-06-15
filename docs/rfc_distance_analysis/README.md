# RFC Distance Analysis

This folder contains research notes and generated artifacts for the original non-systematic RFC
distance-certificate push. Systematic notes are retained as deferred/reference material.

Useful entry points:

```text
rfc_distance_analysis_current_understanding.md current synthesis of what is known/proven/blocked
rfc_conjectural_distance_program.md           relaxed conjectures and falsifiable experiment plan
claude_new_direction.md                       active clean-start plan: ground-truth-first oracle/lift path
rfc_fixed_survivor_rank_tail_status.md       fixed-set rank/subspace-evasion diagnostic status
rfc_fixed_survivor_rank_recurrence_target.md fixed-survivor rank recurrence proof target
rfc_fixed_survivor_one_step_theorem.md       one-step fixed-set rank recurrence theorem
rfc_paired_compression_rank_lemma.md         exact rank doubling on paired coordinates
rfc_d2_rootline_parallel_class_lemma.md      D=2 singleton repair parallel-class lemma candidate
rfc_d2_rootline_pgl2_envelope.md             D=2 cross-ratio-free PGL2 repair envelope
rfc_d3_rootline_geometry_probe.md            D=3 side-colored matroid leakage probe
rfc_d3_surplus_codimension_signal.md         D=3 Hall-OK surplus exponent signal
rfc_rootline_generic_rank_lemma.md           matroid-union generic rank for root-line repair
rfc_rootline_schwartz_zippel_envelope.md     generic D root-line determinant envelope target
rfc_original_proof_upgrade_synthesis.md      route for upgrading original proof via surplus/flat-excess
rfc_incremental_upgrade_ladder.md            crawl-before-run theorem ladder for original-proof upgrades
rfc_original_vs_refined_profile_ledger.md    old-proof row vs refined h=1 bucket accounting
rfc_canonical_h1_bucket_complement.md        canonical h=1 bucket theorem and no-gain complement audit
rfc_common_zero_subcode_replacement_theorem.md full common-zero replacement theorem target
rfc_common_zero_subcode_paired_envelope_redflag.md all-paired compression warning for subcode-zero
rfc_common_zero_paired_shape_ledger.md       exact-kernel all-paired stress for common-zero route
rfc_common_zero_exact_paired_branch.md       theorem brick closing pure all-paired exact buckets
rfc_common_zero_one_spill_ledger.md          first mixed paired/singleton spill stress
rfc_one_spill_label_distribution.py          label-distribution profiler for one-spill row
rfc_one_spill_minority_label_no_go.md        local no-go for minority-label root charge
rfc_one_spill_exact_maximality_no_go.md      local no-go for extra exact-maximality qdims
rfc_one_spill_canonical_counting_no_go.md    no-go for closing one-spill via witness de-dup alone
rfc_one_spill_mixed_incidence_no_go.md       no-go for mixed lower-triple/spill-singleton charge
rfc_tensor_triple_rank_sampler.py            sampler for one-spill lower triple-rank events
rfc_tensor_triple_segre_classifier.py        Segre-line classifier for triple-rank codimension
rfc_flat_excess_control_stack.md             current theorem stack for flat-excess control
rfc_surplus_repair_codimension_target.md     top-profile tolerance and needed surplus exponent
rfc_surplus_incidence_stratification.md      corrected surplus exponent with flat-excess term
rfc_flat_excess_tolerance.md                 how flat excess shifts the near-MDS crossing
rfc_flat_excess_recursive_charge.md          flat excess as child rank event plus quotient blocker
rfc_flat_excess_charge_push.md               flat excess as marked-rank event plus combined exponent
rfc_high_rank_flat_duality_reduction.md      high-rank flat excess as ordinary child zero-set event
rfc_high_rank_flat_endpoint_theorem.md       Tier-1 theorem: h=1 flat excess is child line-zero
rfc_low_rank_flat_closure_endpoint.md        low-rank flat excess as marked closure-tail event
rfc_closure_tail_one_mark_recurrence.md      one-mark closure recurrence plus residual repair factor
rfc_one_mark_closure_defect_crawl.md         Tier-2 direct A0 core-defect correction
rfc_one_mark_pa_lift_containment.md          Tier-2 PA lift containment for scalar closure recurrence
rfc_closure_tail_multimark_recurrence.md     multi-mark closure recurrence with correlated PA chains
rfc_pa_chain_closure_theorem.md              PA-chain theorem target and multi-mark caveat
rfc_pa_chain_weak_rank_scale.md              safe weak PA-chain rank-drop scale check
rfc_one_rank_marked_tail_closure_reduction.md one-rank marked tail reduced to one-mark closure
rfc_small_flat_subcode_charge.md             small flat excess as generalized subcode-zero event
rfc_short_set_rank_tail_target.md            two-parameter rank-tail target for small flat excess
rfc_incremental_flat_rank_tail_target.md     marked quotient-rank target replacing aggregate B_d(z,s)
rfc_marked_incremental_one_step_state.md     top-fold marked categories and PA mixed blocker
rfc_pa_mixed_projection_lemma.md             deterministic PA mixed sibling projection lemma
rfc_multi_pa_projection_gap.md               falsified naive multi-PA projection reduction
rfc_multi_pa_graph_contraction_incidence.md  corrected multi-PA graph-contraction incidence target
rfc_multi_pa_rank_drop_witness_target.md     minimal-support witness proof target for multi-PA drops
rfc_multi_pa_linear_relation_form.md         fiber/relation invariant for multi-PA witness equations
rfc_multi_pa_codimension_budget.md           codimension needed for all-mixed PA stress profiles
rfc_all_mixed_pa_full_span_reduction.md      all-mixed PA low rank as full two-copy span deficiency
rfc_independent_randomizer_route.md          broad alternative: independent (a,b) conditioned a!=b
rfc_larger_block_fold_compatibility.md       arity-4 fold compatibility and protocol/proof tradeoffs
rfc_arity4_obstruction_check.md              whether binary PA obstructions survive true arity-4 blocks
rfc_arity4_occupancy_proxy.md                occupancy proxy for whether arity-4 pushes or changes modes
root_free_additive_rs_fold_report.md         root-free additive/subspace RS fold as MDS alternative
binary_rs_encoder_audit.md                   audit: current binary_rs path is not full additive RS
rfc_fixed_survivor_rank_tail_go_nogo.md      hard go/no-go checkpoint for fixed-survivor rank-tail
rfc_distance_manager_board.md                 current manager board and lane ownership
rfc_distance_stop_report.md                   historical stop/handoff report for the prior lemma-grind path
rfc_distance_resume_status.md                 historical resume note; see claude_new_direction.md for current path
scorecard.md                                  success/failure scorecard for proof ideas
rfc_fable_audit_2026_06_10.md                 external Fable audit and corrections
claude_report.md                              follow-up external review of the resume status
claude_second_opinion_2026_06_13.md           external review of hybrid base-seal direction
rfc_claude_report_response.md                 accepted/corrected actions from Claude report
rfc_distance_certificate_theorem.md           canonical conditional theorem and certificate statement
rfc_canonical_diagram_certificate_plan.md     reset plan: canonical diagram block grammar and ledger
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
rfc_support_two_tau2_quotient_frame_lemma.md  support-two tau-two quotient-frame target
rfc_tau1_quotient_line_incidence_lemma.md     fixed-flag tau-one quotient-line incidence bound
rfc_top_tau1_root_kernel_carry_block.md       theorem contract for top tau-one/root-kernel carry
rfc_root_kernel_container_cover_diagnostic.md root-kernel cover checkpoint and support-three frontier
rfc_support_three_rank_defect_incidence.md    support-three rank-defect incidence charge
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
rfc_two_marked_line_plane_lemma.md            local two-marked-line incidence diagram target
rfc_marked_plane_state_recurrence.md          finite marked-plane recurrence state target
rfc_theta_minus_one_chain_recurrence_target.md recurrence target for theta_2=-1 chains
rfc_theta_minus_one_isolation_lemma.md        outer-chain isolation and kernel nested-flag target
rfc_kernel_branch_nested_flag_recurrence.md   minimal nested-flag recurrence for kernel chains
rfc_theta_minus_one_truncation_status.md      depth-6/7 chain diagnostics and truncation target
rfc_high_defect_hard_segment_gap.md           current high-defect hard-segment gap and closure routes
rfc_depth5_base_seal_candidate.md             finite k=32/e=2 base-seal candidate for the gap
rfc_depth5_rank_pattern_contract.md           rank-pattern recurrence contract for B_5(1,34)
rfc_depth5_rank_pattern_audit.md              audit showing scalar rank-pattern needs flag refinement
rfc_depth5_finite_flag_recurrence_target.md   finite flag DP/theorem target for the base seal
rfc_top_boundary_base_seal_decision.md        decision: handle top boundary by finite base seal
rfc_covering_flag_lift_lemma.md               covering/projectivization lemma target for quotient lifts
rfc_depth5_flag_checkpoint_trace.md           corrected trace showing quotient-incidence blocker
scripts/rfc_distance_analysis/rfc_carried_flag_diagnostic.py carried-flag merge and next diagram diagnostic
scripts/rfc_distance_analysis/rfc_diagram_state.py incidence-diagram state skeleton
scripts/rfc_distance_analysis/rfc_marked_plane_state_diagnostic.py marked-line-in-plane state scanner
scripts/rfc_distance_analysis/rfc_diagram_path_dp.py carried-diagram path diagnostic
scripts/rfc_distance_analysis/rfc_flag_state_choice_diagnostic.py target flag-state choice diagnostic
scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py high-mass pair-family classifier
scripts/rfc_distance_analysis/rfc_quotient_diamond_diagnostic.py support-two quotient-frame diagnostic
scripts/rfc_distance_analysis/rfc_block_grammar_ledger.py canonical diagram block grammar table
scripts/rfc_distance_analysis/rfc_block_potential_probe.py simple potential probe over stress transitions
scripts/rfc_distance_analysis/rfc_tau1_carry_kappa_audit.py tau-one carry/kappa eligibility audit
scripts/rfc_distance_analysis/rfc_base_seal_budget.py depth-5 base-seal calibration budget
scripts/rfc_distance_analysis/rfc_base_seal_child_gap.py corrected child-gap comparison against scalar ceilings
scripts/rfc_distance_analysis/rfc_base_seal_tail_trace.py high-common-zero tail mechanism trace
scripts/rfc_distance_analysis/rfc_defect_conservation.py fixed-witness defect-conservation diagnostic
scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py optimistic shortened-kernel rank recurrence
scripts/rfc_distance_analysis/rfc_brute_force_moment.py exact small-field oracle for true RFC first moments
scripts/rfc_distance_analysis/rfc_lift_validator.py one-step lift validator against exact child moments
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
rfc_flag_bad_pair_classifier_level2_4_4_ge_2_5.csv level-2 healthy marked-plane pair classification
rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8.csv level-3 high-lift tau-one obstruction classification
rfc_flag_bad_pair_classifier_level3_4_7_ge_2_8_kernel_cover.csv fixed-table kernel-cover closure diagnostic
rfc_theta_chain_normal_slice.py shortened-ambient and defect-slice checker for theta-chain truncation
rfc_visible_span_profile.py     exact local visible-span subspace profiler with support summaries
rfc_support_profile_bound.py    support-containment Gaussian bound from delta(A)
rfc_root_line_kernel_profile.py root-line kernel profile with exact-support inversion
rfc_replica_zero_moment.py      loose aggregate replica first-moment recurrence diagnostic
rfc_brute_force_moment.py       exact small-field oracle for true RFC first moments
rfc_lift_validator.py           one-step recurrence/lift validator fed exact child moments
rfc_d3_rootline_geometry_selftest.py D=3 root-line projective-geometry leakage probe
rfc_multicopy_falsification.py  multi-copy broad-family falsification stress model
rfc_arity4_obstruction_profile.py arity-4 local mixed-PA obstruction profiler
rfc_arity_occupancy_proxy.py    arity-4 occupancy model for full-cover versus sparse modes
rfc_paired_spine_cascade.py     paired-spine cascade diagnostic seeded by complete-stride flags
```

The intended hierarchy is:

```text
current understanding / theorem note -> oracle + lift validator -> certificate script -> diagnostic profilers
```

The certificate output is conditional until the corrected tau-2 endpoint theorem, the finite-replica
recurrence, and the polynomial factor `C(d,N)` are fully bounded.
