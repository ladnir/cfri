# RFC Complete-Stride Target Spec

Scope: original non-systematic RFC only.

This note specifies the exact known depth-4, `m=4`, `extra=4` dimension-growth family that the
implementation worker should target first.  The goal is to avoid enumerating all Gaussian subflags:
this family has an explicit row/support construction.

## Existing Artifacts

The relevant saved artifacts are:

```text
docs/rfc_distance_analysis/rfc_uncertainty_near_model_pairs_depth4_m4_e4.csv
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
docs/rfc_distance_analysis/rfc_near_core_stride_dimension_class_depth4_m4_e4.csv
docs/rfc_distance_analysis/rfc_core_containment_depth4_m4_e4_model.csv
docs/rfc_distance_analysis/rfc_near_extremizer_count_depth4_m4.csv
```

They record:

```text
depth = 4
k = 16
live rows m = 4
exact output support = k/m = 4
extra outputs = 4
near output support = 8
generated marked pairs = 7920
```

The kernel-dimension summary is:

```text
kernel_dim = 1: 7872 marked rows
kernel_dim = 2:   48 marked rows
```

The stride-class summary is:

```text
complete_extra_strides = 0, kernel_dim = 1: 7872 marked rows
complete_extra_strides = 1, kernel_dim = 2:   48 marked rows
```

The core-containment summary is:

```text
contained_core_count = 1: 7872 marked rows
contained_core_count = 2:   48 marked rows
```

Representative dimension-growth row:

```text
rows    = 0:1:2:3
outputs = 0:1:4:5:8:9:12:13
```

This output set is the union of two complete stride classes:

```text
C_0 = {0,4,8,12}
C_1 = {1,5,9,13}
outputs = C_0 union C_1.
```

I checked one representative directly in the marked CSV:

```text
rows=0:1:2:3, outputs=0:1:4:5:8:9:12:13
```

appears twice, once for each contained matched core.  Thus the 48 saved dimension-2 rows correspond
to 24 unmarked `(rows, outputs)` support pairs, each with two ordered kernel/quotient flags.

## Construction

At depth 4, index message rows and one-copy outputs by `0..15`.

Row blocks:

```text
B_b = {4b, 4b+1, 4b+2, 4b+3},  b in {0,1,2,3}.
```

Output stride classes:

```text
C_j = {j, j+4, j+8, j+12},  j in {0,1,2,3}.
```

The exact extremizer pairs are:

```text
(B_b, C_j),  b,j in {0,1,2,3}.
```

Each has one-dimensional kernel:

```text
dim {x supported on B_b : supp(Ax) subset C_j} = 1.
```

The dimension-growth family is:

```text
(B_b, C_i union C_j),  b in {0,1,2,3},  0 <= i < j <= 3.
```

Expected dimension:

```text
dim {x supported on B_b : supp(Ax) subset C_i union C_j} = 2.
```

Count:

```text
unmarked support pairs = 4 * binom(4,2) = 24
ordered flags          = 24 * 2 = 48
```

The ordered flags are:

```text
L = line for C_i,  V = span(lines for C_i,C_j), visible quotient V/L on C_j
L = line for C_j,  V = span(lines for C_i,C_j), visible quotient V/L on C_i
```

This is the targeted generator.  It should not enumerate arbitrary `2`-planes in the message
space; it should build `V` from the two exact stride lines.

## Expected Flag Variables

Use the following as the direct-support flag target.  If embedding into a top-level recurrence split,
derive split-specific `p` from the chosen coordinate pairing; for this targeted generator,
`z_V` and `z_L` are the source of truth.

For one unmarked pair `(B_b, C_i union C_j)` and ordered kernel line `L=C_i`, visible quotient
on `A=C_j`:

