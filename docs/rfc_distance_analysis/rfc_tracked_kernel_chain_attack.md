# RFC Tracked-Kernel Chain Attack

Scope: original non-systematic RFC only.

This note isolates the `t=3` ambiguity in the paired-spine cascade attack:

```text
full_kernel_dim = 2
tracked_kappa = 1
tau = 1
```

The question is whether tracking one hidden line inside a 2D singleton kernel loses a projective
line of choices.  If it does, the checkpoint recurrence may undercount the chain by one full
`q`-dimension.

## Why `t=3` Is The Minimal Ambiguous Case

For the first paired-spine gate:

```text
dim W = t
tau = 1
K_full = ker(singleton evaluation on W)
dim K_full = t - tau
```

So:

```text
t = 2: dim K_full = 1
t = 3: dim K_full = 2
```

At `t=2`, the tracked hidden line is the whole kernel.  There is no ambiguity.

At `t=3`, the recurrence may track only:

```text
L_tracked <= K_full
dim L_tracked = 1
```

but the actual object contains the full plane:

```text
dim K_full = 2
```

The dangerous gap is:

```text
number of valid tracked lines inside K_full
```

If that number is generic constant, no issue.  If it is about `q`, the attack gets one extra
`q`-dimension.

## Most Dangerous Mechanism

The worst case is a free hidden-line selector.

Let:

```text
pi: parent ambient -> paired child ambient
K_full <= W
dim K_full = 2
endpoint complete-stride flag L_2 <= V_2
dim L_2 = 1
dim V_2 = 2
```

The dangerous configuration is:

```text
pi(K_full) = L_2
dim(K_full cap ker pi) = 1
```

Then every nonzero line in `K_full` except the projection-kernel line maps to the same endpoint
line `L_2`.  The projective line `P(K_full)` has:

```text
q + 1 lines
```

and about `q` of them can map to `L_2`.

If the recurrence counts only one tracked hidden line, but implementation counts flags

```text
(W, K_full, L_tracked)
```

then this profile has:

```text
hidden_line_fiber_logq ~= 1
```

That is a full missing algebraic degree of freedom.

Equivalent description:

```text
one line in K_full is projection-invisible,
the quotient K_full / (K_full cap ker pi) maps to L_2,
and the choice of tracked line inside K_full is not fixed by support or zero constraints.
```

This is more dangerous than a single vector collision.  It is a structured fiber.

## Target Test Shape

Implementation should test:

```text
depth 4 plus one paired lift
chain length = 2
t = 3
tau = 1
kappa_tracked = 1
kappa_full = 2
endpoint seeds = 48 ordered complete-stride flags
```

Record the same tuple as the paired-spine gate:

```text
h,t,z,p,s,a,tau,kappa_tracked,kappa_full,r1,r0,z_V,z_L,delta,comp,g
endpoint_row_block
endpoint_stride_pair
endpoint_kernel_stride
parent_support_profile
visible_support_profile
```

Add these `t=3`-specific fields:

```text
dim_K_full
dim_pi_K_full
dim_K_full_cap_ker_pi
valid_tracked_line_count
predicted_tracked_line_count
hidden_line_fiber_logq
same_endpoint_line_parent_count
same_K_full_tracked_line_count
```

Expected dangerous ranks:

```text
dim_K_full = 2
dim_pi_K_full = 1
dim_K_full_cap_ker_pi = 1
pi(K_full) = L_2
```

## Counter-Signals

### 1. Positive Hidden-Line Fiber

Define:

```text
hidden_line_fiber_logq =
  log_q(valid_tracked_line_count)
  - log_q(predicted_tracked_line_count)
```

Counter-signal:

```text
hidden_line_fiber_logq > 0
```

Strong counter-signal:

```text
hidden_line_fiber_logq >= 1 - o(1)
```

In finite field output, this should look like:

```text
valid_tracked_line_count about q
```

rather than a bounded count.

### 2. Positive Chain Excess

Use the full chain metric:

