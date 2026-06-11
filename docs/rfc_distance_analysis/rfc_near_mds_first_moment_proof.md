# RFC Near-MDS First-Moment Proof Target

This note formalizes the small-gap-to-MDS direction for original non-systematic RFC-style codes.

## Goal

Let `G` be the `k x N` generator matrix of the original RFC, with:

```text
k = 2^d
N = c k
q = |F|
```

A zero set `Z` of size `z` supports a nonzero codeword vanishing on `Z` iff:

```text
rank(G_Z) < k.
```

Therefore, distance greater than `N-z` follows from:

```text
Pr[exists Z, |Z| = z, rank(G_Z) < k] <= 2^-lambda.
```

Exact MDS would require this at `z=k`. For RFC-style random foldable codes, `z=k` is not the right
target. The near-MDS target is:

```text
z = k + e
```

where `e` is the smallest integer making the first moment small.

## First-Moment Theorem Shape

The first attempted rank-tail lemma was:

```text
For every fixed Z with |Z| = z >= k,
Pr_T[rank(G_Z(T)) < k] <= C(d,c,z) q^-(z-k+1).
```

Here `T` denotes all sampled RFC fold challenges.

This lemma is false as stated for RFCs. The obstruction is structural and comes from sibling pairs.
If `Z` consists entirely of root sibling pairs, then after the determinant-`1` local change of
basis, the parent selected-column matrix is block diagonal:

```text
G_d[Z]  ~  diag(G_{d-1}[U], G_{d-1}[U])
```

where `U` is the set of selected lower coordinates and `|Z| = 2|U|`. Therefore:

```text
rank(G_d[Z]) < 2k_{d-1}
  iff
rank(G_{d-1}[U]) < k_{d-1}.
```

If `|Z| = k_d + e`, then `|U| = k_{d-1} + e/2`. The fixed-set failure exponent is therefore
controlled by the child excess `e/2`, not the parent excess `e`. So a uniform
`q^-(z-k+1)` fixed-set tail cannot hold.

The right theorem is shape-sensitive.

## Corrected Shape-Sensitive Target

Define:

```text
B_d(z) = E_T[# {Z subset [N_d] : |Z|=z and rank(G_d[Z]) < k_d}].
```

Distance greater than `N_d-z` follows from:

```text
B_d(z) <= 2^-lambda.
```

The target is a recurrence proving that sibling-pair structure is the worst case, while singleton
coordinates add fresh random constraints. The paired-only obstruction gives the exact recursive
term:

```text
B_d(2u) >= B_{d-1}(u)
```

and the desired upper-bound recurrence should have the form:

```text
B_d(z) <= poly(d,c,z) * max over root split shapes of recursively charged child terms,
```

where every singleton selected coordinate contributes one fresh root equation unless it is absorbed
by a lower common-zero event.

## One-Step Matroid Lift Lemma

The correct one-step algebra is a matroid-union statement.

Fix the child generator `G = G_{d-1}` with column matroid `M`. At the parent root, let:

```text
P = child coordinates whose two siblings are both selected
S = child coordinates with exactly one selected sibling
```

Sibling pairs contribute two deterministic copies of `G[P]`. Quotient both child row spaces by
`span(G[P])`. For each singleton `j in S`, the selected parent column becomes:

```text
(h_j, lambda_j h_j)
```

where `h_j` is the image of child column `G[j]` in the quotient and `lambda_j` is a rational
function of the fresh root challenge `T_j`.

Over the rational function field in the singleton challenges, the singleton rank is the two-copy
matroid-union rank:

```text
r_2(S) = min_{A subset S} |S \ A| + 2 r_{M/P}(A).
```

Therefore the generic parent rank is:

```text
rank_generic(parent shape)
  = 2 r_M(P) + min_{A subset S} |S \ A| + 2(r_M(P union A) - r_M(P)).
```

Equivalently:

```text
rank_generic(parent shape)
  = min_{A subset S} 2 r_M(P union A) + |S \ A|.
```

