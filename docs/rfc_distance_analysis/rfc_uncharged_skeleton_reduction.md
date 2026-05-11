# RFC Uncharged Skeleton Reduction

This note isolates the reduction needed before applying no-early-gluing inside the defect-coupled
virtual-core proof.

## Purpose

The alpha-2 local hole lemma assumes that, after charges have been emitted, the two child outputs at
an unbalanced node have only one relative scalar. This is the same algebraic condition used in the
exact no-early-gluing lemma.

The reduction target is:

```text
charge all child defects and complete extra stride-class dimensions first;
the remaining uncharged child skeleton is one-dimensional on each side.
```

Then a parent sibling cancellation at coordinate `j` imposes one equation on the single relative
scalar between the two child skeleton lines. Two distinct uncharged cancellations would impose two
fresh-random ratio equations, which is generically impossible.

## Inputs

The near-kernel line lemma says that for a matched child core `C_child` plus outside leaves `E_child`:

```text
dim <= 1 + floor(|E_child| / |C_child|).
```

Equivalently, every additional child output direction requires a complete extra stride class of
size `|C_child|`. In the first-moment certificate this is already charged by:

```text
q^floor(|E_child| / |C_child|).
```

For virtual cores, outside leaves have size:

```text
|E_child| = e_child + h_child.
```

so the dimension charge uses:

```text
floor((e_child+h_child)/|C_child|).
```

This is exactly the corrected accounting in:

```text
scripts/rfc_distance_analysis/rfc_near_extremizer_total_union.py
```

## Reduction Statement

At a parent node, suppose the recursive encoder has chosen child virtual cores:

```text
(C_0,H_0,E_0), (C_1,H_1,E_1).
```

Charge:

```text
1. child outside leaves not used by the selected child skeleton;
2. complete extra child stride classes, via the dimension factor;
3. local overlap/residual leaves outside the selected parent virtual residue.
```

After these charges, each child contributes only one uncharged skeleton line:

```text
u_j = alpha U_j,
v_j = beta  V_j,
```

on the coordinates retained for the parent local comparison, with `U_j,V_j` fixed over the child
field and a single relative scalar:

```text
lambda = beta / alpha.
```

Thus every uncharged sibling cancellation has the form:

```text
lambda = L_j(T_j) * U_j/V_j.
```

The parent challenges `T_j` are fresh and independent across child coordinates. Therefore the
quantitative no-early-gluing lemma gives:

```text
at most one uncharged sibling cancellation at this parent node.
```

This is the input used in the alpha-2 local theorem:

```text
c_C + c_O <= 1.
```

## Non-Power Child Weights

In an unbalanced split the child row weights `a,b` are usually not divisors of the child block
length in the same way as the parent row weight. So the local support inequality:

```text
max(|U|,|V|) >= L
```

is deterministic and easy, but it is not by itself the one-scalar algebra needed for
no-early-gluing.

The intended order is therefore:

```text
1. use one-copy uncertainty only to prove enough outside mass;
2. charge the child support outside the selected parent residue;
3. charge every complete extra child stride-class dimension;
4. apply no-early-gluing only to the residual one-line skeleton.
```

This is the remaining algebraic rank obligation. It is strictly narrower than proving literal
actual-core containment for the unbalanced split. The current formulation should be read globally:
fix the final virtual support first, then use the global admissible dimension to pay excess
cancellation equations before applying the local alpha-2 size lemma.

## Rank-Cancellation Exchange

The useful way to state the algebra is not that extra rank is impossible. Extra admissible rank is
allowed, but it is paid before the alpha-2 hole lemma is applied.

Let `r_0,r_1` be the dimensions of the two child output spaces after the child zero constraints and
the selected local outside sets are fixed. The dimension count pays:

```text
(r_0-1) + (r_1-1)
```

extra projective parameters in the local view. In the repaired proof these local parameters are
only a diagnostic; the actual count pays the global admissible dimension. A sibling cancellation at
parent child-coordinate `j` is one linear equation on the admissible parameters plus the one
relative scalar between the two base lines.
Generically:

```text
t cancelled siblings cost t equations,
one equation can be absorbed by the relative scalar,
each additional absorbed equation must use one paid global admissible-rank parameter or a
no-early-gluing probability loss.
```

Thus the local proof can charge:

```text
max(0, t-1)
```

cancellations either to paid global admissible-rank dimensions or to the no-early-gluing
probability loss. In the deterministic charged-tree language, remove all cancellations paid this
way before invoking the alpha-2 size lemma. The residual uncharged skeleton then satisfies:

```text
c_C + c_O <= 1.
```

The audit found that this should not be interpreted as a fresh child-local dimension payment at
every node. The current first-moment count pays only the global near-kernel dimension factor. The
preferred repair is to fix the final virtual support and charge all excess cancellation equations
against the global admissible dimension budget:

```text
docs/rfc_distance_analysis/rfc_global_rank_budget_pivot.md
```

The precise algebraic sublemma is tracked in:

```text
docs/rfc_distance_analysis/rfc_rank_cancellation_exchange.md
```

## Charge Composition Lemma

Fix the final root virtual core `C_root`. For every final output leaf `ell notin C_root`, define
`first(ell)` to be the highest node where the path to `ell` leaves the selected virtual core path.
Then the sets:

```text
E_v = { ell notin C_root : first(ell)=v }
```

partition the outside leaves.

At an unbalanced node `v`, the local alpha-2 lemma should produce two disjoint subsets of `E_v`:

```text
R_v  replacement leaves, one for each newly born virtual hole;
D_v  true defect leaves.
```

with:

```text
|R_v| = h_v,
h_v <= 2 |D_v|.
```

The replacement leaves certify the identity:

```text
supp(Ax) = (C_root \ H_root) union E_root,
|E_root| = |H_root| + e.
```

The true defect leaves certify:

```text
|H_root| <= 2e.
```

The proof is then automatic after summing over nodes, because the sets `E_v` are disjoint:

```text
|H_root|
  = sum_v h_v
 <= 2 sum_v |D_v|
 <= 2e.
```

Balanced-node overlap and cancellation charges use the same first-divergence partition, but they do
not create new holes. They only increase the true defect leaf set. Thus they cannot weaken the
alpha-2 inequality.

Complete extra stride-class dimensions are not a third kind of leaf payment. They are a rank
payment attached to the already chosen outside set:

```text
q^floor((e+h)/|C|)
```

in the count. Their support leaves still lie in the partition `E_v`, but the dimension factor, not
an additional leaf injection, pays for the extra scalar choices.

## What Is Still Formal

The remaining bookkeeping is no longer the global injection; the composition lemma above reduces
that to local disjointness inside each node. The remaining formal point is the local rank
reduction:

```text
after charging child defects and complete extra stride-class dimensions, each child has only one
uncharged output line on the coordinates used by the parent comparison.
```

Complete extra stride-class dimensions do not need an injective leaf assignment beyond the
`q^floor((e+h)/|C|)` factor in the first-moment count, but their outside leaves are still included
in the chosen set `E`.

For virtual holes, outside leaves have two bookkeeping roles. One outside leaf can serve as the
replacement for a missing core position; only outside leaves left after these replacements are true
output-defect leaves. The alpha-2 theorem is:

```text
holes <= 2 * true output-defect leaves.
```
