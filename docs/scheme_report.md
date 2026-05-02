# Polynomial Commitment Scheme Report

This report summarizes the prover/verifier schemes currently present in the vendored research
code under `third_party/blaze/plonkish`, `third_party/plonkish_han0110`, and `third_party/pipfri`.
It focuses on what each scheme is useful for, where it is weak, and what algebraic setting the
implementation requires.

## Quick Inventory

| Scheme | Present in | Polynomial type | Algebra / assumptions | Setup |
| --- | --- | --- | --- | --- |
| KZG | Blaze plonkish, Han plonkish | univariate and multilinear variants | pairing-friendly curve scalar field; pairings | structured trusted setup |
| Mercury | Han plonkish only | multilinear | pairing-friendly curve scalar field; KZG backend | structured trusted setup |
| Gemini | Blaze plonkish, Han plonkish | multilinear via univariate KZG | pairing-friendly curve scalar field | structured trusted setup |
| Zeromorph | Blaze plonkish, Han plonkish | multilinear via univariate KZG | pairing-friendly curve scalar field | structured trusted setup |
| Zeromorph-FRI | Blaze plonkish only | multilinear via FRI backend | prime field plus hash/Merkle commitments | transparent |
| IPA | Blaze plonkish, Han plonkish | multilinear; Han also has univariate | prime-order elliptic curve group | transparent/hash-derived bases |
| Hyrax | Blaze plonkish, Han plonkish | multilinear; Han also has univariate | prime-order elliptic curve group | transparent/hash-derived bases |
| Brakedown | Blaze plonkish, Han plonkish | multilinear | prime field plus hash/Merkle commitments | transparent |
| FRI | Blaze plonkish only | univariate | FFT-friendly prime field in spirit; implementation is `PrimeField` | transparent |
| BaseFold | Blaze plonkish only | multilinear | any sufficiently large prime field; local binary mode uses `B128` | transparent |
| Blaze | Blaze plonkish only | multilinear over binary inputs / binary extension packing | binary word field plus `B128`, with BaseFold backend | transparent |
| Virgo | PipFRI workspace | multilinear / VPD-style | Arkworks `PrimeField`, hash/Merkle commitments | transparent |
| DeepFold | PipFRI workspace | multilinear | Arkworks `PrimeField`, FRI/DEEP-FRI-style code proofs | transparent |
| PolyFRIM | PipFRI workspace | multivariate / one-to-many | Arkworks `PrimeField`, RS/FRI-style code proofs | transparent |
| PIPFRI / DEPIPFRI | PipFRI workspace | multilinear | Arkworks `PrimeField`, FRI-style code proofs | transparent |

## KZG

KZG is the classic pairing-based polynomial commitment scheme. It is best when proof size and
verification are the priority: commitments and opening proofs are constant-size group elements, and
verification is pairing-based and succinct. This makes it very attractive for on-chain verification or
systems that can afford a trusted SRS.

Its main downside is the structured trusted setup and dependence on pairings. It is not
post-quantum, and proving often means large MSMs over elliptic-curve groups. The trusted setup is
also degree-bound and curve-specific.

Implementation notes:

- `UnivariateKzg<M: MultiMillerLoop>` exists in both plonkish copies.
- `MultilinearKzg<M: MultiMillerLoop>` exists in both plonkish copies.
- Tests instantiate `Bn256`, so the field is the scalar field of BN254/BN256.
- Requires `MultiMillerLoop`, `G1Affine`, `G2Affine`, and scalar field compatibility.

Associated paper: Kate, Zaverucha, and Goldberg, "Constant-Size Commitments to Polynomials and
Their Applications" (ASIACRYPT 2010).

## Mercury

Mercury is a multilinear PCS designed to get constant-size proofs without prover FFTs. It is a
pairing-based multilinear scheme and is relevant when we want KZG-like succinctness but the native
object is a multilinear polynomial rather than a univariate encoding.

The obvious cost is the same family of assumptions as KZG: pairings, structured reference strings,
and no post-quantum story. Compared with transparent hash/code schemes, it is less field-flexible
and less aligned with binary-field/high-throughput STARK-style pipelines. Compared with plain
multilinear KZG, its appeal is avoiding some prover FFT structure, not eliminating group work.

Implementation notes:

- Present only in `third_party/plonkish_han0110`.
- Implemented as `Mercury<UnivariateKzg<M>>`.
- Tests instantiate `Mercury<UnivariateKzg<Bn256>>`.
- Requires a pairing-friendly curve and KZG backend.

Associated paper: Eagen and Gabizon, "MERCURY: A multilinear Polynomial Commitment Scheme with
constant proof size and no prover FFTs" (IACR ePrint 2025/385).

