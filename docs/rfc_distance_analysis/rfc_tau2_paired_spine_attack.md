# RFC Tau=2 Paired-Spine Attack

Scope: original non-systematic RFC only.

This note specifies the `tau=2` paired-spine falsification gate.  It should be run after the
complete-stride intersection gate and the `t=3` tracked-kernel ambiguity have been classified.

The goal is to test whether dense connected `tau=2` profiles create more endpoint or chain
multiplicity than the recurrence charges, especially when they interact with the complete-stride
flags.

## Attack Shape

Initial target:

```text
depth 4 plus one paired lift
chain length = 2
tau = 2
t in {3,4}
kappa_full = t - tau
kappa_tracked = 1 when a distinguished hidden line is followed
```

The checkpoint transition is:

```text
W_1 --paired projection--> V_2
K_1 = ker(singleton evaluation on W_1)
```

with a `tau=2` visible quotient on singleton support `A`.

Record both modes:

```text
local tau=2 endpoint only
tau=2 endpoint composed with a complete-stride flag L_cs <= V_cs
```

The second mode is the higher-priority one, because it tests whether the known complete-stride
exception can combine with the dense `tau=2` endpoint family.

## Required Variables

Every profile row should include:

```text
h
t
z
p
s
a
tau
kappa_full
kappa_tracked
r1
r0
z_V
z_L
delta
comp
g
support_profile
endpoint_profile
chain_length
```

Use the standard meanings:

```text
h       parent recursion depth
t       dim W_1
z       parent zero request
p       paired child zero count
s       singleton count
a       visible singleton support size |A|
tau     visible quotient dimension, fixed to 2 in this gate
kappa_full = dim K_1
kappa_tracked = dimension of distinguished hidden subspace, if any
r1      dim pi(W_1)
r0      dim pi(K_tracked) or dim pi(K_1), depending on row type
z_V     p + s - a
z_L     p + s
delta   rank(S) - rank(S \ A)
comp    component count of the local support matroid
g       measured generic/root-line endpoint parameter for the tau=2 support
```

For rows composed with complete-stride flags, also record:

```text
cs_row_block
cs_stride_pair
cs_kernel_stride
dim_V_tau2_cap_V_cs
dim_K_tau2_cap_V_cs
dim_pi_K_tau2_cap_L_cs
```

## Dangerous Profile 1: Dense Connected Supports With `g >= 2`

The first target family is:

```text
tau = 2
comp = 1
delta >= 2
g >= 2
```

These are dangerous because the local count may have two distinct sources of dimension:

```text
component/local-support freedom
generic endpoint/root-line freedom
```

The proof route is expected to charge the larger endpoint mechanism, not both independently.
The falsification risk is that the real variety has dimension above the maximum bound.

Define:

```text
component_endpoint_logq
generic_endpoint_logq
endpoint_bound_logq = max(component_endpoint_logq, generic_endpoint_logq)

observed_endpoint_logq
endpoint_gap_logq = observed_endpoint_logq - endpoint_bound_logq
```

Counter-signals:

```text
endpoint_gap_logq > 0
endpoint_gap_logq >= 1
```

The first is a proof-lane warning.  The second is a full missing `q`-dimension.

Implementation should group these rows by:

```text
(a, delta, comp, g, support_profile)
```

and report the maximum `endpoint_gap_logq` per group.

## Dangerous Profile 2: Component-Vs-Generic Endpoint Gap

This is the sharpened version of Profile 1.

Bad case:

```text
observed_endpoint_logq
  > max(component_endpoint_logq, generic_endpoint_logq)
```

Interpretation:

```text
the component theorem and the generic endpoint theorem are not alternative explanations;
the same profile appears to use degrees of freedom from both.
```

Concrete fields to report:

```text
component_endpoint_logq
generic_endpoint_logq
endpoint_bound_logq
observed_endpoint_logq
endpoint_gap_logq
which_bound_wins
```

Strong counter-signal:

```text
endpoint_gap_logq >= 1
```

Production-scaled counter-signal at `q=2^128`, `e=71`:

```text
128 * endpoint_gap_logq + log2(profile_multiplicity) > 41.83
```

If this fires without any complete-stride interaction, the local `tau=2` theorem is the blocker.

## Dangerous Profile 3: Kernel Interaction With Complete-Stride Flags

The highest-priority composition test is whether a dense `tau=2` profile aligns with a
complete-stride endpoint flag:

```text
L_cs <= V_cs
dim L_cs = 1
dim V_cs = 2
```

and a `tau=2` profile:

```text
V_tau2
K_tau2
```

The generic intersection baseline at depth 4 is usually zero for two unrelated planes/lines in
the ambient message space.  Therefore any stable positive intersection is serious.

Counter-signals:

```text
dim(V_tau2 cap V_cs) > generic
dim(K_tau2 cap V_cs) > generic
dim(pi(K_tau2) cap L_cs) > generic
V_tau2 = V_cs
L_cs <= V_tau2
pi(K_tau2) contains L_cs
```

The strongest row is:

```text
g >= 2
endpoint_gap_logq > 0
and L_cs <= V_tau2
```

because then the dense `tau=2` endpoint and the complete-stride line can potentially share the
same algebraic degrees of freedom.

