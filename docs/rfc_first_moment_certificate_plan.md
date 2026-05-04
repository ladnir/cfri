# RFC First-Moment Certificate Plan

This note is the proposed replacement for the layer-by-layer threshold certificate. The goal is to
prove distance by bounding the expected number of final low-weight codewords directly.

## Final Event

For the systematic affine RFC at final layer `d`, total codeword weight is:

```text
wt(C_d(m)) = wt(m) + wt(P_d(m)).
```

For a target distance `D`, a message of support `s` is bad when:

```text
wt(P_d(m)) <= D - s.
```

Equivalently, with parity length `n_d`, it has at least:

```text
z = n_d - (D-s)
```

parity zeros.

For a fixed support size `s`, define the zero-subset first moment:

```text
K_d(s,z) =
  sum_{Z subset [n_d], |Z|=z}
    E[ # exact-support-s messages m with P_d(m)[Z] = 0 ].
```

Then:

```text
E[# bad final messages]
  <= sum_{s=1}^{k_d} K_d(s, n_d - (D-s)).
```

This is the first-moment certificate target. If the right-hand side is below `2^-lambda`, then the
distance is at least `D+1` except with probability at most `2^-lambda`.

This proof shape only union-bounds the final event. It does not require every intermediate layer to
be good.

## Why Zero-Set Shape Matters

RFC coordinates are recursively paired, so a zero set is not described only by its size. At a parent
layer, each child coordinate pair can request:

```text
none:       neither parent coordinate is required zero
left:       only L + T D is required zero
right:      only L + (T+1)D is required zero
both:       both parent coordinates are required zero
```

Thus a scalable recurrence should aggregate zero sets by a recursive shape, or by a compressed
summary that upper-bounds all shapes with the same summary.

For a single parent pair:

```text
Y_0 = L + T D
Y_1 = L + (T+1)D
```

with `D = R-L` and `T uniform in F^*`.

The local requested-zero transition is:

```text
request none:
  no condition

request both:
  requires L = 0 and D = 0

request left or right:
  if D = 0, requires L = 0
  if D != 0, holds for exactly one T value
```

For uniform nonzero `T`, a one-sided requested zero with `D != 0` has probability at most
`1/(|F|-1)`. The left and right roots differ by `1`, so requesting both zeros cannot be satisfied
when `D != 0`.

This table is cleaner than the threshold proof's bad-root tail because it tracks requested zeros,
not accidental extra zeros.

## Needed State

Let a child zero-shape pair be `(A,B)`, where:

```text
A = coordinates where L = P_i(l) is required zero
B = coordinates where R = P_i(r) is required zero
```

For parent `both` requests, the child state needs both `L=0` and `R=0`, i.e. membership in
`A cap B`.

For one-sided requests, the condition splits into:

```text
D = 0 and L = 0, i.e. L = R = 0
D != 0 and one root hit
```

Therefore the exact recurrence wants a pair-message enumerator:

```text
E_i(u,v; shape) =
  expected number of ordered child pairs (l,r)
  with wt(l)=u, wt(r)=v
  and a specified pattern of relations between P_i(l) and P_i(r).
```

The one-step sampler currently measures this object empirically by the category counts:

```text
parent_support, equal_nonzero, single_root, double_root.
```

For a proof, we need to compute or upper-bound it recursively.

## Candidate Compressed State

The low-tail category decompositions suggest the dangerous states have very small
`equal_nonzero`; most low-weight mass comes from states where many active child coordinates are
root-capable:

```text
equal_nonzero = 0
single_root + double_root near the target parity weight
```

So a possible compressed state is:

```text
support s
requested zero count z
forced-common-zero count a
root-capable count r
forced-nonzero count f
```

with local transition probabilities:

```text
one-sided roots:  (1/(q-1))^t
generic roots:    (2/(q-1))^t
forced zeros:     child common-zero kernel terms
```

This is not yet a proof. It is the shape to test against the one-step sampler:

1. Compute the compressed upper-bound contribution for each sampled child category state.
2. Check how much slack it introduces in the GF(5) depth-3 one-step artifact.
3. If the slack is small, derive the recursive version.

## Baseline Requirement

The same first-moment machinery must be run for:

```text
1. original non-systematic RFC at total expansion c
2. systematic affine RFC with parity expansion c-1
```

Only then is the systematic/non-systematic gap meaningful.

## Immediate Next Tasks

1. Add a postprocessor for `sample_one_step_categories_*` that evaluates candidate compressed
   upper bounds against the exact one-step low-tail contribution.
2. Keep parent support in the state; the depth-3 data shows support is where the systematic and
   original low tails differ.
3. Derive a recursive upper-bound version of `K_i(s,z)` using zero-set shapes, then compare it to
   the sampled one-step spectrum before scaling to `q=2^128`.

## Projection Test: GF(5), Depth 3

The postprocessor:

```text
python scripts/analyze_one_step_categories.py \
  docs/sample_one_step_categories_gf5_depth3_c8_cpp.csv \
  --prime 5 \
  --old-cutoff 16 \
  --systematic-cutoff 20 \
  --out docs/sample_one_step_projection_analysis_gf5_depth3_c8_cpp.csv
```

computes how much low-tail mass is captured by several projected states. For the sampled one-step
artifact:

```text
original exact low-tail contribution:    1.02263537182
systematic exact low-tail contribution:  1.69857804133
```

Projection by parent support alone gives:

```text
original:
  support 8: 90.0160%
  support 4:  9.9531%

systematic:
  support 4: 71.8379%
  support 8: 18.6657%
  support 2:  9.4950%
```

Projection by `(support, active)` where:

```text
active = equal_nonzero + single_root + double_root
```

already isolates the leading low-tail bands:

```text
original:
  support 8, active 11: 29.3360%
  support 8, active 12: 25.0215%
  support 8, active 10: 19.4351%

systematic:
  support 4, active 10: 22.7399%
  support 4, active  9: 13.1811%
  support 4, active 12: 12.8232%
```

In the top low-tail projected states, `equal_nonzero=0`. This suggests a compressed state based on
`(support, root_capable_count)` may be a good first upper-bound target, with `equal_nonzero` tracked
only as a forced-weight offset.
