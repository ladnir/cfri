# RFC Global Rank Budget Pivot

This note records the response to the audit concern that local child-rank payments may exceed the
root-scale dimension factor in the first-moment count.

## Problem With Local Rank Spending

The uncharged-skeleton reduction previously described a local payment:

```text
child rank at node v
  <= floor((e_v+h_v)/|C_v|).
```

If these local floors are summed over many nodes, they are not obviously bounded by the root-scale
factor used in the certificate:

```text
floor((e_root+h_root)/|C_root|).
```

A deliberately conservative stress check confirms that replacing the aggregation proof by a large
per-defect overhead is not viable. At depth 11, `c=8`, `alpha=2`, and `B=192`, the union bound is:

```text
total_log2_union = 10547.46914530.
```

Artifact:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead192_holes_le2extra.csv
```

So the proof must not pay a fresh `q`-dimension factor independently at each node.

## Pivot

Rank should be paid globally, not locally.

For a fixed final virtual support:

```text
supp(Ax) = (C_root \ H_root) union E_root,
|E_root| = e+h,
```

the near-kernel line lemma gives one global admissible space:

```text
dim <= 1 + floor((e+h)/|C_root|).
```

The first-moment count already pays:

```text
q^floor((e+h)/|C_root|).
```

Therefore extra cancellation equations throughout the recursion should be charged against this
single global projective dimension budget, not against a sum of child-local budgets.

## Revised Exchange Target

Fix the final virtual support and its admissible vector space `V`. Let:

```text
r = dim V,
s = r-1.
```

Let `J` be a set of sibling cancellations anywhere in the recursive tree that are not part of the
one exact-skeleton continuation per active two-child node. The desired algebraic statement is:

```text
the cancellation equations indexed by J have generic rank at least |J|-s
```

after quotienting by the `s` projective degrees of freedom in `V`.

Equivalently, the first-moment contribution for `t=|J|` excess cancellations carries:

```text
q^s * q^(-max(0,t-s)).
```

or, if one global relative scalar is reserved in the exact no-early-gluing style:

```text
q^s * q^(-max(0,t-s-1)).
```

The exact convention depends on whether the base skeleton scalar is included in `V` or quotient out
projectively. The important point is that `s` is the global near-kernel excess dimension, not a sum
of local child dimensions.

## Structural Consequence

The alpha-2 local size inequality should be applied only after the support pattern has been split
into:

```text
1. cancellations absorbed by the global admissible dimension budget;
2. cancellations paid by no-early-gluing probability loss;
3. at most one residual sibling cancellation visible at each local alpha-2 node.
```

This changes the proof obligation. The local statement is no longer:

```text
each node has enough child rank budget to reduce to one scalar.
```

The stronger and better-aligned statement is:

```text
for the final support pattern, all extra cancellation equations in the tree are paid by the global
near-kernel dimension factor and the no-early-gluing loss.
```

## Remaining Work

The open algebraic lemma is now global:

```text
For every fixed final virtual support and every selected family of excess sibling cancellations,
the RFC cancellation equations have generic rank at least the number of excess cancellations minus
the global near-kernel excess dimension.
```

This is closer to what the first-moment count actually measures, and it avoids the child-scale
rank-payment aggregation problem.
