# Systematic RFC Collapse Family

This note records the explicit structural obstruction found while testing the all-level systematic
RFC certificate.

## Construction

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
