# Systematic RFC Collapse Family

This note records the explicit structural obstruction found while testing the all-level systematic
RFC certificate.

## First Construction: Two Live Rows

Fix depth `d`, total expansion `c`, and parity expansion `c_p = c-1`. Let:

```text
k = 2^d.
```

Choose a suffix:

```text
r in {0,1}^{d-1}.
```

Keep only the two systematic rows:

```text
(0,r), (1,r).
```

Equivalently, put every other systematic coordinate into the zero set:

```text
|S| = k - 2.
```

Now choose one parity copy `a` and select every parity path whose first bit is `0`:

```text
Q = { (a, (0,y)) : y in {0,1}^{d-1} }.
```

Thus:

```text
|Q| = 2^(d-1) = k/2.
```

The total zero-set size is:

```text
|S| + |Q| = k - 2 + k/2 = 3k/2 - 2.
```

## Why It Collapses

After quotienting by the selected systematic coordinates, only two rows remain. For every selected
parity column, the lower `d-1` tensor factors evaluate at the fixed suffix `r`, contributing a
column-dependent scalar. The first tensor factor is always the same local vector because every
selected parity path has first bit `0`.

Therefore every selected parity column restricts to a scalar multiple of the same two-entry vector:

```text
u_0(T_a) = (1-T_a, T_a).
```

So:

```text
rank M_d(S,Q) = 1
```

even though two rows remain. Hence:

```text
rank(I_S, P_Q) = k - 1 < k.
```

This is a deterministic structural rank defect, not an accidental field evaluation.

## Distance Consequence

This gives a codeword with at least:

```text
z = 3k/2 - 2
```

zero coordinates, so the all-level systematic code has:

```text
d_min <= N - z = c k - (3k/2 - 2).
```

Asymptotically:

```text
delta_sys <= 1 - 3/(2c).
```

For the current `c=8` setting:

```text
delta_sys <= 1 - 3/16 = 13/16 = 0.8125.
```

For `c=4`:

```text
delta_sys <= 1 - 3/8 = 5/8 = 0.625.
```

This rules out a constant-additive near-MDS theorem for the all-level systematic construction.
The earlier `e=3` hypothesis was therefore too optimistic; the additive loss is at least:

```text
k/2 - 2.
```

## Checked Examples

The helper:

```text
scripts/rfc_systematic_collapse_family.py
```

emits this family. For depth `5`, `c=8`, it produces:

```text
k = 32
|S| = 30
|Q| = 16
z = 46
```

The sampled rank check:

```text
docs/rfc_systematic_collapse_family_depth5_c8_rank.csv
```

shows rank `31` across five independent large-prime challenge assignments.

The same pattern explains the previously found depth-3 and depth-4 high-`z` examples:

```text
depth 3: z = 8 - 2 + 4 = 10 = k + 2
depth 4: z = 16 - 2 + 8 = 22 = k + 6
depth 5: z = 32 - 2 + 16 = 46 = k + 14
```

## Interpretation

The all-level systematic code is still much better than the old threshold certificate in many
parameter regimes, but its true distance cannot be close to MDS. The obstruction is intrinsic to
the all-level systematic structure: deleting almost all systematic rows leaves a two-row quotient,
and a half-tree of parity columns collapses to one projective direction on that quotient.

This does not affect the original non-systematic RFC MDS proof, because the obstruction relies on
quotienting out systematic rows.

## Generalized Live-Subcube Construction

The two-row family is not the worst version. Let:

```text
m = 2^t
```

for some `1 <= t <= d`. Keep a live row subcube of size `m`:

```text
R = { (x, r) : x in {0,1}^t }
```

where `r` fixes the remaining `d-t` row bits. Put every other systematic coordinate into the zero
set:

```text
|S| = k - m.
```

Choose one parity copy `a`. Among the `m` parity directions on the live `t` coordinates, omit one
direction and keep the other `m-1`; over the complementary `d-t` parity coordinates, keep all
possibilities:

```text
Q = { (a, (x,y)) : x in {0,1}^t \ {x_*}, y in {0,1}^{d-t} }.
```

Then:

```text
|Q| = (m-1) 2^(d-t) = (m-1) k/m = k - k/m.
```

On the live row subcube, each selected parity column is a scalar multiple of one of the `m-1`
selected depth-`t` tensor basis directions. Omitting one direction leaves rank at most `m-1` on
`m` live rows. Therefore:

```text
rank M_d(S,Q) <= m-1
rank(I_S, P_Q) <= k-1.
```

The zero-set size is:

```text
z(m) = |S| + |Q|
     = k - m + k - k/m
     = 2k - m - k/m.
```

This is maximized when `m` is the power of two closest to `sqrt(k)`. Consequently:

```text
z >= 2k - O(sqrt(k)).
```

and the all-level systematic distance satisfies:

```text
d_sys <= N - 2k + O(sqrt(k)).
```

Asymptotically:

```text
delta_sys <= 1 - 2/c.
```

For `c=8`:

```text
delta_sys <= 1 - 1/4 = 0.75.
```

For `c=4`:

```text
delta_sys <= 1 - 1/2 = 0.5.
```

So the stronger obstruction is a `k - O(sqrt(k))` additive loss from original MDS, not merely
`k/2`.

## Generalized Checks

The helper now emits the generalized construction via:

```text
--live-bits t
```

At depth `5`, `c=8`, `t=2`, it gives:

```text
k = 32
m = 4
z = 52
rank = 31 over five sampled large-prime challenge assignments.
```

At depth `6`, `c=8`, `t=3`, it gives:

```text
k = 64
m = 8
z = 112
rank = 63 over three sampled large-prime challenge assignments.
```

The artifacts are:

```text
docs/rfc_systematic_collapse_family_depth5_c8_livebits2.csv
docs/rfc_systematic_collapse_family_depth5_c8_livebits2_rank.csv
docs/rfc_systematic_collapse_family_depth6_c8_livebits3.csv
docs/rfc_systematic_collapse_family_depth6_c8_livebits3_rank.csv
```

The ceiling table:

```text
docs/rfc_systematic_collapse_ceiling_c8_depth1_to_11.csv
```

optimizes `m` over powers of two for each depth. At depth `11`, `c=8`, it gives:

```text
k = 2048
best live rows m = 32
zero count z = 4000
distance upper bound = 12384
relative distance upper bound = 0.75585938
```

## Revised Interpretation

The all-level systematic construction still may improve substantially over the old threshold
certificate, but it cannot approach the original MDS distance. The best possible asymptotic
relative distance for this exact all-level systematic structure is at most:

```text
1 - 2/c.
```

For `c=8`, this means an upper ceiling near `0.75`, compared to original MDS near `0.875`.
