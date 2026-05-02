# Blaze + PiPFRI Bring-Up

`Blaze<BaseFold>` is the current working path. Blaze reduces the binary witness into `B128`
multilinear polynomials, and BaseFold can commit to those `B128` polynomials with its foldable
domain code.

`Blaze<PipFri>` needs a real backend compatibility pass before it can be correct:

- The imported PiPFRI code is generic over ark `PrimeField` and uses multiplicative FFT cosets
  through `GeneralEvaluationDomain`.
- Blaze's backend objects are `B128`, a characteristic-two binary extension field.
- There is no field homomorphism from `GF(2^128)` into the odd-characteristic prime fields used by
  the current PiPFRI code, so integer-encoding `B128` elements into Goldilocks would prove a
  different polynomial.
- `GF(2^128)^*` has odd order, so the current multiplicative 2-power FFT-domain PiPFRI shape does
  not directly apply to `B128`.

The right next step is to isolate PiPFRI's Merkle/query/FIAT-Shamir machinery from its
`GeneralEvaluationDomain` arithmetic, then add a binary/additive-domain prover and verifier that
accept `B128` evaluations. After that, Blaze can call the PiPFRI backend where it currently calls
`BlazeBasefoldPcs`.

Current status:

- PiPFRI now has a `FoldableCode` backend seam.
- `MultiplicativeFftCode` preserves the existing ark `PrimeField` / multiplicative FFT coset path.
- The shared engine is `Prover<T, Code, Mode>` and `Verifier<T, Code, Mode>`.
- `Transparent` is the default mode for ordinary PiPFRI openings.
- `Masked` is the hiding mode used by the compatibility `ZKProver` / `ZKVerifier` wrappers.
- The old ZK duplicate implementation has been removed; the wrappers delegate to the shared masked
  engine.
