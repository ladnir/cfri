# RFC Covering Flag-Lift Lemma Target

Scope: original non-systematic RFC, depth-5 base-seal route.

Status: theorem target motivated by the `--cover-lift-mode all` diagnostic.

## Motivation

The current two-layer flag checkpoint counts parent lift multiplicity with Gaussian factors. This
is safe for counting all parent subspaces, but it can be far too pessimistic for an existence
certificate.

For distance we need:

```text
there exists a bad parent line.
```

We do not need to count every bad line inside one higher-dimensional child container separately if
the same child container certifies all of them.

The diagnostic evidence is:

```text
flag checkpoint, no lift cover:       crossing_z = 137
cover tau0 lifts:                     crossing_z = 135
cover tau1 lifts:                     crossing_z = 137
cover tau2 lifts:                     crossing_z = 129
cover tau0+tau1 lifts:                crossing_z = 133
cover tau0+tau2 lifts:                crossing_z = 61
cover tau1+tau2 lifts:                crossing_z = 129
cover all lifts:                      crossing_z = 35
cover all, z=34 vector log2:           27.64399707
cover all, z=34 projective heuristic: -100.35600293
```

Thus the base seal is sensitive to covering/projectivization, but the `all` cover run is
anti-conservative. The valid proof target is to replace crude independent lift/local products by
quotient-incidence counts, not to erase quotient multiplicity.

## One-Layer Covering Map

Fix a parent zero witness split:

```text
P = paired child coordinates,
S = singleton child coordinates.
```

For a bad parent subspace:

```text
W <= H_h,
dim W = t,
```

define:

```text
R   = visible image of W on S,
A   = supp(R),
tau = dim R,
K   = ker(W -> R),
V   = pi(W),
L   = pi(K).
```

The deterministic zero propagation is:

```text
V vanishes on P union (S \ A),
L vanishes on P union S.
```

For `tau > 0`, also record the exact local quotient/root datum:

```text
Q       = W/K,
R       = image of Q on A,
ell_A   = projective root-line assignment on A,
local layer label h if tau=2.
```

The proposed canonical certificate of `W` is:

```text
Cert(W) = (P,S,A,tau,L<=V,full quotient datum R,ell_A,local layer labels),
```

with `L` omitted if `K=0`. Ties must be broken canonically when a line has extra zeros or a
smaller visible support than the chosen witness.

## Fiber Principle

For fixed child flag and fixed local quotient/root datum:

```text
L <= V,
R,
ell_A,
```

every parent subspace `W` in the fiber that maps to this datum satisfies the same requested zero
witness:

```text
paired zeros:
  because V is zero on P;

singleton zeros outside A:
  because V is zero on S \ A;

singleton zeros on A:
  because R is root-compatible with ell_A;

kernel directions:
  because L is zero on all of S.
```

Therefore an existence certificate should count this fiber once at the container level, not with
the Gaussian number of possible parent lifts:

```text
[2 dim V choose t]_q
```

or its two-layer analogue:

```text
q^{kappa(2 dim L-kappa) + tau(2 dim V-t)}.
```

The local quotient/root datum is still counted, including invisible-fiber dimensions outside the
visible singleton support. The claim is only that after this quotient datum is fixed, duplicate
extensions inside the same child container are not separate container events.

## Candidate Lemma

Let `Phi` be an atomic transition profile containing:

```text
P,S,A,tau,
child flag L <= V,
exact local root-line/quotient layer labels,
canonical tie-breaking labels.
```

Let `Child(Phi)` be the set of child flags with:

```text
dim V = r1,  V zero count >= p+s-a,
dim L = r0,  L zero count >= p+s.
```

Then the parent bad-line event contribution for profile `Phi` is bounded by:

```text
Contribution_h(Phi)
  <= Split(Phi)
     * LocalContainer(Phi)
     * F_{h-1}(ChildFlag(Phi)),
```

where `LocalContainer(Phi)` includes:

```text
root probabilities,
exact-support root-line counts,
visible quotient incidence counts,
invisible-fiber dimensions for quotient data,
finite projective/frame constants,
```

but does not include duplicate Gaussian extension multiplicity after the quotient datum has been
fixed.

For a final projective distance event, the top line is counted projectively. If an auxiliary script
prints nonzero-vector moments, subtract one factor of `q` before comparing to `2^-lambda`.

## Safe Subcases And Correction

### Tau Zero

If:

```text
tau = 0,
A = empty,
K = W,
```

then the singleton block is completely invisible. The only child datum is:

```text
V = pi(W),
V zero on P union S.
```