```text
h      = 4
t      = 2
z      = z_V = 8
p      = 8        # common-zero set outside C_i union C_j in direct-support notation
s      = 4        # singleton/visible test set C_j for this ordered flag
a      = 4        # visible support size |A| = |C_j|
tau    = 1
kappa  = 1        # dim K = t - tau
r1     = 2        # expected dim pi(W) in the direct support target
r0     = 1        # expected dim pi(K)
z_V    = 8        # V vanishes outside C_i union C_j
z_L    = 12       # L vanishes outside C_i
delta  = 1        # rank increment of one visible stride class in V/L
comp   = 1        # one rank-1 parallel/component class for the visible stride
g      = NA/0     # tau=1 target; no tau=2 generic endpoint is being used
```

For the unordered full support `C_i union C_j`, useful aggregate local data are:

```text
support size = 8
rank/delta   = 2
components   = 2
generic g    = 0 in the uniform 2*delta-size approximation
```

The worker should emit both the ordered-flag record and the aggregate support record, because the
upper-bound recurrence needs ordered flags while the de-duplication logic needs unmarked supports.

## Targeted Generation Procedure

For each `b in {0,1,2,3}`:

```text
R = B_b
```

For each unordered pair `{i,j}` of output stride indices:

```text
Omega = C_i union C_j
Q     = [0..15] \ Omega
```

Compute the restricted kernel:

```text
K_Omega = {x supported on R : output outside Omega is zero}.
```

Expected:

```text
dim K_Omega = 2.
```

Then split into the two exact stride lines:

```text
ell_i = {x supported on R : output outside C_i is zero}
ell_j = {x supported on R : output outside C_j is zero}
K_Omega = ell_i direct_sum ell_j.
```

Emit two ordered flags:

```text
ell_i <= K_Omega, visible quotient on C_j
ell_j <= K_Omega, visible quotient on C_i
```

Expected total:

```text
24 unmarked support pairs
48 ordered flags
```

## Counter-Signals

The implementation worker should report a counter-signal if any of the following occurs.

1. More unmarked supports than expected:

```text
# { (R, Omega) : dim K_Omega >= 2 } > 24
```

This would mean dimension growth is not limited to complete extra stride classes.

2. More ordered flags than expected:

```text
# ordered one-kernel flags from the 24 supports > 48
```

This would mean the exact flag multiplicity is larger than the simple two-stride decomposition.

3. Non-stride dimension growth:

```text
dim K_Omega >= 2
```

for an `Omega` of size `8` that is not a union of two complete stride classes.

4. Non-direct-sum behavior:

```text
K_Omega != ell_i direct_sum ell_j
```

for `Omega = C_i union C_j`.

5. Multi-copy/generic-flag excess, if a second independent copy is included later:

```text
observed_logq_common_flag_dimension - generic_flag_dimension > 0.
```

At production `q=2^128`, one extra reusable `q`-dimension is a 128-bit effect, much larger than
the current exact-support `e=71` slack.

## How To Use The Existing Artifacts

Use the artifacts as sanity checks, not as the generator.

The summary:

```text
docs/rfc_distance_analysis/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
```

should match:

```text
kernel_dim=2 count 48
```

under the marked-core convention.

The summary:

```text
docs/rfc_distance_analysis/rfc_near_core_stride_dimension_class_depth4_m4_e4.csv
```

should match:

```text
complete_extra_strides=1, kernel_dim=2 count 48
```

The summary:

```text
docs/rfc_distance_analysis/rfc_core_containment_depth4_m4_e4_model.csv
```

should match:

```text
contained_core_count=2 count 48
```

The large CSV:

```text
docs/rfc_distance_analysis/rfc_uncertainty_near_model_pairs_depth4_m4_e4.csv
```

contains marked rows.  Do not use its raw row count as the exact flag count without de-duplicating
by `(rows, outputs)`.  The target generator can avoid this by directly constructing the 24
unmarked supports and then emitting the 48 ordered flags.

## Expected Manager Outcome

If the targeted generator returns exactly:

```text
24 unmarked supports
48 ordered flags
no non-stride dimension growth
no extra flag multiplicity
```

then this family is a controlled local exception, not a counterexample.  The proof agent still has
to charge complete extra stride classes, but the implementation worker can move next to longer
flags or two-copy intersections.

If any counter-signal above appears, the lower-bound lane has a real nested-kernel obstruction to
feed back into the recurrence.
