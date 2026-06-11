# RFC Tau-2 Dense Connected Formula Target

Scope: original non-systematic RFC only. This note analyzes the smallest dense
connected tau-2 endpoint profile:

```text
a = 4
delta = 3
comp = 1
g = 2
```

The paired-spine gate currently reports a small-field artifact around:

```text
GF(11): endpoint_bound_logq = 0
        endpoint_excess_logq = 0.1649
```

This is compatible with finite projective/Gaussian constants. It is not, by
itself, evidence that the asymptotic endpoint exponent is wrong for this
`g=2` profile.

Important scope correction: this note only closes the smallest connected
`g=2` profile. It does not prove the whole tau-2 endpoint theorem. In
particular, connected `a=5, delta=3, comp=1, g=1` rows have a genuine
codimension-one `kappa>=2` layer and need the layer-codimension endpoint
statement in `rfc_tau2_weighted_exterior_bound.md`.

## Endpoint Bound

For this row:

```text
endpoint_generic_logq = 2*g - 4
                      = 0

endpoint_component_logq = comp + 2*delta - 4 - a
                        = 1 + 6 - 4 - 4
                        = -1

endpoint_bound_logq = max(0, -1)
                    = 0
```

So the local theorem predicts q-dimension exponent `0` after the root challenge
factor. It does not predict that finite-field normalized counts are at most `1`
with no Gaussian or projective constant.

## Model

The dense connected model is the rank-3 connected four-coordinate restriction:

```text
U_A <= F_q^4
dim U_A = 3
```

with all four coordinates active and the represented matroid connected. The
uniform rank-3-on-4 model is the intended minimal target.

For tau-2 endpoint counting, use projective line assignments:

```text
ell in (P^1(F_q))^A
# (P^1(F_q))^A = (q+1)^4
```

For each assignment `ell`, let `kappa_A(ell)` be the endpoint kernel dimension.
The endpoint mass before exact-support inversion is:

```text
E_A(2) = sum_ell GaussianBinomial(kappa_A(ell), 2)_q.
```

For this dense connected profile, the generic assignment has:

```text
kappa_A(ell) = 2
GaussianBinomial(2,2)_q = 1
```

Thus the generic stratum alone contributes:

```text
(q+1)^4
```

not exactly `q^4`. After the theorem's q-exponent normalization this is:

```text
(q+1)^4 / q^4 = (1 + q^-1)^4.
```

At `q=11`, this projective factor already contributes:

```text
log_11((12/11)^4) ~= 0.145
```

which explains most of the observed `0.1649`.

## Exceptional Full-Kernel Stratum

The only endpoint stratum that can add more than the generic contribution is:

```text
kappa_A(ell) = 3.
```

Then:

```text
GaussianBinomial(3,2)_q = q^2 + q + 1.
```

For a connected rank-3 restriction, the full-kernel assignments are controlled
by the connected diagonal-endomorphism condition. They form at most one
projective scalar family:

```text
N_3(q) <= q + 1.
```

Since these assignments were already counted once in the generic
`(q+1)^4` term, their extra contribution is bounded by:

```text
N_3(q) * (q^2 + q)
  <= (q+1)(q^2+q).
```

Therefore the non-enumerative finite-field upper bound is:

```text
E_A(2) / q^4
  <= ((q+1)^4 + (q+1)(q^2+q)) / q^4

  = (1 + q^-1)^4 + q^-1(1+q^-1)^2.
```

This tends to `1`, so:

```text
observed_endpoint_logq
  <= log_q((1 + q^-1)^4 + q^-1(1+q^-1)^2)
  -> 0
```

as `q -> infinity`.

For `q=11`, this upper bound is about:

```text
log_11(((12)^4 + 12*(121+11)) / 11^4) ~= 0.176
```

so an observed `endpoint_excess_logq = 0.1649` is within the expected finite
projective/Gaussian envelope.

## Exact-Support Adjustment

The formula above is a safe contained endpoint upper bound for the minimal
dense connected model. The proof row needs exact support:

```text
zero/requested endpoint support = A
```

Exact-support inversion can only remove assignments whose true endpoint support
is a proper subset or a larger contained-support certificate. The endpoint
counter should report:

```text
contained_E_A(2)
exact_E_A(2)
contained_to_exact_correction_logq
```

and compare only `exact_E_A(2)` to the theorem row.

If exact-support inversion lowers the count, the asymptotic conclusion remains
the same. If it does not lower this row, the finite excess is still explained by
the projective and exceptional-stratum constants above.

## Normalizations To Track

