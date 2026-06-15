# Binary-RS Encoder Audit

Status: small deterministic audit of the current `binary_rs` BaseFold path.

## Question

Does the existing `binary_rs` path already commit to a full additive Reed-Solomon code, hence an
MDS code?

Short answer:

```text
No, not as currently modeled.
```

The path is fold-consistent over additive/subspace tables, but the generator is not the obvious
additive-RS/Vandermonde evaluation code. In small cases it also fails MDS.

## What Was Audited

The script:

```text
scripts/rfc_distance_analysis/audit_binary_rs_encoder.py
```

models the relevant Rust routines:

```text
get_table_additive_binary
evaluate_over_foldable_domain
```

over small binary fields `GF(2^m)`, using the same polynomial-basis elements:

```text
basis[i] = 1 << i.
```

It checks:

```text
1. generator rank;
2. exhaustive k-subset rank defects for small k,n;
3. row-space equality with ordinary Vandermonde RS on the obvious additive-subspace domain,
   in both natural and bit-reversed column order.
```

This is an algebra audit, not a performance benchmark.

## Results

Command:

```text
python scripts/rfc_distance_analysis/audit_binary_rs_encoder.py \
  --m 6 --num-vars 1 --num-vars 2 --log-rate 1 --log-rate 2
```

Output:

```text
m,num_vars,log_rate,k,n,rank,checked_k_subsets,defective_k_subsets,first_bad,rs_matches
6,1,1,2,4,2,6,0,,subspace-natural:True;subspace-bitrev:False
6,1,2,2,8,2,28,0,,subspace-natural:False;subspace-bitrev:False
6,2,1,4,8,4,70,0,,subspace-natural:False;subspace-bitrev:False
6,2,2,4,16,4,1820,32,0:2:4:8,subspace-natural:False;subspace-bitrev:False
```

Command:

```text
python scripts/rfc_distance_analysis/audit_binary_rs_encoder.py \
  --m 7 --num-vars 1 --num-vars 2 --num-vars 3 --log-rate 1
```

Output:

```text
7,1,1,2,4,2,6,0,,subspace-natural:True;subspace-bitrev:False
7,2,1,4,8,4,70,0,,subspace-natural:False;subspace-bitrev:False
7,3,1,8,16,8,12870,32,0:1:2:4:6:8:13:15,subspace-natural:False;subspace-bitrev:False
```

So:

```text
1. The generator has full rank in all tested cases.
2. It is not row-space equal to ordinary RS on the obvious additive subspace, except the trivial
   k=2,n=4 case.
3. It is MDS for tiny cases k=2,n=4; k=2,n=8; k=4,n=8.
4. It fails MDS at k=4,n=16 and k=8,n=16.
```

The first `k=4,n=16` bad subset is:

```text
columns = 0,2,4,8
rank = 3.
```

The restricted generator rows on those columns are:

```text
01 01 01 01
00 01 1c 06
00 08 04 02
00 08 33 0c
```

over the toy `GF(2^6)`.

## Interpretation

The existing `binary_rs` path should be read as:

```text
additive-table / binary-subspace fold-consistent BaseFold encoding
```

not as:

```text
full global additive Reed-Solomon encoding.
```

This matches the code structure. The commit path with `rs_basecode=false` does:

```text
1. interpolate Boolean-hypercube evaluations into multilinear coefficients;
2. repeat each coefficient across the rate block;
3. apply recursive two-point fold transforms using additive binary tables.
```

That preserves the fold identity checked by:

```text
test_binary_rs_transform
```

but it does not make the full length-`N` word a degree-`<k` univariate polynomial evaluated on `N`
distinct additive-domain points.

## Why This Matters

The root-free RS idea remains valid, but it is not already obtained by merely selecting:

```text
code_type = "binary_rs"
```

in the current BaseFold path.

To get MDS distance from RS, the committed word must be exactly:

```text
f(x_1), ..., f(x_N)
```

for a degree-`<k` univariate polynomial `f` and `N` distinct additive-subspace points. The current
path instead appears to be a tensor/BaseFold code with additive-subspace fold parameters.

So the design fork is:

```text
Current binary_rs path:
  useful additive folding machinery, fold-consistent, not full RS/MDS.

New additive-RS backend:
  use subspace-polynomial RS encoding globally, then fold by additive quotient maps.
```

## Next Step

Build a minimal true additive-RS encoder/checker:

```text
1. choose an additive domain D of size N;
2. encode degree-<k univariate polynomials on D;
3. fold by subspace quotient L_beta(X)=X^2+beta X;
4. verify folded word is RS on the quotient domain;
5. compare query path/prover cost against current BaseFold.
```

The existing `binary_rs` tables and `TwiddleAccess` machinery are still directly useful for this.
The missing piece is the global RS encoder/code-family contract.
