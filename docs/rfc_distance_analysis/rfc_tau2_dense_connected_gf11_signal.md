# RFC Tau=2 Dense Connected GF(11) Signal

Scope: original non-systematic RFC only.

This note interprets the observed endpoint signal:

```text
field = GF(11)
a = 4
tau = 2
delta = 3
comp = 1
g = 2
endpoint_excess_logq = 0.1649
```

The row is in the dense connected family, so it is relevant.  But the size of the signal at
`q=11` is small enough that it should be treated as a profile-discovery warning, not yet as a
counterexample to the endpoint bound.

## Immediate Interpretation

The excess means:

```text
observed_endpoint_count / predicted_endpoint_count ~= 11^0.1649
```

Numerically this is only about a constant-factor surplus:

```text
11^0.1649 ~= 1.49
```

In bits:

```text
0.1649 * log2(11) ~= 0.57 bits at GF(11)
```

So there are two plausible explanations:

```text
finite-field constant mass:
  the endpoint theorem is asymptotically correct, but the q=11 count has a small constant-factor
  surplus.

true exponent gap:
  the endpoint theorem is missing an asymptotic q^eta factor with eta around 0.1649 or some
  smaller positive value.
```

Current read:

```text
meaningful scout signal,
not yet a serious falsification signal.
```

Reason:

```text
GF(11) is still small enough for determinant coincidences, root collisions, and constant factors
to show up as positive log_q excess.
```

## Why This Row Still Matters

This is not the benign product row:

```text
delta = 2
comp = 2
g = 0
endpoint_bound_logq = -4
```

The GF(11) signal is connected:

```text
comp = 1
g = 2
```

That makes it a real member of the endpoint family we care about.  The only caveat is that
`delta = 3`, not the minimal `delta = 2` target.

Interpretation:

```text
if the signal vanishes over larger fields:
  it was finite-field constant mass.

if the signal persists as a positive exponent over larger fields:
  the dense connected tau=2 endpoint theorem needs refinement.
```

## What Large-Field Replay Must Show

The replay should measure raw counts, not only rounded `log_q` excess.  For the same exact support
profile, report:

```text
q
observed_endpoint_count
predicted_endpoint_count
count_ratio = observed / predicted
endpoint_excess_logq = log_q(count_ratio)
profile_multiplicity
root_distribution_summary
```

After the known finite constants are subtracted, also report:

```text
finite_constant_logq
residual_endpoint_excess_logq =
  endpoint_excess_logq - finite_constant_logq
```

The residual is the real falsification metric.  A raw positive `endpoint_excess_logq` is not
concerning if it is fully explained by an explicit finite constant.

Concerning large-field behavior:

```text
residual_endpoint_excess_logq stays bounded away from 0 as q grows
```

or:

```text
constant-corrected count ratio grows like q^eta for eta > 0
```

Not concerning:

```text
count_ratio remains roughly constant
```

because then:

```text
endpoint_excess_logq = log_q(constant) -> 0
```

For this exact GF(11) row, the constant-factor hypothesis predicts that larger fields should show
something like:

```text
residual_endpoint_excess_logq <= 0
```

even if the raw ratio remains around `1.5`, `2`, or another small constant.

## Residual Thresholds

Use these thresholds after finite constants are subtracted:

```text
residual_endpoint_excess_logq <= 0:
  benign, assuming exact-support and root-distribution checks are also clean.

0 < residual_endpoint_excess_logq < 0.02:
  numerical/profile warning only; replay over a larger field or another seed before escalating.

residual_endpoint_excess_logq >= 0.02:
  concerning if stable over a large prime or repeated seed.

residual_endpoint_excess_logq >= 0.05:
  serious endpoint warning; likely not just leftover constant mass.

residual_endpoint_excess_logq >= 1:
  full missing q-dimension and direct local-theorem failure.
```

The production-scaled residual criterion is:

```text
production_scaled_residual_bits =
  128 * residual_endpoint_excess_logq + log2(profile_multiplicity)

production_scaled_residual_bits > 41.83
```

For the GF(11) row, the raw `0.1649` should be considered harmless if the finite-constant
correction removes it.  The replay becomes credible evidence only if the residual remains positive.

## Concrete Counter-Signals

### 1. Persistent Positive Endpoint Exponent

Counter-signal:

```text
residual_endpoint_excess_logq > 0
```

over a large prime field and the same exact support/root profile.

Stronger counter-signal:

```text
residual_endpoint_excess_logq >= 0.05
```

at a large prime such as `GF(65537)`, especially if the raw ratio grows from smaller fields rather
than staying constant.

Very strong counter-signal:

```text
residual_endpoint_excess_logq >= 0.1649
```

again over a large prime or symbolic dimension calculation.  That would say the GF(11) excess was
not just small-field mass.

### 2. Full Missing Dimension

Strong local-theorem failure:

```text
endpoint_excess_logq >= 1
```

This is unlikely for the observed GF(11) row, but it remains the cleanest proof blocker if found.

