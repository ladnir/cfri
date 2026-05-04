# RFC First-Moment Direction

The threshold certificate proves a safe floor, but it is not the proof shape we ultimately want for
distance. The first-moment target is:

```text
E[# nonzero messages m with wt(C(m)) <= D] < 2^-lambda.
```

For the systematic code this becomes:

```text
sum_s (# messages of systematic support s)
    * Pr[parity_weight(m) <= D-s]
  < 2^-lambda.
```

This prices the systematic block directly and only union-bounds the final bad event. It avoids the
stronger intermediate invariant used by the current support-threshold certificate.

## Baseline Ceiling

The C++ tool now has an ideal random-code first-moment mode:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe \
  --ideal-first-moment \
  --depth 11 \
  --total-expansion 8 \
  --field-bits 128 \
  --security-bits 80
```

The emitted table is saved in:

```text
docs/ideal_random_first_moment_c8_depth1_to_11_global80.csv
```

At depth 11 and total expansion `c=8`, both the fully random linear and systematic-random-parity
first-moment baselines certify:

```text
14264 / 16384 = 0.87060547.
```

This is an intentionally optimistic ceiling, not an RFC estimate. Its main value is diagnostic:
over a 128-bit field, systematicity by itself is not causing the large gap seen in the RFC
threshold certificate. The loss is coming from foldability and/or from the current proof method.

## Exact RFC First Moment: State Issue

For the affine systematic construction, a parent parity pair is:

```text
L + T(R-L),  L + (T+1)(R-L).
```

For a fixed parent message `(l,r)`, the parity-weight distribution depends on the joint behavior of
`P_i(l)` and `P_i(r)`, not only on their individual weights. Thus a scalar weight enumerator is not
closed under recursion.

A natural coordinate category for a pair `(L,D)` with `D=R-L` is:

```text
00:       L = 0, D = 0
10:       L != 0, D = 0
01:       L = 0, D != 0
root0:    L != 0, D != 0, L + D = 0
generic:  L != 0, D != 0, L + D != 0
```

These categories determine the local one-pair output-weight law under uniform nonzero `T`. However,
propagating this pair enumerator exactly through another recursive layer asks for the joint
enumerator of four child messages. In other words, the naive exact first-moment recurrence exposes
a replica hierarchy.

The next step is to find the smallest closed state that is still rigorous enough for the original
RFC baseline and the systematic affine RFC. Two plausible paths:

1. A controlled replica/enumerator hierarchy for low depth, used to calibrate how much slack the
   threshold proof has.
2. A provable domination/decoupling lemma that upper-bounds the needed pair state by a tractable
   support-stratified distribution, then first-moment union-bounds only at the final layer.

The original non-systematic RFC should be analyzed with the same first-moment machinery. That is
the only fair baseline for deciding how much of the systematic gap is real.

## Tiny-Field Sampled Spectra

As a calibration tool, the C++ executable now has a sampled tiny-field mode:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe \
  --sample-rfc-first-moment \
  --prime 5 \
  --depth 3 \
  --total-expansion 8 \
  --samples 100 \
  --seed 11 \
  --spectrum-path docs/sample_first_moment_gf5_depth3_c8_cpp.csv
```

This samples affine RFC constructions with `T` uniform in `GF(5)^*`, exhausts every message, and
averages the nonzero-codeword spectrum. It is not a certificate, but it gives a concrete target for
the exact first-moment recurrence.

For `GF(5)`, `c=8`, depth 2, 500 samples:

```text
original average minimum distance:     14.290000 / 32 = 0.44656250
original first-moment crossing:        13
systematic average minimum distance:   15.760000 / 32 = 0.49250000
systematic first-moment crossing:      14
```

For `GF(5)`, `c=8`, depth 3, 100 samples:

```text
original average minimum distance:     18.870000 / 64 = 0.29484375
original first-moment crossing:        17
systematic average minimum distance:   22.840000 / 64 = 0.35687500
systematic first-moment crossing:      20
```

In this tiny-field sampled regime, the systematic affine construction is not worse than the
original; it is better. That does not prove anything for the target binary-extension-field setting,
but it strengthens the suspicion that the large certified systematic/non-systematic gap is mostly
proof architecture and counting slack rather than an unavoidable systematicity tax.
