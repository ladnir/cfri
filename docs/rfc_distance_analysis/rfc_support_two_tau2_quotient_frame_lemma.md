# RFC Support-Two Tau-Two Quotient-Frame Lemma

Scope: original non-systematic determinant-1 RFC with `T` uniform in `F^*`.

Status: local theorem target / proof sketch. This note isolates the next blocker exposed by the
decoded pair-table trace. It is not yet wired into the global recurrence.

## Stress Shape

The current nested structural trace for the level-3 state:

```text
(4,7) >= (2,8)
```

has top outer row:

```text
p = 2
s = 3
a = 2
tau = 2
dim V = 4
dim L = 2
z_V = 3
z_L = 5
local charge = 4
lift qdim = 12 = 4 kernel + 8 quotient
```

After the lower tau-one quotient is nested into the upper plane, the trace still reports:

```text
outer adjusted lift-minus-charge qdim = 8
outer remaining quotient lift qdim    = 8
```

So the live local obstruction is the upper support-two quotient plane. It is not an independent
tau-one line count and it is not primarily a kernel-fiber cover.

## Exact-Support Structure

For this row, the represented visible support has:

```text
|A| = 2
delta(A) = 2
comp(A) = 2
```

Thus the represented quotient restriction splits as two rank-one components. Write:

```text
A = {j_1, j_2}.
```

The child containers satisfy:

```text
L <= V
dim L = r
dim V = r + 2
V zero on P union (S \ A)
L zero on P union S.
```

For an exact support-two tau-two quotient, the two component directions determine intermediate
child containers:

```text
M_1, M_2
```

with:

```text
L <= M_i <= V
dim M_i = r + 1
M_1 zero on P union (S \ A) union {j_2}
M_2 zero on P union (S \ A) union {j_1}
```

The resulting child object is a diamond, not a chain:

```text
      V(z)
     /    \
 M_1(z+1) M_2(z+1)
     \    /
      L(z+2)
```

Here `z = |P| + |S \ A|`.

## Lemma Target

Fix an exact support-two tau-two transition profile with the decomposable local type above. After
conditioning on the child code, count child diagrams:

```text
L <= M_1, M_2 <= V
```

once in the same child-code instance. For each fixed diagram and fixed determinant-1 root labels
on `j_1,j_2`, the number of parent quotient planes `W/K` realizing the same quotient-frame datum is
bounded by a finite projective constant; in q-dimensional bookkeeping, the ambient quotient-plane
lift:

```text
q^{2(2 dim V - dim W)}
```

is replaced by the quotient-frame child diagram and at most one frame-completion factor:

```text
q + 1
```

for choosing the second line in `V/L` after the first marked component is carried.

Equivalently, the recurrence should route this row through a joint diagram moment such as:

```text
F_child(V(z), M_1(z+1), M_2(z+1), L(z+2))
```

or the coarser ordered version:

```text
(q+1) * F_child((r+2,z), (r+1,z+1), (r,z+2)),
```

where the latter is safe only if the chain relaxation is proved to overcount the diamond.

## Proof Sketch

Condition on the child code and a fixed child diagram `L <= M_1,M_2 <= V`.

The quotient `V/L` is two-dimensional. The exact support-two split gives two quotient lines:

```text
M_i/L <= V/L.
```

A parent quotient plane `W/K` maps to `V/L` by child projection. Under the exact row labels
`pi(W)=V` and `pi(K)=L`, this map is an isomorphism: if a quotient vector projects into `L`, then
its singleton evaluations vanish on all of `S`, so it lies in the singleton kernel `K`.

In the basis given by the two component lines, the determinant-1 singleton equation at `j_i` fixes
the projective root line for the corresponding component direction. Once the root labels and the
two component quotient lines are fixed, the compatible graph of `W/K` inside the doubled child
quotient is therefore determined up to finite choices from projectivization and the nonzero-root
normalization.

The large Gaussian quotient-plane factor counted by the scalar recurrence is thus duplicate
ambient placement data. The real event data are:

```text
1. the child diagram containing the two marked quotient components;
2. the two root labels;
3. finite frame/projective constants;
4. the separate kernel-fiber datum, if it is consumed by another layer.
```

The proof must not replace the child diagram by a product of two line moments. Both marked
components live in the same child-code instance, so the safe first-moment object is one joint
diagram moment.

## Soundness Conditions

The lemma is intended only for the decomposable support-two local type:

```text
|A| = 2, delta(A) = 2, comp(A) = 2.
```

It does not apply to connected `|A|=3,delta=2,comp=1` rows, which are handled by
`rfc_u23_tau2_endpoint_lemma.md`, or to higher-drop tau-two rows.

The global recurrence must still:

```text
1. carry the diamond diagram, or prove a chain relaxation that overcounts it safely;
2. keep quotient/root labels as event data;
3. apply collapsed-active rerouting if `L` and `V` merge;
4. distinguish unconsumed kernel fibers from kernel subspaces consumed by lower layers;
5. charge finite constants from ordered support labels, projective roots, and `q+1` frame completion.
```

No product of child first moments is allowed.

## Immediate Diagnostic Consequence

If this lemma is integrated into the pair-table recurrence, the decoded top row should no longer
show:

```text
outer_remaining_quotient_lift_qdim = 8.
```

That term should be replaced by a joint child diagram query plus at most one q-dimensional
frame-completion factor. This is the concrete implementation test for the lemma.
