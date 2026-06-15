# Towered MLRS From First Principles

Status: conceptual explainer for the common Reed-Solomon idea behind the prime-field and
binary-field versions.

`MLRS` means:

```text
multilinear-native Reed-Solomon
```

The important point is not that Reed-Solomon becomes multivariate. It does not. The code is still a
univariate Reed-Solomon code. The point is that the univariate degree-`<2^d` polynomial space can be
presented in a bit-indexed basis, so folding one bit of the basis looks exactly like multilinear
partial evaluation.

## The Goal

Start with a length-`k` multilinear object, where:

```text
k = 2^d.
```

The message indices are bit strings:

```text
j = j_0 + 2 j_1 + ... + 2^(d-1) j_(d-1),
j_i in {0,1}.
```

A normal multilinear polynomial in coefficient form is:

```text
f(z_0, ..., z_(d-1)) = sum_j a_j product_i z_i^j_i.
```

The RS idea is to map the same coefficient vector into a univariate polynomial:

```text
F_a(X) = sum_j a_j Phi_j(X),
```

where the `Phi_j` are a bit-indexed basis of the univariate degree-`<k` space.

If:

```text
span{Phi_0, ..., Phi_(k-1)} = {univariate polynomials of degree < k},
```

then evaluating `F_a` on `N` distinct field points gives an ordinary Reed-Solomon code:

```text
C(a) = (F_a(x))_(x in D),
|D| = N.
```

So distance is immediate:

```text
dist(C) = N - k + 1.
```

This is the whole trick:

```text
use RS for distance;
choose the RS basis so folding still looks multilinear.
```

## What Folding Needs

For binary folding, each round should pair the current domain into two-point fibers:

```text
D_i -> D_(i+1)
x   -> Q_i(x)
```

where each fiber has two points. We also need a local coordinate `B_i(X)` on each fiber. The desired
one-step decomposition is:

```text
F(X) = F_0(Q_i(X)) + B_i(X) F_1(Q_i(X)).
```

Then a fold challenge `r` produces:

```text
fold_r(F)(Y) = F_0(Y) + r F_1(Y).
```

This is formally the same operation as multilinear partial evaluation:

```text
f(z_0, rest) = f_0(rest) + z_0 f_1(rest)
f(r, rest)   = f_0(rest) + r   f_1(rest).
```

The proof obligations are therefore:

```text
1. Distance:
   the basis spans the degree-<k univariate polynomial space.

2. Fold compatibility:
   the basis recursively decomposes as F_0(Q_i) + B_i F_1(Q_i).

3. Domain compatibility:
   Q_i maps the current evaluation domain two-to-one onto the next domain.
```

Once these hold, the construction is a Reed-Solomon code with a multilinear folding interface.

## The Abstract Tower

A towered MLRS instance consists of:

```text
D_0, D_1, ..., D_d             evaluation domains
Q_i : D_i -> D_(i+1)           two-to-one quotient maps
B_i(X)                         local fiber coordinates
Phi_j(X)                       bit-indexed basis functions
```

At each round:

```text
current word = evaluations of F on D_i
pair fibers  = preimages of Q_i
folded word  = evaluations of F_0 + r F_1 on D_(i+1)
```

After `d` folds, the degree bound is reduced from `<2^d` to `<1`, so the final word is constant on
the remaining rate domain:

```text
|D_d| = N / k = c.
```

This is exactly the usual FRI/BaseFold-style stopping shape, but the distance statement is RS/MDS
because the original code space is univariate degree-`<k`.

## Instantiation A: FFT-Friendly Prime Fields

Let `F = F_p` be an odd prime field with a large power of two dividing:

```text
p - 1.
```

Choose a multiplicative subgroup or coset:

```text
D_0 = gamma * H,
|D_0| = N = 2^m.
```

The quotient map is:

```text
Q_i(X) = X^2.
```

The fiber over `Y = X^2` is:

```text
{x, -x}.
```

The decomposition is the usual even/odd split:

```text
F(X) = F_even(X^2) + X F_odd(X^2).
```

So the fold is:

```text
fold_r(F)(Y) = F_even(Y) + r F_odd(Y).
```

From the two pair values this is:

```text
F_even(Y) = (F(x) + F(-x)) / 2
F_odd(Y)  = (F(x) - F(-x)) / (2x)
fold_r(F)(Y) = F_even(Y) + r F_odd(Y).
```

