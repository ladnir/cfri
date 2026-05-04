# RFC Column Tensor Structure

This note expands the algebra behind the rank-first-moment path.

## Column Formula

Start from one expansion copy, whose depth-0 column is:

```text
1
```

At each fold level, an existing column vector `v` is split using a fresh nonzero challenge `T`:

```text
left child:   [ (1-T) v ]
              [   T   v ]

right child:  [  -T    v ]
              [ (T+1) v ]
```

Equivalently, the two local two-entry vectors are:

```text
u_0(T) = (1-T, T)
u_1(T) = (-T, T+1).
```

Thus a final parity column is a tensor product along a root-to-leaf path:

```text
C_{a,b_1,...,b_d}
  = u_{b_d}(T_{a,b_1,...,b_{d-1}})
    tensor ...
    tensor u_{b_2}(T_{a,b_1})
    tensor u_{b_1}(T_a),
```

where `a` is the initial expansion-copy index. The exact bit order above follows the generator's
column order and is decoded least-significant-bit first by `scripts/describe_rfc_columns.py`; the
main invariant is that flipping one tree bit selects the sibling column at that level.

For total systematic expansion `c`, the parity block has `c-1` independent initial copies:

```text
a in {0, ..., c-2}.
```

The systematic columns are the standard basis vectors indexed by leaf paths.

## Why Sibling Obstructions Appear

At one local node:

```text
u_0(T) - u_1(T) = (1, -1).
```

The sibling difference is independent of `T`. Therefore, when a zero-set includes systematic
columns that pin an entire sibling pair of message coordinates, selecting parity sibling columns
can leave fewer independent constraints than a generic random matrix would.

This is exactly what the depth-2 bad shapes show:

```text
identity pair:  one complete sibling pair
parity pair:    same expansion copy and same lower path, flipped at the top level
```

The same mechanism recurs at deeper levels, where parity columns collide after reducing modulo
subtree block sizes:

```text
depth 3 parity offsets: 28, 14, 7
depth 4 parity offsets: 56, 28, 14, 7
```

The rank defects seen in the shape profiles line up with these recursive sibling collisions.

For example:

```text
python scripts/describe_rfc_columns.py \
  --systematic \
  --depth 2 \
  --parity-expansion 7 \
  --columns 0:1:4:18
```

prints:

```text
I(path=00,col=0)
I(path=10,col=1)
P(copy=0,path=00,local=0)
P(copy=0,path=01,local=14)
```

and a depth-3 bad shape begins:

```text
I(path=000), I(path=100), I(path=010), I(path=110), I(path=001), I(path=101)
P(copy=0,path=000), P(copy=0,path=010), P(copy=0,path=001)
```

The path labels make the obstruction visible: the systematic coordinates include complete sibling
pairs, while the parity coordinates reuse the same expansion copy and collide under low-level path
projections.

## Proof Target In Tree Language

For a systematic zero set `Z`, split it into:

```text
S = selected systematic leaf paths
P = selected parity columns (expansion copy + leaf path)
```

The selected systematic columns reveal/pin the coordinates in `S`. Rank deficiency of `[I | P]_Z`
is equivalent to rank deficiency of the parity columns after quotienting out the span of `S`.

A structural proof can therefore aim to show:

```text
rank(I_S, P_Z) >= min(k, |S| + tree_rank(P_Z mod S)),
```

where `tree_rank` is controlled by recursive separation of parity paths. Deficiency should only
occur when many selected parity paths collide under the same recursive projections already pinned
by systematic sibling subtrees.

The empirical bad-shape signatures suggest a combinatorial upper bound of the form:

```text
rank deficiency <= recursive_collision_score(S, P).
```

The current best diagnostic is to form the restricted parity matrix:

```text
P_Z restricted to rows [k] \ S.
```

The exact rank condition becomes:

```text
rank(I_S, P_Z) = |S| + rank(P_Z | [k]\S).
```

So structural defects are precisely shapes where the selected parity tensor columns lose rank after
the identity rows are quotiented away. In depth-3 examples:

```text
pure structural bad:
  remaining rows:      2
  selected parity:     3
  restricted rank:     1
  projective classes:  1

sporadic finite-field bad:
  remaining rows:      3
  selected parity:     3
  restricted rank:     2 for one seed
  projective classes:  3
  resamples to rank 3
```

The first case is a deterministic quotient collapse. The second is an accidental determinant zero.

If this score is zero for all `|Z| >= k + e`, then the large-field structural distance is at least:

```text
N - (k + e) + 1.
```

If the score is nonzero but forces nonzero determinant polynomials of degree `D_poly`, the
large-field first-moment version should charge it by a Schwartz-Zippel factor:

```text
D_poly / q
```

or by higher powers of `1/q` when several independent rank losses are needed.

## Immediate Lemma Candidates

The data currently supports these candidate statements:

```text
original RFC:
  every k final columns are generically independent.

systematic RFC:
  every k+2 final columns appear generically independent in sampled depths 3 and 4;
  depth 2 already needs only k+1.
```

The original statement would give MDS distance for the non-systematic RFC. The systematic statement
would give near-MDS distance with a very small additive loss, provided it can be proved uniformly
over the recursion depth.
