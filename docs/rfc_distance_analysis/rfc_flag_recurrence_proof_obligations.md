# RFC Flag Recurrence Proof Obligations

Scope: original non-systematic RFC only.

This is the manager-facing proof map for converting the current flag-recurrence insight into a
certificate path. It starts from:

```text
rfc_upper_bound_recurrence_state.md
rfc_upper_bound_flag_lemma_target.md
rfc_flag_recurrence_checkpoint.md
rfc_upper_bound_response_to_multicopy.md
```

## 1. Two-Layer Flag Theorem Skeleton

Let `C_h` be the original RFC code at depth `h`, with message space `H_h`. For a subspace
`V <= H_h`, write:

```text
Z(V) = { output coordinates j : ev_j(V) = 0 }.
```

Define the two-layer flag moment:

```text
F_h(r1,z1;r0,z0)
```

to count, after averaging over the RFC fold randomness, flags:

```text
L <= V <= H_h,
dim V = r1,
dim L = r0,
|Z(V)| >= z1,
|Z(L)| >= z0.
```

The first-moment distance event is recovered from the one-layer case:

```text
B_h(1,z) = projective-line count with |Z(line)| >= z.
```

A two-layer recurrence should upper-bound the one-step contribution for a parent subspace
`W <= H_h`, `dim W=t`, and an exact zero set of size:

```text
z = 2p + s,
```

where `p` child positions are paired and `s` child positions are singleton. For the singleton block
`S`, define:

```text
R     = visible singleton image of W on S,
A     = supp(R) subset S,
a     = |A|,
tau   = dim R,
K     = ker(W -> R),
kappa = t - tau.
```

Let:

```text
V_child = pi(W),
L_child = pi(K),
r1      = dim V_child,
r0      = dim L_child.
```

Then the child zero budgets are:

```text
z_V = p + s - a,
z_L = p + s.
```

The target recurrence shape is:

```text
B_h(t,z)
  <= sum over p,s,a,tau,r0,r1 and local support profiles
       Split(h;p,s,a)
       * Local(A,tau)
       * Lift(t,tau,r0,r1)
       * F_{h-1}(r1,z_V;r0,z_L).
```

Here `Local(A,tau)` includes the root challenge probability for the exact visible support `A`, and
`Lift` counts parent subspaces lifting a child flag.

For "at least z zeros", the certificate may sum exact-size recurrences over `z' >= z`; equivalently
it may count exact zero subsets of size `z`, since any larger zero set contains one.

## 2. Lemma Status

### Safe Lemmas

**Pair/singleton zero propagation.** If `A=supp(R)`, then:

```text
pi(W) vanishes on P union (S \ A),
pi(K) vanishes on P union S.
```

This is coordinatewise and uses invertibility of paired fold coordinates.

**Support containment.** If the visible support is contained in `A`, then:

```text
R <= U_A + U_A,
dim U_A = delta(A) = rank(S) - rank(S \ A).
```

This gives the ambient Gaussian bound for visible subspaces.

**Exact support de-duplication.** Support events are indexed by unmarked exact support sets `A`.
Matched-core or stride-core labels may certify the dimension of an event, but they are not extra
events. In particular, for `b=1`, `s=h+1`:

```text
k * binom(k-1,s-1) = s * binom(k,s)
```

is a marked-support overcount. The corrected multi-copy model uses `binom(k,s)`.

**Gaussian flag-lift upper bound.** For `L<=V`, `dim L=r0`, `dim V=r1`, `kappa=t-tau`:

```text
Lift(t,tau,r0,r1)
  <= Gamma_q^2 q^{kappa(2r0-kappa) + tau(2r1-t)}.
```

This follows by choosing `K <= L+L` and then choosing `W/K <= (V+V)/K`.

### Still Unproved

**Tau-two endpoint local theorem.**

```text
E_A(2) q^-a
  <= poly(a) q^theta_2(A),

theta_2(A) = max_h(2h - 4 - gamma_h(A)).
```

