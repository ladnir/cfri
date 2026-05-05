# RFC Restricted-Rank Induction

This note isolates the local algebra needed for the systematic rank proof. The goal is to replace
the current feature heuristic by a recursive certificate for the restricted parity matrix:

```text
M_d(S,Q) = P_d[rows {0,1}^d \ S, columns Q].
```

The systematic zero-set rank condition is:

```text
rank(I_S, P_Q) = |S| + rank(M_d(S,Q)).
```

So the proof only needs to certify full row rank for `M_d(S,Q)` whenever the zero set is above the
target distance threshold and outside the structural collision family.

## Local Node Algebra

Fix a node and one child parity column `g`. The two parent sibling columns are:

```text
L = [ (1-T) g_0 ]
    [   T   g_1 ]

R = [  -T    g_0 ]
    [ (T+1) g_1 ]
```

where `g_0` is `g` restricted to undeleted rows in the left child and `g_1` is `g` restricted to
undeleted rows in the right child.

The local coefficient matrix is:

```text
[ 1-T   -T  ]
[  T    T+1 ]
```

and its determinant is `1`. Therefore:

```text
span{L,R} = span{ [g_0; 0], [0; g_1] }.
```

This is the key induction fact. A selected sibling pair does not behave like two mysterious tensor
columns; it exactly splits into one independent child obligation on each side.

If only one sibling is selected, the column is a generic coupling:

```text
C = [ alpha(T) g_0 ]
    [ beta(T)  g_1 ]
```

with both coefficients nonzero as polynomials. A singleton therefore cannot be separated into two
independent child obligations for free. It either contributes one coupled dimension across the two
children or, if one child is fully deleted by `S`, degenerates to one ordinary child column on the
live side.

## Immediate Structural Loss

The first deterministic loss occurs when:

```text
1. both parity siblings for a child key are selected; and
2. one of the two systematic child row blocks is completely deleted.
```

Before quotienting by `S`, the sibling pair splits into two independent child columns. After one
child row block is deleted, only one of those two split columns remains. Thus the pair contributes
at most one restricted-rank dimension instead of two.

This is exactly the depth-2 obstruction:

```text
S = one full child row block
Q = one parity sibling pair with the same expansion copy and lower path
```

and gives restricted parity rank `1` on `2` remaining rows.

At deeper depths the same mechanism recurs inside child subtrees. A parent sibling pair can split
into child obligations, and those child obligations can then encounter lower deleted subtrees and
lower parity collisions.

## Candidate Induction State

For a node `U`, define:

```text
r_U = number of undeleted rows in U
q_U = number of selected parity columns whose projection reaches U
```

The desired certificate is a recursive lower bound:

```text
rank M_U >= min(r_U, q_U - rho_U)
```

where `rho_U` is a structural collision score computed from the tree.

A useful recursive shape for `rho_U` is:

```text
rho_U =
  immediate_dead_sibling_pairs(U)
  + rho_left(split child obligations)
  + rho_right(split child obligations)
  + coupling_penalty(U).
```

Here `immediate_dead_sibling_pairs(U)` counts sibling parity pairs whose one side is fully deleted.
The term `coupling_penalty(U)` is the hard part: it must account for singleton columns that couple
left and right child restrictions rather than splitting cleanly. The empirical data suggests that
this penalty is zero unless enough singleton projections collide with lower complete systematic
subtrees to create an actual determinant identity.

## Minor-Certificate Version

The cleanest proof target may be a recursive determinant certificate:

```text
rho_U = 0
  => there is an r_U x r_U minor of M_U with a unique leading monomial.
```

Under a term order that prioritizes the root challenge variables:

1. sibling pairs use the determinant-`1` split and reduce to child minors;
2. singleton columns choose either the left or right leading term, according to a matching of
   columns to undeleted rows;
3. uniqueness of the monomial follows if no recursive collision asks the same child minor to serve
   two incompatible obligations.

