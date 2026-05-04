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
