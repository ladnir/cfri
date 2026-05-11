# RFC Exact Stability Proof Skeleton

This note turns the exact-extremizer evidence into a proof skeleton.

## Correct One-Step Inequality

Let one depth-`d` message split as:

```text
x = (x_0, x_1)
a = wt(x_0)
b = wt(x_1)
m = a+b
```

and let the two child outputs be:

```text
u = A_{d-1} x_0
v = A_{d-1} x_1.
```

Write:

```text
p = wt(u)
q = wt(v).
```

At each child output coordinate, the parent applies a local `2 x 2` MDS matrix. Therefore:

```text
one nonzero input  -> two nonzero outputs
two nonzero inputs -> at least one nonzero output.
```

If the child output supports overlap in `r` positions, then:

```text
wt(A_d x) >= 2(p-r) + 2(q-r) + r
           = 2p + 2q - 3r
           >= max(p,q).
```

If exactly one child is nonzero, the parent output has weight exactly twice the nonzero child
output weight, because every active local input has one nonzero coordinate.

By induction:

```text
p >= 2^(d-1)/a      when a > 0
q >= 2^(d-1)/b      when b > 0.
```

If both children are nonzero:

```text
wt(A_d x) >= max(p,q)
           >= max(2^(d-1)/a, 2^(d-1)/b)
           = 2^(d-1)/min(a,b)
           >= 2^d/(a+b).
```

If exactly one child is nonzero, say `b=0`, then:

```text
wt(A_d x) = 2p
           >= 2 * 2^(d-1)/a
           = 2^d/(a+b).
```

This proves:

```text
wt(x) * wt(A_d x) >= 2^d.
```

## Exact Equality Conditions

Suppose:

```text
wt(x) = m
wt(A_d x) = 2^d/m.
```

Then the induction is tight at every active step. There are two equality branches.

**One-child branch.** If exactly one child is nonzero, then:

```text
1. the nonzero child is an exact extremizer recursively;
2. the parent output is both sibling lifts of the child output support.
```

This is the branch that lets a small live block remain inside one half of the tree while the output
stride doubles.

**Two-child branch.** If both children are nonzero, then:

```text
1. a = b = m/2;
2. p = q = 2^(d-1)/a;
3. supp(u) = supp(v);
4. each overlapping local coordinate contributes exactly one parent output;
5. x_0 and x_1 are exact extremizers recursively.
```

This is the branch where two sibling live blocks glue together and one parent side cancels at every
active child output coordinate.

In particular, `m` is a power of two. Equality cannot appear for an odd split or for a child that is
not itself extremal.

## Matched Block/Stride Theorem

The exact scans suggest the following stronger theorem.

**Theorem target.** If equality holds at depth `d`, then for `m=2^t`:

```text
supp(x) = { s m, s m + 1, ..., s m + m - 1 }
supp(A_d x) = { r, r + m, r + 2m, ..., r + (2^d/m - 1)m }.
```

There are exactly:

```text
2^d
```

matched support pairs for each `m`.

The two equality branches explain the block/stride form. The one-child branch keeps the input block
inside one child and doubles the output stride. The two-child branch glues two sibling input blocks
and chooses one output residue at the current bit.

The remaining missing lemma is cancellation consistency. It has two parts.

First, **no early gluing**:

```text
If the two-child equality branch occurs at a node where the common child output support has size
greater than one, then the required cancellations depend on two or more independent fresh parent
challenges. The two child kernel lines have only one relative scalar, so the simultaneous
cancellations cannot hold generically.
```

Thus the two-child branch can only occur after the one-child branch has descended to the node whose
size equals the live support size. At that point the common child output support has size one.

Second, **local glue consistency**:

```text
At the full live node, two child extremizer lines with the same single output coordinate can be
glued by one nonzero linear relation. The surviving parent side selects the next low-bit residue of
the output stride.
```

This is the algebraic core of exact stability. The no-early-gluing part is what rules out
non-contiguous supports such as one small block in the left half and an unrelated small block in the
right half. The local-glue part is exactly the constructive phase in
`docs/rfc_distance_analysis/rfc_matched_kernel_induction.md`.

The no-early-gluing lemma is split out in:

```text
docs/rfc_distance_analysis/rfc_no_early_gluing_lemma.md
```

## Near-Extremizer Target

The near-extremizer scans suggest the exact stability theorem extends in the simplest possible way:

```text
wt(x)=m, wt(A_d x) <= 2^d/m + e
```

should imply:

```text
supp(x) is still a matched row block,
supp(A_d x) contains a matched stride core of size 2^d/m,
and the remaining output positions are e extras.
```

Equivalently, the support-pair count should be:

```text
2^d * binom(2^d - 2^d/m, e).
```

This near statement is stronger than necessary for distance, but the depth-`4` checks match it
exactly for the tested cases. A weaker version with a small polynomial or `2^O(e log k)` overhead
would still be enough for the current slack calculation.

## Proof Strategy

The next proof step is to formalize the child kernel-line uniqueness:

```text
For every matched block/stride pair (R,W), the restricted zero constraints have a one-dimensional
kernel.
For non-matched pairs at the exact boundary, the kernel is zero.
```

Then exact stability follows by induction from equality conditions. Near stability should follow by
adding `e` free output positions and applying exact stability to the stride core contained in the
near output support.

## Kernel-Line Uniqueness Checks

The helper:

```text
scripts/rfc_distance_analysis/rfc_kernel_line_uniqueness_scan.py
```

checks the exact lemma directly on small depths. At the exact uncertainty boundary, it classifies
every support/output-support pair as either matched or non-matched and records the kernel dimension
of the zero constraints.

Artifacts:

```text
docs/rfc_distance_analysis/rfc_kernel_line_uniqueness_depth3_m2.csv
docs/rfc_distance_analysis/rfc_kernel_line_uniqueness_depth3_m4.csv
docs/rfc_distance_analysis/rfc_kernel_line_uniqueness_depth4_m2.csv
docs/rfc_distance_analysis/rfc_kernel_line_uniqueness_depth4_m4.csv
```

Results:

```text
depth 3, m=2: non-matched kernel dim 0 for 1,952 pairs; matched kernel dim 1 for 8 pairs
depth 3, m=4: non-matched kernel dim 0 for 1,952 pairs; matched kernel dim 1 for 8 pairs
depth 4, m=2: non-matched kernel dim 0 for 1,544,384 pairs; matched kernel dim 1 for 16 pairs
depth 4, m=4: non-matched kernel dim 0 for 3,312,384 pairs; matched kernel dim 1 for 16 pairs
```

This is the cleanest finite evidence for the exact stability theorem. It suggests the theorem can
be proved directly as a recursive rank statement:

```text
dim ker(A[R, [k]\W]) =
  1 if (R,W) is a matched block/stride pair,
  0 otherwise
```

when:

```text
|R| * |W| = k.
```

The constructive matched-pair half of this rank statement is expanded in:

```text
docs/rfc_distance_analysis/rfc_matched_kernel_induction.md
```

The assembled theorem-style draft is:

```text
docs/rfc_distance_analysis/rfc_exact_stability_theorem.md
```
