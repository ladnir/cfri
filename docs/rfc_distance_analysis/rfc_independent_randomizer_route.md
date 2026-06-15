# RFC Independent-Randomizer Route

Status: broad alternative under evaluation.

## Variant

Replace the linked top fold:

```text
L_j = u_j + t_j w_j
R_j = u_j + (t_j+1) w_j
```

by independent slopes:

```text
L_j = u_j + a_j w_j
R_j = u_j + b_j w_j
```

sampled with:

```text
a_j != b_j.
```

The local pair map has determinant:

```text
b_j-a_j,
```

so paired compression remains invertible after conditioning.

## Why Consider It

The current `t,t+1` distribution is protocol-clean and still viable, but the proof has become
state-rich. The main trouble is marked incremental rank, especially mixed `PA` categories. The pure
all-mixed `PA` stress was eventually reduced to full two-copy span deficiency, but this required a
substantial amount of bookkeeping:

```text
marked sets -> PA categories -> graph contraction -> full-span reduction.
```

With independent `(a,b)`, the two sibling root-lines at a child coordinate are no longer tied by a
fixed translation. The hope is that a marked `A` sibling in a `PA` pair has genuinely fresh slope
randomness relative to the `P` sibling, turning multi-`PA` into ordinary finite-root incidence or a
clean matroid-union rank problem.

## Immediate Local Algebra

At one child coordinate:

```text
P sibling: p_j = x_j + a_j y_j
A sibling: q_j = x_j + b_j y_j
```

Modulo `p_j`, the marked sibling is:

```text
q_j - p_j = (b_j-a_j) y_j.
```

Since `b_j-a_j != 0`, the marked increment modulo the same-coordinate `P` line is exactly the
`y_j` direction. Thus the pure all-mixed identity remains:

```text
A_{a,b}(J) = Full(J) - P_a(J),
```

where:

```text
P_a(J) = rank(U + span{x_j+a_j y_j}) - rank(U).
```

The important difference is distributional:

```text
the marked slope b_j does not affect the quotient direction after p_j is contracted;
only the nonzero difference b_j-a_j matters as a scalar.
```

Therefore, in pure `PA`, independent slopes do not magically add a second random constraint after
conditioning on the `P` sibling. The event is still controlled by full-span deficiency plus the rank
of the `P` graph lines.

For `A0` singletons and mixed profiles where the `A` coordinate is not paired with a `P` sibling,
independence should help more directly: the `A` root-line slope is fresh rather than forced by a
neighboring slope.

## Conditioning Cost

Sampling `(a,b)` uniformly with `a != b` gives:

```text
q(q-1)
```

ordered pairs if zero slopes are allowed, or:

```text
(q-1)(q-2)
```

if both slopes must be nonzero. Either way, conditioning costs only a constant factor:

```text
1 + O(1/q)
```

per coordinate relative to independent sampling. At `q=2^128`, this is negligible for bit slack,
but theorem statements should carry a small finite-field constant.

## Candidate Proof Routes

### Route A: Colored Matroid-Union Recurrence

Treat the two sibling sides as independently colored root-line columns. For each top profile, first
contract paired/core columns, then apply a colored matroid-union generic-rank theorem to singleton
and marked columns.

Expected benefit:

```text
fewer affine graph-contraction special cases for non-pure PA/A0 mixtures.
```

Remaining obstruction:

```text
pure all-mixed PA still routes to full two-copy span deficiency.
```

### Route B: Marked Incremental Recurrence With Independent Colors

Keep the marked object:

```text
I_d(P,A,r) = rank(P union A)-rank(P) <= r.
```

But the one-step state uses independent color labels for each singleton/marked sibling. The local
finite-root theorem should become closer to ordinary random row/column rank:

```text
rank(A modulo P) drops by s => at least s independent color equations,
unless a child full-span/projection event occurs.
```

### Route C: Generalized Subcode-Zero / Short-Set Rank Tail

Independent local mixing may make generalized-weight style bounds cleaner for small flats:

```text
h-dimensional subcode vanishing on z coordinates
```

could be charged directly through independent coordinates rather than tracked by linked root-line
families.

