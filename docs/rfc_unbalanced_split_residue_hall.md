# RFC Unbalanced Split Residue-Hall Target

This note isolates the remaining core-preservation gap after the one-child and balanced two-child
cases.

## Setup

At a node of output length `K`, let the live row weight split as:

```text
M = a + b,     a,b > 0,     a != b.
```

The parent target core size is:

```text
L = K/M.
```

Let the child output supports be:

```text
U, V subset {0, ..., K/2-1}.
```

In a balanced branch, the child live weights remain powers of two and the induction exposes child
stride cores. In an unbalanced branch, `a` and `b` need not be powers of two, so we should not assume
the exact matched-core theorem applies verbatim inside each child. The optimistic residue model is:

```text
C_a = { r_a mod a } in U,     |C_a| = K/(2a),
C_b = { r_b mod b } in V,     |C_b| = K/(2b),
```

with possible extra child coordinates outside those cores. When `a` or `b` is not compatible with
an exact child core, the row-split/rounding defect must pay for the deviation before this model can
be used.

The parent transform combines the two child values at the same child coordinate `j` into two
siblings. Therefore a parent stride residue `rho mod M` survives only if every child coordinate
appearing under that residue is present in `U union V`, and the selected sibling is not cancelled.
The cancellation part is already controlled by the no-early-gluing lemma. What remains here is the
deterministic child-coordinate coverage problem.

## Parent Residue Projections

For a parent residue `rho`, define the child-coordinate projections:

```text
P_0(rho) = { j :       j  = rho + t M         lies in the left sibling block },
P_1(rho) = { j : K/2 + j  = rho + t M         lies in the right sibling block }.
```

Equivalently, each `P_s(rho)` is an arithmetic progression in child coordinates with step `M`,
restricted to `0 <= j < K/2`.

In the balanced case `a=b=M/2`, each child core modulo `M/2` splits into two sub-residues modulo
`M`, and one of the two parent residue candidates survives. This is why balanced core preservation
is now reduced.

Even under this optimistic model, the unbalanced case has a real obstruction: a child residue modulo
`a` or `b` generally does not contain a full parent projection:

```text
M mod a = b mod a,
M mod b = a mod b.
```

So the exact balanced lift argument cannot be copied. This is the actual obstruction.

## Hall Form

For a candidate parent residue `rho`, define its deterministic hole count:

```text
h_det(rho)
  = |{ (s,j) : j in P_s(rho), j notin U union V }|.
```

This counts missing parent residue positions, not just distinct child coordinates. If both parent
siblings over the same child coordinate are used by the residue, a missing child coordinate creates
two holes. Sibling cancellations can add at most the already-charged cancellation holes. The unbalanced
residue-Hall target is:

```text
min_rho h_det(rho) = 0
```

or, for the virtual-core fallback,

```text
sum over unbalanced nodes of selected h_det(rho) <= H,
```

with `H <= 5` at the current depth-11, `c=8`, `B=64` certificate point.

The second statement is much weaker than exact core preservation, but still strong enough for the
current first-moment bound. The H=8 stress run already fails, so the hole bound must be genuinely
small.

## Charge Interface

The rounded row-split charge is:

```text
Delta_split(a,b;K)
  = ceil(max((K/2)/a, (K/2)/b)) - ceil(K/(a+b)).
```

This charge measures how much larger the child uncertainty lower bound is than the parent target.
It gives extra supported child coordinates beyond the ideal parent core size. The residue-Hall
proof must use those extra child coordinates to cover the projected parent residue positions.

A certificate-sufficient local statement would be:

```text
Either some parent residue has h_det(rho)=0,
or the uncovered positions for a minimizing rho are paid by row-split charge and coalesce globally
into at most five virtual core holes.
```

The first clause preserves the existing counted theorem with no holes. The second clause uses the
experimentally safe virtual-hole fallback.

## Small Adversarial Models

The helper:

```text
scripts/rfc_unbalanced_residue_hall_search.py
```

checks two deterministic relaxations.

