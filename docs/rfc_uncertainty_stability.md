# RFC Uncertainty Stability Target

This note isolates the classification theorem needed to turn the systematic ceiling into a
distance certificate.

## Why Stability Is Needed

The one-copy uncertainty lemma gives:

```text
wt(x) * wt(Ax) >= k.
```

For a live support of size `m`, this says one copy cannot have fewer than `k/m` nonzero output
coordinates. The systematic collapse family reaches equality by taking `x` on an `m`-point live
subcube and making one parity copy vanish outside a complementary output subcube.

For the multi-copy proof, it is not enough to know that equality is possible. We need to know that
the equality and near-equality cases are structured. Otherwise the union bound would have to count
arbitrary supports:

```text
binom(k,m),
```

which is too large at the target `m ~= sqrt(k)`.

At depth `11`, `m=32`:

```text
log2 binom(2048,32) ~= 232.
```

The aligned live-subcube count is tiny by comparison:

```text
log2(count) = log2 binom(11,5) + 6 ~= 14.78.
```

So a sharp proof must show that the dangerous one-copy kernels are subcube-like, not arbitrary.

## Equality Conditions In The Induction

Write a depth-`d` message as `(x_0,x_1)` with supports:

```text
a = wt(x_0)
b = wt(x_1)
m = a+b.
```

Let:

```text
u = A_{d-1} x_0
v = A_{d-1} x_1.
```

The uncertainty proof used:

```text
p = wt(u) >= 2^(d-1)/a
q = wt(v) >= 2^(d-1)/b
wt(A_d x) >= 2 max(p,q) - min(p,q)
           >= max(p,q)
           >= max(2^(d-1)/a, 2^(d-1)/b)
           >= 2^d/(a+b).
```

Equality at depth `d` forces equality at every step:

```text
1. a = b = m/2;
2. p = q = 2^(d-1)/a;
3. supp(u) = supp(v);
4. each active local 2 x 2 fold cancels exactly one of the two parent outputs, never both;
5. the child messages are equality cases recursively.
```

Thus exact equality can only persist when the support splits evenly at every active node and the
two child output supports coincide recursively. This is the recursive fingerprint of an affine
subcube support.

## Candidate Stability Theorem

The desired theorem is:

```text
If wt(x)=m and wt(Ax)=k/m generically, then m is a power of two and,
up to tree-coordinate relabeling induced by the recursion, supp(x) is an aligned live subcube.
```

The corresponding output support is the complementary quotient subcube of size `k/m`.

A near-equality version should also hold. If:

```text
wt(Ax) <= k/m + e,
```

then all but `O(e)` of the recursive equality conditions above must hold. The support should be
covered by a small number of aligned subcubes, or by a subcube with a small boundary defect. This is
the object the final first-moment proof needs to count.

## Kernel Version

For a live support `R` and parity-zero set `Q`, the extremal one-copy kernel has:

```text
|R| = m
|Q| = k - k/m
dim K(R,Q) = 1.
```

The stability theorem should imply:

```text
dim K(R,Q) = 1 at the extremal zero count
  => R and [k]\Q form a matched pair of recursive subcubes.
```

This turns the worst multi-copy event count from arbitrary support counting into aligned-subcube
counting. Then the two-copy line-intersection moment in:

```text
docs/rfc_systematic_uncertainty_route.md
```

becomes a realistic component of a full certificate, not merely a sanity check for one hand-picked
family.

## Next Mechanical Check

For small depths, enumerate support/output-support pairs over the recursive certificate rather than
over finite-field evaluations. The check should answer:

```text
For each support size m, which row supports R admit an output set W of size k/m
such that there is a nonzero generic vector supported on R and output-supported inside W?
```

The expected answer is:

```text
only recursive subcube pairs, plus finite-field accidental cases when evaluated over tiny fields.
```

This is the next concrete bridge from the current proof outline to a distance certificate near the
`1-2/c` systematic ceiling.

The exact proof skeleton and kernel-line uniqueness formulation are now in:

```text
docs/rfc_uncertainty_exact_stability_proof.md
```

## Exact Small-Depth Scan

The helper:

```text
scripts/rfc_uncertainty_extremizer_scan.py
```

does the first version of this check by evaluating a large-prime sample and exhaustively scanning
support/output-support pairs at the exact uncertainty boundary. Since non-subcube extremizers would
be structural, they should persist and show up in this scan with overwhelming probability.

