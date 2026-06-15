# Common-Zero One-Spill Ledger

Status: first mixed-shape stress test after the exact all-paired branch.

## Purpose

The exact paired branch shows that pure all-paired common-zero spines are safe once canonical
kernel dimension is enforced. The next mixed shape is:

```text
paired for m lower levels,
then one spill level with paired coordinates plus singleton zero requests.
```

This diagnostic asks whether that first mixed branch is already safe under a one-step exact-rank
random-matrix model.

## Script

Added:

```text
scripts/rfc_distance_analysis/rfc_common_zero_one_spill_ledger.py
```

At the spill level, the compressed zero set has:

```text
p0 paired lower positions
s0 singleton spill positions
z0 = 2 p0 + s0.
```

After `m` exact paired lift levels:

```text
z = 2^m z0
h = 2^m h0.
```

For a lower paired-kernel dimension `D0`, the model charges:

```text
lower paired event:  D0(p0-k1+D0)
spill singleton event: h0(s0-2D0+h0)
top residual root:  t-a-2h+1.
```

The script enforces exact-rank feasibility:

```text
p0 >= k1-D0
h0 <= 2D0
s0 >= 2D0-h0.
```

This is still not a certificate. It uses random-matrix exact-rank exponents at the spill level and
does not yet import RFC-specific mixed root-line structure.

## Deep-Spill Target Run

Command:

```text
python scripts/rfc_distance_analysis/rfc_common_zero_one_spill_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --min-h 1 --min-flat-excess 1 --max-flat-excess 72 \
  --min-lift-levels 5 --top 12
```

Output summary:

```text
log2_sum = 2382.701824
extra q-dimensions needed = 19.239858.
```

Dominant row:

```text
h = 864
F = 41
r = 23
a = 87
z = 224
lift_levels = 5
spill_depth = 5
h0 = 27
z0 = 7
p0 = 3
s0 = 1
D0 = 14
lower_q = 14
spill_q = 0
root_residual_q = 31
total_q = 45
log2_shape = 8142.701824
log2_term = 2382.701824.
```

The shape entropy decomposes as:

```text
lower paired positions:   18.381002 bits
spill singleton position:  6.965784 bits
spill side:                1.000000 bits
P/C labels:              211.686742 bits
outside singletons:     6059.668295 bits
top singleton sides:    1845.000000 bits
```

So the positive mass is not from many lower paired choices. It is the ordinary top survivor-profile
entropy combined with a mixed spill event that pays only:

```text
14 + 0 + 31 = 45
```

q-dimensions.

## Interpretation

The first mixed branch is the new concrete frontier. The dominant row has one singleton at the
spill level. Since:

```text
s0 = 2D0 - h0 = 1,
```

that singleton only needs to have full possible rank in the quotient, so the random exact-rank model
pays no spill q-exponent. Closing this row likely needs one of:

```text
1. an RFC-specific lower-paired rank theorem stronger than the random fixed-set exponent here;
2. a mixed root-line/incidence charge showing the spill singleton is not free after conditioning
   on the lower paired rank defect;
3. full-common-zero maximality or top-profile compatibility that reduces the outside singleton
   entropy for this bucket.
```

The target saving is about:

```text
19.24 q-dimensions.
```

That is much smaller than the old relaxed pure-paired artifact (`56.64` q-dimensions), but it is
still a real gap in this diagnostic.

## Lower Triple-Rank Sanity Check

The dominant row's lower paired event is:

```text
spill depth = 5
k1 = 16
p0 = 3
D0 = 14
```

Equivalently, three compressed RFC columns at depth 4 have rank two instead of rank three. The
one-spill ledger initially charges this by the random-matrix exact-rank exponent:

```text
D0(p0-k1+D0) = 14.
```

This exponent is not yet theorem-safe for RFC tensor columns. A targeted sampler was added:

```text
scripts/rfc_distance_analysis/rfc_tensor_triple_rank_sampler.py
```

It samples only the requested tensor columns. A compact check:

```text
python scripts/rfc_distance_analysis/rfc_tensor_triple_rank_sampler.py \
  --q 5 --q 7 --q 11 --q 31 --q 101 \
  --columns 0,11,25 --columns 0,32,74 --samples 10000
```

gave:

```text
q=5:   rates 0.0097 and 0.0053
q=7:   rates 0.0022 and 0.0010
q=11:  rates 0.0005 and 0.0003
q=31:  no hits in 10000 samples for either triple
q=101: no hits in 10000 samples for either triple.
```

This says the lower event is not a deterministic structural triple dependence, and it disappears
quickly as the field grows. However, the data is much more consistent with a tensor-incidence
codimension around `4` than with the random-vector `q^-14` exponent. Therefore the one-spill row
should be read as:

