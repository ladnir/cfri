# RFC Two-Marked-Line Plane Lemma

Scope: original non-systematic RFC, depth-5 base-seal diagnostic path.

Status: local proof target for the first non-chain incidence diagram exposed by the carried-flag
diagnostic. This is not a complete recurrence theorem.

## Situation

The corrected safe-tau-zero `z=34` trace creates the carried child flag:

```text
F_2((4,4),(2,5)).
```

Expanding both layers with their current best rows gives:

```text
outer layer (4,4):
  tau = 2,
  child plane P with zero budget 0,
  marked kernel line M <= P with zero budget 4.

inner layer (2,5):
  tau = 0,
  child line N <= P with zero budget 3.
```

The two lines `M` and `N` lie in the same child 2-plane `P`, but neither equality nor containment is
known. Since both are one-dimensional, this is a small incidence diagram rather than a total chain:

```text
      P
     / \
 M(4) N(3)
```

## Lemma Target

Fix the outer tau-two row data, including:

```text
P,
M <= P,
the outer parent lift W_0,
the exact outer zero witness.
```

Then adding the inner tau-zero child line datum costs at most:

```text
q + 1
```

choices for `N <= P`, up to the finite split/support constants for the inner zero witness.

Equivalently, in q-dimension bookkeeping, this diagram uses a `1`-dimensional line-choice factor
instead of the current coarse ancestor factor:

```text
GaussianBinomial(4,2)_q ~= q^4.
```

So the local diagram can save roughly:

```text
3 q-dimensions
```

relative to choosing an arbitrary inner two-dimensional ancestor inside the four-dimensional parent
space.

## Proof Sketch

Condition on the child code and on the outer row certificate. The outer tau-two row in this
boundary case has:

```text
dim pi(W_0) = 2,
dim pi(K_0) = 1,
dim W_0 = 4,
tau = 2,
lift_qdim = 0.
```

Thus, after `P` and `M` are fixed, the outer lift has no remaining q-dimensional family in the
coarse lift formula.

For the inner tau-zero row, the parent subspace `W_1` has:

```text
dim W_1 = 2,
tau = 0,
pi(W_1) = N,
dim N = 1.
```

For fixed `N`, the tau-zero lift exponent is:

```text
2 * (2*1 - 2) = 0.
```

Therefore the only q-dimensional choice not already present in the fixed outer certificate is the
choice of the child line:

```text
N <= P.
```

A two-dimensional space has at most `q+1` projective lines. Requiring `N` to satisfy its zero
budget, or allowing the possibility `N=M`, can only reduce the count. The inner row's paired /
singleton split choices are finite combinatorial factors and should be counted separately.

## Diagnostic Value

`scripts/rfc_distance_analysis/rfc_carried_flag_diagnostic.py` applies this q-dimensional
replacement to the carried state. It reports:

```text
carried merge saving:       251.97763219 bits
two-marked-line saving:     381.41503750 bits
combined local saving:      633.39266969 bits
```

This is meaningful but still not enough to close the depth-5 `z=34` checkpoint. The result is most
useful because it identifies the next proof state precisely: small incidence diagrams inside a
fixed child plane, not arbitrary longer chains.
