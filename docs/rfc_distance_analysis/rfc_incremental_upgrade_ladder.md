# Incremental Upgrade Ladder For The Original RFC Proof

Status: crawl-before-run plan and first theorem package.

## Purpose

The full near-MDS certificate is still too large to attack in one gulp. The safer route is an
incremental theorem ladder: prove small upgrades to the original fixed-survivor proof, measure what
each upgrade buys, and stop before a partial theorem turns into another open-ended state machine.

The ladder keeps the original proof skeleton:

```text
fixed survivor set
  -> paired/singleton split
  -> exact paired compression
  -> root-line singleton repair
  -> first moment over profiles.
```

Only the singleton repair term is upgraded.

## Common Setup

At one parent fold, let:

```text
P = child positions whose two parent siblings both survive
T = child positions with exactly one surviving parent sibling
K_P = ker(ev_P)
D = dim K_P
t = |T|
```

The root-line repair incidence theorem gives the local bad-root exponent:

```text
repair_exp(P,T) >= t - 2D + 1 - FE(P,T)
```

where

```text
FE(P,T) = max_{A subset T, r(A)<D} ( |A| - 2r(A) )
r(A)    = rank(P union A) - rank(P).
```

The top target profile for `c=8,k=2048,q=2^128,e=71` is:

```text
p = |P| = 137
t = 1845
D = 887
S = t - 2D + 1 = 72.
```

## Tier 0: No-Flat-Excess Repair

### Statement

If every proper-rank subset of singleton functionals satisfies:

```text
|A| <= 2r(A),
```

then:

```text
FE(P,T) <= 0
```

and the local repair failure probability is bounded by:

```text
poly(D,t) q^{-(t - 2D + 1)}.
```

### Proof

This is immediate from the surplus incidence stratification:

```text
bad-root codim >= min_A (t - |A| - 2D + 2r(A) + 1)
                 = t - 2D + 1 - max_A(|A|-2r(A)).
```

If every proper-rank flat has `|A|-2r(A) <= 0`, the clean surplus exponent follows.

### Value

This recovers the ideal top-profile calculation. It is not enough globally because we do not know
that `FE(P,T)=0` for every relevant profile, but it identifies exactly what has to be charged.

## Tier 1: High-Rank Flat-Excess Endpoint

### Statement

Let `A subset T` be a flat-excess witness and put:

```text
h = D - r(A).
```

If `h=1`, then the event:

```text
r(A) <= D-1
```

is exactly the ordinary child line-zero event:

```text
exists 0 != w in K_P with ev_A(w)=0,
```

or equivalently:

```text
exists nonzero child codeword vanishing on P union A.
```

Thus high-rank flat excess is not a new marked-rank phenomenon; it is charged by the original
child zero-set/rank-tail object one level lower.

### Proof

The incremental rank map is:

```text
ev_A|K_P : K_P -> F^A.
```

By rank-nullity:

```text
rank(ev_A|K_P) <= D-1
```

if and only if:

```text
ker(ev_A|K_P) != 0.
```

But:

```text
ker(ev_A|K_P) = ker(ev_{P union A}).
```

So a nonzero vector in this kernel is exactly a nonzero child message whose child codeword vanishes
on every coordinate in `P union A`.

### Target Scale

For the target profile and flat excess `F <= S`, the high-rank endpoint has:

```text
r = D-1
|A| = 2(D-1)+F
|P union A| = 1909+F
```

The child line-zero exponent is:

```text
(1909+F) - 1024 + 1 = 886+F.
```

The residual root-repair exponent is:

```text
S-F = 72-F.
```

So the combined exponent is constant:

```text
(886+F) + (72-F) = 958.
```

This is the first real crawl win: the numerically weakest positive-flat-excess stratum can be
merged into the old proof architecture with no closure machinery.

## Tier 1 Certificate Shape

Tier 1 gives a conditional upgrade:

```text
original fixed-survivor proof
  + surplus incidence repair
  + high-rank flat-excess duality
```

certifies all profiles whose positive flat-excess witnesses have `h=1`, provided the child
line-zero/rank-tail bound used by the original proof is available at the required lower layer.