This exactly captures the paired obstruction:

```text
S = empty  =>  rank_generic = 2 r_M(P).
```

It also captures why singletons are valuable: if `S` contains two disjoint bases of the quotient
matroid `M/P`, then the singleton lift spans both quotient copies and completes the parent rank.

The script:

```text
scripts/rfc_distance_analysis/rfc_matroid_lift_check.py
```

checks this formula against sampled parent ranks. Current small checks:

```text
child_depth=2, expansion=4, GF(65537), 500 random shapes: failures=0
child_depth=3, expansion=3, GF(65537), 300 random shapes: failures=0
```

It also has a finite-field failure-profile mode. For structurally full singleton lifts, define:

```text
m = quotient rank after paired groups
excess = |S| - 2m
```

The first heuristic finite-field tail was:

```text
Pr[actual rank drops below generic rank] ~= q^-(excess+1).
```

Small sampled checks looked superficially consistent with this direction:

```text
GF(31), child_depth=3, expansion=4:
  excess=0: drop rate ~= 3.46e-2 ~= q^-1
  excess=1: drop rate ~= 1.00e-2
  excess=2: drop rate ~= 6.67e-4
  excess>=3: no drops in the sampled run

GF(31), child_depth=2, expansion=4:
  excess=0: drop rate ~= 2.00e-2
  excess>=1: no drops in the quick sampled run
```

This suggests the second one-step lemma:

```text
Finite-Field Lift Tail.
Conditioned on the child matroid and on a structurally full singleton lift,
Pr[rank drops below generic] <= poly(|S|) q^-(excess+1).
```

This lemma is still too coarse and is false for general matroids. A direct-sum example shows the
problem. Suppose the quotient child matroid is a direct sum of `m` rank-`1` components, and singleton
columns contain exactly two representatives in many components, with all extra columns concentrated
in one component. The global excess can be large, but any exact two-column component still drops
rank when its two slopes coincide, with probability about `q^-1`.

So the finite-field tail is controlled by a **local bottleneck**, not by total excess.

The correct one-step finite-field exponent is:

```text
alpha_lift(S)
  = min_{C subset S, r(C) < m}
      |S \ C| - 2(m-r(C)) + 1,
```

where `m` is the quotient rank and `r` is the quotient matroid rank.

Equivalently, finite-field drop below generic rank is governed by low-weight dual witnesses in the
quotient/restricted child matroid, not just by `|S|-2m`.

This is not a dead end. It identifies the next necessary state:

```text
structural rank state:
  matroid-union rank / base-packing condition

finite-field state:
  local bottleneck exponent for rank-drop witnesses
```

The near-MDS proof needs to show that after summing over shapes, these local bottlenecks preserve
the same small relative gap.

### Proof Of The Local Lift Bound

Let `H` be an `m x |S|` represented matroid of rank `m`, and consider:

```text
L(lambda) = [ H ; H Lambda ],
```

where `Lambda = diag(lambda_j)` and each `lambda_j` is sampled from `F^*`.

Rank drops below full `2m` iff there is a nonzero pair of dual vectors `(a,b)` such that for every
selected singleton column `h_j`:

```text
a(h_j) + lambda_j b(h_j) = 0.
```

For a fixed witness pair `(a,b)`, each coordinate is one of three types:

```text
a(h_j)=b(h_j)=0:        automatic
a(h_j),b(h_j) nonzero: one allowed lambda_j value
exactly one is zero:    impossible over F^*
```

Let:

```text
C = {j in S : a(h_j)=b(h_j)=0}.
```

Then `a,b` both lie in the annihilator of `span(H_C)`, whose dimension is:

```text
t = m - r(C).
```

The projective family of witness pairs `(a,b)` in this `t`-dimensional annihilator has at most:

```text
O(q^(2t-1))
```

possibilities. For each such pair, the `|S\C|` outside singleton slopes are fixed, while the `C`
slopes are free. Thus the number of bad slope assignments associated with this `C` is at most:

```text
O(q^(|C| + 2t - 1)).
```

