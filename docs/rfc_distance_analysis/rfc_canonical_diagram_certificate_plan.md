# RFC Canonical Diagram Certificate Plan

Scope: original non-systematic RFC distance certificate, determinant-1 fold with `T` uniform in
`F^*`.

Status: reset plan and theorem architecture. This is not yet a proof, but it is the intended way
to stop local row-chasing.

## Why Reset

The last several proof iterations found useful local charges, but the workflow became:

```text
run recurrence -> find dominant bad row -> add a local rule -> repeat.
```

That is not a satisfying route to a theorem unless the local rules are instances of one general
certificate principle. The emerging pattern is:

```text
count canonical child diagrams and quotient/root data once;
do not multiply child moments over shared randomness;
do not count duplicate parent representatives after the event data are fixed;
charge rank defects by incidence.
```

This note packages that pattern into a proposed canonical diagram theorem.

## Core Object

A bad top codeword gives a top projective line:

```text
W_top, dim W_top = 1.
```

Unfolding recursively creates a tree of parent subspaces. At one fold, for every active parent
layer `W_i`, define:

```text
K_i = kernel of singleton evaluation on W_i,
V_i = pi(W_i),
L_i = pi(K_i),
Q_i = W_i / K_i        when tau_i > 0,
tau_i = dim Q_i,
A_i = exact visible singleton support.
```

The child zero budgets are:

```text
V_i zero on P_i union (S_i \ A_i),
L_i zero on P_i union S_i.
```

The canonical certificate is a finite diagram:

```text
nodes:
  child containers V_i, L_i, component planes/lines, quotient lines/planes,
  consumed kernel subspaces, incidence/rank-defect markers;

edges:
  containment relations, e.g. L_i <= V_i, M_j <= V_i, lower layers inside upper layers;

labels:
  dimension, zero budget, exact support A_i, quotient type tau_i, local matroid data
  delta(A_i), comp(A_i), layer/codimension label, root labels.
```

Equal-dimension containments are merged and keep the strongest zero budget. Exact-support labels
are recomputed after every merge.

## Canonicalization Rules

Every recursive witness should be mapped to a canonical diagram by the following deterministic
rules.

### C1. Child Code First

Condition on the depth-`h-1` child code first. All child nodes live in the same child-code
instance. Therefore the proof counts one child diagram event, never a product of child moments
unless independent child randomness is explicitly present.

### C2. Exact-Support Rerouting

If:

```text
L_i <= V_i,
dim L_i = dim V_i,
A_i nonempty,
```

then `L_i = V_i`, so the projected parent container is already zero on the claimed active support.
The row is not an exact tau-positive row and is rerouted with smaller actual `A_i`.

The nested tau-zero equal-container filter is the same rule across layers.

### C3. Quotient/Root Data Are Real

For `tau_i > 0`, the quotient/root datum is event data:

```text
tau = 1: a full projective quotient line plus root labels;
tau = 2: a quotient plane plus root-line/layer certificate;
decomposable tau = 2: marked component lines/planes and their child diagram.
```

The certificate may condition a descendant quotient on an already-carried quotient datum, but it
may not delete quotient-line or quotient-plane incidence.

### C4. Kernel Fibers Are Audited

A hidden kernel subspace can be forgotten only if it is unconsumed:

```text
no sibling or descendant diagram node marks a subspace inside that hidden kernel lift.
```

If a lower layer consumes hidden kernel data, the consumed subspace must be carried as a node or
charged by a local incidence bound. This is the theorem-grade version of the scalar/root-kernel
cover diagnostics.

### C5. Rank Defects Pay Incidence

Whenever local freedom increases because coordinate restrictions become dependent, the dependency
must be represented by an incidence marker. The support-three rank-defect lemma is the prototype:

```text
rank defect among three active restrictions
=> child 4-container V lies in one extra hyperplane
=> q^-4 for fixed relation, q^2 projective relations, net q^-2.
```

Thus extra local freedom is balanced by a container incidence condition.

## Dimension Ledger

The theorem should bound every transition by a ledger of q-exponents:

```text
transition exponent
  = split/support choices
  + child diagram moment
  + quotient/root incidence
  + consumed kernel data
  + relation/defect marker count
  - root charges
  - incidence codimensions
  + finite constants.
```

The following factors must not appear as independent multipliers after their data are already in
the diagram:

```text
duplicate parent lifts inside fixed child container;
duplicate kernel lifts K_parent <= L+L when unconsumed;
fresh descendant quotient lines/planes that are already conditioned on a carried full quotient;
products of child moments for two nodes in the same child code.
```

This ledger is the proposed replacement for ad hoc local row repair.

## Block Grammar

The current local lemmas can be organized as these blocks.