The endpoint counter should track the finite-field terms separately and compute
a residual exponent after removing them. Use the following formulas for tau-2
endpoint rows.

Let:

```text
q = field_size
a = |A|
```

For tau-2, each coordinate root-line assignment lies in:

```text
P^1(F_q)
```

so the exact number of projective line choices per active coordinate is:

```text
#P^1(F_q) = q + 1.
```

The asymptotic endpoint theorem charges this as `q^a`. Therefore:

```text
projective_line_factor_logq =
  log_q((q+1)^a / q^a)
  = a * log_q(1 + q^-1)
```

For this row:

```text
projective_line_factor_logq = 4*log_q(1 + q^-1).
```

The determinant-1 RFC root law samples each root from `F_q^*`, while the
asymptotic theorem shorthand uses a `q^-a` root factor. If all required root
values on the exact support are nonzero, the probability correction is:

```text
nonzero_root_normalization_logq =
  log_q(q^-a / (q-1)^-a with sign chosen as actual/theorem)
  = log_q((q/(q-1))^a)
  = a * log_q(q/(q-1))
  = -a * log_q(1 - q^-1).
```

If some required root value is zero, the determinant-1 nonzero-root probability
is zero and the row must be classified as:

```text
classification = impossible_nonzero_root_row
observed_endpoint_logq = -inf
```

Do not apply the positive normalization factor to an impossible zero-root row.

Also track exceptional higher-kernel strata separately:

```text
exceptional_kappa3_count = N_3(q)
exceptional_kappa3_extra_logq =
  log_q(1 + N_3(q)*(q^2+q)/(q+1)^4)
```

For the `a=4, delta=3, comp=1, g=2` dense connected row with the connected
diagonal-endomorphism bound `N_3(q) <= q+1`, this can be upper-bounded by:

```text
exceptional_kappa3_extra_logq
  <= log_q(1 + (q+1)*(q^2+q)/(q+1)^4)
   = log_q(1 + q/(q+1)^2).
```

Define the finite-constant allowance:

```text
finite_endpoint_constants_logq =
    projective_line_factor_logq
  + nonzero_root_normalization_logq
  + exceptional_strata_extra_logq
  + gaussian_normalization_extra_logq
```

For rows with no separate exceptional or Gaussian correction, set the missing
terms to `0`.

Then define the implementation-facing residual:

```text
residual_endpoint_excess_logq =
    observed_endpoint_logq
  - endpoint_bound_logq
  - finite_endpoint_constants_logq.
```

The acceptance test for finite-field diagnostics is:

```text
residual_endpoint_excess_logq <= 0
```

not bare `observed_endpoint_logq - endpoint_bound_logq <= 0`.

Required output columns:

```text
field_size
support_size
tau
endpoint_bound_logq
observed_endpoint_logq
projective_line_factor_logq
nonzero_root_normalization_logq
exceptional_strata_extra_logq
gaussian_normalization_extra_logq
finite_endpoint_constants_logq
residual_endpoint_excess_logq
zero_required_root_count
nonzero_required_root_count
classification
```

For ordinary exact tau-2 rows with all requested root values nonzero:

```text
nonzero_required_root_count = a
zero_required_root_count = 0
projective_line_factor_logq = a*log_q(1+q^-1)
nonzero_root_normalization_logq = a*log_q(q/(q-1)).
```

These terms are constants in q-dimension accounting. They are visible over small
fields such as `GF(11)` and negligible at production `q=2^128`, but the
certificate script should reserve an explicit finite-constant bit budget for
them.

## Interpretation

The asymptotic endpoint value for the dense connected row is:

```text
observed_endpoint_logq -> 0.
```

At finite fields, especially `q=11`, the correct acceptance condition is not
literal:

```text
endpoint_excess_logq <= 0.
```

It is:

```text
endpoint_excess_logq
  <= projective/Gaussian/nonzero-root normalization allowance.
```

A value around `0.1649` at `GF(11)` is compatible with this allowance. It says
the local theorem needs an explicit polynomial/constant factor in its finite
statement, not a stronger exponent.

## Failure Signal

This row becomes a true local-theorem blocker only if implementation finds:

```text
exact_E_A(2) / q^4
```

growing like `q^epsilon` for some stable positive `epsilon`, after:

- exact-support inversion;
- canonical de-duplication;
- correct rank data for `a=4, delta=3, comp=1, g=2`;
- determinant-1 RFC roots with independent `T in F_q^*`;
- separation of projective/Gaussian constants from q-exponent growth.

Only then should the proof lane repair the asymptotic endpoint theorem or
evaluate whether the remaining contribution forces `e=72`.
