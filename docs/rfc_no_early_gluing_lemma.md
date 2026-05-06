# RFC No-Early-Gluing Lemma

This note isolates the algebraic obstruction that should complete exact stability.

## Setup

At a parent node, suppose both child messages are nonzero and exact:

```text
u = A x_0
v = A x_1.
```

Equality in the uncertainty proof forces:

```text
supp(u) = supp(v) = W'
```

and the parent output support contains exactly one sibling over each coordinate of `W'`.

For each `j in W'`, the parent local outputs are:

```text
y_{0,j} = (1-T_j) u_j + T_j v_j
y_{1,j} = -T_j u_j + (T_j+1) v_j.
```

To kill the left sibling:

```text
v_j / u_j = -(1-T_j)/T_j.
```

To kill the right sibling:

```text
v_j / u_j = T_j/(T_j+1).
```

The fresh variables `T_j` are independent across `j`.

## Lemma Target

If:

```text
|W'| > 1,
```

then the two-child equality branch cannot occur generically.

Reason: after fixing the two child kernel lines, the ratio `v_j/u_j` across `W'` is determined up to
one global scalar between the two child messages. But the required ratios above involve independent
fresh parent challenges for each `j`. One scalar cannot satisfy two independent rational conditions
unless a nontrivial algebraic coincidence occurs, which is excluded over the rational function
field.

Therefore any exact extremizer must use the one-child branch until:

```text
|W'| = 1.
```

This is exactly the node whose size equals the live support size `m`.

## Consequence

The equality recursion has forced form:

```text
1. descend through one child while the current node is larger than m;
2. once the live rows fill the current node, glue sibling child extremizers;
3. repeat the glue step recursively inside the selected output coordinate.
```

This yields:

```text
R = contiguous block of size m
W = stride class modulo m.
```

Together with the matched-kernel construction, this gives the exact stability theorem:

```text
dim ker(A[R, [k]\W]) =
  1 if (R,W) is matched,
  0 otherwise
```

at the exact boundary `|R||W|=k`.

## Proof Detail Still To Formalize

The remaining formal detail is a clean rational-function argument for the one-scalar obstruction.
A good way to write it:

```text
Pick two coordinates j1,j2 in W'. The ratio of the two required cancellation equations eliminates
the child global scalar and gives a nonconstant rational equation involving the independent parent
variables T_{j1}, T_{j2}. Since the child output ratios do not involve these fresh variables, the
identity cannot hold in the rational function field.
```

This is small, local, and independent of the rest of the distance counting machinery.
