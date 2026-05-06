# RFC Near-Stability Proof Strategy

This note records the quantitative extension of exact stability.

## Goal

For a live support size:

```text
m = 2^t
```

the exact theorem says that output support size:

```text
k/m
```

forces a matched block/stride pair.

The near-stability target is:

```text
wt(x) = m
wt(Ax) <= k/m + e.
```

The observed model is:

```text
support pairs <= k * binom(k-k/m, e),
```

i.e. choose one exact matched stride core and then add `e` arbitrary extra output positions.

## Quantitative No-Early-Gluing

The exact no-early-gluing lemma says that if the two-child branch tries to cancel over a common
child output support `W'` with:

```text
|W'| > 1,
```

then it cannot cancel one sibling at every coordinate. The quantitative version is:

```text
With one relative scalar between the two child kernel lines, at most one coordinate of W' can
satisfy the required parent cancellation equation generically.
```

Therefore, if a two-child branch happens with common support size:

```text
h = |W'|,
```

then the parent output support over those `h` child coordinates has size at least:

```text
1 + 2(h-1) = 2h - 1.
```

The exact matched branch would have output size `h`. Thus early gluing costs at least:

```text
h - 1
```

extra output positions.

## Consequence

If the total near-extremal slack is `e`, then any early two-child glue can only occur at a node
where:

```text
|W'| <= e + 1.
```

Equivalently, all higher levels where the projected output support has size greater than `e+1` must
follow the one-child descent branch. This forces a large matched row block and a matched stride core
before any deviations are possible.

In the strongest expected form, every near-extremizer contains an exact matched stride core of size:

```text
k/m
```

and the `e` slack positions are simply extras around that core.

## Why This Matches The Scans

At depth `4`, the checked near-extremizers satisfy exactly this model:

```text
m=4,e=1: count = 16 * binom(12,1) = 192
m=2,e=1: count = 16 * binom(8,1)  = 128
m=2,e=2: count = 16 * binom(8,2)  = 448
```

Each saved near-extremizer contains a matched stride core and uses all remaining allowed support as
extras.

## Proof Path

The proof should proceed by induction with a defect parameter `e`.

At a node with live support size `m_node` and output support budget:

```text
k_node/m_node + e_node,
```

the equality proof branches become:

```text
one-child descent:
  preserves the defect e_node;

two-child glue over common child support h:
  consumes at least h-1 units of defect unless h=1.
```

Thus all gluing before the full live node must be charged to the defect budget. After deleting the
charged extra output positions, the remaining uncharged structure follows the exact matched
induction.

This should prove a count of the form:

```text
near support pairs with defect e
  <= k * binom(k-k/m, <= e)
```

or a slightly looser `k * (ek)^e` bound. The exact binomial form is supported by the current scans;
the looser form would still be enough for the depth-`11`, `c=8` slack estimates.

## All-`m` Slack Check

The helper:

```text
scripts/rfc_near_extremizer_slack_all_m.py
```

evaluates the matched-plus-extra model over every power-of-two live size `m`. At depth `11`,
`c=8`, and `q=2^128`, the table:

```text
docs/rfc_near_extremizer_slack_all_m_depth11_c8.csv
```

shows that the worst term for every `m` is the exact case `e=0`:

```text
log2 union bound = -100.60768258.
```

So the distance-dominant `m=32` case is not hiding a worse small- or large-`m` near-extremal leak
under this model.
