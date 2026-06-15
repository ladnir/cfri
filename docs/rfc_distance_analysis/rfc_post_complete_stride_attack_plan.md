# RFC Post Complete-Stride Falsification Plan

Scope: original non-systematic RFC only.

This note assumes the latest gate is clean:

```text
large-prime size-8 complete-stride scan: clean
two-copy complete-stride flag intersections: clean
```

The remaining confirmation step for the complete-stride family is three-copy plus seed variation.
If that also stays clean, the lower-bound lane should stop treating complete-stride as an
independent multicopy obstruction and move to nested/recursive obstructions.

## Immediate Gate: Three-Copy And Seed Variation

The three-copy check should not be broad.  It should use the same complete-stride flags:

```text
L_i <= V_i
dim L_i = 1
dim V_i = 2
i in {1,2,3}
```

Real counter-signals:

```text
dim(V_1 cap V_2 cap V_3) > generic
dim(L_1 cap L_2 cap L_3) > generic
dim(L_i cap V_j cap V_k) > generic for distinct i,j,k
same canonical L active in >= 3 copies
same canonical V active in >= 3 copies
same support-profile incidence repeats under seed variation
```

At depth 4, the generic intersections are zero.  So any nonzero stable intersection over a large
prime is a real signal.  A one-seed one-off should be classified as determinant noise unless the
same support profile repeats under another seed or prime.

Clean interpretation:

```text
complete-stride is a local dimension-growth exception,
but independent copies see it in generic position.
```

That would close the standalone complete-stride multicopy attack.  It would not close nested
flags along one recursive path.

## Re-Ranked Attacks If This Gate Passes

### Rank 1: Paired-Spine Cascades

This becomes the top target.

Reason: the production recurrence is dominated by all-paired compression followed by local
singleton events.  If complete-stride flags are generic across independent copies, the remaining
way to lose a full `q`-dimension is for a flag to survive through the paired spine with less
entropy loss than the recurrence charges.

Target object:

```text
W_0
  -> child flag L_1 <= V_1
  -> child flag L_2 <= V_2
  -> ...
```

where each transition is a checkpoint state:

```text
V_j = pi(W_{j-1})       zero on z_V = p+s-a
L_j = pi(K_{j-1})       zero on z_L = p+s
K_{j-1} = kernel of singleton evaluation on W_{j-1}
```

Concrete counter-signal:

```text
chain_excess_logq =
  observed_logq_chain_count - predicted_logq_chain_count
```

Treat as real if either:

```text
chain_excess_logq >= 1
```

or, at `q = 2^128` and `e = 71`,

```text
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83 bits
```

The `41.83` bit number is the approximate gap between the current best exact-support scalar
stress at `e=71` and the `2^-80` target.  One full missing `q`-dimension is already fatal.

Implementation should test next:

```text
depth 4 first, then one paired lift
t in {2,3,4}
tau in {1,2}
kappa = t - tau
complete-stride local flags as seeds
one or two consecutive checkpoint transitions
```

Required output fields:

```text
h,t,z,p,s,a,tau,kappa,r1,r0,z_V,z_L,delta,comp,g
chain_length
support_profile_sequence
observed_logq_chain_count
predicted_logq_chain_count
chain_excess_logq
chain_excess_bits_at_q128
max surviving kernel dimension
canonical deepest L/V hashes for positive-excess rows
```

A clean result means:

```text
no stable positive chain_excess_logq,
no full extra q-dimension,
no support-profile sequence with bit excess near the e=71 slack.
```

### Rank 2: Dense Connected `tau=2` Profiles

This is the next independent local-algebra threat.

Reason: complete-stride is a sparse structured exception.  Dense connected `tau=2` profiles could
still break the endpoint-aware local theorem even if all complete-stride intersections are clean.
This risk is not primarily multicopy; it is a local visible-span/counting risk.

Target object:

