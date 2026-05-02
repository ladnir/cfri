# cfri

`cfri` is a bring-up workspace for ultra-efficient FRI-based ZK provers and verifiers.

Owned research-code imports:

- Blaze / BaseFold plonkish fork, trimmed to the active Blaze/BaseFold target plus support code: `crates/cfri/src/plonkish_backend`
- PIP_FRI family implementations: `crates/cfri/src`
- Original `han0110/plonkish` import, kept for reference after the repo correction: `third_party/plonkish_han0110`

The implementations are now copied into the core crate so we can claim ownership and simplify them directly. The old third-party directories remain as references, but the normal build/test scripts target the owned source. The active Blaze/BaseFold code now builds on stable Rust:

```powershell
.\scripts\build.ps1
.\scripts\test.ps1
```

`scripts/test.ps1` defaults to compile-only tests (`cargo test --no-run`) so bring-up stays fast and deterministic. Use `.\scripts\test.ps1 -Run` for full test execution.

Benchmarks are intentionally not part of the unified scripts.

## API direction

The owned implementations are being moved behind a small concrete PCS facade before deeper cleanup. The documented target is in [`docs/api.md`](docs/api.md). The important rule is that each scheme should expose the same simple `setup -> trim -> commit -> open -> verify` shape without adding a heavyweight generic composition layer.

## Bring-up notes

- `scripts/build.ps1` passes on Windows against the owned core crates.
- `scripts/test.ps1` passes as a compile-only test gate.
- `scripts/test.ps1 -Run` now executes the default fast tests in both owned crates. Large proof-size sweeps and proof-system matrix tests are marked `#[ignore]` with reasons, so they remain available explicitly without slowing the default gate.
- Initial slow-test findings:
  - PiPFRI's `fri_pcs_test` used `variable_num = 20` and exceeded 60 seconds.
  - PiPFRI proof-size tests swept large parameters such as 17 through 23 variables.
  - PiPFRI's interwoven Merkle test used a randomized retry loop against a proof-size threshold and took several seconds.
  - The initial `han0110/plonkish` import was the wrong Blaze source; it contained Brakedown, not the Blaze RAA/PRAA implementation. The actual Blaze implementation is now imported from `hadasz/plonkish_basefold`.
- PiPFRI currently emits a warning in `de_network` for a `const Cell<bool>` that does not provide shared mutable state. That is worth fixing during the testing hardening pass.
