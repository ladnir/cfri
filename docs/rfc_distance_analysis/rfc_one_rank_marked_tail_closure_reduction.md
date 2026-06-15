# One-Rank Marked Tail To Closure Reduction

Status: deterministic matroid reduction for the weak PA-chain route.

## Purpose

The safe multi-PA implication gives a child event:

```text
rank(Q union J)-rank(Q) <= |J|-1.
```

This is a one-rank marked incremental tail. This note shows that such an event can be reduced to a
one-mark closure event after enlarging the core by other marked coordinates from a minimal
dependent subset.

This matters because we already have a closure-tail route. We do not need a fully general marked
rank-tail theorem for this PA-chain endpoint.

## Statement

Let `M` be any matroid represented by a child code. Let:

```text
Q = core set
J = marked set, disjoint from Q
b = |J|
```

If:

```text
rank(Q union J)-rank(Q) <= b-1,
```

then there exist:

```text
empty != B subseteq J
j in B
```

such that:

```text
j in cl(Q union (B \ {j})).
```

Equivalently, the one-rank marked tail is contained in a union of one-mark closure events:

```text
T(Q,J)
  subset union_{empty != B subseteq J} union_{j in B}
    { j in cl(Q union (B\{j})) }.
```

## Proof

If:

```text
rank(Q union J)-rank(Q) < |J|,
```

then `J` is dependent in the contraction matroid `M/Q`.

Choose a circuit `B` of `M/Q` contained in `J`. Then for every:

```text
j in B,
```

the circuit property gives:

```text
j in cl_{M/Q}(B\{j}).
```

Translating closure in the contraction back to `M` gives:

```text
j in cl_M(Q union (B\{j})).
```

This proves the containment.

## Counting Consequence

For a fixed `b=|J|`, union over the size:

```text
m = |B|, 1 <= m <= b.
```

The closure core size becomes:

```text
|Q| + m - 1.
```

Thus a one-rank marked-tail event can be counted by:

```text
sum_{m=1}^b binom(b,m) m * C_child(|Q|+m-1, 1)
```

times the lift/placement factors for the unchosen marked coordinates.

In the PA-chain target family from the weak scale diagnostic:

```text
|Q| = y = p-F
b = F.
```

The largest closure core occurs at:

```text
|Q|+b-1 = p-1.
```

So the weakest one-mark closure exponent is:

```text
k_child - (p-1).
```

For the target:

```text
k_child = 512
p = 137
```

this is:

```text
512 - 136 = 376.
```

This exactly matches the stable `376` q-exponent observed in:

```text
rfc_pa_chain_weak_rank_scale.py
```

for `F in {1,4,16,72}`.

## Meaning For The Proof

The weak PA-chain route can now be organized as:

```text
parent PA closure with b marks
  -> child one-rank marked tail on (Q,J)
  -> minimal circuit B inside J over Q
  -> one-mark child closure with enlarged core Q union (B\{j}).
```

This avoids needing a new general marked-rank theorem for the endpoint. The remaining theorem is a
one-mark closure-tail bound with variable core size:

```text
C_d(p,1)
```

for `p` up to about `136` in the target child layer, plus lift factors for choosing the circuit and
unused marked coordinates.

## Caveat

This reduction is a union bound over circuits/minimal dependent subsets. It may lose factors:

```text
sum_m binom(b,m) m.
```

For the target `b <= 72`, this is at most:

```text
b 2^b
```

which is tiny compared with the `376 * 128` bits of q-exponent. It should fit comfortably inside
the proof constants.

## Next Step

Replace the random one-rank charge in:

```text
rfc_pa_chain_weak_rank_scale.py
```

by the closure-reduction union bound:

```text
one-rank tail <= circuit union over one-mark closures.
```

This would make the weak PA-chain diagnostic closer to a theorem-grade recurrence using only the
closure-tail primitive.
