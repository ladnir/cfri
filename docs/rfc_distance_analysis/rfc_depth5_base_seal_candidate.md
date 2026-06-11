# RFC Depth-5 Base Seal Candidate

Scope: original non-systematic RFC distance certificate, target `c=8,k=2048,e=71`.

Status: candidate route for closing the current `child_k=32` high-defect hard-segment gap.

## Motivation

The current high-defect hard-segment blocker is localized at:

```text
child_k = 32,
reachable floor zeros = 34.
```

This is exactly the finite depth-5 base case:

```text
depth = 5,
k = 32,
n = 256,
target z = k + 2 = 34.
```

If the proof can seal the depth-5 aggregate first moment directly at `z=34`, then the
length-four theta-chain truncation does not need to close every artificial high-defect normal
slice at `child_k=32`; the recursion can stop at this finite base seal.

## Calibration Run

The finite-replica/rank-pattern calibration:

```text
python scripts/rfc_distance_analysis/rfc_replica_zero_moment.py \
  --depth 5 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge replica \
  --print-window 4 \
  --trace-z 34
```

reports:

```text
crossing_z = 34,
crossing_excess = 2,
log2 moment at z=34 = -115.10435419.
```

The dominant aggregate trace for `z=34` is:

```text
level 5: z=34, p=2, s=30, c=0, u=2
level 4: z=2,  p=0, s=2,  c=0, u=0
level 3: z=0
level 2: z=0
level 1: z=0
```

This is not the hard-chain stress shape. It is singleton-heavy at the top and then terminates
quickly.

## Non-Result

The older `component-uniform` calibration is too pessimistic here:

```text
python scripts/rfc_distance_analysis/rfc_replica_zero_moment.py \
  --depth 5 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge component-uniform \
  --print-window 4 \
  --trace-z 34
```

It crosses only at:

```text
crossing_z = 249.
```

So the base seal cannot rely on the old component-uniform shortcut. It needs the rank-pattern
induction theorem targeted in:

```text
docs/rfc_distance_analysis/rfc_rank_pattern_induction_target.md
```

## Proof Target

Prove a finite depth-5 rank-pattern base theorem:

```text
For depth d <= 5, expansion c=8, and q=2^128,
sum_{|Z|=34} E[# nonzero messages vanishing on Z] <= 2^-80,
```

or more generally:

```text
B_5(1,34) <= 2^-80.
```

The theorem should be independent of empirical sampling. A sufficient path is to prove the
rank-pattern induction recurrence for all replica counts:

```text
r = 1,2,4,8,16,32
```

that occur while unfolding the depth-5 first moment, using the exact rank-one singleton algebra
instead of the component-uniform shortcut.

The precise recurrence contract and calibration tables are now in:

```text
docs/rfc_distance_analysis/rfc_depth5_rank_pattern_contract.md
```

## Interaction With The High-Defect Gap

The hard-segment stress row:

```text
child_k=32,
zeros=(34,39,44),
b=14
```

still has a local three-q-dimension gap in the normal-slice hard-trace accounting. The base seal
does not prove that local row false or impossible. Instead, it gives a stopping rule:

```text
Once the recurrence reaches child_k=32 and zero budget at least 34,
use the finite base theorem rather than continuing the theta-chain truncation.
```

This is attractive because the production zero floor guarantees at least `34` zeros at `child_k=32`
for the `e=71` target.

## Current Verdict

Promising partial.

It turns the current frontier from a global high-defect chain problem into a finite rank-pattern
base theorem. The remaining work is nontrivial but more bounded than a new arbitrary-depth
theta-chain lemma.
