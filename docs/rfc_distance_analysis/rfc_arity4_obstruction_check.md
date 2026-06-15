# Arity-4 Obstruction Check

Question: if we replace the binary RFC block by a true arity-4 interpolation block, do the same
mixed-PA obstructions remain?

## Short Answer

The exact binary obstruction does **not** remain for the sparse one-P/one-A profile. It becomes a
weaker covered-subblock obstruction.

But an analogous obstruction **does** remain whenever the P/A profile collectively covers all four
local outputs of an arity-4 block. In that case low A-rank modulo P is again just full local span
deficiency.

So arity 4 changes the geometry in a useful way, but it does not make rank obstructions disappear.
It moves the proof object from binary root-line/full-two-copy span to ranks of covered
interpolation subblocks.

## Local Model

For each child coordinate `j`, a true arity-4 block has four child-copy columns:

```text
e_0(h_j), e_1(h_j), e_2(h_j), e_3(h_j) in H^4.
```

The parent stores four invertible interpolation outputs:

```text
o_h(j) = sum_s M_{h,s} e_s(h_j),  h = 0,1,2,3,
```

where `M` is the local 4-by-4 Reed-Solomon/Lagrange matrix. For this local rank question, only
invertibility and the chosen output subsets matter.

Given a previously conditioned span `U`, let:

```text
P = selected P outputs,
A = selected A outputs,
covered = P union A,
full = all four child-copy directions.
```

The marked increment is:

```text
rank_A = rank(U + P + A) - rank(U + P).
```

This always satisfies the identity:

```text
rank_A = covered_rank - p_rank.
```

The binary case has only two outputs, so for a mixed PA pair:

```text
covered_rank = full_rank.
```

That is why the binary all-mixed PA event reduced exactly to full two-copy span deficiency.

For arity 4 with only one P output and one A output:

```text
covered_rank != full_rank
```

in general. The two untouched local outputs can still carry the missing child-copy directions.

## Diagnostic Script

Added:

```text
scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py
```

It samples small prime fields and reports:

```text
child_rank
p_rank
a_rank
covered_rank
full_rank
```

The script supports either a true 4-point Reed-Solomon interpolation matrix or a random invertible
4-by-4 local matrix.

## Sparse One-P/One-A Profile

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py \
  --q 11 --k 6 --mixed 3 --u-rows 3 --instances 200 \
  --matrix rs --p-indices 0 --a-indices 1
```

Output summary:

```text
child_rank,p_rank,a_rank,covered_rank,full_rank,count
2,3,3,6,12,22
3,3,3,6,12,178
```

Interpretation:

```text
rank_A = covered_rank - p_rank = 6 - 3 = 3,
but full_rank = 12.
```

So the sparse mixed profile sees a full-rank two-output covered subblock while the full arity-4
block has twice as much rank. The old binary full-span obstruction is absent here.

A random invertible local matrix gives the same qualitative result:

```text
python scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py \
  --q 7 --k 6 --mixed 3 --u-rows 3 --instances 200 \
  --matrix random --p-indices 0 --a-indices 1
```

Most samples have:

```text
p_rank=3, a_rank=3, covered_rank=6, full_rank=12.
```

## Full-Cover Profile

Command:

```text
python scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py \
  --q 11 --k 6 --mixed 3 --u-rows 3 --instances 200 \
  --matrix rs --p-indices 0,1,2 --a-indices 3
```

Output summary:

```text
child_rank,p_rank,a_rank,covered_rank,full_rank,count
2,9,3,12,12,22
3,9,3,12,12,178
```

Now:

```text
covered_rank = full_rank,
rank_A = full_rank - p_rank.
```

This is the direct arity-4 analogue of the binary all-mixed obstruction. It appears only once P
and A collectively cover all local output directions.

The same happens for a two-P/two-A split:

```text
python scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py \
  --q 11 --k 6 --mixed 3 --u-rows 3 --instances 200 \
  --matrix rs --p-indices 0,1 --a-indices 2,3
```

Output summary:

```text
child_rank,p_rank,a_rank,covered_rank,full_rank,count
2,6,6,12,12,22
3,6,6,12,12,178
```

Again, covered equals full.

## Low-Dimensional Stress

With `k=2`, sparse one-P/one-A can have low marked rank:

```text
python scripts/rfc_distance_analysis/rfc_arity4_obstruction_profile.py \
  --q 11 --k 2 --mixed 3 --u-rows 1 --instances 200 \
  --matrix rs --p-indices 0 --a-indices 1
```

Output summary:

```text
child_rank,p_rank,a_rank,covered_rank,full_rank,count
1,1,2,3,7,54
1,2,1,3,7,52
1,2,2,4,7,94
```

This is not a full-block collapse. It is a deficiency of the two-output covered subblock while the
full arity-4 block still has rank 7.

For full-cover P/A on the same stress:

```text
child_rank,p_rank,a_rank,covered_rank,full_rank,count
1,5,2,7,7,151
1,6,1,7,7,49
```

the full-span identity is back.

## Implication For The Proof

Arity 4 seems genuinely different from binary in the important sparse mixed profile.

Binary:

```text
one P sibling + one A sibling = whole local block
low A-rank -> full two-copy span deficiency
```

True arity 4:

```text
one P output + one A output = half of local output space
low A-rank -> covered two-output interpolation-subblock deficiency
```

The untouched outputs mean the event is less recursively rigid. That is promising for a
first-moment proof, because the bad structure is no longer automatically an all-paired
full-copy compression event.

But the obstruction reappears in profiles where the zero requests cover enough siblings:

```text
P_count + A_count = 4.
```

Therefore a proof for arity 4 should track:

```text
covered local dimension ell = |P_indices union A_indices|
```

not just whether a block is mixed. The recurrence state would stratify local blocks by
`ell = 1,2,3,4`. Only `ell=4` has the old full-block obstruction.

## Current Verdict

Arity 4 does not merely rename the old obstruction. It weakens the dominant sparse mixed-PA
geometry from full-block span deficiency to covered-subblock span deficiency.

This is a real reason to keep exploring true larger blocks. The next useful test is a first-moment
top-profile comparison using a covered-dimension state:

```text
binary:      ell in {1,2}; mixed PA immediately has ell=2=arity
arity 4:     ell in {1,2,3,4}; sparse mixed PA has ell=2<arity
```

If the top bad profiles mostly remain at `ell=2`, arity 4 may give a meaningful proof advantage.
If the first moment shifts mass to `ell=4`, the same obstruction returns with wider blocks.
