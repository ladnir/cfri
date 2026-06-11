# RFC Theta Minus One Chain Recurrence Target

Scope: original non-systematic RFC global distance recurrence.

The local `g=1` first-drop endpoint is now quantified:

```text
theta_2 = -1.
```

This note identifies the next global question: can these one-q-dimension penalties recur across
many levels in a way that threatens the near-MDS certificate?

## Local Shape

The first concrete local shape is:

```text
tau = 2
|A| = 5
delta = 3
comp = 1
g = 1
h = 2
theta_2 = -1.
```

The local theorem says the root-line layer has codimension at least one:

```text
gamma_2 >= 1.
```

Thus the post-root endpoint exponent is:

```text
2h - 4 - gamma_h = -1.
```

## Why This Is Still A Global Issue

One q-dimension is `128` bits at the production field, so a single such layer is a real charge.
However, the certificate has only finite slack after support-combinatorial factors. The global
recurrence must rule out a chain where the same kind of low-charge tau-two layer appears repeatedly
while kernel directions remain aligned.

The dangerous chain should look like:

```text
parent span t = 2 or 3,
visible tau = 2,
kernel dimension kappa = t - tau,
child quotient Q = V/L,
exact support A with theta_2(Q|A,A) = -1,
child flag carries enough zeros to repeat the shape.
```

## Required Recurrence State

The scalar state cannot certify this. The recurrence needs at least:

```text
L <= V,
dim V,
dim L,
zeros of V,
zeros of L,
visible support size |A|,
local theta_2 layer h,
whether the row is connected first-drop or decomposable product.
```

For the `|A|=2` decomposable boundary row, route through the joint marked-line/frame state before
entering this chain analysis.

## First Checkpoint

Run the safe depth-4/5 flag recurrence with:

```text
endpoint-tau2-layer-incidence
exact-support Grassmann cap
joint marked-line bound for |A|=2 product row
no product-of-first-moments flag bound
```

and trace the first row with:

```text
theta_2 = -1.
```

Report:

```text
level,
span t,
zero request z,
p,s,a,
tau,
outer_span, inner_span,
outer_zeros, inner_zeros,
local_charge, lift_qdim,
child state used.
```

The target is to decide whether the `theta_2=-1` row is isolated by support/root charge or can
feed into itself recursively.

## Current Diagnostic Evidence

The checkpoint script now supports a pruned final-span path:

```text
--prune-to-final-span 1
```

For a distance first moment this computes only spans that can feed the final top line. This makes
depth 5 and 6 usable for this diagnostic without changing the recurrence values on the final
span-1 path.

With:

```text
--singleton-charge endpoint-tau2-layer-incidence
--max-visible-tau 2
--report-local-theta -1
```

the first real `theta_2=-1` best-transition states found are:

```text
depth 5 pruned to final span 1:
  level=4, span=2, z=69
  p=32, s=5, a=5, tau=2
  outer_span=3, inner_span=0
  outer_zeros=32, inner_zeros=37
  local_charge=9, delta=3, comp=1, g=1, theta=-1

depth 6 pruned to final span 1:
  level=4, span=4, z=37
  p=16, s=5, a=5, tau=2
  outer_span=7, inner_span=4
  outer_zeros=16, inner_zeros=21
  local_charge=9, delta=3, comp=1, g=1, theta=-1

  level=5, span=2, z=197
  p=96, s=5, a=5, tau=2
  outer_span=3, inner_span=0
  outer_zeros=96, inner_zeros=101
  local_charge=9, delta=3, comp=1, g=1, theta=-1
```

The top-level crossing traces at these depths do not use `theta_2=-1`; they use either the closed
`|A|=3,delta=2,comp=1` row or larger-support rows with much more negative theta. Tracing the
reported `theta_2=-1` states themselves gives:

```text
span=2,z=69:
  theta=-1 row -> all-paired child span=3,z=32 -> all-paired compression.

span=2,z=197:
  theta=-1 row -> all-paired child span=3,z=96 -> larger-support tau-two row.

span=4,z=37:
  theta=-1 row -> outer all-paired child span=7,z=16.
  The inner child span=4,z=21 also has no theta=-1 best transition; it begins all-paired.
```

This is not a proof, but it points to a sharper isolation statement:

```text
In every current best theta_2=-1 transition, the row has s=a=5. Thus the singleton burst consumes
all singleton requests in the visible quotient and passes no singleton-derived zero increment to
the outer child span. The extra five zeros go only to the kernel flag. In the observed K=0 rows
there is no inner recursive state; in the observed K!=0 row the inner child state starts with an
all-paired transition rather than another theta_2=-1 row.
```

The proof task is now to turn this diagnostic into an isolation lemma:

```text
Either a theta_2=-1 first-drop row is followed by at least one all-paired compression on every
active child flag branch, or the branch pays an additional support/fiber charge that makes the
would-be consecutive theta_2=-1 chain no worse than the heavier large-support rows.
```

The current proof skeleton is now recorded in:

```text
docs/rfc_distance_analysis/rfc_theta_minus_one_isolation_lemma.md
```

It proves the outer-branch zero-budget accounting exactly. The remaining proof-grade work is the
kernel-branch case: if a first-drop row recurses through `L`, the recurrence must promote to a
nested flag instead of multiplying separate child moments.

The kernel-branch recurrence contract is now:

```text
docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
```

## Proof Target

Prove either:

```text
theta_2=-1 rows cannot occur on consecutive active levels without extra child-zero charge,
```

or add the missing state that captures the chain and show its first moment still stays below the
target security threshold.
