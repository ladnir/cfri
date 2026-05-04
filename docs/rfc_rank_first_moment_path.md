# RFC Rank First-Moment Path

This note records an alternate but compatible proof view.

The column-level tree algebra is expanded in:

```text
docs/rfc_column_tensor_structure.md
```

Small bad-shape examples can be decoded with:

```text
python scripts/describe_rfc_columns.py --systematic --depth 3 --parity-expansion 7 --columns 0:1:2:3:4:5:8:22:36
```

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

The aggregate rank profile is sharper:

```text
original:
  z=0,1,2,3: all deficient for dimension reasons
  z=4:       0 deficient among 35960

systematic:
  z=0,1,2,3: all deficient for dimension reasons
  z=4:       28 deficient among 35960
  z=5:       0 deficient among 201376
```

Thus, in this large-prime depth-2 check, the original RFC is MDS while the systematic RFC is
one-symbol below MDS:

```text
original distance:    29 / 32
systematic distance:  28 / 32
```

This is much better than the threshold certificate shape and suggests the real large-field
systematic tax may be small.

The systematic shape profile explains the one-symbol loss. For `s_identity` selected identity
columns and `z_parity` selected parity columns:

```text
s_identity  first z_parity with no deficient shapes
0           4
1           3
2           3    (not 2: there are 28 bad shapes)
3           1
4           0
```

The only extra obstruction beyond the dimension count is the `s=2, z_p=2` slice. Once one more
parity column is added, every shape spans.

The bad `s=2, z_p=2` shapes are structured, not noisy. They are exactly:

```text
identity pair:  (0,1) or (2,3)
parity pair:    (j, j+14), for 0 <= j < 14
```

in parity-local indexing. This is a sibling/tree-half obstruction: after two systematic coordinates
pin a depth-1 sibling pair, the two parity columns selected from matching positions in the two
top-level halves do not give two independent remaining constraints. This is the first concrete
shape the proof has to account for.

## Depth-3 Shape Sampling

Exact shape enumeration is too large at depth 3 (`k=8`, `N=64`, parity length `56`), so the script
also has a stratified sampler:

```text
python scripts/sample_rfc_rank_failure.py \
  --systematic \
  --systematic-shape-sample-profile \
  --prime 65537 \
  --depth 3 \
  --total-expansion 8 \
  --zero-count 1 \
  --seed 223 \
  --shape-samples 2000 \
  --shape-extra-max 2
```

For each `s_identity`, it samples around the dimension threshold `z_parity = k - s_identity`.
The results are:

```text
total zero count 7: all sampled shapes deficient, as expected since z < k
total zero count 8: rare structured deficiencies for s=3,4,5,6
total zero count 9: very rare sampled deficiencies
total zero count 10: no sampled deficiencies
```

This suggests a possible pattern:

```text
depth 2: systematic large-field distance appears Singleton - 1
depth 3: sampled obstruction appears to disappear by Singleton - 2
```

That is only a hypothesis, not a proof. But if the true structural loss is `O(depth)`, the target
parameters would still be close to the ideal random-parity first-moment distance and far above the
current threshold certificate.

Some depth-3 rows are still small enough to enumerate exactly:

```text
s_identity  z_parity  total z  checked   deficient
6           2         8        43120     560
5           3         8        1552320   13027
6           3         9        776160    224
```

The exact `s=6,z_p=3` row confirms that deficiencies persist at total zero count `k+1`, so the
depth-3 systematic distance is at most Singleton minus two in this sampled large-prime instance.
The bad shapes are still tree-structured. For example, in `s=6,z_p=3`, all 224 bad shapes have:

```text
identity_pairs = 3
identity_quads = 1
parity_half_pairs = 1
parity_mod_quarter_collisions = 2
```

So the obstruction is not arbitrary rank failure; it is tied to selecting many systematic sibling
pairs together with parity columns that collide under recursive halves/quarters.

The distinction between structural defects and accidental finite-field zeros is visible by
resampling selected shapes:

```text
python scripts/resample_rank_shapes.py \
  --depth 3 \
  --total-expansion 8 \
  --prime 65537 \
  --seed-start 230 \
  --samples 20 \
  --shape sporadic1=0:1:2:4:7:13:15:32 \
  --shape pure_bad=0:1:2:3:4:8:9:36 \
  --shape pure_good=0:1:2:4:6:8:9:36 \
  --out docs/rank_resample_shapes_systematic_p65537_depth3_c8.csv
```

The structural bad shape stayed rank `7` in all 20 samples. The sporadic bad shapes from the first
depth-3 row run became rank `8` in all 20 samples, as did the pure-good control. Thus the proof
should separate:

```text
structural tree defects: counted combinatorially
accidental determinant zeros: charged by large-field Schwartz-Zippel
```

Resampling every bad shape in the three exact depth-3 rows gives the same split:

```text
row s=6, z_p=2:  560 / 560 bad shapes persistent across 20 samples
row s=6, z_p=3:  224 / 224 bad shapes persistent across 20 samples
row s=5, z_p=3:  12992 / 13027 bad shapes persistent across 20 samples
                 35 / 13027 disappeared in every resample
```

The persistent counts exactly match the pure tree-signature buckets in
`docs/rank_signature_row_*`. The 35 disappearing cases are best treated as accidental zeros of
nonzero determinant polynomials in the large-field proof.

The resulting structural summary for the exact depth-3 rows is:

```text
s_identity  z_parity  total z  checked   persistent structural  structural fraction
6           2         8        43120     560                    0.0129870
5           3         8        1552320   12992                  0.0083694
6           3         9        776160    224                    0.0002886
```

This is the first concrete combinatorial object to bound: persistent structural defects are sparse
and described by recursive sibling/collision signatures. Accidental failures are much rarer and
should vanish at `q=2^128` after a Schwartz-Zippel union bound.

## Depth-4 Shape Sampling

A light depth-4 pass used `1000` stratified shapes per row around the dimension boundary:

```text
python scripts/sample_rfc_rank_failure.py \
  --systematic \
  --systematic-shape-sample-profile \
  --prime 65537 \
  --depth 4 \
  --total-expansion 8 \
  --zero-count 1 \
  --seed 307 \
  --shape-samples 1000 \
  --shape-extra-max 3
```

Aggregating by total zero count:

```text
total z  sampled failures / sampled shapes
15       16000 / 16000    (dimension-deficient, since z < k=16)
16       25 / 17000
17       1 / 17000
18       0 / 17000
19       0 / 17000
```

This is only a sampled check, but it is consistent with the depth-3 picture: the observed
systematic obstruction is concentrated near `z=k` and `z=k+1`, and no sampled failures appeared by
`z=k+2`. If this persists, the systematic distance loss may be only a couple of symbols, not a
growing fraction of block length.

For comparison, a non-systematic depth-4 sample at `z=k=16` saw no rank failures:

```text
python scripts/sample_rfc_rank_failure.py \
  --prime 65537 \
  --depth 4 \
  --total-expansion 8 \
  --zero-count 16 \
  --code-samples 5 \
  --subset-samples 5000 \
  --seed 401
```

The result was `0 / 25000` failures, with minimum sampled rank `16`. This is not a proof of the
original MDS claim, but it continues to support the split:

```text
original RFC:     no visible structural obstruction
systematic RFC:   visible obstruction caused by identity/tree collisions
```

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
