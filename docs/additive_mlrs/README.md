# Additive MLRS

Status: new design space, separated from RFC distance analysis.

`additive_mlrs` means:

```text
additive-domain multilinear-native Reed-Solomon
```

For the unified first-principles explanation covering both FFT-friendly prime fields and binary
additive fields, see:

```text
docs/mlrs/towered_mlrs_first_principles.md
```

The goal is a foldable code that remains compatible with multilinear polynomial claims while
getting Reed-Solomon distance by construction.

This is not the current RFC construction and not the existing `binary_rs` BaseFold path.

## Motivation

The RFC distance proof keeps running into tensor/mixed-shape obstructions. RS codes have full MDS
distance, and FRI shows that RS can be foldable. The issue is that ordinary FRI often relies on
roots of unity or multiplicative domains, while Blaze/BaseFold wants binary/additive-field
compatibility and a native multilinear interface.

The proposed route is:

```text
multilinear message coefficients
  -> interpreted in a recursive additive/subspace polynomial basis
  -> evaluated as a univariate RS code on an additive domain
  -> folded by additive quotient maps
```

Distance would come from RS:

```text
distance = N - k + 1.
```

Multilinear compatibility would come from the recursive basis:

```text
folding at challenge r maps coefficient halves (f0, f1) to f0 + r f1.
```

## Core Algebra

For a one-dimensional additive fiber:

```text
{x, x + beta}
```

the quotient map is the subspace polynomial:

```text
L_beta(X) = X^2 + beta X.
```

It is constant on each fiber:

```text
L_beta(x + beta) = L_beta(x).
```

A degree-bounded univariate polynomial can be decomposed as:

```text
F(X) = F0(L_beta(X)) + B_beta(X) F1(L_beta(X)),
```

where `B_beta` is a normalized local coordinate that distinguishes the two points in a fiber.

Folding at multilinear challenge `r` gives:

```text
G(Y) = F0(Y) + r F1(Y).
```

This is the same algebra as a multilinear partial evaluation:

```text
f(x_1,...) = f0(...) + x_1 f1(...)
f(r,...)   = f0(...) + r f1(...).
```

## What This Is Not

This is not current RFC:

```text
RFC = tensor/multilinear recursive code with random or structured local pair transforms.
```

This is not the repo's current `binary_rs` path:

```text
binary_rs = additive-table fold-consistent BaseFold path.
```

The current audit found that `binary_rs` is not a full global additive RS/MDS code. It has small
MDS failures in the standalone model.

See:

```text
docs/rfc_distance_analysis/binary_rs_encoder_audit.md
```

## Target Properties

The construction should satisfy:

```text
1. Native multilinear message interface:
   messages are length k=2^d coefficient/evaluation data for multilinear claims.

2. Free or near-free translation:
   the existing Boolean-hypercube interpolation/coefficient transform is enough.

3. Global RS codeword:
   committed word is evaluation of a degree-<k univariate polynomial on N=c*k distinct
   additive-domain points.

4. Fold compatibility:
   one fold round on the codeword equals multilinear partial evaluation on the message.

5. MDS distance:
   every k columns are independent.

6. Root-free:
   no multiplicative roots of unity; folding uses additive subspace quotient maps.
```

## Immediate Audit Tasks

1. Build a minimal true additive-MLRS prototype over small `GF(2^m)`.
2. Verify generator row space equals ordinary RS/Vandermonde row space on an additive domain.
3. Exhaustively check all `k`-subsets for small `k,n`.
4. Verify fold identity:

```text
encode(f0 + r f1) == fold_r(encode(f0, f1)).
```

5. Compare query path and encoding cost against current BaseFold.

Scripts for this design space should live in:

```text
scripts/additive_mlrs/
```

The current executable verifier is:

```text
scripts/additive_mlrs/verify_additive_mlrs.py
```

It checks the triangular basis theorem, equality with RS/Vandermonde row space, small MDS minors,
and recursive fold compatibility over small `GF(2^m)`.

## Current Notes

Distance/basis verification:

```text
distance_basis_theorem.md
```

This proves the core MDS-distance claim for the proposed construction: the additive multilinear
basis is triangular and spans exactly the univariate degree-`<k` polynomial space.

Fold compatibility:

```text
fold_compatibility_theorem.md
```

This proves the one-step identity:

```text
fold_r(encode(a_0, a_1)) = encode(a_0 + r a_1)
```

and explains how it iterates through the additive quotient tower.

Construction target:

```text
ml_native_additive_rs_spec.md
```

The initial idea was recorded during the RFC distance work:

```text
docs/rfc_distance_analysis/root_free_additive_rs_fold_report.md
```

Going forward, new work should be added here instead of under `rfc_distance_analysis`.