```text
chain_excess_logq =
  observed_logq_chain_count - predicted_logq_chain_count
```

Counter-signal:

```text
chain_excess_logq > 0
```

Strong counter-signal:

```text
chain_excess_logq >= 1
```

The hidden-line fiber should feed directly into this metric.  If
`hidden_line_fiber_logq ~= 1` but `chain_excess_logq <= 0`, implementation should explain where
the recurrence charged the extra projective-line choice.

### 3. Production-Scaled Failure

At the target parameters:

```text
q_log2 = 128
e = 71
```

compute:

```text
chain_excess_bits =
  128 * chain_excess_logq + log2(finite_count_ratio)
```

Counter-signal:

```text
chain_excess_bits > 41.83
```

One full missing `q`-dimension gives about `128` bits and is therefore fatal for the current
`e=71` target unless another proof-side charge cancels it.

### 4. Repeated Hidden `L` Across The Paired Lift

Record canonical hashes for:

```text
L_2
V_2
K_full
L_tracked
W
```

Counter-signals:

```text
same L_2 receives many distinct L_tracked inside one K_full
same K_full contributes many L_tracked to the same endpoint complete-stride flag
same L_tracked remains valid for multiple endpoint row_block/stride_pair profiles
same endpoint support profile has a stable positive hidden_line_fiber_logq under seed variation
```

The first two are the main `t=3` ambiguity.  The last two are stronger because they indicate
profile-stable multiplicity, not just a one-off finite-field collision.

### 5. Uncharged Extra Kernel Direction

Counter-signal:

```text
kappa_full = 2
kappa_tracked = 1
and the second kernel direction is not represented in predicted_logq_chain_count
```

Concrete rank form:

```text
dim(K_full cap ker pi) = 1
dim pi(K_full) = 1
```

with:

```text
valid_tracked_line_count about q
```

This says the extra direction is not harmless.  It creates a fiber of possible tracked lines.

## Clean Result

The `t=3` tracked-kernel gate is clean if:

```text
max hidden_line_fiber_logq <= 0
max chain_excess_logq <= 0
max chain_excess_bits <= 0 or far below 41.83
no stable endpoint profile with valid_tracked_line_count growing like q
no repeated hidden L beyond the predicted preimage count
every kappa_full = 2 row shows an explicit recurrence charge for the second direction
```

Interpretation:

```text
tracking one hidden line is safe for the first ambiguous parent dimension.
```

That would not prove all paired-spine cascades, but it would close the smallest place where a full
kernel could hide behind `tracked_kappa = 1`.

## Should `t=3` Come Before Widening?

Yes.

Recommended order:

```text
1. t = 2, tau = 1, kappa_full = kappa_tracked = 1
2. t = 3, tau = 1, kappa_tracked = 1, kappa_full = 2
3. t = 4, tau = 1
4. tau = 2
5. chain length = 3
```

Reason:

```text
t=3 is the first case where the tracked object and the full kernel differ.
```

If the recurrence is missing a projective-line fiber, `t=3` should expose it with the smallest
state space and the cleanest witness.  Widening to `t=4`, `tau=2`, or longer chains before this
would mix the tracked-kernel ambiguity with other effects and make a dirty row harder to classify.

## Implementation Pass/Fail Summary

Pass:

```text
all valid_tracked_line_count values are bounded as q grows,
hidden_line_fiber_logq <= 0,
chain_excess_logq <= 0,
and no endpoint profile has stable repeated hidden-line multiplicity.
```

Fail:

```text
valid_tracked_line_count about q for a fixed endpoint/profile,
hidden_line_fiber_logq > 0,
chain_excess_logq > 0,
chain_excess_logq >= 1,
or chain_excess_bits > 41.83.
```

The highest-priority witness is:

```text
dim_K_full = 2
dim_pi_K_full = 1
dim_K_full_cap_ker_pi = 1
valid_tracked_line_count about q
chain_excess_logq >= 1
```

That is the cleanest falsification of the tracked-kernel simplification.
