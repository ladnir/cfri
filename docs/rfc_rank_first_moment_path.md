# RFC Rank First-Moment Path

This note records an alternate but compatible proof view.

## Rank Formulation

For a linear generator matrix `G` with `k` message rows and `N` coordinate columns, a codeword has
weight at most `D` iff it is zero on some coordinate set `Z` with:

```text
|Z| = N - D.
```

The set `Z` can support a nonzero codeword only if the selected coordinate columns do not span the
message space:

```text
rank(G_Z) < k.
```

Thus distance can be certified by proving that all sufficiently large zero sets have full rank. A
first-moment version is:

```text
E[# Z, |Z|=z, rank(G_Z) < k] < 2^-lambda.
```

For the systematic code, `G = [I | P]`. If `Z` contains `s` systematic columns, those constraints
force `s` message coordinates to zero, and the parity columns only need to span the remaining
`k-s` dimensions. This is the nontrivial way to use the systematic block; it is not the crude
argument that simply discards it.

## Why This Could Be Powerful

For a fully random large-field generator, a fixed `z`-column submatrix has rank deficiency
probability about:

```text
q^-(z-k+1)
```

when `z >= k`. This is the tail that produces near-Singleton first-moment distances at `q=2^128`.

If RFC coordinate columns have a comparable rank-deficiency tail, then the systematic affine RFC
should certify close to the ideal random-parity baseline. This would explain why the threshold
certificate is pessimistic: it proves a recursive distance invariant instead of directly measuring
rank deficiency of final zero sets.

## What The Tiny Probe Says

The script:

```text
python scripts/sample_rfc_rank_failure.py \
  --prime 5 \
  --depth 3 \
  --total-expansion 8 \
  --zero-count 47 \
  --code-samples 20 \
  --subset-samples 1000 \
  --seed 41
```

and the systematic variant at `zero_count=44` both saw no rank failures in `20000` random zero-set
samples. This does not mean the rank-deficient sets are absent; the product spectrum shows they are
rare among all subsets.

Using:

```text
python scripts/zero_set_union_from_spectrum.py \
  docs/sample_product_first_moment_gf5_depth3_c8_cpp.csv \
  --total-n 64 \
  --zero-count 47
```

gives:

```text
original z=47:
  low-word first moment:       1.68677128408
  zero-set union moment:       398940.68684 ~= 2^18.61

systematic z=44:
  low-word first moment:       1.54962178218
  zero-set union moment:       113173.044659 ~= 2^16.79
```

So random zero-set sampling is blind because the bad sets are sparse in the ambient subset space.
The rank viewpoint is still useful, but only if we prove an analytic rank-deficiency tail for fixed
zero-set shapes.

## MDS Check

The same script has a small exact `k`-subset mode. At `GF(65537)`, depth 2, `c=8`, it checked all
`35960` subsets of `k=4` columns:

```text
original:
  rank-deficient k-subsets: 0

systematic:
  rank-deficient k-subsets: 28
  first bad subset: 0:1:4:18
```

So the original stacked RFC looks generically MDS in this tiny test, but the systematic generator
does not. This does not kill the rank route: distance at the target parameters only needs large
zero sets of size `z > k` to span, and systematic identity columns should be conditioned on rather
than treated as random parity columns. But it does rule out the simplest possible proof statement
that every `k` final coordinates are independent.

## Proof Obligation

A strong certificate would prove something like:

```text
Pr[rank(G_Z) < k] <= poly(N, d, c) q^-(z-k+1)
```

for every final zero-set shape `Z`, with the systematic version conditioning on the number of
identity columns in `Z`.

This can then be summed over zero-set shapes, or combined with the sharper codeword/product
first-moment recurrence. The central algebraic question becomes:

```text
Does every selected RFC column set of size z >= k have generic rank k,
and can rank loss r be charged q^-r-like probability?
```

This is a cleaner full-proof target than the old threshold-distance recurrence. It also gives a
direct place to use the fact that `T` is uniform nonzero over a very large field.