The old generic/component shortcut is false as a general local theorem.  The first concrete
failure is a connected `a=5, delta=3, comp=1, g=1` exact support with a codimension-one `h=2`
layer.  The local algebra obligation is now to prove the layer codimension theorem, with the
full-kernel layer charged by diagonal endomorphisms.

**Actual recursive flag moment.** The checkpoint script uses a safe min of one-layer relaxations.
The certificate needs the true recursive `F_h(r1,z1;r0,z0)` transition.

**Local-lift incidence.** The checkpoint currently multiplies a local endpoint charge by a
Gaussian quotient lift.  With the layer endpoint, dominant traces can have:

```text
local_charge_tau2(A) = quotient_lift_qdim.
```

Concrete depth-5 checkpoint trace:

```text
level=4, span=2, z=69:
  p=5, s=59, a=59, tau=2
  outer_span=4, inner_span=0
  outer_zeros=5, inner_zeros=64
  delta=3, comp=1, g=0
  theta_2=-4, dominant h=2, gamma=4
  local_charge=12, lift_qdim=12
```

This neutral transition is why the coarse flag checkpoint worsens when `theta_2` replaces the old
shortcut.  The broad exact-support quotient-lift part is now controlled by the Grassmann cap:

```text
local_charge >= |A|.
```

In the full-cover subcase, since `P union S` is the whole child coordinate set and `K=L=0`,
restriction gives `V ~= V|_A`, so:

```text
E_{V,A}(2) q^-a <= Gamma_q q^(4 dim(V)-4-a).
```

For the trace above this is `q^(12-59)=q^-47`, or equivalently `local_charge=a=59` against the
ambient two-plane lift.

The other neutral boundary row is:

```text
tau=2, |A|=2, delta=2, comp=2, K=L=0.
```

It has no local q-exponent slack, but it is decomposable. The two rank-one component lines are
zero on the common outer-zero set plus the other visible coordinate. However, the two lines live in
the same child-code instance, so a product of line first moments is not proof-safe. This exact row
needs a joint marked-line/frame state, for example:

```text
product_row_contribution
  <= poly(N) * (q+1) * F_child((2, outer_zeros), (1, outer_zeros + 1)).
```

The invalid squared-first-moment checkpoint is retired. The current depth-4 diagnostic instead
uses a coarse joint marked-line child state for this row, which safely moves the dominant trace to
the connected `|A|=3,delta=2,comp=1` endpoint. That local endpoint has a direct `U_{2,3}` proof in:

```text
docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
```

The diagnostic remains below theorem grade until the joint marked-line/frame recurrence is written
as part of the actual flag moment.

The remaining proof-grade recurrence must count the incidence of:

```text
child flag L <= V,
parent quotient W/K,
visible singleton image R,
root-line layer X_h(A)
```

as one object.  It is not enough to independently multiply an endpoint count by a generic lift
count and hope the scalar state absorbs small-support or mixed-kernel branches.

**Higher visible dimension.** The first certificate attempt should try to close with `tau<=2`. If
dominant traces require `tau>=3`, we need either a higher-tau local theorem or a quotient-flag
decomposition lemma.

**Active-copy structured intersections.** Exact-size support counting removes the false seven-copy
alarm, but a lower-bound family could still force an additional state if independent copy
constraints intersect with dimension above the flag-state prediction.

## 3. Constants To Track

The certificate must track these in bits, not hide them in q-exponents:

```text
Gamma_q lift constants:
  one Gaussian <= log2 Gamma_q,
  two-Gaussian lift <= 2 log2 Gamma_q.

Split counts:
  paired/singleton/support choices,
  singleton orientation factor 2^s,
  exact zero-size summation over z' if used.

Exact-support inversion:
  contained-support to exact-support conversion,
  root-line unused-coordinate factors,
  finite GL_tau/projectivization constants.

Root distribution:
  determinant-1 fold with T uniform nonzero,
  any constant loss between actual roots and all projective root lines.

Active-copy profile:
  binom(c,r_active),
  product_i binom(k,s_i) for unmarked exact supports,
  common flag/projective dimension t_common.

Log-sum overhead:
  number of recurrence states and support-profile classes included in the certificate.
```

