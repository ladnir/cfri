# RFC Tau-2 Product Endpoint Formula

Scope: original non-systematic RFC only. This note derives the non-enumerative
endpoint count for the product-style `tau=2` row in the paired-spine gate.

## Row

The target row is:

```text
support size a = 8
delta = 2
comp = 2
g = 0
tau = 2
```

The support decomposes as a disjoint union:

```text
A = A_1 disjoint union A_2
|A_1| = |A_2| = 4
```

where each `A_i` is a rank-one complete-stride component. The local endpoint
space decomposes as:

```text
U_A = U_1 direct sum U_2
dim U_1 = dim U_2 = 1
dim U_A = delta = 2
```

Each component generator is assumed to be exact on its stride component: all
four coordinates in `A_i` are nonzero, and it has no support outside `A_i` for
this exact local row.

## Root Assumptions

Use the determinant-1 RFC fold with independent roots:

```text
T_j uniform in F_q^*
```

The fold equations are evaluated under the nonzero-root law. Thus a single
root equation has probability `1/(q-1)` when the required value is nonzero. The
complete-stride component generators have nonzero coordinates, so the target
root values in this product row are nonzero.

This note does not use the `T'=-T` algebra.

## Component Count

For one rank-one component `A_i`, the diagonal endpoint compatibility condition
is that the induced diagonal multiplier preserve the line `U_i`.

Since `U_i` is rank one with nonzero coordinates on all four positions, this
means the multiplier is component-wise constant after normalizing by the fixed
component pattern. Equivalently, the admissible root assignments on `A_i` have
the form:

```text
(T_j)_{j in A_i} = s_i * beta_i
```

for one scalar `s_i in F_q^*` and a fixed nonzero pattern `beta_i`.

Therefore:

```text
# admissible assignments on A_i = q - 1
# total assignments on A_i      = (q - 1)^4
Pr(component i endpoint)       = (q - 1)^(-3)
```

The two components are disjoint and the roots are independent, so the product
row has:

```text
# admissible assignments on A = (q - 1)^2
# total assignments on A      = (q - 1)^8
Pr(product endpoint)          = (q - 1)^(-6)
```

For each admissible product assignment, the `tau=2` visible plane is forced:

```text
V = U_1 direct sum U_2
```

so there is no additional Grassmannian factor. In the q-exponent convention used
by the local theorem:

```text
E_A(2) <= q^2
E_A(2) * q^(-8) <= q^(-6)
```

With the exact nonzero-root probability, the log-q value is:

```text
observed_endpoint_logq
  <= log_q((q - 1)^(-6))
   = -6 + 6 log_q(q/(q - 1)).
```

Thus the dimension exponent is `-6`; the displayed extra term is only the
finite nonzero-root normalization constant.

## Comparison To Endpoint Bound

The corrected tau-2 endpoint theorem gives:

```text
endpoint_generic_logq   = 2*g - 4
                        = -4

endpoint_component_logq = comp + 2*delta - 4 - |A|
                        = 2 + 4 - 4 - 8
                        = -6

endpoint_bound_logq     = max(endpoint_generic_logq,
                              endpoint_component_logq)
                        = -4
```

The product structure gives the stronger component value:

```text
observed_endpoint_logq <= -6
```

in q-dimension accounting, or:

```text
observed_endpoint_logq
  <= -6 + 6 log_q(q/(q - 1))
```

under exact `T uniform in F_q^*` probability accounting.

Therefore:

```text
endpoint_excess_logq
  = observed_endpoint_logq - endpoint_bound_logq
  <= -2
```

in q-dimension accounting, and:

```text
endpoint_excess_logq
  <= -2 + 6 log_q(q/(q - 1))
```

with exact nonzero-root normalization. For production `q=2^128`, this is still
essentially `-2` q-dimensions of slack.

## Exact Support

This derivation is for exact support:

```text
zero/requested endpoint support = A_1 disjoint union A_2
```

Both rank-one components must be present. A certificate that uses only one
component, collapses a component to a smaller support, or reuses a contained
support is not an event for this exact `a=8, delta=2, comp=2` row.

If an implementation starts from contained-support counts, it must perform
exact-support inversion before comparing to the formula above. Any excess that
comes from one-component or smaller-support certificates is classified as
contained-support overcount.

## Proof Consequence

The product-style row is not a tau-2 endpoint obstruction. Its exact product
count lands at the component exponent `-6`, two q-dimensions below the safe
theorem bound `-4`.

If an endpoint counter reports a value near `-4` for this row, the row is not
being counted as the exact product endpoint above. The likely explanations are:

- contained-support certificates were counted as exact events;
- marked complete-stride labels were counted instead of unmarked supports;
- active-copy labels were counted as events without changing the canonical
  endpoint plane;
- the support is hiding a connected or generic `g > 0` subprofile;
- the root distribution or fold algebra does not match determinant-1 RFC with
  independent nonzero roots.

Only after those explanations are ruled out would a positive
`endpoint_excess_logq` force a local theorem repair or an `e=72` evaluation.
