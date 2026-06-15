# PA-Chain Weak Rank Scale

Status: repaired scale check after the multi-mark PA closure caveat.

## Purpose

The strong multi-mark closure diagnostic assumed:

```text
parent PA closure with b marked siblings
  -> child closure of all b projected marks.
```

The proof-facing PA-chain note corrected this. The safe deterministic implication is:

```text
parent PA closure with b marked siblings
  -> rank_child(Q union J)-rank_child(Q) <= b-1.
```

That is, `b` projected marked child coordinates have at least one rank dependency modulo the child
projection core `Q`.

This note checks whether the safe weak implication has enough scale for the flat-excess endpoint.

## Diagnostic

Added:

```text
scripts/rfc_distance_analysis/rfc_pa_chain_weak_rank_scale.py
```

It is a one-step scale check. For each number `b` of PA-routed marked coordinates and child core
size `y`, it charges:

```text
rank_child(Q union J)-rank_child(Q) <= b-1
```

with q-exponent:

```text
(b-(b-1)) * (D_child-(b-1))
  = D_child-b+1,

D_child = k_child-y.
```

It uses the same loose parent lift count as the multi-mark closure diagnostic and adds direct
closure charges for marks not routed through PA.

The script supports two charge modes:

```text
random_rank:
  random one-rank marked-tail scale.

closure_union:
  deterministic minimal-circuit reduction to one-mark closure, with an added
  circuit union factor at most b*2^(b-1).
```

## Target Profile Results

Target:

```text
depth = 10
expansion = 8
q = 2^128
p = 137
S = 72
residual repair exponent = S-F
```

Runs:

```text
python scripts/rfc_distance_analysis/rfc_pa_chain_weak_rank_scale.py \
  --depth 10 --expansion 8 --q-log2 128 --p 137 \
  --a F --residual-q-exp (72-F)
```

Results:

| F | residual q-exp | log2 weak PA | combined log2 | best b | best y | child q-exp | status |
|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 71 | -47122.877140 | -56210.877140 | 1 | 136 | 376 | pass |
| 4 | 68 | -47106.231735 | -55810.231735 | 4 | 133 | 376 | pass |
| 16 | 56 | -47061.971021 | -54229.971021 | 16 | 121 | 376 | pass |
| 72 | 0 | -46997.108805 | -46997.108805 | 72 | 65 | 376 | pass |

The best row always routes all `F` marks through PA and chooses:

```text
y = p-F.
```

Then:

```text
D_child-b+1 = 512 - (p-F) - F + 1 = 512 - 137 + 1 = 376.
```

So the weak PA-chain route has a stable child rank-drop exponent of `376` q-factors for all `F`
in this target family.

## Interpretation

This is stronger than the loose strong-closure diagnostic for small `F`.

The safe deterministic implication already gives a substantial child rank-drop event:

```text
q^-376
```

before residual repair. Therefore the multi-mark caveat is not fatal. We can route PA chains to a
child marked rank-increment drop rather than child full closure.

For the target profile, even the boundary `F=72` with no residual repair is about:

```text
46997 bits
```

below expectation in this one-step weak-rank scale.

The theorem-facing `closure_union` mode gives essentially the same conclusion:

| F | residual q-exp | combined log2 | best b | best y | status |
|---:|---:|---:|---:|---:|---|
| 4 | 68 | -55805.231735 | 4 | 133 | pass |
| 72 | 0 | -46919.938880 | 72 | 65 | pass |

The loss relative to `random_rank` is only the circuit union factor.

## Proof Consequence

The closure endpoint can be handled by splitting marked coordinates:

```text
direct/A0 marks:
  pay closure cost q^{-(a-b)(k-p)};

PA-routed marks:
  pay a child marked rank-drop event
  rank(Q union J)-rank(Q) <= b-1.
```

For theorem purposes this means we do not need the strong multi-mark closure claim. The required
child theorem is a one-rank marked incremental tail for PA-routed marks.

This is much closer to the already identified marked incremental rank-tail framework, but in a
very special and high-slack regime:

```text
increment drop = 1,
child quotient dimension about 376,
large residual repair slack for small F.
```

## Remaining Work

Formalize the weak PA-chain theorem:

```text
parent PA closure with b marks
  -> child rank increment <= b-1.
```

Then prove a child marked one-rank-drop bound in the required regime. The random scale says this is
more than enough; the main task is to make the bound shape-sensitive under RFC recursion.

The deterministic closure reduction is recorded in:

```text
rfc_one_rank_marked_tail_closure_reduction.md
```