The checked artifacts are:

```text
docs/rfc_uncertainty_extremizer_scan_depth3_m2.csv
docs/rfc_uncertainty_extremizer_pairs_depth3_m2.csv
docs/rfc_uncertainty_extremizer_scan_depth3_m4.csv
docs/rfc_uncertainty_extremizer_pairs_depth3_m4.csv
docs/rfc_uncertainty_extremizer_scan_depth4_m2.csv
docs/rfc_uncertainty_extremizer_pairs_depth4_m2.csv
docs/rfc_uncertainty_extremizer_scan_depth4_m4.csv
docs/rfc_uncertainty_extremizer_pairs_depth4_m4.csv
```

Results:

```text
depth 3, k=8,  m=2: checked 1,960 pairs,     extremizers 8,  matched 8
depth 3, k=8,  m=4: checked 1,960 pairs,     extremizers 8,  matched 8
depth 4, k=16, m=2: checked 1,544,400 pairs, extremizers 16, matched 16
depth 4, k=16, m=4: checked 3,312,400 pairs, extremizers 16, matched 16
```

No non-subcube extremizer appeared. In fact, every extremizer is a stricter block/stride pair:

```text
R = { s m, s m + 1, ..., s m + m - 1 }
W = { r, r + m, r + 2m, ..., r + (k/m - 1)m }.
```

There are exactly:

```text
(k/m) * m = k
```

such matched pairs for each power-of-two `m`. This is even better than arbitrary aligned-subcube
counting and matches the explicit collapse family: the live rows occupy a recursive block, while
the surviving output direction fixes the low `log2(m)` path bits.

## Near-Extremizer Scan

The next leakage case allows the output support to be slightly larger than the exact uncertainty
minimum:

```text
|W| = k/m + e.
```

At depth `4`, the first exact scans show the cleanest possible behavior: every near-extremizer is
an exact matched block/stride extremizer plus `e` arbitrary extra output positions.

Artifacts:

```text
docs/rfc_uncertainty_near_extremizer_scan_depth4_m4_w5.csv
docs/rfc_uncertainty_near_extremizer_pairs_depth4_m4_w5.csv
docs/rfc_uncertainty_near_extremizer_class_depth4_m4_w5.csv
docs/rfc_uncertainty_near_extremizer_scan_depth4_m2_w9.csv
docs/rfc_uncertainty_near_extremizer_pairs_depth4_m2_w9.csv
docs/rfc_uncertainty_near_extremizer_class_depth4_m2_w9.csv
docs/rfc_uncertainty_near_extremizer_scan_depth4_m2_w10.csv
docs/rfc_uncertainty_near_extremizer_pairs_depth4_m2_w10.csv
docs/rfc_uncertainty_near_extremizer_class_depth4_m2_w10.csv
```

Results:

```text
depth 4, m=4, e=1: checked 7,949,760 pairs, near-extremizers 192
depth 4, m=2, e=1: checked 1,372,800 pairs, near-extremizers 128
depth 4, m=2, e=2: checked   960,960 pairs, near-extremizers 448
```

These match exactly:

```text
k * binom(k-k/m, e).
```

The classifier verifies every saved near-extremizer has:

```text
R is a matched row block;
W contains a matched stride core of size k/m;
the remaining e positions are extras.
```

The corresponding depth-`11`, `m=32` model counts are in:

```text
docs/rfc_near_extremizer_count_depth11_m32.csv
```

Selected values:

```text
e   |W|   log2 count
0    64    11.00000000
1    65    21.95419631
2    66    31.90766527
4    68    50.22745718
8    72    83.31397614
16   80   141.92951259
32   96   243.50842042
64  128   414.59156052
```

This grows, but it grows as a boundary-extra count around the matched family rather than as
arbitrary `binom(k,m)` support counting. That is the near-extremizer stability shape we need.

The slack calculation that combines this count with independent-copy zero tails is:

```text
docs/rfc_near_extremizer_slack_depth11_m32_c8.csv
```

At depth `11`, `m=32`, `c=8`, and `q=2^128`, the worst union term over `0 <= e <= 128` is the exact
case `e=0`:

```text
log2 union bound = -100.60768258.
```

For positive `e`, the first copy has extra output weight, so another copy must create at least
`e+1` zeros to beat the collapse baseline; those terms decay quickly.
