# RFC h=1 Root-Line Kernel Drop Bound

This note records the corrected first kernel-profile bound.

## Correct Baseline

For `A subset S`, let:

```text
U_A = {u in U : supp(u) subset A}
delta = dim U_A.
```

For a projective root-line assignment `ell in (P^1)^A`, define:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for all j in A }
kappa_A(ell) = dim K_A(ell).
```

The earlier baseline:

```text
max(0, 2delta - |A|)
```

is only the equation-count lower bound. The correct generic baseline is:

```text
r_gen(A)     = generic rank of the root-line constraint matrix
kappa_gen(A) = 2delta - r_gen(A).
```

The generic rank is the two-copy matroid-union rank of the coordinate matroid of `U_A`:

```text
r_gen(A) = min_{B subset A} |A \ B| + 2 rank_{U_A}(B).
```

When the two-copy union has full possible rank, this reduces to:

```text
kappa_gen(A) = max(0, 2delta - |A|).
```

Dense substructures can make `kappa_gen` larger. Those are not exceptions; they are exactly the
same matroid-union bottleneck seen in the one-step rank analysis.

## h=1 Bound

The first rank-drop target is:

```text
N_1(A) = # { ell in (P^1)^A : kappa_A(ell) >= kappa_gen(A)+1 }.
```

Claim:

```text
N_1(A) <= r_gen(A) * (q+1)^(|A|-1).
```

Up to the harmless factor `r_gen(A) <= |A|`, this is the desired `q^(|A|-1)` bound.

## Proof

Write the root-line constraint matrix as:

```text
M_A(ell): U_A + U_A -> F^A.
```

The row for coordinate `j` is the linear relation selecting `ell_j`. Over the rational function
field of independent projective root-line variables, `M_A` has rank `r_gen(A)`. Therefore at least
one `r_gen(A) x r_gen(A)` minor is a nonzero multihomogeneous polynomial on:

```text
(P^1)^A.
```

Each row of the minor uses only one root-line variable, so the minor has multidegree at most `1` in
each coordinate and total degree at most `r_gen(A)`.

If:

```text
kappa_A(ell) >= kappa_gen(A)+1,
```

then:

```text
rank M_A(ell) <= r_gen(A)-1,
```

so this nonzero minor vanishes at `ell`.

A nonzero multihomogeneous polynomial on `(P^1)^a` with total degree `D` has at most:

```text
D * (q+1)^(a-1)
```

zeros. Applying this with `D <= r_gen(A)` proves:

```text
N_1(A) <= r_gen(A) * (q+1)^(a-1).
```

## What This Proves And Does Not Prove

This proves the first drop layer relative to the correct generic kernel dimension.

For `tau=2`, it controls the first nontrivial intermediate case whenever:

```text
kappa_gen(A) = 1.
```

Then:

```text
# { ell : kappa_A(ell) >= 2 } <= r_gen(A) * (q+1)^(a-1).
```

It does not yet control higher drops:

```text
kappa_A(ell) >= kappa_gen(A)+h, h >= 2.
```

Those should satisfy a determinantal `h^2`-type bound, except for recursive component/subsupport
spikes. That is the next local theorem after `h=1`.
