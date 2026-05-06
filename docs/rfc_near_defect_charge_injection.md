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

## Canonical First-Divergence Lemma

Fix a candidate matched core `C` in the final output leaves. For an output leaf `ell notin C`, define
`first(ell)` to be the highest internal node where the path to `ell` takes a sibling branch that is
not used by the core path through that node.

The bookkeeping lemma we need is:

```text
Every local failed cancellation at a two-child node can be assigned to an output leaf ell notin C
whose first divergence is that node.
```

If this assignment exists, it is automatically injective. Indeed, two different nodes cannot have
the same `first(ell)`. Two failed cancellations at the same node but at different child coordinates
also produce different descendant leaves, because the child coordinate suffix differs.

This is the combinatorial heart of the near theorem. The algebraic input only proves that a node
with common continuation set of size `h` creates at least `h-1` non-core sibling leaves. The
first-divergence lemma says those locally created leaves stay available for charging and are not
reused by lower nodes.

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
  Extra lifts to both siblings;
  both the defect and the number of extra leaves double when moving from the child to the parent.

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

## Budget Recurrence

The induction should track an integer budget `E(node)` rather than reusing the same symbol at every
scale.

For a node of output length `K`, live-row weight `M`, and output support `Y`, define:

```text
E(node) = |Y| - K/M.
```

The desired conclusion is:

```text
there is a matched core C(node) subset Y with |Y \ C(node)| = E(node).
```

This form makes the one-child case exact. If the live rows occupy only one child, then:

```text
Y_parent = lift(Y_child)
|Y_parent| = 2 |Y_child|
K_parent/M = 2 K_child/M
E_parent = 2 E_child.
```

By induction the child has:

```text
Y_child = C_child union Extra_child
|Extra_child| = E_child.
```

Lifting both sides gives:

```text
Y_parent = lift(C_child) union lift(Extra_child)
|lift(Extra_child)| = 2 E_child = E_parent.
```

Thus the one-child step has no hidden loss.

The two-child step is the only place where algebra is used. If the two child branches have common
candidate continuation set of size `h`, the exact matched recursion can keep one coordinate without
paying an extra leaf. Quantitative no-early-gluing forces every other attempted continuation to
leave an additional sibling output. The local contribution is therefore at least:

```text
h - 1
```

new leaves outside the selected core continuation.

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
