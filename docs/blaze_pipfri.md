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

BaseFold now also has the same conceptual hiding shape through `HidingBasefold`. It commits with one
extra multilinear variable:

```text
F(x, t) = f(x) + t * r(x)
```

Openings are still public openings of `f(z)`: the prover opens `F(z, 0)`, so the verifier API keeps
the original point length and evaluation. The underlying BaseFold instance sees one more variable.
This keeps the ordinary `Basefold` hot path unchanged and makes hiding an explicit opt-in type.

Blaze has matching opt-in entry points: `setup_with_hiding`, `trim_with_hiding`,
`commit_and_write_with_hiding`, `open_with_hiding`, and `verify_with_hiding`. They extend each
Blaze row with a random masked half and open at `t = 0`. This gives the same algebraic shape at the
Blaze boundary while avoiding a second backend-level hiding pass when Blaze is composed with a
transparent backend.
