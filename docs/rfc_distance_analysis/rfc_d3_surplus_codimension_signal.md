# RFC D=3 Surplus Codimension Signal

Status: small-field signal for the surplus repair codimension target.

This note tests the revised local target:

```text
Pr[root-line repair rank < 2D] <= poly(D,t) q^{-(t-2D+1)}
```

in the first nontrivial case:

```text
D = 3,
target exponent = t - 5.
```

The purpose is not exact probability classification. The earlier D=3 geometry probe already showed
that exact probabilities can split under side-colored matroid signatures over `GF(5)`. Here we only
ask whether the failure probability is within a moderate finite constant of `q^{-(t-5)}` for
Hall-OK examples.

## Script

```text
scripts/rfc_distance_analysis/rfc_d3_surplus_codim_probe.py
```

For sampled projective configurations in `P^2(F_q)`, the script:

1. filters to Hall-OK examples:

   ```text
   rank(T)=3
   min_A(|T|-|A|+2rank(A)) >= 6
   ```

2. exactly enumerates RFC nonzero-root assignments;
3. reports:

   ```text
   observed codimension = -log_q(failure probability)
   ratio to q^{-(t-5)}
   ```

## Results

### GF(5), t=7

Command:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_surplus_codim_probe.py \
  --q 5 --rows 7 --sides alternating --max-configs 40 --seed 7 --print-limit 12
```

Summary:

```text
target codim = 2
checked = 40
Hall-OK = 40
worst ratio to q^-2 = 55375/16384 ~= 3.38
worst observed codim = 1.243324
```

This is not clean at small field size: the apparent exponent is below `2`, but only by a moderate
constant factor when interpreted as a `q^-2` bound.

### GF(5), t=8

Command:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_surplus_codim_probe.py \
  --q 5 --rows 8 --sides alternating --max-configs 24 --seed 8 --print-limit 12
```

Summary:

```text
target codim = 3
checked = 24
Hall-OK = 24
worst ratio to q^-3 = 522875/65536 ~= 7.98
worst observed codim = 1.709647
```

The worst small-field constant is larger for `t=8`, but most sampled rows have observed codimension
around `2.05` to `2.37`. This is still not a clean finite-field theorem, but it does not look like
a catastrophic exponent collapse.

### GF(7), t=7

Command:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_surplus_codim_probe.py \
  --q 7 --rows 7 --sides alternating --max-configs 10 --seed 7 --print-limit 10
```

Summary:

```text
target codim = 2
checked = 10
Hall-OK = 10
worst ratio to q^-2 = 750337/279936 ~= 2.68
worst observed codim = 1.493316
```

The ratio improves from the `GF(5),t=7` run, which is consistent with finite constants rather than
a missing q-exponent.

### GF(7), t=8

Command:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_surplus_codim_probe.py \
  --q 7 --rows 8 --sides alternating --max-configs 2 --seed 8 --print-limit 2
```

Summary:

```text
target codim = 3
checked = 2
Hall-OK = 2
worst ratio to q^-3 = 1759247/839808 ~= 2.10
worst observed codim = 2.619989
```

This is the most encouraging small signal: with a slightly larger field and one more singleton row,
the observed exponent is close to the `t-5` target.

## Interpretation

The D=3 surplus target is not falsified.

The small-field checks show finite constants are real, especially over `GF(5)`, but the `GF(7)`
checks move toward the target codimension:

```text
t=7 target 2: worst observed 1.49 over GF(7)
t=8 target 3: worst observed 2.62 over GF(7)
```

This supports the revised proof direction:

```text
stop classifying exact D=3 probabilities;
prove a codimension/layer bound with explicit polynomial constants.
```

## Next Proof Target

A promising incidence proof is:

1. A bad root assignment means there exists a nonzero pair:

   ```text
   (x,y) in K_P plus K_P
   ```

   such that:

   ```text
   ell_i(x) + alpha_i ell_i(y) = 0
   ```

   for all singleton rows.

2. Stratify by the rank of the two-column restriction:

   ```text
   i -> (ell_i(x), ell_i(y)).
   ```

3. Rows where both entries vanish free an alpha variable, but force `(x,y)` into a lower-dimensional
   incidence stratum.

4. Hall should ensure each stratum has total dimension at most:

   ```text
   t - (t-2D+1)
   ```

   up to polynomial constants.

This is now the main local proof problem.