For `q=2^128`, `Gamma_q` itself is negligible, but the recurrence should still expose it. The more
dangerous constants are split counts, exact-support bookkeeping, and active-copy/support-profile
sums.

## 4. e=71 Margin And e=72 Fallback

Current status for `c=8,k=2048,q=2^128`:

```text
ideal final-shape certificate: e=71 with about 39.68 bits slack;
corrected exact-size multi-copy stress: e=71 best r=1 at -121.83277193;
old r=7 marked-core stress: corrected to -151.93726555;
e=70 remains unsafe in the corrected stress model;
e=72 is comfortable.
```

Interpretation:

```text
e=71 remains the active target.
e=72 is the fallback if honest constants or a real nested-kernel family consume the e=71 margin.
```

Move to `e=72` if either:

```text
1. multiplicity-corrected falsification finds an e=71 row above -80 bits;
2. the rigorous flag recurrence closes e=71 with less than the reserved constant slack.
```

A reasonable manager threshold is to reserve at least `20` bits after all explicit constants.

## 5. Implementation Worker Spec

The exact flag-intersection enumerator should count events, not certificates.

Inputs:

```text
depth h,
field/prime for exact checks,
expansion c,
zero target z,
allowed tau range, initially tau<=2,
support-count model exact-size.
```

For each transition shape, emit:

```text
level h,
parent dimension t,
zero request z,
paired count p,
singleton count s,
exact visible support size a,
visible dimension tau,
kernel dimension kappa=t-tau,
child outer dimension r1,
child inner dimension r0,
outer zero budget z_V=p+s-a,
inner zero budget z_L=p+s,
  delta(A),
  comp(A),
  g(A),
  theta_2(A),
  dominant layer h and gamma_h(A),
  split_log2,
  local_log2,
  lift_log2,
child_flag_log2,
constant_log2,
total_log2.
```

Counting rules:

```text
1. Count unmarked exact visible supports A once.
2. Record marked-core/stride certificates only as diagnostic multiplicity.
3. Bucket exact supports separately from contained supports.
4. Count flags L<=V, not independent subspaces L and V.
5. Track active-copy profile separately from q-dimensional flag state.
6. Report the dominant trace and the strongest non-dominant r_active>=2 trace.
7. Reject impossible states, especially tau=2 with delta(A)<2.
```

For small exact fields, the enumerator should validate:

```text
exact support de-duplication,
two-layer flag dimensions,
generic-intersection dimension versus observed intersection dimension,
dense tau-two endpoint rows where g(A)>=2.
```

## Feedback For Falsification Agent

The sharpest statement to break is:

```text
After exact-size support de-duplication, every high-zero family either
  (a) follows paired compression with preserved relative gap, or
  (b) creates a two-layer flag where each invisible kernel receives the stricter z_L=p+s charge,
and the resulting corrected e=71 first moment stays below -80 bits with explicit constants.
```

Please attack this by producing a multiplicity-corrected nested-kernel cascade with:

```text
kappa=t-tau > 0
```

on at least two consecutive levels. The report should include the full transition table from the
implementation spec above, plus whether the construction is explicit or only a stress model.

The priority search classes are:

```text
1. b>1 stride-core rows after unmarked support grouping;
2. dense tau-two rows with g(A)>=2;
3. active-copy r>=2 intersections whose observed dimension exceeds the generic flag dimension;
4. all-paired spine followed by a late singleton burst with a surviving kernel.
```

If no such cascade survives exact-support counting, the proof lane should keep `e=71` as the target
and spend effort on the tau-two endpoint theorem plus the rigorous flag recurrence.
