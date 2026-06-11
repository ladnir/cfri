# RFC U23 Tau-2 Endpoint Lemma

Scope: original non-systematic RFC local endpoint analysis.

This note closes the small connected row:

```text
tau = 2
|A| = 3
delta = 2
comp = 1
```

It is a direct proof of the component/full-kernel endpoint in the smallest connected rank-two
case. It avoids depending on the still-open general component theorem.

## Statement

Let `U <= F^A` be a connected rank-two representation on three non-loop, non-parallel
coordinates. For exact-support tau-two root compatibility:

```text
E_A(2) q^-|A| <= poly(q) q^-2,
```

and at q-exponent level:

```text
theta_2(A) = -2.
```

Equivalently:

```text
gamma_2(A) = 2 = |A| - comp(A).
```

There is no intermediate layer because `delta=2`.

## Normal Form

Since the matroid is connected, simple, and rank two on three elements, it is `U_{2,3}`. After
choosing a basis of `U` and rescaling coordinates, the three coordinate functionals can be written:

```text
l_1(x_1,x_2) = x_1
l_2(x_1,x_2) = x_2
l_3(x_1,x_2) = x_1 + x_2.
```

For a projective root-line assignment:

```text
ell_j = [alpha_j : beta_j] in P^1,
```

the rank-one compatibility condition on a pair `(x,y) in U + U` is:

```text
alpha_j l_j(x) + beta_j l_j(y) = 0.
```

Thus the condition matrix has three rows in the four variables `(x_1,x_2,y_1,y_2)`:

```text
r_1 = (alpha_1, 0, beta_1, 0)
r_2 = (0, alpha_2, 0, beta_2)
r_3 = (alpha_3, alpha_3, beta_3, beta_3).
```

## Rank-Drop Layer

The tau-two endpoint requires kernel dimension at least two. Since `dim(U+U)=4`, this means:

```text
rank(r_1,r_2,r_3) <= 2.
```

Equivalently the three rows are linearly dependent. Suppose:

```text
c_1 r_1 + c_2 r_2 + c_3 r_3 = 0.
```

If `c_3=0`, then `c_1 r_1 + c_2 r_2=0`, so `c_1=c_2=0` because each projective line has
`(alpha_j,beta_j) != (0,0)`. Hence any dependency has `c_3 != 0`.

Comparing coordinates gives:

```text
c_1 alpha_1 + c_3 alpha_3 = 0
c_1 beta_1  + c_3 beta_3  = 0
c_2 alpha_2 + c_3 alpha_3 = 0
c_2 beta_2  + c_3 beta_3  = 0.
```

Therefore:

```text
[alpha_1:beta_1] = [alpha_3:beta_3],
[alpha_2:beta_2] = [alpha_3:beta_3].
```

So the rank-drop layer is exactly the common-line diagonal:

```text
ell_1 = ell_2 = ell_3.
```

This is a copy of `P^1` inside `(P^1)^3`, hence:

```text
gamma_2(A) = codim X_2(A) = 2.
```

For each common line the kernel dimension is exactly two: it consists of pairs `(x,y)` whose
coordinate pair is in that common line for every coordinate, equivalently `y` is a fixed scalar
multiple of `x` in the affine chart. Thus the two-plane is forced.

## Endpoint Exponent

The rank-drop layer has at most `q+1` projective line assignments. For each assignment:

```text
GaussianBinomial(2,2)_q = 1.
```

Thus:

```text
E_A(2) <= q+1.
```

Averaging over three independent nonzero roots gives:

```text
E_A(2) q^-3 <= poly(q) q^-2.
```

In the layer notation:

```text
theta_2(A) = 2*2 - 4 - gamma_2(A)
           = -2.
```

This closes the `|A|=3,delta=2,comp=1` row.
