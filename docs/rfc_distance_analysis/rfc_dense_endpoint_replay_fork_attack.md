# RFC Dense Endpoint Replay Fork Attack

Scope: original non-systematic RFC only.

This note interprets the replay mismatch:

```text
GF(11) captured row:
  a = 4
  delta = 3
  comp = 1
  g = 2

GF(31) same coordinates:
  no longer comp = 1
```

Attack-side conclusion:

```text
the GF(31) residual is not the same endpoint profile.
```

Therefore it should not be compared directly to the GF(11) residual exponent.  The same coordinate
set changed its matroid/component profile, so the replay failed as an exact-profile replay.

## Interpretation Of The Mismatch

The mismatch is evidence for one of these:

```text
GF(11)-specific connectedness:
  the connected comp=1 certificate depended on a small-field coincidence.

coordinate capture too weak:
  same coordinates do not determine the same algebraic support profile across fields.

profile migration:
  the row survives as an aggregate shape somewhere else over GF(31), but not at the same
  coordinates.
```

What it is not:

```text
evidence that the GF(11) residual persists over GF(31).
```

Once `comp` changes, the residual belongs to a different theorem row.

## Fork A: Guarded Larger-Field Search

Route A searches over the larger field for any profile with:

```text
a = 4
delta = 3
comp = 1
g = 2
```

Guardrails:

```text
group by exact support profile,
emit component certificate,
emit g certificate,
subtract finite constants,
report residual_endpoint_excess_logq,
do not compare rows unless the profile certificate matches.
```

Usefulness:

```text
high for falsification.
```

Reason:

```text
we care whether this profile class exists stably over larger fields with positive residual,
not whether one GF(11) coordinate set survives verbatim.
```

Counter-signal:

```text
there exists a GF(31) or larger-field row with:
  a = 4
  delta = 3
  comp = 1
  g = 2
  residual_endpoint_excess_logq > 0
```

Concerning:

```text
residual_endpoint_excess_logq >= 0.02
```

Serious:

```text
residual_endpoint_excess_logq >= 0.05
```

Production-dangerous:

```text
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83
```

Strongest signal:

```text
same certified support-profile type appears over GF(31) and GF(65537)
with residual_endpoint_excess_logq bounded away from 0.
```

## Fork B: Field-Lift The GF(11) Support/Profile

Route B attempts to lift the GF(11) object, not just the coordinates.

This means preserving:

```text
component certificate for comp=1
rank/delta certificate
g certificate
root profile
local evaluation matrix pattern
relations among fold parameters responsible for the GF(11) row
```

Usefulness:

```text
medium for diagnosis,
lower for immediate falsification.
```

Reason:

```text
the same coordinates already failed over GF(31).
```

A field-lift is useful only if it can explain which algebraic relation made the GF(11) support
connected and then test whether that relation defines a real positive-dimensional stratum over
large fields.

Counter-signal:

```text
the GF(11) comp=1 certificate lifts to a symbolic or large-field family
with residual_endpoint_excess_logq > 0.
```

Failure signal:

```text
the comp=1 certificate requires a polynomial relation that holds accidentally over GF(11)
but not generically.
```

Interpretation:

```text
then the GF(11) row is a small-field artifact.
```

## Which Route Is More Useful?

For finding a real counter-signal, Route A is more useful:

```text
guarded larger-field search for any a=4,delta=3,comp=1,g=2 profile.
```

Reason:

```text
the attack target is a stable endpoint profile class with positive residual.
The same-coordinate GF31 replay already says the captured GF11 coordinates are not a reliable
field-independent representative.
```

Route B should be used only if Route A finds nothing but we still need to explain the GF(11)
artifact or build a symbolic small-field-degeneracy certificate.

## Next Single Implementation Diagnostic

Run one guarded GF(31) search:

```text
field = GF(31)
target:
  a = 4
  tau = 2
  delta = 3
  comp = 1
  g = 2

mode:
  search for any certified exact profile,
  not same-coordinate replay.
```

Required output:

```text
total_profiles_checked
matching_profiles_count
max_residual_endpoint_excess_logq
max_production_scaled_residual_bits
max_root_concentration_excess_logq
worst_support_profile_hash
worst_root_profile_hash
component_certificate
delta_certificate
g_certificate
finite_constant_logq
observed_endpoint_count
predicted_endpoint_count
residual_endpoint_excess_logq
```

Emit every row with:

```text
residual_endpoint_excess_logq > 0
root_concentration_excess_logq > 0
```

Pass condition:

```text
matching_profiles_count = 0
```

or:

```text
all matching profiles have residual_endpoint_excess_logq <= 0
and root_concentration_excess_logq <= 0.
```

Fail condition:

```text
matching profile with residual_endpoint_excess_logq > 0.
```

Escalate if:

```text
residual_endpoint_excess_logq >= 0.02
```

or:

```text
128 * residual_endpoint_excess_logq + log2(profile_multiplicity) > 41.83.
```

If this GF(31) guarded search is dirty, replay only the worst certified profile over `GF(65537)`.
If it is clean, return to the minimal dense connected endpoint target:

```text
tau = 2
comp = 1
delta = 2
g = 2
minimal support size a.
```

## Feedback For Proof Agent

The GF31 mismatch supports a caution:

```text
coordinate replay is not profile replay.
```

A proof or falsification note should not treat two rows as the same endpoint object unless the
component, delta, g, and root certificates match.  The next proof-relevant signal is not the GF11
coordinate set itself; it is whether the certified dense connected profile class exists over large
fields with positive residual endpoint exponent.
