# RFC Short-Set Rank-Tail Target

Status: active theorem target for charging small-rank flat excess.

## Why This Exists

The flat-excess incidence correction says the desired surplus repair exponent is reduced by:

```text
flat_excess = max_A (|A|-2 rank(A)).
```

For a top fixed-survivor profile, let:

```text
P   = paired child positions,
T   = singleton child positions,
D   = dim ker(child evaluation on P),
K_P = that kernel,
A   subset T,
r   = rank(K_P|_A),
F   = |A|-2r.
```

A witness `A` with positive flat excess gives a child set:

```text
Z = P union A
```

with:

```text
|Z| = |P| + 2r + F.
```

The common kernel inside `K_P` has dimension at least:

```text
h = D-r.
```

So the child restriction to `Z` has rank at most:

```text
k_child - h = k_child - D + r.
```

Equivalently, relative to the maximum possible rank `|Z|`, the child set `Z` has rank deficit:

```text
s = |Z| - (k_child-D+r)
  = |P| + 2r + F - k_child + D - r
  = rho + r + F,

rho = |P| + D - k_child.
```

Thus a small overloaded flat is not merely analogous to a generalized-Hamming-weight event. It is
exactly a short-set rank-tail event:

```text
rank(G_child[Z]) <= |Z| - s,
where s = rho+r+F.
```

## Random-Matrix Scale

For a fully random `k x z` matrix with `z <= k`, the probability of rank at most `z-s` has
q-exponent:

```text
s(k-z+s).
```

In the flat-excess variables:

```text
k_child - |Z| + s
  = k_child - (|P|+2r+F) + (rho+r+F)
  = D-r
  = h.
```

So the random-matrix exponent is:

```text
s h = (rho+r+F)(D-r).
```

This is the same exponent recorded in `rfc_small_flat_subcode_charge.md`:

```text
-(D-r)(rho+r+F).
```

For the dominant `c=8,k=2048,e=71` top profile:

```text
k_child = 1024
|P|     = 137
D       = 887
rho     = 0
```

Even `r=0,F=1` would pay a random-matrix exponent:

```text
887 q-dimensions.
```

This is far more than the entropy of choosing such a small flat.

## Important RFC Caveat

The fixed-set random-matrix exponent is false for RFC in this raw form.

If the child set `Z` is itself all-paired at the next lower RFC level, then:

```text
rank_parent(Z) = 2 rank_lower(U),
|Z| = 2|U|.
```

A parent rank deficit `s` then follows from a lower rank deficit roughly `ceil(s/2)`, with the
probability controlled by the lower code. For example, a one-rank drop in `138` columns of a
`k=1024` child can compress to a one-rank drop in `69` columns of a `k=512` grandchild, costing about:

```text
q^-(512-69+1)
```

rather than:

```text
q^-(1024-138+1).
```

So the theorem cannot be:

```text
every fixed Z behaves like a random matrix.
```

The theorem must be shape-sensitive, exactly as in the near-MDS rank-tail problem.

## Correct Theorem Target

Define the short-set rank-tail first moment:

```text
B_d(z,s)
  = E_T[# {Z subset [N_d] :
           |Z|=z and rank(G_d[Z]) <= min(k_d,z)-s}].
```

For small-flat charging we mainly need the regime:

```text
z < k_d,
s >= 1,
k_d-z+s large.
```

The target recurrence should prove:

```text
B_d(z,s) <= shape-sensitive random-like bound,
```

where:

1. all-paired branches recurse exactly:

```text
B_d(2u,s) receives a term from B_{d-1}(u,ceil(s/2));
```

2. singleton branches pay root-line/generic-rank constraints;
3. finite-field rank drops pay surplus codimension, corrected by lower-level flat events;
4. enough total exponent remains after unioning over `Z` and over flat witnesses `A`.

This is not a separate proof universe from the near-MDS proof. It is the same shape-sensitive
rank-tail recurrence, but with a rank-deficit parameter `s` and with `z` allowed below `k`.

## Connection Back To Flat Excess

The corrected surplus repair bound gives:

```text
local repair codim >= |T|-2D+1-F
```

unless there is a flat witness `A` with:

```text
F = |A|-2r(A).
```

The witness implies:

```text
B_child(|P|+2r+F, rho+r+F)
```

for the child. Therefore the flat-excess proof can close if the short-set rank-tail recurrence is
strong enough to make these child events rare.

This is the current best formulation of the blocker:

```text
prove a two-parameter shape-sensitive rank-tail theorem B_d(z,s),
then use it to charge flat excess in the surplus incidence recurrence.
```

## What Would Count As Progress

A useful next step is not more broad sampling. It is a go/no-go recurrence for:

```text
B_d(z,s)
```

with exact paired compression and a conservative singleton rank-drop envelope. The first diagnostic
should test the dominant flat-excess profile:

```text
d_child = 10,
k_child = 1024,
z       = 137 + 2r + F,
s       = r + F,
```

for small `r,F`, especially:

```text
(r,F) = (0,1), (1,1), (0,8), (8,1).
```

If that recurrence retains large negative log-moment after shape-sensitive paired compression, then
small flats are likely controllable. If it does not, the near-MDS surplus route has a genuine
obstruction rather than just a missing local lemma.

## First Paired-Envelope Diagnostic

The helper:

```text
scripts/rfc_distance_analysis/rfc_short_set_rank_tail_envelope.py
```

computes only the obvious all-paired degradation:

```text
(d,z,s) -> (d-1,z/2,ceil(s/2))
```

and compares the random-matrix rank-tail exponent at each compressed level against coordinate
entropy. It is not a recurrence proof because it does not handle mixed singleton shapes. It is a
sanity check for whether the most transparent RFC obstruction already kills the small-flat plan.

For the dominant child profile `d=10,k=1024,n=8192,q=2^128`:

```text
(r,F)=(0,1): z=138, s=1
  random exponent at depth 10: 887 q-dimensions
  paired-compressed exponent:  444 q-dimensions
  worst log2 slack after coordinate entropy: 56331.154937 bits

(r,F)=(1,1): z=140, s=2
  random exponent at depth 10: 1772 q-dimensions
  paired-compressed exponent:  222 q-dimensions
  worst log2 slack after coordinate entropy: 28164.345920 bits

(r,F)=(0,8): z=145, s=8
  no all-paired compression because z is odd
  worst log2 slack after coordinate entropy: 907241.662066 bits

(r,F)=(8,1): z=154, s=9
  paired-compressed exponent: 2200 q-dimensions
  worst log2 slack after coordinate entropy: 281052.952822 bits
```

So the naive random-matrix exponent is definitely too optimistic as a theorem statement, but the
first RFC-specific degradation still leaves huge slack in the small-flat stress cases. The next
real issue is mixed singleton shapes and finite-field rank-drop incidence inside the two-parameter
`B_d(z,s)` recurrence.
