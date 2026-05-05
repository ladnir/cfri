# Systematic RFC Distance Certificate Outline

This note states the current conditional certificate in a form that can be turned into a full proof.

## Target

For total expansion:

```text
c = c_p + 1
```

and block dimension:

```text
k = 2^d,
```

the all-level systematic RFC has the explicit collapse upper bound:

```text
d_sys <= (c_p-1)k + min_m (m + k/m)
```

where `m` ranges over powers of two. Thus:

```text
d_sys <= (c_p-1)k + 2 sqrt(k) + O(1).
```

Equivalently:

```text
delta_sys <= 1 - 2/c + O(1/sqrt(k)).
```

The proof target is the matching lower bound with a small safety slack:

```text
d_sys >= (c_p-1)k + 2 sqrt(k) - slack(d,c,lambda).
```

For `c=8`, this means a distance certificate near:

```text
0.75.
```

## Lemma Stack

The certificate should follow from four lemmas.

### Lemma 1: One-Copy Uncertainty

For one RFC parity copy `A`:

```text
wt(x) * wt(Ax) >= k
```

over the generic/rational-function model, with finite-field root losses charged separately.

This gives the kernel-dimension envelope:

```text
dim { x in F^R : (Ax)|_Q = 0 }
  <= max(0, |R| + 1 - ceil(k/(k-|Q|))).
```

### Lemma 2: Exact Extremizer Stability

If equality holds in the one-copy uncertainty bound:

```text
wt(x) = m
wt(Ax) = k/m,
```

then `m` is a power of two and the live input support and surviving output support are a matched
block/stride pair:

```text
R = { s m, s m + 1, ..., s m + m - 1 }
W = { r, r + m, r + 2m, ..., r + (k/m - 1)m }.
```

There are exactly:

```text
k
```

such exact extremizer support pairs for each `m`.

Equivalently, at the exact boundary `|R| |W| = k`, the desired rank statement is:

```text
dim ker(A[R, [k]\W]) =
  1 if (R,W) is a matched block/stride pair,
  0 otherwise.
```

The constructive matched half is:

```text
docs/rfc_matched_kernel_induction.md
```

The small-depth scans support this:

```text
depth 3, m=2: all 8 extremizers matched
depth 3, m=4: all 8 extremizers matched
depth 4, m=2: all 16 extremizers matched
depth 4, m=4: all 16 extremizers matched
```

### Lemma 3: Multi-Copy Kernel Intersection

For independent parity copies, extremal or near-extremal one-copy kernels intersect like independent
subspaces.

For exact extremizer lines on a live support of size `m`, two copies both being extremal for the
same nonzero message costs:

```text
q^{-(m-1)}.
```

The matched-family union bound through depth `11`, `c=8`, and `q=2^128` has worst term:

```text
2^-111.60768258
```

at `m=2`, and the distance-dominant `m=32` term is:

```text
2^-3947.60768258.
```

### Lemma 4: Non-Extremal Copy Density

Conditioned on a message coming from one extremal copy, every other independent copy has full
generic support except with a root/determinant event. For a fixed nonzero vector `x` independent of
copy `j`, a crude union tail is:

```text
Pr[wt(A_j x) <= b] <= binom(k,b) q^{-(k-b)}.
```

Informally:

```text
wt(A_j x) = k
```

for all `j` not responsible for the extremal collapse, outside negligible finite-field failures.

This lemma is the part that still needs the cleanest algebraic statement. It should be easier than
the first copy: after conditioning on `x`, the challenges of another copy are independent, and no
matched block/stride zero pattern has been imposed on that copy.

The depth-`11`, `q=2^128` conditioned-copy tail is:

```text
docs/rfc_conditioned_copy_tail_depth11_q128.csv
```

Some entries:

```text
output weight <= 1984: log2 tail <= -7785.43020642
output weight <= 2047: log2 tail <=  -117.00000000
```

So even a single zero in an independent second copy is already below an 80-bit target for one fixed
message vector. At the distance-dominant depth-`11`, `m=32` point, multiplying by the rough matched
candidate count:

```text
c_p * k * m
```

costs only about `18.8` bits, still leaving roughly `98` bits of slack for the one-zero event.

The first sanity checks are:

```text
docs/rfc_extremizer_second_copy_check_depth4_m2.csv
docs/rfc_extremizer_second_copy_check_depth4_m4.csv
```

They extract the one-dimensional kernel vector for each depth-`4` exact extremizer of one copy and
evaluate that same vector in an independent second copy. For both `m=2` and `m=4`, all `16`
extracted vectors have full second-copy output support. The `m=4` row is:

```text
first-copy output weight:   4
second-copy output weight: 16
```

and the `m=2` row is:

```text
first-copy output weight:   8
second-copy output weight: 16
```

## Consequence

If the four lemmas hold, then every nonzero codeword has, with overwhelming probability:

```text
wt(x) + sum_{i=1}^{c_p} wt(A_i x)
  >= m + k/m + (c_p-1)k
```

unless two or more copies share a nonzero kernel vector, which is charged by Lemma 3.

Optimizing over `m` gives:

```text
d_sys >= (c_p-1)k + 2 sqrt(k) - slack.
```

This matches the explicit generalized collapse family up to slack, and is the desired distance
certificate structure.

## Next Real Issue

The biggest remaining proof risk is not the aligned collapse family. It is near-extremal leakage:

```text
wt(Ax) = k/m + e
```

for moderate `e`. The kernel envelope controls the dimension as `e` grows, but the final proof must
sum those near-extremal kernel dimensions across copies. The right next object is therefore a
near-extremizer stability/counting lemma, not another threshold recurrence.

The first exact near-extremizer scans support the strongest simple counting model:

```text
near-extremizer count for |W|=k/m+e:
  k * binom(k-k/m, e).
```

That is, choose one of the `k` matched exact block/stride cores and then choose `e` extra output
positions outside the core. Depth-`4` checks match this count for:

```text
m=4,e=1: 192
m=2,e=1: 128
m=2,e=2: 448
```

For the depth-`11`, `m=32` distance-dominant point, this model is tabulated in:

```text
docs/rfc_near_extremizer_count_depth11_m32.csv
```

Combining this count with the conditioned-copy zero tail gives:

```text
docs/rfc_near_extremizer_slack_depth11_m32_c8.csv
```

Here `e` extra outputs in the first sparse copy require at least `e+1` zeros in some independent
copy to beat the collapse baseline. The resulting union terms are:

```text
e    log2 union bound
0     -100.60768258
1     -207.65419088
2     -316.28709399
4     -536.29416556
8     -982.78823448
16   -1890.11183560
32   -3735.18084799
64   -7488.51449998
128 -15125.78793032
```

So, under the near-extremizer counting model, the dangerous term is still the exact extremizer case,
and even that has about `100` bits of slack in this depth-`11`, `c=8`, `q=2^128` calculation.
