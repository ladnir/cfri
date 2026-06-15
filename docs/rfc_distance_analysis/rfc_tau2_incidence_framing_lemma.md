# RFC Tau-2 Incidence Framing Lemma

Scope: original non-systematic RFC only.

This note states the next proof object after the tau-2 layer-codimension correction. The checkpoint
failure mode is not just local: a local tau-2 layer charge can cancel a generic quotient lift if the
recurrence multiplies the two factors independently.

## Problem Branch

The enriched flag checkpoint found a dominant branch:

```text
depth 5, endpoint-tau2-layer, z=138

level = 4
parent span t = 2
p = 5
s = 59
a = 59
tau = 2
outer child span r1 = 4
inner child span r0 = 0
outer zeros z_V = 5
inner zeros z_L = 64
delta(full child after outer zeros) = 3
comp = 1
g = 0
theta_2 = -4
local_charge = 12
quotient_lift_qdim = 12
```

The scalar checkpoint therefore sees a neutral transition:

```text
quotient_lift_qdim - local_charge = 0.
```

This is too coarse. In this branch `P union S` is the whole child coordinate set, and the exact
visible support `A` is the complement of the outer zero set. Since `K=0` and `V=pi(W)`, the child
quotient visible to the singleton block is not the full child code after the outer zeros. It is the
fixed child span `V` itself.

Thus the local ambient dimension should be:

```text
dim(V/L) = r1 - r0 = 4,
```

not the full-code residual value `3`. The corrected incidence calculation has:

```text
theta_2 = -4
local_charge = 4*(r1-r0) - 4 - theta_2 = 16
quotient_lift_qdim = 12
net exponent = -4.
```

This already gives a `q^-4` penalty if the layer endpoint `theta_2(V,A)=-4` is available. The
stronger observation below is that exact support gives an unconditional Grassmann cap:

```text
q^(4r - 4 - a) = q^(12 - 59) = q^-47.
```

So the neutral branch is an artifact of mixing a full-code local bound with an independent generic
lift bound and then failing to use exact support.

## Incidence Object

Fix a child flag:

```text
L <= V <= H_{h-1}
dim L = r0
dim V = r1.
```

For a parent flag transition with kernel `K` and visible quotient dimension `tau`, define:

```text
Q = V / L.
```

The parent quotient `W/K` maps into:

```text
Q + Q.
```

On the singleton block, let:

```text
R <= Q|_S + Q|_S
```

be the visible image of `W/K`, with exact support `A` and `dim R=tau`.

The local root-line layer is now computed inside the represented quotient matroid of `Q`, not
inside the full child code. For tau two:

```text
theta_2(Q,A) = max_h(2h - 4 - gamma_h(Q,A)).
```

## Lemma Target

For fixed `L <= V`, exact singleton support `A`, and tau `2`, the number of parent quotients
`W/K` whose visible image has exact support `A` and lies in a root-line layer should be bounded by:

```text
poly(a,r1,r0) q^(
    fiber_qdim(Q,A)
  + theta_2(Q,A)
)
```

after the root challenge factor.

Here:

```text
fiber_qdim(Q,A)
```

counts only quotient directions of `Q+Q` invisible to the singleton block. If the singleton block
covers all non-outer child coordinates for `Q`, then:

```text
fiber_qdim(Q,A) = 0.
```

In particular, for the neutral checkpoint branch above:

```text
fiber_qdim = 0
theta_2 = -4
```

so the transition keeps a `q^-4` penalty.

## Relation To The Old Lift Bound

The old recurrence used:

```text
Lift <= q^(kappa(2r0-kappa) + tau(2r1-t))
```

and then multiplied by a local charge computed from the full child code. That is safe but too
coarse when the child flag is already fixed.

The incidence lemma should replace the quotient part:

```text
tau(2r1-t) - local_charge(full child)
```

by:

```text
fiber_qdim(Q,A) + theta_tau(Q,A).
```

The kernel part:

```text
kappa(2r0-kappa)
```

remains a separate lift term for `K <= L+L`.

## Current Calibration

`rfc_flag_span_moment.py` now has:

```text
--singleton-charge endpoint-tau2-layer-incidence
```

