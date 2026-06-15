# High-Rank Flat-Excess Endpoint Theorem

Status: proof-ready Tier-1 brick for the incremental original-proof upgrade.

## Purpose

This note isolates the first crawl step in the upgraded original RFC proof. It proves that the
high-rank flat-excess endpoint is not a new marked-rank object. It is exactly an ordinary child
line-zero event.

This gives a partial theorem we can merge into the original proof before attempting the low-rank
closure endpoint.

## Setup

At one parent fold, fix a survivor profile and write:

```text
P = paired child positions
T = singleton child positions
p = |P|
t = |T|
```

Let the child message dimension be `k_child`, and let:

```text
ev_P : F^k_child -> F^P
K_P  = ker(ev_P)
D    = dim K_P.
```

For a singleton witness set `A subset T`, define:

```text
r(A) = rank(P union A) - rank(P).
```

Equivalently, `r(A)` is the rank of the restricted map:

```text
ev_A|K_P : K_P -> F^A.
```

Write:

```text
h = D - r(A).
```

The flat excess of `A` is:

```text
F = |A| - 2r(A).
```

The local singleton repair incidence theorem gives residual repair exponent:

```text
max(0, S-F),      S = t - 2D + 1.
```

## Theorem: High-Rank Endpoint Is Child Line-Zero

For any fixed `P` and `A`, the following are equivalent:

```text
h >= 1
```

and

```text
there exists a nonzero child message w
such that w vanishes on every coordinate in P union A.
```

In particular, the high-rank endpoint:

```text
h = 1
```

is contained in the ordinary child line-zero event on the zero set:

```text
Z = P union A.
```

It does not require a marked-pair rank-tail theorem.

## Proof

By definition:

```text
r(A) = rank(ev_A|K_P).
```

Rank-nullity gives:

```text
dim ker(ev_A|K_P) = D - r(A) = h.
```

But:

```text
ker(ev_A|K_P)
  = { w : ev_P(w)=0 and ev_A(w)=0 }
  = ker(ev_{P union A}).
```

Therefore `h>=1` if and only if `ker(ev_{P union A})` contains a nonzero child message. This is
exactly the ordinary line-zero event for the child zero set `P union A`.

The statement for `h=1` follows immediately.

## Counting Form

Let:

```text
L_{d-1}(z)
```

denote any valid child line-zero first-moment bound for a fixed child zero set of size `z`:

```text
L_{d-1}(z)
  >= E[# projective child lines vanishing on a fixed Z, |Z|=z].
```

Then the contribution from parent profiles with an `h=1` flat-excess witness of excess `F` and
fixed `(p,t,D)` is bounded by:

```text
binom(n_child, p)
binom(n_child-p, a)
binom(n_child-p-a, t-a)
2^t
L_{d-1}(p+a)
q^-max(0,S-F)
```

where:

```text
a = |A| = 2(D-1)+F.
```

The three binomial factors choose `P`, the marked flat witness `A`, and the remaining singleton
positions. The `2^t` factor chooses singleton sides. The line-zero factor charges the existence of
the nonzero child message vanishing on `P union A`. The residual root factor is the corrected
singleton-repair incidence exponent after flat excess `F` is exposed.

This is an overcount because the same child zero set can have many `(P,A,T\A)` decompositions. That
is acceptable for Tier 1; the target has huge slack.

## Random-Code Scale

For a random linear child code, the fixed-set line-zero exponent is:

```text
L_{d-1}(z) ~= q^{-(z-k_child+1)}
```

for `z >= k_child`.

For the target top profile:

```text
c = 8
k_parent = 2048
k_child = 1024
n_child = 8192
p = 137
t = 1845
D = 887
S = 72
```

the `h=1` endpoint has:

```text
a = 2(D-1)+F = 1772+F
z = p+a = 1909+F.
```

So:

```text
child line-zero exponent = z-k_child+1 = 886+F.
```

For `F <= S`, the residual repair exponent is:

```text
S-F = 72-F.
```

The combined q-exponent is:

```text
(886+F) + (72-F) = 958.
```

Thus positive flat excess cancels out in the high-rank endpoint: increasing `F` makes the child
line-zero request larger by exactly the amount it reduces the root-repair surplus.

## Checked Target Rows

The helper:

```text
scripts/rfc_distance_analysis/rfc_incremental_upgrade_ladder.py
```

reports:

```text
F=1:  line_zero=887, residual=71, total=958
F=16: line_zero=902, residual=56, total=958
F=72: line_zero=958, residual=0,  total=958
```

The older stratified scale diagnostic agrees that these rows are dominated by:

```text
r = D-1.
```

and have log2 first-moment terms below:

```text
-113000 bits.
```

## What This Does And Does Not Prove

This theorem proves the high-rank endpoint reduction. It does not prove the entire distance
certificate.

It closes the following part of the flat-excess stack:

```text
flat-excess witness with h=1
  -> ordinary child line-zero event on P union A
  -> residual root-repair factor q^-max(0,S-F).
```

The remaining flat-excess work begins at:

```text
h >= 2.
```

The most delicate endpoint there remains:

```text
r(A)=0,  A subset cl(P),
```

which is the one-mark/multi-mark closure-tail program. Tier 1 deliberately avoids that program.

## Next Proof Step

Splice this theorem into the main certificate statement as a closed local brick. Then separately
state the reduced remaining assumption:

```text
All flat-excess witnesses with h>=2 are controlled by subcode-zero/closure-tail bounds.
```

This gives a real partial theorem: the dominant high-rank flat-excess stratum is handled by the
original child line-zero recurrence, and the open work is confined to lower-rank endpoints.
