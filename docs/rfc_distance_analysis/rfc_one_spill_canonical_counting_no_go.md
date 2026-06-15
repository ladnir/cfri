# One-Spill Canonical-Counting No-Go

Status: witness de-duplication and exact-support counting do not close the dominant one-spill row.

## Purpose

The dominant one-spill row is short by:

```text
30.196314 q-dimensions.
```

One possible explanation was that the ledger counts the same witness family many times:

```text
1. choose C inside Z=P union C;
2. choose a singleton side for common-zero coordinates C;
3. choose T\C as a selected subset of outside singleton zeros.
```

This note checks the most generous version of that idea. It removes every obvious duplicate
description and asks whether the row becomes safe.

## Dominant Row

The structural one-spill row is:

```text
h = 864
F = 41
z = 224
a = |C| = 87
t-a = 1758
total_q = 34
baseline_log2_term = 3785.128176
baseline_gap = 30.196314 q-dimensions.
```

The top child length is:

```text
n_child = 8192,
n_child-z = 7968.
```

## Exact Outside Tail

For a fixed projective witness line, every outside coordinate in `T\C` contributes at most two
singleton sides and one root equation. The selected-subset ledger uses:

```text
binom(7968,1758) 2^1758 q^-1758.
```

The exact "at least 1758 outside singleton zeros" tail is:

```text
sum_{m>=1758} binom(7968,m) 2^m q^-m.
```

At `q=2^128`, this tail is dominated by its first term. Therefore exact outside support counting
does not recover any q-dimension:

```text
selected outside shape bits        = 7817.668295
exact tail shape-equivalent bits   = 7817.668295
outside-tail saving                = 0.000000 q-dimensions.
```

This is not a numerical accident. Consecutive terms have ratio roughly:

```text
((7968-m)/(m+1)) * 2/q,
```

which is far below one at the target field size.

## Common-Zero Side And P/C Labels

The most aggressive canonicalization can also delete:

```text
common-zero side choices: 87 bits       = 0.679688 q-dimensions
P/C labels inside Z:       211.686742 bits = 1.653803 q-dimensions.
```

This is generous: removing all P/C labels treats the actual parent zero support as if every
coordinate in `C` were counted as paired, so the parent support has:

```text
selected zero count = 2p+t = 2119
actual zero count   = 2(p+a)+(t-a) = 2206.
```

Even under this aggressive model, the row remains positive:

```text
aggressive canonical log2 term = 3486.441435
remaining gap                  = 27.862824 q-dimensions.
```

## Diagnostic

Added:

```text
scripts/rfc_distance_analysis/rfc_one_spill_canonical_counting_budget.py
```

Command:

```text
python scripts/rfc_distance_analysis/rfc_one_spill_canonical_counting_budget.py
```

Key output:

```text
baseline_gap_q,30.196314
outside_tail_saving_qdims,0.000000
common_zero_side_saving_qdims,0.679688
pc_label_saving_qdims,1.653803
aggressive_canonical_gap_q,27.862824
```

## Consequence

Sharper canonical counting of the same witness family is not enough for this row. It can remove at
most the visible duplicate-description entropy, about:

```text
2.333490 q-dimensions,
```

leaving:

```text
27.862824 q-dimensions
```

unaccounted for.

Combined with the earlier local no-gos:

```text
1. P/C minority labels are invisible inside Z=P union C;
2. exact maximality outside C only supplies the already-counted residual root exponent;
3. exact outside-support counting has the same q-exponent as selected-subset counting;
```

the current common-zero one-spill route needs a genuinely new algebraic/global q-charge. The likely
remaining options are:

```text
1. a stronger mixed tensor-rank theorem that couples the lower rank-two triple to the spill
   singleton or to the lifted block structure;
2. a global survivor-profile theorem that proves additional q-codimension, not just fewer labels;
3. a different framework than common-zero replacement for mixed paired/singleton shapes.
```