| Block | Diagram Data | Q-Exponent Rule | Status |
|---|---|---|---|
| Paired descent | child container only | exact child zero propagation | closed algebra |
| Tau-zero container | `V` zero on `P union S` | count child `V` once; no parent lift | theorem-shaped |
| Collapsed active | `L=V` with active `A` | reroute to smaller exact support | theorem target |
| Nested tau-zero equal container | lower tau-zero node equals upper child projection | reroute upper active support | local lemma target |
| Tau-one line | full quotient line `R` plus root labels | support-subcode/projective line incidence | local lemma target |
| Tau-one full-line carry | carried `R`, transition map `phi` | conditional line count `q^dim ker phi` | proof sketch |
| Support-two diamond | `V >= M_1,M_2 >= L` | one joint child diagram, not product | local theorem target |
| Support-two high-lift | two component planes in codim-one slices of fixed `V` | `q^4` instead of `q^8` post-root | local theorem target |
| Support-three stratified | three component planes plus rank-defect marker | `q^2` post-root in both rank-three and defect strata | local theorem target |
| Support-four root-kernel | root-compatible 4D container | count container once after quotient/root data | diagnostic theorem target |
| Kernel-fiber cover | unconsumed `K_parent <= L+L` | no Gaussian kernel-lift multiplier | theorem target |
| Consumed kernel | lower node marks hidden kernel subspace | carry node or charge containment | theorem target |
| Shortened rank defect | enlarged child kernel `H(B)` | expose rank event `rho_h(D,z)` | conditional target |

The goal is to prove that every one-step witness expands into a finite composition of these
blocks, with finite constants bounded globally.

The current block IDs and test cases are also emitted by:

```text
scripts/rfc_distance_analysis/rfc_block_grammar_ledger.py
```

That script is deterministic bookkeeping for the next LP/potential pass; it is not a profiler.

The first coarse potential probe is:

```text
scripts/rfc_distance_analysis/rfc_block_potential_probe.py
```

It checks the current stress transitions against a simple template:

```text
Phi_h(D) =
  level_weight * h
  + dim_weight * sum(node dimensions)
  - zero_weight * sum(node zero budgets)
  + node_weight * number_of_nodes.
```

This is only a diagnostic. With unconstrained feature weights, the best probe is trivial and asks
for `level_weight = 2.13994737` q-dimensions, bottlenecked by the support-three stratified row.
With a modest positive zero reward and no block credits:

```text
python -B scripts/rfc_distance_analysis/rfc_block_potential_probe.py \
  --zero-grid 0.2:2:0.05 --top 1 --show-transitions
```

the bottleneck remains:

```text
support3_stratified_to_3_6, margin 0.00000000
```

and requires `level_weight = 3.93994737` q-dimensions. Interpretation: a naive global
zero/dimension potential does not yet see the support-three incidence credit as a reusable
theorem block.

The block-credit probe adds explicit ledger offsets by block ID. These are still diagnostics, not
certificate claims:

```text
python -B scripts/rfc_distance_analysis/rfc_block_potential_probe.py \
  --zero-grid 0.2:2:0.05 --credit-profile local-incidence --top 1 --show-transitions
```

Using only the support-three incidence credit, the best traced potential has
`level_weight = 3.00795584`. The tight row is the top tau-one/root-kernel feed:

```text
top_tau1_to_2_15, margin 0.00000000
support3_stratified_to_3_6, margin 1.06800847
support2_diamond_to_3_2_ge_1_4, margin 0.92385528
flag_tau1_tau0_to_base_flag, margin 3.13837075
```

Adding the suspected support-two quotient-diamond credit as a sensitivity check leaves the same top
row tight and gives the support-two row almost three q-dimensions of slack. Adding the broader
`current-target` ceiling credits makes the top row slack by `0.93199153`, the support-two row slack
by `1.85584681`, and returns the tight row to support-three with `level_weight = 1.93994737`.

The `current-target` profile is now known to be too generous on the top edge. The audit:

```text
python -B scripts/rfc_distance_analysis/rfc_tau1_carry_kappa_audit.py
```

reports that `top_tau1_to_2_15` has no descendant tau-one line edge, its parent tau-one profile has
`charged_postroot_qdim = -1`, and the same row has:

```text
kernel_dim = 0
kernel_lift_qdim = 0.
```

Therefore neither `tau1_full_line_carry` nor `kernel_fiber_cover` is a legal credit for that
immediate transition. The audited profile:

```text
python -B scripts/rfc_distance_analysis/rfc_block_potential_probe.py \
  --zero-grid 0.2:2:0.05 --credit-profile audited-top-kernel --top 1 --show-transitions
```

removes both top credits. It gives `level_weight = 3.00795584`; the top row remains tight, while
support-three has `1.06800847` q-dimensions of slack and support-two has `2.92385528`.

