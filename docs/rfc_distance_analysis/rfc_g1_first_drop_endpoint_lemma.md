# RFC g=1 First-Drop Endpoint Lemma

Scope: original non-systematic RFC local tau-two endpoint analysis.

This note specializes the first root-line rank-drop bound to the current blocker:

```text
|A| = 5
delta = 3
comp = 1
g = 1
h = 2.
```

It closes the local algebra for the `h=2` layer. It does not by itself prove that repeated
`theta_2=-1` layers are harmless in the global recurrence.

## Setup

Let:

```text
U_A <= F^A
delta = dim U_A
```

and for a projective root-line assignment:

```text
ell in (P^1)^A
```

define:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for every j in A }
kappa_A(ell) = dim K_A(ell).
```

Let:

```text
g = kappa_gen(A)
  = 2delta - r_gen(A),
```

where `r_gen(A)` is the generic rank of the root-line constraint matrix `M_A(ell)`.

## First-Drop Bound

Assume:

```text
g = 1.
```

Then:

```text
r_gen(A) = 2delta - 1.
```

Since this is the generic rank, some `r_gen(A) x r_gen(A)` minor of `M_A(ell)` is a nonzero
multihomogeneous polynomial on `(P^1)^A`. Each matrix row depends on one root-line variable, so the
minor has total degree at most `r_gen(A)` and multidegree at most one in each coordinate.

The first-drop layer is:

```text
X_2(A) = { ell : kappa_A(ell) >= 2 }.
```

Equivalently:

```text
rank M_A(ell) <= r_gen(A) - 1.
```

Therefore `X_2(A)` lies in the zero set of that nonzero minor. The multiprojective
Schwartz-Zippel bound gives:

```text
|X_2(A)| <= r_gen(A) * (q+1)^(|A|-1).
```

Thus:

```text
gamma_2(A) >= 1
```

up to the explicit polynomial factor `r_gen(A)`.

This is the content of `rfc_h1_kernel_drop_bound.md`, specialized to `g=1`.

## Tau-Two Endpoint Consequence

For tau two, the exact-support local count is bounded by the contained-support root-line count:

```text
E_A(2) <= C_A(2)
        = sum_ell GaussianBinomial(kappa_A(ell), 2)_q.
```

On the first-drop layer excluding higher layers:

```text
kappa_A(ell) = 2,
```

so:

```text
GaussianBinomial(2,2)_q = 1.
```

Therefore the `h=2` layer contributes:

```text
<= r_gen(A) * (q+1)^(|A|-1).
```

After the root factor:

```text
q^-|A|,
```

the exponent is:

```text
-1
```

with an explicit finite factor `r_gen(A) * (1+1/q)^(|A|-1)`.

In layer notation:

```text
2h - 4 - gamma_h(A)
  = 0 - 1
  = -1.
```

## The |A|=5, delta=3, comp=1 Row

For the concrete blocker:

```text
|A| = 5
delta = 3
comp = 1
g = 1
```

we have:

```text
r_gen(A) = 2delta - g = 5.
```

The first-drop layer satisfies:

```text
|X_2(A)| <= 5 * (q+1)^4,
theta_2 from h=2 <= -1.
```

The only higher layer is the full-kernel layer `h=delta=3`. By the connected full-rank kernel
lemma:

```text
|X_3(A)| <= q+1,
gamma_3(A) = |A| - comp = 4.
```

Its tau-two contribution is:

```text
(q+1) * GaussianBinomial(3,2)_q * q^-5
  <= poly(q) q^-2.
```

Thus the local exponent for this row is:

```text
theta_2(A) = max(-1, -2) = -1.
```

This proves the corrected layer endpoint for the first concrete `g=1` blocker. The old
two-endpoint shortcut predicted `-2`, so the row is genuinely one q-dimension heavier than the
shortcut, but it is now quantified rather than mysterious.

## Remaining Work

This lemma does not prove the full layer theorem. It leaves:

```text
1. higher-drop layers kappa >= g+2;
2. formal determinant-1 root-to-line constants;
3. global recurrence control for chains of theta_2=-1 rows.
```

For the target certificate, item 3 is now the main question attached to this row.