Every parent subspace `W` inside `V+V` has the requested paired and singleton zeros, because both
child projections are zero on every requested child coordinate. Therefore for an existence bound:

```text
count V once,
do not multiply by # { W <= V+V }.
```

The only finite data left are the split choices and the child container event `F_{h-1}((dim V,
p+s))`. This proves tau-zero lift covering as a direct container argument.

### Tau One: Correction

The previous tempting statement was: fix the child flag and visible support, then count the parent
fiber once. That is too optimistic unless the full quotient line datum is also counted.

Assume:

```text
tau = 1,
K = ker(W -> R),
dim R = 1.
```

Fix the child flag:

```text
L = pi(K) <= V = pi(W),
```

and fix the full visible quotient datum:

```text
R <= (V+V)/(L+L)
```

with exact support `A` on the singleton block and root-line assignment `ell_A`.

The child zero budgets are:

```text
V zero on P union (S \ A),
L zero on P union S.
```

For every parent lift in this fiber:

```text
kernel directions vanish on all S through L,
visible quotient directions vanish on S \ A through V,
visible quotient directions vanish on A by the fixed root line ell_A.
```

Thus all parent lifts in the fiber are bad for the same witness. However, the quotient line `R`
is itself an event variable. For a fixed child container there can be many possible quotient lines,
and the probability that at least one is root-compatible grows with the quotient ambient
dimension. Therefore a proof may count the container tuple:

```text
(L <= V, R, ell_A)
```

once, but it may not omit the count of possible `R`.

The tau-one local count is therefore:

```text
root factor q^-a
times the number of exact-support quotient lines R in the represented quotient.
```

This is exactly the support-subcode line count from `delta(A)`, with finite projective constants,
plus any invisible-fiber dimension from quotient coordinates outside the singleton support. In
other words, tau-one covering removes duplicate extensions of a fixed quotient line, but it does
not remove quotient-line incidence.

### Tau Two

The same covering philosophy should apply to tau two, but this is the live proof obligation rather
than a closed subcase. The local datum must include:

```text
R <= (V+V)/(L+L),
dim R = 2,
exact support A,
root-line layer X_h(A),
```

and for the decomposable `a=2,delta=2,comp=2` row it must include the joint marked-line/frame child
state. The tau-two branch is where a proof can accidentally reintroduce either:

```text
1. a product of child moments over shared randomness, or
2. a Gaussian quotient-lift factor that the container map was supposed to remove.
```

Closing tau two means proving the weighted exterior/root-line count, including any invisible-fiber
dimension, is the right count of quotient data `R`. It should replace the independent product:

```text
Gaussian quotient lift * local charge computed in the wrong ambient,
```

but it does not justify setting the quotient-lift exponent to zero.

## Anti-Conservative Shortcut Retired

The diagnostic:

```text
--cover-lift-mode all
```

sets all lift exponents to zero. This is useful only as a sensitivity test. It is not a candidate
theorem.

Counterexample shape: take a fixed child container `V` and a singleton block `A` with `a` roots.
If the quotient ambient has dimension `m`, the event that some quotient line is compatible has
rough exponent:

```text
q^(m-1-a)
```

in the rare-event range. Counting the container once with only a root factor `q^-a` misses the
projective quotient-line family `q^(m-1)`.

For tau two the same issue is the family of quotient planes. The valid replacement is the
incidence/fiber expression:

```text
fiber_qdim(Q,A) + theta_2(Q|_A,A),
```

or the coarse exact-support Grassmann cap:

```text
2(m-2) - |A|.
```

Those terms are quotient data counts. They cannot be dropped.

## Proof Obligations

1. Canonical covering: every bad parent line with a fixed exact zero witness maps to at least one
   canonical profile `Phi`.
2. No undercounting from fibers: for fixed `Phi`, counting one child container plus local quotient
   datum covers all bad parent lifts in that fiber.
3. Controlled overlap: if one bad line maps to many profiles because of extra zeros or multiple
   root supports, the tie-breaking/profile count is polynomial in `N,d` and fits the finite
   constants bucket.
4. Shared child randomness: the child event is one joint flag event, never a product of separate
   child moments.
5. Local quotient counts: for `tau=1`, use support-subcode line counts; for `tau=2`, use the
   weighted exterior/root-line theorem plus the exact-support Grassmann cap and marked-line row.

## Current Risk

The lemma is plausible but not automatic. The dangerous case is a mixed local profile where only
some parent lifts inside a child container satisfy the singleton roots. In that case the proof must
show that `LocalContainer(Phi)` counts the satisfying quotient/root data sharply enough, instead of
silently covering a larger fiber.

This is why the next implementation should emit both:

