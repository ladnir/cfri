# RFC Distance Analysis Scripts

Research calculators, probes, classifiers, and toy checkers for the RFC distance-certificate work.

These scripts are intentionally separate from production backend code. Most commands in the notes
assume they are run from the repository root, for example:

```text
python scripts/rfc_distance_analysis/rfc_distance_certificate.py --depth 11 --expansion 8 --q-log2 128 --security-bits 80
```

Sibling imports are local to this folder, so direct execution by path from the repository root is
the preferred style.

Recent original-RFC near-MDS probes:

```text
rfc_distance_certificate.py        conditional theorem-based distance certificate driver
rfc_original_falsification_report.py original-RFC bad-family/falsification report
rfc_exterior_constraint_profile.py  exact r=2 exterior-count profiles by matroid components
rfc_replica_rank1_profile.py        r=2 root-compatibility singleton profile
rfc_replica_zero_moment.py          loose/optimistic aggregate replica recurrence diagnostic
rfc_replica_span_moment.py          ordered-tuple span-aware diagnostic
rfc_subspace_span_moment.py         subspace-span diagnostic exposing visible-kernel state
rfc_flag_span_moment.py             two-layer flag diagnostic for kernel-zero propagation
rfc_theta_chain_normal_slice.py     shortened-ambient/defect-slice checker with optimistic rho and strict hard-trace columns
rfc_defect_conservation.py          fixed-witness rank-defect conservation checker for exposed shortened ambients
rfc_shortened_rank_recurrence.py    optimistic rho_h(D,z) recurrence with paired-spine compression
rfc_state_constant_budget.py        q-dimensional budget helper for hard-trace state constants
rfc_visible_span_profile.py         exact local visible-span subspace profiler with support summaries
rfc_support_profile_bound.py        Gaussian support-containment bound from delta(A)
rfc_root_line_kernel_profile.py     root-line kernel profile with exact-support inversion/discovery
rfc_flag_intersection_enum.py       exact small-depth flag-intersection and complete-stride gates
rfc_paired_spine_cascade.py         paired-spine lift diagnostics seeded by complete-stride flags
rfc_multicopy_falsification.py      multi-copy broad-family falsification stress model
```

The intended hierarchy is:

```text
theorem note -> rfc_distance_certificate.py -> diagnostic profilers
```

`rfc_distance_certificate.py` is a final-shape conditional calculator. It uses theorem exponents
and emits CSV to stdout plus optional JSON/CSV artifacts; it does not consume empirical profiler
counts. The profiler scripts below it are diagnostics for local theorem and recurrence development.
After the `a=5, delta=3, comp=1, g=1` endpoint correction, this driver should be read as
conditional on a layer-codimension tau-2 theorem, not on the older generic/component shortcut.

`rfc_subspace_span_moment.py`, `rfc_replica_span_moment.py`, and `rfc_flag_span_moment.py` support
`--singleton-charge endpoint-tau2-layer`. This mode uses the calibration charge:

```text
theta_2(A)=max_h(2h-4-gamma_h(A))
charge=4delta(A)-4-theta_2(A)
```

with the uniform-matroid heuristic `gamma_h=(h-g)^2` for intermediate layers and
`gamma_delta=a-comp` for the full-kernel component layer.

`rfc_flag_span_moment.py` also has `--singleton-charge endpoint-tau2-layer-incidence`, a slower
checkpoint mode that floors the local tau-two quotient dimension by `r1-r0` after the child flag is
chosen. It also applies the tau-two exact-support Grassmann cap `local_charge >= |A|`. This models
the first quotient-incidence correction in `rfc_tau2_incidence_framing_lemma.md`; refined
partial-cover fiber accounting is still proof work, not a certified script feature.
For the boundary `tau=2,a=2,delta=2,comp=2,K=0` row, it uses a safe joint marked-line child bound;
traces mark this route with `dominant_h=-4` if it is selected.

`rfc_theta_chain_normal_slice.py` also reports the conservative strict hard-trace potential used by
the current theorem target. These columns use `rfc_shortened_rank_recurrence.py` internally with
`allowed_singletons=(0,5)`, `hard_force_all_singletons=True`, `hard_visible_dim_loss=2`, and
`hard_step_charge=9`, then report the charged dynamic-programming optimum:

```text
strict_hard_potential_logq = 9 * hard_steps + rho_terminal
strict_hard_margin_logq    = strict_hard_potential_logq - E_anc - split_const
```

The columns are deterministic theorem-exponent diagnostics; they do not consume empirical profiler
counts. The hard charge is optimized inside the dynamic program, so these columns are not the
post-hoc charge of a trace chosen only to minimize terminal `rho`.
The companion `boundary_plus_strict_hard_margin_logq` column adds the scenario's explicit
`--charge` to the strict hard potential. Treat it as boundary-only: it applies only after a
separate lemma proves that the local charge is disjoint from the internal hard steps already
counted in `strict_hard_potential_logq`.
The reachable production-floor stress table for the current c=8/k=2048/e=71 target is
`docs/rfc_distance_analysis/rfc_theta_chain_normal_slice_reachable_child32_z34_sweep.csv`.

