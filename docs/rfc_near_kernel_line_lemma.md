# RFC Near-Core Kernel-Line Lemma

This note isolates the second structural fact needed by the near-stability certificate.

## Corrected Lemma Target

Let `A_d` be one RFC parity transform, `k=2^d`, and let `(R,C)` be a matched block/stride pair:

```text
|R| = m
|C| = k/m
```

For an extra output set:

```text
E subset [k] \ C
```

the first attempted target was:

```text
dim { x in F^R : supp(A_d x) subset C union E } = 1.
```

This is false once `E` contains a complete additional stride class. The correct target is a
dimension bound, not line uniqueness:

```text
dim { x in F^R : supp(A_d x) subset C union E }
  <= 1 + floor(|E| / |C|).
```

Equivalently, a new message direction costs at least one full extra stride class, i.e. `|C|=k/m`
extra output leaves.

## Why This Is Needed

The near-stability count uses:

```text
k * binom(k-k/m, e)
```

support/output pairs. If a support pair has kernel dimension `r`, the first moment must also pay a
projective-vector factor of about:

```text
q^(r-1).
```

Under the corrected target this is bounded by:

```text
q^floor(e/(k/m)).
```

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
depth 4, m=4, e=4 generated matched-core pairs
```

The generated-family artifacts are:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_e2_model.csv
docs/rfc_near_pair_kernel_dim_depth4_m4_e3_model.csv
docs/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
docs/rfc_near_core_stride_dimension_class_depth4_m4_e2.csv
docs/rfc_near_core_stride_dimension_class_depth4_m4_e3.csv
docs/rfc_near_core_stride_dimension_class_depth4_m4_e4.csv
```

with:

```text
e=2: 1056 pairs, all kernel_dim = 1
e=3: 3520 pairs, all kernel_dim = 1
e=4: 7920 pairs, 7872 have kernel_dim = 1 and 48 have kernel_dim = 2
```

The `e=4` dimension-`2` cases are exactly the warning sign: for depth `4`, `m=4`, the matched core
has size `|C|=4`, and dimension first grows when the extras can include a full second stride class.

The stride-class classifier checks the sharper identity:

```text
kernel_dim - 1 = number of complete extra stride classes
```

for the generated `m=4`, `e=2,3,4` models. The e=4 classification is:

```text
complete extra strides 0, kernel_dim 1: 7872 pairs
complete extra strides 1, kernel_dim 2:   48 pairs
```

and every checked pair has:

```text
complete_extra_strides - (kernel_dim - 1) = 0.
```

## Proof Shape

The exact matched-kernel proof descends to the live block and then glues a single output coordinate
through a full binary subtree. Extra allowed outputs remove some zero constraints. The corrected
lemma says the kernel dimension can only grow when the removed constraints contain a complete
additional global stride class.

A direct proof should use the same recursion as:

```text
docs/rfc_matched_kernel_induction.md
```

but with a punctured set of zero constraints.

The intended invariant is:

```text
At each glue node, each independent new output direction requires all copies of one additional
stride residue to be allowed.
```

Thus every new degree of freedom consumes at least `|C|` extra output leaves. This is the rank
analogue of the defect-charge statement: isolated extras do not create a new line; complete
stride-class extras can.

The stronger conjectural form is:

```text
dim { x in F^R : supp(A_d x) subset C union E }
  = number of complete stride classes contained in C union E.
```

The certificate only needs the upper bound, but this equality is a useful proof target because it
matches the recursive structure: a matched row block is naturally diagonalized by output stride
classes.

## Relationship To Near Stability

The near-stability theorem has two separate jobs:

```text
1. core containment:
   wt(x)=m, wt(Ax)<=k/m+e implies supp(Ax) contains some matched core C;

2. kernel dimension:
   once C and E=supp(Ax)\C are fixed, the admissible x-space has dimension at most
   1 + floor(|E|/|C|).
```

The defect-charge injection addresses the first job. This note isolates the second job so the final
certificate can cite it independently.
