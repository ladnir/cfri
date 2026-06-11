# RFC Claude Report Response

Scope: response to `docs/rfc_distance_analysis/claude_report.md`.

## Accepted

1. The `g=1` row was over-summarized in the resume note. The first-drop layer proof gives:

   ```text
   gamma_2 >= 1,
   first-drop exponent = -1.
   ```

   The full row conclusion `theta_2=-1` still relies on controlling the `h=delta=3` full-kernel
   layer by the connected component endpoint. Until that endpoint is proved directly for
   `delta=3`, or proved generally, this should be labeled conditional.

2. The resume note mixed proof statuses. It now labels artifacts as proved, conditional,
   proof skeleton, recurrence contract, or diagnostic.

3. The depth-4/6/7 flag checkpoint crossings are pessimistic and scale linearly in `N`. They are
   useful obstruction diagnostics, not evidence that the implemented checkpoint already reaches
   near-MDS scaling.

4. The best next proof step is the shared-randomness-safe multi-layer flag transition theorem. The
   decomposable `|A|=2,delta=2,comp=2` marked-line row and the `theta_2=-1` kernel-chain row are two
   faces of the same missing object: count one nested child flag over shared randomness, not a
   product of child moments.

## Corrected

The report's construction-mismatch claim is too strong for this repository. The local encoder in:

```text
crates/cfri/src/backend/basefold.rs
```

uses:

```text
left + T * right,
left + (T+1) * right,
```

whose pair matrix has determinant `1`. Thus the in-repo code is already aligned with the
determinant-1 algebra in the current certificate notes, not the paper's `[[1,T],[1,-T]]` matrix.

The remaining construction issue is narrower but still important:

```text
the proof notes often assume T uniform nonzero,
while the current table generation appears to sample field elements directly.
```

The theorem must either restate the root law for uniform `T in F`, or track the finite constant
between uniform `F` and uniform `F^*`.

## Numbering Clarification

The ideal final-shape artifact gives:

```text
e=71 log2_bad = -119.67789152872501,
slack_bits    =  39.67789152872501.
```

The `-121.83` number is the corrected scalar stress row from the multi-copy/exact-support
falsification model, not the ideal binomial moment.

## Updated Target

The target theorem is now:

```text
docs/rfc_distance_analysis/rfc_multilayer_flag_transition_theorem.md
```

It defines:

```text
F_h((t_0,z_0), ..., (t_m,z_m))
```

for multi-layer flags with:

```text
1. exact visible-support indexing,
2. shared-randomness-safe child lifts,
3. tau-two exact-support Grassmann incidence,
4. marked-line/frame specialization for |A|=2,delta=2,comp=2,
5. kernel-chain specialization for theta_2=-1 rows.
```

Then use that theorem to discharge the two current recurrence blockers as corollaries.

Remaining proof work: turn the safe transition proposition into a theorem-grade lift bound with
all projection/kernel hypotheses stated, then prove the two corollaries rather than treating them
as proof targets.
