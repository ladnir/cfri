# RFC Upper-Bound Flag Lemma Target

Scope: original non-systematic RFC only.

This note states the proof lemmas needed to turn the flag checkpoint in
`rfc_flag_recurrence_checkpoint.md` into a rigorous upper-bound recurrence. It incorporates Volta's
third-iteration feedback: exact-size support counting removes the false seven-copy marked-core
stress, and the depth-4 flag toy materially improves the scalar endpoint-tau2 crossing.

## Target Recurrence Object

At one fold, let:

```text
W <= H_h = H_{h-1} + H_{h-1}
dim W = t.
```

For a zero request with paired positions `P` and singleton positions `S`, let:

```text
R   = visible singleton image of W on S,
A   = supp(R) subset S,
tau = dim R,
K   = ker(W -> R).
```

Then:

```text
dim K = t - tau.
```

The child flag is:

```text
L = pi(K) <= V = pi(W) <= H_{h-1},
dim L = r0,
dim V = r1.
```

The zero budgets are:

```text
V is zero on P union (S \ A), so z_V >= |P| + |S| - |A|;
L is zero on P union S,       so z_L >= |P| + |S|.
```

The rigorous recurrence should sum over:

```text
P, S, A, tau, r0, r1,
local support data delta(A), comp(A), g(A),
child flag count F_{h-1}((r1,z_V),(r0,z_L)).
```

This is the object numerically approximated by `rfc_flag_span_moment.py`.

## Lemma 1: Exact Visible-Support Counting

The recurrence must count visible supports by unmarked exact support sets, not by certificates that
explain why a support is large-dimensional.

For a fixed singleton set `S` and visible support size `a`, the global split count is:

```text
choose P,
choose S,
choose A subset S,
choose singleton orientation.
```

For the exact support `A`, the local algebraic count is:

```text
E_A(tau),
```

where `E_A(tau)` is obtained from the root-line containment polynomial by exact-support inversion.
Any matched-core or stride-core label is only a witness for the dimension of the event. It is not a
separate support event.

Concretely, for the broad top-level one-copy family with `b=1` and `s=h+1`, the upper-bound count is:

```text
binom(k, s),
```

not:

```text
k * binom(k - 1, s - 1).
```

The latter is larger by:

```text
s.
```

For Volta's old seven-copy row this gives the exact-support saving:

```text
7 log2(1783) = 75.60063691 bits.
```

### Formal Statement

Let `A subset S`. Let `mathcal C_A` be any collection of internal certificates, such as marked
cores, matched strides, or complete extra stride classes, all implying that a visible quotient has
exact support `A`. Then the union-bound term for support `A` is:

```text
E_A(tau),
```

not:

```text
sum_{certificate in mathcal C_A} E_{A,certificate}(tau).
```

Equivalently, certificates may be used to upper-bound `E_A(tau)`, but they may not be multiplied
after the event has already been indexed by `A`.

This lemma is bookkeeping, but it is essential: without it, a marked-core model can lose more than
the full security margin at `e=71`.

## Lemma 2: Flag-Lift Count L5

Let:

```text
L <= V <= H_{h-1},
dim L = r0,
dim V = r1.
```

Fix:

```text
t       = dim W,
tau     = visible singleton quotient dimension,
kappa   = t - tau = dim K.
```

We need to count pairs:

```text
K <= W <= V + V
```

such that:

```text
dim K = kappa,
dim W = t,
pi(K) <= L,
pi(W) <= V.
```

A sufficient upper bound is:

```text
# lifts <= Gamma_q^2
           q^{ kappa(2r0 - kappa) + tau(2r1 - t) }.
```

Here:

```text
Gamma_q = product_{j>=1} (1 - q^{-j})^{-1}.
```

The proof is the direct Gaussian-binomial count:

```text
choose K <= L + L:
  [2r0 choose kappa]_q <= Gamma_q q^{kappa(2r0-kappa)};

for fixed K, choose W/K in (V+V)/K:
  [2r1-kappa choose tau]_q <= Gamma_q q^{tau(2r1-kappa-tau)}
                            = Gamma_q q^{tau(2r1-t)}.
```

This is exactly the lift exponent used by the checkpoint script:

```text
kappa(2r0-kappa) + tau(2r1-t).
```

If `tau = 0`, this reduces to choosing `W <= V+V`:

```text
# lifts <= Gamma_q q^{t(2r1-t)}.
```

The recurrence may further restrict to `pi(K)=L` or `pi(W)=V`; using containment only is a safe
upper bound.

### Incidence Refinement For Tau-Two

The independent lift bound is too coarse when it is multiplied by a tau-two local endpoint charge
computed in the full child code.  For a fixed child flag:

```text
L <= V,
Q = V/L,
```

the visible quotient `W/K` lives in `Q+Q`, so the local endpoint theorem must be applied to the
represented quotient `Q` on the singleton support.

There is a universal exact-support cap for the quotient-lift part. After the child flag and kernel
are fixed, a tau-two quotient plane lies in an ambient quotient `E` of dimension `m`, so:

```text
# Gr(2,E) <= Gamma_q q^(2(m-2)).
```

For each coordinate of the exact visible support `A`, a fixed two-plane accepts at most one
projective root line. Thus root averaging gives a charge of at least:

```text
|A|
```

against the Gaussian quotient lift. The tau-two incidence recurrence should therefore use:

```text
local_charge >= max(layer_charge(Q|_A,A), |A|).
```

The boundary product row needs one more structural refinement. For:

```text
tau = 2,
|A| = 2,
delta = 2,
comp = 2,
K = L = 0,
```

