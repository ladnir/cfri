# Original RFC MDS Investigation

This note records the current status for the original, non-systematic RFC generator.

## Current Status

The non-systematic RFC still has strong evidence for **per-subset generic full rank**: for a
fixed selected column set `Q` of size `k`, the recursive orientation certificate appears to
construct a nonzero determinant polynomial.

That is not yet an MDS theorem for a sampled code. A sampled code is MDS only if:

```text
every k-column subset is full rank for the same sampled diagonal challenges.
```

The BaseFold paper reports concrete RFC distance bounds below the Singleton/MDS distance. In the
extracted text, the theorem we checked is a high-probability lower bound on distance, not an
explicit impossibility theorem for MDS. Still, the paper's table and "tight bounds" language are a
warning that the previous "original RFC is generically MDS" wording was too strong.

The working distinction is:

```text
fixed Q:        determinant is plausibly nonzero as a polynomial
all Q at once:  not proved, and likely false at large parameters without extra structure
```

At target sizes, the number of `k`-subsets is enormous. Even if every fixed-subset determinant is
nonzero, accidental roots after sampling the RFC challenges can create deficient subsets unless the
determinants have a much stronger collision-free structure than a generic Schwartz-Zippel bound.

## Claim Shape

For depth `d` and expansion `c`, the original RFC generator has:

```text
k = 2^d
N = c k
```

The strongest possible statement would be sampled MDS:

```text
every k selected columns have rank k.
```

Equivalently, the original code has distance:

```text
N - k + 1.
```

## Same Recursive Certificate

The recursive leading-monomial certificate from the systematic restricted-rank analysis may apply
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

Thus the same certificate is a natural route to proving fixed-subset generic full rank. It is not,
by itself, enough to prove sampled MDS.

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

Additional exact numeric checks for the current determinant-`1` fold form:

```text
depth 2, expansion 8, GF(65537): all 35,960 k-subsets full rank
depth 3, expansion 2, GF(65537): all 12,870 k-subsets full rank
```

For the paper's literal `T'=-T` form, a depth-2, expansion-8 check over `GF(65537)` found an
actual deficient `k`-subset for one seed. Resampling that fixed shape across 1000 seeds produced
one failure, so this looks like an accidental finite-field determinant root rather than a structural
bad shape.

These checks support "no visible structural obstruction" for small cases. They do not establish MDS
with high probability at large parameters.

## Current Read

Original RFC still looks structurally much stronger than the systematic all-level code. The
remaining proof work is not solved by the orientation combinatorics alone:

```text
1. prove the fixed-Q determinant polynomial is nonzero, or find a structural counterexample;
2. quantify the probability that any k-subset determinant vanishes after sampling all challenges;
3. reconcile the result with the BaseFold paper's reported RFC distance bounds.
```

The systematic obstruction requires deleted row subtrees; with `S=empty`, every recursive child
remains live. That removes the known systematic collapse family, but it does not remove the
finite-field all-subsets problem.

## Inductive Certificate Theorem

The fixed-subset certificate theorem candidate is:

```text
For every depth d and every set Q of exactly 2^d original RFC columns,
certificate_full(empty, Q, d) = true.
```

This is stronger than the sampled checks above because it would prove that the recursive
leading-monomial minor construction never gets stuck for a fixed `Q`.

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

Therefore `certificate_full(empty,Q,d)` holds for every `|Q|=2^d`.

This proves only the combinatorial orientation part. A complete determinant proof still has to show
that the chosen leading term cannot cancel with other recursive terms, and a sampled-MDS proof would
also need a uniform all-subsets argument.

## Consequence

If the determinant soundness theorem can be completed, it gives the fixed-subset statement:

```text
For every fixed k-column set Q, the corresponding determinant polynomial is nonzero.
```

This is not yet the sampled-MDS statement:

```text
Pr[the sampled generator is MDS] = 1 - negligible.
```

The next proof target is a first-moment or structural-collision bound for deficient `k`-subsets.
That is the point where the BaseFold paper's lower-bound distance analysis has to be compared
directly against this MDS direction.

The practical distance/query impact of this upgrade is summarized in:

```text
docs/rfc_distance_analysis/rfc_original_distance_upgrade.md
```