The total slope space has size `(q-1)^|S|`, so:

```text
Pr[rank L(lambda) < 2m]
  <= poly(|S|) * max_C q^-(|S\C| - 2(m-r(C)) + 1).
```

This proves the local lift bound:

```text
Pr[rank drop] <= poly(|S|) q^-alpha_lift(S).
```

The generic full-rank condition is exactly `alpha_lift(S) >= 1`, matching the matroid-union
base-packing condition.

## Codimension First-Moment Program

The most precise formulation is not a fixed-set probability tail and not a line-count tail. It is a
codimension first moment.

For each zero-set shape `Z`, define:

```text
alpha_d(Z) = codimension over the challenge torus (F^*)^{N_0+...+N_{d-1}}
             of the variety rank(G_d[Z]) < k_d.
```

Then, over a large finite field, the expected number of deficient `z`-sets should be controlled by:

```text
B_d(z) <= poly(d,N) * sum_{|Z|=z} q^-alpha_d(Z).
```

The near-MDS theorem follows if:

```text
sum_{|Z|=k+e} q^-alpha_d(Z) <= 2^-lambda / poly(d,N).
```

This is the right object because it handles all three cases:

1. **Fixed generic rank.** `alpha_d(Z) > 0` iff `G_d[Z]` is generically full rank.
2. **Paired compression.** If `Z` is a sibling-pair lift of `U`, then:

   ```text
   alpha_d(Z) = alpha_{d-1}(U).
   ```

3. **Finite-field accidents.** Extra singleton coverage only helps if it increases the codimension
   of the local rank-drop variety; global singleton excess alone is not enough.

### Base Case

At depth `1`, with base repetition expansion `c`, every selected column is one of:

```text
L_j = (1 - T_j, T_j)
R_j = (-T_j, T_j + 1).
```

For `z >= 2`, rank deficiency means all selected projective points coincide. For any fixed shape
using `z` selected columns, this imposes at least `z-1` independent affine equalities among the
selected `T_j` values, except that a sibling pair from the same `j` is never deficient because
`det(L_j,R_j)=1`.

Thus the depth-1 codimension matches the random-matrix exponent:

```text
alpha_1(Z) >= z - 1 = z - k_1 + 1.
```

### One-Step Recurrence Target

At depth `d`, split `Z` into root pairs `P` and root singletons `S`. The deterministic generic rank
is governed by the matroid-union formula:

```text
rank_generic(Z)
  = min_{A subset S} 2 r_{d-1}(P union A) + |S \ A|.
```

Rank deficiency can happen in two ways:

1. **Structural child bottleneck.** Some child set `P union A` has smaller rank than expected. This
   contributes child codimension terms `alpha_{d-1}(P union A)`.

2. **Root-slope bottleneck.** The child matroid is structurally sufficient, but the singleton slopes
   fall into the local rank-drop variety of the two-copy matroid lift. This contributes a root
   codimension determined by the local quotient/restriction matroid.

The required recurrence has the schematic form:

```text
alpha_d(Z)
  >= min over witnesses W [
       child_codimension(W)
       + root_lift_codimension(W)
     ].
```

The proof task is to define the witnesses `W` tightly enough that:

```text
sum_{|Z|=k+e} q^-alpha_d(Z)
```

obeys the same crossing scale as the random-rank first moment.

## Replica Zero-Moment Attempt

A direct distance certificate can avoid sampled-MDS language entirely. For `r >= 1`, define the
aggregate nonzero replica moment:

```text
B_d(r,z)
  = sum_{|Z|=z}
      E[# nonzero ordered r-tuples of messages that all vanish on Z].
```

The final distance first moment is `B_d(1,z)`. If `B_d(1,z) <= 2^-lambda`, then with probability at
least `1-2^-lambda` there is no nonzero codeword with `z` or more zeros.

At a root split, let `P` be the sibling-pair groups and `S` the singleton groups, with
`z=2|P|+|S|`. For a singleton coordinate and an ordered `r`-tuple of parent messages, write the two
child value vectors as:

```text
X = (A_1, ..., A_r)
Y = (B_1, ..., B_r).
```

The selected singleton output is zero for all `r` replicas iff:

```text
X + T(Y-X) = 0.
```

Thus:

```text
X=Y=0:                         automatic
span(X,Y) has dimension 1:      at most one root T
span(X,Y) has dimension 2:      impossible
```

The first loose aggregate recurrence ignored the `span(X,Y)` condition and used only the common-zero
alternative. This gives a valid but much too weak upper bound:

```text
B_d(r,z)
  <= sum_{2p+s=z} sum_{c<=s}
       2^s ((q-2)^c/(q-1)^s)
       binom(u,p) binom(n-u,s-c)
       B_{d-1}(2r,u),

u = p+c.
```

The corresponding prototype is:

```text
scripts/rfc_distance_analysis/rfc_replica_zero_moment.py
```

The local replacement lemma is written separately in:

```text
docs/rfc_distance_analysis/rfc_replica_root_compatibility.md
```

Small runs show why this is not the certificate:

```text
depth=2, c=8, q=2^128:
  loose crossing z=9, k=4, e=5
  ideal rank-tail crossing z=4, e=0

depth=4, c=8, q=2^128:
  loose crossing z=97, k=16, e=81
```

The depth-4 trace at `z=97` is:

```text
level,z,p,s,c,u
4,97,48,1,1,49
3,49,24,1,0,24
2,24,8,8,0,8
1,8,0,8,0,0
```

The bottom line is the defect: at depth `1`, the loose recurrence counts `r=8` singleton constraints
as only `8` root equations total. The exact base-layer count charges `r` equations per generic
singleton. For one base singleton group:

```text
sum_{(X,Y) in F^r x F^r} Pr_T[X + T(Y-X)=0] = q^r,
```

not `q^(2r-1)`. Equivalently, root compatibility costs the missing `q^(r-1)` factor by forcing
`span(X,Y)` to be one-dimensional.

So the correct first-moment proof cannot track only common-zero coordinates. It must track
root-compatible rank-one coordinates.

## Root-Compatibility Lemma Target

The next local lemma should replace the loose singleton charge by a rank-pattern charge.

For an ordered `r`-tuple of parent messages and a child coordinate `j`, define the local image rank:

```text
rho_j = dim span{X_j,Y_j} <= 2,
```

where `X_j` and `Y_j` are the two child value vectors across the `r` replicas.

Then a selected singleton at `j` can be satisfied only when:

```text
rho_j <= 1.
```

If `rho_j=0`, it is a common zero and costs no root randomness. If `rho_j=1`, it costs one root
challenge and also imposes the algebraic condition that all replica roots agree. If `rho_j=2`, it is
impossible.

For `r=2`, the rank-one condition is the vanishing of the exterior product:

```text
X_1 Y_2 - X_2 Y_1 = 0.
```

This is the missing constraint in the loose recurrence. For general `r`, all `2 x 2` minors of the
`2 x r` local value matrix must vanish.

The proof target is therefore a finite-replica rank-pattern theorem:

```text
Replica Rank-Pattern Theorem.
For every fixed recursive request pattern that marks coordinates as
  common-zero / root-compatible / impossible,
the expected number of nonzero replica tuples satisfying the pattern is bounded by the random-linear
rank exponent, up to explicit poly(d,N,r) factors and recursive paired-compression terms.
```

This theorem is stronger than the local matroid-union rank statement, but it is aimed at the exact
first moment needed for distance. It is also the natural formal version of the product-recurrence
state in `docs/rfc_distance_analysis/rfc_product_recurrence.md`.

### r=2 Diagnostic

The first nontrivial replica case is now isolated. For `r=2`, root compatibility is one determinant:

```text
X_1 Y_2 - X_2 Y_1 = 0.
```

The script:

```text
scripts/rfc_distance_analysis/rfc_replica_rank1_profile.py
```