```text
container count,
local quotient/root datum count,
```

not just zero out lift factors globally.

## Kernel-Lift Container Cover Lemma

Status: theorem target; supported by the level-3 stress diagnostic below, not yet a completed
certificate lemma.

Fix a one-step transition profile:

```text
P,S,A,tau,
child flag L <= V,
local quotient/root datum Q,
root labels ell_A,
local tau-two layer labels if tau=2.
```

Here `L` and `V` are the child kernel and outer containers, with:

```text
pi(K) <= L <= V,
pi(W) <= V,
```

and zero budgets:

```text
V zero on P union (S \ A),
L zero on P union S.
```

The local datum `Q` contains exactly the quotient information that affects the singleton equations:
for tau one it is the quotient line and root labels; for tau two it is the quotient plane together
with the root-line/layer certificate. It must include the quotient-incidence count. In particular,
the lemma does not delete the projective family of quotient lines or planes.

For a fixed profile as above, all choices of the parent kernel lift:

```text
K_parent <= L + L,
dim K_parent = kappa = t - tau,
```

are duplicate certificates for the same container event. The coarse recurrence pays:

```text
q^{kappa(2 dim(L) - kappa)}
```

for these choices. The container-cover target is to replace that factor by one event after
`L <= V` and `Q` are fixed.

The proof idea is direct. Every vector in `K_parent` maps into `L`, and `L` is zero on all requested
singleton child positions `P union S`. Therefore changing the kernel lift cannot change any
requested parent zero:

```text
paired zeros:
  follow from V zero on P;

singletons outside A:
  follow from V zero on S \ A;

singletons inside A:
  depend only on Q and ell_A, since kernel directions vanish there through L.
```

Thus the fiber over:

```text
(P,S,A,tau,L<=V,Q,ell_A,layer labels)
```

is covered by one transition container. The recurrence may count that profile once, provided the
state it passes upward is the container tuple rather than an arbitrary representative parent
subspace.

The remaining proof obligations are:

```text
1. define the quotient datum Q canonically enough that every parent W maps to at least one profile;
2. prove tie-breaking for extra zeros/support shrinkage costs only polynomial factors;
3. prove later folds consume the carried container tuple without needing to recover the discarded
   kernel-lift representative;
4. keep quotient incidence in LocalContainer(Phi), including tau-one line counts and tau-two
   exterior/root-line counts.
```

### Latest Kernel-Lift Checkpoint

The current scalar use of this subcase is the opt-in diagnostic:

```text
--cover-kernel-lift
```

It removes only the duplicate `K_parent <= L+L` kernel-lift factor after the child flag and
quotient/root datum are fixed. It keeps tau-one and tau-two quotient incidence counted.

Combined with the exact-support filters, support-two high-lift component planes, and the safe
support-three component-plane saving, the depth-5 checkpoint reports:

```text
final_span_1_crossing_z,102
final_span_1_z_report,34,565.92392782
```

The top level-5 row is then locally charged and feeds the child state `(2,15)`. Inside `(2,15)`,
the large tau-one row is reduced by the kernel-lift cover, and the next visible obstruction is the
level-3 table `(4,7)>=(1,8)`: an outer tau-zero child container together with an inner connected
tau-one quotient-line row. This remaining row keeps quotient-line incidence; the missing state is
a marked child line inside a fixed container, not another kernel-fiber cover.

### Canonical Quotient/Root Datum

This subsection fixes the meaning of obligation 1 for the current transition.

Condition on the child code and on the current fold roots. For the singleton child block `S`, let:

```text
ev_S^T : H_{h-1} + H_{h-1} -> F^S
```

be the linear singleton-evaluation map induced by the determinant-1 fold. Its exact formula is not
important here; only linearity and the zero propagation rules are used. For a parent subspace
`W`, define:

```text
K = ker(ev_S^T|_W),
Q_W = W / K,
R_A = image(ev_A^T : Q_W -> F^A),
```

where `A` is the exact visible singleton support. Since `K` is the kernel of the full singleton
map, `Q_W -> F^A` is injective after restricting to the exact support, so:

```text
dim Q_W = dim R_A = tau.
```

The canonical quotient/root datum `Q` is the represented object:

```text
(A, R_A, root labels on A, local tau-two layer labels if tau=2),
```

together with any quotient preimage/frame data that the local incidence bound explicitly counts.
Thus:

```text
tau = 1: Q is a represented quotient line with its root labels;
tau = 2: Q is a represented quotient plane with the exterior/root-line certificate.
```

This definition separates two kinds of multiplicity:

