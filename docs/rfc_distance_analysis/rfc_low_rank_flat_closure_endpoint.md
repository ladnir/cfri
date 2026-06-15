# Low-Rank Flat Closure Endpoint

Status: refinement of the flat-excess endpoint `r=0`.

## Setup

In the fixed-survivor repair step, let:

```text
P = paired child positions
A = flat-excess witness positions
K_P = ker(ev_P)
D = dim K_P
```

The low-rank endpoint is:

```text
r(A) = rank(A modulo P) = 0.
```

Equivalently:

```text
ev_A|K_P = 0.
```

So every child codeword that vanishes on `P` also vanishes on `A`.

## Closure Interpretation

The condition `rank(A modulo P)=0` is exactly:

```text
A subset cl(P)
```

in the child code matroid. In matrix terms:

```text
rank(P union A) = rank(P).
```

This is a marked incremental event, not an unmarked rank-tail event. It says the marked extra
coordinates lie in the closure of the paired core. It does **not** merely say `P union A` has some
rank defect.

For `|A|=F`, the random matrix exponent conditioned on `rank(P)=p` is:

```text
F * (k_child - p)
```

or, in the top profile with `D=k_child-p`,

```text
F D.
```

At the target profile:

```text
k_child = 1024
p = 137
D = 887
```

this is:

```text
F * 887.
```

Examples from:

```text
scripts/rfc_distance_analysis/rfc_incremental_flat_rank_scale.py
```

are:

```text
F=1:
  log2 first moment ~= -112523.330980

F=72:
  log2 first moment ~= -8173003.290581
```

## Why Aggregate B_d(z,s) Is The Wrong Tool

The aggregate short-set recurrence for:

```text
z = |P|+F = 138
s = F = 1
```

still reports:

```text
B_10(138,1) ~= +828.343072 bits
```

with dominant trace:

```text
depth 10: child witness p=1,t=136,a=45 -> child z=46,s=23.
```

This is a hidden internal rank defect inside the unmarked union `P union A`. It does not imply:

```text
A subset cl(P).
```

So the aggregate recurrence overcounts the endpoint by counting the wrong event. This is exactly
why the marked incremental formulation is necessary.

## One-Step Closure Split

For a top fold, the marked closure event `A <= cl(P)` splits by local categories:

```text
A0:
  marked A is a singleton whose sibling is not in P.

PA:
  marked A is the complementary sibling of a P coordinate.
```

For one marked coordinate:

```text
|A|=1.
```

The `A0` case should cost about:

```text
q^{-D}
```

after conditioning on `P`.

The `PA` case is cheaper but still deterministic-chargeable. The PA mixed projection lemma says:

```text
if the complementary A sibling lies in span(P),
then the child column h_j lies in the projection span of the other P columns.
```

For the dominant top profile this gives a child closure exponent at least:

```text
k_grandchild - (p-1) = 512 - 136 = 376
```

q-dimensions, before recursive paired losses.

The existing scale helper:

```text
scripts/rfc_distance_analysis/rfc_marked_one_a_profile_scale.py
```

with:

```text
depth=10, expansion=8, p=137, q=2^128
```

reports:

```text
A0:
  q_exponent = 887
  log2_moment = -112523.355314

PA_crude:
  q_exponent = 376
  log2_moment = -47121.232771
```

Thus even the mixed endpoint is far from free; it routes to a lower-level closure event.

## Proposed Closure Tail Theorem

Define a marked closure first moment:

```text
C_d(p,a)
  = E[# {(P,A): |P|=p, |A|=a, A subset cl(P)}].
```

More generally allow a core defect:

```text
rank(P)=p-u.
```

The target theorem should bound `C_d(p,a)` by a shape-sensitive recurrence:

```text
C_d(p,a)
  <= A0_terms paying q^{-aD}
     + PA_terms routing to C_{d-1}(p',a')
     + PP/all-paired recursive compression terms
```

where `D = k_d - rank(P)`.

For the flat-excess endpoint, only the regime:

```text
p << k_d,
a = F small or moderate,
D large
```

is needed.

## Current Verdict

The low-rank endpoint is not solved by the aggregate short-set recurrence, but it has a cleaner
marked interpretation:

```text
A lies in the child matroid closure of P.
```

This is likely easier than the full marked incremental rank-tail theorem. The next proof target is
therefore:

```text
closure-tail recurrence C_d(p,a),
starting with a=1 and the A0/PA split.
```

If `C_d(137,1)` can be bounded with even the PA projection exponent scale, then the `r=0,F=1`
endpoint is safe; larger `F` should multiply the closure cost or recurse through several marked
coordinates.
