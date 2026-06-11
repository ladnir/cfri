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

### State-Label Split Lemma

The hard/non-hard split is determined by the atomic recurrence labels, not by a diagnostic choice.
For an active singleton row, the profile includes:

```text
S = singleton zero coordinates,
A subset S = exact visible support,
s = |S|,
a = |A|.
```

For a connected `theta_2=-1` first-drop row:

```text
a >= 5.
```

Therefore:

```text
s < 5:
  impossible for a connected theta_2=-1 row, since a <= s;

s = 5:
  any connected theta_2=-1 row has a = s = 5.
  This is the minimal hard label and has no outer residue;

s > 5, a = 5:
  the row is a minimal-support first-drop row with explicit residue s-a = s-5
  on the outer child;

s > 5, a > 5:
  the row is not the minimal hard label.
  It is a larger-support tau-two row and must be charged by its own local profile
  and the larger flag gap z_L-z_V=a.
```

Proof. The inclusion `A subset S` gives `a <= s`. The connected first-drop local theorem gives
`a >= 5`. The four cases above exhaust the possible values of `s` and `a`. The residue statement
is Lemma 1:

```text
z_V = p+s-a,
z_L = p+s,
z_L-z_V = a.
```

Only the case `s=a=5` is allowed to enter the minimal hard-trace potential with charge exactly
`9` per hard step. The larger-support case may be better, but it is a different local row; the
certificate must use its actual local charge/state count rather than silently treating it as the
minimal hard label.

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

## Lemma 4: Hard-Trace Potential

Consider a kernel-chain defect route in which every exposed shortened-kernel rank trace step is one
of:

```text
s = 5  hard connected first-drop burst,
s = 0  all-paired compression.
```

Let:

```text
H = number of s=5 trace steps above the terminal rank event,
rho_term = shortened-kernel rank cost of the terminal event,
E_anc = sum_i (t_i-t_{i+1})(D_i-t_i)
```

for the ancestor choices introduced by the extra nested flag layers. Then the chain contribution is
dominated whenever:

```text
9H + rho_term >= E_anc + log_q(state constants).
```

Reason. Each `s=5` step is not merely a generic rank recurrence edge; it is exactly a connected
tau-two first-drop row with minimal support `a=s=5`, so the local tau-two theorem supplies the
`q^-9` charge for that row. Each `s=0` step is all-paired and contributes no new local row; it
compresses the rank event to the child. The terminal event is then charged by the shortened-kernel
rank recurrence. The ancestor factor is deterministic after the child zero witnesses are fixed,
so it appears only through `E_anc` and explicit state constants.

For the near-dimension toy:

```text
python scripts/rfc_distance_analysis/rfc_shortened_rank_recurrence.py \
  --depth 5 \
  --expansion 8 \
  --dim 12 \
  --zeros 21 \
  --allowed-singletons 0,5 \
  --ancestor-exponent 12 \
  --trace
```

the diagnostic gives:

```text
hard_steps = 2,
rho_term = 1,
E_anc = 12,
hard_trace_margin = 7.
```

This does not finish the proof because `log_q(state constants)` still has to be bounded and the
hard-trace restriction must be derived from the actual recurrence state, not imposed by the
diagnostic. It does show the corrected accounting that makes the paired-spine toy compatible with
the theta-chain truncation route.

### Constants Budget For The Toy Margin

For the default target:

```text
N = 16384,
q = 2^128,
log_q N = 14/128 = 0.109375.
```

Thus polynomial state counts have the following q-dimensional costs:

```text
N^8  -> 0.875
N^16 -> 1.75
N^32 -> 3.5
N^48 -> 5.25
N^64 -> 7.0
```

The near-dimension toy hard-trace margin before constants is `7`, so this route can absorb a
substantial fixed polynomial state count, but not an untracked exponential-in-depth family. Explicit
frame factors of size `q+1` should be counted as about one q-dimension each. This is why the proof
must keep the hard-trace state labels finite and canonical: duplicate certificates cannot be
allowed to grow into a hidden `N^Omega(d)` or `q^Omega(1)` loss.

### Canonical-State Constants Lemma

Work with exact nested zero witnesses and atomic profiles as in
`rfc_multilayer_flag_transition_theorem.md`. For a minimal hard trace step:

```text
s = a = 5,
A subset S,
```

we have `A=S`. Therefore, after the singleton witness set `S` is fixed, the visible-support label
has no extra choice. The remaining real choices fall into the following buckets:

```text
already charged:
  - zero witness and paired/singleton split choices in the outer first-moment sum;
  - local root-line variety size in the q^-9 hard-row charge;
  - ancestor subspace choices in E_anc;
  - terminal shortened-kernel rank event in rho_term;

finite canonical labels:
  - active edge order and merge/delete labels alpha,beta;
  - local component/matroid type on five support coordinates;
  - hard/all-paired trace word of length at most d;
  - finite root-fiber normalization for the determinant-1 law;
  - Gaussian prefactors Gamma_q^O(d).
```

The duplicate-certificate rule is essential. Different bases of the same visible two-plane,
different bases of the same parent/kernel subspaces, marked-core labels that collapse to the same
exact support, and unconsumed tracked sublines are not events. They may be used to prove a bound
for one atomic event, but they cannot multiply the event after the canonical state has been chosen.

For the default target, a conservative illustrative budget is:

```text
N^32 * (q+1)^2 * 52^11.
```

The helper:

```text
python scripts/rfc_distance_analysis/rfc_state_constant_budget.py \
  --poly-degree 32 \
  --q-factors 2 \
  --finite-labels 52 \
  --label-power 11 \
  --margin 7
```

reports:

```text
qdim_total = 5.98988154,
slack      = 1.01011846.
```

This is not a final constants proof, but it shows the scale: a canonical hard-trace state count of
roughly this size fits the toy margin. A much coarser `N^64` count plus any `q+1` factors would not
fit, so the implementation/theorem must avoid broad grouped profiles that reintroduce duplicate
certificate multiplicity.

### Padded Atomic Constants Target

The previous budget deliberately spends `52^11` as a placeholder for finite local labels. A safer
atomic target is to allow a per-level alphabet of size:

```text
3328 = 2 * 52 * 32.
```

The intended interpretation is:

```text
2   hard/all-paired trace symbol,
52  partition/component label on the five minimal singleton coordinates,
32  bounded local normalization bucket for the r_gen<=5 minor factor, determinant-1 root
    normalization, and Gaussian endpoint constants.
```

Here `52` is the Bell number `B_5`. The minimal connected row itself uses only the connected
partition, but allowing every partition of the five active singleton coordinates makes the bound
stable under grouped component labels without paying for support choices again.
The determinant-`1` nonzero-root normalization is the injective affine-root map stated in
`rfc_distance_certificate_theorem.md`; it contributes only the finite factor
`(q/(q-1))^a` on a size-`a` singleton support, not another `q+1` dimension.

This is still not charging support choices: in an atomic profile the nested witnesses `B_i`, the
paired/singleton split `P_i,S_i`, and the exact singleton support `A_i=S_i` are already part of the
outer first-moment sum and local row. The padded count only covers labels that survive after those
objects are fixed.

Keeping the same conservative residual polynomial pad gives:

```text
N^32 * (q+1)^2 * 3328^11.
```

The helper reports:

```text
python scripts/rfc_distance_analysis/rfc_state_constant_budget.py \
  --poly-degree 32 \
  --q-factors 2 \
  --finite-labels 3328 \
  --label-power 11 \
  --margin 7

qdim_total = 6.50550654,
slack      = 0.49449346.
```

Thus the finite-constant blocker is now quite specific: prove that the minimal hard-trace
canonicalization really leaves at most this padded atomic label count, and prove that the
determinant-1/root-frame factors are globally bounded by two `q+1`-scale factors for the exposed
toy trace. If either point fails, the constants proof must refine the residual `N^32` pad downward
by using the exact witness/support accounting more aggressively.

### Padded Canonicalization Lemma

For the default target `d=11`, after fixing the exact nested zero witnesses and atomic
paired/singleton split profile, the minimal hard-trace contribution can be injected into:

```text
{residual polynomial labels}
  x {0,1}^d
  x {partitions of a five-set}^d
  x {local finite buckets}^d
  x {at most two frame lines}.
```

The corresponding count is bounded by:

```text
N^32 * (2 * 52 * 32)^11 * (q+1)^2.
```

Here the factors have the following proof meaning.

```text
residual polynomial labels:
  order-preserving insertion/deletion labels alpha,beta;
  choice of the canonical hard spine when several equal children are available;
  trace endpoint, first/last active depth, and log-sum over shortened-rank terminal rows.

{0,1}^d:
  whether a level is a minimal hard singleton burst or all-paired compression.

52^d:
  a padded component/partition label for the five active singleton coordinates.

32^d:
  finite local constants: r_gen<=5 first-drop minor factor, determinant-1 nonzero-root
  normalization, bounded Gaussian prefactors, and the finite choice between equivalent local
  normal forms.

(q+1)^2:
  reserved for at most two decomposable marked-line/frame completions at entry/exit boundaries.
```