the local root exponent is genuinely neutral, but the quotient is decomposable. The two rank-one
components give two child lines, each zero on the common outer-zero set plus the other visible
coordinate. A product of two line first moments would be anti-conservative because the two lines
share the same child-code randomness. Therefore this row should use a joint marked-line/frame
state, for example:

```text
product_row_contribution
  <= poly(N) * (q+1) * F_child((2, outer_zeros), (1, outer_zeros + 1))
```

instead of the coarse two-dimensional child-span term.

In the full-cover case:

```text
P union S = all child coordinates,
K = L = 0,
tau = 2,
```

restriction gives an isomorphism:

```text
V ~= V|_A,
```

because `V` is zero on `P` and `A` is the complement of `P`.  Hence parent two-planes are counted
directly by:

```text
E_{V,A}(2) q^(-|A|)
  <= Gamma_q q^(4r1 - 4 - |A|),
```

with no separate quotient-lift factor.  If the layer-codimension theorem is also available for the
represented space `V|_A`, the recurrence may use the stronger of this exact-support Grassmann cap
and `poly(a,r1) q^theta_2(V,A)`.

For the depth-5 neutral trace with `r1=4` and `|A|=59`, this gives exponent:

```text
4r1 - 4 - |A| = 12 - 59 = -47,
```

so the branch is controlled without assuming any generic behavior of `theta_2(V,A)`.  This is the
first incidence lemma needed to replace the neutral branch observed in the checkpoint.

### Bit Constants

For the production field `q=2^128`, `log2 Gamma_q` is negligible. A universal finite-field bound
can keep the recurrence honest:

```text
[m choose d]_q <= Gamma_q q^{d(m-d)}.
```

Thus a two-Gaussian lift contributes at most:

```text
2 log2 Gamma_q
```

bits per transition term, before summing over split profiles. The certificate script should track
this constant explicitly rather than silently absorbing it into the q-exponent.

## Lemma 3: Child Flag Count

The checkpoint currently upper-bounds:

```text
F_{h-1}((r1,z1),(r0,z0))
```

by the best of several one-layer relaxations. The proof needs the actual recursive flag moment:

```text
F_h((t0,z0),(t1,z1),...,(tm,zm)).
```

For the first certificate attempt, the basic needed two-layer case is:

```text
F_h((r1,z_V),(r0,z_L)).
```

It should satisfy monotonicity:

```text
r0 <= r1,
z_V <= z_L,
F_h((r1,z_V),(r0,z_L)) <= F_h((r1,z_V)) + log_q [r1 choose r0]_q
```

and the recursive transition produced by Lemmas 1 and 2. Longer flags should be introduced only if
the recurrence follows an active row through the kernel child. In particular, `theta_2=-1`
first-drop chains through `L` require the three-layer state:

```text
F_h((r_V,z_V),(r_U,z_U),(r_M,z_M)).
```

The minimal recurrence contract for that case is recorded in:

```text
docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
```

The key rule is that the three-layer child event is counted once as a nested flag. It is not
bounded by multiplying an upper child moment by a lower child moment over the same code
randomness.

## Active-Copy And Generic-Intersection State

No new algebraic state is needed solely for the old seven-copy broad-family row after exact-support
correction. The common projective dimension in the generic-intersection model is exactly the
dimension of the surviving message subspace. In the recurrence, that is the flag dimension `t`.

However, the certificate trace should still record an active-copy profile:

```text
r_active,
support sizes s_i,
projective dimensions D_i,
common dimension t_common.
```

This profile contributes real binary factors:

```text
binom(c, r_active) * product_i binom(k, s_i)
```

and it is the cleanest way to audit generic-intersection assumptions.

The rule is:

```text
active-copy count is a combinatorial/support profile,
common intersection dimension is a flag-state dimension.
```

If Volta finds a family where several independent copy constraints intersect with dimension larger
than the flag recurrence permits, then the recurrence needs an additional structured-intersection
state. Until then, `t_common` plus exact support profiles should be enough.

## What Would Make e=72 Necessary

The proof side should move the conservative certificate to `e=72` if either of these happens:

```text
1. A multiplicity-corrected lower-bound model has log2 expectation above -80 at e=71.
2. The rigorous flag recurrence closes only with less than the reserved finite-constant slack.
```

The corrected multi-copy row no longer forces this move. The current proof target remains `e=71`,
with `e=72` as the fallback if constants or a real nested-kernel family eat the margin.

## Feedback For Lower-Bound Agent

Please attack the precise nested-kernel cascade that the flag recurrence is designed to charge.

The useful output is not just a final modeled expectation. Please produce a level-by-level shape:

```text
level h,
parent dimension t,
zero request z,
paired count p,
singleton count s,
exact visible support a,
visible dimension tau,
kernel dimension kappa = t - tau,
child outer dimension r1,
child inner dimension r0,
outer zero budget z_V = p+s-a,
inner zero budget z_L = p+s,
local data delta(A), comp(A), g(A).
```

Concrete attacks:

1. Find a cascade with `kappa > 0` for at least two consecutive levels after exact-support
   de-duplication. This is the first family that may force longer flags.

2. Redo the seven-copy broad-family search with the flag variables above. If the best corrected
   row is still `r=1`, report the strongest nontrivial `r>=2` row anyway, since it tests the
   active-copy trace.

3. For `b>1`, group by actual unmarked support subspace and report the number of distinct marked
   core certificates per support separately. The upper bound needs this as diagnostic metadata, not
   as multiplicity.

4. Search dense tau-two supports where `g(A) >= 2` and the generic endpoint dominates. These are
   still the most plausible local way to defeat the component endpoint.

5. If a candidate cascade survives, report whether it is an explicit support construction or only a
   generic-intersection stress row. The proof response is different in the two cases.