## Gemini

Gemini is a compiler-style multilinear PCS built over a univariate PCS, used here with KZG. It is
good when the rest of the proof system is multilinear/sumcheck-based but the commitment backend is
a mature univariate PCS. It gives a clean path from multilinear openings to univariate openings.

Its weakness is that it inherits the backend. With KZG it gets succinct verification and compact
proofs, but also trusted setup and pairing dependence. If instantiated over a transparent backend,
its overhead profile changes; in this repo it is KZG-oriented.

Implementation notes:

- Present in both plonkish copies as `Gemini<UnivariateKzg<M>>`.
- Tests instantiate `Gemini<UnivariateKzg<Bn256>>`.
- Requires the scalar field of the pairing curve used by the KZG backend.

Associated paper: Bootle et al., "Gemini: Elastic SNARKs for Diverse Environments" (IACR ePrint
2022/420).

## Zeromorph

Zeromorph is another generic construction for committing to multilinear polynomials via a univariate
PCS. It is good as a modular bridge: plug in a univariate PCS and get multilinear evaluations.
With KZG it can be very compact, and it is useful for HyperPlonk-style systems that want a
multilinear interface.

Its weakness is backend inheritance and reduction overhead. In this repo's main instantiation,
Zeromorph is a pairing/KZG scheme, so it is not transparent or post-quantum. The FRI variant
avoids trusted setup, but FRI-style backends have larger proofs and hash-heavy verification.

Implementation notes:

- `Zeromorph<UnivariateKzg<M>>` exists in both plonkish copies.
- `ZeromorphFri<Fri<F, H>>` exists only in the Blaze fork.
- KZG version requires a pairing-friendly curve scalar field.
- FRI version requires a `PrimeField` and hash/Merkle commitment backend.

Associated paper: Kohrita and Towa, "Zeromorph: Zero-Knowledge Multilinear-Evaluation Proofs from
Homomorphic Univariate Commitments" (IACR ePrint 2023/917; Journal of Cryptology 2024).

## IPA

IPA commitments are based on discrete-log style inner-product arguments. They are good when we want
transparent setup and no pairings, while still working over elliptic-curve scalar fields. They are
especially attractive in recursive-proof contexts where the group arithmetic or verifier can be
amortized.

The downside is proof and verification scaling. IPA proofs are logarithmic rather than constant, and
verification involves group MSM-style work. For very large code-based workloads, this can be less
prover-friendly than hash/code schemes; for on-chain verification, it is usually worse than KZG
unless pairings are unavailable or setup transparency dominates.

Implementation notes:

- `MultilinearIpa<C: CurveAffine>` exists in both plonkish copies.
- `UnivariateIpa<C: CurveAffine>` exists only in Han plonkish.
- Tests instantiate Pasta/Pallas-like `Affine` in the Han copy.
- Requires a prime-order elliptic-curve group and scalar field; no pairing requirement.

Associated papers: Bootle et al.'s inner-product argument lineage and Bulletproofs; see
"Bulletproofs: Short Proofs for Confidential Transactions and More" (IEEE S&P 2018).

## Hyrax

Hyrax is a transparent, discrete-log-based commitment/proof approach. It is good when avoiding
trusted setup is important and the prover/verifier can tolerate group operations. The multilinear
Hyrax interface is a natural match for sumcheck-style proof systems.

The downside is that proof size and verifier work are not KZG-small. Hyrax trades setup simplicity
for larger communication and more group work. It is not a hash-only/post-quantum scheme.

Implementation notes:

- `MultilinearHyrax<C: CurveAffine>` exists in both plonkish copies.
- `UnivariateHyrax<C: CurveAffine>` exists only in Han plonkish and internally uses univariate IPA.
- Requires a prime-order elliptic-curve group and scalar field.

Associated paper: Wahby, Tzialla, shelat, Thaler, and Walfish, "Doubly-efficient zkSNARKs without
trusted setup" (IACR ePrint 2017/1132), the Hyrax paper.

## Brakedown

Brakedown is a linear-code/Merkle-style PCS family. It is good for field-agnostic, transparent
SNARKs with linear-time-ish prover goals. It avoids elliptic curves and pairings, so it is a better
fit for post-quantum-oriented or hash-based systems.

The downside is proof size and verifier query overhead. Hash/code schemes usually do not compete
with KZG on proof size or verifier compactness. They also require careful parameter selection for
soundness, rate, query count, and code distance. In research code, the danger is especially high:
it is easy to benchmark a fast path while under-checking low-degree or proximity invariants.

Implementation notes:

