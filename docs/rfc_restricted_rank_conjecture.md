# RFC Restricted-Rank Conjecture

This note states the current target lemma suggested by the rank experiments.

## Setup

Let `P_d` be the parity generator for systematic RFC at depth `d` and parity expansion `e`.
Rows are indexed by leaf paths `x in {0,1}^d`. Parity columns are indexed by:

```text
(a, y) in [e] x {0,1}^d
```

where `a` is the expansion copy and `y` is the parity path.

For a systematic zero set:

```text
S subset {0,1}^d
Q subset [e] x {0,1}^d
```

define the restricted parity matrix:

```text
M_d(S,Q) = P_d[rows {0,1}^d \ S, columns Q].
```

Then:

```text
rank([I_S | P_Q]) = |S| + rank(M_d(S,Q)).
```

So the systematic rank certificate reduces to proving:

```text
rank(M_d(S,Q)) = 2^d - |S|
```

for every zero-set shape above the target size.

## Recursive Collision Family

For each subtree `U`, write:

```text
S_U = S cap U
Q_U = parity paths in Q whose projection enters U
```

The observed structural defects arise when:

```text
1. S contains complete sibling subtrees; and
2. Q contains parity columns that collide after projection to the same quotient subtree; and
3. the colliding parity columns reuse the same expansion copy or lower-prefix randomness.
```

Informally, the systematic rows delete enough coordinates that two or more tensor columns become
dependent in the quotient.

## Candidate Lemma

There should exist a recursively defined collision score:

```text
rho_d(S,Q) >= 0
```

with these properties:

```text
rho_d(S,Q) = 0
  => M_d(S,Q) has generic rank min(|Q|, 2^d - |S|)

rho_d(S,Q) = r
  => generic rank loss of M_d(S,Q) is at most r
```

The level-aware approximation currently tested is:

```text
I_l(S) = # complete selected systematic subtrees of size 2^l
P_l(Q) = # parity path collisions under level-l projection
```

but the final `rho_d` likely needs the full recursive placement of those subtrees, not just their
level totals.

## Evidence

Depth 2 exact:

```text
s=2,z_p=2:
  one complete identity sibling subtree + one parity level-1 collision
  => 28/28 bad
```

Depth 3 exact rows:

```text
s=6,z_p=2: persistent structural bad = 560
s=5,z_p=3: persistent structural bad = 12992, accidental = 35
s=6,z_p=3: persistent structural bad = 224
```

For the exact `s=6,z_p=3` row, the level-aware feature bucket:

```text
I_1=3, I_2=1, P_1=2, P_2=1
```

is exactly pure bad:

```text
224 / 224.
```

All other feature buckets in that row are clean.

## Distance Consequence

If one proves:

```text
rho_d(S,Q) = 0
```

for all:

```text
|S| + |Q| >= 2^d + e
```

then, ignoring accidental finite-field zeros:

```text
distance >= N - (2^d + e) + 1.
```

The current samples suggest `e=2` may hold at depths 3 and 4 for systematic expansion `7`, while
the original non-systematic RFC may have `e=0`.

## Next Proof Step

Work recursively on the root split:

```text
S = S_0 union S_1
Q = Q_0 union Q_1 union Q_cross
```

where `Q_0,Q_1` are parity columns separated by the top path bit and `Q_cross` are projection
collisions that survive after deleting complete systematic subtrees. The proof should show that if
both child restrictions have full generic rank and no cross-collision survives, then the parent
restriction also has full generic rank.

The local determinant-`1` sibling split and the current recursive diagnostics are developed in:

```text
docs/rfc_restricted_rank_induction.md
```
