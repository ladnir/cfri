# Protocol Box Audit

This audit covers the protocol boxes around the Blaze/BaseFold construction in
`docs/blaze_protocol_theory_paper.tex`.

## Requirements

1. Protocol boxes must read as executable math.
2. Every message must name the sender, receiver, and value.
3. Every verifier-sampled challenge or query must name its domain and must be
   sent to the prover if the prover uses it.
4. Every opened value must name its handle, coordinate, and output variable.
5. Authenticated opening failure must be represented as data, e.g.
   `Open(C,a) -> x in F union {bot}`, so checks can test `x = bot`.
6. Verifier checks must be concrete predicates or equations.
7. Boxes must not rely on hidden global state such as `all checks pass`,
   `the needed values`, `the requested values`, or `Q_path` without a domain.
8. Words may anchor control flow (`for`, `if`, `return`, `let`), but words
   should not carry protocol semantics.

## Infractions Found

1. `BaseFold.Open` sampled `Q_path,Q_rel` as aggregate objects.  This hid the
   query domains and the sampled elements.
2. `BaseFold.Open` ended with `return accept iff all checks pass`.  This hid the
   exact accepting predicate.
3. `BaseFold.Open` used prose such as `if any of the three openings rejects`.
   The box lacked an authenticated-open return convention.
4. `BaseFold.Open` sent `PathOpen(...)` and `RelOpen(...)` without defining the
   opened coordinates, handles, or returned variables in the box.
5. `BaseFold.Open` referenced `Omega` after the public parameters had been
   rewritten to use layer domains `Omega^(s)`.
6. `Blaze.Open`, `BaseFold.Open`, sumcheck, BaseFold IOPP, and RAA residual
   commit had verifier challenges that were sampled and later used by the
   prover without an explicit send in the protocol text.
7. The commitment functionality described success/failure in prose instead of
   giving a typed open result.  This encouraged later boxes to say
   `opening rejects` rather than checking an explicit value.
8. `CompositeOpen` and `Pi_blazeCommit.Open` did not preserve the `bot` output
   from the underlying authenticated opening.
9. `CheckResidualAt` mixed prose such as `open from C_u2` and `or reject` with
   protocol logic.
10. `RAAResidualCommit` committed product-tree and component-residual handles
    without explicitly sending those handles to the verifier.
11. The older BaseFold PCS Evaluation box still used the obsolete
    three-argument `F_Commit.Open(C,a,value)=1` convention.
12. The ideal PCS functionality and ideal IOPP functionality used prose-level
    behavior rather than explicit handle creation, storage, and returned bits.
13. The sumcheck randomization box described the round polynomial and endpoint
    check in prose rather than explicit prover messages, verifier challenges,
    rejection tests, and the output claim `(C,r,u)`.
14. The standalone BaseFold IOPP box still said the verifier "asks the prover
    to open" values and "checks" lines, without assigning opened variables or
    testing `bot`.
15. The RAA residual query box said to use `r_eta` as an opened value, but did
    not return an explicit vector of opened virtual residual values.
16. Several older boxes had local steps such as "Prover computes" or "Compute"
    without starting the step with an explicit party identifier.

## Fix Strategy

1. Define authenticated opening as returning a value in the logical value space
   or `bot`.
2. In `BaseFold.Open`, sample path queries as `t_q <- T_path` and relation
   queries as `(tau_l,t_l) <- disjoint union_tau {tau} x T_tau`.
3. In `BaseFold.Open`, assign every opened value with an explicit
   `F_Commit.Open(handle,coordinate)` call.
4. Replace prose rejection conditions with tests of `bot` and polynomial
   equations.
5. Replace vague final acceptance with immediate rejects followed by
   `return accept`.
6. Keep verifier-to-prover sends explicit for `rho`, `theta_s`, sumcheck
   challenges, BaseFold IOPP challenges/query coordinates, and RAA residual
   challenges.

## Fixes Applied

1. `F_Commit.Open`, `CompositeOpen`, and `Pi_blazeCommit.Open` now return
   logical values or `bot`, and callers test `bot` explicitly.
2. `F_PCS` and `F_IOPP` now create handles, store submitted objects, and return
   concrete bits/handles through explicit functionality steps.
3. Sumcheck now lists every prover message `g_i`, every verifier challenge
   `r_i <- F`, every verifier-to-prover send, each rejection predicate, and the
   output randomized claim `(C,r,u)`.
4. BaseFold IOPP now opens named coordinates through
   `F_Commit.Open(handle, coordinate)`, checks `bot`, checks interpolation
   equations, and explicitly opens the base word before testing membership.
5. BaseFold PCS Evaluation now uses the typed two-argument authenticated open
   convention and reconstructs opened values from handles rather than relying
   on an obsolete value-supplied open call.
6. RAA residual commit/query now names senders and receivers for challenges and
   handles, checks `bot`, and returns the vector of virtual residual openings.
7. The final Blaze/BaseFold protocol boxes now route the virtual compressed
   commitment through `Pi_blazeCommit`, then pass a composite handle into the
   generic `BaseFold` box.