### 3. Production-Scaled Failure

At the target parameters:

```text
q_log2 = 128
e = 71
```

compute:

```text
production_scaled_residual_bits =
  128 * residual_endpoint_excess_logq + log2(profile_multiplicity)
```

Counter-signal:

```text
production_scaled_residual_bits > 41.83
```

If the GF(11) residual `0.1649` were truly asymptotic, it would contribute:

```text
128 * 0.1649 ~= 21.1 bits
```

That is not enough by itself to cross `41.83`, but it becomes concerning if combined with more
than about `20.7` bits of profile multiplicity or recursive reuse.

### 4. Exact-Support Inversion Failure

Counter-signal:

```text
aggregate endpoint_excess_logq <= 0
but this exact a=4,delta=3,comp=1,g=2 profile has residual_endpoint_excess_logq > 0
```

Stronger:

```text
max_exact_support_endpoint_excess_logq persists over large fields
```

Interpretation:

```text
the aggregate endpoint row may be safe, but the recurrence needs exact-support control.
```

### 5. Root-Distribution Mismatch

Counter-signal:

```text
root_concentration_excess_logq > 0
```

or:

```text
same root line accounts for the positive endpoint excess across many exact supports.
```

This can matter even if the total endpoint count becomes safe, because recursive bad events depend
on reusable low-dimensional directions.

## Small-Field Evidence: Useful Or Misleading?

Useful:

```text
GF(11) identifies a concrete dense connected profile to replay.
It is larger than GF(5)/GF(7), so it is a better scout than the earlier small-field red flags.
It can reveal support shapes and root concentration patterns.
```

Misleading:

```text
endpoint_excess_logq = 0.1649 corresponds to only a 1.49x count ratio.
Such a ratio can easily be a constant term, determinant coincidence, or root collision.
Small q converts harmless constants into visible log_q excess.
```

Policy:

```text
GF(11) dirty row:
  replay target, not proof blocker.

large-prime stable positive exponent:
  real endpoint warning.

large-prime ratio bounded by constants:
  finite-field mass, not an asymptotic obstruction.
```

## Recommended Next Implementation Query

Run a targeted large-field replay of the exact GF(11) profile:

```text
profile:
  tau = 2
  a = 4
  delta = 3
  comp = 1
  g = 2

fields:
  GF(101)
  GF(1009) or another mid-size prime if cheap
  GF(65537)

seeds:
  at least two large-prime seeds if the first large-prime replay is dirty
```

Required output:

```text
q
field
seed
support_profile
root_profile
observed_endpoint_count
predicted_endpoint_count
finite_constant_factor
finite_constant_logq
count_ratio
endpoint_excess_logq
residual_endpoint_excess_logq
profile_multiplicity
production_scaled_residual_bits
distinct_root_count
max_supports_per_root
root_concentration_excess_logq
aggregate_endpoint_excess_logq
max_exact_support_endpoint_excess_logq
```

For the replay to be credible, save enough support data to reconstruct the same profile without
guesswork:

```text
ambient_depth
ambient_coordinate_count
ordered support coordinates
visible support A
singleton set S
paired-zero set P, if present
row/block labels
stride-class labels
component partition
rank data for S and S \ A
delta certificate
component/circuit certificate for comp=1
g certificate and root-line definition
canonical support hash
canonical root hash
local evaluation matrix pattern
fold-parameter seed and T-values or symbolic fold labels
```

The important point is that a large-field replay must reuse the same exact support/matroid/root
profile.  Re-finding "some" dirty `a=4,delta=3,comp=1,g=2` row is weaker evidence because it can
mix support shapes and finite constants.

Pass condition:

```text
residual_endpoint_excess_logq <= 0 or decreases toward 0 as q grows,
count_ratio stays bounded by a small constant,
root_concentration_excess_logq <= 0,
and no exact-support inversion failure appears.
```

Fail condition:

```text
residual_endpoint_excess_logq remains bounded away from 0 over large fields,
or constant-corrected count ratio grows like q^eta for eta > 0,
or production_scaled_residual_bits exceeds 41.83.
```

## Next Step After Replay

If the GF(11) profile is benign at large field, return to the minimal dense connected endpoint
query:

```text
tau = 2
comp = 1
delta = 2
g = 2
a = minimal support size where this appears
```

If the GF(11) profile stays dirty over large fields, route it immediately into the paired-spine
composition test:

```text
depth 4 plus one paired lift
chain length = 2
tau = 2
t in {3,4}
endpoint seed = this a=4,delta=3,comp=1,g=2 profile
```

The follow-up counter-signal there is:

```text
chain_excess_logq > 0
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83
```

## Feedback For Proof Agent

This row is a reminder that the local endpoint theorem must separate:

```text
asymptotic exponent gaps
from finite-field constants.
```

A `0.1649` excess over `GF(11)` is too small to trust as exponent evidence.  The proof lane should
not react to it unless large-field replay shows stable positive dimension or exact-support/root
concentration that survives the large-field check.
