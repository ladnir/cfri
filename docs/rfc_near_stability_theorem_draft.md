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
   The child support pair has the same defect e.
   Lifting preserves matched-core-plus-extras.

2. Two-child branch:
   If common child output support h > 1, charge h-1 extras.
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

To beat the collapse baseline, the remaining `c_p-1` copies must provide at least `e+1` aggregate
zeros, bounded by:

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

So adding extra output positions around a matched core does not appear to create larger kernel
families in these checks. This supports the counting model: each near support pair contributes one
candidate line, not a high-dimensional subspace that would need extra first-moment mass.

## Remaining Proof Gap

The proof gap is now narrow:

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
