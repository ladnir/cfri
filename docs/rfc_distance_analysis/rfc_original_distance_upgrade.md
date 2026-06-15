# Original RFC Distance Upgrade Candidate

This note summarizes the practical impact if the original RFC MDS direction can be upgraded from a
fixed-subset generic-minor statement to a sampled-code theorem.

## Main Upgrade

The old threshold-style non-systematic proof gave a conservative distance floor. The new
leading-monomial certificate may prove the stronger fixed-subset generic statement:

```text
for every fixed k-column set Q, det(G_Q) is a nonzero polynomial.
```

This is not yet the same as sampled MDS. Sampled MDS requires every `k`-column subset to be full
rank for the same sampled diagonal challenges. The BaseFold paper reports RFC distance bounds below
Singleton, so this distinction must be resolved before using MDS query counts.

If sampled MDS holds, the original RFC distance is:

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

## Depth-11, c=8 Candidate Snapshot

At depth `11`, `k=2048` and `N=16384`.

```text
old non-systematic threshold proof:   0.69299316
systematic threshold proof:           0.57482910
candidate original MDS distance:      0.87506104
```

If sampled MDS holds, it improves on the old non-systematic threshold bound by:

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
old non-systematic threshold proof:  47 queries
candidate original MDS distance:     27 queries
```

That is a reduction of about:

```text
20 queries.
```

The full depth-1-to-11 comparison is saved in:

```text
docs/rfc_distance_analysis/rfc_original_mds_vs_prior_c8_depth1_to_11.csv
```

## Interpretation

The original code looks structurally stronger than the old threshold proof shows. The old proof
loses distance through the threshold/union-bound proof architecture. The new certificate uses the
actual folding algebra, but currently supports only the fixed-subset/generic-minor direction unless
we add an all-subsets sampled-code argument.

The systematic all-level code is different. It has an explicit live-subcube collapse family with:

```text
z = 2k - O(sqrt(k))
delta_sys <= 1 - 2/c.
```

For `c=8`, that puts the systematic all-level ceiling near `0.75`, still well above the old
threshold certificate but meaningfully below the candidate original MDS distance. This systematic
obstruction does not directly transfer to the original code because it depends on quotienting out
systematic row subtrees. It also does not settle sampled MDS for the original code.
