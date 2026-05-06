# RFC Rank-Cancellation Exchange

This note isolates the algebraic lemma left after the alpha-2 size accounting.

## Goal

At an unbalanced parent node, after all child defects and complete extra child stride-class
dimensions are paid, the local hole lemma needs:

```text
c_C + c_O <= 1.
```

Equivalently, every sibling cancellation beyond the first must be charged before the local
alpha-2 hole count is applied.

## Local Model

Fix a parent node and fix the two child support windows used by the selected parent residue. After
child zero constraints are imposed, let the two child admissible output spaces have dimensions:

```text
r_0 = 1 + s_0,
r_1 = 1 + s_1.
```

Choose one base line in each child and write the residual outputs as:

```text
u = alpha U + sum_i a_i U_i,
v = beta  V + sum_i b_i V_i.
```

The relative base scalar is:

```text
lambda = beta / alpha.
```

The paid child-rank budget is:

```text
s = s_0 + s_1.
```

The near-kernel line lemma pays this by complete extra stride classes:

```text
s <= floor((e_child+h_child)/|C_child|)
```

on each child, and the systematic first-moment count already includes the corresponding factor.

## Exchange Lemma Target

For any set `J` of parent child-coordinates where sibling cancellations occur, the cancellation
conditions impose independent equations except for:

```text
1 relative scalar lambda,
s paid child-rank parameters.
```

So after paying `s` rank parameters, the residual number of uncharged sibling cancellations is at
most one:

```text
|J| - s <= 1.
```

The charged-tree proof can then remove `s` cancellations from the local hole calculation and pass
only the residual set to the alpha-2 size lemma.

## Why This Is The Right Shape

The exact no-early-gluing proof is the `s=0` case. Two different cancelled siblings force two
different equations for the same `lambda`; fresh independent fold challenges make this impossible
except on the usual bad-challenge event.

With paid child rank, the equations become linear constraints in:

```text
lambda, a_i/alpha, b_i/alpha.
```

Each extra child direction can absorb at most one additional independent cancellation equation.
Thus the proof should not try to forbid extra cancellations outright. It should charge them to the
rank dimensions that the first-moment count already paid for.

## First-Moment Interface

For a fixed local pattern with `s` paid rank parameters and `t` sibling cancellations, the expected
contribution should carry a factor:

```text
q^s * q^(-max(0,t-s-1)).
```

The `q^s` term is already present in the near-kernel dimension count. The residual loss is the
same no-early-gluing loss as the exact proof once `t>s+1`.

For the deterministic structural theorem, we can phrase this as:

```text
pay min(s,t-1) cancellations by rank;
leave at most one uncharged cancellation for the alpha-2 local lemma.
```

## Remaining Algebraic Check

The real work is to prove the independence statement for the actual RFC fold equations with the
current challenge choice `T` uniform nonzero. A useful local certificate would be:

```text
For every fixed child support/rank pattern and every cancellation set J,
the cancellation matrix has generic rank at least |J|-1 over the paid child-rank quotient.
```

This is now the only algebraic obstruction in the proof chain. If it holds, the rest of the
systematic distance certificate is already reduced to:

```text
1. alpha-2 local size inequality;
2. first-divergence charge composition;
3. first-moment count with virtual holes and q^floor((e+h)/|C|).
```
