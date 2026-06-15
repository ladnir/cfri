# RFC Multi-PA Rank-Drop Witness Target

Status: next local proof target for the current `t,t+1` distribution.

## Object

Fix:

```text
U <= H plus H,
J = mixed PA coordinates,
x_j = (h_j,0),
y_j = (0,h_j),
p_j(alpha) = x_j + alpha_j y_j,  alpha_j in F_q^*.
```

The mixed marked increment is:

```text
rank_A(alpha)
  = rank{y_j : j in J} modulo U + span{p_j(alpha) : j in J}.
```

We need to bound:

```text
Pr_alpha[ rank_A(alpha) <= r ].
```

## Witness Equation

A rank drop below full marked rank on a support `S subseteq J` means there is a nonzero vector
`c in F^S` such that:

```text
sum_{j in S} c_j y_j
  in U + span{p_j(alpha) : j in S}.
```

Equivalently, there exist `mu_j` and `(u_1,u_2) in U` satisfying:

```text
sum_{j in S} mu_j h_j = -u_1,
sum_{j in S} (c_j - alpha_j mu_j) h_j = u_2.
```

The one-coordinate PA projection lemma is the special case `|S|=1`.

## Minimal-Support Strategy

Take `S` minimal for a nonzero witness `c`. Then:

1. `c_j != 0` for every `j in S`;
2. no proper subset of `S` supports the same kind of marked dependence;
3. if `mu=0`, then `sum c_j h_j in pi_2(U)`, giving a child projection rank event;
4. if `mu != 0`, then the first equation forces:

```text
sum mu_j h_j in pi_1(U),
```

and the second equation imposes an affine root-line condition on the `alpha_j`.

The desired lemma is:

```text
For every minimal support S, either
  (A) h_S has a projection rank defect relative to pi_1(U)+pi_2(U), or
  (B) the admissible alpha_S lie in a proper algebraic hypersurface
      of degree <= |S|, with codimension at least one.
```

For higher rank drop `g-r`, repeat this with a flag of independent minimal witnesses. The target
codimension is one root equation per independent witness after subtracting child projection defects.

## Why This Might Be Enough

The flat-excess recurrence has huge slack in the small-flat regime. Therefore the multi-PA theorem
does not need a perfect random-matrix exponent. A weaker statement of the form:

```text
each independent mixed PA rank drop costs at least one q-factor
unless it routes to a child projection/marked-rank event
```

may be enough, provided the remaining child event is counted with the marked incremental recurrence.

The small-field graph-contraction profile supports this direction:

```text
GF(5), mixed=3: rank-drop probability about 0.00909
GF(7), mixed=3: rank-drop probability about 0.00199
GF(11), mixed=3: no drops in 100000 sampled root assignments
```

These are much smaller than constant probability and look like finite-field incidence, not a
structural collapse.

## Proof Shape To Try Next

1. Fix support `S` and a candidate `mu` modulo the relation fiber:

```text
sum mu_j h_j in pi_1(U).
```

2. Choose the compatible fiber `F_U(-sum mu_j h_j)` in the linear relation `U <= H x H`.
   The sharper invariant form is recorded in:

```text
rfc_multi_pa_linear_relation_form.md
```

3. Substitute into the second equation. For fixed `c,mu`, the condition is affine-linear in
   `alpha_j` modulo the second-projection fiber:

```text
sum alpha_j mu_j h_j in span{c_j h_j} + pi_2(U).
```

4. If this linear condition is identically true for all `alpha`, then the span of `{mu_j h_j}` is
already contained in the projection target, giving a child projection rank defect.

5. Otherwise it cuts at least one dimension from the root choices.

The nontrivial part is counting the possible `c,mu` without losing the q-factor gained from the
root equation. The likely route is projectivization plus minimal support: quotient the witness
space by scalar and charge its dimension to the child projection rank defect.