- Present in both plonkish copies as `MultilinearBrakedown<F, H, S>`.
- Requires `F: PrimeField`, a hash function, and a `BrakedownSpec`.
- Tests instantiate `Fr` with Blake2s/Keccak variants depending on copy.

Associated paper: Golovnev et al., "Brakedown: Linear-time and Field-agnostic SNARKs for R1CS"
(IACR ePrint 2021/1043; CRYPTO 2023).

## FRI

FRI is a Reed-Solomon proximity proof rather than a PCS by itself, but it is commonly wrapped into
a transparent PCS. It is good for STARK-like systems: transparent setup, hash-based commitments,
fast provers over FFT-friendly domains, and post-quantum-friendly assumptions.

Its downside is that it naturally wants Reed-Solomon domains with the right algebraic structure.
This usually means FFT-friendly fields or extension fields. Proofs are much larger than KZG/IPA,
and verifier work is hash-query heavy. A naive multilinear adaptation can introduce extra encoding
or reduction overhead.

Implementation notes:

- Present in the Blaze fork as univariate `Fri<F, H>`.
- The implementation is generic over `F: PrimeField`, but practical FRI wants suitable roots of
unity / domain structure.
- The old large sweep test is currently ignored to keep default tests fast.

Associated paper: Ben-Sasson, Bentov, Horesh, and Riabzev, "Fast Reed-Solomon Interactive Oracle
Proofs of Proximity" (ICALP 2018).

## BaseFold

BaseFold generalizes FRI's foldability idea from Reed-Solomon codes to foldable linear codes. It is
good when the field choice matters: it aims to be field-agnostic while still supporting efficient
multilinear PCS workflows. That makes it very relevant for proving computations over non-FFT-friendly
fields or fields tied to application logic.

The downside is that code/folding choices are subtle. The binary-field path in this repo already
needed algebraic attention around the folding pair choices. Compared with KZG, proof size and
verification are larger; compared with vanilla FRI, BaseFold has more moving pieces around code
selection, basecode, and folding invariants.

Implementation notes:

- Present only in the Blaze fork as `Basefold<F, H, ExtParams>`.
- Requires `F: PrimeField` in the trait implementation.
- The local code also supports a binary-code path using `B128` and `code_type = "binary_rs"`.
- Parameters include repetition count, rate, basecode rounds, RS basecode flag, and code type.

Associated paper: Zeilberger, Chen, and Fisch, "BaseFold: Efficient Field-Agnostic Polynomial
Commitment Schemes from Foldable Codes" (IACR ePrint 2023/1705; CRYPTO 2024).

## Blaze

Blaze is a multilinear PCS over binary extension fields using interleaved RAA-style coding and a
BaseFold backend in this implementation. It is good for workloads that naturally have binary
witnesses or benefit from bit-level packing. The design target is extremely fast commitment and
opening by exploiting binary arithmetic and very fast coding.

Its downside is specialization. It is not a generic prime-field PCS in the same sense as KZG or
BaseFold. It depends on binary representation, packing into `B128`, RAA coding, and a backend PCS.
This is precisely why testing matters: the RAA layer may work in isolation while the backend folding
or binary-code assumptions fail if the algebra is even slightly off.

Implementation notes:

- Present only in the Blaze fork.
- Uses `BlazeField` word types such as `Blazeu64`, conversion to/from `B128`, and internal
  `Basefold<B128, H, ...>` calls.
- The implementation is not exposed as a clean `PolynomialCommitmentScheme` trait implementation
  in the same way as the other plonkish PCS modules; it is a standalone API plus tests.

Associated paper: "Blaze: Fast SNARKs from Interleaved RAA Codes" (IACR ePrint 2024/1609).

## Virgo

Virgo is a transparent polynomial delegation / SNARK-style construction. It is good for transparent
proofs with logarithmic proof size and verification in the VPD setting, and it influenced later
FRI/RS-based multilinear PCS work.

Its weaknesses are complexity and historical fragility. The Virgo line has had follow-up work
studying limitations and implementation vulnerabilities, so it should be treated as research code
unless the exact protocol variant is audited. It is also less directly aligned with the newer
BaseFold/PIPFRI design target of very fast multilinear PCS proving.

Implementation notes:

- Present in `third_party/pipfri/virgo`.
- Generic over Arkworks `PrimeField`.
- Uses hash/Merkle commitments and FRI-like query proofs.

Associated paper: Zhang et al., "Transparent Polynomial Delegation and Its Applications to Zero
Knowledge Proof" (IACR ePrint 2019/1482), the Virgo paper.

## DeepFold

