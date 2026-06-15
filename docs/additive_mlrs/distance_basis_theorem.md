# Additive MLRS Distance Basis Theorem

Status: distance verification for the proposed additive-MLRS construction.

## Claim

For additive-MLRS, the distance question reduces to one basis theorem:

```text
The multilinear-native additive basis spans exactly the univariate polynomials
of degree < k.
```

Once this holds, the code is just Reed-Solomon evaluation of degree-`<k` polynomials on `N`
distinct points, so:

```text
distance = N - k + 1.
```

## Setup

Work over a field `F` of characteristic two. Choose an ordered `F_2`-linearly independent basis:

```text
beta_0, beta_1, ..., beta_{m-1}
```

and define the prefix additive subspaces:

```text
V_i = span_F2(beta_0, ..., beta_{i-1})
|V_i| = 2^i.
```

Let the subspace polynomial be:

```text
W_i(X) = product_{u in V_i} (X - u).
```

Then:

```text
deg W_i = 2^i.
```

In characteristic two these are linearized/additive polynomials. Normalize:

```text
S_i(X) = W_i(X) / W_i(beta_i).
```

So:

```text
S_i(beta_i) = 1,
deg S_i = 2^i.
```

This normalization is exactly the additive FFT/subspace-polynomial normalization used by the
repo's `binary_rs` twiddle machinery.

## Additive Multilinear Basis

For an integer:

```text
j = sum_i j_i 2^i,  j_i in {0,1},
```

define:

```text
Phi_j(X) = product_i S_i(X)^{j_i}.
```

For message dimension:

```text
k = 2^d,
```

the additive-MLRS message basis is:

```text
Phi_0, Phi_1, ..., Phi_{k-1}.
```

The multilinear coefficient vector:

```text
a_j, 0 <= j < k
```

defines the univariate polynomial:

```text
F_a(X) = sum_{j=0}^{k-1} a_j Phi_j(X).
```

This is the intended "free translation": the message is still a length-`2^d` multilinear coefficient
vector; we are only changing the polynomial basis used for the committed code.

## Theorem: Basis Equals Degree-`<k`

For every `0 <= j < 2^d`:

```text
deg Phi_j = j.
```

Therefore:

```text
{Phi_j : 0 <= j < 2^d}
```

is a basis for the vector space of univariate polynomials of degree `<2^d`.

### Proof

Each normalized subspace polynomial `S_i` has:

```text
deg S_i = 2^i
```

and nonzero leading coefficient.

For:

```text
Phi_j = product_i S_i^{j_i},
```

the leading term is the product of the leading terms of those `S_i` with `j_i=1`. No cancellation is
possible in the top term because this product is the unique way to obtain that total degree.

Thus:

```text
deg Phi_j = sum_i j_i 2^i = j.
```

Moreover, `Phi_j` has nonzero coefficient at degree `j` and only lower-degree terms below it. Hence
the change-of-basis matrix from:

```text
Phi_0, ..., Phi_{2^d-1}
```

to:

```text
1, X, X^2, ..., X^{2^d-1}
```

is triangular with nonzero diagonal. It is invertible.

So the additive basis spans exactly all polynomials of degree `<2^d`.

## Distance Corollary

Let:

```text
D = V_m
N = |D| = 2^m
k = 2^d
d <= m
```

Define the code:

```text
C = { (F_a(x))_{x in D} : a in F^k }.
```

Since `F_a` ranges over all univariate polynomials of degree `<k`, this is the Reed-Solomon code:

```text
RS(D,k).
```

If `F_a` is nonzero, it has at most `k-1` roots. Therefore a nonzero codeword has at most `k-1`
zero coordinates and at least:

```text
N - k + 1
```

nonzero coordinates.

Thus:

```text
dist(C) = N - k + 1.
```

This is exact MDS distance.

## Rate

For rate `1/c`, take:

```text
c = 2^rho
m = d + rho
N = 2^m = c * 2^d = c*k.
```

This gives:

```text
relative distance = 1 - 1/c + 1/N.
```

For the usual `c=8`, this is:

```text
relative distance = 7/8 + 1/N.
```

## Multilinear Compatibility

The basis is indexed by binary strings:

```text
j < 2^d <-> (j_0, ..., j_{d-1}) in {0,1}^d.
```

So a multilinear coefficient vector can be interpreted directly as additive-basis coefficients:

```text
sum_j a_j prod_i x_i^{j_i}
```

becomes:

```text
sum_j a_j prod_i S_i(X)^{j_i}.
```

If the input is in Boolean-hypercube evaluation form, the normal multilinear interpolation/Mobius
transform converts it to coefficients. That is the same kind of translation already used by
BaseFold-style multilinear commitments.

## Remaining Proof Obligation

Distance is verified by the basis theorem above. What remains is fold compatibility:

```text
fold_r(encode(a_0, a_1)) = encode(a_0 + r a_1)
```

on the quotient additive domain.

This should follow from the standard decomposition:

```text
F(X) = F_0(W_1(X)) + S_0(X) F_1(W_1(X)),
```

and its iterated version through the subspace tower.

That fold theorem is separate from distance. It is needed for protocol correctness, not for the
MDS distance statement.
