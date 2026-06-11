# RFC Depth-5 Rank-Pattern Audit

Scope: original non-systematic RFC, finite depth-5 base seal.

Status: current scalar recurrence is a calibration, not yet proof-safe.

## What Was Audited

The depth-5 base-seal contract uses the scalar recurrence:

```text
B_h(r,z)
  <= sum_{p,s,c}
       2^s
       binom(u,p)
       binom(n_child-u, s-c)
       q^{-r(s-c)}
       B_{h-1}(2r,u),

u = p+c.
```

The intended interpretation is:

```text
paired coordinates:
  recurse as common-zero child coordinates;

common-zero singleton coordinates:
  also recurse as common-zero child coordinates;

remaining singleton coordinates:
  each cost r aggregate equations.
```

The top-level `r=1` use is proof-safe: a singleton zero for one parent message has probability at
most `(q-1)^-1` unless the child pair is already common-zero.

The first nontrivial use is the depth-4 child state `B_4(2,u)`. There the recurrence says that a
block of `a=s-c` non-common singleton coordinates costs `q^{-2a}`.

## Concrete Dominant States

For the production seal:

```text
depth = 5,
expansion = 8,
target z = 34.
```

The top level is dominated by `c=0` branches and calls `B_4(2,u)` for `0 <= u <= 17`.

Inside the depth-4 `r=2` state, the best one-step terms for `B_4(2,z)` are:

```text
z   p   s   c   u
0   0   0   0   0
1   0   1   0   0
2   0   2   0   0
3   0   3   0   0
4   0   4   0   0
5   0   5   0   0
6   0   6   0   0
7   0   7   0   0
8   0   8   0   0
9   0   9   0   0
10  0   10  0   0
11  0   11  0   0
12  0   12  0   0
13  0   13  0   0
14  0   14  0   0
15  1   13  0   1
16  1   14  0   1
17  1   15  0   1
```

So the finite base proof is not only the easy `s=2` exterior case. It also needs dense
`tau=2` singleton blocks over a depth-3 child code of dimension `8`, with active singleton sizes
up to `15` after one common-zero coordinate.

## Small Exact Check

The command:

```text
python scripts/rfc_distance_analysis/rfc_exterior_constraint_profile.py \
  --prime 5 \
  --child-depth 3 \
  --expansion 8 \
  --size 2
```

returned:

```text
rho,components,predicted_codim,shapes,min_codim,avg_codim,max_codim
1,1,1,18,0.9077814658,0.9077814658,0.9077814658
2,2,2,1998,1.8155629317,1.8155629317,1.8155629317
```

This is consistent with the `r=2` exterior theorem:

```text
free two-coordinate supports:
  exterior codimension tends to 2;

rank-one accidental supports:
  exterior codimension tends to 1.
```

After the two root averages, the free supports match the scalar `q^{-4}` charge for two
singletons. Rank-one supports would lose one q-dimension, but over `q=2^128` such accidental
projective collisions should be a separate bad-minor event, not a structural blocker.

This validates the smallest `B_4(2,2)` shape conditional on the expected no-collision/minor
genericity event. It does not validate the dense `s=13,14,15` shapes.

## Why The Scalar Recurrence Is Too Compressed

For `r=2`, singleton compatibility is a tau-two root-line problem. Let `A` be the exact visible
singleton support after quotienting by common-zero coordinates, and let:

```text
delta = dim U_A,
comp  = number of connected components of U_A,
g     = generic root-line kernel dimension.
```

The scalar recurrence uses charge:

```text
2|A|.
```

That is correct in the free/generic range. For a uniform connected quotient of rank `delta`, it is
supported by the generic endpoint while:

```text
|A| <= 2 delta - 2.
```

The dense endpoint can be weaker:

```text
|A| = 2 delta - 1:
  first-drop layer can lose one q-dimension;

|A| >= 2 delta:
  component/full-kernel endpoint gives charge about |A| + 2 delta - 1,
  losing |A| - 2 delta + 1 q-dimensions versus 2|A|.
```

In the finite base seal, the relevant quotient can have:

```text
delta = 7 or 8,
|A| = 13,14,15.
```

Those are exactly at or near the dense tau-two boundary. Therefore the current scalar recurrence
cannot be promoted directly into a proof.

## Proof-Safe Replacement State

The next recurrence must be exact-support and visible-dimension aware.

For each singleton block, after fixing paired/common-zero child coordinates:

```text
E = remaining singleton coordinates,
A = coordinates where the visible projection is nonzero,
tau = dim visible image on A.
```

Then:

