# RFC Dense Endpoint Support-Capture Attack

Scope: original non-systematic RFC only.

This note defines when a captured dense connected `tau=2` endpoint row is useful for falsification,
and when it is insufficient.  The motivating row is:

```text
GF(11)
a = 4
tau = 2
delta = 3
comp = 1
g = 2
raw endpoint_excess_logq = 0.1649
```

After finite constants are subtracted, the live metric is:

```text
residual_endpoint_excess_logq
```

Support capture is the bridge between a small-field scout signal and a credible larger-field
endpoint replay.

## What Makes Capture Useful

A support capture is useful only if it identifies the same exact support/root profile across
fields.  The aggregate tuple:

```text
a, delta, comp, g
```

is not enough.

Useful capture must include:

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
rank(S)
rank(S \ A)
delta certificate
component/circuit certificate for comp=1
g certificate
root-line or root-plane definition
canonical support hash
canonical root hash
local evaluation matrix pattern
fold-parameter seed
T-values or symbolic fold labels
observed endpoint roots
predicted endpoint bound row
finite constant factor used in residual correction
```

The capture is useful if a replay can answer:

```text
is this exact support profile present over a larger field?
does it have the same delta, comp, and g?
does it have the same root-distribution shape?
does residual_endpoint_excess_logq persist after finite constants are subtracted?
```

## What Makes Capture Insufficient

Capture is insufficient if it only records:

```text
aggregate tuple (a,delta,comp,g)
raw endpoint_excess_logq
field and seed
```

That is not enough because several unrelated exact supports can share the same tuple.  A large-field
run might find a different exact profile, and the comparison would be ambiguous.

Capture is also insufficient if it omits:

```text
canonical support hash
component partition
root profile
finite constant correction
exact support coordinates
or the local evaluation matrix pattern
```

Without these, the replay cannot distinguish:

```text
same profile with changing asymptotics
from different profile with same aggregate labels.
```

## Failure Mode 1: Captured Rows Are GF(11)-Specific

Failure pattern:

```text
the exact support/root profile exists over GF(11),
but the same profile does not appear over larger fields.
```

Possible causes:

```text
small-field determinant coincidence
root collision
field-specific rank drop
T-value accident
```

Interpretation:

```text
not a credible endpoint counter-signal.
```

Action:

```text
archive as small-field artifact,
do not route into paired-spine composition,
return to the minimal dense connected profile search.
```

## Failure Mode 2: Profile Does Not Replay Over Larger Fields

Failure pattern:

```text
same aggregate tuple appears,
but exact support hash, component partition, or root profile changes.
```

Interpretation:

```text
the capture was too coarse or the GF(11) row is not stable.
```

Action:

```text
do not compare endpoint_excess_logq across these rows as if they were the same profile.
rerun capture with finer support/root identifiers.
```

A replay is credible only when:

```text
same canonical support profile
same component certificate
same g certificate
same root-profile type
```

are all matched.

## Failure Mode 3: Exact-Support Inversion Changes

Failure pattern:

```text
aggregate row stays safe,
but the worst exact support changes between GF(11) and larger fields.
```

or:

```text
GF(11) exact support has positive residual,
larger field aggregate remains similar,
but exact-support residual disappears.
```

Interpretation:

```text
the aggregate endpoint row is not enough to track recurrence risk.
```

Two outcomes are possible:

```text
residual disappears:
  GF(11) signal was finite-field mass or unstable exact-support concentration.

residual persists in a matched exact support:
  real endpoint warning.
```

Implementation should report:

```text
aggregate_endpoint_excess_logq
aggregate_residual_endpoint_excess_logq
max_exact_support_residual_endpoint_excess_logq
worst_exact_support_hash
whether worst_exact_support_hash is stable across fields
```

## Failure Mode 4: Root Concentration Disappears

Failure pattern:

```text
GF(11) has root_concentration_excess_logq > 0,
but large-field replay has root_concentration_excess_logq <= 0.
```

Interpretation:

```text
small-field roots collided or the finite field was too small to model generic distribution.
```

Action:

```text
treat the GF(11) concentration as non-asymptotic.
```

This supports the finite-constant explanation.

## Failure Mode 5: Root Concentration Persists

Failure pattern:

```text
same root profile remains concentrated over larger fields.
```

Concerning if:

```text
root_concentration_excess_logq > 0
```

Serious if:

```text
root_concentration_excess_logq >= 0.05
```

or:

```text
128 * root_concentration_excess_logq
  + log2(root_profile_multiplicity) > 41.83
```

Interpretation:

```text
even if total endpoint mass is controlled, reusable bad directions may still be too concentrated
for the recursive first moment.
```

Action:

```text
route the matched profile into the paired-spine composition test.
```

## Success Criteria For Capture

Support capture succeeds if it enables a replay row with:

```text
same canonical support hash
same component partition
same delta certificate
same comp=1 certificate
same g=2 certificate
same root-profile type
same finite-constant correction rule
```

and the replay reports:

```text
residual_endpoint_excess_logq
root_concentration_excess_logq
exact-support inversion status
```

Capture is not judged by whether the row is dirty.  It is judged by whether the row is reproducible
and interpretable.

## Counter-Signals After Successful Capture

After capture succeeds, the larger-field replay is concerning if any of these hold:

```text
residual_endpoint_excess_logq > 0
residual_endpoint_excess_logq >= 0.02 over a large prime
residual_endpoint_excess_logq >= 0.05 over a large prime
root_concentration_excess_logq > 0
max_exact_support_residual_endpoint_excess_logq > 0
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

The most serious endpoint witness is:

```text
same exact support/root profile over GF(11) and GF(65537)
residual_endpoint_excess_logq >= 0.05
root_concentration_excess_logq > 0
```

That would make the row more than a finite-field constant artifact.

## Next Attack If Support Capture Succeeds

If support capture succeeds, the next attack is targeted large-field residual replay:

```text
same exact support/root profile
fields:
  GF(101)
  GF(1009) or another mid-size prime
  GF(65537)
metric:
  residual_endpoint_excess_logq
```

If the residual is clean:

```text
return to the minimal dense connected endpoint search:
  tau = 2
  comp = 1
  delta = 2
  g = 2
  minimal support size a
```

If the residual persists:

```text
route the captured profile into the tau=2 paired-spine composition attack:
  depth 4 plus one paired lift
  chain length = 2
  t in {3,4}
```

The composition counter-signal is:

```text
chain_excess_logq > 0
chain_excess_logq >= 1
128 * chain_excess_logq + log2(finite_count_ratio) > 41.83
```

## Feedback For Proof Agent

The proof lane should not treat captured GF(11) rows as endpoint-theorem failures until support
capture enables a matched large-field replay.  The useful proof split is:

```text
finite-field constants
exact-support inversion
root concentration
asymptotic endpoint exponent
```

The endpoint theorem is threatened only by the last three after constants are subtracted.
