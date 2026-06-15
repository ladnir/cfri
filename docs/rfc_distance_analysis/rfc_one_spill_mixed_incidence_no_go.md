# One-Spill Mixed-Incidence No-Go

Status: mixed lower-triple/spill-singleton incidence does not close the dominant one-spill row.

## Purpose

The dominant one-spill row needs:

```text
30.196314 q-dimensions.
```

After ruling out local P/C minority-label charge, exact maximality, and canonical counting, the
remaining local hope was:

```text
the spill singleton is not actually free after conditioning on the lower rank-two triple.
```

This note checks that hope in the exact compressed profile:

```text
h0 = 27
z0 = 7
p0 = 3
s0 = 1
D0 = 14.
```

## Algebra

At the spill level, the three paired lower coordinates impose:

```text
rank(P0) = 2
dim K_P0 = D0 = 14.
```

The spill singleton is one lower coordinate `c` and one side. It imposes one linear equation on:

```text
K_P0 plus K_P0.
```

The row has:

```text
h0 = 27 = 2D0 - 1,
```

so the singleton must cut by rank exactly one. This fails only if the lower evaluation functional
at `c` vanishes on `K_P0`, equivalently if the fourth lower column lies in the span of the three
rank-two lower columns.

For rank-one tensor RFC columns, three dependent columns lie on a Segre line. Therefore the spill
singleton is forced to be rank zero only when its lower column is also forced onto that same Segre
line by the same incidence equations.

## Diagnostic

Added:

```text
scripts/rfc_distance_analysis/rfc_one_spill_mixed_incidence_budget.py
```

For the target structural lower stratum:

```text
python scripts/rfc_distance_analysis/rfc_one_spill_mixed_incidence_budget.py \
  --depth 4 --expansion 8 --target-codim 3 --top 20
```

the output is:

```text
triples                         = 7168
total singleton choices          = 896000
forced closure singleton choices = 7168
current log2 count               = 20.773139
corrected log2 count             = 20.761551
saving                           = 0.011588 bits
saving                           = 0.000091 q-dimensions at q=2^128.
```

Every codim-3 triple has exactly one lower singleton coordinate forced onto the same line:

```text
forced_closure_per_triple = 1 for all 7168 triples.
```

The fourth-column incidence profile is:

```text
extra codim 0:     7168 choices
extra codim 3:   688128 choices
impossible:      200704 choices.
```

The nearby codim-4 stratum is even cleaner:

```text
python scripts/rfc_distance_analysis/rfc_one_spill_mixed_incidence_budget.py \
  --depth 4 --expansion 8 --target-codim 4 --top 12
```

gives:

```text
forced closure singleton choices = 0
saving                           = 0 q-dimensions.
```

## Consequence

The rank-one spill singleton condition is generic after the lower triple event. The bad case where
the singleton becomes rank zero is a line-closure exclusion, not a new root equation.

Thus mixed root-line incidence at the spill singleton cannot supply the missing:

```text
30.196314 q-dimensions.
```

It supplies only:

```text
0.000091 q-dimensions
```

in the exact dominant codim-3 lower stratum.

## Updated One-Spill Ledger

The current accounting for the dominant row is now:

```text
structural gap                         30.196314 qdims
canonical counting maximum saving       2.333490 qdims
mixed singleton incidence saving        0.000091 qdims
remaining after both                   27.862733 qdims.
```

Together with the exact-maximality and P/C-label no-gos, this gives a concrete counter-obstruction
to the present common-zero one-spill route. Closing the row now requires a stronger global
framework: for example a theorem that adds real q-codimension to the full top-profile geometry, or
a different mixed-shape recurrence. Merely refining the local spill singleton or recounting the
same witness family is not enough.
