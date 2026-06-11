# RFC Theta Minus One Isolation Lemma

Scope: original non-systematic RFC global distance recurrence.

Status: proof skeleton plus diagnostic evidence. This note separates the part that is already a
clean recurrence identity from the part that still needs a nested-flag proof.

## Setup

At one fold, let:

```text
W <= H_h = H_{h-1} + H_{h-1}
dim W = t.
```

For a zero request split into paired positions `P` and singleton positions `S`, write:

```text
p = |P|,
s = |S|,
z = 2p + s.
```

Let the singleton visible image have:

```text
R     = ev_S(W),
tau   = dim R,
A     = supp(R) subset S,
a     = |A|,
K     = ker(W -> R),
kappa = dim K = t - tau.
```

The child flag is:

```text
L = pi(K) <= V = pi(W).
```

The propagated child zero budgets are:

```text
z_V = p + s - a,
z_L = p + s.
```

The `theta_2=-1` row under discussion is the tau-two first-drop row:

```text
tau = 2,
g = 1,
h = 2,
gamma_2 = 1,
theta_2 = 2h - 4 - gamma_2 = -1.
```

The smallest connected example is:

```text
a = 5,
delta = 3,
comp = 1.
```

More generally, a connected first-drop row with `h=2<delta` has `a >= 5`. The `delta=2`
connected row is the separate `U_{2,3}` endpoint, where the full-kernel/component layer gives
`theta_2=-2`, not `-1`.

## Lemma 1: No Outer Zero Amplification When s=a

If a tau-two first-drop row has `s=a`, then:

```text
z_V = p.
```

Thus the row creates no singleton-derived zero surplus on the outer child span `V`. All of the
singleton surplus goes only to the kernel child:

```text
z_L - z_V = a.
```

If `s>a`, then the outer child receives the explicit residue:

```text
z_V = p + (s-a).
```

Those `s-a` coordinates are not hidden algebraic freedom. They are singleton positions outside the
visible support, so the whole parent subspace is invisible there and the child span `V` is forced
to vanish there. The recurrence can charge them as an explicit extra child-zero budget.

Proof: this is exactly the coordinatewise zero propagation lemma:

```text
V is zero on P union (S \ A),
L is zero on P union S.
```

Taking cardinalities gives the formulas above.

## Lemma 2: Outer-Branch First-Drop Chains Burn Zero Budget

Consider a path that follows the outer child `V` through `m` consecutive tau-two first-drop rows,
each with no singleton residue:

```text
s_i = a_i.
```

Let `z_i` be the zero budget at the start of row `i`. Then:

```text
z_i = 2 z_{i+1} + a_i,
```

and hence:

```text
z_0 = 2^m z_m + sum_{i=0}^{m-1} 2^i a_i.
```

Since every connected `theta_2=-1` first-drop row has `a_i >= 5`, an outer-branch chain of length
`m` consumes at least:

```text
5(2^m - 1)
```

extra top-level zero requests compared with an all-paired compression path ending at the same
child state.

This is the main isolation mechanism on the outer branch. A consecutive outer `theta_2=-1` row is
not free: it must find at least five singleton positions inside the inherited paired set at the
next fold, and this appears as explicit zero-budget burn in the recurrence.

## Kernel Branch

The kernel child is different. When `kappa>0`, the row passes:

```text
z_L = p + a = z_V + a
```

to `L`. This is intentionally stronger than the outer budget; it is the mechanism that keeps the
invisible directions honest. A consecutive `theta_2=-1` row on the kernel branch is therefore not
ruled out by Lemma 2 alone.

The right statement is:

```text
kernel-branch theta chains must be represented as nested flags.
```

A scalar one-layer recurrence can hide this issue, and a product of independent child moments is
not proof-safe. The proof-grade recurrence must keep the joint flag:

```text
L <= V
```

and, if `L` itself takes a first-drop row, extend to a longer nested flag. This is exactly the same
lesson as the decomposable `|A|=2,delta=2,comp=2` row: shared child-code randomness forces a joint
state.

## Pure Kernel-Chain Budget Identity

If a first-drop chain follows kernel children at every step, the hardest case is `s=a` at every
step. Otherwise the outer child receives residue `s-a`, and any later exit through the outer branch
is charged by Lemma 2.

In the hard case, connected first-drop rows have `a=s=5` at minimum. Along the followed kernel
branch:

```text
z_{i+1} = z_{L,i} = p_i + 5,
z_i     = 2p_i + 5.
```

Thus:

```text
z_{i+1} = (z_i + 5)/2,
z_i     = 2z_{i+1} - 5.
```

For `m` consecutive kernel-following first-drop rows:

```text
z_m = z_0/2^m + 5(1 - 2^-m),
z_0 = 2^m z_m - 5(2^m - 1).
```

This is the reason the kernel branch cannot be controlled by scalar zero budgets alone. The scalar
deepest event looks easier than all-paired compression for fixed `z_m`. The missing charge is the
nested flag data: each step inserts a lower child layer with same-depth zero-budget gap at least:

```text
z_L - z_V = a >= 5.
```

The truncation proof must keep those inserted layers, or replace them with a theorem-grade
ancestor-count bound that explicitly pays for the lost flag gaps.

## Lemma 3: Cheap Paired-Spine Rank Traces Are Not Hard Theta Chains

The optimistic shortened-kernel rank recurrence exposes a possible obstruction: a large shortened
kernel can be cheap when a bottom root-line collision is lifted through paired spines. This does
not by itself produce a consecutive hard `theta_2=-1` chain.

A connected hard first-drop row requires:

```text
tau = 2,
a = |A| >= 5,
s >= a.
```

The minimal zero-budget case that can stack without outer residue is:

```text
s = a = 5.
```

Thus any rank-trace split with:

```text
s < 5
```

cannot be a connected `theta_2=-1` first-drop row at that level. Any split with:

```text
s > 5
```

can contain an `a=5` first-drop support only with explicit singleton residue:

```text
s-a >= s-5
```

which is passed to the outer child as additional zero budget by Lemma 1.

The diagnostic trace for the dangerous near-dimension toy is:

```text
h=5: D=12,z=21,p=10,s=1,forced=0 -> child D=7,z=10
h=4: D=7,z=10,p=1,s=8,forced=7 -> child D=4,z=8
h=3: D=4,z=8,p=4,s=0 -> child D=2,z=4
h=2: D=2,z=4,p=2,s=0 -> child D=1,z=2
h=1: D=1,z=2,p=0,s=2 -> root-line collision
```

None of these steps has `s=5`. The first step has too few singleton coordinates to support a
connected first-drop row. The second step has at least three singleton-residue zeros if an `a=5`
theta row is forced into it. The remaining steps are all-paired or too small. Therefore this cheap
rank trace is a paired-spine rank obstruction, not a proof of a self-feeding hard
`theta_2=-1` chain.

The proof still has to charge the paired-spine rank obstruction in the full recurrence. But the
theta-chain truncation lemma can split cases:

```text
hard theta-compatible split: s=5, use nested-flag normal/rho margin;
non-hard rank split: route to paired-spine rank state or charge residue s-5.
```

Moreover, the hard-compatible restriction changes the accounting. If a cheap rank trace is forced
to use only:

```text
s = 5  hard theta-compatible singleton bursts,
s = 0  all-paired compression,
```

then every `s=5` step is an actual first-drop row and must contribute its own local charge `q^-9`.
For the near-dimension toy, the constrained trace has two such steps before paired compression:

```text
h=5: s=5 -> child D=7,z=11
h=4: s=5 -> child D=4,z=8
h=3: s=0 -> child D=2,z=4
h=2: s=0 -> child D=1,z=2
```

Thus a truncation test that charges only one new `q^-9` row is too pessimistic for the
hard-compatible trace. The correct hard-trace potential counts:

```text
9 * (# hard s=5 rank-trace steps)
  + rho_terminal
  - E_anc
  - state constants.
```

## Diagnostic Check

The checkpoint script now reports the immediate child transition type for each best
`theta_2=-1` row:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py \
  --depth 6 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2-layer-incidence \
  --max-visible-tau 2 \
  --prune-to-final-span 1 \
  --report-local-theta -1
```

The current depth-6 report has three best-transition `theta_2=-1` rows. In all three, the
immediate outer child best transition has `tau=0`, and in the one row with a nonzero kernel child,
the immediate inner child best transition also has `tau=0`:

```text
level span z   a tau outer_span inner_span outer_next_tau inner_next_tau
4     4    37  5 2   7          4          0              0
4     2    69  5 2   3          0          0
5     2    197 5 2   3          0          0
```

This does not prove that every subdominant chain is harmless, but it supports the isolation
picture: the dominant first-drop rows are singleton bursts followed by paired compression, not
self-feeding theta chains.

## Proof Integration Target

The recurrence proof should use the following split.

Outer branch:

```text
Use Lemma 2. Consecutive first-drop rows carry an explicit zero-budget burn
sum_i 2^i a_i, with a_i >= 5.
```

Kernel branch:

```text
Promote the recurrence to nested flags whenever a first-drop row is followed through L.
Do not multiply independent first moments for the child branches.
```

Operationally, the certificate can track a small marker on each active flag edge:

```text
last edge was theta_2=-1 first-drop
```

If the next row follows the outer edge, the marker is discharged by the `a>=5` zero-budget burn.
If the next row follows the kernel edge, the state is refined to include the nested child flag
created by the two rows. The minimal recurrence contract for this case is:

```text
docs/rfc_distance_analysis/rfc_kernel_branch_nested_flag_recurrence.md
```

The remaining work is to turn that contract into a certificate implementation and prove the
truncation rule: length three is enough for the target certificate, or length-four kernel chains
are already dominated by accumulated first-drop charges and zero-budget burn.
