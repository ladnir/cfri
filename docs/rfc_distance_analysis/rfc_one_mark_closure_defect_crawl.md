# One-Mark Closure Defect Crawl

Status: Tier-2 bounded experiment.

## Purpose

Tier 2 asks whether the low-rank endpoint can be handled without reopening the whole flag-state
machine. The endpoint for one marked coordinate is:

```text
C_d(p,1) = E[# {(P,a): |P|=p, a notin P, a in cl(P)}].
```

The first diagnostic recurrence assumed the core `P` was effectively full rank in the direct `A0`
branch. This note records the defect-safe correction.

## Key Point

Trying to recurse on an exact state:

```text
C_d(p,1;u),    rank(P)=p-u
```

is probably the wrong first move. In the `PA` branch, parent rank defect can arise from several
child projection geometries, so exact `u` is unlikely to be closed by itself.

The cleaner bounded crawl is:

```text
use u only to stratify the direct A0 branch;
leave PA as a total child one-mark closure overcount.
```

This keeps the recurrence to one scalar state `p`, while ensuring the direct branch no longer
gets a hidden full-rank-core assumption.

## Direct A0 Correction

In the `A0` case, the marked coordinate's sibling is not in `P`.

If:

```text
rank(P)=p-u
and
a in cl(P),
```

then:

```text
rank(P union {a}) <= p-u.
```

Thus the direct branch is contained in an ordinary rank-tail event on a fixed set of size `p+1`.
In the random-matrix scale, this costs:

```text
(u+1)(k-p+u)
```

q-dimensions, since:

```text
m = p+1
r = p-u
(m-r)(k-r) = (u+1)(k-p+u).
```

For `u=0`, this recovers the old direct exponent:

```text
k-p.
```

For `u>0`, the defect is not free; it is charged by the ordinary rank-tail of `P union {a}`.

## Defect-Safe Recurrence Shape

The candidate one-mark recurrence is:

```text
C_d(p,1)
  <= A0_defect(d,p)
     + PA_total(d,p),
```

where:

```text
A0_defect(d,p)
  <= n_d binom(n_d-2,p)
     sum_{u=0}^p q^{-(u+1)(k_d-p+u)}.
```

The PA term remains the conservative child-closure lift:

```text
PA_total(d,p)
  <= sum_y C_{d-1}(y,1)
       2^{1+y} binom(2(n_{d-1}-1)-y, p-1-y).
```

The PA projection lemma gives the deterministic implication used here:

```text
parent PA closure
  -> child marked coordinate lies in the projection span of the other P columns.
```

The lift overcounts by choosing one parent side over every child core coordinate and then choosing
all remaining parent core coordinates arbitrarily.

## Target Diagnostic

The helper:

```text
scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_defect_scale.py
```

implements this defect-safe A0 correction and keeps the old conservative PA lift.

Target run:

```text
python scripts/rfc_distance_analysis/rfc_closure_tail_one_mark_defect_scale.py \
  --depth 10 --expansion 8 --q-log2 128 --p 137 --residual-q-exp 71
```

Output:

```text
log2_C        = 1389.160433
combined_log2 = -7698.839567
status        = pass
```

The dominant trace is still the PA chain:

```text
137 -> 119 -> 101 -> 84 -> 67 -> 51 -> 37 -> 25 -> 15 -> 8 -> 3.
```

So adding the defect-safe direct branch does not create a new dominant term.

## Go/No-Go Verdict

This is good news for Tier 2, but not yet a proof.

What looks safe:

```text
Direct A0 defects can be routed to ordinary rank-tail on P union {a}.
PA can be overcounted by total child one-mark closure without tracking parent u.
The target has about 7700 bits of slack after residual repair.
```

What remains open:

```text
1. Replace the random-matrix A0 rank-tail exponent by the actual RFC child rank-tail bound.
2. Prove the PA lift recurrence as a genuine containment/counting statement.
3. Add finite constants and nonzero-root normalization.
```

The important go/no-go result is that the first defect correction does not force arbitrary child
projection matroids. Tier 2 remains alive as a bounded theorem target.

## Next Step

The PA lift containment is now recorded in:

```text
docs/rfc_distance_analysis/rfc_one_mark_pa_lift_containment.md
```

It proves:

```text
{ parent PA one-mark closure with p core coordinates }
  subseteq
union_y { child one-mark closure with y core coordinates } x Lift(p,y).
```

The next remaining Tier-2 step is therefore the direct `A0` rank-tail theorem for RFC, replacing
the random-matrix exponent in this diagnostic by the actual ordinary rank-tail object already used
by the original proof.
