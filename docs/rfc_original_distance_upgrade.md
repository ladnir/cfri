# Original RFC Distance Upgrade

This note summarizes the practical impact of the original RFC MDS proof direction.

## Main Upgrade

The old threshold-style non-systematic proof gave a conservative distance floor. The new
leading-monomial certificate proves the stronger generic statement:

```text
every k columns of the original RFC generator are independent.
```

Thus the original RFC is generically MDS:

```text
d_min = N - k + 1.
```

For expansion `c`, where `N = c k`, the asymptotic relative distance is:

```text
delta_MDS = 1 - 1/c.
```

Examples:

```text
rate 1/4, c=4: delta ~= 0.75
rate 1/8, c=8: delta ~= 0.875
```

## Depth-11, c=8 Snapshot

At depth `11`, `k=2048` and `N=16384`.

```text
old non-systematic threshold proof:  0.69299316
systematic threshold proof:          0.57482910
new original MDS distance:           0.87506104
```

So the original MDS proof improves on the old non-systematic threshold bound by:

```text
absolute relative-distance gain: 0.18206788
relative distance gain:          26.27%
```

Against the systematic threshold certificate, the gap is:

```text
absolute relative-distance gain: 0.30023194
relative distance gain:          52.23%
```

## Query Impact

Using the rough soundness proxy:

```text
(1-delta)^q <= 2^-80
```

the implied query counts at depth `11`, expansion `8`, are:

```text
old non-systematic threshold proof: 47 queries
new original MDS distance:          27 queries
```

That is a reduction of about:

```text
20 queries.
```

The full depth-1-to-11 comparison is saved in:

```text
docs/rfc_original_mds_vs_prior_c8_depth1_to_11.csv
```

## Interpretation

The original code now looks essentially optimal from a distance perspective. The old proof was not
tight; it lost distance through the threshold/union-bound proof architecture. The new certificate
uses the actual folding algebra and recovers the Singleton-bound distance.

The systematic all-level code is different. It has real structural rank defects at `z=k+2` in
depth `3`, so its near-MDS target should currently be `k+3` or weaker unless the construction is
changed. This does not affect the original RFC MDS theorem because the obstruction depends on
quotienting out systematic row subtrees, which the original code does not have.
