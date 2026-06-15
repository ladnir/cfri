# RFC Replica Root Compatibility

This note isolates the local fact that the near-MDS first-moment proof needs next.

## Setup

At one RFC parent coordinate, write the two child values of one message as:

```text
A, B in F.
```

The two parent outputs are:

```text
Y_0 = A + T(B-A)
Y_1 = B + T(B-A)
```

where `T` is uniform in `F^*`.

For an ordered `r`-tuple of parent messages, collect the child values into vectors:

```text
X = (A_1, ..., A_r)
Y = (B_1, ..., B_r).
```

Selecting one singleton output asks for:

```text
X + T(Y-X) = 0.
```

## Local Lemma

For fixed `X,Y in F^r`:

```text
Pr_T[ X + T(Y-X) = 0 ]
```

is:

```text
1          if X=Y=0,
<=1/(q-1) if dim span(X,Y)=1 and X,Y are not both zero,
0          if dim span(X,Y)=2.
```

The middle case is sometimes also zero because the unique root may be `0`, which is not sampled, or
because the selected side is the incompatible endpoint. The upper bound `1/(q-1)` is all the
first-moment proof needs.

## Proof

If `X=Y=0`, the equation is identically true.

Otherwise, suppose a solution `T` exists. Then:

```text
X = -T(Y-X).
```

So `X` is a scalar multiple of `Y-X`, and therefore `X` and `Y` are linearly dependent. Hence
`dim span(X,Y) <= 1`.

If `X` and `Y` are linearly dependent and not both zero, the equation has at most one solution
`T`, because two distinct solutions would imply `Y-X=0` and then `X=0`. Thus the probability is at
most `1/(q-1)`.

If `dim span(X,Y)=2`, no solution exists.

## Aggregate Count

The generic saving is stronger than the pointwise `1/(q-1)` root probability suggests. For one
coordinate:

```text
sum_{X,Y in F^r} Pr_T[ X + T(Y-X)=0 ] = q^r.
```

Reason: for each fixed `T`, the map

```text
(X,Y) -> X + T(Y-X)
```

has rank `r` from `F^(2r)` to `F^r`, so its kernel has size `q^r`. Averaging over `T in F^*` keeps
the same count.

Thus a generic singleton coordinate costs `r` field equations in the aggregate `r`-replica moment,
not one. A recurrence that tracks only whether `X=Y=0` loses the rank-one compatibility condition
and overcounts by about `q^(r-1)` per generic singleton.

## Exterior Form

For `r=2`, root compatibility is the single exterior equation:

```text
X_1 Y_2 - X_2 Y_1 = 0.
```

For general `r`, root compatibility is the vanishing of all `2 x 2` minors of the `2 x r` matrix:

```text
[ X ]
[ Y ].
```

This is the algebraic state that the global proof must preserve. Common-zero coordinates are the
rank-`0` part. Root-compatible singleton coordinates are the rank-`1` part. Rank-`2` coordinates
cannot satisfy a singleton request.

## Exact r=2 Local Counts

For `r=2`, take `X,Y` uniformly in `F^2`. The local categories have closed-form counts:

```text
rank0:             X=Y=0
                   1

rank1, no root:    X=Y != 0
                   q^2 - 1

rank1, one root:   exactly one of X,Y is zero
                   2(q^2 - 1)

rank1, two roots:  X,Y nonzero, distinct, and collinear
                   (q+1)(q-1)(q-2)

rank2:             linearly independent
                   q^4 - (q+1)(q^2-1) - 1
```

For `q=5`, these are:

```text
category          count out of 625
rank0             1
rank1_no_root     24
rank1_one_root    48
rank1_two_roots   72
rank2             480
```

So about `76.8%` of generic singleton coordinates are rank-`2` and cannot satisfy a two-replica
singleton request at all. This is the main exponent missing from the loose common-zero-only
recurrence.

The profiler:

```text
scripts/rfc_distance_analysis/rfc_replica_rank1_profile.py
```

checks this in two regimes.

Exact small run:

```text
python scripts/rfc_distance_analysis/rfc_replica_rank1_profile.py \
  --prime 5 \
  --child-depth 1 \
  --expansion 4 \
  --max-singletons 8 \
  --seed 11
```

Saved outputs:

```text
docs/rfc_distance_analysis/rfc_replica_rank1_exact_gf5_child_depth1_c4_categories.csv
docs/rfc_distance_analysis/rfc_replica_rank1_exact_gf5_child_depth1_c4_singletons.csv
```

The exact category rates are:

```text
rank0             0.00159744408946
rank1_no_root     0.0384000983043
rank1_one_root    0.0768001966085
rank1_two_roots   0.115200294913
rank2             0.768001966085
```

The small deviation from the exact `1/625,24/625,...` values is only from excluding the all-zero
ordered two-replica parent tuple.

GF(5), depth-2 child / depth-3 parent sampled run:

```text
python scripts/rfc_distance_analysis/rfc_replica_rank1_profile.py \
  --prime 5 \
  --child-depth 2 \
  --expansion 8 \
  --max-singletons 16 \
  --tuple-samples 100000 \
  --seed 29
```

Saved outputs:

```text
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_categories.csv
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_singletons.csv
```

The sampled category rates are essentially the same:

```text
rank0             0.0015996875
rank1_no_root     0.0381875
rank1_one_root    0.0769096875
rank1_two_roots   0.115181875
rank2             0.76812125
```

For oriented singleton sets of size `s`, the exact elementary-symmetric sum is much smaller than
the loose common-zero-only bound. At the depth-2 child calibration:

```text
s=4:   loose/exact ~= 2^9.44
s=8:   loose/exact ~= 2^15.52
s=12:  loose/exact ~= 2^20.39
s=16:  loose/exact ~= 2^26.09
```

This confirms the diagnosis from `rfc_replica_zero_moment.py`: the loose recurrence fails because
it throws away the rank-one compatibility equation, not because the first-moment approach is
inherently too weak.

## Consequence For The Proof Plan

The loose replica recurrence in:

```text
scripts/rfc_distance_analysis/rfc_replica_zero_moment.py
```

is intentionally not a certificate. Its trace shows the exact missing factor: it charges only one
root equation for a generic singleton even when the replica dimension is larger.

The next theorem should bound finite-replica request patterns with coordinate labels:

```text
rank 0: common zero
rank 1: root-compatible singleton
rank 2: impossible for singleton requests
```

For `r=1`, every non-common singleton is automatically rank `1`, so the theorem reduces to the
ordinary line-count first moment. For `r>1`, the exterior/minor equations are the extra constraints
needed to recover the near-random first-moment exponent.

For `r=2`, the multi-coordinate exterior count is now tracked in:

```text
docs/rfc_distance_analysis/rfc_exterior_component_codimension.md
```

The key correction is now endpoint-aware. For exact support `A`, the component/full-rank endpoint
is only one endpoint; dense supports also have a generic two-copy kernel endpoint. The tau-2
root-weight exponent is:

```text
max(2g(A)-4, 2delta(A)+comp(A)-4-|A|).
```

This is recorded in:

```text
docs/rfc_distance_analysis/rfc_tau2_weighted_exterior_bound.md
```