So the challenge `r` is the point where the affine restriction in the local coordinate `X` is
evaluated. This is the multiplicative analogue of evaluating an additive fiber line at local
coordinate `r`.

The basis here can simply be the monomial basis:

```text
Phi_j(X) = X^j.
```

Since exponents have binary expansions, the coefficient vector is already bit-indexed:

```text
X^j = product_i (X^(2^i))^j_i.
```

Folding the low bit of `j` is exactly the even/odd split. Repeating the fold removes one exponent bit
at a time.

This is the standard root-of-unity FRI geometry, viewed through the multilinear coefficient lens.

### Prime-Field Requirements

The prime-field version needs enough two-adicity:

```text
N = c*k must divide p - 1
```

or at least the chosen coset must support the required chain:

```text
D_0 --square--> D_1 --square--> ... --square--> D_d.
```

This is why FFT-friendly primes such as BabyBear are natural for this version. It is not
field-agnostic.

## Instantiation B: Binary Additive Fields

Let:

```text
F = GF(2^M).
```

Choose an ordered `F_2`-linearly independent basis:

```text
beta_0, beta_1, ..., beta_(m-1).
```

Define prefix additive subspaces:

```text
V_i = span_F2(beta_0, ..., beta_(i-1)).
```

The evaluation domain is:

```text
D_0 = V_m,
|D_0| = 2^m = N.
```

The first fiber is:

```text
{x, x + beta_0}.
```

The quotient map is a normalized subspace polynomial:

```text
Q_0(X) = S_1(X),
S_i(X) = W_i(X) / W_i(beta_i),
W_i(X) = product_(u in V_i) (X - u).
```

Because `W_i` is a linearized polynomial with root space `V_i`, it is constant on cosets of `V_i`.
In particular:

```text
S_1(x + beta_0) = S_1(x).
```

The local coordinate is:

```text
B_0(X) = S_0(X) = X / beta_0,
```

and it distinguishes the pair:

```text
B_0(x + beta_0) = B_0(x) + 1.
```

The bit-indexed basis is:

```text
Phi_j(X) = product_i S_i(X)^j_i,
j = sum_i j_i 2^i.
```

Since:

```text
deg S_i = 2^i,
```

we get:

```text
deg Phi_j = j.
```

Thus:

```text
Phi_0, ..., Phi_(2^d-1)
```

is a triangular basis for all univariate polynomials of degree `<2^d`. Evaluation is therefore
Reed-Solomon and has MDS distance.

The fold decomposition is:

```text
F(X) = F_0(S_1(X)) + S_0(X) F_1(S_1(X)).
```

Folding at `r` gives:

```text
fold_r(F)(Y) = F_0(Y) + r F_1(Y).
```

After quotienting, the higher basis functions descend to the next additive tower. So the same
argument repeats.

This is the root-free, characteristic-two analogue of the multiplicative squaring construction.

## Same Skeleton, Different Geometry

The two constructions share the same skeleton:

```text
univariate RS code
recursive two-to-one quotient tower
bit-indexed basis
fold = coefficient-bit partial evaluation
```

They differ in the geometry of the quotient tower:

```text
FFT-friendly prime:
  domain       = multiplicative subgroup/coset
  pair         = {x, -x}
  quotient     = X^2
  basis        = monomials X^j

binary additive:
  domain       = additive F_2-subspace
  pair         = {x, x + beta}
  quotient     = subspace polynomial
  basis        = products of normalized subspace polynomials
```

Both are Reed-Solomon underneath. Both expose a multilinear-style coefficient split. Neither is a
generic field-agnostic random foldable code.

## What This Says And Does Not Say

This construction does say:

```text
1. Multilinear interfaces and RS/MDS distance are compatible.
2. Binary fields can get a root-free RS folding tower using additive subspaces.
3. FFT-friendly prime fields can use the standard multiplicative FRI tower.
4. The right mental model is "univariate RS with a multilinear-native basis."
```

This construction does not say:

```text
1. Every field has such a binary quotient tower.
2. Random foldable codes are automatically near-MDS.
3. The existing RFC/BaseFold encoder is secretly this exact RS code.
4. A systematic/holographic backend integration is automatic.
```

The main design conclusion is:

```text
If the field supplies a clean two-to-one quotient tower, use RS and get MDS distance by construction.
```

For `GF(2^128)`, use the additive/subspace tower. For large FFT-friendly primes, use the
multiplicative squaring tower.
