# RFC Replica Span State

This note records the failure mode of the first component-aware recurrence model. The local
component-only charge described here has since been superseded by the endpoint-aware tau-2 bound in
`rfc_tau2_weighted_exterior_bound.md`, but the replica-span diagnosis remains valid.

## What Was Tested

The script:

```text
scripts/rfc_distance_analysis/rfc_replica_zero_moment.py
```

now has:

```text
--singleton-charge component-uniform
```

This mode applies the first `r=2` component correction in a uniform-quotient model. For a parent
replica count `r`, a non-common singleton block `E` is charged as:

```text
|E| + r rank(E) - comp(E),
```

with the special case `r=1` charged as just `|E|`, since rank-one compatibility is vacuous for one
replica.

## Result

At `c=8`, `q=2^128`, security `80`, the crossings are:

```text
depth  k   optimistic-free e   component-uniform e
2      4   1                   3
3      8   1                   35
4      16  2                   98
```

The depth-4 crossing trace is:

```text
trace_level,z,p,s,c,u
4,114,56,2,0,56
3,56,24,8,0,24
2,24,8,8,0,8
1,8,0,8,0,0
```

The bottom line is the issue. At level `1`, the child is the base repetition code of dimension `1`
and expansion `8`. All eight base coordinates form one rank-`1` parallel component. For parent
replica count `r=8`, the component-aware charge is:

```text
|E| + r rank(E) - comp(E)
  = 8 + 8*1 - 1
  = 15,
```

where the free-set optimistic charge would be:

```text
r|E| = 64.
```

This produces a huge high-replica moment.

## Interpretation

This does not yet look like a true distance obstruction. It is a moment-state obstruction.

The high-replica moment is dominated by ordered replica tuples whose span in message space is small.
For example, if an `r`-tuple spans only a line, then all replica root equations agree for structural
reasons. The component-aware coordinate charge correctly sees fewer exterior equations, but the
state still counts the tuple inside the full ordered `r`-replica moment.

So the recurrence is mixing two different effects:

```text
1. coordinate-side rank/component collapse,
2. replica-side span collapse.
```

Charging the first without tracking the second is too pessimistic.

## Correct Next State

The recurrence should track replica span dimension.

For an ordered `r`-tuple in a `k`-dimensional message space, let:

```text
t = dim span(m_1, ..., m_r).
```

The number of such tuples has exponent:

```text
t(k - t) + r t = t(k + r - t),
```

instead of the full `r k`.

The rank-one compatibility charge should depend on `t`, not only on ordered replica count `r`.
In the older component-only model the expected singleton charge was:

```text
|E| + t rank(E) - comp(E),
```

at least in the generic case where the `t`-dimensional replica span sees the selected child image
faithfully.

The corrected tau-2 local state additionally needs the generic two-copy root-line kernel dimension
`g(A)` for the visible support `A`; dense supports can be governed by the generic endpoint rather
than by the component endpoint.

This fixes the bottom parallel-class diagnosis:

```text
r=8, t=1, |E|=8, rank(E)=1, comp(E)=1
charge = 8 + 1*1 - 1 = 8,
tuple-count exponent = k + r - 1 rather than r k.
```

The lower charge is then paired with a much smaller tuple-count exponent. The current recurrence
gets only the lower charge, which is why it blows up.

## Next Lemma Target

The next finite-replica theorem should be stratified by both:

```text
coordinate matroid state: rank(E), comp(E)
replica state:            span dimension t
```

A plausible one-step bound is:

```text
M_d(r,t,z)
  <= sum over root split profiles
       q^[child tuple-span exponent]
       q^-[|E| + t rank(E) - comp(E)]
       * recursive common-zero terms.
```

The proof task is to make the child tuple-span exponent recursive rather than inserting the ambient
Grassmann count by hand.

## Subspace-Span Toy Model

A cleaner toy model was added:

```text
scripts/rfc_distance_analysis/rfc_subspace_span_moment.py
```

It counts message subspaces rather than ordered replica tuples and pays ordered tuple coefficients
only at the end. This fixes one overcount but exposes another missing state.

At `c=8`, `q=2^128`, security `80`:

```text
depth  k   subspace-span e
2      4   16
```

The depth-2 trace is:

```text
trace_level,span,z,p,s,c,u,child_span
2,1,20,4,12,0,4,2
1,2,4,0,4,0,0,1
```

