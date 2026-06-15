# RFC Exterior Component Codimension

This note records the component/full-rank endpoint for the `r=2` local calculation.

Important correction: the earlier component-only conjecture is not the full `tau=2` theorem.
Dense restrictions also have a generic two-copy matroid-union endpoint. The corrected local bound
is stated in:

```text
docs/rfc_distance_analysis/rfc_tau2_weighted_exterior_bound.md
```

## Setup

Let `U <= F^S` be the child image space restricted to a selected singleton set `S`. Write:

```text
rho = dim U
M   = represented matroid of U on S
c   = number of connected components of M
```

For two replicas, root compatibility at every coordinate in `S` requires:

```text
x_j w_j - y_j z_j = 0  for all j in S,
```

with `x,y,z,w in U`.

Define the exterior variety:

```text
V_2(U) = { (x,y,z,w) in U^4 : x_j w_j - y_j z_j = 0 for all j in S }.
```

## Component Endpoint

The component endpoint is:

```text
full-rank root-line assignments have dimension c,
ordered-basis contribution has exponent 2rho + c.
```

This refines the earlier naive guess for the high-kernel layer.

Examples:

```text
free set of s coordinates:
  rho=s, c=s
  codim = s

rank-1 parallel class:
  rho=1, c=1
  codim = 1

connected full-rank endpoint:
  c=1
  exponent = 2rho + 1

direct sum of components M_i:
  endpoint exponents add as sum_i (2 rho_i + 1) = 2rho + c
```

For exact-support `tau=2`, this endpoint contributes the root-weight exponent:

```text
2rho + c - 4 - |S|.
```

The generic endpoint can be larger. If `g` is the generic two-copy kernel dimension, the corrected
root-weight exponent is:

```text
max(2g - 4, 2rho + c - 4 - |S|).
```

So this note should be read as one endpoint, not the whole local theorem.

## Proof Skeleton

There is a natural lower-dimensional algebra behind the formula.

Let:

```text
End_diag(U) = { lambda in F^S : lambda * u in U for every u in U },
```

where `*` is coordinatewise product. For a represented matroid, the diagonal endomorphism algebra
has dimension equal to the number of connected components:

```text
dim End_diag(U) = c.
```

If `lambda in End_diag(U)` and `x,y in U`, then:

```text
z = lambda * x
w = lambda * y
```

gives a solution of all exterior equations. This gives a family of dimension:

```text
2rho + c.
```

The component endpoint proof should show that line assignments with `dim K(ell)=rho` are exactly
component-wise diagonal endomorphisms. It should not claim that every irreducible component of the
full exterior variety has dimension at most `2rho+c`; that statement fails for dense restrictions.

## Full-Rank Proof Target

The full-rank endpoint should be proved by induction on the represented matroid `M|S`.

First reduce to connected components. If:

```text
S = S_1 disjoint union ... disjoint union S_c
```

is the matroid component decomposition, then rank is additive and the row space decomposes over the
coordinate direct sum:

```text
U = U_1 direct sum ... direct sum U_c.
```

The full-rank line-assignment family is a product:

```text
End_diag(U) = product_i End_diag(U_i).
```

So it is enough to prove the connected case:

```text
dim End_diag(U) = 1.
```

For connected `U`, this is the standard diagonal-endomorphism lemma. If:

```text
lambda * u in U for every u in U,
```

then `lambda` is scalar on the connected matroid component. Therefore full-rank root-line kernels
are exactly component-wise constant line assignments:

```text
# {ell : dim K(ell)=rho} ~= q^c.
```

The harder, still-open local theorem is the intermediate rank-drop endpoint:

```text
dim {ell : dim K(ell) >= h}
```

between the generic kernel dimension `g` and the full-rank layer `rho`.

## Evidence

The diagnostic script is:

```text
scripts/rfc_distance_analysis/rfc_exterior_constraint_profile.py
```

It computes the exact finite-field count by enumerating `(x,y) in U^2`; for each `(x,y)`, the
constraints on `(z,w)` are linear.

Exact `GF(5)`, child depth `1`, expansion `4`, selected size `3`:

```text
python scripts/rfc_distance_analysis/rfc_exterior_constraint_profile.py \
  --prime 5 \
  --child-depth 1 \
  --expansion 4 \
  --size 3 \
  --seed 11
```

Output summary:

```text
rho components predicted_codim shapes min_codim  avg_codim  max_codim
1   1          1               2      0.907781   0.907781   0.907781
2   1          3               24     2.533119   2.533119   2.533119
2   2          2               30     1.815563   1.815563   1.815563
```

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_exterior_exact_gf5_child_depth1_c4_size3_summary.csv
docs/rfc_distance_analysis/rfc_exterior_exact_gf5_child_depth1_c4_size3_subsets.csv
docs/rfc_distance_analysis/rfc_exterior_exact_gf7_child_depth1_c4_size3_summary.csv
docs/rfc_distance_analysis/rfc_exterior_exact_gf11_child_depth1_c4_size3_summary.csv
```

Sampled `GF(5)`, child depth `2`, expansion `4`, selected size `4`, `20` sampled subsets:

```text
rho components predicted_codim shapes min_codim  avg_codim  max_codim
2   1          3               2      2.533119   2.533119   2.533119
3   1          5               2      3.599157   3.599157   3.599157
3   2          4               7      3.440900   3.440900   3.440900
3   3          3               4      2.723344   2.723344   2.723344
4   4          4               5      3.631126   3.631126   3.631126
```

Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_exterior_sample_gf5_child_depth2_c4_size4_summary.csv
docs/rfc_distance_analysis/rfc_exterior_sample_gf5_child_depth2_c4_size4_subsets.csv
```

For the size-3 exact checks the generic kernel has dimension `<2`, so the component endpoint is the
active endpoint. The finite `GF(5)` codimensions are below the integer asymptotic predictions
because constant factors are visible at tiny field size, and the structural grouping is by
`(rho, c)`.

For size-4 sampled checks, dense rank-3 supports already show the generic endpoint. For example,
`rho=3, components=1` has predicted component codimension `5`, but the observed finite-field
codimension is near `4`, matching the generic two-copy kernel endpoint. This is why the full
component-only exterior theorem is not the right local theorem.

The same exact depth-1 check over larger tiny fields moves toward the integer codimensions:

```text
GF(7), child depth 1, expansion 4, selected size 3:
rho components predicted_codim shapes min_codim
1   1          1               5      0.940638
2   1          3               12     2.618220
2   2          2               39     1.881276

GF(11), child depth 1, expansion 4, selected size 3:
rho components predicted_codim shapes min_codim
1   1          1               5      0.966885
2   1          3               12     2.695707
2   2          2               39     1.933770
```

## Consequence

The rank-pattern recurrence should not use a scalar singleton count. For `tau=2`, it needs a state
containing at least:

```text
singleton set size |S|
support-subcode dimension delta(S)
generic two-copy kernel dimension g(S)
component count c(S)
```

The local root-weight exponent should be bounded by:

```text
max(2g(S)-4, 2delta(S)+c(S)-4-|S|).
```

Equivalently, the one-step charge is the negative of this exponent when it is negative; dense
supports with `g(S)>=2` may have zero or positive local exponent and must be paid elsewhere in the
global rank-pattern recurrence.

The first scalar component-aware model is therefore not the right next model. The recurrence needs
the root-line/generic-kernel state together with replica-span state. See:

```text
docs/rfc_distance_analysis/rfc_replica_span_state.md
```
