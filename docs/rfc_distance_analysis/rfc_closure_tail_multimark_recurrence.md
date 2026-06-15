# Multi-Mark Closure-Tail Recurrence

Status: diagnostic for the low-rank flat endpoint `r=0,F>=1`.

## Purpose

The low-rank flat endpoint is:

```text
A subset cl(P),
|A| = F.
```

The one-mark diagnostic showed that standalone closure may be common in a loose PA-chain model,
but closure times the residual singleton-repair factor is safe for `F=1`.

This note extends that check to multiple marked coordinates without assuming independent marks.

## Diagnostic

Added:

```text
scripts/rfc_distance_analysis/rfc_closure_tail_multimark_recurrence.py
```

It models:

```text
C_d(p,a) = E[# {(P,A): |P|=p, |A|=a, A subset cl(P)}].
```

The recurrence allows correlated PA chains:

```text
b marked coordinates:
  routed through PA projection to a child event C_{d-1}(y,b);

a-b marked coordinates:
  charged directly at the parent level as A0-style closure events.
```

The lift count is intentionally loose:

```text
1. choose marked sides over PA child positions;
2. choose one P side over child core positions;
3. choose remaining P coordinates arbitrarily;
4. choose remaining direct marked coordinates arbitrarily, then charge their closure by q-exponent.
```

So a pass is meaningful evidence of slack. A fail would mostly identify an overcount to sharpen.

## Target Profile

The target top-profile parameters for the child closure endpoint are:

```text
depth = 10
expansion = 8
q = 2^128
p = 137
S = 72
residual repair exponent = S-F
```

For each `F`, run:

```text
python scripts/rfc_distance_analysis/rfc_closure_tail_multimark_recurrence.py \
  --depth 10 --expansion 8 --q-log2 128 --p 137 \
  --a F --residual-q-exp (72-F)
```

## Results

| F | residual q-exp | log2 C | combined log2 | status |
|---:|---:|---:|---:|---|
| 1 | 71 | 1389.234865 | -7698.765135 | pass |
| 4 | 68 | 1334.607680 | -7369.392320 | pass |
| 16 | 56 | 1042.036794 | -6125.963206 | pass |
| 32 | 40 | -193337.590301 | -198457.590301 | pass |
| 72 | 0 | -2529736.692902 | -2529736.692902 | pass |

The small-`F` cases are safe because the residual repair exponent supplies thousands of bits.
The large-`F` cases become safe from the closure event itself.

## Dominant Trace Shapes

For `F=1`, the trace is a pure PA chain:

```text
137 -> 119 -> 101 -> 84 -> 67 -> 51 -> 37 -> 25 -> 15 -> 8 -> 3.
```

This is why standalone closure has positive log mass. The residual factor is essential.

For `F=16`, the trace carries all marks through PA for several layers, then switches:

```text
depth 6:
  p=64,a=16
  route b=1 mark through PA
  charge 15 marks directly
```

For `F=72`, the trace quickly becomes intrinsically expensive:

```text
depth 10:
  p=137,a=72 -> child p=65,a=72

depth 9:
  route 64 marks through PA
  charge 8 marks directly

depth 8:
  route 1 mark through PA
  charge 63 marks directly
```

The direct charges dominate and make closure alone negligible.

## Interpretation

This supports the upgraded original-proof architecture for the low-rank endpoint:

```text
closure-tail event
times
residual singleton-repair equations.
```

The dangerous small-`F` range is handled by the residual repair exponent. The dangerous large-`F`
range is handled by the cost of placing many marked coordinates in the closure.

The diagnostic is still not a theorem. Its PA lift count is loose and its direct closure charge is
random-quotient style. But it no longer relies on independent marks; it permits correlated PA
chains, which were the main concern.

There is one important theorem caveat. The multi-mark PA routing assumes the strong implication:

```text
parent PA closure for b marked siblings
  -> child closure of all b projected marks.
```

The one-mark implication is proved by the PA projection lemma. For multiple PA marks, applying the
one-mark lemma independently may use the sibling-P columns of the other marked positions. The safe
deterministic conclusion is weaker:

```text
rank_child(Q union J)-rank_child(Q) <= b-1.
```

This routes to a child marked rank-increment drop, not necessarily to full child closure of all
`b` marks. The proof-facing statement and this caveat are recorded in:

```text
rfc_pa_chain_closure_theorem.md
```

## Theorem Target

A theorem-grade closure-tail statement can be weaker than a standalone closure certificate. It is
enough to prove, for the target top profile:

```text
C_10(137,F) q^{-(72-F)} <= 2^-lambda_F
```

with enough slack to union over `F=1..72` and polynomial constants.

The diagnostic suggests this should be feasible:

```text
min observed slack over sampled F values is > 6000 bits.
```

The next proof target is to formalize or repair the PA-chain recurrence:

```text
PA closure marks at the parent
  -> child closure/rank-increment event via the PA projection lemma
  -> loose but controlled lift count.
```

Then add the direct A0 closure charge for marks not routed through PA.