This mode floors the local tau-two quotient dimension by `r1-r0` in the checkpoint and applies the
exact-support Grassmann cap `local_charge >= |A|`. It is only a calibration heuristic, not a proof.
Before the cap, the depth-4 analogue confirmed that the former neutral trace was at least charged
by incidence:

```text
local_charge = 12
lift_qdim = 8
net = -4
```

for the analogous full-cover branch. After the cap, every tau-two exact-support quotient branch
pays at least `local_charge=|A|`.

The current depth-4 checkpoint has:

```text
crossing_excess = 49.
```

The first full-cover-only capped trace at `z=65` moved to a large-support partial-cover branch:

```text
child length = 32
p = 2
s = 29
a = 29
p+s = 31 < 32
local_charge = 12
lift_qdim = 8
```

After extending the cap to all tau-two exact-support quotient branches, that large-support trace is
no longer dominant. The new trace at `z=65` is:

```text
child length = 32
p = 16
s = 2
a = 2
local_charge = 4
lift_qdim = 4
theta_2 = 0
```

So the calibration now points at small-support tau-two layers rather than hidden large-support
fibers.

The depth-5 checkpoint is too slow in this mode without more pruning. The proof route should not
wait on this toy model; it should formalize the quotient-incidence count directly.

## Next Proof Step

The full-cover case is now the proved subcase:

```text
P union S = all child coordinates,
K = 0,
L = 0,
tau = 2.
```

Then:

```text
Q = V,
A = complement of the outer zero set,
dim Q|_A = dim V = r1.
```

The parent quotient count is bounded by the exact-support Grassmann cap, with no extra
quotient-lift fiber. This closes the concrete neutral branch.

The safe partial-cover quotient cap is now:

```text
local_charge >= |A|.
```

The next step is the refined partial-cover quotient version. For:

```text
L <= V,
Q = V/L,
B = coordinates outside A where Q may still be nonzero,
```

prove a bound of the form:

```text
fiber_qdim(Q,A,B) + min(theta_tau(Q|_A,A), tau(2 dim(Q|_A)-tau)-|A|),
```

where `fiber_qdim` counts only the invisible lifts through `B`, not a fresh full Gaussian quotient
lift. This refinement is needed mainly for small `|A|` rows where the universal `|A|` cap is too
weak and the layer theorem carries the charge.

There is also a safe coarse cap that works before the refined fiber theorem.  Once the child flag
and kernel are fixed, the quotient-lift choice is just a two-plane in some ambient quotient `E`.
If `dim E=m`, then:

```text
# Gr(2,E) <= Gamma_q q^(2(m-2)).
```

For exact visible support `A`, every coordinate of `A` accepts at most one projective root line
for a fixed two-plane. Thus averaging over independent nonzero roots gives, up to the standard
nonzero-root constants:

```text
# quotient planes after roots
  <= Gamma_q q^(2(m-2)-|A|).
```

So, relative to the Gaussian quotient lift already present in the recurrence, every tau-two
exact-support incidence branch has the universal charge:

```text
local_charge >= |A|.
```

This cap is weak for small `|A|`, but it is uniform and handles partial-cover large-support traces
without any genericity assumption.

## Exact-Support Grassmann Incidence Cap

Statement. Fix a child flag and a kernel choice. Let:

```text
E = quotient ambient for W/K,
dim E = m.
```

For tau two, the number of quotient planes with exact visible singleton support `A`, after
averaging over the singleton roots on `A`, is bounded by:

```text
Gamma_q q^(2(m-2)-|A|).
```

Equivalently, relative to the Gaussian quotient-lift exponent:

```text
[m choose 2]_q <= Gamma_q q^(2(m-2)),
```

the local exact-support charge is at least:

```text
|A|.
```

Proof. There are at most `[m choose 2]_q` quotient two-planes before imposing root compatibility.
For a fixed quotient two-plane and a fixed coordinate in the exact support, compatibility with the
RFC singleton equation forces the root into at most one projective root line; if the coordinate
projection has rank two, no root works. Since the roots are independent and uniform nonzero, the
probability for all coordinates in `A` is at most the standard nonzero-root constant times
`q^-|A|`. Multiplying by the Gaussian count gives the claim.

