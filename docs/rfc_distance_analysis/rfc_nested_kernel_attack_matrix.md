# RFC Nested-Kernel Falsification Attack Matrix

Scope: original non-systematic RFC only.

This is a manager-facing lower-bound/falsification note.  It assumes marked-core multicopy is dead:
with exact support / flag counting, the old seven-copy row drops to about `-151.94` at `e=71`, and
the best exact-support scalar stress is the one-copy row:

```text
e=70:    3.41617672
e=71: -121.83277193
e=72: -247.08241391
```

So the remaining falsification target is not another marked-core scalar sweep.  It is whether
nested flags create extra multiplicity or dimension that is invisible to exact support counting.

## Variable Dictionary

Flag transition variables:

```text
h      recursion depth of the parent node
t      parent span dimension dim(W)
z      parent zero request
p      paired child zero count
s      singleton count at this level
a      visible singleton support size |A|
tau    visible quotient dimension on A
kappa  kernel dimension dim(K) = t - tau
r1     child outer span dimension dim pi(W)
r0     child inner span dimension dim pi(K)
z_V    outer child zero request = p + s - a
z_L    inner child zero request = p + s
delta  local support/subcode increment delta(A) = rank(S) - rank(S \ A)
comp   component count of the local support matroid on A
g      generic two-copy/root-line kernel dimension for A
```

For `tau=1`, `g` is not the main local endpoint; record it as `0` or `NA` unless a two-copy root
line is explicitly being counted.  For `tau=2`, the suspicious dense endpoint is usually
`g = max(0, 2*delta - a)` in the uniform approximation, but the enumerator should measure it
directly.

The child flag is:

```text
L <= V
V = pi(W), zero on z_V = p + s - a
L = pi(K), zero on z_L = p + s
```

Production slack reference: the best exact-support scalar `e=71` stress is about `-121.83`, so an
unaccounted flag effect of more than `41.83` bits can move the model above the 80-bit target.  A
single missing `q`-dimension at `q=2^128` is already `128` bits.

## Candidate 1: Complete-Stride One-Kernel Cascade

This is the first real nested-kernel target.  The parent span has one visible direction and one
invisible kernel direction.  The kernel survives only if the singleton support contains complete
stride classes; isolated extras have entropy but do not create kernel dimension.

Flag template:

```text
h:      exact check at h=4 first; production extrapolation on h=5..11
t:      2
z:      checkpoint/spine value at level h
p:      chosen so z = 2p + s
s:      singleton block size; prioritize s around one matched core plus extras
a:      1 or 2 first; then exact complete-stride visible support sizes
tau:    1
kappa:  1
r1:     2 preferred; also test r1=1,3,4 if exact enumeration is cheap
r0:     1
z_V:    p + s - a
z_L:    p + s
delta:  1 for the visible quotient A in the first pass
comp:   1 for connected/matched visible support; otherwise exact component count
g:      NA for tau=1, but record g(S) for the full singleton set S if available
```

Structural subcase to prioritize:

```text
matched core size b > 1,
extras E contain exactly one complete extra stride class,
dim(K) increases from 1 to 2 in the local near-kernel model.
```

Known small-depth anchor:

```text
depth 4, m=4, extra outputs 4:
  7872 rows have kernel_dim = 1
  48 rows have kernel_dim = 2
```

Counter-signal at `e=71`:

```text
exact_log2_flag_count + observed_q_exponent - generic_flag_bound_log2 > 41.83 bits
```

or, more locally:

```text
observed flag intersection dimension exceeds generic flag dimension by >= 1 q-dimension.
```

Check type:

```text
Exact small-depth first: depth 4, large-prime RFC sample or symbolic stride-class enumeration.
Production use is heuristic extrapolation until the exact pattern is classified.
```

## Candidate 2: Higher-Rank Late Singleton Burst

The flag checkpoint killed the scalar hidden-kernel trace at depth 4, but only for the two-layer
coarse state.  A later singleton burst after several all-paired compressions may create a longer
flag:

```text
V_0 >= V_1 >= V_2 >= ...
```

where each quotient is small, but the deepest kernel accumulates many zeros.

Flag template for the first executable approximation:

```text
h:      exact check h=4; production h follows paired spine 11 -> 10 -> ... -> 5
t:      3 or 4 first; then t up to the observed child span on the paired spine
z:      paired-spine target at level h, e.g. 65 at depth 4 toy crossing
p:      mostly paired, so p large and s late/small-to-moderate
s:      late singleton burst size
a:      visible support in the burst, usually a << s for a dangerous kernel
tau:    1 or 2
kappa:  t - tau, so at least 2 for the real higher-rank threat
r1:     around t or 2t after projection; exact enumerator should record actual rank
r0:     around kappa or 2*kappa after projection
z_V:    p + s - a
z_L:    p + s
delta:  at least tau on A
comp:   exact component count of A
g:      for tau=2 quotients, exact generic kernel dimension on A
```

