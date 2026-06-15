# Common-Zero Subcode Replacement Theorem

Status: theorem target for turning the local bucket wins into an original-proof replacement.

## Purpose

The refined h-bucket ledger showed:

```text
each canonical common-zero bucket is cheap,
and the sum of all target buckets is cheap.
```

This note states the local replacement theorem suggested by that evidence. If proved, it replaces
the original one-minor repair bound for a fixed survivor profile by a canonical common-zero
decomposition:

```text
bad-root witnesses
  -> full common-zero set C(x,y)
  -> child subcode-zero event on P union C
  -> residual root constraints outside C.
```

Unlike the earlier h=1 split, this has no complement. Every bad-root witness has a full common-zero
set.

## Setup

Fix one parent fold and a child survivor profile:

```text
P = paired child positions
T = singleton child positions
p = |P|
t = |T|
K_P = ker(ev_P)
D = dim K_P.
```

For a root-line repair failure, there is a nonzero projective witness:

```text
(x,y) in K_P plus K_P
```

such that for each singleton coordinate `i in T`:

```text
ell_i(x) + alpha_i ell_i(y) = 0.
```

Define the full common-zero set:

```text
C(x,y) = { i in T : ell_i(x)=0 and ell_i(y)=0 }.
```

This is canonical. Therefore:

```text
bad-root witnesses = disjoint union over exact full sets C.
```

For counting, the exact bucket for `C` is upper-bounded by the larger event where all coordinates
in `C` are common-zero; we do not need to enforce that coordinates outside `C` are non-common-zero.

## Local Bucket Bound

Fix:

```text
C subset T
a = |C|
H_C = ker(ev_{P union C})
h = dim H_C.
```

A projective bad witness with common-zero set containing `C` lies in:

```text
H_C plus H_C.
```

The number of projective witness lines inside `H_C plus H_C` is at most:

```text
O(q^{2h-1}).
```

Every singleton coordinate outside `C` imposes at most one root value for each fixed witness line,
so the residual root probability contributes:

```text
q^{-(t-a)}
```

up to nonzero-root normalization constants. Thus after a child event with kernel dimension `h`, the
residual root exponent is:

```text
t - a - 2h + 1.
```

If this quantity is negative, the corresponding profile violates the Hall/generic repair
condition and must be handled as a deterministic child-rank obstruction. For the near-MDS target
profile we focus on Hall-compatible buckets where:

```text
t - a - 2h + 1 >= 0.
```

## Subcode-Zero Charge

The child event is:

```text
dim ker(ev_{P union C}) >= h.
```

Equivalently, there exists an `h`-dimensional child message subspace vanishing on:

```text
Z = P union C.
```

Let:

```text
S_{d-1}(h,z)
```

denote the child first moment for h-dimensional subspaces vanishing on a zero set of size `z`.
Random-code scale predicts:

```text
S_{d-1}(h,z) ~= q^{-h(z-k_child+h)}
```

after the zero set is fixed.

The local bucket contribution for fixed sizes is therefore bounded by:

```text
choose P, C, T\C, singleton sides
times S_{d-1}(h,p+a)
times q^{-(t-a-2h+1)}.
```

In q-exponent form, the random-code model gives:

```text
h(p+a-k_child+h) + t-a-2h+1.
```

This is exactly the exponent used by:

```text
scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py
```

## Flat-Excess Coordinates

It is often convenient to write:

```text
r = rank(C modulo P) = D-h
F = a - 2r.
```

Then:

```text
a = 2(D-h)+F
t-a-2h+1 = t - 2D + 1 - F = S-F.
```

The root residual depends only on flat excess `F`, while the child subcode-zero exponent grows with
`h`.

For the target profile:

```text
p = 137
t = 1845
D = 887
S = 72
k_child = 1024.
```

The random-model q-exponent is:

```text
h(p + 2(D-h)+F - k_child + h) + S-F.
```

The dominant bucket over all:

```text
h = 1..887
F = 1..72
```

is:

```text
h = 1
F = 1
total exponent = 958.
```

## Target Ledger

Command:

```text
python scripts/rfc_distance_analysis/rfc_original_vs_refined_profile_ledger.py \
  --depth 11 --expansion 8 --q-log2 128 --excess 71 \
  --all-h --min-flat-excess 1 --max-flat-excess 72 \
  --summary-only --top 8
```

Output summary:

```text
old_full_log2              = 8973.439028
old_same_bucket_logsum     = 10823.608953
refined_bucket_logsum      = -113097.934786
gain_vs_old_same_logsum    = 123921.543739
refined_logsum-old_full    = -122071.373814
dominant bucket            = h=1,F=1
```

So the all-bucket replacement is numerically much stronger than the original one-minor row at the
target profile, assuming the child subcode-zero bound is available.

## Paired-Envelope Red Flag

The child subcode-zero bound cannot be a uniform fixed-set random-matrix statement. RFC has exact
paired compression. If `P union C` is all-paired inside the child code, then:

```text
(h,z) -> (ceil(h/2), z/2)
```

at the next lower level.

The ledger stress mode:

```text
--subcode-model paired_envelope
```

applies this all-paired exponent pessimistically while still using arbitrary-set entropy. It gives
a losing target row:

```text
refined_bucket_logsum = 10672.925272
dominant bucket       = h=479,F=71,z_child=1024,total_q=2.
```

The follow-up paired-shape ledger counts pure all-paired sets with reduced lower-level entropy.
It originally used the relaxed compression `ceil(h/2^m)`, which is valid for a loose
`dim ker >= h` event but not for a canonical exact-`h` bucket. For a pure all-paired shape, rank
doubles at every lifted level, so exact kernel dimension satisfies:

```text
h = 2^m h_compressed.
```

After enforcing this exact all-paired lift, the full-depth pure-spine stress is safe:

```text
log2_sum = -912.890257
dominant bucket:
  h=768,F=9,z=384,paired_levels=7
  compressed depth=3, compressed z=3, compressed h=6
  total_q=69
  extra q-dimensions needed for an 80-bit target=0.
```

The relaxed mode still shows a positive row:

```text
h=735,F=71,z=512,paired_levels=8
compressed h=3
log2_term=7169.731801.
```

but this is an overcounting artifact: compressed `h'=3` lifts to actual `h=768`, not `h=735`, and
therefore has a different flat-excess/root-residual bucket.

Thus the theorem can close the pure all-paired branch with an exact-lift lemma. The missing
statement remains shape-recursive, but the hard part is now mixed shapes rather than pure paired
spines:

```text
C(x,y) bucket
  + paired-spine depth m
  + compressed lower support U
  + labels of lifted P and C inside the spine
  + mixed singleton/root-line spill outside the pure spine
```

and must charge exact kernel dimensions consistently across paired and singleton parts.

The first mixed diagnostic is:

```text
scripts/rfc_distance_analysis/rfc_common_zero_one_spill_ledger.py
```

For deep one-spill shapes (`lift_levels >= 5`), it finds:

```text
log2_sum = 2382.701824 under the optimistic random lower charge
dominant bucket:
  h=864,F=41,z=224,lift_levels=5
  compressed spill profile h0=27,z0=7,p0=3,s0=1,D0=14
  lower_q=14, spill_q=0, root_residual_q=31, total_q=45
  extra q-dimensions needed for an 80-bit target=19.239858.
```

The lower event is a three-column RFC tensor rank drop. Sampling suggests its codimension is closer
to `4` than to the random-matrix value `14`; with `--triple-rank-codim-override 4`, the same row has:

```text
log2_sum = 3662.701824
lower_q=4, spill_q=0, root_residual_q=31, total_q=35
extra q-dimensions needed for an 80-bit target=29.239858.
```

The structural Segre-line classifier is sharper: it stratifies depth-4 triples by local
projective-equality codimension and finds `7168` codim-3 triples. With
`--triple-rank-structural-profile`, the same row has:

```text
log2_sum = 3785.128176
lower_q=3, lower_log2_count=12.807355
spill_q=0, root_residual_q=31, total_q=34
extra q-dimensions needed for an 80-bit target=30.196314.
```

So the next proof target is a one-spill mixed branch lemma. It must recover roughly `30.2`
q-dimensions on the row above, prove a stronger tensor triple-rank codimension for the exact
canonical triples, or identify another canonical-bucket overcount analogous to the relaxed
pure-paired artifact.

The first tempting label-based recovery is not local. The label distribution profiler shows that a
penalty of about `3.25` q-dimensions per minority P/C label would close the row, but
`rfc_one_spill_minority_label_no_go.md` observes that inside:

```text
Z = P union C
```

both labels impose the same conditioned equation:

```text
x_j = y_j = 0.
```

Thus the P/C arrangement inside `Z` is invisible to the local child kernel and singleton root
equations. Any label-dependent saving must come from exact full-common-zero maximality outside `C`,
survivor-profile coupling, or witness de-duplication, not from a local root-line charge inside the
lifted block.

Exact maximality itself was then checked locally. For the dominant row:

```text
t-a = 1758
2h-1 = 1727
root_residual_q = 31.
```

Those `31` q-dimensions are already included in the structural ledger. The additional requirement
that positions in `T\C` are not common-zero is an inequality condition on witnesses, not an
algebraic codimension source. So exact maximality does not supply the missing `30.196314`
q-dimensions locally; any benefit must come from global/canonical counting rather than local
root-line algebra.

This is recorded in:

```text
rfc_common_zero_subcode_paired_envelope_redflag.md
```

The conclusion is not that the common-zero route fails. The conclusion is that `S_{d-1}(h,z)` must
be a shape-sensitive recursive subcode-zero recurrence. All-paired compressed sets must be counted
with all-paired split entropy, not with arbitrary `binom(n,z)` entropy.

## Theorem Target

For a fixed profile `(P,T)` satisfying the generic Hall condition, prove:

```text
Pr[root repair fails for (P,T)]
  <= poly(d,N)
     sum_{C subset T}
       S_{d-1}(h_C, |P|+|C|)
       q^{-(t-|C|-2h_C+1)}
```

where:

```text
h_C = dim ker(ev_{P union C}).
```

For a proof-facing recurrence, replace the random value `h_C` by a sum over possible h-dimensional
child subcode witnesses:

```text
sum_h
  S_{d-1}(h, |P|+|C|)
  q^{-(t-|C|-2h+1)}
```

with Hall-incompatible buckets routed to the child rank-obstruction recurrence.

## Main Proof Obligations

1. Canonical decomposition:

```text
Every bad-root witness has a unique full common-zero set C(x,y).
```

This is immediate by definition.

2. Exact bucket upper bound:

```text
exact C(x,y)=C
  <= event C subset C(x,y).
```

This is a safe overcount.

3. Projective witness count:

If `dim ker(ev_{P union C}) = h`, then projective bad witnesses with at least C common-zero are
contained in `P(H_C plus H_C)`, of size `O(q^{2h-1})`.

4. Root residual:

For each fixed projective witness line and each coordinate outside `C`, at most one root value can
satisfy the singleton equation, with the usual determinant-1 nonzero-root normalization.

5. Child subcode-zero recurrence:

The remaining nonlocal ingredient is the first moment for h-dimensional child subspaces vanishing
on `P union C`.

This is exactly the original RFC distance/rank-tail machinery generalized from lines to
h-dimensional subcodes. It must be recursive and shape-sensitive; a uniform random fixed-set bound
is false for RFC because of paired compression.

## Meaning

This theorem is the honest version of the earlier flat-excess program. It does not peel buckets
and leave a hard complement. It replaces the entire bad-root event by the canonical common-zero
sum.

If the child subcode-zero recurrence can be proved with near random-code exponents on the target
range, this gives an end-to-end improved local repair step inside the original proof skeleton and
is plausibly strong enough for near-MDS.