compares the exact oriented-singleton generating function against the loose recurrence that treats
every non-common coordinate as root-capable.

For `GF(5)`, child depth `2`, expansion `8`, with `100000` sampled ordered two-replica parent
tuples, the coordinate category rates are:

```text
rank0             0.0015996875
rank1_no_root     0.0381875
rank1_one_root    0.0769096875
rank1_two_roots   0.115181875
rank2             0.76812125
```

These match the closed-form random `2 x 2` local counts over `GF(5)`:

```text
1, 24, 48, 72, 480 out of 625.
```

The exact singleton sum beats the loose common-zero recurrence by:

```text
s=4:   9.44 bits
s=8:   15.52 bits
s=12:  20.39 bits
s=16:  26.09 bits
```

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_categories.csv
docs/rfc_distance_analysis/rfc_replica_rank1_sample_gf5_child_depth2_c8_singletons.csv
```

Interpretation: the missing exponent is real and local. The next theorem should upper-bound
recursive request patterns using rank-`0` and rank-`1` labels, instead of trying to repair the
common-zero-only recurrence.

The current theorem target is stated in:

```text
docs/rfc_distance_analysis/rfc_rank_pattern_induction_target.md
```

### Optimistic Rank-One Recurrence

The loose aggregate recurrence now has a diagnostic switch:

```text
python scripts/rfc_distance_analysis/rfc_replica_zero_moment.py \
  --depth d \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge replica
```

This keeps the same recursive common-zero split, but charges every non-common singleton in an
`r`-replica parent state as `r` field equations. This is the exponent suggested by the local
rank-one aggregate identity. It is not yet a proof because it still needs the global rank-pattern
induction.

Small-depth crossings:

```text
depth  k   loose e   rank-one-model e   ideal random-rank e
2      4   5         1                  0
3      8   25        1                  0
4      16  81        2                  1
5      32  -         2                  1
6      64  -         3                  2
```

This is the strongest numerical signal so far. Once the rank-one compatibility exponent is kept,
the recurrence moves back to the random-rank crossing scale. The remaining risk is formal, not
numerical: prove that recursive rank-`1` request patterns can be counted without losing this
exponent to direct-sum or paired-compression bottlenecks.

There is now one concrete correction to the optimistic model. For an `r=2` singleton exact support
`A`, the local root-line count has two endpoints. Let:

```text
delta = dim U_A
g     = generic two-copy root-line kernel dimension
comp  = comp(U_A).
```

After the root equations, the tau-2 local exponent is bounded by:

```text
max(2g-4, 2delta+comp-4-|A|).
```

The old component-only expression is just the high-kernel endpoint and is false as a full local
bound on dense supports. This is tracked in:

```text
docs/rfc_distance_analysis/rfc_exterior_component_codimension.md
```

The scalar component-aware recurrence by itself is too pessimistic. Its high-replica moments are
dominated by tuples with small replica span dimension. The next state must also track:

```text
t = dim span(m_1, ..., m_r).
```

This diagnosis is tracked in:

```text
docs/rfc_distance_analysis/rfc_replica_span_state.md
```

The first span-aware toy model shows that global `t` is still not enough. The singleton block sees
only the restriction of the replica/message subspace to that block. The state must track the visible
dimension and kernel:

```text
tau_E = restricted span dimension on singleton block E
t - tau_E = directions invisible on E
```

The local visible-span profiler adds one more necessary variable:

```text
a_E = number of coordinates where the visible span has projection rank 1.
```

This is the number of root equations paid by the singleton block. The local state and evidence are
in:

```text
docs/rfc_distance_analysis/rfc_visible_span_local_state.md
```

## Current Proof State

What is now solid:

```text
1. Exact MDS is not the right target for random RFC.
2. The uniform fixed-Z q^-(z-k+1) lemma is false.
3. The one-step generic-rank algebra is matroid union and is validated by small checks.
4. The one-step finite-field rank-drop probability has an elementary codimension bound.
5. The first nontrivial finite-replica singleton condition is the rank-one/exterior equation, and
   small exact/sampled checks show it recovers the exponent lost by the loose recurrence.
