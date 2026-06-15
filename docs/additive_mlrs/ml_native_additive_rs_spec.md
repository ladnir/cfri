# ML-Native Additive RS Spec

Status: first-pass construction target.

## Objective

Build a foldable code for multilinear polynomial commitments with:

```text
message dimension k = 2^d
code length N = c*k
MDS distance N-k+1
binary/additive-field compatibility
no roots of unity
```

The code should be Reed-Solomon as a code, but expose a multilinear-native folding interface.

## Domain Tower

Work over a field containing an additive subspace:

```text
V_m <= F
dim_F2(V_m) = m
|V_m| = 2^m
```

Choose:

```text
m = d + log2(c)
N = 2^m.
```

Let:

```text
V_0 < V_1 < ... < V_m
```

be a basis-prefix tower. At round `i`, quotient by a one-dimensional fiber:

```text
{x, x + beta_i}.
```

The quotient map is the subspace polynomial:

```text
L_i(X) = X^2 + beta_i X.
```

More generally, for non-normalized subspaces:

```text
W_{i+1}(X) = W_i(X)(W_i(X) + W_i(beta_i)).
```

This is exactly the additive-subspace analogue of FRI's multiplicative squaring map.

## Message Basis

The message is a length-`2^d` multilinear coefficient vector:

```text
a_b, b in {0,1}^d.
```

Interpret it as coefficients in the recursive additive basis:

```text
Phi_b(X) = product over i of B_i(Y_i)^{b_i},
```

where:

```text
Y_0 = X,
Y_{i+1} = L_i(Y_i),
```

and `B_i` is the normalized local coordinate for the fiber at level `i`.

Then:

```text
F_a(X) = sum_b a_b Phi_b(X)
```

should span the ordinary degree-`<2^d` univariate polynomial space.

This basis choice is the bridge:

```text
multilinear coefficients <-> univariate RS polynomial.
```

## Encoding

Evaluate:

```text
F_a(X)
```

on all points of an additive domain:

```text
D = V_m
```

or an affine coset of it.

The code is:

```text
C = { (F_a(x))_{x in D} : a in F^k }.
```

Because this is the degree-`<k` RS space evaluated on `N` distinct points:

```text
distance(C) = N-k+1.
```

## Fold

Split coefficients:

```text
a = (a_0, a_1)
```

corresponding to:

```text
F_a(X) = F_0(L_0(X)) + B_0(X) F_1(L_0(X)).
```

Given challenge `r`, define:

```text
G(Y) = F_0(Y) + r F_1(Y).
```

Then `G` is the additive-MLRS encoding of the multilinear partial evaluation:

```text
a_r = a_0 + r a_1.
```

Verifier check on a pair `{x, x+beta_0}`:

```text
1. interpolate F_a locally in coordinate B_0;
2. evaluate the line at r;
3. compare to folded oracle at Y=L_0(x).
```

## Difference From Current BaseFold Binary-RS Path

Current BaseFold `binary_rs` uses additive subspace tables, but its code family is not the global
degree-`<k` RS code. Small audit:

```text
docs/rfc_distance_analysis/binary_rs_encoder_audit.md
```

shows MDS failures for the modeled current path:

```text
k=4,n=16: 32 defective k-subsets
k=8,n=16: 32 defective k-subsets
```

The present spec requires a stronger invariant:

```text
generator row space == degree-<k RS row space on D.
```

## First Prototype Gates

For small `GF(2^m)`, verify:

```text
1. basis span:
   rank(Phi_b evaluated on D) = k.

2. RS equivalence:
   rowspace(Phi_b(D)) == rowspace([x^j]_{j<k, x in D}).

3. MDS:
   every k-column subset has rank k for small k,n.

4. fold:
   fold_r(encode(a_0,a_1)) == encode(a_0 + r a_1)
   on the quotient domain.
```

Passing these gates would make the construction meaningfully different from RFC and from current
`binary_rs`: it would be a multilinear-native presentation of an MDS additive RS code.