This would prove generic full rank over the indeterminate ring, not merely high probability over a
large field. Accidental finite-field failures then become exactly the shapes whose determinant
minor is nonzero but happens to evaluate to zero for a sampled challenge assignment.

## What Must Be Proved Next

The remaining proof obligation is now precise:

```text
For systematic expansion c=8, every zero set with |S|+|Q| >= k+2 has rho_U=0 at the root.
```

Equivalently, any structural restricted-rank loss must have:

```text
|S|+|Q| <= k+1.
```

The depth-3 exact rows and depth-4 samples are consistent with this. The next mechanical step is to
turn the node diagnostics into a recursive `rho` implementation and compare it against the exact
bad-shape tables, separating deterministic structural losses from accidental determinant zeros.

## Current Split-Profile Checks

The helper:

```text
scripts/rfc_split_profile.py
```

computes the node diagnostics above. The first checks are:

```text
depth 2 example:
  bad shape  0:1:4:18 has one root dead sibling pair
  good shape 0:2:4:18 has the same parity sibling pair but no full deleted child at that node

depth 3, s=6,zp=3 exact bad shapes:
  all 224 shapes have the same split profile
  dead sibling groups appear at levels 2 and 3

depth 3, s=5,zp=3 exact bad shapes:
  the dominant bad families have a root or lower dead sibling pair
  5-seed resampling leaves the four dominant dead-sibling profiles persistent
  the smaller no-dead-sibling profiles resample to full rank and are accidental zeros
```

The checked-in profile artifacts are:

```text
docs/rfc_split_profile_depth2_examples.csv
docs/rfc_split_profile_bad_s6_zp3_systematic_p65537_depth3_c8_resample5.summary.csv
docs/rfc_split_profile_bad_s5_zp3_systematic_p65537_depth3_c8_resample5.summary.csv
docs/rfc_split_profile_feature_bad_s11_zp5_systematic_p65537_depth4_c8.summary.csv
docs/rfc_split_profile_feature_bad_s14_zp3_systematic_p65537_depth4_c8.summary.csv
```

This is better than the first read: the persistent `s=5,zp=3` families still have dead-sibling
structure. The no-dead-sibling profiles are currently explained by accidental finite-field
determinant zeros, not by a new structural obstruction. The full proof still needs a coupling term,
but the evidence now says it may be used to prove singleton couplings are harmless unless they feed
into the same recursive dead-sibling collision.

## Recursive Certificate Count

The profiler now also computes a constructive certificate rank. The rule is:

```text
1. sibling parity pairs are split into left and right child obligations using the determinant-1
   local transform;
2. singleton parity columns are oriented to one live child as a leading-term obligation;
3. the certified rank is the best recursive split rank over those singleton orientations.
```

This is still a proof heuristic rather than a generic-rank theorem, but it is now the right
finite-depth model: it says exactly when the leading-monomial induction fails to cover every
undeleted row.

The exact row counter:

```text
scripts/rfc_certified_defect_row.py
```

matches the persistent structural rank failures in all exact rows currently checked:

```text
depth 2, s=2,zp=2:
  rank failures:        28
  certified defective:  28

depth 3, s=6,zp=2:
  persistent failures:  560
  certified defective:  560

depth 3, s=6,zp=3:
  persistent failures:  224
  certified defective:  224

depth 3, s=5,zp=3:
  sampled rank failures: 13027
  accidental failures:      35
  persistent failures:   12992
  certified defective:   12992
```

The checked summaries are:

```text
docs/rfc_certified_defect_row_s2_zp2_depth2_c8.csv
docs/rfc_certified_defect_row_s6_zp2_depth3_c8.csv
docs/rfc_certified_defect_row_s6_zp3_depth3_c8.csv
docs/rfc_certified_defect_row_s5_zp3_depth3_c8.csv
```

This is the strongest evidence so far that the structural family is exactly the failure set of the
recursive leading-monomial certificate, while the remaining sampled rank defects are ordinary
finite-field determinant zeros.