6. The `r=2` exterior/root-line exponent is endpoint-aware: the singleton block state must include
   support-subcode dimension, generic two-copy kernel dimension, and component count.
7. A scalar endpoint recurrence is still too coarse; it must also track replica span dimension to
   avoid high-replica proportional-tuple overcounting.
8. A global span dimension alone is still too coarse; the induction must track the visible span and
   kernel on each singleton block.
9. The visible span must be stratified by root support size `a_E`; for `tau=1`, compatibility is
   automatic and the whole cost is the support-subcode profile.
10. The support-containment lemma is now explicit: for root support `A`, compatible visible
    subspaces with support contained in `A` live in `U_A + U_A`, where
    `dim U_A = rank(S) - rank(S \ A)`.
11. The sharper local object is the root-line kernel profile. Fixing projective root lines `ell`
    gives a linear kernel `K_A(ell)`, and exact-support compatible counts are obtained by weighted
    inclusion-exclusion over supports. This recovers the component/exterior savings in the small
    checks without enumerating subspaces.
```

The remaining mathematical task is global:

```text
Global finite-replica rank-pattern sum.
Use the rank-0/rank-1 singleton request recurrence recursively to prove

B_d(1,k+e) <= 2^-lambda

for e at the near-MDS crossing scale.
```

This should be an induction on `d` plus a first-moment sum over root split profiles. The hard part
is proving and summing a state that tracks both coordinate-side structure and replica-side span:

```text
coordinate side: delta(A), generic two-copy kernel dimension g(A), comp(A)
replica side:    t = span dimension of the replica tuple
local side:      tau_S = visible span dimension on the singleton block
root side:       a_S = visible root support size
support side:    delta_S(A) = rank(S) - rank(S \ A)
root-line side:  kappa_A(ell) = dim K_A(ell)
```

The older codimension-sum view remains compatible:

```text
sum_{|Z|=k+e} q^-alpha_d(Z) <= 2^-lambda / poly(N,d).
```

But the finite-replica formulation is currently the sharper route because it counts the actual
distance first moment rather than sampled rank deficiency.

For the all-paired branch, the near-MDS crossing is stable:

```text
d,   k=2048, c=8, q=2^128: e ~= 71
d-1, k=1024, c=8, q=2^128: e ~= 36
2 * 36 = 72
```

So the structured pair obstruction changes the proof architecture but not the expected relative
gap.

More generally, a message that is constant across a top split has only `k/2` degrees of freedom,
and a zero set made of sibling pairs has only `z/2` effective child constraints. Both the message
dimension and the zero-set size compress by the same factor. Repeating this compression down a
subtree gives an effective problem:

```text
k_eff = 2^h
N_eff = c k_eff
z_eff = k_eff + e_eff
z = 2^(d-h) z_eff
e = 2^(d-h) e_eff
```

Thus the relative gap is preserved:

```text
e / N = e_eff / N_eff.
```

For `c=8`, `q=2^128`, `lambda=80`, the baseline crossings are:

```text
k_eff   e_eff   e_eff / (8 k_eff)
1       0       0
2       0       0
4       0       0
8       0       0
16      1       0.00781250
32      1       0.00390625
64      2       0.00390625
128     5       0.00488281
256     9       0.00439453
512     18      0.00439453
1024    36      0.00439453
2048    71      0.00433350
```

This is the main evidence that the correct theorem should still be near-MDS even though the
uniform fixed-set `q^-(z-k+1)` lemma is false.

## If A Shape Tail Is Proved

Then:

```text
Pr[exists deficient Z of size z]
  = B_d(z).