```text
1. quotient incidence:
   choices of represented lines/planes, frames, and root labels that change R_A or its local
   certificate. These are real event data and stay in LocalContainer(Phi).

2. kernel fiber:
   choices of K_parent <= L+L that induce the same child container and the same represented
   quotient/root datum Q. These are the duplicate fibers targeted by the container cover.
```

The key invariance is:

```text
ev_S^T(L+L) = 0
```

whenever `L` is zero on `S`. Therefore changing an unconsumed kernel lift inside `L+L` does not
change the induced singleton quotient/root datum. If a row can see the difference between two such
kernel lifts only by marking an additional hidden subspace inside the lift, then that hidden
subspace is consumed data and is outside the cover.

### Container Composability

This subsection addresses obligation 3 for the unconsumed-kernel case, after the
quotient/root datum has been made canonical.

The recurrence must be interpreted as a container first moment. A state does not count every
subspace representative `W` inside a container; it counts a canonical container diagram plus the
quotient/root data that are actually used by the current fold. For one active layer, the kept data
are:

```text
L <= V,
zero budgets for L and V,
the full quotient/root datum Q used by the current fold,
root/layer labels for Q on A.
```

The forgotten data are the choices of:

```text
K_parent <= L+L
```

that realize the same kernel container `L`. The datum `Q` must include the quotient-line or
quotient-plane incidence and the visible root labels; it is not an uncharged placeholder for those
choices. In particular, if the chosen representation of `Q` depends on `K_parent`, the
canonicalization must record the induced quotient/root object before `K_parent` is forgotten.

The forgetful map is sound for later folds provided no later profile asks for an additional marked
subspace inside `K_parent` that is not already determined by `L`, `V`, or the recorded quotient
datum `Q`.

Formally, let `Phi` be a full multi-level certificate tree. Suppose two certificates differ only by
replacing an internal kernel lift `K_parent` by another feasible lift with the same kept tuple:

```text
(L <= V, Q, root/layer labels).
```

Assume every descendant condition below that edge is expressed only in terms of the child
containers descended from `L` and `V`, and every ancestor condition above that edge is expressed
only in terms of `V` and the quotient datum `Q`. Then the two certificates have identical zero
witnesses and identical carried child diagrams. Hence they map to the same canonical container
certificate and must be counted once.

The proof is by locality of the fold:

```text
1. Descendants see only child-code containers. The kernel lift lives in the doubled parent space;
   after projection, its contribution is contained in L, whose zero budget is already kept.
2. Ancestors see paired zeros through V, singleton zeros outside A through V, and singleton zeros
   inside A through Q and the root labels. Changing only the unconsumed kernel lift changes none
   of these evaluations because the discarded directions project into child containers already
   zero on the relevant singleton block.
3. The quotient incidence is not forgotten. Different Q lines/planes are distinct local events and
   remain counted by LocalContainer(Phi).
```

Therefore the Gaussian kernel-lift factor:

```text
q^{kappa(2 dim(L)-kappa)}
```

is removable exactly for unconsumed kernel lifts.

This is not a quotient-ambient shortcut. If `L` and `V` collapse, or if a tau-positive support is
only visible through the particular kernel representative, the profile must either canonicalize
that quotient/root datum explicitly or route the row through the collapsed-active filter. The
container cover removes only duplicate kernel fibers after the local quotient event has already
been counted.

The condition is important. If a later or parallel profile marks a line:

```text
ell <= K_parent
```

and that line is not determined by `L` or `Q`, then `ell` is a downstream consumer. It must be
included in the state and counted as event data. This is the same distinction used in
`rfc_tracked_kernel_chain_state.md`: unconsumed selected kernel lines are duplicate certificates,
while consumed selected lines are real state.

### Collapsed-Active Exact-Flag Rerouting

This subsection gives the theorem form of the diagnostic `--exclude-collapsed-active`.

In one active transition, the child containers satisfy:

```text
L <= V,
V zero on P union (S \ A),
L zero on P union S.
```

If:

```text
dim L = dim V,
```

then exact containment gives:

```text
L = V.
```

Consequently `V` is also zero on all of `S`. A tau-positive row whose active singleton support
`A` is visible only through the distinction between `V` and `L` is therefore not an exact-support
row of this shape. It must be rerouted to a canonical profile with either:

```text
tau = 0,
```

or a strictly smaller exact visible support after recomputing the singleton image.

The recurrence consequence is:

```text
tau > 0, |A| > 0, dim L = dim V, and z_L > z_V
```

is a duplicate/inexact certificate, not a separate tau-positive event. Removing this family is not
a distance discount. It is exact-support normalization: the same parent witness is counted in the
profile where its actual visible singleton support is used.

