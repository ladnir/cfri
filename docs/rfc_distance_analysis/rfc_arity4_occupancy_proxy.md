# Arity-4 Occupancy Proxy

Question: does true arity 4 change the bad mode, or does it merely push the same binary
obstruction out by one level?

This note records a small combinatorial proxy. It is not a distance proof. It checks whether, under
mixed `P/A` occupancy constraints, the large-count profiles naturally cover whole arity-4 blocks.

## Proxy

For an arity `b` local block, each local output can be:

```text
unused
P
A
```

with P and A disjoint. For each block define:

```text
ell = number of local outputs covered by P union A.
```

The old binary full-span obstruction appears when an A-active block has:

```text
ell = b.
```

For binary `b=2`, a mixed block with one P and one A automatically has `ell=2`. For arity 4, one P
and one A has only `ell=2<4`.

The relevant PA-conditioned proxy requires every A-active block to also contain at least one P
output. This prevents the unrestricted combinatorial model from separating A-only and P-only
blocks, which is not the mixed-PA obstruction we are testing.

Added script:

```text
scripts/rfc_distance_analysis/rfc_arity_occupancy_proxy.py
```

It enumerates local occupancy profiles and groups them by:

```text
ell_sum over A-active blocks
number of full-cover A-active blocks
number of A-active blocks
```

The output includes the total log2 profile count, the group log2 mass, and a representative
highest-mass local shape decomposition.

## Binary Baseline

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity_occupancy_proxy.py \
  --arity 2 --blocks 16 --p-total 8 --a-total 8 \
  --require-p-for-a --top 8
```

Output:

```text
ell_sum=16, full_cover_a_blocks=8, a_active_blocks=8,
avg_ell_on_a=2, full_cover_fraction=1,
best=(0,0)x8 (1,1)x8
```

So the PA-conditioned binary profile is forced into full cover. This matches the old obstruction:
mixed PA immediately covers the whole local block.

## Arity 4, Same P/A Budget

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity_occupancy_proxy.py \
  --arity 4 --blocks 16 --p-total 8 --a-total 8 \
  --require-p-for-a --top 10
```

Dominant group:

```text
ell_sum=14, full_cover_a_blocks=0, a_active_blocks=6,
avg_ell_on_a=2.333333, full_cover_fraction=0,
best=(0,0)x8 (1,0)x2 (1,1)x4 (1,2)x2
```

The top several groups all have:

```text
full_cover_fraction = 0.
```

This says the high-count mixed profiles remain sparse. They do not naturally recreate the binary
full-span obstruction.

## Arity 4, Extra P Budget

Maybe the obstruction reappears once the profile has enough P requests to cover more siblings.

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity_occupancy_proxy.py \
  --arity 4 --blocks 16 --p-total 16 --a-total 8 \
  --require-p-for-a --top 10
```

Dominant group:

```text
ell_sum=16, full_cover_a_blocks=0, a_active_blocks=7,
avg_ell_on_a=2.285714, full_cover_fraction=0,
best=(0,0)x1 (1,0)x8 (1,1)x5 (1,2)x1 (2,1)x1
```

Even with twice as many P requests as A requests, the dominant groups still do not force full
cover. They mostly keep `ell` near 2 or 3.

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity_occupancy_proxy.py \
  --arity 4 --blocks 16 --p-total 24 --a-total 8 \
  --require-p-for-a --top 10
```

Dominant group:

```text
ell_sum=17, full_cover_a_blocks=0, a_active_blocks=7,
avg_ell_on_a=2.428571, full_cover_fraction=0,
best=(1,0)x3 (1,1)x4 (1,2)x1 (2,0)x6 (2,1)x2
```

The pure full-cover mode exists:

```text
ell_sum=32, full_cover_a_blocks=8, a_active_blocks=8,
avg_ell_on_a=4, full_cover_fraction=1,
best=(0,0)x8 (3,1)x8
```

but its group log2 mass is:

```text
29.651724
```

while the total profile log2 mass is:

```text
81.471573.
```

So the all-full-cover mode is about:

```text
51.819849 bits
```

below the total occupancy mass and far below the dominant sparse groups in this small proxy.

## Interpretation

This supports the view that true arity 4 changes the mode, rather than merely pushing the same
obstruction out one level.

Binary PA-conditioned occupancy:

```text
mixed A-active block -> full cover automatically.
```

Arity-4 PA-conditioned occupancy:

```text
mixed A-active block -> sparse cover by default.
full cover requires extra local zero requests.
```

The old obstruction is still present as a boundary mode:

```text
P_count + A_count = 4 on the same local block.
```

But it is not the combinatorial default, even with extra P budget. The first-moment optimum would
have to deliberately buy full-cover blocks.

## Caveat

This proxy does not include finite-field rank charges. The full first-moment comparison must weigh:

```text
occupancy entropy
rank-deficiency charge for covered subblocks
recursive charge for full-cover blocks
```

The current evidence only says that the old obstruction is not free at arity 4. The next step is to
add a simple q-exponent model:

```text
score = occupancy entropy - q_log2 * covered-deficiency charge
```

and sweep whether any plausible charge model moves the optimum from sparse `ell=2,3` to full
`ell=4`.
