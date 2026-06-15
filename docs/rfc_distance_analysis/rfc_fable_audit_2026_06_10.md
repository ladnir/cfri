# RFC Fable Audit 2026-06-10

Scope: original non-systematic RFC distance-certificate proof status.

Reviewer invoked through:

```text
claude -p --model claude-fable-5 --effort high
```

## High-Severity Findings

### Product Of First Moments Is Not A Safe Upper Bound

The attempted rank-one product split for:

```text
tau = 2
|A| = 2
delta = 2
comp = 2
K = L = 0
```

used:

```text
poly(N) * F_child(1, outer_zeros + 1)^2.
```

This is not proof-safe. The two component lines live in the same child-code instance, so this
multiplies two first moments over correlated randomness. The safe replacement is a joint
marked-line or frame state, for example:

```text
poly(N) * (q+1) * F_child((2, outer_zeros), (1, outer_zeros + 1)),
```

or a sharper state that marks both component lines.

### Flag Checkpoint `best` Mode Was Anti-Conservative

`rfc_flag_span_moment.py --flag-bound best` included:

```text
F(r1,z1) * F(r0,z0)
```

as one candidate child-flag bound. This is not a safe upper bound on a joint flag moment. The
default `best` mode has been changed to use only the two Gaussian-containment relaxations:

```text
outer first: F(r1,z1) * [r1 choose r0]_q
inner first: F(r0,z0) * [child_k-r0 choose r1-r0]_q.
```

The explicit `--flag-bound product` mode remains a heuristic-only diagnostic.

Safe depth-4 rerun after this change:

```text
crossing_excess = 49
z=65/e=49 log2_vector_moment = -351.48393067
dominant tau-two row: p=16, s=2, |A|=2, local_charge=4, lift_qdim=4
```

So the `|A|=2` boundary row remains live until the joint marked-line/frame state is implemented.

Follow-up: a coarse joint marked-line diagnostic was added for this exact row. It counts a joint
flag `(V,L)` with `dim V=2`, `dim L=1`, `V` zero on `outer_zeros`, `L` zero on
`outer_zeros+1`, and pays one `q+1` factor for the second line in `V`. This avoids multiplying two
line first moments.

Depth-4 rerun with that joint marked-line bound:

```text
crossing_excess = 49
z=65/e=49 log2_vector_moment = -596.35983667
dominant tau-two row: |A|=3, delta=2, comp=1, theta_2=-2
```

This is a diagnostic replacement for the invalid squared split; the proof still needs the actual
joint marked-line/frame recurrence stated cleanly.

## Medium Findings

### Exact-Support Grassmann Cap Looks Sound But Needs Constants

The reviewer agreed with the shape:

```text
local_charge >= |A|
```

for fixed exact-support quotient branches. The formal proof still needs:

```text
1. explicit determinant-1 fold matrix and root-to-line injectivity for both singleton orientations;
2. explicit nonzero-root constants (q-1)^-|A| rather than silent q^-|A|.
```

The same cap should generalize to every visible dimension `tau`, not only `tau=2`.

### The |A|=3 Row Is Closed But Needs A Direct Lemma

The row:

```text
|A| = 3
delta = 2
comp = 1
```

has:

```text
theta_2 = -2.
```

This is the connected `U_{2,3}` component/full-kernel endpoint. It should get a direct proof rather
than relying only on the still-open general component lemma.

Follow-up: direct proof added in:

```text
docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
```

### Current Diagnostic Still Has Heuristic Local State

The checkpoint still uses:

```text
uniform_component_count
quotient_delta_floor
generic gamma_h >= (h-g)^2
```

as calibration shortcuts. These are not theorem ingredients.

## Suggested Next Proof Tasks

1. Add a joint marked-line/frame recurrence state for the `|A|=2,delta=2,comp=2` row.
2. Prove root-to-line injectivity and nonzero-root constants for the determinant-1 singleton fold.
3. Write the direct `U_{2,3}` full-kernel endpoint lemma.
4. Prove the `g=1` first-drop layer bound by a nonzero maximal-minor/Schwartz-Zippel argument:

```text
gamma_2 >= 1
```

for all `g=1` supports.

Follow-up: local proof added in:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
```

It closes the first-drop local exponent as `theta_2=-1`; global stacking of these rows remains
open.
5. Finish the connected full-kernel diagonal-stabilizer lemma for the `h=delta` layer.
