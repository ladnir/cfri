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

This is a generic rank/support statement over the rational function field of the challenges. The
message witnessing a kernel may depend on the challenges, so the proof cannot treat the message
coordinates as constants independent of the root challenge. The right local input is the MDS
support property of each local `2 x 2` fold matrix over that rational function field.

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

The local `2 x 2` matrix is MDS over the rational function field: all entries are nonzero
polynomials and the determinant is nonzero. Therefore, for each coordinate `j`:

```text
wt(u_j,v_j) = 1  =>  wt(y_{0,j},y_{1,j}) = 2
wt(u_j,v_j) = 2  =>  wt(y_{0,j},y_{1,j}) >= 1.
```

Let:

```text
p = wt(u)
q = wt(v).
```

If the supports of `u` and `v` overlap in `r` positions, the parent output weight is at least:

```text
2(p+q-2r) + r.
```

This is minimized when the overlap is as large as possible, giving:

```text
wt(A_d x) >= 2 max(p,q) - min(p,q).
```

In particular:

```text
wt(A_d x) >= max(p,q).
```

By induction, if `a = wt(x_0)` and `b = wt(x_1)`, then:

```text
p >= 2^(d-1)/a
q >= 2^(d-1)/b
```

with the convention that an absent child contributes no constraint. Therefore:

```text
wt(A_d x) >= max(p,q)
           >= max(2^(d-1)/a, 2^(d-1)/b)
           = 2^(d-1) / min(a,b)
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
For `GF(5)`, depth `3`, these drops do occur in sampled evaluations. This means the theorem must
be phrased with the standard generic/large-field exception: the structural support certificate
holds away from determinant/root hypersurfaces, and tiny fields hit those hypersurfaces often.

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

## One-Copy Kernel Envelope

The one-copy uncertainty theorem gives a useful dimension bound, not just a yes/no bound.

Fix a live support `R` with:

```text
|R| = m.
```

Let `Q` be `q` parity coordinates in one copy, and define:

```text
K(R,Q) = { x in F^R : (A x)|_Q = 0 }.
```

If `dim K(R,Q) = r`, then elementary linear algebra gives a nonzero vector in `K(R,Q)` with input
support at most:

```text
m - r + 1.
```

That vector has output support at most `k-q`. By one-copy uncertainty:

```text
k - q >= k / (m-r+1).
```

Therefore:

```text
dim K(R,Q) <= max(0, m + 1 - ceil(k/(k-q))).
```

The helper:

```text
scripts/rfc_one_copy_kernel_envelope.py
```

emits this envelope. For depth `11`, `k=2048`, the distance-dominant live size is `m=32`. Around
the extremal one-copy parity-zero count `q=1984`:

```text
q       output budget   dim bound
1978    70              3
1979    69              3
1980    68              2
1981    67              2
1982    66              1
1983    65              1
1984    64              1
1985    63              0
```

So the extremal copy really leaves only a line, and asking for even one more zero in that same copy
kills the kernel structurally.

The remaining combinatorial issue is classifying when the line case can happen. That stability
target is split out in:

```text
docs/rfc_uncertainty_stability.md
```

The short version: equality in the uncertainty induction forces even support splitting and matching
child output supports at every active recursive node, which should classify exact extremizers as
aligned subcubes. This classification is what keeps the final union bound from paying
`binom(k,m)` for arbitrary supports.

## Aligned Extremal Intersection Check

The helper:

```text
scripts/rfc_collapse_intersection_moment.py
```

counts the simplest dangerous multi-copy event: two independent parity copies are both extremal on
the same aligned live subcube. For a live subcube of size `m`, each copy contributes an extremal
kernel line; two lines in `F^m` coincide with probability about:

```text
q^{-(m-1)}.
```

The depth-`11`, `c=8`, `q=2^128` table is:

```text
docs/rfc_collapse_intersection_moment_c8_depth1_to_11.csv
```

The worst aligned two-copy union bound through depth `11` occurs for `m=2` and is already:

```text
log2 aligned-subcube bound = -108.14825096.
log2 matched-extremizer bound = -111.60768258.
```

The distance-dominant collapse at depth `11` uses `m=32`; its two-copy intersection term has
collision exponent:

```text
-(32-1) * 128 = -3968.
```

With the matched block/stride extremizer count suggested by the exact scans, the full depth-`11`,
`m=32` two-copy term is:

```text
log2 matched-extremizer bound = -3947.60768258.
```

So within the aligned extremal family, simultaneous two-copy sparsity is negligible. The real proof
still needs the arbitrary-support/kernel-dimension count and the exact stability theorem, but this
check supports the picture that one sparse copy should dominate the systematic distance.

## Current Best Proof Shape

The proof should not copy the old threshold recurrence. The better structure is:

```text
one-copy uncertainty theorem
  + per-copy kernel-dimension enumerator
  + multi-copy first moment over kernel intersections
  + Schwartz-Zippel/large-field charge for accidental determinant zeros
```

The conditional theorem statement is now split out in:

```text
docs/rfc_systematic_distance_certificate_outline.md
```

This directly tracks the distribution, not just a failing threshold. It also explains the observed
gap:

```text
original RFC:      MDS, relative distance 1 - 1/c
systematic RFC:    one copy can be uncertainty-sparse, relative ceiling 1 - 2/c
```

For `c=8`, the realistic systematic target remains close to `0.75`, with slack depending on how
tightly the multi-copy kernel-intersection moment can be counted.
