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

## State

Use exact witness zero sets and subspace flags.

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

The top-level scalar trace calls depth-4 `r=2` states:

```text
B_4(2,u), 0 <= u <= 17.
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
optimistic scalar replica recurrence:  crossing_z = 34
span/subspace endpoint diagnostics:    crossing_z = 249
two-layer flag checkpoint:             crossing_z = 137
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
```

## Next Concrete Implementation

Add a finite dynamic program for the depth-5 base seal that:

```text
1. decomposes ordered pairs by span t=1 or t=2;
2. transitions t=2 states through tau=0,1,2 exact-support branches;
3. stores two-layer child flag states instead of replacing them by one-layer relaxations;
4. uses theorem exponents for tau=2, including theta_2(A) and the Grassmann cap;
5. emits the dominant trace and slack for z=34.
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