### Two-Layer Sibling Consumption Test

For a nested parent flag:

```text
W_1 <= W_0
```

with the same singleton evaluation map `ev_S^T`, let:

```text
K_i = ker(ev_S^T|_{W_i}),
t_i = dim W_i,
tau_i = dim ev_S^T(W_i).
```

Then:

```text
K_1 = W_1 cap K_0,
dim(K_1) = t_1 - tau_1.
```

The proof is immediate from `W_1 <= W_0`: the vectors of `W_1` that vanish on the singleton block
are exactly the vectors of `W_1` lying in the upper kernel `K_0`.

Therefore a lower sibling layer consumes hidden subspace data inside the upper kernel lift exactly
through its own kernel dimension `t_1 - tau_1`.

```text
t_1 = tau_1:
  the lower layer is fully visible and cannot consume the upper kernel fiber.

t_1 > tau_1:
  the lower layer marks a positive-dimensional subspace of the upper kernel; this is consumed
  kernel data and must be carried or charged.
```

This is the algebra behind the classifier column:

```text
top_outer_kernel_unconsumed_by_inner.
```

## Level-3 Flag Stress Row

The marked-plane recurrence diagnostics now isolate a concrete stress row:

```text
level 3 flag: (4,7)>=(2,8)
baseline truncated pair sum: 2094.40570138 bits
coarse/table baseline:       1187.19455102 bits
naive loss:                   907.21115036 bits = 7.08758711 q-dim
```

The dominant bad outer witness is:

```text
p=3, s=1, a=1, tau=1,
child=(4,4), z=3,
charge=1, lift=19.
```

This `lift=19` splits as:

```text
kernel_lift   = kappa(2r0-kappa) = 3(8-3) = 15,
quotient_lift = tau(2r1-t)       = 1(8-4) = 4.
```

The classifier CSV now emits these split columns directly:

```text
top_outer_kernel_lift_qdim
top_outer_quotient_lift_qdim
top_inner_kernel_lift_qdim
top_inner_quotient_lift_qdim
top_outer_kernel_dim
top_inner_kernel_dim
top_outer_kernel_unconsumed_by_inner
```

A diagnostic collapsed-active filter removes the exact-flag overcount where equal-dimensional child
containers force the active singleton support to vanish. That cuts the displayed loss to
`4.08510402` q-dimensions but does not close the row; the next bad row still has a tau-one
kernel-lift factor.

The decisive diagnostic is the fixed-table kernel-cover what-if:

```text
python -B scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py \
  --level 3 \
  --outer-state 4,7 \
  --inner-state 2,8 \
  --term-limit 300 \
  --kernel-cover-mode unconsumed-container
```

It gives:

```text
pair sum:              940.85227227 bits
coarse/table baseline: 1187.19455102 bits
saving:                246.34227875 bits = 1.92454905 q-dim
```

The stricter sibling-safe diagnostic does not cover every displayed kernel lift. It covers only an
upper kernel fiber when the displayed lower layer is fully visible:

```text
python -B scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py \
  --level 3 \
  --outer-state 4,7 \
  --inner-state 2,8 \
  --term-limit 300 \
  --kernel-cover-mode sibling-unconsumed
```

By itself this leaves the consumed-kernel rows dominant:

```text
pair sum:              2092.28967759 bits
remaining loss:        905.09512656 bits = 7.07105568 q-dim
top_inner_kernel_dim:  1
top_outer_kernel_unconsumed_by_inner: no
```

But after exact-flag collapsed-active rerouting:

```text
python -B scripts/rfc_distance_analysis/rfc_flag_bad_pair_classifier.py \
  --level 3 \
  --outer-state 4,7 \
  --inner-state 2,8 \
  --term-limit 300 \
  --exclude-collapsed-active \
  --kernel-cover-mode sibling-unconsumed
```

the row closes:

```text
pair sum:              1071.43723477 bits
coarse/table baseline: 1187.19455102 bits
saving:                115.75731625 bits = 0.90435403 q-dim
```

This is strong evidence for the intended division of labor:

```text
keep quotient incidence;
reroute collapsed active exact flags;
cover only sibling-unconsumed duplicate kernel lifts.
```

Concretely, for fixed child flag `L<=V` and fixed canonical local quotient/root datum `R`, the
recurrence should count the container tuple once and not multiply by the Gaussian number of
possible `K <= L+L` kernel lifts. The proof still has to show that this container tuple is a valid
first-moment event at intermediate flag states and that no later/sibling profile consumes hidden
subspace data inside the forgotten kernel lift.
