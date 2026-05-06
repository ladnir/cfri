# RFC Near-Stability Defect Charge Injection

This note sketches the bookkeeping needed to complete near-stability.

## Local Charge

At a node where both child outputs are nonzero, let the common child output support be:

```text
W' = supp(u) cap supp(v)
h = |W'|.
```

For each `j in W'`, the parent has two sibling output positions:

```text
(0,j), (1,j).
```

Exact gluing would keep exactly one of these siblings for every `j`. Quantitative no-early-gluing
says that, generically, at most one coordinate `j` can have a cancelled sibling using the single
relative scalar between child kernel lines.

Therefore at least:

```text
h - 1
```

coordinates in `W'` have both parent siblings present. For each such coordinate, choose one of the
two siblings as the core-continuation position and charge the other sibling as an extra output.

## Disjointness Principle

Charges can be made injective by assigning every charge to the first node on the root-to-leaf path
where that output position diverges from the eventual matched stride core.

Equivalently:

```text
An output leaf outside the matched stride core has a unique highest ancestor at which it first takes
the wrong sibling relative to the core path.
```

Charge the leaf to that ancestor. Then:

```text
different charged divergences produce different output leaves.
```

This avoids double-counting across recursive levels.

## Inductive Counting Form

Let `Core(node)` be the matched stride core exposed by following the uncharged branch choices. Let
`Extra(node)` be the set of output leaves charged by failed cancellations.

The induction should maintain:

```text
supp(Ax at node) subset Core(node) union Extra(node)
|Extra(node)| <= e_node.
```

The transitions are:

```text
one-child descent:
  Core lifts to both siblings;
  Extra lifts to both siblings only when those leaves are already in the output support budget;
  defect count is preserved in the child coordinates.

two-child exact glue (h=1):
  Core chooses one sibling;
  no extra is charged.

two-child near glue (h>1):
  at most one coordinate continues as exact glue;
  every other common coordinate contributes one charged sibling leaf;
  these charged leaves are outside the core and disjoint by first-divergence assignment.
```

This yields the structural containment:

```text
support pair with defect e
  => contains a matched core plus at most e extra leaves.
```

## Counting

Once the containment is proved, counting is immediate:

```text
choose matched core:        k choices
choose extra output leaves: binom(k-k/m, <= e)
```

For exact defect `e`, the model used in the certificate takes:

```text
k * binom(k-k/m, e).
```

The final theorem may naturally produce the cumulative `<= e` form. That only changes the union
bound by a small factor for the target parameters, and can be handled by summing over all smaller
defects.

## Remaining Formal Detail

The subtle point is the one-child descent line above. A clean way to avoid ambiguity is to prove the
near theorem top-down on support pairs rather than on already-formed cores:

```text
At every node, choose a maximal exact matched core contained in the current output support.
Show that any output outside that core is charged to its first divergence.
```

This should turn the defect-charge injection into a deterministic tree statement, independent of
field algebra. The field algebra only enters through the quantitative no-early-gluing lemma.
