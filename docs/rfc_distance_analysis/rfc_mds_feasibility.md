# RFC MDS Feasibility

This note separates three claims that were previously conflated.

## 1. Foldable MDS Codes Exist In Principle

MDS is possible for some foldable codes. Reed-Solomon/FRI-style foldable codes are the standard
example: under a suitable domain split, the folded code remains Reed-Solomon-like, and the
underlying evaluation code is MDS.

That does not mean a random foldable code is sampled MDS. Reed-Solomon achieves MDS through a
global normal-rational-curve structure, not by making each minor a generic random polynomial.

## 2. Fixed-Subset Generic Rank Is Not Sampled MDS

For RFC-style random foldable codes, the recursive certificate may prove:

```text
for every fixed k-column set Q, det(G_Q) is not the zero polynomial.
```

This is only a fixed-subset generic-minor statement. It does not imply:

```text
after sampling the RFC challenges, all k-column sets Q have det(G_Q) != 0.
```

The second statement is sampled MDS.

The difference is the number of minors. At `N = c k`, there are `binom(N,k)` possible `k`-column
sets. If those determinants behave even roughly like random nonzero field elements, sampled MDS is
not plausible unless the field is exponentially larger than `binom(N,k)` or the code has a special
RS-like global structure.

## 3. First-Moment Scale

For a random `k x z` matrix over `GF(q)`, the probability of rank `< k` is about:

```text
q^-(z-k+1).
```

Thus the expected number of deficient `z`-subsets is modeled by:

```text
E_bad(z) ~= binom(N,z) q^-(z-k+1).
```

At `z = k`, this becomes:

```text
E_bad(k) ~= binom(N,k) / q.
```

Concrete scales:

```text
c=4, k=2048, q=2^128:
  log2 binom(N,k) ~= 6639.37
  E_bad(k) ~= 2^6511.37

c=8, k=2048, q=2^128:
  log2 binom(N,k) ~= 8899.03
  E_bad(k) ~= 2^8771.03

c=8, k=2048, q=2^256:
  log2 binom(N,k) ~= 8899.03
  E_bad(k) ~= 2^8643.03

c=8, k=2^25, q=2^256:
  log2 binom(N,k) ~= 145911955.45
  E_bad(k) ~= 2^145911699.45
```

So sampled MDS for a random-like RFC is not a realistic target at these parameters.

## 4. Near-MDS First-Moment Target

The same first-moment model gives a realistic near-MDS target. For security target `2^-80`, solve:

```text
log2 binom(N,z) - (z-k+1) log2(q) <= -80.
```

Examples:

```text
c=4, k=2048, q=2^128:
  crossing z = k + 53
  relative distance ~= 0.74365
  MDS relative distance ~= 0.75012

c=8, k=2048, q=2^128:
  crossing z = k + 71
  relative distance ~= 0.87073
  MDS relative distance ~= 0.87506

c=8, k=2048, q=2^256:
  crossing z = k + 35
  relative distance ~= 0.87292
  MDS relative distance ~= 0.87506

c=8, k=2^25, q=2^256:
  crossing z = k + 576256
  relative distance ~= 0.87285
  MDS relative distance ~= 0.87500
```

This suggests the right random-RFC target is not exact MDS but a first-moment near-MDS theorem:

```text
Pr[exists deficient z-subset] <= 2^-lambda
```

for `z = k + e`, where `e` is determined by the first-moment crossing above.

The formal proof target is tracked in:

```text
docs/rfc_distance_analysis/rfc_near_mds_first_moment_proof.md
```

## 5. Current Read

For RFC-style random foldable codes:

```text
sampled exact MDS:      unlikely without RS-like global structure
fixed-subset generic:  plausible and supported by small checks
near-MDS first moment: plausible and likely the right proof target
```

For "any version" broadly construed, MDS is possible by moving toward Reed-Solomon/FRI-style
foldable codes. The tradeoff is that this may lose the field-agnostic/simple-RFC structure that made
BaseFold attractive.
