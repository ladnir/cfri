# Systematic RFC Uncertainty Route

This note records the cleaner proof route suggested by the collapse family.

The rank-defect formulation is equivalent to a codeword formulation. For the all-level systematic
code:

```text
G = [ I | A_1 | A_2 | ... | A_{c_p} ]
```

where each `A_i` is one independent RFC parity tree and `c_p = c-1`. If a zero set deletes all
systematic coordinates outside a live set `R`, then a parity zero set `Q` is rank-deficient exactly
when there is a nonzero message `x` supported on `R` that vanishes on every parity coordinate in
`Q`.

Thus the systematic distance question is:

```text
min_{x != 0} wt(x) + sum_i wt(A_i x).
```

The collapse family is not just a bad rank shape. It is a one-copy uncertainty extremizer.

## One-Copy Uncertainty

Let `A_d` be one depth-`d` RFC parity transform, so `k = 2^d`. The sharp structural statement is
over the rational function field in the RFC challenges:

```text
wt(x) * wt(A_d x) >= k
```

for every nonzero `x`. Equivalently, if `m = wt(x)`, one parity copy has at most:

```text
k - ceil(k/m)
```

structural zeros.

This is a generic-support statement, not a statement about every finite-field evaluation. If a
factor such as `1-T` evaluates to zero, an otherwise nonzero output polynomial can vanish. Those
events are the accidental determinant/root events that must be charged separately by the large
field size.

The proof is the natural induction on the butterfly.

Write the child messages as `x_0,x_1`, and let:

```text
u = A_{d-1} x_0
v = A_{d-1} x_1.
```

At each child output coordinate, the parent pair is:

```text
y_0 = u + T (v-u)
y_1 = v + T (v-u).
```

The local map from `(u_j,v_j)` to `(y_{0,j},y_{1,j})` is invertible over the rational function
field, so if `(u_j,v_j) != (0,0)`, the parent pair is not identically zero. Therefore its generic
support obeys:

```text
wt(A_d x) >= |supp(u) union supp(v)|
           >= max(wt(u), wt(v)).
```

By induction, if `a = wt(x_0)` and `b = wt(x_1)`, then:

```text
wt(u) >= 2^(d-1)/a
wt(v) >= 2^(d-1)/b
```

with the convention that an absent child contributes no constraint. Hence:

```text
wt(A_d x) >= max(2^(d-1)/a, 2^(d-1)/b)
           >= 2^d/(a+b)
           = k/wt(x).
```

This is exactly tight for recursive live-subcube vectors: if `x` is supported on an `m`-row subcube
and chosen to annihilate one live direction, then one copy can have output support `k/m`.

The diagnostic:

```text
scripts/rfc_uncertainty_check.py
```

exhausts tiny evaluated fields and records where the evaluated support product drops below `k`.
For `GF(5)`, depth `3`, these drops do occur, as expected, because the tiny field frequently hits
local roots. This is useful calibration for the large-field proof: the uncertainty lemma gives the
nonzero polynomial support, while Schwartz-Zippel charges the evaluated root losses.

## Why This Alone Is Not Enough

Applying the one-copy bound independently to all `c_p` copies would give:

```text
wt(codeword) >= m + c_p k/m,
```

which is only `O(sqrt(k))` after optimizing `m`. That bound is far too weak because it allows the
same message to be uncertainty-extremal for every independent parity copy.

The collapse family only makes one copy sparse. The other `c_p-1` independent copies should be
dense for that same message. This is why the observed upper ceiling is:

```text
wt(codeword) <= (c_p-1)k + m + k/m
```

and after optimizing `m ~= sqrt(k)`:

```text
delta_sys <= (c_p-1)/(c_p+1) = 1 - 2/c.
```

For `c=8`, this is the `0.75` ceiling.

## Multi-Copy Proof Target

The matching lower-bound target is now concrete.

For a fixed live support `R` of size `m` and a fixed copy `i`, a selected parity-zero set `Q_i`
defines a kernel:

```text
K_i(R,Q_i) = { x in F^R : A_i x vanishes on Q_i }.
```

One-copy uncertainty says that if:

```text
|Q_i| > k - k/m,
```

then generically `K_i(R,Q_i) = {0}`. At the extremal size `|Q_i| = k-k/m`, the collapse examples
produce a one-dimensional kernel.

Therefore, to beat the `1-2/c` systematic distance ceiling, a bad codeword must do one of two
things:

```text
1. beat the one-copy uncertainty bound in some copy; or
2. land in nonzero kernels for two or more independent copies.
```

The first event should be ruled out structurally by the uncertainty theorem. The second event is
where the large-field first moment belongs. For the extremal subcube kernels, two independent
copies produce two independent lines in `F^m`; their intersection is nonzero only when the lines
coincide, a probability about:

```text
q^-(m-1).
```

More generally, if the selected zero sets leave copy-kernels of dimensions `r_1,...,r_L`, then a
fixed-support intersection heuristic gives:

```text
Pr[ intersection_i K_i != {0} ] ~= q^-( (L-1)m - sum_i (r_i-1) ).
```

The certificate should sum this over support shapes and per-copy zero patterns, preserving the
kernel dimension rather than only recording success/failure.

## Current Best Proof Shape

The proof should not copy the old threshold recurrence. The better structure is:

```text
one-copy uncertainty theorem
  + per-copy kernel-dimension enumerator
  + multi-copy first moment over kernel intersections
  + Schwartz-Zippel/large-field charge for accidental determinant zeros
```

This directly tracks the distribution, not just a failing threshold. It also explains the observed
gap:

```text
original RFC:      MDS, relative distance 1 - 1/c
systematic RFC:    one copy can be uncertainty-sparse, relative ceiling 1 - 2/c
```

For `c=8`, the realistic systematic target remains close to `0.75`, with slack depending on how
tightly the multi-copy kernel-intersection moment can be counted.
