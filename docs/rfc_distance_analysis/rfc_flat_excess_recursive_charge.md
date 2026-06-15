# RFC Flat-Excess Recursive Charge

Status: proof-structure note after the flat-excess correction.

The surplus incidence stratification shows that repair failure pays:

```text
t - 2D + 1 - flat_excess.
```

So near-MDS requires control of positive flat excess in the quotient singleton matroid `K_P|_T`.
This note records the first recursive way to view such flat excess.

## From Flat Excess To Child Rank Event

At the top step, condition on paired child positions `P`.

Let:

```text
R_P = rank_child(P)
D   = k_child - R_P
K_P = ker(child evaluation on P).
```

For a singleton subset `A subseteq T`, let:

```text
r = rank(K_P|_A)
a = |A|.
```

Then:

```text
rank_child(P union A) = R_P + r = k_child - D + r.
```

If `A` has flat excess:

```text
F = a - 2r,
```

then `P union A` is a child rank event with:

```text
child rank deficit = D-r,
child set size     = |P| + 2r + F.
```

Let:

```text
rho = |P| - R_P = |P| + D - k_child.
```

This is the nullity slack already present on the paired set `P`. Then:

```text
|P union A| - rank_child(P union A) = rho + r + F.
```

So an overfull quotient flat is not free: it is the shadow of a child set with a large internal
nullity.

## Large-Flat Versus Small-Flat Split

The ordinary near-MDS rank-tail theorem only sees child survivor sets of size at least `k_child`.
For `P union A`, this requires:

```text
|P| + 2r + F >= k_child.
```

Equivalently:

```text
r >= (k_child - |P| - F)/2.
```

For the dominant top-profile stress at `c=8,k=2048,e=71`, the tolerance model reports:

```text
|P| = 137
D   = 887
t   = 1845
rho = |P| + D - k_child = 0.
```

Thus for `F=0`, ordinary child rank-tail only sees flats with:

```text
r >= 444.
```

For `F=16`, it sees:

```text
r >= 436.
```

So flat-excess control splits into two regimes:

```text
large r: charge recursively as a child rank-tail event on P union A;
small r: ordinary distance does not see P union A, so we need a quotient-uniformity theorem.
```

## Deterministic Bookkeeping Script

The parameter translation is implemented in:

```text
scripts/rfc_distance_analysis/rfc_flat_excess_child_event.py
```

Example:

```text
python -B scripts/rfc_distance_analysis/rfc_flat_excess_child_event.py \
  --child-k 1024 --paired 137 --pair-deficit 887 --flat-excess 16 --step 64
```

Sample rows for the dominant profile:

```text
F=0:
flat_rank child_set_size child_rank_bound child_rank_deficit ordinary_visible
0         137            137              887                no
128       393            265              759                no
256       649            393              631                no
384       905            521              503                no
512       1161           649              375                yes

F=16:
flat_rank child_set_size child_rank_bound child_rank_deficit ordinary_visible
0         153            137              887                no
128       409            265              759                no
256       665            393              631                no
384       921            521              503                no
512       1177           649              375                yes
```

So the recursive distance/rank-tail charge only turns on around:

```text
r ~= 444 for F=0,
r ~= 436 for F=16.
```

## Meaning

This is a sharper blocker than "control flat excess" in the abstract.

To finish a near-MDS proof through the fixed-survivor route, we need one of:

```text
1. a recursive rank-tail charge for large-rank overloaded flats;
2. a local quotient-uniformity theorem ruling out or counting small-rank overloaded flats;
3. a global profile recurrence carrying flat rank r and flat excess F.
```

The small-rank regime is currently the harder part. It asks whether, after conditioning on a large
paired kernel `K_P`, the singleton evaluations `K_P|_T` can place too many points in a low-rank
flat. The reframing in:

```text
rfc_small_flat_subcode_charge.md
```

shows this is a generalized-Hamming-weight/subcode-zero question for the child RFC.
