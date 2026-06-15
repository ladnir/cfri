# Flat-Excess Control Stack

Status: current theorem stack for the upgraded original binary RFC proof.

## Purpose

The original-proof upgrade needs to control the flat-excess loss in singleton repair:

```text
Pr[repair failure]
  <= poly(D,t) q^{-(t - 2D + 1 - flat_excess)}.
```

Earlier notes treated this as requiring a broad marked incremental rank-tail theorem. The current
stack is narrower. Flat-excess witnesses split into endpoint and reduction cases that can be
charged by simpler objects.

## Core Definitions

At a fixed top survivor profile:

```text
P = paired child positions
T = singleton child positions
K_P = ker(ev_P)
D = dim K_P
t = |T|
S = t - 2D + 1
```

For a flat-excess witness:

```text
A subset T
r = rank(A modulo P)
F = |A| - 2r
```

the residual repair exponent is:

```text
max(0, S-F).
```

At the target top profile:

```text
p = |P| = 137
t = 1845
D = 887
S = 72
q = 2^128
```

## Step 1: Incidence Correction

The root-line repair incidence theorem gives:

```text
repair codim >= S - F.
```

Source:

```text
rfc_surplus_incidence_stratification.md
```

This explains exactly where flat excess enters. Hall-OK alone gives only at least one q-factor;
surplus requires bounding or charging overfull flats.

## Step 2: Marked Rank Bridge

Flat excess is a marked child rank event:

```text
r = rank(P union A) - rank(P).
```

Source:

```text
rfc_flat_excess_charge_push.md
```

Random scale for this event gives combined exponent:

```text
Q(r,F) = (r+F)(D-r) + max(0,S-F).
```

The weakest high-rank stratum has `r=D-1` and combined exponent:

```text
D - 1 + S.
```

At target:

```text
D - 1 + S = 958.
```

## Step 3: Duality To Child Subcode-Zero Events

Let:

```text
h = D-r.
```

Then:

```text
rank(A modulo P) <= D-h
```

if and only if there exists an `h`-dimensional child subspace vanishing on:

```text
P union A.
```

Source:

```text
rfc_high_rank_flat_duality_reduction.md
```

This splits the remaining work:

```text
high-rank endpoint h=1:
  ordinary child line-zero/rank-tail event;

low-rank endpoint h=D:
  marked closure/subcode-zero endpoint;

middle h:
  larger subcode-zero exponents, numerically easier.
```

## Step 4: High-Rank Endpoint

For `h=1`, the witness is:

```text
there exists a nonzero child codeword vanishing on P union A.
```

At target:

```text
|P union A| = 1909 + F
```

and the child line-zero exponent is:

```text
886 + F.
```

Combined with residual repair:

```text
(886+F) + (72-F) = 958
```

for `F <= 72`.

This endpoint is therefore covered by the original child distance/rank-tail object.

## Step 5: Low-Rank Endpoint

For `r=0`, the witness is:

```text
A subset cl(P).
```

Source:

```text
rfc_low_rank_flat_closure_endpoint.md
```

This must be treated as a marked closure event, not as aggregate `B_d(|P|+F,F)`. The aggregate
short-set recurrence overcounts by allowing hidden dependencies inside `P union A`.

## Step 6: One-Mark And Multi-Mark Closure Diagnostics

One-mark closure:

```text
rfc_closure_tail_one_mark_recurrence.md
```

Target result:

```text
C_10(137,1) q^-71 ~= 2^-7698.800577.
```

Multi-mark closure with correlated PA chains:

```text
rfc_closure_tail_multimark_recurrence.md
```

Sampled target results:

```text
F=1:  combined ~= -7698.77 bits
F=4:  combined ~= -7369.39 bits
F=16: combined ~= -6125.96 bits
F=32: combined ~= -198457.59 bits
F=72: combined ~= -2529736.69 bits
```

These diagnostics use a strong PA-chain closure assumption, which has a caveat for multiple marks.

## Step 7: PA-Chain Caveat And Repair

For multiple PA marks, parent closure does not directly imply:

```text
J subset cl_child(Q).
```

The safe deterministic implication is:

```text
rank_child(Q union J)-rank_child(Q) <= |J|-1.
```

Source:

```text
rfc_pa_chain_closure_theorem.md
```

Then the one-rank marked tail reduces to one-mark closure by a circuit argument:

```text
rfc_one_rank_marked_tail_closure_reduction.md
```

The theorem-facing scale check:

```text
rfc_pa_chain_weak_rank_scale.py --charge-mode closure_union
```

still passes:

```text
F=4:  combined ~= -55805.231735 bits
F=72: combined ~= -46919.938880 bits
```

Source:

```text
rfc_pa_chain_weak_rank_scale.md
```

## Current Reduced Blocker

The broad blocker:

```text
prove a full marked incremental rank-tail theorem
```

has been reduced for flat-excess control to:

```text
1. ordinary child line-zero/rank-tail for the high-rank endpoint;
2. one-mark closure-tail bounds with controlled lifts;
3. PA-chain weak rank-drop -> one-mark closure via circuit union;
4. direct A0 closure charges for marks not routed through PA.
```

The most theorem-facing remaining statement is:

```text
For child RFC closure events C_d(p,1), prove a shape-sensitive recurrence with:
  A0 direct charge q^{-(k-p)},
  PA lift routed through one-rank/circuit closure reduction,
  all-paired compression handled recursively.
```

This is substantially narrower than the previous marked-pair recurrence.

## Next Proof Step

Write the formal closure-tail theorem:

```text
C_d(p,1) recurrence with A0 and PA terms
```

then plug it into:

```text
C_d(p,F) q^{-(S-F)}
```

using the circuit-union reduction for multi-mark PA chains.
