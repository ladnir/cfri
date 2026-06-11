# Upper-Bound Response To Multi-Copy Stress

Scope: original non-systematic RFC only.

This note answers the lower-bound/falsification update in
`rfc_lower_bound_falsification_multicopy.md`. It does not complete the distance certificate, but it
does identify a principled saving for the current seven-copy broad near-family stress row.

## Volta Stress Row

The reported strongest `e=71` row is:

```text
c = 8
k = 2048
q = 2^128
target excess e = 71
active copies r = 7
live_rows = 2048
core support b = 1
extra support h = 1782
one-copy support s = b + h = 1783
one-copy projective dimension D = 1782
generic intersection exponent I = 7*1782 - 6*2047 = 192
deterministic zeros = 7*(2048 - 1783) = 1855
tail zeros in last copy = 264
modeled log2 expected = -76.33662864
```

This misses the 80-bit target by about `3.66` bits if taken literally.

## Upper-Bound Answer

For this exact row, the stress model is using a marked-core support count:

```text
k * binom(k - 1, h)
```

per active copy. With `b=1` and `s=h+1`, this is:

```text
k * binom(k - 1, s - 1) = s * binom(k, s).
```

But the linear event in one copy is indexed by the unmarked output support:

```text
L_A = { message lines whose copy output is supported in A },
|A| = s.
```

For `b=1`, choosing a different core coordinate inside the same support `A` does not give a new
linear support event. It is only a different certificate that `A` contains a one-coordinate core.
The upper bound should union over exact/unmarked supports `A`, not over marked pairs
`(core, extras)`.

Therefore the seven-copy row has an exact-support de-duplication saving of:

```text
r log2(s) = 7 log2(1783) = 75.60063691 bits.
```

Applying only this correction gives:

```text
-76.33662864 - 75.60063691 = -151.93726555.
```

So this specific seven-copy stress row is not a reason to move the certificate to `e=72`.

## Corrected Sweep Check

A small deterministic sweep with only the `b=1` unmarked-support correction gives:

```text
e=70: best modeled log2 expected =    3.41617672
e=71: best modeled log2 expected = -121.83277193
e=72: best modeled log2 expected = -247.08241391
```

After this correction, the best `e=71` row is again a one-copy broad family, not a seven-copy
intersection. This is consistent with the earlier picture:

```text
e=70 remains unsafe in the stress model,
e=71 is the first plausible 80-bit point,
e=72 is comfortable.
```

The sweep is not a certificate. It only says the new seven-copy warning was caused by marked-core
overcounting for `b=1`.

## Proof Obligation

The certificate recurrence needs an exact-support family-count lemma.

For each top-level copy and each support size `s`, the event:

```text
copy-output support is contained in A
```

should be counted by unmarked support sets:

```text
binom(k, s),
```

with exact-support inversion used to avoid counting smaller supports through all supersets. For the
`b=1` broad near-family this replaces:

```text
k * binom(k - 1, s - 1)
```

by:

```text
binom(k, s).
```

For multiple independent copies, the corresponding support-family factor should be:

```text
binom(c, r) * product_i binom(k, s_i)
```

for the unmarked supports, plus the projective intersection/flag count for the common message
lines. The copy-subset factor `binom(c,r)` is real and should not be removed unless a separate
canonical-tail argument is proved.

This saving is a support-overlap correction, not a local exterior theorem. It should live in the
global recurrence next to the flag state:

```text
flags of message subspaces
unmarked exact output supports
visible quotient/kernel zero budgets
```

## Recommendation

Do not move to `e=72` solely because of Volta's seven-copy row. The upper-bound side has a
principled exact-support saving that recovers far more than the missing `3.66` bits for that exact
shape.

Still, do not claim an `e=71` certificate yet. The certificate needs the exact-support family-count
lemma and the flag recurrence to include this de-duplication explicitly. If the multiplicity-
corrected lower-bound search finds a different row above `-80` bits, or if finite constants consume
the corrected `e=71` slack, then `e=72` should become the conservative certificate target.

## Feedback For Lower-Bound Agent

Please rerun the multi-copy stress model with exact-support/unmarked-family counting.

Concrete constraints to test:

1. For `b=1`, replace each per-copy family count:

   ```text
   k * binom(k - 1, h)
   ```

   by:

   ```text
   binom(k, h + 1).
   ```

   This is not optional for an upper-bound-compatible model; the marked core is a duplicate
   certificate of the same support event.

2. Report the best rows at `e=70,71,72` after this correction. The quick upper-bound-side sweep
   predicts:

   ```text
   e=70:    3.41617672
   e=71: -121.83277193
   e=72: -247.08241391
   ```

3. For `b>1`, group by the unmarked output support `A` and by the actual dimension of the support
   subspace, not by every matched-core certificate. The multiplicity to measure is:

   ```text
   # admissible stride cores B subset A.
   ```

   If many core certificates yield the same linear subspace `L_A`, the lower-bound model should
   count `L_A` once.

4. Separate contained-support and exact-support buckets. A line with output support size `w < s`
   must be charged in the `w` bucket, not counted through all `binom(k-w, s-w)` supersets.

5. Keep the `binom(c,r)` active-copy factor for now. I do not see a rigorous way to remove it from
   the upper-bound side without a canonical-tail or overlap lemma.

6. After multiplicity correction, search specifically for:

   ```text
   b > 1 structured stride-core rows,
   dense tau-two rows with g >= 2,
   nested-kernel cascades that survive exact-support de-duplication.
   ```

If any multiplicity-corrected `e=71` row remains above `-80` bits, that is a real signal for the
upper-bound recurrence. If not, the current multi-copy stress supports keeping `e=71` as the active
target while holding `e=72` as the fallback.