This is deliberately crude: it ignores the fact that most two-planes are incompatible at many
coordinates. Its value is that it is uniform in the represented quotient and applies equally to
full-cover and partial-cover branches.

## Refined Partial-Cover Fiber Form

For sharper bounds, split the quotient ambient by restriction to `A`:

```text
E_A = image(E -> coordinates A),
F_A = ker(E -> coordinates A),
m_A = dim E_A,
f_A = dim F_A.
```

A quotient two-plane with visible dimension two maps injectively to a two-plane in `E_A`; each
fixed visible two-plane has at most `q^(2 f_A)` lifts through the invisible fiber. Therefore:

```text
post-root exponent
  <= 2 f_A + local_endpoint(E_A,A).
```

Using only the Grassmann endpoint on `E_A` recovers the coarse cap:

```text
2 f_A + (2(m_A-2)-|A|)
  = 2(m-2)-|A|.
```

Using the layer endpoint on `E_A` gives the sharper target:

```text
2 f_A + theta_2(E_A,A).
```

The production recurrence should use the minimum of these two endpoints, plus the separate kernel
lift term.

## Rank-One Product Row Audit

The boundary row:

```text
tau = 2
|A| = 2
delta = 2
comp = 2
K = L = 0
```

has no local q-exponent slack after root averaging. This is real, not a counting bug: the exact
endpoint is a product of two rank-one components.

Let:

```text
A = {j_1,j_2}
U_A = U_1 direct_sum U_2
dim U_i = 1
supp(U_i) = {j_i}.
```

For any compatible exact-support two-plane, the restriction to `A` is:

```text
R|_A = R_1 direct_sum R_2,
```

where `R_i` is a line supported on `j_i`. Thus in the child quotient, the two component lines can
be represented by child lines:

```text
v_1 zero on P union (S \ A) union {j_2},
v_2 zero on P union (S \ A) union {j_1}.
```

Therefore this row should be bounded by a joint child object that remembers the two marked
rank-one component lines, each with one extra zero beyond the common outer-zero set, rather than by
a bare two-dimensional child-span moment with only the common zeros.

In the checkpoint notation:

```text
outer_zeros = |P| + |S \ A|,
```

The tempting split bound:

```text
product_row_contribution
  <= poly(N) * F_child(1, outer_zeros + 1)^2
```

is not proof-safe by itself, because the two child lines live in the same child-code instance. It
multiplies two first moments and would require an independence or negative-correlation theorem that
we do not have.

The proof-safe replacement is a joint marked-line or frame state, for example:

```text
product_row_contribution
  <= poly(N) * (q+1) * F_child((2, outer_zeros), (1, outer_zeros + 1)),
```

or a two-marked-line state that tracks both component lines. The structural decomposition remains
useful, but the squared first-moment shortcut is invalid.

The factor `q+1` pays for the second line inside the two-dimensional child span after one marked
extra-zero line has been selected. This keeps both lines in the same child-code instance.

The invalid shortcut was temporarily tested in the checkpoint and moved the dominant tau-two row
to:

```text
|A| = 3
delta = 2
comp = 1
g = 1
theta_2 = -2
local_charge = 6
lift_qdim = 4.
```

That calibration is now classified as anti-conservative until replaced by a joint marked-line
state.

Safe depth-4 calibration using the joint marked-line bound above gives the same dominant row:

```text
z=65/e=49 log2_vector_moment = -596.35983667
```

so the `|A|=2` row is controlled in the checkpoint without multiplying first moments.

This row is the connected rank-two full-kernel endpoint, not a new intermediate-layer obstruction.
For a connected rank-two restriction on three coordinates, full compatibility means the root
diagonal preserves the represented two-plane. The diagonal stabilizer has one component scalar, so:

```text
# compatible root assignments <= poly(a) q^1
root challenge factor          = q^-3
post-root exponent             = -2.
```

Equivalently:

```text
theta_2 = comp + 2delta - 4 - |A|
        = 1 + 4 - 4 - 3
        = -2.
```

Thus the `|A|=3` row is closed by the component endpoint. The next unclosed local row is again the
intermediate layer:

```text
|A| = 5
delta = 3
comp = 1
g = 1
h = 2
theta_2 = -1.
```

## Full-Cover Lemma

Statement.  Fix a child subspace:

