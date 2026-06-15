# One-Mark Closure-Tail Recurrence

Status: diagnostic for the low-rank flat endpoint `r=0,F=1`.

## Purpose

The low-rank flat endpoint is:

```text
A subset cl(P).
```

For `F=1`, this becomes the one-mark closure event:

```text
C_d(p) = E[# {(P,a): |P|=p, a notin P, a in cl(P)}].
```

This note records a deliberately conservative recurrence for `C_d(p)` and how it combines with
the residual singleton-repair exponent in the upgraded original proof.

## Diagnostic

Added:

```text
scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_recurrence.py
```

It uses the split:

```text
A0:
  the marked coordinate's sibling is not in P.
  Pay direct quotient closure cost q^{-(k-p)}.

PA:
  the marked coordinate's sibling is in P.
  Use the PA projection lemma:
    if the marked sibling lies in span(P), then the child column lies in the
    projection span of the other P columns.
  Route to a child one-mark closure event.
```

The PA lift count is intentionally loose. Given a child closure witness `(Q,j)`, the script:

```text
1. chooses the marked side over j;
2. chooses one parent side over every q in Q;
3. chooses all remaining P coordinates arbitrarily.
```

So this is a stress model, not a polished theorem.

## Target Run

Command:

```text
python scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_recurrence.py \
  --depth 10 --expansion 8 --q-log2 128 --p 137
```

Standalone closure output:

```text
log2_C = 1389.199423
status = fail
```

The trace is a PA chain:

```text
depth 10: p=137 -> child y=119
depth 9:  p=119 -> child y=101
depth 8:  p=101 -> child y=84
...
depth 0:  p=3
```

Interpretation: the loose closure recurrence alone is not enough. A long PA chain can push the
marked closure event to small depth, where closure becomes cheap.

This is not a contradiction. The flat-excess proof does not use closure alone.

## Residual Repair Factor

For the target top profile:

```text
p = 137
t = 1845
D = 887
S = t - 2D + 1 = 72
```

The `F=1` flat-excess stratum also has residual singleton-repair exponent:

```text
S-F = 71.
```

Command:

```text
python scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_recurrence.py \
  --depth 10 --expansion 8 --q-log2 128 --p 137 --residual-q-exp 71
```

Output:

```text
log2_C = 1389.199423
combined_log2 = -7698.800577
status = pass
```

Thus even this loose PA-chain closure bound has about:

```text
7698 bits
```

of slack once multiplied by the residual root-repair equations.

## Meaning

For `r=0,F=1`, the proof does not need:

```text
C_d(137) <= 2^-80
```

as a standalone event. It needs:

```text
C_d(137) * q^{-(S-F)} <= 2^-lambda
```

inside the flat-excess stratum. The diagnostic strongly supports this, even with very loose PA
lift counting.

This is important because it shows the low-rank endpoint can be handled by the same upgraded
original-proof architecture:

```text
closure-tail event
times
residual singleton-repair root equations.
```

## Proof Target

A theorem-grade version should prove:

```text
C_d(p,1; PA-chain-stress)
  <= poly(n,d) 2^{O(profile entropy)}
```

with enough precision that:

```text
C_d(p,1) q^{-(S-1)}
```

is negligible for the target top profile.

The diagnostic suggests that a very coarse upper bound may already suffice: the PA-chain entropy is
only about `1389` bits, while the residual root equations contribute:

```text
71 * 128 = 9088 bits.
```

## Remaining Work

Extend from one mark to `F` marks:

```text
C_d(p,F) = E[# {(P,A): |P|=p, |A|=F, A subset cl(P)}].
```

The closure-only event may have correlated PA chains, so do not simply raise the one-mark bound to
the `F`th power. However, for larger `F`, the residual factor decreases:

```text
q^{-(S-F)}
```

while the closure condition asks for more marked coordinates. The next useful diagnostic is a
multi-mark closure recurrence with grouped PA chains.
