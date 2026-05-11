# Original RFC MDS Certificate Direction

This note records the current status for the original, non-systematic RFC generator.

## Claim Shape

For depth `d` and expansion `c`, the original RFC generator has:

```text
k = 2^d
N = c k
```

The desired full-strength statement is generic MDS:

```text
every k selected columns have rank k.
```

Equivalently, the original code has distance:

```text
N - k + 1.
```

## Same Recursive Certificate

The recursive leading-monomial certificate from the systematic restricted-rank analysis applies
directly with no deleted systematic rows:

```text
S = empty
Q = selected original columns.
```

Sibling pairs still use the determinant-`1` split:

```text
span{L,R} = span{[g_0;0], [0;g_1]}.
```

Singleton columns are oriented to a live child as their leading-term obligation. Since no systematic
rows are deleted, the obstruction that hurt the systematic code is absent: a split sibling pair
never loses one child side by quotienting out a full systematic subtree.

Thus the same certificate is a natural route to proving original generic MDS.

## Evidence

The checker:

```text
scripts/rfc_distance_analysis/rfc_original_certified_defect.py
```

tests whether selected original columns are certified full rank. The current results for expansion
`8` are:

```text
depth 2 exact:
  checked k-subsets: 35960
  certificate defects: 0

depth 3 sample:
  checked k-subsets: 100000
  certificate defects: 0
```

The checked artifacts are:

```text
docs/rfc_distance_analysis/rfc_original_certified_defect_depth2_c8_exact.csv
docs/rfc_distance_analysis/rfc_original_certified_defect_depth3_c8_sample100k.csv
```

The orientation theorem also has a small executable trace:

```text
scripts/rfc_distance_analysis/rfc_original_orientation_trace.py
```

It is not needed for the proof, but it checks that the constructive split in the theorem aligns
with the implementation. It has been sampled with:

```text
depth 5, expansion 8, samples 20000: failures 0
depth 8, expansion 8, samples 2000:  failures 0
```

Depth `2` was also previously checked by direct numeric rank over a large prime, and every
`k`-subset was full rank. The certificate result is stronger in the proof direction because it
constructs a recursive minor rather than evaluating one random code instance.

## Current Read

Original RFC still looks like the full-strength/max-separable object. The remaining proof work is
combinatorial, and in the original case the combinatorics now appears essentially solved:

```text
Show that for S=empty and |Q|=2^d, the recursive leading-monomial certificate is always full.
```

This should be easier than the systematic case. The systematic obstruction requires deleted row
subtrees; with `S=empty`, every recursive child remains live, and singleton orientation should be
provable by a Hall-style tree matching argument.

## Inductive Certificate Theorem

The original MDS certificate theorem is:

```text
For every depth d and every set Q of exactly 2^d original RFC columns,
certificate_full(empty, Q, d) = true.
```

This is stronger than the sampled checks above because it proves that the recursive
leading-monomial minor construction never gets stuck.

### Proof

Induct on `d`.

The base case `d=0` is immediate: any one selected original column is the nonzero depth-0 column.

For the inductive step, split the selected depth-`d` columns by root sibling groups. A group is
indexed by:

```text
(copy, lower_path)
```

and can contain either:

```text
1. both root siblings, or
2. exactly one singleton sibling.
```

Let:

```text
p = number of paired sibling groups
s = number of singleton groups.
```

Since the selected set has size `2^d`:

```text
2p + s = 2^d.
```

Each paired sibling group is transformed by the determinant-`1` local matrix into one left child
obligation and one right child obligation. Therefore paired groups force:

```text
p left obligations
p right obligations.
```

Both child certificates need exactly `2^(d-1)` obligations. The number of singleton groups that
must be oriented left is:

```text
2^(d-1) - p.
```

The same number must be oriented right, and this is feasible because:

```text
s = 2^d - 2p = 2(2^(d-1)-p).
```

Choose any `2^(d-1)-p` singleton groups and orient them left; orient the rest right. The projected
child columns are distinct within each child:

```text
same side + same (copy, lower_path) would be the same original column,
opposite sides + same (copy, lower_path) would have been a paired group.
```

Thus each child receives exactly `2^(d-1)` distinct original child columns. By the induction
hypothesis, both child sets have full recursive certificates. Combining the two child certificates
with the determinant-`1` split for paired groups and the chosen leading terms for singletons gives a
full parent certificate.

Therefore `certificate_full(empty,Q,d)` holds for every `|Q|=2^d`. QED.

## Consequence

Combining the inductive certificate theorem with the soundness theorem in
`docs/rfc_distance_analysis/rfc_restricted_rank_induction.md` gives:

```text
Every k-column submatrix of the original RFC generator has generic rank k.
```

So the original non-systematic RFC generator is generically MDS:

```text
d_min = N - k + 1.
```

The only remaining writeup work is to merge the certificate soundness theorem and this original
orientation theorem into a polished final proof. The algebra and combinatorics are now aligned.

The practical distance/query impact of this upgrade is summarized in:

```text
docs/rfc_distance_analysis/rfc_original_distance_upgrade.md
```
