# RFC Product First-Moment Recurrence

This note records the proof structure suggested by the product sampler.

## Product Target

For one RFC tree at depth `d`, write `G_m(x)` for the output-weight generating polynomial of a
fixed nonzero message `m`:

```text
G_m(x) = E_T x^wt(A_T m).
```

For total expansion `c`, the original non-systematic first moment is:

```text
sum_{m != 0} [x^<=D] G_m(x)^c.
```

For the systematic affine code with parity expansion `c-1`, it is:

```text
sum_{m != 0} [x^<=D] x^wt(m) G_m(x)^(c-1).
```

So the expansion itself is not the hard part. The hard part is proving an upper bound on the
single-tree laws `G_m` that is tight enough after taking the product.

## One-Step Law

For a parent message `(l,r)`, let a sampled child tree produce:

```text
L = A l
R = A r
D = R - L.
```

For one coordinate, the fresh parent fold is:

```text
Y_0 = L + T D
Y_1 = L + (T+1)D
```

with `T` uniform in `F^*`. The local output-weight polynomial is:

```text
common zero:       1
equal nonzero:     x^2
single-root:       (x + (q-2)x^2)/(q-1)
double-root:       (2x + (q-3)x^2)/(q-1)
```

Thus if the child pair has category counts `(z,e,a,b)`, the parent one-tree law contribution is:

```text
Phi_{z,e,a,b}(x)
  = x^(2e)
    ((x + (q-2)x^2)/(q-1))^a
    ((2x + (q-3)x^2)/(q-1))^b.
```

Common zeros contribute no weight. The exact parent single-tree law is:

```text
G_(l,r)(x) = E_child_tree Phi_{cat(A l, A r)}(x).
```

This is the same local algebra as the one-step sampler, now phrased for a single tree.

For a finite-replica first moment, the same local algebra has one extra condition. For `r` replicas,
write the two child value vectors at one coordinate as:

```text
X = (A_1, ..., A_r)
Y = (B_1, ..., B_r).
```

All `r` selected singleton outputs can be zero for one common root challenge only if:

```text
dim span(X,Y) <= 1.
```

If `X=Y=0`, the coordinate is a common zero. If the span has dimension `1`, the coordinate is
root-compatible and costs one root challenge. If the span has dimension `2`, the singleton request is
impossible. For `r=2`, root compatibility is the single exterior equation:

```text
X_1 Y_2 - X_2 Y_1 = 0.
```

This rank-one condition is the missing savings in any recurrence that tracks only common-zero
coordinates. It is the main local state needed by a proof-strength finite-replica recurrence.

The first `r=2` diagnostic is:

```text
scripts/rfc_distance_analysis/rfc_replica_rank1_profile.py
```

For `GF(5)`, child depth `2`, expansion `8`, the sampled two-replica profile gives the expected
random local category rates and shows that rank-one compatibility improves the oriented-singleton
sum by `15.52` bits at `s=8` and `26.09` bits at `s=16` compared to the loose common-zero-only
bound. See:

```text
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_categories.csv
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_singletons.csv
```

The next refinement is endpoint-aware. For a singleton exact support `A` in the child matroid, the
rank-one/root-line count has a generic two-copy kernel endpoint and a component/full-rank endpoint.
With:

```text
delta = dim U_A
g     = generic two-copy root-line kernel dimension
comp  = comp(U_A),
```

the tau-2 root-weight exponent is:

```text
max(2g-4, 2delta+comp-4-|A|).
```

This replaces the older component-only correction. The evidence and proof target are in:

```text
docs/rfc_distance_analysis/rfc_exterior_component_codimension.md
```

A scalar endpoint-aware recurrence is still too coarse unless it also tracks replica span dimension.
The high-replica overcount and proposed span state are recorded in:

```text
docs/rfc_distance_analysis/rfc_replica_span_state.md
```

## Why Pair State Is Not Closed

To know `G_(l,r)`, we need the category distribution of `(A l, A r)`. To recurse that pair
distribution one more layer exactly, we must know joint relations among four grandchildren:

```text
l_0, l_1, r_0, r_1.
```

This is the replica hierarchy seen earlier. The product formulation does not remove it; it makes
the hierarchy finite and tied to the expansion moment we actually need.

## Finite-Replica Route

For expansion `c`, the first moment involves `c` independent single trees evaluated on the same
message. Instead of trying to certify every `G_m` individually, we can certify the coefficient of
`G_m(x)^c` directly.

That suggests a finite `c`-replica recurrence. At each coordinate we track the vector of values
across the `c` independent trees:

```text
(A_1 m[j], A_2 m[j], ..., A_c m[j]).
```

For the systematic code, the parity side uses `c-1` replicas and the systematic support is added as
a deterministic shift.

The local transition then acts coordinatewise on `c` independent fresh fold challenges. The exact
state is an equality/zero pattern of the replicated left and right value vectors. Over a large
field, most nonzero accidental equalities are expensive, so the certificate should be able to
dominate the exact pattern state by a compressed zero/root-capability state.

This is likely the right proof architecture:

```text
finite replica first moment
  -> compressed zero/root pattern upper bound
  -> expansion product coefficient
  -> final Markov certificate
```

It is closer to a direct first-moment proof than to the original threshold-distance recurrence.

There is also a compatible rank-first-moment view in:

```text
docs/rfc_distance_analysis/rfc_rank_first_moment_path.md
```

The rank view asks for an analytic tail bound on final zero-set column-rank deficiency. It may be
the cleanest way to formalize the large-field advantage, while the product/replica view remains the
sharper way to count actual low-weight codewords.

## Immediate Concrete Test

The compressed state should first reproduce the GF(5), depth-3 product sampler crossings:

```text
original:    17 / 64
systematic: 20 / 64
```

The support profile to preserve is:

```text
original low tail:    mostly support 8, then support 4
systematic low tail:  mostly support 4, then supports 2 and 8
```

If a proposed recurrence loses this support split, it is probably too coarse.