```text
tau = 0:
  this is not a non-common singleton branch; route it to a larger common-zero set C.

tau = 1:
  root compatibility is automatic after choosing one projective child line.
  The invisible/kernel direction must recurse as an additional child zero/marked-line event.

tau = 2:
  use the weighted exterior/root-line theorem:

    E_A(2) q^-|A| <= poly(a) q^theta_2(A),

  with

    theta_2(A) = max_h (2h - 4 - gamma_h(A)).
```

This is the finite version of the broader flag recurrence. The important correction is that dense
tau-two branches are not allowed to keep the full scalar `2|A|` charge unless the corresponding
kernel/root-line layer is proved to have that codimension.

## Current Assessment

The base-seal route is still promising, but the proof target changed:

```text
old target:
  prove the scalar q^{-r|E|} rank-pattern recurrence.

new target:
  prove a finite exact-support recurrence for the depth-4 r=2 states,
  then rerun the depth-5 seal with tau=0/1/2 local charges.
```

The likely closeable theorem is not "every singleton costs r equations" in the scalar state. It is
"any loss from dense tau-two local layers creates kernel/marked-line child structure that is
charged recursively." That is the same mechanism as the main flag proof, but the finite base seal
only needs it through depth five and only for the table above.

## Diagnostic Rechecks After The Audit

The span-aware endpoint diagnostic:

```text
python scripts/rfc_distance_analysis/rfc_replica_span_moment.py \
  --depth 5 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2-layer \
  --print-window 4 \
  --trace-z 34
```

crosses only at:

```text
crossing_z = 249
crossing_excess = 217
```

This is too pessimistic and confirms the older diagnosis: span alone does not charge invisible
kernel directions as recursive child zero events.

The subspace-span version gives the same crossing:

```text
crossing_z = 249
crossing_excess = 217
```

The two-layer flag checkpoint:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py \
  --depth 5 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2-layer-incidence \
  --flag-bound best \
  --max-visible-tau 2 \
  --prune-to-final-span 1 \
  --print-window 4 \
  --trace-z 34 \
  --report-local-theta -1 \
  --report-limit 8
```

crosses at:

```text
crossing_z = 137
crossing_excess = 105
```

This is still far from the desired `z=34`, but it moves in the right direction and uses the right
child-flag shape:

```text
pi(K) <= pi(W),
z_pi(W) = p + s - a,
z_pi(K) = p + s.
```

The remaining gap is therefore not the local root equation itself. It is the looseness of the
current finite flag checkpoint, especially the child flag bound and shortened-ambient accounting.

### Cover-Lift Diagnostic

The flag checkpoint now has a diagnostic-only cover mode:

```text
--cover-lift-mode tau0|all
```

This tests where parent-lift and quotient-incidence multiplicity is concentrated. It does not give
a certificate, because tau-positive quotient lines/planes are themselves event data.

With only tau-zero lifts covered:

```text
crossing_z = 135
crossing_excess = 103
```

so all-paired/invisible lift multiplicity is not the main source of the `z=137` pessimism.

With all lift multiplicity removed:

```text
crossing_z = 35
crossing_excess = 3
log2 vector moment at z=34 = 27.64399707.
```

The full mixed-mode table is:

```text
none:      z=137
tau0:      z=135
tau1:      z=137
tau2:      z=129
tau0tau1:  z=133
tau0tau2:  z=61
tau1tau2:  z=129
all:       z=35
```

If one incorrectly treated the all-cover run as a projective certificate, the final line/vector
conversion would read:

```text
log2 projective/container moment at z=34
  approx 27.64399707 - 128
  = -100.35600293.
```

This number is anti-conservative: removing all lift multiplicity also removes quotient-line/plane
incidence. The valid lesson is narrower but still useful: the remaining base-seal work is not a
new local root-equation miracle; it is to replace crude lift/local products by a proof-safe
quotient-incidence/fiber count.

## Next Work Items

1. Implement a finite `r=2` exact-support recurrence for the `B_4(2,u)` states with local
   branches `tau=0,1,2`.
2. For the `tau=2` branch, plug in the layer theorem from
   `rfc_tau2_weighted_exterior_bound.md`, not the scalar `q^{-2a}` charge.
3. Track whether the dense boundary states `|A|=13,14,15` over quotient rank `7` or `8` are rescued
   by recursive kernel/marked-line charges.
4. Prove a covering/projectivization lemma for quotient lifts: duplicate extensions after fixing
   the child container and quotient/root datum should be counted once, while the quotient
   line/plane incidence and invisible-fiber dimensions are still charged.
5. Only after that rerun the depth-5 seal and report a certified crossing.
