# RFC Uncharged Skeleton Reduction

This note isolates the reduction needed before applying no-early-gluing inside the defect-coupled
virtual-core proof.

## Purpose

The alpha-2 local hole lemma assumes that, after charges have been emitted, the two child outputs at
an unbalanced node have only one relative scalar. This is the same algebraic condition used in the
exact no-early-gluing lemma.

The reduction target is:

```text
charge all child defects and complete extra stride-class dimensions first;
the remaining uncharged child skeleton is one-dimensional on each side.
```

Then a parent sibling cancellation at coordinate `j` imposes one equation on the single relative
scalar between the two child skeleton lines. Two distinct uncharged cancellations would impose two
fresh-random ratio equations, which is generically impossible.

## Inputs

The near-kernel line lemma says that for a matched child core `C_child` plus outside leaves `E_child`:

```text
dim <= 1 + floor(|E_child| / |C_child|).
```

Equivalently, every additional child output direction requires a complete extra stride class of
size `|C_child|`. In the first-moment certificate this is already charged by:

```text
q^floor(|E_child| / |C_child|).
```

For virtual cores, outside leaves have size:

```text
|E_child| = e_child + h_child.
```

so the dimension charge uses:

```text
floor((e_child+h_child)/|C_child|).
```

This is exactly the corrected accounting in:

```text
scripts/rfc_near_extremizer_total_union.py
```

## Reduction Statement

At a parent node, suppose the recursive encoder has chosen child virtual cores:

```text
(C_0,H_0,E_0), (C_1,H_1,E_1).
```

Charge:

```text
1. child outside leaves not used by the selected child skeleton;
2. complete extra child stride classes, via the dimension factor;
3. local overlap/residual leaves outside the selected parent virtual residue.
```

After these charges, each child contributes only one uncharged skeleton line:

```text
u_j = alpha U_j,
v_j = beta  V_j,
```

on the coordinates retained for the parent local comparison, with `U_j,V_j` fixed over the child
field and a single relative scalar:

```text
lambda = beta / alpha.
```

Thus every uncharged sibling cancellation has the form:

```text
lambda = L_j(T_j) * U_j/V_j.
```

The parent challenges `T_j` are fresh and independent across child coordinates. Therefore the
quantitative no-early-gluing lemma gives:

```text
at most one uncharged sibling cancellation at this parent node.
```

This is the input used in the alpha-2 local theorem:

```text
c_C + c_O <= 1.
```

## What Is Still Formal

The remaining bookkeeping is to show that the charges above can be assigned to final output leaves
without colliding with the local alpha-2 hole payments. This is handled by the same
first-divergence rule:

```text
each charged leaf is assigned to the highest node where it leaves the selected virtual core path.
```

Complete extra stride-class dimensions do not need an injective leaf assignment beyond the
`q^floor((e+h)/|C|)` factor in the first-moment count, but their outside leaves are still included
in the chosen set `E`.