Counter-signal at `e=71`:

```text
The exact count of longer flags beats the product/two-layer approximation by > 41.83 bits after
production scaling.
```

A depth-4 local counter-signal is:

```text
there are many chains L2 <= L1 <= V whose count is larger than the Gaussian flag count predicted
from the one-layer child bounds.
```

Check type:

```text
Exact small-depth for h=4 with t=3,4 and tau<=2.
Heuristic production extrapolation along the paired spine only after exact depth-4 chains are
classified.
```

Why this is ranked second:

```text
It is the most plausible way the two-layer checkpoint could still be too optimistic.  But if the
depth-4 exact longer-flag count is generic, this family probably collapses.
```

## Candidate 3: Dense Connected tau=2 Generic-Kernel Supports

This is the local algebra obstruction that killed the old component-only tau=2 theorem.  It may
seed a nested cascade if dense connected supports with `g >= 2` align recursively.

Flag template:

```text
h:      exact checks at h=2,3,4; production extrapolation only after grouping by profile
t:      2 first
z:      parent zero request being tested
p:      paired child zero count
s:      singleton count
a:      dense visible support size
tau:    2
kappa:  0 for the pure local tau=2 quotient; also test kappa>0 when embedded in Candidate 2
r1:     2,3,4
r0:     0 for pure quotient; kappa projection when embedded
z_V:    p + s - a
z_L:    p + s if kappa>0, otherwise inactive
delta:  >= 2
comp:   1 for the connected dense profiles of interest
g:      measured generic two-copy/root-line kernel dimension, suspicious when g >= 2
```

Counter-signal at `e=71`:

```text
exact tau=2 flag/root-line count exceeds the endpoint-aware theorem by enough that the production
trace loses > 41.83 bits, or by one full q-dimension in any recursively reusable profile.
```

Exact local excess to record:

```text
eta = observed_logq_count - max(generic_endpoint_logq, component_endpoint_logq).
```

The concerning case is:

```text
eta > 0 on connected profiles that recurse or compose with Candidate 1/2.
```

Check type:

```text
Exact small-depth over GF(5), GF(7), GF(11) for h<=4, grouped by (a, delta, comp, g).
Production extrapolation is heuristic until a profile-stability theorem exists.
```

## Ranked Enumerator Recommendation

1. **Depth-4 two-layer flags, t=2, tau=1, kappa=1, complete-stride extras.**

   This directly tests Candidate 1.  Start from the known `m=4, extra=4` dimension-growth class.
   For each exact support profile, count distinct child flags `L <= V`, not marked cores.  Group by:

   ```text
   h,t,z,p,s,a,tau,kappa,r1,r0,z_V,z_L,delta,comp,g,
   complete_extra_stride_count,
   exact flag count,
   generic Gaussian flag count,
   observed generic-flag excess.
   ```

2. **Depth-4 longer flags with t=3,4 and tau<=2.**

   This tests Candidate 2 and whether the two-layer checkpoint is hiding a longer nested-kernel
   chain.  Do not enumerate all large states first; seed from the paired-spine/depth-4 crossing
   trace and vary the singleton burst.

3. **Depth-4 dense connected tau=2 profiles with g>=2.**

   This tests Candidate 3.  Reuse the root-line/exterior profile machinery if possible, but emit
   the same flag variables as above so the proof lane can plug the result into the recurrence.

4. **Two-copy intersections of exact flags.**

   Run only after single-copy exact flag profiles are grouped.  The first question is whether
   multiple complete-stride certificates inside one support are genuinely distinct flags; the
   second is whether independent copies intersect generically.

## Feedback For Proof Agent

The claim most likely to fail is the harmlessness of the flag-lift count:

```text
Given exact supports and zero budgets, the number of parent flags lifting a child flag is only the
generic Gaussian flag factor, up to polynomial/constants.
```

The danger is not marked-core support multiplicity anymore.  The danger is that complete-stride
structure creates many distinct nested flags with the same unmarked support data, and that these
flags survive paired compression with more dimension than the generic flag-intersection exponent
allows.

The proof agent should make the flag-lift lemma explicit before spending constants.  If the exact
depth-4 enumerator finds even one reusable profile with a full extra `q`-dimension, `e=71` likely
needs either a stronger structural charge or a move to `e=72`.

## Manager Summary

Current lower-bound position:

```text
No corrected e=71 row above -80 is known.
The marked-core multicopy obstruction is dead.
The b>1 scalar exact-support scan is harmless.
The next falsification gate is exact flag enumeration, especially complete-stride nested kernels.
```