The bottom line says: take a two-dimensional parent subspace over a one-dimensional base child
projection and ask for four singleton zeros. The model charges this as finite, but a full
two-dimensional subspace of `H plus H` over a rank-`1` child component maps onto `F^2` at each visible
coordinate, so no singleton root can kill it. The only compatible subspaces are those with a kernel
on the singleton block.

Therefore global replica span dimension `t` is still not enough. The local state must track:

```text
visible span dimension on E:
  tau_E = rank of the replica/message subspace after restriction to the singleton block E

kernel on E:
  t - tau_E
```

The charge should use `tau_E`, while the kernel directions must be paid as common-zero/vanishing
structure elsewhere. In other words, the right induction is not just:

```text
coordinate rank/components + global replica span.
```

It is:

```text
coordinate rank/components + restricted visible span/kernel data.
```

This is exactly the point where the proof becomes a matroid-rank induction rather than a scalar
recurrence.

## Visible-Span Profile

The local visible-span object is now profiled directly in:

```text
scripts/rfc_distance_analysis/rfc_visible_span_profile.py
```

and summarized in:

```text
docs/rfc_distance_analysis/rfc_visible_span_local_state.md
```

The key result is:

```text
tau = 1:
  compatibility is automatic; root support is the line support.
  Counts are controlled by delta(A) = rank(S) - rank(S \ A), the dimension of child words
  supported on the root support A.

tau = rank(U):
  compatible full-visible-rank subspaces are component-wise diagonal graphs,
  with dimension comp(U).

0 < tau < rank(U):
  root support distribution, and more precisely the support subcode dimension delta(A), is a
  genuine state variable.
```

So the next recurrence should track visible dimension `tau`, visible root support `A`, and the
support subcode dimension `delta(A)`, not only global span dimension.

## Endpoint-Tau2 Diagnostic

The span diagnostics now have an `endpoint-tau2` mode:

```text
python scripts/rfc_distance_analysis/rfc_subspace_span_moment.py \
  --depth 3 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2

python scripts/rfc_distance_analysis/rfc_replica_span_moment.py \
  --depth 3 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge endpoint-tau2
```

This mode replaces the component-only `tau=2` charge by the corrected generic/component endpoint:

```text
min(4delta - 2g, |A| + 2delta - comp(A)).
```

Small-depth results after rejecting impossible `tau=2, delta<2` local states:

```text
subspace span, depth=2: component-uniform e=16, endpoint-tau2 e=16
subspace span, depth=3: component-uniform e=48, endpoint-tau2 e=46
ordered span,  depth=3: component-uniform e=49, endpoint-tau2 e=49
```

The old dominant traces used an invalid bottom branch with `span=2` over a rank-1 parallel block.
The exact local checks:

```text
python scripts/rfc_distance_analysis/rfc_visible_span_profile.py \
  --prime 5 --child-depth 0 --expansion 8 --size 8 --tau 2

python scripts/rfc_distance_analysis/rfc_root_line_kernel_profile.py \
  --prime 5 --child-depth 0 --expansion 8 --size 8 --tau 2
```

produce no compatible exact-support rows. Saved artifacts:

```text
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_base_c8_size8_tau2_summary.csv
docs/rfc_distance_analysis/rfc_visible_span_exact_gf5_base_c8_size8_tau2_support_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_base_c8_size8_tau2_summary.csv
docs/rfc_distance_analysis/rfc_root_line_kernel_exact_gf5_base_c8_size8_tau2_supports.csv
```

The corrected depth-3 subspace trace at the crossing is:

```text
depth=3 subspace trace at z=54:
3, span=1, z=54 -> p=22, s=10, u=22, child_span=2
2, span=2, z=22 -> p=6,  s=10, u=6,  child_span=1
1, span=1, z=6  -> p=0,  s=6,  u=0,  child_span=1
```

The invalid `span=2` bottom line is gone. The model is still much too pessimistic at depth 4, where
the last finite trace is:

```text
4, span=1, z=120 -> p=56, s=8, u=56, child_span=2
3, span=2, z=56  -> p=24, s=8, u=24, child_span=2
2, span=2, z=24  -> p=8,  s=8, u=8,  child_span=1
1, span=1, z=8   -> p=0,  s=8, u=0,  child_span=1
```

So the next missing state is more precise than the first bottom bug: when a visible span loses
dimension on a singleton block, the invisible kernel directions must be tied to the child
common-zero request that made them invisible. A scalar `(span,z)` state still does not remember
where those kernel directions vanished.