Interpretation: the block grammar is coherent enough to move the bottleneck in predictable ways.
The next theorem work is now a sharper fork: either find a different top-row theorem for
`top_tau1_to_2_15`, or accept that the current block-ledger route is missing a top-specific
ingredient. Support-three incidence still behaves like a reusable block, but it is no longer the
next obstruction.

The top carry/root-kernel contract is now recorded in:

```text
docs/rfc_distance_analysis/rfc_top_tau1_root_kernel_carry_block.md
```

The key restriction is that the broad tau-one root-kernel diagnostic is not theorem-grade. A legal
line-carry transition must carry the full quotient line, compute the descendant projective-fiber
dimension `kappa_phi`, and split hidden fibers into consumed and unconsumed parts. The current top
edge supports neither line-carry nor kernel-fiber credit.

## Current Dominant Rows As Tests

### Test 1: Level-3 `(4,7)>=(2,8)`

This old stress state is not a scalar product row. Its top rows are:

```text
support2-quotient-diamond + tau1-quotient-line.
```

The grammar route is:

```text
child diagram: V >= M_1,M_2 >= L;
tau-one line: count full line or carry it through phi;
kernel lift: cover only if sibling-unconsumed.
```

This test checks C1, C3, and C4 simultaneously.

### Test 2: Support-Three Row `(2,19) -> (4,8)`

The safe count left:

```text
a=3, delta=3, comp=3, tau=2, K=0, dim V=4
```

dominant at `408.56599456` bits. The grammar route is:

```text
rank-three active restrictions:
  component planes fixed up to constants, q^2 local family;

rank-defect restrictions:
  add a projective relation marker and extra hyperplane incidence,
  q^2 relation count minus q^4 hyperplane incidence.
```

This test checks C5. It is recorded in:

```text
docs/rfc_distance_analysis/rfc_support_three_rank_defect_incidence.md
```

### Test 3: Root-Kernel Cover Rows

The latest root-kernel probes remove support-two/support-four/tau-one post-root container families
only after quotient/root data are fixed. The grammar route is:

```text
if the discarded fiber is unconsumed:
  count the canonical container tuple once;
else:
  create a consumed-kernel node and charge/carry it.
```

This test is the main remaining risk. If hidden fibers are consumed later and the state does not
record them, the cover is false.

### Test 4: Stratified Checkpoint Residual

With:

```text
--support3-component-plane-mode stratified
```

and the current root-kernel probes, the depth-5 diagnostic reports:

```text
final_span_1_crossing_z,72
final_span_1_z_report,34,200.01114103
```

The exposed residual path routes through:

```text
(2,15) -> (3,6) -> (3,2)>=(1,4).
```

The lower child table `(3,2)>=(1,4)` is already strong:

```text
baseline_log2 =   15.71424552
table_log2    = -240.28575448
```

Therefore the remaining gap is not broad lower table incompleteness. It is a higher-level
container/fiber/carry issue in the canonical ledger.

## Proposed Global Theorem

The target theorem should have this form.

For every depth `h`, dimension/zero query, and exact recursive witness, there is a canonical
diagram certificate `D` such that:

```text
bad witness -> D,
event(D) implies the required child diagram events and local root/incidence conditions,
sum_D Pr[event(D)] upper-bounds the first moment.
```

Moreover, each one-step diagram transition has exponent bounded by the block grammar ledger above.
The finite number of canonical split/support/ordering choices contributes:

```text
C(h,c,tau,N) = poly(N,h,c) * product of fixed Gaussian/projective constants.
```

The final theorem then plugs these exponents into the finite recurrence and proves:

```text
B_d(1,k+e) <= 2^-lambda.
```

## What Would Falsify This Route

This plan can fail in useful, visible ways:

```text
1. A row needs a new kind of node not expressible as container, quotient/root datum, consumed
   kernel, or incidence marker.
2. A kernel/root-container cover discards data later consumed by a descendant, and carrying it costs
   more than the current slack.
3. A rank-defect stratum has extra freedom without an incidence codimension.
4. The finite constants for canonical diagrams exceed the remaining q-dimensional slack.
5. The block grammar admits no dual potential/LP certificate with positive drift.
```

If one of these happens, continued row-by-row repair is bad evidence for the target or for this
proof route.

## Next Concrete Work

1. Rewrite the root-kernel and tau-one carry diagnostics as canonical diagram transitions with an
   explicit consumed/unconsumed audit.
2. Build a small block-ledger verifier that takes the current dominant rows and checks which block
   identities are being used.
3. Search for a dual potential over block states:

   ```text
   Phi(state) = alpha * zeros - beta * dim + diagram penalties.
   ```

   Every block transition should satisfy:

   ```text
   local exponent + Phi(child diagram) <= Phi(parent diagram) - slack.
   ```

4. Only after the potential/ledger is stable, return to the certificate driver and replace
   diagnostic flags by theorem flags.