```text
positive even under an optimistic/random lower-rank exponent.
```

The proof must either justify a strong RFC triple-rank exponent, replace this lower event by a
different tensor-specific bound, or recover the missing q-dimensions elsewhere.

The ledger now supports:

```text
--triple-rank-codim-override 4
```

With this override, the same deep-spill target gives:

```text
log2_sum = 3662.701824
dominant row unchanged
lower_q = 4
total_q = 35
extra q-dimensions needed = 29.239858.
```

This is the more conservative frontier until the tensor triple-rank codimension is proved.

The still sharper structural mode is:

```text
--triple-rank-structural-profile
```

It uses the Segre-line classifier to count only triples in the minimum codimension stratum. For
depth 4, expansion 8, there are `7168` codim-3 triples. The same deep-spill target becomes:

```text
log2_sum = 3785.128176
dominant row unchanged
lower_q = 3
lower_log2_count = 12.807355
total_q = 34
extra q-dimensions needed = 30.196314.
```

This is the best current diagnostic number for the one-spill frontier.

## P/C Label Compatibility Check

The dominant row pays:

```text
log2 binom(224,87) = 211.686742 bits
```

for arbitrary labels of the lifted zero set `Z=P union C` into `P` and `C`. Since this is a visible
entropy term, the script now supports:

```text
--label-model block_constant
```

which requires each `2^lift_levels` lifted block to be entirely `P` or entirely `C`.

For the structural one-spill run:

```text
python scripts/rfc_distance_analysis/rfc_common_zero_one_spill_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --min-h 1 --min-flat-excess 1 --max-flat-excess 72 \
  --min-lift-levels 5 --triple-rank-structural-profile \
  --label-model block_constant --top 12
```

the result is:

```text
row_count = 0.
```

This does not close the row. It says block-constant labels are incompatible with the fixed top
profile. For the target profile:

```text
p = 137
z = 224
lift = 32.
```

If labels were block-constant, both `|P|` and `|C|` inside `Z` would be multiples of `32`; but
`137` is not. Therefore the dominant one-spill obstruction necessarily uses nonconstant `P/C`
labels inside the lifted blocks.

Conclusion: the missing theorem cannot simply assert block-constant labels. Any top-profile or
full-common-zero compatibility saving must handle genuinely mixed labels within the lifted blocks.

## Minority-Label Penalty Target

A focused profiler was added:

```text
scripts/rfc_distance_analysis/rfc_one_spill_label_distribution.py
```

For the dominant row it stratifies:

```text
7 lifted blocks
block size = 32
total C labels = 87
```

by the number of mixed blocks and by:

```text
minority_labels = sum_{mixed blocks} min(C_i, 32-C_i).
```

The raw entropy by mixed-block count is:

```text
mixed=1:  31.455681 bits
mixed=2:  64.740103 bits
mixed=3:  98.044686 bits
mixed=4: 129.068422 bits
mixed=5: 159.526069 bits
mixed=6: 189.474530 bits
mixed=7: 211.686742 bits.
```

So a per-mixed-block penalty is not promising: once any positive q-penalty is applied, the union
concentrates on the minimum mixed-block stratum and still needs roughly `29` q-dimensions on that
one block.

The minority-label stratification is more useful. The minimum possible minority count is:

```text
9.
```

Running:

```text
python scripts/rfc_distance_analysis/rfc_one_spill_label_distribution.py \
  --blocks 7 --block-size 32 --c-total 87 \
  --needed-q 30.196314 --minority-mode \
  --penalty-step 0.25 --max-penalty 6
```

shows that the row closes once the theorem supplies about:

```text
3.25 q-dimensions per minority label.
```

At that penalty, the label-union gain is:

```text
30.549372 q-dimensions.
```

The penalized union concentrates on:

```text
mixed_blocks = 3
minority_labels = 9.
```

A typical extremal shape is three mostly-common-zero blocks, each with about:

```text
29 C labels and 3 P labels.
```

This is now the cleanest formulation of the missing one-spill theorem:

```text
each minority P/C label inside a mixed lifted block should force
about 3.25 q-dimensions of root/full-common-zero/top-profile incompatibility,
or the row remains open.
```

Follow-up local algebra check:

```text
docs/rfc_distance_analysis/rfc_one_spill_minority_label_no_go.md
```

shows that this charge cannot come from local equations inside `Z=P union C`. For `j in P` and
`j in C`, conditioning on `Z` imposes the same equation:

```text
x_j = y_j = 0.
```

