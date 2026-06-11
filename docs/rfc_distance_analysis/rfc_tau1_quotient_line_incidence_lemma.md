# RFC Tau-One Quotient-Line Incidence Lemma

Scope: original non-systematic RFC, determinant-1 fold, `T` uniform in `F^*`.

Status: proof-safe local subcase for the finite quotient-incidence recurrence.

## Purpose

The cover-lift diagnostics showed that tau-positive quotient data cannot be erased. The smallest
safe correction is the tau-one fixed-flag row:

```text
L <= V,
Q = V/L,
tau = 1.
```

For a fixed child flag, all parent lifts with the same quotient line datum are covered by one
container event. But the quotient line itself is still event data and must be counted.

## Setup

Fix the child code and a child flag:

```text
L <= V.
```

Let `E` be the quotient ambient in which the parent visible line `W/K` is chosen after the kernel
child is fixed. In the coarse one-layer lift this ambient is a subspace of:

```text
(V+V)/K.
```

After quotienting by the child kernel flag, its visible part maps to:

```text
Q + Q,  Q = V/L.
```

Fix a singleton block `S` and an exact visible support:

```text
A subset S,  |A| = a.
```

Define the contained-support ambient:

```text
E_A = { e in E : e has zero singleton projection on S \ A }.
```

Let:

```text
m_A = dim image(E_A -> A),
f_A = dim ker(E_A -> A).
```

Thus:

```text
dim E_A = f_A + m_A.
```

The exact-support requirement further removes lines whose projection vanishes on some coordinate
of `A`; for an upper bound we may ignore that removal and count all projective lines in `E_A` with
nonzero projection to `A`.

## Lemma

For fixed `E`, `S`, and `A`, the expected number over the independent nonzero roots `T_j`,
`j in A`, of tau-one quotient lines with support contained in `A` and compatible with all singleton
zero equations is at most:

```text
C_q^(a+1) q^(f_A + m_A - 1 - a),
```

where:

```text
C_q = q / (q - 1).
```

Equivalently, up to finite constants, the post-root q-exponent is:

```text
f_A + (m_A - 1) - |A|.
```

If `m_A = 0`, no exact-support line exists.

## Proof

The number of projective lines in `E_A` is:

```text
(q^(f_A+m_A) - 1) / (q - 1) <= C_q q^(f_A+m_A-1).
```

Fix one such line `R`. For each `j in A`, exact support means the projection of `R` to the
two-dimensional endpoint pair at coordinate `j` is nonzero. Pick any nonzero representative
`(x_j,y_j)` of that projected line. The selected singleton equation has the form:

```text
x_j + T_j (y_j - x_j) = 0.
```

If a solution exists, it is unique in `F`; it may be zero, in which case it is not sampled. Hence:

```text
Pr[T_j in F^* satisfies the equation] <= 1/(q-1).
```

The roots are independent over coordinates, so:

```text
Pr[R is compatible on A] <= (q-1)^(-a)
                         = C_q^a q^(-a).
```

Multiplying by the line count contributes the extra projective-line factor `C_q` and proves the
bound. Requiring exact support on every coordinate of `A` or imposing additional child-code
incidence constraints only decreases the count.

This argument is binary-field compatible. For `q=2`, `C_q q^-1 = 1`, which matches the fact that
there is only one nonzero root and the pointwise probability bound can be trivial.

## Relation To The Recurrence

The fixed quotient line datum is:

```text
R <= E_A.
```

For fixed child flag and fixed `R`, all parent lifts in the same fiber satisfy the same zero
witness:

```text
V zero on P union (S \ A),
L zero on P union S,
R root-compatible on A.
```

Therefore duplicate parent extensions of that fixed `R` should not be counted as separate
container events. However, the family of possible quotient lines contributes the exponent above.
This is exactly why:

```text
cover tau-positive lifts completely
```

is anti-conservative.

## Sharpening Target

The universal bound above may be loose when the represented quotient matroid has strong support
structure. A sharper tau-one row should replace the projective ambient count by exact support
subcode counts:

```text
line_count_exact(Q,A)
```

or by a rooted diagonal-intersection count after fixing the root assignment. That sharpening is
optional for proof safety but may be needed to close the depth-5 base seal with small constants.

## Diagnostic Meaning

The dominant safe tau-zero trace contains tau-one rows. This lemma says those rows are well-posed
as quotient-incidence events, but it does not claim a new exponent by itself. For rows with small
`a`, especially `a=1`, the universal exponent can match the existing coarse quotient-lift cost.
The next implementation step is to expose `E_A`, `m_A`, and `f_A` in the finite state so we can see
which tau-one rows admit support-subcode sharpening.
