# Blaze Modular Protocol Requirements

This document states the requirements for replacing the current monolithic Blaze
protocol presentation with modular protocol boxes.  The goal is to expose the
true API boundaries:

- `Blaze.Commit`
- `Blaze.Open`
- `blazeCommit.Commit` and `blazeCommit.Open`
- `BaseFold.Commit`
- `BaseFold.Open`

The boxes should be readable as structured pseudocode.  Do not use gotos,
resampling jumps, or references such as "repeat B12."  Use structured loops,
nested loops, local subroutines, and explicit `If`/`While` blocks.

## Global Style Requirements

Each protocol box should begin with an underlined signature, followed by a short
plain-text public-parameter paragraph, followed by a structured step list.  The
public-parameter paragraph should not be a bullet point.  Example:

```text
underline{Blaze.Open(P: f, V: (C_f,z,v)) -> {0,1}}

Public parameters: field F; sizes L=2^ell, K=2^k, N=2^n; ...

1. P: ...
2. V: ...
```

For a module with two API calls, such as `blazeCommit.Com` and
`blazeCommit.Open`, use two signatures in the same box when that is clearer.

Every executable line should begin with a party identifier:

- `P:` for prover-only computation or messages.
- `V:` for verifier-only sampling/checking.
- `P,V:` for public deterministic derivations both parties can perform.
- `P -> V:` for prover messages.

Use `L=2^\ell`, `K=2^k`, and `N=2^n`.  Words such as `Public parameters`,
`For`, `While`, `If`, `Let`, `Sample`, `Check`, and `Return` should anchor the
code, but the protocol semantics should live in explicit variables, domains,
and API calls.

## Abstract Commitment API

The paper should treat `F_commit` as a handle-based logical tensor commitment
interface:

```text
R <- F_commit.Com(rep over domain D)
x <- F_commit.Open(R, a)
```

The representation `rep` is implementation-specific.  A Merkle implementation
uses the literal tensor as `rep`.  A virtual implementation may store handles
and public randomness, provided its `Open` procedure returns the logical tensor
value.

## Box: blazeCommit

`blazeCommit` is a virtual implementation of the same commitment API.  It is
not a new algebraic proof system and not a separate PCS.

Required API:

```text
R_star <- blazeCommit.Com(R_outer, rho)
x <- blazeCommit.Open(R_star, b)
```

Required behavior:

```text
blazeCommit.Com(R_outer, rho):
  Store (R_outer, rho).
  Return R_star with logical domain B^n.

blazeCommit.Open(R_star, b):
  col <- F_commit.Open(R_outer, b)
  Return <rho, col>.
```

The backend must see only `R_star` and `F_commit.Open(R_star,b)`.  It should not
inline the column-opening calculation.

## Box: Blaze.Commit

Required API:

```text
C_f <- Blaze.Commit(P: f, V: bottom)
```

Required behavior:

```text
P: choose/derive y in F^{B^ell x B^k} representing f.
P: for h in B^ell, compute c[h,*] = RAA(y[h,*]).
P: for b in B^n, set C_outer[b] = c[*,b] in F^{B^ell}.
P: R_outer <- F_commit.Com(C_outer in (F^{B^ell})^{B^n}).
P -> V: R_outer.
V: set C_f = R_outer.
Return C_f.
```

`C_f` is the PCS commitment handle.  It is the outer column commitment.

## Box: Blaze.Open

Required API:

```text
accept/reject <- Blaze.Open(P: f, V: C_f, z, v)
```

Required behavior:

```text
P: use y representing f.
P: for h in B^ell, set u[h] = MLE(y[h,*])(z_col).
P -> V: u.
V: check v = sum_h chi_h(z_row) u[h].
V: sample rho <- F^{B^ell}.
P,V: R_star <- blazeCommit.Com(C_f, rho).
P,V: v_star = <rho,u>.
P: y_star[a] = sum_h rho[h] y[h,a].
P,V: run BaseFold.Commit using R_star and the compressed claim.
P,V: run BaseFold.Open using the backend state.
V: return the BaseFold result.
```

`Blaze.Open` is the only box that translates the original PCS claim
`MLE(y)(z_row,z_col)=v` into the compressed backend claim
`MLE(y_star)(z_col)=v_star`.

## Box: BaseFold.Commit

Required API:

```text
B <- BaseFold.Commit(public: R_star, z_col, v_star; P witness: y_star)
```

Required behavior:

```text
P: build the RAA helper tensors u1,u2,u3,u4,c_star_wit.
P: commit helper tensors u2,u3,u4.
V: sample residual challenges.
P: commit product-tree helpers and component residual tensors.
V: sample eta.
P,V: define R_eta from component residual openings.
P,V: define base handles H0=R_star, H1=C_u2, H2=C_u3, H3=C_u4, H4=CheckResidualAt.
P,V: define A^(0)=(c_star_wit,u2,u3,u4,R_eta).
P,V: define w^(0) and S_0=v_star.
P: commit initial compiler parity P^(0).
P -> V: backend handles.
Return backend commitment state B.
```

This box binds backend-owned data.  It should not verify queries.

## Box: BaseFold.Open

Required API:

```text
accept/reject <- BaseFold.Open(public: B, R_star, z_col, v_star)
```

Required behavior:

1. Run the fold/evaluation spine.
2. Sample query sets.
3. Close the query sets with structured loops.
4. Open all required handles through a unified `OpenLayer`.
5. Check path equations, parity equations, terminal scalar equation, and
   residual terminal equation.

The query closure must be structured.  A valid shape is:

```text
V: initialize query sets.
V: changed = 1.
V: while changed = 1:
     changed = 0.
     for each newly required path/parity/residual item:
       add its dependencies.
       if a dependency was new, set changed = 1.
V: if input query count is too large, reject this sampling attempt and resample
   by enclosing the whole sampling procedure in a bounded/structured outer
   loop.
```

The box should not use labels or gotos such as "repeat B12."

## Responsibility Boundary

The modular invariant should be visible:

- `Blaze.Commit` owns the outer column commitment `C_f=R_outer`.
- `blazeCommit` turns `(R_outer,rho)` into a virtual handle `R_star`.
- `BaseFold.Commit/Open` consume `R_star` only through the abstract opening API.
- `Blaze.Open` orchestrates the claim translation and delegates the compressed
  backend proof to BaseFold.
