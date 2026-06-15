# RFC D=3 Root-Line Geometry Probe

Status: first falsification probe after the D=2 PGL2 envelope.

The fixed-survivor rank-tail route now has a theorem-shaped `D=2` envelope. The next risk is
whether `D=3` immediately requires full projective configuration data, which would likely make the
global recurrence too large.

This note records the first small-field probe.

## Script

```text
scripts/rfc_distance_analysis/rfc_d3_rootline_geometry_selftest.py
```

The script works directly with projective directions:

```text
ell_i in P^2(F_q) = P(K_P^*)
```

and exact root-line rows:

```text
(ell_i, alpha_i ell_i) in K_P^* plus K_P^*.
```

It computes:

```text
Pr[rank < 6]
```

by exact enumeration over root assignments.

It can group configurations by:

```text
rank-histogram      coarse counts of subset ranks,
canonical-matroid   canonical uncolored rank table,
colored-matroid     canonical rank table with left/right side colors.
```

The colored signature is the fair analogue of the D=2 parallel-class side counts: side placement is
part of the local object.

## Checks Run

All-left exhaustive `GF(3)`:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_rootline_geometry_selftest.py \
  --q 3 --rows 6 --sides left --print-limit 20
```

Result:

```text
configs=1716 groups=4 ambiguous_groups=0
```

Mixed-side coarse rank-histogram exhaustive `GF(3)`:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_rootline_geometry_selftest.py \
  --q 3 --rows 6 --sides alternating --print-limit 20
```

Result:

```text
configs=1716 groups=4 ambiguous_groups=4
```

This says the coarse rank histogram is definitely too weak once side labels mix.

Mixed-side colored-matroid sampled `GF(3)`:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_rootline_geometry_selftest.py \
  --q 3 --rows 6 --sides alternating \
  --signature colored-matroid --max-configs 200 --seed 3 --print-limit 20
```

Result:

```text
configs=200 groups=16 ambiguous_groups=0
```

Mixed-side colored-matroid sampled `GF(5)`:

```text
python -B scripts/rfc_distance_analysis/rfc_d3_rootline_geometry_selftest.py \
  --q 5 --rows 6 --sides alternating \
  --signature colored-matroid --max-configs 80 --seed 5 --print-limit 20
```

Result:

```text
configs=80 groups=26 ambiguous_groups=7
```

Example split values include:

```text
513/2048, 1027/4096, 257/1024
287/1024, 1149/4096
1159/4096, 145/512
```

## Interpretation

This is not a proof, and the `GF(5)` result is a real warning.

What failed:

```text
D=3 cannot be compressed to a coarse rank histogram under mixed side domains.
```

What survived the first probe:

```text
side-colored matroid data may still be enough, at least in small GF(3) samples.
```

What failed in the second probe:

```text
side-colored matroid data is not enough to predict exact D=3 probabilities over GF(5).
```

So exact D=3 table compression is probably the wrong target. This does not immediately kill the
fixed-survivor route, because exact probabilities are stronger than the certificate needs. The next
candidate is the determinant-polynomial envelope in:

```text
rfc_rootline_schwartz_zippel_envelope.md
```

## Updated Pressure Point

The next theorem target is no longer exact D=3 table compression. It is:

```text
prove Hall/generic full rank for the root-line system,
then apply a D/(q-1) Schwartz-Zippel failure bound.
```

If the global recurrence cannot tolerate this one-q-factor repair bound, the fixed-survivor route
should be downgraded. If it can, D=3 exact geometry leakage becomes annoying but not fatal.