`rfc_shortened_rank_recurrence.py` supports `--allowed-singletons 0,5` to restrict optimistic
`rho_h(D,z)` traces to all-paired steps and hard theta-compatible singleton bursts. Use this when
checking whether a cheap paired-spine rank trace should also pay one `q^-9` first-drop charge per
hard `s=5` step. Add `--ancestor-exponent E` to print the hard-trace margin
`9*hard_steps + rho - E`.
For kernel-following minimal hard rows, add `--hard-force-all-singletons`; this requires an `s=5`
hard step to propagate all five singleton zeros into the child rank event, matching the
`z_L=p+s` hard-row zero propagation rather than the looser survivor-envelope minimum over forced
singletons.
Add `--hard-visible-dim-loss 2` for the conservative tau-two kernel-child coverage target, where
the visible two-dimensional quotient is removed before halving the child rank dimension.

`rfc_state_constant_budget.py` is deterministic bookkeeping, not a profiler. It converts canonical
state-count models such as `N^32 * (q+1)^2 * 52^11` into q-dimensional overhead so the hard-trace
potential in `rfc_theta_minus_one_isolation_lemma.md` can be checked against its remaining margin.

`rfc_root_line_kernel_profile.py` reports the corrected tau-2 asymptotic endpoint columns:
`generic_endpoint_logq`, `component_endpoint_logq`, `endpoint_bound_logq`, and
`endpoint_root_weight_logq`. These columns are now best interpreted as the older two-endpoint
diagnostic bound. Positive residual rows can indicate a missing intermediate layer rather than a
finite-constant issue.
Newer detail rows also report the dominant contained `kappa` layer and an empirical
`theta_tau` diagnostic.
It also has guarded support-targeted and connected-support discovery modes for the dense
`a=4, delta=3, comp=1, g=2` endpoint profile.

`rfc_subspace_span_moment.py` and `rfc_replica_span_moment.py` support
`--singleton-charge endpoint-tau2`, which swaps in the corrected tau-2 endpoint charge under the
same uniform quotient approximation used by their component-uniform diagnostics.

`rfc_flag_span_moment.py` is the first checkpoint for the proposed child flag state
`pi(K) <= pi(W)`. It is size-guarded and intended for small depths only; it uses one-layer child
counts to upper-bound two-layer child flags and should not be read as a certificate.
Use `--prune-to-final-span 1` when checking the distance first moment at larger diagnostic depths;
this computes only spans that can feed the final top line. Use `--report-local-theta -1` to list
best tau-two transitions using the first-drop `theta_2=-1` layer, and `--trace-span` with
`--trace-z` to follow a non-line state. The theta report also includes the immediate outer/inner
child transition type, which is useful for spotting possible first-drop chains.
The anti-conservative diagnostic-only `--cover-lift-mode` switch removes selected parent-lift
multiplicities to locate where lift/quotient incidence is concentrated. Supported probes are
`tau0`, `tau1`, `tau2`, `tau0tau1`, `tau0tau2`, `tau1tau2`, and `all`. It is not a certificate
mode. For tau-positive branches, quotient lines/planes are event data and must be counted by a
quotient-incidence/fiber theorem; zeroing those lifts is only a sensitivity test.
The narrower `--cover-kernel-lift` diagnostic removes only the `K<=L+L` kernel-lift factor while
keeping quotient incidence. In the current depth-5 base checkpoint it leaves the crossing at
`z=137`; combined with safe tau-zero covering it still only reaches `z=135`, so kernel-lift
multiplicity is not the active source of looseness.
The diagnostic-only `--flag-bound best-shortened` mode tests an ideal shortened ambient for the
inner-first child flag count. It leaves the same depth-5 checkpoint crossing, so the missing bound is
not recovered by this one-layer shortened-child correction.
Use `--report-tau1-incidence` to print fixed-flag tau-one quotient-line diagnostics:
quotient ambient dimension, universal post-root exponent, support-subcode saving, and charged
post-root exponent. In the current `z=34` tau-zero-cover trace, all tau-one rows have zero
support-subcode saving, so tau-one local sharpening is not the active rescue.
Trace rows also print `child_bound_choice` plus the outer-first and inner-first child-flag
relaxation values. Use these columns to see whether the current bound is following the outer
container, the stricter inner event, or a shortened diagnostic ambient.
By default `--trace-z` follows the selected child-bound branch. Pass `--trace-follow projection`
to recover the older outer-projection spine; that path is useful for orientation but is not always
the dominant recurrence branch.
Use `--report-theta-chains -1` to summarize the longest consecutive best-transition chain of
tau-two `theta_2=-1` rows through the outer/inner child graph.

`rfc_multicopy_falsification.py` is adversarial. It estimates whether broad one-copy near-families
can intersect across many independent RFC copies often enough to threaten the target excess.

`rfc_replica_zero_moment.py` has two explanation helpers for the finite depth-5 base-seal route:
use `--explain-z Z --top-terms K` to print the dominant one-step split terms along the best trace,
and `--dump-level L --dump-start A --dump-stop B` to print aggregate moments at an intermediate
level. These are deterministic recurrence diagnostics, still conditional on the rank-pattern
local theorem.

`rfc_visible_span_profile.py` precomputes the exact Gaussian-binomial number of local subspaces and
skips subsets above `--max-subspaces`; this keeps sampled runs from accidentally turning into large
enumerations.