Report:

```text
cs_intersection_excess =
  observed_intersection_dim - generic_intersection_dim
```

Counter-signals:

```text
cs_intersection_excess > 0
cs_intersection_excess >= 1
```

At this depth, `>= 1` is already the likely value of any nontrivial line intersection.

## Chain Metric

For every composed row, compute:

```text
chain_excess_logq =
  observed_logq_chain_count - predicted_logq_chain_count
```

where the prediction is the checkpoint recurrence/theorem exponent for the same full tuple:

```text
h,t,z,p,s,a,tau,kappa_full,kappa_tracked,r1,r0,z_V,z_L,delta,comp,g
```

Warning:

```text
chain_excess_logq > 0
```

Strong counter-signal:

```text
chain_excess_logq >= 1
```

Production-scaled counter-signal:

```text
chain_excess_bits =
  128 * chain_excess_logq + log2(finite_count_ratio)

chain_excess_bits > 41.83
```

The `41.83` bit threshold is the approximate remaining margin between the current best
exact-support scalar stress at `e=71` and the `2^-80` goal.

## Implementation Output Requirements

Emit aggregate maxima plus all positive rows for:

```text
endpoint_gap_logq > 0
cs_intersection_excess > 0
chain_excess_logq > 0
chain_excess_bits > 41.83
```

Required row fields:

```text
h,t,z,p,s,a,tau,kappa_full,kappa_tracked,r1,r0,z_V,z_L,delta,comp,g
support_profile
endpoint_profile
component_endpoint_logq
generic_endpoint_logq
endpoint_bound_logq
observed_endpoint_logq
endpoint_gap_logq
cs_row_block
cs_stride_pair
cs_kernel_stride
dim_V_tau2_cap_V_cs
dim_K_tau2_cap_V_cs
dim_pi_K_tau2_cap_L_cs
cs_intersection_excess
observed_logq_chain_count
predicted_logq_chain_count
chain_excess_logq
finite_count_ratio
chain_excess_bits
canonical_V_tau2_hash
canonical_K_tau2_hash
canonical_V_cs_hash
canonical_L_cs_hash
```

Aggregate fields:

```text
total_tau2_profiles
total_g_ge_2_profiles
positive_endpoint_gap_count
positive_cs_intersection_count
positive_chain_excess_count
max_endpoint_gap_logq
max_cs_intersection_excess
max_chain_excess_logq
max_chain_excess_bits
```

## Clean Result

The `tau=2` paired-spine gate is clean if:

```text
max_endpoint_gap_logq <= 0
max_cs_intersection_excess <= 0
max_chain_excess_logq <= 0
max_chain_excess_bits far below 41.83
no stable dense connected g>=2 row has positive excess
no complete-stride endpoint line or plane intersects tau=2 data beyond generic position
```

Interpretation:

```text
dense tau=2 local profiles do not amplify through the first paired-spine lift,
and complete-stride flags do not compose with them non-generically.
```

This would make the local tau=2 obstruction much less likely to be the e=71 blocker.

## Dirty Result Triage

If dirty, classify in this order:

```text
1. endpoint theorem failure:
   endpoint_gap_logq > 0 without complete-stride interaction.

2. complete-stride composition failure:
   cs_intersection_excess > 0 but endpoint_gap_logq <= 0.

3. true cascade failure:
   chain_excess_logq > 0 after endpoint and intersection bounds are both included.

4. production failure:
   chain_excess_bits > 41.83.
```

The most important witness is:

```text
comp = 1
g >= 2
endpoint_gap_logq >= 1
chain_excess_logq >= 1
```

The most dangerous hybrid witness is:

```text
comp = 1
g >= 2
L_cs <= V_tau2
chain_excess_bits > 41.83
```

## Next Attack If Tau=2 Is Clean

If this gate is clean, the next falsification target should be longer paired-spine chains, not
broader one-step local searches.

Recommended next gate:

```text
chain length = 3
mixed tau sequence:
  tau = 1 -> tau = 1
  tau = 1 -> tau = 2
  tau = 2 -> tau = 1
t <= 4 first
complete-stride endpoint seeds retained
```

Counter-signal for that next gate:

```text
multi_step_chain_excess_logq > 0
multi_step_chain_excess_logq >= 1
128 * multi_step_chain_excess_logq + log2(finite_count_ratio) > 41.83
```

Reason:

```text
if one-step tau=2 is clean, the remaining plausible obstruction is not a single local profile.
It is a multiplicative error where individually clean steps compose with too little entropy loss.
```

After the chain-length-3 gate, the next target should be production-depth recurrence constants for
the best exact-support scalar row.

## Feedback For Proof Agent

This gate tests the claim:

```text
the tau=2 endpoint theorem and the complete-stride endpoint exception do not share algebraic
degrees of freedom beyond the maximum endpoint bound.
```

The proof should explicitly separate:

```text
component endpoint freedom,
generic g-endpoint freedom,
complete-stride line/plane intersections,
and paired-spine lift entropy.
```

If implementation finds positive `endpoint_gap_logq` or positive complete-stride intersection
excess on a stable `g>=2` connected profile, the local theorem needs a sharper stratification
before the global certificate can be trusted.
