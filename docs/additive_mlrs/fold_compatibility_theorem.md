# Additive MLRS Fold Compatibility Theorem

Status: one-step folding theorem for the proposed additive-MLRS construction.

## Claim

The additive-MLRS encoding is compatible with multilinear partial evaluation:

```text
fold_r(encode(a_0, a_1)) = encode(a_0 + r a_1).
```

Here `(a_0,a_1)` are the two coefficient halves for the first multilinear variable.

This is the protocol-correctness counterpart to:

```text
distance_basis_theorem.md
```

The distance theorem says the code is RS/MDS. This note says the RS code can still be folded as a
native multilinear commitment.

## Setup

Use the same additive tower as in the distance note:

```text
V_i = span_F2(beta_0, ..., beta_{i-1})
W_i(X) = product_{u in V_i} (X-u)
S_i(X) = W_i(X) / W_i(beta_i).
```

The additive multilinear basis is:

```text
Phi_j(X) = product_i S_i(X)^{j_i},
j = sum_i j_i 2^i.
```

We fold the low-order basis bit first. If an implementation folds variables in the opposite order,
it only needs to reverse/reindex the coefficient bits.

## Quotient Map

The first fiber is:

```text
{x, x + beta_0}.
```

The quotient map is:

```text
Y = S_1(X) = W_1(X) / W_1(beta_1).
```

This is constant on the fiber because `W_1` vanishes on:

```text
V_1 = {0, beta_0}.
```

So:

```text
S_1(x + beta_0) = S_1(x).
```

The local coordinate inside a fiber is:

```text
B_0(X) = S_0(X) = X / beta_0.
```

It distinguishes the two fiber points:

```text
B_0(x + beta_0) = B_0(x) + 1.
```

## Higher Basis Factors Descend

For every `i >= 1`, `S_i(X)` is constant on cosets of `V_1`, since `W_i` is the linearized
subspace polynomial with root space `V_i`:

```text
V_1 subset V_i
```

and therefore `W_i(X+v)=W_i(X)` for every `v in V_1`.

More precisely, there are quotient-basis polynomials:

```text
S'_0(Y), S'_1(Y), ...
```

for the image additive tower under `Y=S_1(X)` such that:

```text
S_i(X) = S'_{i-1}(Y)       for i >= 1.
```

Reason: both sides are normalized subspace polynomials for the same quotient subspace. They have
the same zeros, the same degree, and the same normalization at the image of `beta_i`.

This is the key structural fact. After quotienting by the first pair fiber, the remaining basis is
again the same additive multilinear basis on the quotient domain.

## Decomposition

Write a message coefficient vector as two halves:

```text
a = (a_0, a_1),
```

where `a_0` contains coefficients with first bit `0`, and `a_1` contains coefficients with first bit
`1`.

Using the descent relation above:

```text
F_a(X) = F_0(Y) + B_0(X) F_1(Y),
Y = S_1(X),
B_0(X) = S_0(X).
```

Here:

```text
F_0(Y) = additive-MLRS polynomial for a_0 on the quotient basis
F_1(Y) = additive-MLRS polynomial for a_1 on the quotient basis.
```

This is exactly the multilinear split:

```text
f(x_0, rest) = f_0(rest) + x_0 f_1(rest).
```

## Pair Fold

Take a domain pair:

```text
x_0, x_1 = x_0 + beta_0.
```

Let:

```text
Y = S_1(x_0) = S_1(x_1),
b = B_0(x_0).
```

The two codeword values are:

```text
F_a(x_0) = F_0(Y) + b       F_1(Y),
F_a(x_1) = F_0(Y) + (b + 1) F_1(Y).
```

Thus the restriction to the pair is a line in the local coordinate `B_0`.

Folding at multilinear challenge `r` means evaluating that line at local coordinate `r`:

```text
fold_r(F_a)(Y) = F_0(Y) + r F_1(Y).
```

Equivalently, from the two pair values alone:

```text
F_1(Y) = F_a(x_0) + F_a(x_1)
fold_r(F_a)(Y) = F_a(x_0) + (B_0(x_0) + r) * (F_a(x_0) + F_a(x_1)).
```

The coefficient `B_0(x_0)+r` is the ordinary interpolation weight for evaluating the affine
restriction on the pair at local coordinate `r`.

But this is precisely the additive-MLRS encoding of the folded coefficient vector:

```text
a_r = a_0 + r a_1.
```

Therefore:

```text
fold_r(encode(a_0, a_1)) = encode(a_0 + r a_1).
```

## Relation To Raw Interpolation Points

Some implementations express the pair check as interpolation in the raw field coordinate `X`.

Since:

```text
B_0(X) = X / beta_0,
```

evaluating the raw line at:

```text
Z = beta_0 * r
```

is equivalent to evaluating the local-coordinate line at `r`.

More generally, after the first fold, the same statement holds in the quotient coordinate system.
The verifier challenge may be represented either as:

```text
1. a local multilinear challenge r, or
2. a raw interpolation point Z whose normalized local coordinate is r.
```

The algebra is the same; the implementation just needs to be consistent about this conversion.

## Iteration

After one fold, the quotient domain is the image of the original additive domain under:

```text
S_1.
```

The descended basis:

```text
S'_0, S'_1, ...
```

has the same form as the original basis with one fewer message variable. Therefore the same
argument applies recursively.

After `d` folds, a degree-`<2^d` additive-MLRS word is reduced to a degree-`<1` word on a domain of
size:

```text
N / 2^d = c.
```

So the final oracle is a repetition/constant word over the remaining rate domain, matching the
usual foldable-code stopping condition.

## Consequence

Together with the distance theorem:

```text
1. Additive-MLRS is RS/MDS as a code.
2. Additive-MLRS folds according to native multilinear partial evaluation.
```

This is the clean separation we wanted:

```text
distance comes from RS;
protocol compatibility comes from the additive basis recursion.
```

## Implementation Requirements

An implementation must fix:

```text
1. basis order: which multilinear variable corresponds to beta_0 first;
2. domain order: how additive-domain pairs are laid out in the Merkle oracle;
3. challenge convention: local coordinate r versus raw interpolation point Z;
4. final rate-domain handling after d folds.
```

These are indexing/protocol choices. They do not affect the algebraic fold theorem.
