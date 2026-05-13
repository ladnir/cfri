# Blaze Modular Protocol Requirements

This document states the requirements for the modular protocol boxes in the
Blaze theory explainer.  The goal is to expose the true API boundaries:

- `Blaze.Commit`
- `Blaze.Open`
- `Pi_blazeCommit.Com` and `Pi_blazeCommit.Open`
- `RAAAdapter.Prepare`
- `BaseFold.Commit`
- `BaseFold.Open`

The companion change spec is `docs/basefold_generic_api_change_spec.md`.

## Global Style Requirements

Each protocol box begins with a short plain-text public-parameter paragraph at
the top of the figure, followed by a small spacer, followed by an underlined
signature, followed by a structured step list.

```text
Public parameters: field F; sizes L=2^ell, K=2^k, N=2^n; ...

underline{Blaze.Open(P: f, V: (C_f,z,v)) -> {0,1}}

1. P: ...
2. V: ...
```

Every executable line begins with a party identifier:

- `P:` for prover-only computation or messages.
- `V:` for verifier-only sampling/checking.
- `P,V:` for public deterministic derivations both parties can perform.
- `P -> V:` for prover messages.

Use `L=2^\ell`, `K=2^k`, and `N=2^n`.  Words such as `Public parameters`,
`For`, `If`, `Let`, `Sample`, `Check`, and `Return` may anchor the code, but
the protocol semantics should live in explicit variables, domains, and API
calls.

Use structured loops.  Do not use gotos, labels such as "repeat B12",
resampling jumps, or stateful acceptance flags.

## Commitment API

The paper treats `F_commit` as a handle-based logical tensor commitment
interface:

```text
R <- F_commit.Com(rep over domain D)
x <- F_commit.Open(R, a)
```

The representation `rep` is implementation-specific.  A Merkle implementation
uses the literal tensor as `rep`.  A virtual implementation may store handles
and public randomness, provided its `Open` procedure returns the logical tensor
value.  Failed authentication is represented by `bot`, so
`F_commit.Open(R,a)` has output type `L union {bot}`.

## Composite Commitment API

Composite commitment is a logical routing layer over existing handles.  It is
not a new cryptographic primitive.

```text
C <- CompositeCommit((Omega_i, psi_i, C_i)_i)
x <- CompositeOpen(C, omega)
```

The subsets `Omega_i` are public and disjoint.  The maps
`psi_i : Omega_i -> D_i` are public fixed formulas or tables.  Opening at
`omega` routes to the unique part containing `omega` and calls
`F_commit.Open(C_i, psi_i(omega))`.

## Box: Pi_blazeCommit

`Pi_blazeCommit` is a virtual implementation of the same commitment API.  It is
not a separate PCS.

Required API:

```text
R_star <- Pi_blazeCommit.Com(R_outer, rho)
x <- Pi_blazeCommit.Open(R_star, b)
```

Required behavior:

```text
Pi_blazeCommit.Com(R_outer, rho):
  Store (R_outer, rho).
  Return R_star with logical domain B^n.

Pi_blazeCommit.Open(R_star, b):
  col <- F_commit.Open(R_outer, b)
  Return <rho, col>.
```

The backend sees only `R_star` and `F_commit.Open(R_star,b)`.

## Box: Blaze

The Blaze PCS surface should be one box with two function signatures:

```text
C_f <- Blaze.Commit(P: f, V: bottom)
accept/reject <- Blaze.Open(P: f, V: (C_f,z,v))
```

Required `Blaze.Commit` behavior:

```text
P: choose/derive y in F^{B^ell x B^k} representing f.
P: for h in B^ell, compute c[h,*] = RAA(y[h,*]).
P: for b in B^n, set C_outer[b] = c[*,b] in F^{B^ell}.
P: R_outer <- F_commit.Com(C_outer in (F^{B^ell})^{B^n}).
P -> V: R_outer.
V: set C_f = R_outer.
Return C_f.
```

Required `Blaze.Open` behavior:

