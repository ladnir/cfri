# RFC Matched Kernel Induction

This note spells out the constructive half of the exact stability theorem:

```text
dim ker(A[R, [k]\W]) = 1
```

for matched block/stride pairs.

## Matched Pair

At depth `d`, let:

```text
k = 2^d
m = 2^t.
```

A matched pair is:

```text
R = { s m, s m + 1, ..., s m + m - 1 }
W = { r, r + m, r + 2m, ..., r + (k/m - 1)m }.
```

Here `R` is a row block of size `m`, and `W` is an output stride class modulo `m`.

The zero constraints are all output positions outside `W`. The claim is that the restricted map
from messages supported on `R` to outputs outside `W` has a one-dimensional kernel.

## Phase 1: Descend To The Live Block

If the current node size is larger than `m`, then the row block `R` lies entirely in one child.
Assume it lies in the left child; the right-child case is identical.

The parent outputs are:

```text
y_0 = (1-T) u
y_1 = -T u
```

because the right child message is zero. Over the generic field, both scalars are nonzero. Thus:

```text
supp(parent output) = left lift of supp(u) union right lift of supp(u).
```

The parent stride set `W` is exactly the union of the two sibling lifts of the child stride set:

```text
W_child = { r, r + m, ..., r + (k/(2m)-1)m }.
```

Therefore the parent zero constraints outside `W` are equivalent to the child zero constraints
outside `W_child`. The kernel dimension is preserved.

Repeating this step descends to the unique depth-`t` node whose full row set is `R`.

## Phase 2: Full Node To One Output

Now the current node has size `m`, and the live row set is the full node. The matched output support
inside this node has size:

```text
1.
```

Write the desired output residue `r` in binary. At the root of this node, the desired output lies in
one of the two parent output halves. Both child messages must be nonzero and exact recursively, with
the same child output coordinate. The local `2 x 2` MDS matrix maps the two child scalar outputs to
two parent sibling outputs. Imposing that one sibling output is zero gives one linear relation
between the two child kernel-line scalars.

Since each child kernel is one-dimensional by induction, this relation leaves a one-dimensional
parent kernel. Then recurse into the selected child output coordinate. Each bit of `r` chooses which
parent sibling survives at that level.

Thus the full-node matched pair has a one-dimensional kernel.

## Uniqueness

The same induction also gives uniqueness:

```text
Phase 1 preserves the child kernel exactly.
Phase 2 glues two one-dimensional child kernels with one nonzero linear relation.
```

So the resulting kernel never has dimension more than one. Existence and uniqueness together prove:

```text
dim ker(A[R, [k]\W]) = 1
```

for every matched pair.

## Non-Matched Pairs

For the exact boundary `|R| |W| = k`, any nonzero kernel vector would have:

```text
wt(x) <= |R|
wt(Ax) <= |W| = k/|R|.
```

By one-copy uncertainty, both inequalities must be equalities:

```text
wt(x) = |R|
wt(Ax) = |W|.
```

Therefore any nonzero kernel at the exact boundary is an exact extremizer. The exact stability
classification then rules out every non-matched pair. Equivalently:

```text
dim ker(A[R, [k]\W]) = 0
```

for non-matched pairs.

So the remaining proof task is the classification direction: every exact extremizer must be
matched. The matched-kernel induction above supplies the constructive side and the kernel-line
uniqueness needed by the cancellation-consistency argument.
