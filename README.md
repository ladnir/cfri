# cfri

`cfri` is a bring-up workspace for ultra-efficient FRI-based ZK provers and verifiers.

Imported research code:

- Blaze / BaseFold plonkish fork: `third_party/blaze`
- Original `han0110/plonkish` import, kept for reference after the repo correction: `third_party/plonkish_han0110`
- PIP_FRI and DePIP_FRI: `third_party/pipfri`

The imports currently keep their native Rust workspaces intact because they use different toolchain needs. Blaze's Rust workspace is nested at `third_party/blaze/plonkish` and currently requires Rust nightly for `portable_simd`. The top-level scripts invoke nightly for Blaze and the default Cargo toolchain for PiPFRI:

```powershell
.\scripts\build.ps1
.\scripts\test.ps1
```

`scripts/test.ps1` defaults to compile-only tests (`cargo test --no-run`) so bring-up stays fast and deterministic. Use `.\scripts\test.ps1 -Run` for full test execution.

Benchmarks are intentionally not part of the unified scripts.

## Bring-up notes

- `scripts/build.ps1` passes on Windows with the native toolchain pins from both imported projects.
- `scripts/test.ps1` passes as a compile-only test gate.
- `scripts/test.ps1 -Run` now executes the default fast tests in both imports. Large proof-size sweeps and proof-system matrix tests are marked `#[ignore]` with reasons, so they remain available explicitly without slowing the default gate.
- Initial slow-test findings:
  - PiPFRI's `fri_pcs_test` used `variable_num = 20` and exceeded 60 seconds.
  - PiPFRI proof-size tests swept large parameters such as 17 through 23 variables.
  - PiPFRI's interwoven Merkle test used a randomized retry loop against a proof-size threshold and took several seconds.
  - The initial `han0110/plonkish` import was the wrong Blaze source; it contained Brakedown, not the Blaze RAA/PRAA implementation. The actual Blaze implementation is now imported from `hadasz/plonkish_basefold`.
- PiPFRI currently emits a warning in `de_network` for a `const Cell<bool>` that does not provide shared mutable state. That is worth fixing during the testing hardening pass.
