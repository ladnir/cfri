# Generic BaseFold API Change Spec

This spec records the protocol-boundary change for the Blaze/BaseFold
presentation.  The goal is to make BaseFold a generic backend over a committed
message tensor, while Blaze and the RAA relation layer remain responsible for
constructing the particular message tensor used by the Blaze proof.

## Target Separation

The protocol should have four visible layers.

1. `Blaze.Commit` is the PCS commit call.  It commits to the outer column
   tensor produced by row-wise RAA encoding.
2. `Pi_blazeCommit` is a virtual implementation of the same commitment API.
   Given an outer column handle and a row-combination vector, it exposes a
   logical handle for the compressed codeword.
3. `RAAAdapter.Prepare` is Blaze-specific.  It builds the backend message
   tensor and the commitment handle to that tensor.
4. `BaseFold.Commit/Open` is generic.  It sees only a message tensor `A`, a
   handle `C_A` for `A`, and a linear claim `<w,A>=S`.

BaseFold must not know that `A` came from RAA.  It must not contain the hard
coded rows `{0,1,2,3,4}`, accumulator equations, product-tree checks, or
`Pi_blazeCommit` internals.

## Generic Objects

Let `D` be a finite message coordinate domain.  The BaseFold input message is

```text
A in F^D.
```

The verifier does not receive `A`.  Instead, it receives a commitment handle

```text
C_A
```

whose logical tensor is `A`.  This handle may be physical, virtual, or
composite.

The normalized claim passed to BaseFold is

```text
<w,A> = S,
```

where `w in F^D` is public and `S in F` is public.

The systematic BaseFold code is public:

```text
Enc_BF : F^D -> F^Omega.
```

Its codeword coordinate domain is split as

```text
Omega = Omega_msg disjoint-union Omega_ext.
```

A public bijection

```text
iota : D -> Omega_msg
```

identifies the systematic positions, so for `y = Enc_BF(A)`,

```text
y[iota(a)] = A[a]  for every a in D.
```

BaseFold owns only the extension commitment.  It commits to

```text
y_ext = y restricted to Omega_ext.
```

The full codeword handle is a composite commitment whose message positions route
to `C_A` and whose extension positions route to the BaseFold-owned extension
handle.

## Composite Commitment

Composite commitment is a logical construction, not a new cryptographic
primitive.  It combines handles over disjoint coordinate subsets.

Given public disjoint subsets `Omega_i`, public maps `psi_i : Omega_i -> D_i`,
and handles `C_i` for tensors over `D_i`, define

```text
C = CompositeCommit((Omega_i, psi_i, C_i)_i).
```

Opening `C` at `omega` finds the unique `i` with `omega in Omega_i` and returns

```text
F_commit.Open(C_i, psi_i(omega)).
```

The maps are fixed public formulas or tables from the protocol parameters.  The
paper should not present this as an arbitrary callback that might hide
computation or extra proof work.

## RAA Adapter Objects

For Blaze, `RAAAdapter.Prepare` constructs a prover-held witness tensor

```text
A = (c_star, u2, u3, u4, R_eta) in F^D,
D = {0,1,2,3,4} x B^n.
```

It also constructs a handle `C_A` for this whole tensor:

```text
row 0 -> R_star
row 1 -> C_u2
row 2 -> C_u3
row 3 -> C_u4
row 4 -> C_eta
```

Here `C_eta` is a virtual residual handle: opening it at `b` invokes the
residual-opening subroutine with `R_star`, the helper handles for
`u2,u3,u4`, product-tree helper handles, component-residual handles, public
layout/root addresses, and the challenges `alpha,beta,gamma,eta`.  It returns
the batched value `R_eta[b]`.

The adapter also computes the public claim

```text
<w,A> = S,
S = v_star,
```

where `w` has support only on row `0` and is derived from
`raaCodewordEvalWeights(RAA,z_col)`.

Any RAA consistency checks are adapter responsibilities.  In the Blaze adapter,
the product-tree and component-residual checks are inside the opening rule for
the virtual handle `C_eta`.  The relation descriptor passed to BaseFold is then
the fixed zero-residual check on row `4`: for `b in B^n`, open `(4,b)` and
check that the returned value is `0`.

The verifier receives `C_A`, `w`, `S`, and the fixed descriptors.  It does not
receive `A`.

## Generic BaseFold API

The BaseFold protocol box should expose two function signatures:

```text
BaseFold.Commit(P: A, V: C_A) -> C_code
BaseFold.Open(P: A, V: (C_code,w,S,Relations)) -> {0,1}
```

`Relations` is optional in the pure BaseFold case.  When an adapter supplies
relations, it supplies fixed public local checks over coordinates of `D`.
BaseFold may query these relations by opening their required message
coordinates through the systematic part of `C_code`.

Each relation descriptor has the concrete form

```text
tau = (T_tau, m_tau, (phi_tau,j)_{j=1..m_tau}, P_tau)
phi_tau,j : T_tau -> D
P_tau : F^{m_tau} -> F
```

On query `(tau,t)`, BaseFold opens `A[phi_tau,j(t)]` for each `j` through
`C_code` and checks `P_tau(...) = 0`.

`BaseFold.Commit` performs:

```text
P: y = Enc_BF(A)
P: y_ext = y restricted to Omega_ext
P: C_ext = F_commit.Com(y_ext)
P -> V: C_ext
P,V: C_code = CompositeCommit((Omega_msg, iota^{-1}, C_A),
                              (Omega_ext, id, C_ext))
Return C_code
```

`BaseFold.Open` performs the fold/evaluation/proximity protocol over
`C_code`.  It must open systematic coordinates through
`CompositeOpen(C_code,iota(a))`, so the same engine works when `C_A` is an
ordinary Merkle handle, a Blaze virtual handle, or a composite adapter handle.

## Presentation Requirements

Protocol boxes should be code-like.

- Public parameters appear at the top of the figure, outside the function
  signature.
- Each signature is underlined.
- Each executable step begins with `P:`, `V:`, `P,V:`, or `P -> V:`.
- Structured loops are explicit.
- No gotos, resampling labels, or stateful acceptance flags.
- The words in a box should anchor the math; the math should carry the
  protocol.
