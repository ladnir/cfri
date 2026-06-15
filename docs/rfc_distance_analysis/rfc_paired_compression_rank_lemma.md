# RFC Paired Compression Rank Lemma

Status: local theorem for the fixed-survivor rank-tail route.

This proves the paired-coordinate identity observed by the diagnostics:

```text
rank(parent restricted to paired coordinates over P)
  = 2 rank(child restricted to P).
```

## Setup

At one RFC fold, the two parent halves are encoded from two child codewords `u` and `w`. For a child
coordinate `j`, the two sibling symbols are:

```text
left_j  = u_j + t_j w_j
right_j = u_j + (t_j + 1) w_j
```

with `t_j` sampled from `F_q^*`.

For a paired survivor coordinate, both siblings survive. Thus at each paired child position `j`,
the parent exposes:

```text
(left_j, right_j).
```

## Invertible Local Transform

The map from child-pair symbols `(u_j,w_j)` to sibling symbols `(left_j,right_j)` is:

```text
[ left_j  ]   [ 1  t_j     ] [ u_j ]
[ right_j ] = [ 1  t_j + 1 ] [ w_j ].
```

The determinant is:

```text
(t_j + 1) - t_j = 1.
```

So this map is invertible for every field and every `t_j`, including characteristic two. No
`T'=-T` trick is used.

Therefore, on a paired set `P`, the parent restricted to both siblings over `P` is row/column
equivalent to two independent copies of the child restriction on `P`.

## Rank Identity

Let:

```text
R_child(P) = rank(child generator restricted to P).
```

The paired parent block has one copy of the child restriction for `u` and one copy for `w`, followed
by an invertible block-diagonal coordinate transform over the paired coordinates. Hence:

```text
rank(parent paired block over P) = 2 R_child(P).
```

This identity is deterministic. It does not require randomness, genericity, or a large field.

## Consequence

For a survivor set `S`, let:

```text
P = top child positions where both siblings survive,
T = top child positions where exactly one sibling survives.
```

Let:

```text
D = k_child - R_child(P).
```

The paired block contributes:

```text
2(k_child - D)
```

rank. The parent dimension is:

```text
k_parent = 2 k_child.
```

So singleton coordinates must add at least:

```text
2D
```

rank modulo the paired block. Parent full rank is equivalent to:

```text
singleton_increment(P,T) >= 2D.
```

This is the structural reason the fixed-survivor recurrence separates into:

```text
paired child rank tail
plus
root-line singleton repair.
```