```

In the purely random-matrix model, it is enough to choose `e` so:

```text
log2 binom(N,k+e) + log2 C(d,c,k+e) - (e+1) log2 q <= -lambda.
```

Ignoring shape factors gives the baseline first-moment crossing:

```text
log2 binom(N,k+e) - (e+1) log2 q <= -lambda.
```

## Concrete Crossings Without the C Factor

For `lambda = 80`:

```text
c=4, k=2048, q=2^128:
  e = 53
  z = 2101
  relative distance >= (N-z+1)/N ~= 0.743652
  MDS relative distance ~= 0.750122

c=8, k=2048, q=2^128:
  e = 71
  z = 2119
  relative distance >= (N-z+1)/N ~= 0.870728
  MDS relative distance ~= 0.875061

c=8, k=2048, q=2^256:
  e = 35
  z = 2083
  relative distance >= (N-z+1)/N ~= 0.872925
  MDS relative distance ~= 0.875061

c=8, k=2^25, q=2^256:
  e = 576256
  z = 34130688
  relative distance >= (N-z+1)/N ~= 0.872853
  MDS relative distance ~= 0.875000
```

Thus the predicted loss from MDS is:

```text
relative loss = e / N.
```

At `c=8, k=2048, q=2^128`, this is:

```text
71 / 16384 ~= 0.004333.
```

## What Needs To Be Proved

The hard step is the shape-sensitive recurrence for `B_d(z)`. A direct Schwartz-Zippel bound from a
nonzero minor only gives roughly:

```text
Pr[rank(G_Z) < k] <= degree / q,
```

which is enough to show fixed-subset generic rank but not enough for the near-MDS first moment.

We need a stronger but shape-aware statement: after accounting for recursive sibling-pair lifts,
rank deficiency should still impose enough independent random constraints that `B_d(k+e)` crosses
below `2^-lambda` for small `e/N`.

Useful formulations:

1. **Sequential pivot form.**
   Reveal columns or recursive obligations in an order where, after `k-1` successful independent
   pivots, the final failure requires `e+1` independent linear coincidences.

2. **Kernel-vector / line-count form.**
   Bound the expected number of projective message lines that vanish on `Z`, but stratify by the
   recursive equality pattern of the message. The naive fixed-line bound `q^-z` is false for
   messages with repeated child halves, because paired coordinates collapse to child constraints.

   ```text
   E_T[# vanishing message lines for Z]
   ```

   should be bounded by a recursive complexity measure that charges both message dimension and
   zero constraints at the same tree scale.

3. **Rank-defect stratification.**
   Prove for defect `r >= 1`:

   ```text
   Pr[rank(G_Z) <= k-r] <= C_r q^-r(z-k+r).
   ```

   The `r=1` case gives `q^-(z-k+1)`.

## Relation To The BaseFold Paper

The BaseFold paper proves a lower bound on distance by controlling zero counts of all nonzero
messages. Its recurrence is conservative but already has the right global quantifier.

This near-MDS route tries to replace the paper's threshold recurrence with a rank-tail estimate:

```text
old paper proof:  union over messages / common-zero sets
near-MDS proof:   union over coordinate sets / rank-deficiency tails
```

The rank-tail proof must be strong enough to survive the `binom(N,z)` union. This is exactly where
the previous fixed-subset MDS direction was insufficient.

## Immediate Next Lemma

For the original determinant-`1` RFC form, prove:

```text
Lemma A'.
B_d(k+e) <= 2^-lambda
```

for `e` near the first-moment crossing, e.g. `e=71` at `c=8, k=2048, q=2^128`, up to explicit
slack for the recurrence constants.

The first sublemma should be a one-step split bound. Partition a parent zero set by root groups:

```text
p = number of sibling-pair groups
s = number of singleton groups
z = 2p + s
```

Paired groups recurse deterministically into both children. Singleton groups supply fresh random
root equations. The proof must show that the singleton equations cannot be systematically avoided
except by moving into lower-dimensional child-equality strata, which are then charged recursively.

Equivalently, prove a shape-sensitive exponent `alpha_d(Z)` such that:

```text
Pr_T[rank(G_Z(T)) < k] <= q^-alpha_d(Z)
sum_{|Z|=z} q^-alpha_d(Z) <= 2^-lambda.
```
