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
scripts/rfc_original_certified_defect.py
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

depth 4 sample:
  checked k-subsets: 20000
  certificate defects: 0

depth 5 sample:
  checked k-subsets: 20000
  certificate defects: 0
```

The checked artifacts are:

```text
docs/rfc_original_certified_defect_depth2_c8_exact.csv
docs/rfc_original_certified_defect_depth3_c8_sample100k.csv
docs/rfc_original_certified_defect_depth4_c8_sample20k_fullonly.csv
docs/rfc_original_certified_defect_depth5_c8_sample20k_fullonly.csv
```

Depth `2` was also previously checked by direct numeric rank over a large prime, and every
`k`-subset was full rank. The certificate result is stronger in the proof direction because it
constructs a recursive minor rather than evaluating one random code instance.

## Current Read

Original RFC still looks like the full-strength/max-separable object. The remaining proof work is
combinatorial:

```text
Show that for S=empty and |Q|=2^d, the recursive leading-monomial certificate is always full.
```

This should be easier than the systematic case. The systematic obstruction requires deleted row
subtrees; with `S=empty`, every recursive child remains live, and singleton orientation should be
provable by a Hall-style tree matching argument.
