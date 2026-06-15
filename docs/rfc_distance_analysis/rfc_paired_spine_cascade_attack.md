# RFC Paired-Spine Cascade Attack

Scope: original non-systematic RFC only.

This note specifies the next falsification gate after clean complete-stride cross-copy
intersections.  The attack is a paired-spine cascade seeded by the 48 ordered complete-stride
flags.

No broad enumeration is intended here.  The implementation should target the exact shape below.

## Starting Point

Use the depth-4 complete-stride flags from the clean size-8 scan.

Complete-stride supports:

```text
B_b = {4b, 4b+1, 4b+2, 4b+3},  b in {0,1,2,3}
C_j = {j, j+4, j+8, j+12},      j in {0,1,2,3}
Omega = C_i union C_j,          i < j
```

For each unmarked support pair `(B_b, Omega)`:

```text
dim K_Omega = 2
K_Omega = ell_i direct_sum ell_j
```

Emit two ordered endpoint flags:

```text
ell_i <= K_Omega
ell_j <= K_Omega
```

Expected seed set:

```text
24 unmarked complete-stride supports
48 ordered flags L_end <= V_end
dim L_end = 1
dim V_end = 2
```

## Attack Shape

Target:

```text
depth 4 plus one paired lift
chain length 2
t in {2,3}
tau = 1
tracked hidden kappa = 1
```

The endpoint is a complete-stride flag:

```text
L_2 <= V_2
```

The paired lift searches for parent checkpoint data:

```text
W_1
K_1 = hidden singleton kernel line inside W_1
pi(W_1) = V_2
pi(K_1) = L_2
```

so the chain is:

```text
K_1 <= W_1  --paired projection-->  L_2 <= V_2
```

When the implementation keeps the previous checkpoint flag as well, record it as:

```text
L_1 <= V_1  --paired projection-->  L_2 <= V_2
```

with:

```text
dim V_1 = t
dim visible quotient = tau = 1
dim tracked hidden line = 1
```

Important convention:

```text
For t = 2:
  tau = 1 and tracked kappa = 1 agree with full kernel dimension.

For t = 3:
  tracked kappa = 1 means one distinguished hidden line is being followed.
  The full singleton kernel may have dimension t - tau = 2.
  Implementation must record both:
    kappa_tracked = 1
    kappa_full = dim ker(singleton evaluation on W_1)
```

Do not collapse these two meanings.  A false clean result can happen if the tracked line is counted
but the extra full-kernel direction is dropped without charge.

## Required Variables

Each row should include:

```text
h
t
z
p
s
a
tau
kappa_tracked
kappa_full
r1
r0
z_V
z_L
delta
comp
g
endpoint_row_block
endpoint_stride_pair
endpoint_kernel_stride
chain_length
```

Use the checkpoint meanings:

```text
h       parent recursion depth
t       dim W_1 before paired projection
z       parent zero request
p       paired child zero count
s       singleton count at this level
a       visible singleton support size
tau     visible singleton quotient dimension
r1      dim pi(W_1)
r0      dim pi(K_1) for the tracked hidden line
z_V     p + s - a
z_L     p + s
delta   rank(S) - rank(S \ A)
comp    component count of the local support profile
g       measured generic/root-line kernel parameter when available
```

For the target gate:

```text
chain_length = 2
tau = 1
kappa_tracked = 1
t in {2,3}
```

Expected endpoint:

```text
r1 = dim V_2 = 2
r0 = dim L_2 = 1
```

If the paired lift produces other `(r1,r0)` values, emit them as separate profiles rather than
folding them into the target row.

## Main Counter-Signal

Define:

```text
chain_excess_logq =
  observed_logq_chain_count - predicted_logq_chain_count
```

The prediction should be the checkpoint recurrence/theorem exponent for the same full variable
tuple, including exact endpoint support, visible support, hidden-line tracking, and paired
projection constraints.

Warning signal:

```text
chain_excess_logq > 0
```

Strong counter-signal:

```text
chain_excess_logq >= 1
```

Production-scaled counter-signal at the target parameters:

```text
q_log2 = 128
e = 71

chain_excess_bits =
  128 * chain_excess_logq + log2(finite_count_ratio)

chain_excess_bits > 41.83
```

The `41.83` bit threshold is the approximate remaining margin between the current best
exact-support scalar stress at `e=71` and the `2^-80` goal.  A full extra `q`-dimension gives
`128` bits and is therefore fatal for the current target unless another structural charge appears.

## Specific Dirty Signals

### 1. Positive Chain Excess

Emit every row with:

```text
chain_excess_logq > 0
```

Required fields:

```text
observed_logq_chain_count
predicted_logq_chain_count
chain_excess_logq
finite_count_ratio
chain_excess_bits
```

Interpretation:

```text
The paired lift is producing more compatible parent flags than the recurrence charges.
```

### 2. Full Extra `q`-Dimension

Emit as high priority if:

```text
chain_excess_logq >= 1
```

Interpretation:

```text
The recurrence is missing at least one algebraic degree of freedom.
```

This is a direct falsification candidate for `e=71`.