So the P/C arrangement inside `Z` is invisible to the child kernel and to the local singleton root
equation. The minority-label target must therefore be interpreted globally: full-common-zero
maximality outside `C`, survivor-profile coupling, witness de-duplication, or a stronger lower
tensor-rank theorem.

## Exact-Maximality Budget

The exact maximality condition:

```text
C(x,y)=C
```

was also checked as a possible source of the missing q-dimensions. The note:

```text
docs/rfc_distance_analysis/rfc_one_spill_exact_maximality_no_go.md
```

records the local accounting. For the dominant row:

```text
t-a = 1758
2h-1 = 1727
(t-a)-(2h-1) = 31.
```

That `31` is exactly the residual root exponent already charged by the ledger. Exact maximality
also requires positions in `T\C` not to be common-zero, but those are nonvanishing inequalities on
the witness pair. They do not add algebraic q-codimension locally.

Thus exact maximality is necessary to justify the existing residual-root charge, but it does not
recover the remaining:

```text
30.196314 q-dimensions.
```

## Canonical-Counting Budget

The next possible recovery was witness de-duplication or exact-support counting: perhaps the ledger
counts the same witness many times through P/C labels, common-zero singleton sides, or selected
outside singleton subsets.

The diagnostic:

```text
scripts/rfc_distance_analysis/rfc_one_spill_canonical_counting_budget.py
```

checks the most aggressive plausible version of this idea. It removes all common-zero side choices,
removes all P/C label entropy inside `Z`, and replaces the selected outside subset count by the
exact binomial tail:

```text
sum_{m>=1758} binom(7968,m) 2^m q^-m.
```

At `q=2^128`, the outside tail is dominated by its first term, so exact outside support counting
saves no q-dimension. Removing common-zero sides and P/C labels saves only:

```text
0.679688 + 1.653803 = 2.333490 q-dimensions.
```

The aggressive canonical model still has:

```text
aggressive canonical log2 term = 3486.441435
remaining gap                  = 27.862824 q-dimensions.
```

The full note is:

```text
docs/rfc_distance_analysis/rfc_one_spill_canonical_counting_no_go.md
```

So sharper canonical counting of the same witness family does not close this row. Any successful
one-spill theorem needs a new algebraic/global q-charge.

## Mixed Spill-Incidence Budget

The remaining local hope was that the spill singleton might not be free after conditioning on the
lower rank-two triple. In the compressed row:

```text
p0 = 3
s0 = 1
D0 = 14
h0 = 27 = 2D0-1.
```

The singleton fails to cut `K_P0 plus K_P0` by rank one only when its lower column is already in
the span of the lower rank-two triple. For rank-one tensor columns this means the fourth column is
forced onto the same Segre line as the dependent triple.

The diagnostic:

```text
scripts/rfc_distance_analysis/rfc_one_spill_mixed_incidence_budget.py
```

checks this forced closure. On the exact codim-3 lower stratum:

```text
triples                         = 7168
forced closure singleton choices = 7168
current log2 count               = 20.773139
corrected log2 count             = 20.761551
saving                           = 0.000091 q-dimensions.
```

Every codim-3 triple has exactly one singleton lower column forced onto the same line. Excluding
that rank-zero choice is only a constant/counting correction, not a q-dimensional incidence charge.
The nearby codim-4 stratum has no forced closure choices.

The full note is:

```text
docs/rfc_distance_analysis/rfc_one_spill_mixed_incidence_no_go.md
```

Therefore mixed root-line incidence at the spill singleton does not close the row.

## Coverage Caveat

A full low-lift sweep (`lift_levels <= 4`) timed out before producing a summary. The deep-spill
range `lift_levels >= 5` is the range connected to the old paired-spine stress and is fully swept
in the command above. Before treating this one-spill ledger as exhaustive, the script should be
pruned or dynamic-programmed enough to cover all lift levels routinely.

## Next Theorem Target

Prove a one-spill mixed branch bound. In proof language:

```text
exact common-zero bucket
  -> exact lower paired kernel D0
  -> singleton spill quotient map
  -> lifted exact h bucket
  -> top residual root constraints.
```

The first target is the dominant row:

```text
(m,h,F,z,h0,z0,p0,s0,D0) = (5,864,41,224,27,7,3,1,14).
```

The proof must find roughly `20` q-dimensions not present in the current one-step random-rank
ledger, or roughly `30.196314` q-dimensions in the current structural triple-rank ledger. The
canonical-counting no-go shows that ordinary witness de-duplication can explain only about
`2.333490` q-dimensions of that structural gap, and the mixed-incidence no-go adds only
`0.000091` q-dimensions. The remaining gap after both is still about:

```text
27.862733 q-dimensions.
```

So the current common-zero one-spill route needs a stronger global framework, not another local
cleanup of the same row.
