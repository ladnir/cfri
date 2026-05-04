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

## Local Category Law

The exact first-moment recurrence should preserve at least the following one-coordinate category
law. Write the affine parent pair as:

```text
Y_0 = L + T D
Y_1 = L + (T+1)D
```

where `D = R-L` and `T` is uniform in `F^*`. For a non-common-zero coordinate:

```text
category                         output-weight law
L = 0, D = 0                     wt(Y_0,Y_1) = 0
L != 0, D = 0                    wt(Y_0,Y_1) = 2
L = 0, D != 0                    wt = 1 with prob 1/(|F|-1), else 2
L != 0, D != 0, L+D = 0          wt = 1 with prob 1/(|F|-1), else 2
L != 0, D != 0, L+D != 0         wt = 1 with prob 2/(|F|-1), else 2
```

This law is exact for both odd and binary characteristic when `T` is sampled from `F^*`; in binary
fields the two generic roots are still distinct because they differ by `1`.

The issue is not the local law. The issue is closure: to know the category distribution of
`(P_i(l), P_i(r))`, the next layer asks about joint distributions of four grandchildren. That is
the concrete form of the replica hierarchy.

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

The depth-3 low-weight mass up to the first-moment crossing is support-localized:

```text
original, weights <= 17:
  support 8: 1.68 expected codewords
  support 4: 0.28 expected codewords

systematic, weights <= 20:
  support 4: 0.84 expected codewords
  support 2: 0.16 expected codewords
  support 8: 0.08 expected codewords
```

In this tiny-field sampled regime, the systematic affine construction is not worse than the
original; it is better. That does not prove anything for the target binary-extension-field setting,
but it strengthens the suspicion that the large certified systematic/non-systematic gap is mostly
proof architecture and counting slack rather than an unavoidable systematicity tax.

## One-Step Pair Transition

The C++ tool also has a one-step pair-transition sampler:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe \
  --sample-rfc-one-step \
  --prime 5 \
  --depth 3 \
  --total-expansion 8 \
  --samples 100 \
  --seed 11 \
  --spectrum-path docs/sample_one_step_first_moment_gf5_depth3_c8_cpp.csv
```

For each sampled child code at depth `2`, it enumerates every ordered child pair `(l,r)`, computes
the category counts of `(P(l), P(r)-P(l))`, and exactly integrates over the fresh parent-layer
diagonal `T`. This produces the conditional first-moment spectrum for depth `3` without sampling
the parent diagonal.

For `GF(5)`, `c=8`, parent depth `3`, 100 child samples:

```text
original one-step first-moment crossing:      16
systematic one-step first-moment crossing:    20
```

The direct sampled depth-3 run gave crossings `17` and `20`. The systematic crossing matches
exactly; the original crossing differs by one symbol, plausibly because the one-step computation
averages over the parent diagonal instead of sampling it. This is a useful intermediate target:
the next rigorous recurrence should reproduce this pair-transition calculation without sampling
the child code.

The one-step category and low-tail decompositions are saved in:

```text
docs/sample_one_step_categories_gf5_depth3_c8_cpp.csv
docs/sample_one_step_low_tail_contrib_gf5_depth3_c8_cpp.csv
```

For the low-tail cutoffs above, the expected low-tail contribution by parent support is:

```text
original, cutoff <= 16:
  support 8: 0.920535
  support 4: 0.101784
  support 2: 0.000316

systematic, cutoff <= 20:
  support 4: 1.220223
  support 8: 0.317051
  support 2: 0.161279
```

This is qualitatively important. The original low tail is driven mostly by full parent support,
whereas the systematic low tail is driven by medium support. Any tight first-moment certificate
should therefore stay support-stratified; a scalar weight enumerator will blur exactly the part of
the distribution that matters.
