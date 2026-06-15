# MLRS

`MLRS` means:

```text
multilinear-native Reed-Solomon
```

This folder records the unified view:

```text
univariate Reed-Solomon
+ recursive two-to-one quotient tower
+ bit-indexed basis
+ multilinear-style folding
```

Start here:

```text
towered_mlrs_first_principles.md
```

That note presents the common abstraction and then specializes it to:

```text
1. FFT-friendly prime fields via multiplicative roots of unity and X -> X^2;
2. binary extension fields via additive subspaces and subspace-polynomial quotient maps.
```

The binary-specific proof notes and verifier live in:

```text
docs/additive_mlrs/
scripts/additive_mlrs/
```