### 3. Production-Scaled Margin Failure

Emit as high priority if:

```text
chain_excess_bits > 41.83
```

This can happen either from a full `q`-dimension or from a smaller exponent excess multiplied by a
large finite profile multiplicity.  It is the production criterion, not just the local algebra
criterion.

### 4. Repeated Hidden `L` Across Paired Lift

Record canonical hashes for:

```text
L_2
V_2
tracked hidden K_1 line
parent W_1
```

Counter-signals:

```text
same tracked hidden K_1 line maps to the same L_2 for multiple distinct parent profiles
same L_2 is hit by more parent W_1 profiles than the generic preimage count predicts
same hidden line remains active after varying endpoint row_block or stride_pair
```

Equivalent dimension tests:

```text
dim(K_1 cap pi^{-1}(L_2)) > generic
dim(W_1 cap pi^{-1}(V_2)) > generic
```

At this small depth, generic excess should be zero unless the tuple forces containment.  Any
stable positive excess is serious.

### 5. Hidden Full-Kernel Direction For `t = 3`

For `t=3`, record:

```text
kappa_full = dim ker(singleton evaluation on W_1)
```

Counter-signals:

```text
kappa_full > 1 and the extra kernel direction is not charged by predicted_logq_chain_count
the extra direction also projects into V_2 or another endpoint complete-stride plane
the extra direction persists under seed variation
```

This is the main reason to include `t=3` in the first gate.  It checks whether tracking only one
hidden line is too optimistic.

### 6. Support-Profile Sequence Correlation

Group by:

```text
endpoint_row_block
endpoint_stride_pair
endpoint_kernel_stride
parent_support_profile
visible_support_profile
```

Counter-signal:

```text
positive chain_excess_logq concentrated in a stable profile sequence
```

This matters more than a single literal vector collision.  A profile-stable excess is what would
survive in the proof and production recurrence.

## Clean Result

This gate is clean if:

```text
max chain_excess_logq <= 0
max chain_excess_bits <= 0
no chain_excess_bits near 41.83
no repeated hidden L beyond generic preimage count
no stable support-profile sequence with positive excess
t=3 extra kernel directions are either absent or fully charged
```

Interpretation:

```text
complete-stride flags do not amplify through one paired lift.
the checkpoint recurrence is plausible for the first production-shaped nested-kernel gate.
```

This does not prove the recurrence.  It only closes the first targeted paired-spine falsification
test.

## Dirty Result Triage

If dirty:

```text
1. Separate literal vector collisions from profile-stable excess.
2. Re-run the same profile under a second large-prime seed.
3. Check whether the excess is in endpoint complete-stride data, parent lift data, or the t=3
   extra full-kernel direction.
4. Report the smallest witness tuple with chain_excess_logq > 0.
```

Classification:

```text
literal one-seed collision:
  likely determinant noise unless repeated.

profile-stable positive excess:
  proof blocker.

chain_excess_logq >= 1:
  strong lower-bound obstruction.

production-scaled > 41.83 bits:
  direct e=71 certificate threat even if local logq excess is below 1.
```

## Follow-Up Ranking If Clean

### 1. Same Gate, Wider Parent State

Extend:

```text
t = 4
tau = 1
kappa_tracked = 1
record kappa_full
chain length = 2
```

Reason:

```text
t=4 is the next place a tracked hidden line can hide inside a larger full kernel.
```

### 2. Same Gate, `tau = 2`

Extend:

```text
t in {3,4}
tau = 2
kappa_tracked = 1
chain length = 2
```

Reason:

```text
tau=2 is the known local-algebra risk.  Testing it on the paired spine tells us whether it can
compose with complete-stride endpoint flags.
```

### 3. Longer Paired-Spine Chains

Extend:

```text
chain length = 3
t in {2,3}
tau = 1 first
```

Reason:

```text
one paired lift may be generic while two consecutive paired lifts create a reusable hidden-line
cascade.
```

### 4. Dense Connected `tau=2` Endpoint Profiles

Move off complete-stride seeds and test:

```text
connected tau=2 profiles
delta >= 2
measured endpoint parameter g
```

Reason:

```text
if complete-stride is clean through paired lifts, the next independent risk is the dense tau=2
local theorem, not complete-stride multiplicity.
```

### 5. Broad Longer Nested Flags

Run only as a discovery pass after the targeted gates above:

```text
chain length >= 3
t <= 4
tau <= 2
mixed support profiles
```

Reason:

```text
broad nested-flag enumeration is expensive and hard to interpret unless the targeted gates are
already clean.
```

## Feedback For Proof Agent

This gate tests the claim:

```text
one paired-spine lift of a complete-stride endpoint flag pays the generic flag/preimage entropy.
```

The proof needs to account for:

```text
tracked hidden line L,
full kernel dimension when t > tau + 1,
paired projection preimages,
exact endpoint support,
and finite-field determinant exceptions.
```

The most likely failure is not another cross-copy collision.  It is a missing charge for a hidden
kernel line that survives one paired lift while the recurrence counts only the visible endpoint
flag.