DeepFold is an RS/FRI-family multilinear PCS that improves soundness/proof efficiency by using
DEEP-FRI-style outside-domain sampling ideas. It is good when we want transparent, hash-based
multilinear commitments with smaller proofs or better concrete soundness than simpler BaseFold-like
approaches.

Its downside is that it is still code/hash based, so proofs and verifier query work are larger than
pairing-based commitments. It also has more protocol machinery than plain FRI or Brakedown, so it
needs careful tests around challenge derivation, query consistency, and folding equations.

Implementation notes:

- Present in `third_party/pipfri/deepfold`.
- Generic over Arkworks `PrimeField`.
- Uses Merkle query proofs over vectors of field elements.

Associated paper lineage: Ben-Sasson et al., "DEEP-FRI: Sampling Outside the Box Improves
Soundness" (IACR ePrint 2019/336; ITCS 2020), plus the DeepFold MLPCS line referenced by PIPFRI.

## PolyFRIM

PolyFRIM is a fast RS-IOP-based multivariate polynomial commitment. It is good for multivariate
commitment workloads and one-to-many proof settings where a prover must answer many related
openings efficiently. It is relevant to verifiable secret sharing and distributed protocols.

The downside is that it is more specialized than a general drop-in PCS. It is built around RS/FRI
machinery and particular batching patterns, so it is not the simplest base layer for arbitrary
SNARK integration. It also inherits the usual hash/code proof-size tradeoff.

Implementation notes:

- Present in `third_party/pipfri/polyfrim`.
- Generic over Arkworks `PrimeField`.
- Uses Merkle commitments and interpolation/query helper layers.

Associated paper: Zhang et al., "Fast RS-IOP Multivariate Polynomial Commitments and Verifiable
Secret Sharing" (USENIX Security 2024), introducing PolyFRIM.

## PIPFRI / DEPIPFRI

PIPFRI is the newest FRI-based multilinear PCS in this workspace. Its core pitch is combining
linear-time encodable-code PCS prover behavior with the compact proofs and verifier efficiency of
Reed-Solomon/FRI-style schemes. DEPIPFRI adds distributed proving and accountability.

This is highly relevant to this repo's goal: ultra-efficient FRI-based provers/verifiers. The
advantage over older FRI/MLPCS designs is lower FFT/hash overhead and better concrete prover time.
The downside is recency: it is new research code, so we should assume the implementation needs
strong modular correctness tests, rejection tests, and transcript/query audits before it becomes a
trusted core dependency.

Implementation notes:

- Present in `third_party/pipfri/pip_fri` and `third_party/pipfri/de_pip_fri`.
- Generic over Arkworks `PrimeField`.
- The current tests cover small end-to-end open/verify paths; large proof-size experiments are
  ignored by default.

Associated paper: Li et al., "Shred-to-Shine Metamorphosis of (Distributed) Polynomial
Commitments" (IACR ePrint 2025/1354; USENIX Security 2026 accepted), introducing PIPFRI and
DEPIPFRI.

## Practical Recommendations For This Repo

For the core direction of "ultra efficient ZK prover/verifiers based on FRI", the most relevant
schemes are:

1. PIPFRI / DEPIPFRI: likely the main target if the implementation can be hardened.
2. BaseFold: important baseline and conceptual foundation, especially for field-agnostic MLPCS.
3. Blaze: important if binary witnesses / binary extension fields are central.
4. FRI and Zeromorph-FRI: useful reference baselines and composition targets.
5. Brakedown / Virgo / PolyFRIM / DeepFold: useful comparison points and sources of test ideas.

Pairing and group-based schemes should not disappear, but they are probably secondary:

- KZG, Gemini, Zeromorph, Mercury are the succinct trusted-setup pairing family.
- IPA and Hyrax are the transparent discrete-log family.
- They are valuable baselines and may be useful for recursion/on-chain comparisons, but they are
  not the natural center of a FRI-first library.

## Sources

- KZG: https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf
- Hyrax: https://eprint.iacr.org/2017/1132
- FRI: https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2018.14
- DEEP-FRI: https://eprint.iacr.org/2019/336
- Virgo: https://eprint.iacr.org/2019/1482
- Brakedown: https://eprint.iacr.org/2021/1043
- Gemini: https://eprint.iacr.org/2022/420
- Zeromorph: https://eprint.iacr.org/2023/917
- BaseFold: https://eprint.iacr.org/2023/1705
- Blaze: https://eprint.iacr.org/2024/1609
- Mercury: https://eprint.iacr.org/2025/385
- PIPFRI / DEPIPFRI: https://eprint.iacr.org/2025/1354
- PolyFRIM: https://www.usenix.org/system/files/usenixsecurity24-zhang-zongyang.pdf