Equivalently, the remaining uncharged flat-excess witnesses are only:

```text
h >= 2.
```

This is a genuine narrowing. It removes the high-rank endpoint, which the scale check says is the
dominant flat-excess stratum under the random-matrix model.

## Tier 2: One-Mark Closure Endpoint

### Statement Target

For the low-rank endpoint `r(A)=0`, flat excess means:

```text
A subset cl(P).
```

The smallest nontrivial case is:

```text
C_d(p,1) = E[# {(P,a): |P|=p, a notin P, a in cl(P)}].
```

The proof target is the one-mark recurrence:

```text
C_d(p,1)
  <= A0_direct(d,p)
     + sum_y C_{d-1}(y,1) * Lift(d,p,y),
```

where:

```text
A0_direct(d,p) = n_d binom(n_d-2,p) q^{-(k_d-p)}
Lift(d,p,y)    = 2^{1+y} binom(2(n_{d-1}-1)-y, p-1-y).
```

The `A0` term is the case where the marked coordinate's sibling is not in `P`. The recursive term
is the `PA` case; the PA projection lemma routes parent closure to child closure.

### Current Evidence

The diagnostic recurrence gives:

```text
C_10(137,1) ~= 2^1389.199423.
```

Standalone this is too large, but the actual flat-excess stratum also has residual repair:

```text
q^{-(S-1)} = q^-71.
```

Thus:

```text
C_10(137,1) q^-71 ~= 2^-7698.800577.
```

Tier 2 is therefore a good bounded experiment. It has large slack if the recurrence can be made
theorem-grade, but it should be abandoned if it immediately requires arbitrary child matroid shape
state.

## Tier 3: Multi-Mark Closure Via Circuits

For multiple PA marks, the strong claim:

```text
J subset cl_child(Q)
```

is unsafe. The safe deterministic statement is:

```text
rank_child(Q union J)-rank_child(Q) <= |J|-1.
```

By matroid circuit reduction, this is contained in a union of one-mark closure events:

```text
exists empty != B subseteq J, exists j in B:
  j in cl_child(Q union (B\{j})).
```

The circuit-union loss is at most:

```text
|J| 2^|J|.
```

For the target `|J| <= 72`, that loss is tiny compared to the available `q`-exponents. Tier 3
should only be attempted after Tier 2 is theorem-grade.

## Go/No-Go Guardrails

This ladder is meant to prevent another rabbit hole.

Proceed from Tier 1 to Tier 2 only if the proof can be stated with:

```text
core size p,
one rank-defect parameter u,
and the A0/PA split.
```

Stop and reassess if one-mark closure requires:

```text
arbitrary child projection matroids,
multi-layer marked flags,
or a large table of local diagram types.
```

In that case, the correct output is still valuable: Tier 1 remains a clean upgrade, and the failure
would identify the low-rank closure endpoint as the true obstruction.

## Completed Tier-1 Artifact

Tier 1 is now written as a standalone theorem:

```text
docs/rfc_distance_analysis/rfc_high_rank_flat_endpoint_theorem.md
```

and the small helper:

```text
scripts/rfc_distance_analysis/rfc_incremental_upgrade_ladder.py
```

confirms the target rows:

```text
F=1,16,72 -> total exponent 958 q-dimensions.
```

## Immediate Next Step

Do not jump straight to full multi-mark closure. The next bounded crawl step is:

```text
Prove the one-mark closure recurrence C_d(p,1)
with at most one core-rank defect parameter u.
```

The first defect correction is recorded in:

```text
docs/rfc_distance_analysis/rfc_one_mark_closure_defect_crawl.md
```

It suggests a better formulation: do not recurse on exact `C_d(p,1;u)`. Instead, use `u` only in
the direct `A0` branch, where defective cores are charged by the ordinary rank-tail event
`rank(P union {a}) <= p-u`, and keep the `PA` branch as a total child-closure overcount.

The go/no-go condition remains: if formalizing the `PA` lift requires arbitrary child projection
matroids or multi-layer flag diagrams, stop and keep Tier 1 as the proven partial upgrade.
