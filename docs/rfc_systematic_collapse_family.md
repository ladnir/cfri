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

At depth `4`, `c=8`, `t=2`, it gives:

```text
k = 16
m = 4
z = 24
rank = 15 over five sampled large-prime challenge assignments.
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
docs/rfc_systematic_collapse_family_depth4_c8_livebits2.csv
docs/rfc_systematic_collapse_family_depth4_c8_livebits2_rank.csv
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

## Optimality Within Live-Subcube Collapses

The generalized family is optimal among collapses of this specific live-subcube form.

Fix a live subcube of size:

```text
m = 2^t.
```

After restricting to that live subcube, every selected parity column factors as:

```text
scalar depending on fixed coordinates
  *
depth-t original RFC column type.
```

The column type is indexed by:

```text
(copy, live_direction).
```

For each selected type there are:

```text
k/m
```

duplicate scalar multiples, one for each complementary parity path. Therefore, to maximize the
number of selected parity columns while keeping restricted rank below `m`, it is enough to maximize
the number of selected depth-`t` original RFC column types whose span has rank below `m`.

By the original RFC MDS theorem, every `m` such types are independent. Hence any rank-deficient type
set has size at most:

```text
m - 1.
```

The generalized collapse construction selects exactly `m-1` types and all `k/m` duplicates of each
type, so within the live-subcube collapse model it is optimal:

```text
max |Q| = (m-1) k/m = k - k/m.
```

Thus:

```text
max z(m) = k - m + k - k/m = 2k - m - k/m.
```

The remaining lower-bound challenge is broader: prove that every systematic structural rank defect
is bounded by this live-subcube envelope, or identify an even more global obstruction. The original
MDS theorem strongly suggests this is the right envelope, because any quotient obstruction must
ultimately come from deleting systematic rows until many parity columns project onto fewer than
their required number of original-RFC column types.

## Gap To Current Lower Certificate

The comparison table:

```text
docs/rfc_systematic_threshold_vs_collapse_ceiling_c8_depth1_to_11.csv
```

compares the old systematic threshold certificate to this collapse-family upper ceiling.

At depth `11`, `c=8`:

```text
old systematic threshold certificate: 0.57482910
collapse-family upper ceiling:        0.75585938
headroom:                             0.18103028
```

So there is still a lot of room for a sharper systematic lower-bound proof. The realistic target is
no longer near-MDS, but it may still be close to the collapse ceiling around `0.75` for `c=8`.

## Proof Route After The Collapse

The cleaner lower-bound route is now separated into:

```text
docs/rfc_systematic_uncertainty_route.md
```

The key reframing is that a systematic rank defect is the same thing as a nonzero message, supported
on the live systematic rows, whose parity outputs vanish on the selected parity coordinates. A
single RFC parity copy should satisfy the sharp uncertainty law:

```text
wt(x) * wt(Ax) >= k.
```

The live-subcube collapse achieves equality for one copy. Therefore the remaining proof problem is
not one-copy algebra; it is multi-copy intersection. To beat the `1-2/c` ceiling, the same message
would have to be simultaneously sparse in two or more independent RFC parity copies. That is the
right place to use a first-moment/kernel-intersection count over the large field.
