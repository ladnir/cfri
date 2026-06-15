# RFC Depth-5 Finite Flag Recurrence Target

Scope: original non-systematic RFC, finite base seal at `k=32`, `z=34`.

Status: theorem target after the scalar rank-pattern audit.

## Objective

Prove a finite base theorem:

```text
B_5(1,34) <= 2^-80
```

for:

```text
expansion c = 8,
q = 2^128,
T uniform nonzero,
determinant-1 fold.
```

The optimistic scalar recurrence reaches:

```text
log2 B_5(1,34) = -115.10435419.
```

But the scalar local charge `q^{-r|E|}` is not proof-safe. The finite proof must use a flag state
that exposes low-visible-rank singleton blocks.

Budget update: relative to the optimistic scalar calibration, the top boundary has only:

```text
35.10435419 bits = 0.27425277 qdims
```

of uniform child-bound slack before the calibrated `B_5(1,34)` moment exceeds `2^-80`. For dominant
`B_4(2,u)` child values with `u=0..5`, a loss on just that child value can be only about
`0.29..0.31` qdims. Therefore the finite exact-support recurrence must be nearly scalar-sharp on
the dominant depth-4 states; any theorem step that loses a full q-dimension there cannot close the
base seal.

This is not a safe proof margin. The scalar recurrence is already retired as a theorem, so the
budget is a falsification/calibration threshold for the finite DP: if theorem-grade child bounds
fall behind scalar by whole q-dimensions, the hybrid base-seal route should be abandoned or
replaced by a more row-specific argument.

## State

Use exact witness zero sets and subspace/container flags. The point is to bound existence of a bad
parent line, not to count every nonzero vector or every line inside a higher-dimensional bad
container separately.

For a child code at depth `h`, define:

```text
F_h((t_0,z_0), ..., (t_m,z_m))
```

to count flags:

```text
V_0 >= V_1 >= ... >= V_m,
dim V_i = t_i,
V_i vanishes on at least z_i requested child coordinates.
```

For the finite base seal, the first implementation only needs:

```text
one-layer states:  F_h((t,z))
two-layer states:  F_h((t_1,z_1), (t_0,z_0)),  t_0 < t_1
```

through:

```text
h <= 4,
t <= 2 for parent depth-4 states,
child spans <= 4 after projection.
```

Higher-dimensional child spans appear as projected ambient spans, but every dangerous singleton
quotient in the audited depth-5 base route has visible dimension:

```text
tau <= 2.
```

## One-Step Transition

For a parent subspace:

```text
W <= H_h,
dim W = t,
```

split the zero request into paired and singleton child coordinates:

```text
P = paired coordinates,
S = singleton coordinates,
p = |P|,
s = |S|,
z = 2p+s.
```

On the singleton block, define:

```text
R   = visible image of W on S,
A   = supp(R) subset S,
a   = |A|,
tau = dim R,
K   = ker(W -> R).
```

Then:

```text
dim K = t - tau.
```

The deterministic zero propagation is:

```text
pi(W) vanishes on P union (S \ A),
pi(K) vanishes on P union S.
```

So the child recurrence must count:

```text
pi(K) <= pi(W)
```

with zero budgets:

```text
z_outer = p + s - a,
z_inner = p + s.
```

This is the core correction missing from the scalar rank-pattern recurrence.

## Local Branches Needed

### tau = 0

The singleton block is invisible:

```text
A = empty,
K = W.
```

The transition is just a stronger common-zero child event:

```text
pi(W) vanishes on P union S.
```

No root-line local factor is paid.

### tau = 1

The visible quotient is a line. Root compatibility is automatic after the visible line is fixed,
but root averaging still pays for the exact visible support:

```text
q^-a * line_count(U_A + U_A).
```

The kernel `K` receives the stricter zero budget `p+s`. This branch is where low-span counterexample
mass must be charged instead of being compared to a false `q^{-2a}` local cost.

### tau = 2

Use the exact-support weighted exterior/root-line theorem:

```text
E_A(2) q^-a <= poly(a) q^theta_2(A),

theta_2(A) = max_h (2h - 4 - gamma_h(A)).
```

Also apply the exact-support Grassmann incidence cap:

```text
local charge >= a.
```

For the decomposable neutral row:

```text
a = 2,
delta = 2,
comp = 2,
K = 0,
```

do not split into a product of two child first moments. Route through a joint marked-line/frame
state:

```text
F_child((2,z_outer), (1,z_outer+1))
```

with the already documented `(q+1)` frame-completion factor.

## Finite State Table To Certify

The top-level scalar trace is dominated by depth-4 `r=2` states:

```text
B_4(2,u), 0 <= u <= 17.
```

The full top sum can still mention `u > 17` through common-zero singleton branches. Under the
optimistic scalar recurrence these terms are far below the maximum, but the child-gap diagnostic
shows they can re-enter if a coarse theorem bound makes high-`u` child states too large. Therefore
the finite base-seal DP must report all `0 <= u <= 34`, with special attention to:

```text
u=0..5:   dominant scalar branches with only 0.29..0.31 qdims one-u slack.
u>=9:     high-common-zero tail that component-uniform child bounds already push above the scalar
          one-u ceilings.
```

