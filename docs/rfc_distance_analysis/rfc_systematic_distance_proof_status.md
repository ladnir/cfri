# RFC Systematic Distance Proof Status

This is the current proof snapshot for the all-level systematic RFC distance certificate.

## Target

Parameters:

```text
c = 8
c_p = 7
d = 11
k = 2048
N = 16384
q = 2^128
```

The collapse family gives the upper bound:

```text
d_sys <= 12384
delta_sys <= 12384 / 16384 = 0.755859375.
```

So the certificate target is:

```text
d_sys >= 12384
```

up to the usual random-challenge failure probability.

## Current Certificate Shape

The proof target is no longer literal actual-core containment. The working theorem is a
defect-coupled virtual-core statement:

```text
wt(x) = m,
wt(Ax) = k/m + e

=> supp(Ax) = (C \ H) union E
   where C is a matched stride core of size k/m,
         H subset C,
         E subset [k]\C,
         |E| = e + |H|,
         |H| <= 2e.
```

This is the right replacement because unbalanced row splits can create holes in the selected
parent residue, even when they are still accompanied by proportional outside support.

## Count

The depth-11 systematic count with virtual holes coupled to true output defect gives:

```text
alpha=2: Pr[d_sys < 12384] <= 2^-99.60689084.
```

The same audited count remains negative through:

```text
alpha=5: log2 failure bound = -21.53925283
```

and breaks at:

```text
alpha=6: log2 failure bound = 25.89359792.
```

So the proof target is `alpha=2`, with slack up to but not including `alpha=6`.

The count includes:

```text
k * sum_{h <= alpha e} binom(k/m, h) binom(k-k/m, e+h) * 2^(B e)
```

with `B=64`, and the near-kernel dimension factor:

```text
q^floor((e+h)/(k/m)).
```

The key artifacts are:

```text
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le2extra.csv
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le5extra.csv
docs/rfc_distance_analysis/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le6extra.csv
```

## What Is Reduced

The one-child case is reduced:

```text
child virtual core lifts to both parent siblings;
holes and defect both double.
```

The balanced two-child case is reduced:

```text
after overlap and paid cancellation charges, no-early-gluing leaves an actual parent core.
```

The unbalanced two-child case is reduced to a local alpha-2 theorem:

```text
new parent virtual holes <= 2 * new true output-defect leaves.
```

The deterministic size part of this theorem is written. At an unbalanced node:

```text
M = a+b, a != b,
L = K/M,
R = L/2,
C = child projection of selected parent residue,
A = |C \ (U union V)|,
O = |(U union V) \ C|.
```

One-copy uncertainty gives:

```text
O >= R + A.
```

After globally paid excess cancellations are removed, the residual skeleton seen by this local
size argument should have:

```text
c_C + c_O <= 1.
```

Then:

```text
h_local = 2A + c_C,
e_local >= 2O - c_O - h_local
        >= 2R - (c_C+c_O)
        >= 2R - 1,
```

and for `R>=2`:

```text
h_local <= 2 e_local.
```

The integer small cases are covered by:

```text
scripts/rfc_distance_analysis/rfc_check_alpha2_local_inequality.py --max-r 64
```

which reports:

```text
checked_R=1..64 failures=0.
```

## Global Composition

The global bookkeeping is no longer the main gap. Fix the final virtual core `C_root`; every
outside leaf has a unique highest first-divergence node.

For each unbalanced node `v`, split the outside leaves first diverging at `v` into:

```text
R_v = replacement leaves, one per new virtual hole;
D_v = true defect leaves.
```

The local theorem gives:

```text
|R_v| = h_v,
h_v <= 2 |D_v|.
```

The first-divergence sets are disjoint across nodes, so:

```text
|H_root| = sum_v h_v
        <= 2 sum_v |D_v|
        <= 2e.
```

Balanced-node charges create no holes and only add true defect leaves. Complete extra stride-class
dimensions are rank payments, not additional hole payments. The current count uses the root-scale
factor:

```text
q^floor((e+h)/|C_root|)
```

An earlier local-rank formulation would have needed to aggregate child-scale rank payments into
this root-scale factor. The current better pivot avoids local rank spending: fix the final virtual
support first, use the global near-kernel dimension bound once, and charge all excess cancellation
equations against that global projective dimension plus no-early-gluing probability loss. This is
tracked in:

```text
docs/rfc_distance_analysis/rfc_global_rank_budget_pivot.md
docs/rfc_distance_analysis/rfc_global_rank_cancellation_probe.md
docs/rfc_distance_analysis/rfc_rank_active_cancellation_independence.md
```

## Remaining Algebraic Lemma

One remaining serious proof obligation is the global rank-cancellation exchange:

```text
after paying the global admissible-support dimension and no-early-gluing loss, at most one sibling
cancellation remains visible at each alpha-2 local node.
```

Global formulation:

```text
V = admissible vector space for the fixed final virtual support,
s = dim(V)-1 <= floor((e+h)/|C_root|).
```

If `t` excess sibling cancellations occur across the recursive tree, then generically:

```text
t cancellations impose t equations,
the global projective dimension absorbs at most s equations,
any remaining equations are paid by no-early-gluing probability loss.
```

So the intended first-moment factor is:

```text
q^s * q^(-max(0,t-s-1)).
```

The optional `-1` convention reserves one relative scalar in the exact no-early-gluing style. The
formal version must fix whether that scalar is already part of `V`.

After global rank payments and no-early-gluing probability-loss payments are removed from the
structural case under consideration, at most one cancellation remains visible to each alpha-2 local
size lemma.

Equivalently, for every fixed final virtual support and selected excess-cancellation family `J`,
the actual RFC cancellation matrix should have generic rank at least:

```text
|J| - floor((e+h)/|C_root|)
```

up to the same scalar convention. This must be proved for the actual fold equations with `T`
uniform nonzero.

This is tracked in:

```text
docs/rfc_distance_analysis/rfc_rank_cancellation_exchange.md
```

## Audit Findings

A fresh audit identified five issues that should be treated as active proof obligations, not
cosmetic TODOs:

```text
P0. Rank-cancellation is not deterministic by itself.
    Paying s rank parameters leaves max(0,t-s) cancellations; only one is absorbed by the
    relative scalar. The remaining max(0,t-s-1) must be paid by no-early-gluing probability loss.

P0. Local child-rank spending may exceed the root-scale dimension factor.
    The notes use child-scale floors floor((e_child+h_child)/|C_child|), while the current
    certificate count uses only floor((e_root+h_root)/|C_root|).

P1. There is a possible double-spend between outside mass charged away for skeleton reduction and
    outside mass later used as replacement/true defect leaves in the alpha-2 inequality.

P1. First-divergence composition still needs a local-to-final survival proof: local non-core
    parent outputs must correspond to final outside leaves with the same first-divergence node.

P2. The charged-label overhead is B*e, not B*(e+h). This is fine only if hole/replacement labels
    are already determined by support choices or are injectively charged to the true-defect budget.
```

These findings do not kill the direction, but they mean the proof is not "one lemma away" unless
the global rank-budget and no-double-spend interfaces are included in that lemma package.

## Current Honesty Level

We have a plausible full proof architecture and the counting already supports the desired distance
bound with real slack. We do not yet have a complete proof. The remaining work is:

```text
1. rank-cancellation exchange over the actual RFC fold equations;
2. global rank-budget cancellation accounting, replacing local child-rank spending;
3. disjoint resource accounting between skeleton-reduction charges, replacement leaves, and true
   defect leaves;
4. local-to-final survival for first-divergence charged leaves.
```

If these obligations are proved, the proof chain closes as:

```text
rank-cancellation exchange
  + global rank-budget cancellation accounting
  + disjoint resource accounting
  + local-to-final first-divergence survival
  => residual c_C+c_O<=1
  => local alpha-2 hole/defect coupling
  => first-divergence global |H|<=2e
  => defect-coupled virtual-core count
  => Pr[d_sys < 12384] <= 2^-99.60689084.
```

The most important audit questions are:

```text
1. Is the local inequality using O >= R+A correctly in every unbalanced residue?
2. Does first-divergence really prevent double-spending between replacement leaves and true defect
   leaves?
3. Is the rank-cancellation exchange stated with the correct quotient and dimension payment?
4. Does the first-moment count pay for every rank parameter and every charged-tree label exactly
   once, without undercounting virtual holes?
```
