# High-Rank Flat Duality Reduction

Status: theorem target for the weakest flat-excess stratum in the upgraded original-proof route.

## Purpose

The flat-excess charge push showed that the numerically weakest flat-excess stratum is:

```text
r = D - 1,
```

where `D` is the paired-child kernel dimension. This note records a cleaner structural reduction:
that stratum is not a genuinely new marked-pair event. It is an ordinary child zero-set event.

More generally, a rank drop of `h = D-r` in the singleton quotient is equivalent to an
`h`-dimensional child subspace vanishing on the paired core plus the flat witness.

## Setup

Let `C` be the child RFC code at a fixed lower layer. Let:

```text
P = paired child positions
A = singleton flat witness positions
```

Let:

```text
ev_P : F^k -> F^P
K_P = ker(ev_P)
D = dim K_P
```

The singleton quotient functionals are the evaluations on `A` restricted to `K_P`:

```text
ev_A|K_P : K_P -> F^A.
```

Their rank is:

```text
r(A) = rank(P union A) - rank(P).
```

## Lemma

For any integer `h` with `0 <= h <= D`,

```text
r(A) <= D - h
```

if and only if there exists an `h`-dimensional subspace:

```text
W <= F^k
```

such that every `w in W` vanishes on all coordinates in:

```text
P union A.
```

Equivalently:

```text
dim ker(ev_{P union A}) >= h.
```

## Proof

The map controlling the incremental rank is:

```text
ev_A|K_P : K_P -> F^A.
```

By rank-nullity:

```text
dim ker(ev_A|K_P) = D - rank(ev_A|K_P).
```

But:

```text
ker(ev_A|K_P)
  = {m in F^k : ev_P(m)=0 and ev_A(m)=0}
  = ker(ev_{P union A}).
```

Thus:

```text
rank(ev_A|K_P) <= D-h
```

if and only if:

```text
dim ker(ev_{P union A}) >= h.
```

This is exactly the stated `h`-dimensional child subspace event.

## High-Rank Flat Case

For:

```text
r = D-1,
h = 1,
```

the flat witness means:

```text
there exists a nonzero child message vanishing on P union A.
```

So the numerically weakest high-rank flat-excess stratum is the ordinary child distance/rank-tail
event:

```text
B_child(1, |P|+|A|).
```

At the target top profile:

```text
p = |P| = 137
D = 887
S = t - 2D + 1 = 72
```

for flat excess `F` and `r=D-1`:

```text
|A| = 2(D-1) + F
|P|+|A| = 137 + 1772 + F = 1909 + F.
```

The child has:

```text
k_child = 1024.
```

Therefore the ordinary line-zero q-exponent is:

```text
(|P|+|A|) - k_child + 1
  = (1909+F) - 1024 + 1
  = 886 + F.
```

The residual root-repair exponent is:

```text
max(0, S-F).
```

For `F <= S`, the combined exponent is:

```text
(886+F) + (72-F) = 958.
```

This matches the stratified marked-rank scale exactly.

## Meaning

The highest-risk flat-excess stratum does not require the full marked incremental rank-tail theorem.
It can be charged by the existing original-proof object one level down:

```text
ordinary child line-zero count for |P|+|A| zeros.
```

This is a major simplification for the upgraded original proof. The theorem stack can split:

```text
high-rank flat excess, h=1:
  ordinary child distance/rank-tail recurrence

lower-rank flat excess, h>=2:
  higher-dimensional child subcode-zero event
  or marked incremental rank-tail recurrence
```

## General h

For `h = D-r`, the same duality gives an `h`-dimensional child subcode vanishing on:

```text
Z = P union A.
```

A random linear code first moment for such an event has q-exponent approximately:

```text
h(|Z| - k_child + h).
```

Here:

```text
|Z| = p + 2(D-h) + F.
```

For the target top profile:

```text
p = 137
D = 887
k_child = 1024
```

this becomes:

```text
h(|Z|-k_child+h)
  = h(137 + 2(887-h) + F - 1024 + h)
  = h(887 + F - h).
```

For `F=1`, the endpoints are:

```text
h = 887, r = 0:
  |Z| = 138
  q-exponent = 887
  random-code log2 moment ~= -112530

h = 1, r = 886:
  |Z| = 1910
  q-exponent = 887
  random-code log2 moment ~= -107124
```

Middle values have much larger q-exponents. Thus the flat-excess subcode-zero problem has two weak
endpoint regimes:

```text
1. high-rank flat, h=1:
   ordinary line-zero event with many zeros;

2. small-rank flat, h=D:
   large-dimensional subcode-zero event with few extra zeros.
```

The high-rank endpoint is already the original child distance/rank-tail object. The low-rank
endpoint is the generalized-subcode version of the same phenomenon. It is still simpler than the
full mixed marked-pair recurrence because it forgets the marked split and asks only for a child
subspace vanishing on the union `P union A`.

## Remaining Proof Obligation

This reduction does not by itself prove the distance certificate. It says the worst flat-excess
case is already covered by the original child zero-set recurrence.

The remaining hard part is now narrower:

```text
control child subcode-zero events
  dim W = h,
  W vanishes on Z,
  log_q scale h(|Z|-k+h),

especially the low-rank endpoint h=D.
```

This is a cleaner target than a fully general marked-pair theorem for every `(P,A,r)`.
