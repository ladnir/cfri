# Additive MLRS Scripts

Scripts in this folder investigate a new foldable code design:

```text
additive-domain multilinear-native Reed-Solomon
```

This is separate from RFC distance-analysis scripts. Use this folder for prototypes that test:

```text
1. additive/subspace RS encoding,
2. MDS generator checks,
3. fold compatibility with multilinear partial evaluation,
4. query-path and cost comparisons against BaseFold.
```

Do not put RFC common-zero or random-foldable-code diagnostics here.

## Verifier

```text
verify_additive_mlrs.py
```

Standalone finite-field verifier for the proposed additive-MLRS construction.

It checks:

```text
1. additive basis triangularity by univariate degree;
2. additive-basis evaluation row space equals ordinary Vandermonde/RS row space;
3. MDS k-column minors, exhaustive when the subset count is small enough;
4. recursive fold identity for all or sampled field challenges.
```

Example:

```text
python scripts/additive_mlrs/verify_additive_mlrs.py --m 5 --d 2 --rho 2 --all-r
```

Parameters:

```text
m   = field degree for GF(2^m)
d   = log2(message length k)
rho = log2(rate expansion c), so N = 2^(d+rho)
```

Verified runs:

```text
GF(2^4), d=2, rho=1: k=4, N=8, exhaustive 70 MDS minors, all 16 challenges
GF(2^5), d=2, rho=2: k=4, N=16, exhaustive 1820 MDS minors, all 32 challenges
GF(2^5), d=3, rho=1: k=8, N=16, exhaustive 12870 MDS minors, all 32 challenges
GF(2^6), d=2, rho=3: k=4, N=32, exhaustive 35960 MDS minors, all 64 challenges
GF(2^6), d=3, rho=2: k=8, N=32, exact basis/row-space/fold checks, 50000 sampled MDS minors
```
