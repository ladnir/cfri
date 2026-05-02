# C-FRI PCS API

This document records the API shape we are moving the imported research code toward. The first goal is clarity and testability, not a large abstraction hierarchy.

## Target Shape

Each scheme should expose the same concrete module-level operations:

```rust
setup(config) -> Params
trim(params) -> (ProverKey, VerifierKey)
commit(prover_key, polynomial, point) -> (Commitment, ProverState)
open(prover_key, prover_state, commitment, point, value) -> Proof
verify(verifier_key, commitment, point, value, proof) -> bool
```

The `point` argument on `commit` is temporary. A normal PCS commitment should not depend on the opening point. Some imported PiPFRI-family code currently derives fixed combining tensors from the opening point before commitment, so the transition facade keeps that dependency visible instead of hiding it. Removing this point dependency is part of the cleanup phase.

## Rules

- Prefer concrete module functions over a global trait until a trait removes real duplication.
- Keep proof, commitment, parameter, and state types concrete per scheme.
- Do not introduce dynamic dispatch or heap-allocated callback abstractions in prover/verifier hot paths.
- A transcript should absorb typed values and squeeze typed values, but should not be parameterized by one field type globally.
- Tests should use the facade API, not raw upstream-shaped prover/verifier choreography.
- Every default test should be small. Release-mode target: no individual test over 4 seconds, with most tests far below that.

## Current Facade Status

`crates/cfri/src/pcs.rs` is the transitional facade for the owned PiPFRI-family code:

- `pcs::fri`
- `pcs::pip_fri`
- `pcs::de_pip_fri`
- `pcs::deepfold`
- `pcs::polyfrim`
- `pcs::virgo`

The facade intentionally stores some verifier-side public messages inside proof objects because the imported implementations currently pass those messages by mutating verifier structs during opening. Later cleanup should move those messages into explicit proof fields and make verification reconstruct its verifier state from `(VerifierKey, Commitment, Point, Value, Proof)`.

Blaze/BaseFold now lives in `crates/cfri/src/plonkish_backend`, re-exported through `cfri::blaze`. It builds on stable Rust and should get the same concrete facade as the PiPFRI-family code.