First, if `M | K/2`, each parent residue projects to one child residue class modulo `M`, with both
parent siblings over those child coordinates. A support set `S subset {0,...,K/2-1}` kills every
parent residue exactly when it omits at least one coordinate from every child class modulo `M`.
Thus the largest arbitrary `S` with no surviving parent residue has size:

```text
K/2 - M.
```

For the depth-11 root scale, the cheapest unbalanced splits satisfy:

```text
K=2048

M     split charge   cheapest split   child lower bounds   arbitrary disjoint supports can kill all residues
32    5              15+17            69,61                yes
64    2              31+33            34,32                yes
128   1              61+67            17,16                yes
256   1              114+142           9, 8                yes
512   1              205+307           5, 4                yes
1024  1              342+682           3, 2                no
```

So a size-only Hall proof is impossible in the high-`M` regime. Arbitrary child supports satisfying
only the uncertainty lower bounds can hide inside a set that misses every parent residue class.

Second, the script checks an optimistic child-residue model: pretend the unbalanced children expose
single residue classes modulo `a` and `b`, even though non-power child weights do not have an
established exact matched-core theorem. At `K=256`, the most balanced cheapest unbalanced split at
each `M` gives:

```text
K,M,a,b,split_charge,L,child_period,pairs,zero_pairs,min_holes,worst_min_holes,histogram
256,8,3,5,11,32,16,15,0,16,16,16:15
256,16,7,9,3,16,8,63,0,10,12,10:51;12:12
256,32,15,17,1,8,4,255,0,4,6,4:98;6:157
256,64,31,33,1,4,2,1023,66,0,2,0:66;2:957
256,128,63,65,1,2,1,4095,4095,0,0,0:4095
```

The artifact is:

```text
docs/rfc_unbalanced_residue_hall_child_residue_k256.csv
```

This model is not a counterexample to RFC distance, because child residue classes modulo non-power
`a,b` are not known to be attainable sparse child supports. But it is a genuine warning: even
stride-like child structure does not automatically align with the parent modulus. The final proof
must exploit more than "one residue in each child"; it must use the actual recursive/algebraic
constraints that make non-power unbalanced child supports costly.

The depth-11 scale has the same warning in a sharper unit-charge example. Take:

```text
K=2048, M=128, L=16, N=K/2=1024,
a=63, b=65.
```

The rounded row-split charge is only:

```text
ceil(max(1024/63, 1024/65)) - 16 = 1.
```

But in the optimistic child-residue model:

```text
U = { j : j = 0 mod 63 },      |U|=17=ceil(1024/63)
V = { j : j = 35 mod 65 },     |V|=16=ceil(1024/65)
```

every parent residue modulo `128` has at least six missing projected child coordinates:

```text
min_rho child-coordinate coverage holes = 6,
child-hole histogram over rho: 6:2, 7:28, 8:98.
```

One minimizing residue is:

```text
rho = 112
C_rho = 112,240,368,496,624,752,880,1008
covered = 880,1008
holes   = 112,240,368,496,624,752
```

Each missing child coordinate corresponds to two missing parent residue positions, one in each
sibling half. So this relaxed model has at least `12` parent-core holes in the best parent residue.
The statement "unit high-`M` imbalance preserves a parent residue core" is false in this model.

However this is not a low-total-defect local extremizer. The same example has almost no child
support overlap. Even if `|U cap V|` were maximized at `16`, no-early-gluing would force parent
support at least:

```text
2|U| + 2|V| - 2|U cap V| - 1 >= 33,
```

so the parent defect relative to `L=16` is at least `17`. In the displayed residue example the
overlap is smaller, so the defect is larger. Thus the six virtual holes are coupled to many actual
extra output leaves.

This suggests a stronger and probably certificate-sufficient target:

```text
For the selected parent residue, parent-core holes <= alpha * local output defect,
with a small absolute constant alpha.
```

Formula checks of the existing first-moment count support this target. If virtual core holes are
allowed but constrained by:

```text
holes <= alpha * extra,
```

then the depth-11, `c=8`, `B=64` totals are:

```text
alpha=1: total log2 union = -99.60768253
alpha=2: total log2 union = -99.60689084
alpha=5: total log2 union = -21.53925283
alpha=6: total log2 union =  25.89359792
```

The dangerous `H=8` failure came from allowing holes at `extra=0`. The unbalanced residue models
found so far do not behave that way; their holes come with overlap/cancellation extras. Therefore
the best next theorem is constant-defect-coupled virtual-core containment, not absolute actual-core
containment. The proof should aim for `alpha=2`; after the corrected dimension accounting, the
certificate has slack through `alpha=5` and breaks at `alpha=6` under the current crude count.

## Defect-Coupled Local Lemma Candidate

The promising deterministic constraint is simple and does not depend on child residue alignment.

Assume `L=K/M >= 2` is even, so each parent residue `rho mod M` projects to one child-coordinate
class:

```text
C = { rho + tM : 0 <= t < L/2 } subset {0,...,K/2-1}.
```

Let:

```text
R = |C| = L/2,
S = U union V.
```

For an unbalanced split `M=a+b`, both child weights are at most `M-1`. One-copy uncertainty gives:

```text
|S| >= max(|U|, |V|)
    >= ceil((K/2)/(M-1))
    = ceil(R * M/(M-1))
    >= R + 1.
```

Now let:

```text
h_cov = |C \ S|.
```

Then:

```text
|S \ C| = |S| - |S cap C|
        = |S| - (R - h_cov)
        >= h_cov + 1.
```

Every coordinate in `S \ C` produces at least one parent output outside the selected parent residue
core, and usually two. These outputs are distinct for distinct child coordinates, so they are charge
targets. A missing child coordinate in `C` creates two parent-core holes, so the right statement is
not the overly optimistic `holes <= extra`; it is a small-constant coupling between parent-core
holes and local output defect.

The remaining `+1` pays the possible cancellation hole inside the selected parent residue. Indeed,
if a coordinate of `C` lies in both `U` and `V`, local invertibility leaves at least one parent
sibling nonzero, and quantitative no-early-gluing allows at most one vanished sibling over the
fixed residue class. Coordinates of `C` that lie in only one child have both siblings nonzero and
create no cancellation hole.

Thus a candidate local theorem is:

```text
At an unbalanced two-child node with L>=2, every parent residue rho defines a virtual parent
matched core whose parent-position holes are bounded by alpha times the local output defect,
for a small constant alpha, ideally alpha=2.
```

Equivalently:

```text
virtual holes introduced by an unbalanced split <= alpha * local output defect.
```

The `L=1` full-live case is harmless: a parent core has size one, and any nonzero parent output is
already an actual contained core.

This lemma would replace the fragile absolute `H<=5` fallback with the much stronger constrained
fallback:

```text
holes <= alpha * extra.
```

The current first-moment formula is essentially unchanged for `alpha=1,2`, and remains negative
through `alpha=5`. The dangerous terms were precisely the artificial `extra=0, holes>0` cases. The
calculation uses the corrected stride-dimension factor `floor((extra+holes)/(k/m))`, since virtual
holes add outside-core output positions that can complete additional stride classes.

The reproducible arithmetic artifacts are:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_coupled_extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le2extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le5extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le6extra.csv
```

## Why This Is The Right Remaining Object

The field algebra is already doing two jobs:

```text
1. one-copy uncertainty gives the child support lower bounds;
2. no-early-gluing prevents many sibling cancellations over a fixed child core.
```

Unbalanced splitting fails before either of those tools can finish the lift, because the child
stride moduli `a,b` are not the parent stride modulus `M`. The missing step is therefore a finite
residue covering/Hall lemma, not a new random-field identity.

This also explains why the proof cannot simply copy the exact RFC paper. The exact proof only uses
balanced two-child gluing at the full-live node; an unbalanced near-extremizer can be close in
weight while exposing child cores with incompatible moduli.