```text
P: use y representing f.
P: for h in B^ell, set u[h] = MLE(y[h,*])(z_col).
P -> V: u.
V: check v = sum_h chi_h(z_row) u[h].
V: sample rho <- F^{B^ell}.
V -> P: rho.
P,V: R_star <- Pi_blazeCommit.Com(C_f, rho).
P,V: v_star = <rho,u>.
P: y_star[a] = sum_h rho[h] y[h,a].
P,V: run RAAAdapter.Prepare(y_star; R_star,z_col,v_star).
P: retain A.
P,V: receive (C_A,w,S,Relations).
P,V: C_code <- BaseFold.Commit(A; C_A).
P,V: run BaseFold.Open(A; C_code,w,S,Relations).
V: return the BaseFold result.
```

`Blaze.Open` is the only box that translates the original PCS claim
`MLE(y)(z_row,z_col)=v` into the compressed backend claim.

## Box: RAAAdapter.Prepare

This box is Blaze-specific.  It constructs the message tensor passed to
BaseFold and the commitment handle for that tensor.

Required output:

```text
P witness: A = (c_star, u2, u3, u4, R_eta) in F^D
D = {0,1,2,3,4} x B^n
C_A = CompositeCommit(row handles for A)
w in F^D
S = v_star
Relations = fixed zero-residual local check over row 4
```

The verifier receives `C_A`, `w`, `S`, and `Relations`; it does not receive
`A`.

For Blaze, product-tree and component-residual checks are inside the opening
rule for the virtual handle `C_eta`.  The relation descriptor passed to
BaseFold is the fixed check: for `b in B^n`, open `(4,b)` and verify that the
value is `0`.

The handle for row `0` is `R_star`.  The handles for rows `1,2,3` are helper
Merkle handles.  The handle for row `4` is a virtual residual handle whose
opening is implemented by the residual-opening subroutine using `R_star`, the
helper handles, product-tree helper handles, component-residual handles, public
layout/root addresses, and the residual challenges.

## Box: BaseFold

BaseFold is generic.  It does not mention RAA, Blaze columns, product trees, or
the five-row adapter layout.

The BaseFold backend surface should be one box with two function signatures:

```text
C_code <- BaseFold.Commit(P: A, V: C_A)
accept/reject <- BaseFold.Open(P: A, V: (C_code,w,S,Relations))
```

Required `BaseFold.Commit` behavior:

```text
P: y = Enc_BF(A) in F^Omega.
P: y_ext = y restricted to Omega_ext.
P: C_ext <- F_commit.Com(y_ext).
P -> V: C_ext.
P,V: C_code <- CompositeCommit((Omega_msg, iota^{-1}, C_A),
                               (Omega_ext, id, C_ext)).
Return C_code.
```

Here `Enc_BF : F^D -> F^Omega` is systematic, `Omega = Omega_msg union
Omega_ext`, and `iota : D -> Omega_msg` identifies the systematic coordinates.

Required `BaseFold.Open` behavior:

1. Run the BaseFold evaluation/folding/proximity protocol for the claim
   `<w,A>=S`.
2. Whenever the verifier samples a challenge or query set that the prover must
   use, the verifier sends it explicitly.
3. Sample path queries from an explicit path query domain `T_path`; do not write
   bare placeholders such as `sample Q_path`.
4. Open codeword coordinates only through `C_code`; the prover sends opened
   values and authentication data for explicitly named coordinates.
5. When a relation check needs a message coordinate `a in D`, open it at the
   systematic coordinate `iota(a)` through `C_code`.
6. Apply any adapter-supplied local relation descriptors.  Each descriptor has
   query domain `T_tau`, arity `m_tau`, maps `phi_tau,j : T_tau -> D`, and
   check polynomial `P_tau : F^{m_tau} -> F`.
7. Use immediate reject conditions for each concrete check; do not end a box
   with vague phrases such as `accept iff all checks pass`.

The relation descriptors are fixed public local checks.  They are not arbitrary
callbacks or a place to hide extra proof logic.

## Responsibility Boundary

- `Blaze.Commit` owns the outer column commitment `C_f=R_outer`.
- `Pi_blazeCommit` turns `(R_outer,rho)` into a virtual handle `R_star`.
- `RAAAdapter.Prepare` constructs the backend message tensor `A`, the handle
  `C_A`, the linear claim `<w,A>=S`, and the fixed RAA relation descriptors.
- `BaseFold.Commit/Open` consumes `A`, `C_A`, `w`, `S`, and the descriptors
  without knowing their Blaze-specific origin.