This route should be revisited only if local marked algebra looks substantially simpler.

## Early Assessment

Independent randomizers likely help, but not in the most naive way. For a pure `PA` pair, after
contracting the `P` sibling, the `A` sibling is simply a scalar multiple of the missing `y_j`
direction. The second independent slope does not create a new direction; it only guarantees a
nonzero scalar.

So the main question is:

```text
Are the hard profiles mostly pure PA, or mixed PA/A0/AA/root-line profiles?
```

If hard profiles are pure PA, independent randomizers may not buy much over the full-span reduction
we already found. If hard profiles involve interactions between PA and A0/singletons, independent
colors may simplify the recurrence significantly.

## Decision Criteria

Switch toward independent randomizers if at least one of these is true:

```text
1. The local colored matroid-union theorem gives a clean surplus codimension for mixed marked
   profiles without carrying relation fibers.
2. The global recurrence can avoid the marked PA/full-span state explosion.
3. Obstruction search finds no new bad family and estimates improve materially.
```

Stay with current `t,t+1` if:

```text
1. pure PA/full-span remains the dominant state either way;
2. independent colors only improve lower-order finite-root constants;
3. conditioning and changed construction add proof surface without a simpler recurrence.
```

## Open Questions For Subagents

1. Is there a clean generic-rank theorem for independent colored root-lines after contracting a
   marked `P` core?
2. Does the finite-field bad variety have codimension growing with surplus, or only one-minor
   codimension?
3. Are all-paired and all-mixed PA spines still the dominant obstruction?
4. Can the certificate recurrence be reduced to shape counts plus a small number of rank-tail
   states?

## Subagent Synthesis

Three independent review lanes were launched:

```text
local algebra,
global recurrence/certificate,
skeptical obstruction.
```

The local and skeptical lanes agree on the main correction:

```text
independent (a,b) does not remove pure PA/all-mixed geometry.
```

For:

```text
L = x + a y,
R = x + b y,
delta = b-a != 0,
```

contracting the `P` sibling gives:

```text
R-L = delta y.
```

Thus the marked sibling is just the missing `y` direction up to a nonzero scalar. The second
randomizer disappears from the rank question in pure `PA`. The hard all-mixed identity remains:

```text
A(J) = Full(J) - P_a(J).
```

The global recurrence lane is more optimistic for a different reason: independent randomizers may
make the ordinary singleton-repair theorem cleaner. If one can prove a local random-like rank-tail:

```text
Pr[rank <= 2D-r] <= poly * q^{-r(t-2D+r)},
```

then the certificate recurrence could avoid much of the marked-state machinery for non-pure
profiles. The expected distance estimates would remain roughly the same:

```text
c=8, k=2048, q=2^128:
  excess e ~= 71,
  distance ~= 14266,
  gap to MDS ~= 71.

c=4, k=2048, q=2^128:
  excess e ~= 53,
  distance ~= 6092,
  gap to MDS ~= 53.
```

The skeptical lane found no new deterministic low-distance family, but noted a proof hygiene issue:
if slopes are sampled in `F_q^*` with `a != b`, then `(a,delta=b-a)` is not a product distribution
because `a+delta != 0`. This should cost only tiny finite-field constants, but a theorem should use
restricted-domain Schwartz-Zippel/counting rather than pretending all variables are iid.

## Revised Assessment

Independent randomizers are not a broad escape hatch from the latest all-mixed `PA` blocker. They
are best viewed as a cleaned-up variant that may simplify:

```text
A0/singleton repair,
mixed PA+A0/AA profiles,
finite-field side-domain constants.
```

They do not simplify:

```text
all-paired spines,
pure all-mixed PA full-span deficiency.
```

Therefore the next useful test is computational/local:

```text
add independent-randomizer support to the small-field oracle/profiler,
compare linked vs independent on mixed profiles beyond pure all-PA,
and check whether the random-like rank-tail recurrence is empirically plausible.
```

If those mixed profiles simplify, independent randomizers may still raise the proof odds. If they
do not, the variant is mostly cosmetic and the current `t,t+1` route remains the right primary path.