```text
tau = 2 visible quotient
connected support profile
delta >= 2
component count comp = 1
measured generic/root-line kernel dimension g
```

Concrete counter-signal:

```text
eta =
  observed_logq_profile_count
  - endpoint_theorem_logq_bound(a, delta, comp, g)
```

Treat as real if:

```text
eta > 0
```

on a profile that is stable across field/seed variation and can be embedded in a recursive
checkpoint state.  Treat it as production-dangerous if:

```text
128 * eta + log2(profile_multiplicity) > 41.83 bits
```

Implementation should test after the paired-spine seed case:

```text
depth <= 4
tau = 2
t in {2,3,4}
group by (a, delta, comp, g)
measure exact endpoint count, not just support count
```

Required output fields:

```text
h,t,z,p,s,a,tau,kappa,r1,r0,z_V,z_L,delta,comp,g
observed_logq_profile_count
endpoint_bound_logq
eta
field
seed
profile multiplicity
recursive embeddability flag
```

A clean result means the local theorem route is probably structurally correct for `tau=2`; the
remaining work is constants and finite-field factors.

### Rank 3: General Longer Nested Flags

This is ranked below paired-spine cascades and dense `tau=2`.

Reason: broad longer-flag enumeration can become expensive and hard to interpret.  It should be
used as a discovery tool only after the production-shaped paired-spine checks and the independent
`tau=2` local checks are clean.

Target object:

```text
L_m <= V_m <= ... <= L_1 <= V_1 <= W_0
```

with mixed `tau` values and no assumption that every transition is complete-stride.

Concrete counter-signal:

```text
generic_flag_excess_logq =
  observed_logq_nested_flag_count - generic_gaussian_flag_logq
```

Treat as real if:

```text
generic_flag_excess_logq >= 1
```

or if a sub-unit excess repeats across enough profiles to exceed the `e=71` bit slack.

Implementation should test only after Rank 1 and Rank 2:

```text
depth 4 exact nested flags
chain length 3 first
t <= 4
tau <= 2
group by complete flag-variable tuple, not just support shape
```

The purpose is to find an unmodeled family, not to run a broad production extrapolation.

## Recommended Next Implementation Step

If three-copy/seed variation stays clean, run this exact next check:

```text
paired-spine cascade, depth 4 plus one paired lift
seeded by the 48 ordered complete-stride flags
t in {2,3}
tau = 1 first
kappa = 1 first
chain length = 2 checkpoint transitions
```

Pass condition:

```text
max chain_excess_logq <= 0
no positive-excess support-profile sequence stable under seed variation
no deepest L or V active beyond generic multiplicity
```

Fail condition:

```text
chain_excess_logq >= 1
```

or:

```text
chain_excess_bits_at_q128 > 41.83
```

If this exact check passes, extend the same enumerator in this order:

```text
1. t = 4, tau = 1, kappa = 3
2. tau = 2 with kappa > 0
3. chain length = 3
4. dense connected tau=2 endpoint profiles
```

## Feedback For Proof Agent

If the three-copy/seed gate is clean, the proof side should stop spending complexity on
complete-stride cross-copy correlation.  The claim most likely to fail next is:

```text
the checkpoint recurrence charges the full generic flag entropy for nested kernel survival along
paired compression.
```

The proof needs an explicit flag-lift lemma that handles chains, not only one transition.  The
lemma should state where the generic loss is paid at every paired-spine step and should expose the
finite-field exceptional determinant factors separately.

## Manager Summary

Current ranking after clean two-copy complete-stride:

```text
0. close complete-stride with three-copy plus seed variation
1. paired-spine cascades
2. dense connected tau=2 profiles
3. general longer nested flags
```

The next concrete counter-signal is a positive paired-spine `chain_excess_logq`, especially a full
extra `q`-dimension.  The next implementation test should be the depth-4-plus-one-paired-lift
cascade seeded by the 48 ordered complete-stride flags.
