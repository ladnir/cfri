# RFC Near-Core Kernel-Line Lemma

This note isolates the second structural fact needed by the near-stability certificate.

## Lemma Target

Let `A_d` be one RFC parity transform, `k=2^d`, and let `(R,C)` be a matched block/stride pair:

```text
|R| = m
|C| = k/m
```

For an extra output set:

```text
E subset [k] \ C
```

the target lemma is:

```text
dim { x in F^R : supp(A_d x) subset C union E } = 1.
```

Equivalently, allowing extra output positions around a matched core does not create new message
directions beyond the exact matched kernel line.

## Why This Is Needed

The near-stability count uses:

```text
k * binom(k-k/m, e)
```

support/output pairs. This is the right first-moment count only if each pair contributes one
candidate line. If the kernel dimension grew with `e`, the union bound would need an additional
factor depending on that dimension.

## Evidence

The checked exact and near families all have kernel dimension `1` on matched-core pairs:

```text
depth 3, m=2 exact matched cores
depth 3, m=4 exact matched cores
depth 4, m=2 exact matched cores
depth 4, m=4 exact matched cores
depth 4, m=4, e=1 exhaustive near pairs
depth 4, m=2, e=1 exhaustive near pairs
depth 4, m=2, e=2 exhaustive near pairs
depth 4, m=4, e=2 generated matched-core pairs
depth 4, m=4, e=3 generated matched-core pairs
```

The generated-family artifacts are:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_e2_model.csv
docs/rfc_near_pair_kernel_dim_depth4_m4_e3_model.csv
```

with:

```text
e=2: 1056 pairs, all kernel_dim = 1
e=3: 3520 pairs, all kernel_dim = 1
```

## Proof Shape

The exact matched-kernel proof descends to the live block and then glues a single output coordinate
through a full binary subtree. Extra allowed outputs remove some zero constraints. The lemma says
that, despite removing those constraints, the remaining forbidden outputs still force the same
recursive glue ratios.

A direct proof should use the same recursion as:

```text
docs/rfc_matched_kernel_induction.md
```

but with a punctured set of zero constraints.

The intended invariant is:

```text
At each glue node, unless both child kernel vectors are on the exact child lines, at least one
forbidden sibling outside C union E becomes nonzero.
```

Thus every attempted new degree of freedom must consume an extra output leaf. Since `E` is fixed in
advance, the only way to obtain a positive-dimensional family is to satisfy a set of cancellation
identities indexed by the extra leaves. Those identities use independent fresh challenges and should
have no generic solution except the exact matched line.

## Relationship To Near Stability

The near-stability theorem has two separate jobs:

```text
1. core containment:
   wt(x)=m, wt(Ax)<=k/m+e implies supp(Ax) contains some matched core C;

2. line uniqueness:
   once C and E=supp(Ax)\C are fixed, the admissible x-space is still one-dimensional.
```

The defect-charge injection addresses the first job. This note isolates the second job so the final
certificate can cite it independently.
