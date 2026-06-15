# RFC Small-Flat Subcode Charge

Status: reframing of the small-rank flat-excess blocker.

The previous note split flat excess into:

```text
large-rank flats: visible to ordinary child rank-tail;
small-rank flats: need quotient-uniformity.
```

This note identifies the small-rank regime as a generalized-Hamming-weight/subcode-zero event.

## Flat As A Vanishing Subcode

At a top step, let:

```text
K_P = ker(child evaluation on paired positions P)
dim K_P = D.
```

Let `A subseteq T` have quotient rank:

```text
r = rank(K_P|_A)
```

and flat excess:

```text
F = |A| - 2r.
```

The common kernel of the functionals in `A` inside `K_P` is a subspace:

```text
L = {x in K_P : ell_i(x)=0 for all i in A}
```

with:

```text
dim L >= D-r.
```

Every vector in `L` vanishes on:

```text
P union A.
```

So a rank-`r` flat-excess witness gives an `h=(D-r)`-dimensional child subcode with at least:

```text
z = |P| + 2r + F
```

common zero coordinates.

## Random-Code First-Moment Scale

For a random `[n_child,k_child]` code, the first-moment exponent for such a subcode is:

```text
[k_child choose h]_q * binom(n_child,z) * q^{-hz}.
```

Ignoring polynomial/coordinate entropy terms, the q-exponent is:

```text
h(k_child-h-z).
```

Substitute:

```text
h = D-r,
z = |P| + 2r + F,
rho = |P| + D - k_child.
```

Then:

```text
k_child-h-z
  = k_child-(D-r)-(|P|+2r+F)
  = -rho-r-F.
```

Therefore the q-exponent is:

```text
-(D-r)(rho+r+F).
```

In the dominant near-MDS top profile:

```text
rho=0.
```

So any positive `r+F` pays a large q-exponent:

```text
-(D-r)(r+F).
```

This is exactly the missing charge for small-rank overloaded flats.

## Dominant Profile Numbers

For the `c=8,k=2048,e=71` top-profile stress:

```text
k_child = 1024
n_child = 8192
|P|     = 137
D       = 887
rho     = 0
```

The bookkeeping script:

```text
scripts/rfc_distance_analysis/rfc_flat_excess_subcode_charge.py
```

reports the random-code first-moment scale. Example with `F=1`:

```text
flat_rank r=0: q_exponent = -887,  log2 moment ~= -112530
flat_rank r=1: q_exponent = -1772, log2 moment ~= -225799
flat_rank r=2: q_exponent = -2655, log2 moment ~= -338811
```

These are enormous compared to the coordinate entropy for such small zero sets. Thus small-rank
flat excess should be chargeable if the RFC child code satisfies a suitable subcode-zero moment
bound.

## Theorem Target

The needed theorem is a generalized subcode zero bound for RFC:

```text
For every h-dimensional message subspace L, bound the probability/count that L vanishes on z child
coordinates by roughly q^{-h z}, up to the correct RFC structural constants.
```

Equivalently, prove that RFC has near-MDS generalized weights, at least in the parameter range:

```text
h = D-r large,
z = |P|+2r+F small-to-moderate,
rho+r+F positive.
```

This may be easier than the original codeword-distance problem in the small-flat regime because
the subcode dimension `h` is huge, so each extra zero coordinate costs `h` equations.

## Meaning For The Proof

Flat-excess control now splits into a plausible two-part strategy:

```text
small r:
  charge by subcode-zero/generalized-weight first moment;

large r:
  charge recursively as an ordinary child rank-tail event on P union A.
```

This reduces the quotient-uniformity blocker to a known coding-theory object: generalized Hamming
weights/subcode zero moments for RFC.
