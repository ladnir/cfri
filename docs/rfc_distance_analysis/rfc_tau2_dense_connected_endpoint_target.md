# RFC Tau-2 Dense Connected Endpoint Target

Scope: original non-systematic RFC only. This note selects the next endpoint
target after the product-style `a=8, delta=2, comp=2, g=0` row is resolved.

## Endpoint Invariant

Use the corrected tau-2 generic parameter:

```text
g(A) = 2*delta(A)
     - min_{B subset A} (|A| - |B| + 2*rank_A(B)).
```

The endpoint theorem compares the exact endpoint count to:

```text
endpoint_bound_logq(A)
  = max(2*g(A)-4,
        comp(A)+2*delta(A)-4-|A|).
```

The dense connected obstruction starts when:

```text
comp = 1
g >= 2
```

because then the generic term `2*g-4` can dominate the component/full-rank
term.

## Delta-2 Feasibility

The hoped-for smallest row would be:

```text
comp = 1
delta = 2
g >= 2
```

For an exact support in the RFC local matroid, this is not feasible unless the
support contains loops or collapses to rank one, both of which violate the
intended exact connected `delta=2` endpoint.

Reason: with `delta=2`,

```text
g(A) = 4 - min_{B subset A} (a - |B| + 2*rank_A(B)).
```

To have `g >= 2`, one would need:

```text
a - |B| + 2*rank_A(B) <= 2
```

for some `B subset A`.

For an exact no-loop support:

- if `B` is nonempty, then `rank_A(B) >= 1`;
- if `B=A`, then `rank_A(B)=delta=2`;
- if `B` is a proper rank-one subset, then
  `a-|B|+2*rank_A(B) >= 1+2 = 3`;
- if `B` has rank two, then the expression is at least `4`.

Thus the minimum is at least `3`, and:

```text
g <= 1
```

for every exact connected `delta=2` no-loop support. The row
`comp=1, delta=2, g>=2` should therefore not be used as the dense connected
target. If implementation reports such a row, it should first classify it as
one of:

- loop or zero-coordinate degeneration;
- contained-support overcount;
- incorrect `delta` computation;
- incorrect `g` computation;
- support not actually exact;
- support not actually `delta=2`.

## Smallest Delta-2 Connected Sanity Row

The smallest connected `delta=2` exact support is:

```text
a = 3
delta = 2
comp = 1
g = 1
```

For example, a rank-2 connected three-element restriction has:

```text
min_B (a-|B|+2rank(B)) = 3
g = 4 - 3 = 1
```

Its endpoint bound is:

```text
endpoint_generic_logq   = 2*g - 4 = -2
endpoint_component_logq = comp + 2*delta - 4 - a
                        = 1 + 4 - 4 - 3
                        = -2
endpoint_bound_logq     = -2
```

This is a useful sanity target for the endpoint counter, but it is not the
`g>=2` dense obstruction.

## Next Feasible Dense Connected Target

The first matroid-level dense connected target with `g>=2` is:

```text
a = 4
delta = 3
comp = 1
g = 2
```

This is the rank-3 four-coordinate connected restriction, e.g. the local
rank-3 hyperplane/circuit-style target used in the tau-2 weighted exterior
notes.

Expected rank data:

```text
rank_A(A) = 3
rank_A(empty) = 0
rank_A(singleton) = 1
rank_A(pair) = 2       # for the uniform dense model
rank_A(triple) = 3
```

Then:

```text
min_B (a-|B|+2rank_A(B)) = 4
g = 2*3 - 4 = 2
```

and the endpoint bound is:

```text
endpoint_generic_logq   = 2*g - 4
                        = 0

endpoint_component_logq = comp + 2*delta - 4 - a
                        = 1 + 6 - 4 - 4
                        = -1

endpoint_bound_logq     = max(0, -1)
                        = 0
```

This is the smallest genuine generic-endpoint target currently visible from the
local theorem. Implementation still has to confirm that the RFC local matroid
realizes this exact restriction. If the RFC local matroid forbids this `a=4`
restriction at the tested depth, the next target should be the smallest exact
connected support with:

```text
delta >= 3
g >= 2
comp = 1
```

ranked first by support size `a`, then by largest positive
`observed_endpoint_logq - endpoint_bound_logq`.

## Exact-Support Requirements

The dense connected target must be counted with exact support:

```text
zero/requested endpoint support = A
```

The counter must reject or separately classify:

- contained-support witnesses from `B superset A`;
- rows where some coordinate in `A` is actually inactive;
- product decompositions with `comp > 1`;
- marked labels that collapse to the same unmarked support;
- duplicate bases for the same visible two-plane;
- root assignments outside the determinant-1, independent nonzero RFC law.

Exact-support inversion must happen before comparing to `endpoint_bound_logq`.

## Meaning Of Observed Endpoint Count

For this target:

```text
observed_endpoint_logq
```

means the q-log of the exact, canonical tau-2 endpoint contribution after the
root challenge factor. It should count visible two-plane endpoint events, not
certificates.

For the `a=4, delta=3, comp=1, g=2` target:

```text
endpoint_bound_logq = 0
endpoint_excess_logq = observed_endpoint_logq
```

So:

- `observed_endpoint_logq <= 0` supports the corrected local theorem at this
  smallest dense connected gate;
- `observed_endpoint_logq > 0` is a direct local endpoint-theorem blocker,
  before paired-spine Lift accounting;
- a positive value that survives exact-support inversion and canonical
  de-duplication forces a local theorem repair before any `e=71` certificate can
  rely on this gate.

Only after the local theorem repair fails, or after production scaling shows the
remaining positive contribution consumes the `e=71` slack, should this target
force `e=72` evaluation.