The best depth-4 scalar splits are:

```text
z   p   s   c   u
0   0   0   0   0
1   0   1   0   0
2   0   2   0   0
3   0   3   0   0
4   0   4   0   0
5   0   5   0   0
6   0   6   0   0
7   0   7   0   0
8   0   8   0   0
9   0   9   0   0
10  0   10  0   0
11  0   11  0   0
12  0   12  0   0
13  0   13  0   0
14  0   14  0   0
15  1   13  0   1
16  1   14  0   1
17  1   15  0   1
```

Therefore the finite flag recurrence must handle singleton exact supports up to:

```text
a <= 15
```

over depth-3 child quotient rank at most:

```text
delta <= 8.
```

The dense boundary rows are:

```text
delta = 7, a = 13
delta = 7, a = 14
delta = 8, a = 15
```

These are exactly where the scalar `2a` charge may be one or more q-dimensions too optimistic
unless the kernel/marked-line child flag is charged recursively.

## Diagnostic Baselines

Current deterministic diagnostics bracket the target:

```text
optimistic scalar replica recurrence:  crossing_z = 34   (not theorem-safe)
span/subspace endpoint diagnostics:    crossing_z = 249  (safe but too pessimistic)
two-layer flag checkpoint:             crossing_z = 137  (proof-shaped but too loose)
tau0 cover-lift checkpoint:            crossing_z = 135  (safe partial cover)
all cover-lift checkpoint:             crossing_z = 35   (anti-conservative diagnostic)
```

More granular cover-lift diagnostics:

```text
none:      z=137
tau0:      z=135
tau1:      z=137
tau2:      z=129
tau0tau1:  z=133
tau0tau2:  z=61
tau1tau2:  z=129
all:       z=35
```

Interpretation:

```text
scalar:
  too optimistic; false local charge.

span/subspace:
  too pessimistic; forgets which kernel directions gained zero budget.

two-layer flag checkpoint:
  proof-shaped but still too loose; uses coarse child flag relaxations and not the finite exact
  recurrence for the reachable states.

cover-lift checkpoints:
  diagnostic only. Covering tau-zero/all-paired lifts barely helps, but zeroing all quotient lifts
  moves the vector-count crossing to z=35. That all-cover mode is anti-conservative as a theorem:
  for tau>0, quotient lines/planes are themselves event data and must be counted through quotient
  incidence or invisible-fiber terms. The mixed-mode table is still useful because it shows where
  multiplicity is concentrated. The corrected lemma target is `rfc_covering_flag_lift_lemma.md`.
```

## Next Concrete Implementation

Add a finite dynamic program for the depth-5 base seal that:

```text
1. decomposes top bad lines by child container span rather than ordered-vector multiplicity;
2. transitions t=2 states through tau=0,1,2 exact-support branches;
3. stores two-layer child flag/container states instead of replacing them by one-layer relaxations;
4. uses quotient-incidence exponents for tau>0, including invisible-fiber dimension,
   theta_2(A), and the Grassmann cap;
5. emits both vector-count and projective/container-count diagnostics for z=34.
```

The first smoke target is:

```text
depth 4,
top span t=2,
zero counts z=2..17,
expansion 8,
q_log2=128.
```

If this finite DP still crosses far above `34`, the base-seal route is probably not the fastest
path. If it moves close to `34`, the remaining task is to turn the finite transition inequalities
into theorem-grade lemmas and constants.

The first budget check is:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_budget.py
```

Use it to compare any proposed theorem-grade depth-4 child bounds against the optimistic
`u`-specific loss budgets. Passing this comparison is necessary for the hybrid route but not
sufficient for a proof, because the comparison baseline is scalar and non-theorem.

The first child-gap check is:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_child_gap.py
```

The corrected default `component-uniform` comparison matches scalar through `u=8` but fails for
`u>=9`. This redirects the finite-DP experiment: the child recurrence must keep the high-common-zero
tail sharp as well as the low-`u` dominant terms.

The first tail trace is:

```text
python -B scripts/rfc_distance_analysis/rfc_base_seal_tail_trace.py --min-u 9 --max-u 17
```

It exposes an all-singleton/common-zero staircase in the depth-4 child. For example, `u=17` follows
singleton splits with many common-zero child groups until the child common-zero count saturates the
child dimension; the coarse component-uniform rule then assigns zero charge to remaining singleton
extras. A theorem-grade finite DP has to replace that saturation shortcut by a sharp high-zero child
bound, especially for the all-paired top branch `B_5(1,34) -> B_4(2,17)`.

The obstruction is substantial but not obviously fatal. The trace reports both the excess above the
one-`u` ceiling and the charge missing relative to the optimistic scalar rule. At `u=9`, only about
`0.70` qdims must be recovered, while the staircase has more than fifty missing scalar qdims. At
`u=17`, about `16.38` qdims must be recovered, still far below the total missing scalar charge. So
the next theorem target is not "prove full scalar locally"; it is a high-common-zero tail lemma that
recovers enough of this missing charge uniformly across `u>=9`.

The corrected checkpoint trace after retiring the all-cover shortcut is recorded in:

```text
docs/rfc_distance_analysis/rfc_depth5_flag_checkpoint_trace.md
```