```text
V <= H_{h-1}
dim V = r.
```

Let the child coordinate set be the disjoint union:

```text
P disjoint union A,
```

where `V` is zero on `P`.  Consider parent two-planes:

```text
W <= V + V
dim W = 2
```

with:

```text
K = ker(W -> singleton visible image on A) = 0,
tau = 2,
exact visible support A.
```

Then, after averaging over the singleton roots, the number of such parent planes is bounded by:

```text
E_{V,A}(2) q^(-|A|)
  <= Gamma_q q^(4r - 4 - |A|).
```

where `E_{V,A}(2)` is the exact-support tau-two root-line count for the represented matroid
`V|_A`. If the layer-codimension theorem is also available for `V|_A`, then the stronger combined
form is:

```text
E_{V,A}(2) q^(-|A|)
  <= poly(a,r) q^min(theta_2(V,A), 4r - 4 - |A|),
```

where:

```text
theta_2(V,A)
  = max_h(2h - 4 - gamma_h(V,A)).
```

No extra quotient-lift factor appears.

Proof.  Since `V` is zero on `P`, any vector in `V` that is also zero on `A` is zero on every child
coordinate.  The child RFC generator is injective, so:

```text
ker(V -> V|_A) = 0.
```

Therefore restriction gives an isomorphism:

```text
V ~= V|_A.
```

Likewise:

```text
V + V ~= V|_A + V|_A.
```

A parent two-plane `W <= V+V` satisfying the singleton rank-one/root compatibility on every
coordinate of `A` maps injectively to a two-plane:

```text
R <= V|_A + V|_A
```

with the same exact visible support and the same projective root-line assignment.  Conversely, any
such local two-plane `R` lifts uniquely through the isomorphism to a parent two-plane `W`.

Thus the event is counted exactly by the local exact-support root-line object `E_{V,A}(2)`, up to
the finite root-law/projectivization constants already tracked in the endpoint theorem.

The exact support assumption is decisive: a compatible two-plane determines the projective root
line on every coordinate of `A`, because each nonzero coordinate projection has rank one. Therefore
the number of compatible two-planes is bounded by the total number of two-planes in `V|_A + V|_A`.
With `dim V=r`:

```text
# Gr(2, V|_A + V|_A)
  <= Gamma_q q^(2(2r - 2))
   = Gamma_q q^(4r - 4).
```

Multiplying by the independent nonzero-root challenge factor `q^-a` gives:

```text
E_{V,A}(2) q^(-a)
  <= Gamma_q q^(4r - 4 - a).
```

Applying the layer-codimension endpoint theorem, when proved, gives the additional
`q^theta_2(V,A)` upper bound. Taking the minimum proves the combined full-cover lemma.

## Why This Closes The Neutral Trace

The neutral checkpoint branch used:

```text
r = dim V = 4,
a = 59.
```

The independent-lift recurrence counted:

```text
q^(tau(2r-t)) = q^12
```

possible parent planes before applying the local charge.  The full-cover lemma says that this
Gaussian family is itself the whole possible exact-support family before the root challenges. The
unconditional Grassmann cap gives post-root exponent:

```text
4r - 4 - a
  = 12 - 59
  = -47.
```

Equivalently, if one writes the cap as a local charge against the ambient `V+V`, then:

```text
local_charge = a = 59,
lift_qdim    = 2(2r-2) = 12,
net          = -47.
```

The earlier neutral value came from multiplying an independent quotient lift by a root-line layer
charge that did not use exact support. The Grassmann cap is crude, but it is uniform in the
represented child span `V` and does not require a genericity assumption on `theta_2(V,A)`.

## Generalization Target

For a general child flag `L <= V`, quotient by `L`:

```text
Q = V/L.
```

The same proof should apply on any singleton block whose requested support covers all coordinates
where `Q` can be nonzero.  If not all quotient coordinates are covered, the remaining invisible
coordinates produce a genuine fiber term:

```text
fiber_qdim(Q,A).
```

The conjectural general incidence transition is therefore:

```text
kernel_lift_qdim
  + fiber_qdim(Q,A)
  + theta_tau(Q,A),
```

instead of:

```text
kernel_lift_qdim
  + quotient_lift_qdim
  - local_charge(full child,A).
```