The injection is canonicalized as follows. Given an actual parent contribution, first discard all
basis data and keep only the subspaces and exact supports. If several child insertions or equal
dimension/zero-budget layers represent the same contribution, keep the lexicographically first
order-preserving `alpha,beta` merge map. If several hard spines are compatible with the same
nested flag, keep the first one by depth and child index. These tie-breaks are functions of the
already fixed atomic profile and therefore introduce only the residual polynomial labels above;
they do not create new q-dimensional events.

For a minimal hard row, `A=S` and `|A|=5`, so there is no additional support label once the
singleton witness set is fixed. Exact-support root compatibility for a tau-two visible plane fixes
one affine root line per coordinate; by the determinant-`1` normalization this contributes only
the finite `(q/(q-1))^5` factor already assigned to the local bucket. The local first-drop
Schwartz-Zippel proof contributes the finite `r_gen<=5` factor and the q-dimensional charge
`q^-9`; it does not require choosing a new projective frame.

The marked-line factor `q+1` is not paid per hard row. It appears only for the decomposable
`|A|=2,delta=2,comp=2` boundary row, where the second component line inside a fixed child
two-plane is genuinely not determined by a marked child line. Therefore a minimal hard trace that
does not pass through that boundary row has no `q+1` frame loss, and a grouped toy trace with at
most two such boundary interfaces is covered by the explicit `(q+1)^2` reserve. Any recurrence
profile with more decomposable boundary interfaces is not part of this minimal hard-trace state; it
must be charged by the joint marked-line recurrence as a separate structural row.

All q-dimensional linear choices left after this canonicalization are exactly the three terms in
Lemma 4:

```text
9H          local first-drop charges,
rho_term    terminal shortened-kernel rank event,
E_anc       ancestor subspace choices.
```

Thus, for the hard-compatible near-dimension toy:

```text
9H + rho_term - E_anc = 7
```

and the padded constants above cost:

```text
6.50550654
```

q-dimensions at `N=16384,q=2^128`, leaving:

```text
0.49449346.
```

This closes the finite-label budget for the toy trace, conditional on the shortened-kernel rank
recurrence supplying the stated `rho_term` and on the profile containing at most two decomposable
marked-line boundary interfaces.

### Boundary-Interface Separation Lemma

The phrase "at most two decomposable marked-line boundary interfaces" should be read segmentwise.
A minimal hard segment is a maximal consecutive rank-trace segment whose non-paired active rows are
minimal connected first-drop rows:

```text
tau = 2,
s = a = 5,
delta = 3,
comp = 1,
g = 1,
theta_2 = -1.
```

All other active tau-two rows are segment boundaries, not internal hard labels. In particular, the
decomposable marked-line row has:

```text
a = 2,
delta = 2,
comp = 2,
K = L = 0.
```

It cannot be a connected first-drop hard row because `a<5`. When it appears, the transition
supplies a joint marked-line child state:

```text
(2,z_V), (1,z_V+1)
```

and pays its own frame-completion factor `(q+1)` through the marked-line recurrence. The recurrence
therefore cuts the hard segment at this row. It is not legitimate to keep following the same
minimal hard-trace potential while silently multiplying another `q+1` frame factor.

Consequently, one maximal minimal-hard segment can have at most one marked-line interface on the
way into the segment and at most one on the way out. Those two possible interfaces are exactly the
explicit `(q+1)^2` reserve in the padded canonicalization bound. A profile with more decomposable
interfaces is represented as several hard segments separated by joint marked-line transitions; each
marked-line transition is charged by:

```text
C_row * (q+1) * F_child((2,z_V), (1,z_V+1)),
```

not by the minimal hard-segment constants budget.

This separation is deterministic from the atomic labels:

```text
minimal hard row:       a=s=5, connected first-drop;
decomposable boundary:  a=2, delta=2, comp=2;
larger-support row:     a>5 or s>5, charged by its own local profile/residue.
```

Thus the constants proof does not need to absorb an unbounded number of frame factors inside one
hard segment. Any attempt to group several marked-line rows into the same hard segment is a
duplicate-certificate overcount and must be refined back into atomic profiles.

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
