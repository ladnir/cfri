# RFC Support-Three Rank-Defect Incidence Lemma

Scope: original non-systematic RFC, determinant-1 fold with `T` uniform in `F^*`.

Status: local theorem target/proof sketch for the support-three component-plane row. This is not a
complete distance certificate by itself.

## Row

The row is the decomposable support-three tau-two row:

```text
parent span = 2
tau = 2
|A| = 3
delta(A) = 3
comp(A) = 3
K = 0
dim V = 4
```

After the child zero coordinates have been fixed, write the remaining child ambient as:

```text
E = H_child restricted by the already-requested child zeros.
```

Let the three active coordinate functionals on `E` be:

```text
lambda_1, lambda_2, lambda_3 in E^*.
```

For this `delta=3` exact-support row, these three active functionals are independent modulo the
already-zero coordinates. If they were not, the row would have `delta < 3` after exact-support
normalization and would be routed to a smaller support profile.

## Component-Plane Count

Fix a child container:

```text
V <= E,  dim V = 4.
```

For each active coordinate `i`, the corresponding rank-one component plane `P_i <= V` is zero at
the other two active coordinates:

```text
P_i <= V cap ker(lambda_j) cap ker(lambda_k),  {i,j,k}=A.
```

If the restrictions `lambda_i|_V` have rank three, each two-kernel intersection has dimension two.
Thus the three component planes are fixed up to finite constants after `V` is fixed, and the only
remaining q-dimensional local family is the exact-support two-plane inside the three component
directions:

```text
q^2.
```

This is the `rank3` diagnostic count.

The safe count allowed one proportional pair among the active restrictions. In that stratum, one
intersection can be three-dimensional and its component plane can cost:

```text
[3 choose 2]_q = O(q^2).
```

Together with the local exact-support two-plane, this gives the safe `q^4` post-root count.

## Rank-Defect Incidence Charge

The missing observation is that the rank-defect stratum for `V` has codimension two at the
container level.

If:

```text
rank(lambda_1|_V, lambda_2|_V, lambda_3|_V) < 3,
```

then there is a nonzero projective relation:

```text
[c_1:c_2:c_3] in P^2(F_q)
```

such that:

```text
mu_c = c_1 lambda_1 + c_2 lambda_2 + c_3 lambda_3
```

vanishes on `V`:

```text
V <= ker(mu_c).
```

Since `delta(A)=3`, every nonzero `mu_c` is a genuine extra functional on `E`; it is not already in
the span of the child zero coordinates. For a fixed relation `c`, the number of four-dimensional
containers inside this extra hyperplane is:

```text
[dim(E)-1 choose 4]_q.
```

Relative to the unconstrained container count:

```text
[dim(E) choose 4]_q,
```

this costs `q^{-4}` up to the standard Gaussian constant. There are:

```text
|P^2(F_q)| = q^2 + q + 1
```

projective relations. Therefore the union of rank-defect containers costs:

```text
O(q^2) * q^{-4} = O(q^{-2})
```

relative to the unrestricted child-container event.

This two-q-dimensional incidence charge exactly pays for the extra `q^2` component-plane family
that safe mode allowed in the proportional-pair stratum. Hence both strata have the same
q-dimensional exponent:

```text
rank-three restrictions:
  component/local count q^2

rank-defect restrictions:
  incidence charge q^-2 * safe component/local count q^4 = q^2.
```

So the support-three row can use the four-qdim saving previously exposed by the `rank3` sensitivity
mode, provided the recurrence accounts for the finite Gaussian and projective constants.

## Recurrence Consequence

The support-three component-plane helper now exposes this as:

```text
--support3-component-plane-mode stratified
```

It has the same exponent as `rank3`, but a different interpretation:

```text
rank3:
  sensitivity mode assuming pairwise-independent active restrictions.

stratified:
  theorem-target mode splitting rank-three and rank-defect containers, charging rank defect by the
  extra-hyperplane incidence count above.
```

This keeps quotient-plane incidence visible. It does not use the retired all-lift shortcut and does
not multiply child moments over shared randomness.

## Finite Constants

The finite overhead from this local split is bounded by a fixed product of:

```text
|P^2(F_q)| / q^2 <= 1 + q^-1 + q^-2,
Gaussian ratio constants for [m-1 choose 4]_q / [m choose 4]_q,
the ordered choice of the proportional/defective relation.
```

At `q=2^128`, this is far below one q-dimension. The final certificate still has to place these
constants in the global finite bucket.

## Remaining Caveat

This lemma relies on the exact-support interpretation of `delta(A)=3`: the active coordinate
functionals must be independent modulo the already-zero child coordinates. If a future recurrence
state allows the active span to have smaller rank, that state must be rerouted to the corresponding
smaller `delta` row rather than charged by this lemma.
