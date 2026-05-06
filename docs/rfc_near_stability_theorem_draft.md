# RFC Near-Stability Theorem Draft

This note drafts the near-extremizer theorem needed for the systematic distance certificate.

## Statement Target

Let `A_d` be one RFC parity transform with `k=2^d`. Let `m=2^t`. Suppose:

```text
wt(x) = m
wt(A_d x) <= k/m + e.
```

The strongest target suggested by the scans is:

```text
supp(x) is a matched row block R of size m;
supp(A_d x) contains a matched stride core W_0 of size k/m;
supp(A_d x) \ W_0 has size at most e.
```

Equivalently, the number of possible support pairs is at most:

```text
k * binom(k-k/m, <= e).
```

The final distance proof only needs this count, not necessarily the exact structural statement.

## Defect

Define the uncertainty defect:

```text
def(x) = wt(A_d x) - k/wt(x).
```

For `wt(x)=m`, near-extremality means:

```text
def(x) <= e.
```

At a parent node with child outputs `u,v`, write:

```text
p = wt(u)
q = wt(v)
r = |supp(u) cap supp(v)|.
```

The parent output has at least:

```text
2p + 2q - 3r.
```

If a two-child branch attempts to glue over common support size `h=r`, the exact branch would keep
one output over each common coordinate. Quantitative no-early-gluing says at most one common
coordinate can cancel generically with the single relative scalar. Hence at least:

```text
h - 1
```

extra outputs are charged to the defect budget.

Before cancellation, child support mismatch is also charged. If:

```text
U = supp(u), V = supp(v), h = |U cap V|,
```

then coordinates in `U triangle V` have no cancellation partner and contribute two parent outputs.
The conservative overlap charge is:

```text
|U \ V| + |V \ U|.
```

There is also a row-split defect. If the parent live weight `m=a+b` is split across both children
with `a,b>0`, then:

```text
wt(Ax at parent) >= max((K/2)/a, (K/2)/b).
```

So an unbalanced split pays at least:

```text
max((K/2)/a, (K/2)/b) - K/(a+b)
```

before any cancellation accounting. This vanishes only at the balanced split `a=b`.

## Induction Invariant

For a node of size `K`, live support size `M`, and output budget:

```text
K/M + e,
```

the support pair can be covered by:

```text
one matched core of size K/M
plus at most e extra output positions,
```

unless the live support already pays at least one unit of defect for each deviation from a single
recursive row block.

The intended induction:

```text
1. One-child descent:
   If the live rows occupy only one child, the parent output is the two-sibling lift of the child
   output. Thus the parent defect is twice the child defect:

   ```text
   def_parent = 2 def_child.
   ```

   Lifting preserves matched-core-plus-extras with the same scaling: each child extra leaf lifts to
   two parent extra leaves, so the charged-extra count remains bounded by the parent defect budget.

2. Two-child branch:
   If the row split is unbalanced, charge the row-split defect first.
   If child output supports do not match, charge the symmetric-difference overlap defect.
   If common child output support h > 1, charge h-1 cancellation extras.
   Remove those charged extras; the uncharged part has h=1 and follows the exact glue step.

3. Repeat until the matched core is exposed.
```

The exact theorem is the special case `e=0`; no early gluing forces all two-child branches to occur
only at `h=1`.

## Counting Consequence

If the theorem holds, then for one sparse parity copy:

```text
number of near support/output pairs with defect e
  <= k * binom(k-k/m, e).
```

Including the choice of sparse parity copy gives:

```text
c_p * k * binom(k-k/m, e).
```

For the optimal collapse live sizes this must be paired with at least `e+1` aggregate zeros in the
remaining `c_p-1` copies, bounded by:

```text
binom((c_p-1)k, e+1) q^{-(e+1)}.
```

So the contribution of defect `e` and live size `m` is bounded by:

```text
c_p * k * binom(k-k/m, e)
  * binom((c_p-1)k, e+1) q^{-(e+1)}.
```

This is exactly the model evaluated in:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128.csv
```

The cumulative-count variant:

```text
number of near support/output pairs with defect at most e
  <= k * sum_{i<=e} binom(k-k/m, i)
```

is evaluated in:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_cumulative.csv
```

At depth `11`, `c=8`, both versions give the same displayed total log2 union bound:

```text
-97.14825096.
```

The corrected certificate model uses the actual sparse-side gap for every `m`:

```text
needed_zeros = max(1, m + k/m + e - 96 + 1)
```

and pays for possible near-core kernel growth by:

```text
q^floor(e/(k/m)).
```

This is evaluated in:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_cumulative_stride_dim.csv
```

At depth `11`, `c=8`, both corrected versions give:

```text
-99.60768258.
```

## Kernel-Dimension Check

The helper:

```text
scripts/rfc_near_pair_kernel_dim.py
```

computes kernel dimensions for saved near-extremizer support pairs. For the checked depth-`4`
families:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_w5.csv
docs/rfc_near_pair_kernel_dim_depth4_m2_w9.csv
docs/rfc_near_pair_kernel_dim_depth4_m2_w10.csv
```

every saved near-extremizer still has kernel dimension exactly:

```text
1.
```

The generated matched-core model at depth `4`, live size `m=4`, and defect `e=2` has:

```text
16 * binom(12,2) = 1056
```

candidate support pairs, saved in:

```text
docs/rfc_uncertainty_near_model_pairs_depth4_m4_e2.csv
```

The kernel-dimension check:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_e2_model.csv
```

finds kernel dimension exactly `1` for all `1056` generated pairs.

The same generated-model check at `m=4`, `e=3` gives:

```text
16 * binom(12,3) = 3520
```

pairs, and:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_e3_model.csv
```

again finds kernel dimension exactly `1` for every pair.

At `m=4`, `e=4`, the generated-model check gives:

```text
16 * binom(12,4) = 7920
```

pairs, saved in:

```text
docs/rfc_near_pair_kernel_dim_depth4_m4_e4_model.csv
```

and the dimensions split as:

```text
kernel_dim 1: 7872 pairs
kernel_dim 2:   48 pairs
```

This corrects the earlier line-uniqueness guess. Extras can create a new kernel direction when they
contain an entire additional stride class. The first-moment model should therefore charge a factor:

```text
q^floor(e/(k/m))
```

rather than assuming every matched-core-plus-extra pair contributes only one line.

The classifier:

```text
scripts/rfc_classify_near_core_dimension.py
```

checks the sharper relation between dimension excess and complete extra stride classes. For the
generated `m=4`, `e=2,3,4` models, every pair satisfies:

```text
kernel_dim - 1 = complete_extra_stride_count.
```

The kernel-dimension sublemma is split out in:

```text
docs/rfc_near_kernel_line_lemma.md
```

That sublemma now has a direct proof: above the live row block, a child output coordinate can remain
nonzero only when both sibling lifts are allowed; iterating this condition leaves exactly complete
global stride classes. Inside the live block the RFC transform is invertible, so the kernel
dimension equals the number of complete allowed stride classes.

## Remaining Proof Gap

The remaining proof obligation is specific:

```text
Show that every deviation from matched-core recursion can be charged injectively to an extra output
position.
```

The quantitative no-early-gluing lemma gives the local charge `h-1`. What remains is bookkeeping:
charges from different recursive nodes must not overcount the same extra output position.

A plausible way to formalize this is to charge each failed cancellation to the surviving sibling
output position created at that node. That position is outside the eventual matched stride core and
is unique to that node/coordinate in the recursion tree.

The charge-injection bookkeeping is split out in:

```text
docs/rfc_near_defect_charge_injection.md
```
