# RFC Fixed-Survivor Rank Recurrence Target

Status: proof target extracted from `rfc_fixed_survivor_rank_tail.py` diagnostics. This is for the
original non-systematic RFC with determinant-1 fold and nonzero roots.

## Fixed-Set Object

For a survivor coordinate set `S` at parent depth `d`, let:

```text
R_d(S) = rank G_d(T)|_S.
```

Distance by erasure recovery asks for high-probability lower bounds on `R_d(S)` for every large
survivor set `S`. Equivalently, for zero-set size `z`, use survivor size `n-z`.

At the top fold, split child positions into:

```text
P = positions where both siblings survive,
T = positions where exactly one sibling survives,
U = P union T.
```

Let `k_d = 2^d`, so the child dimension is `k_{d-1}`.

## Exact Paired Compression

The diagnostic consistently verifies the exact identity:

```text
rank(parent restricted to paired coordinates over P)
  = 2 * rank_child(P).
```

This should be proved directly from the determinant-1 pair matrix: on paired coordinates, the two
parent equations are an invertible linear transform of the two child half-codeword symbols.

So the rank recurrence has the form:

```text
R_d(S) = 2 * R_{d-1}(P) + I_d(P,T),
```

where `I_d(P,T)` is the singleton rank increment modulo the paired block.

## Repair Condition

Let the child pair deficit be:

```text
D = k_{d-1} - R_{d-1}(P).
```

The parent dimension is `k_d = 2 k_{d-1}`, so parent full rank is equivalent to:

```text
I_d(P,T) >= 2D.
```

The parent rank deficit is:

```text
k_d - R_d(S) = max(0, 2D - I_d(P,T)).
```

This matches the exact/sampled histograms:

```text
pair/inc 2/1 is bad when k_d=4 and pair block rank 2 needs increment 2.
pair/inc 2/2 repairs the same child deficit.
pair/inc 4/0 is already full rank.
```

## Local Singleton Theorem Needed

For fixed child code and paired set `P`, prove a lower-tail bound:

```text
Pr_T[ I_d(P,T) < 2D | R_{d-1}(P)=k_{d-1}-D ].
```

The trivial cap is:

```text
I_d(P,T) <= |T|.
```

Therefore if `|T| < 2D`, the parent cannot repair the child pair deficit. This explains the
pair-heavy hard rows with only one singleton: a one-rank child deficit requires two singleton rank
increments.

The useful theorem must cover `|T| >= 2D`: diagonal fold randomness should make the singleton
constraints transverse to the paired-block quotient except with a probability that can be charged.

The current diagnostic groups singleton repair events by:

```text
(|P|, |T|, D, rank_child(U)).
```

In the checked small cases this grouping already predicts parent failure, because the pair-block
identity reduces parent failure to repair failure:

```text
I_d(P,T) < 2D.
```

Empirical local repair examples at `q=5`:

```text
|T|=1, D=1  -> repair fails always.
|T|=2, D=1  -> repair failure around q^-0.77 to q^-0.94 in checked rows.
|T|=3, D=1  -> repair failure around q^-1.07 in checked rows.
|T|=6, D=2  -> repair failure around q^-1.79 in checked rows.
|T|=7, D=2  -> repair failure around q^-2.45 in checked rows.
```

This suggests the local theorem may be a finite-field diagonal-transversality bound depending mainly
on singleton surplus over `2D`, with correction terms for the child quotient geometry.

The simple key is not yet theorem-tight. In sampled `d=2,c=4,q=5` structured rows, the same
`(|P|,|T|,D,rank_child(U))` group can have a much worse per-set repair rate than its aggregate:

```text
|P|=2, |T|=2, D=1, rank_child(U)=2:
  aggregate repair_fail ~= q^-0.774
  worst-set repair_fail ~= q^-0.311
```

So a rigorous local theorem likely needs one more invariant: multilevel profile, quotient matroid,
or a direct geometric parameter of the paired-kernel quotient as seen on singleton coordinates.

The first quotient diagnostic is now implemented behind:

```text
--quotient-signature
```

It groups each repair event by the rank-increment histogram:

```text
(|A|, rank_child(P union A)-rank_child(P)) for A subseteq T.
```

This is a child-column proxy for the kernel object `K_P|_T`. In the first targeted checks it splits
the worst profile into deterministic invisible-singleton rows and visible-but-probabilistic
root-line repair rows. This points to a two-stage local lemma:

```text
1. Visibility/Hall stage: rule out deterministic failure from rank(K_P|_T) < D
   or from the two-copy capacity/matroid Hall obstruction.
2. Root-line anti-concentration stage: once a full-rank minor exists, bound the
   bad alpha roots on the nonzero challenge domain.
```

The safe first theorem target is only `q^-1` for non-deterministic repair failure. A stronger
surplus exponent can be pursued after the root-line rank-drop strata are classified.

The diagnostic now also computes the generic root-line Hall quantity:

```text
hall_min(K_P|_T) = min_A ( |T|-|A| + 2 rank(K_P|_A) )
hall_deficit     = max(0, 2D - hall_min).
```

For `T<=3`, `--rootline-exact-max 3` can exactly enumerate the actual finite-field top-root repair
probability. On the `d=2,c=4,q=5,k+2` bad balanced row, this gives:

```text
hall_deficit > 0  -> exact_rootline_fail = 1
hall_deficit = 0  -> exact_rootline_fail = 1/4
```

and the sampled parent repair rates follow those exact values. This is evidence that the right
local abstraction may be:

```text
rank function of K_P|_T
plus root-line rank-drop layer codimensions
```

rather than full projective child state. This remains unproved; it should be attacked with larger
`T,D` rows before being promoted to a recurrence.

The first larger check used exact root-line enumeration for `T<=6` on the `T=6,D=2` rows. It found
no ambiguity:

```text
(singleton side pattern, K_P|_T rank/Hall signature) -> exact root-line failure probability
```

was single-valued for all signatures observed in the `d=2,c=4,q=5,k+2` top-family sample. The
Hall-OK rows had different exponents, but those differences were explained by different
rank-function signatures:

```text
exact_rootline_fail = 67/256, 31/256, 11/128, 13/256, 39/1024, 7/256, ...
```

So Hall should be used as a deterministic-generic-failure gate, not as the whole exponent. The
certificate recurrence needs a local table/theorem for the first root-line rank-drop probability of
`K_P|_T` after Hall passes.

A mixed-side block-family check showed that this statement needs one refinement. The coarse
rank/Hall histogram can be ambiguous once left and right singleton domains mix:

```text
rootline_signature_ambiguity total_signatures=96 ambiguous_signatures=54
```

But the `D=2` projective parallel-class signature with side counts was still single-valued:

```text
rootline_parallel_class_ambiguity total_signatures=105 ambiguous_signatures=0
```

So the current `D=2` local theorem candidate is not "rank histogram only." It is:

```text
zero singleton columns in K_P|_T,
multiset of (left_count, right_count) over projective parallel classes of K_P|_T.
```

See `rfc_d2_rootline_parallel_class_lemma.md`.

The diagnostic now contains an executable version of this local table:

```text
parallel_class_formula_failure_d2
```

It matched actual exact root-line enumeration on the mixed-side `q=5,D=2,T=6` block sample:

```text
rootline_parallel_formula_mismatches=0
```

The next recurrence step can therefore treat `D=2` repair as a finite local table over
parallel-class side-count signatures, while the proof work derives that table symbolically.

## Recursive Rank-Tail Shape

A plausible recurrence for fixed survivor sets is:

```text
Pr[R_d(S) < k_d]
  <= sum_D Pr[R_{d-1}(P)=k_{d-1}-D]
        * Pr[I_d(P,T)<2D | D].
```

For distance, this must be unioned over survivor sets or run through a profile grammar that counts
structured families. The diagnostics suggest the hard families are small and recursive:

```text
balanced-at-each-level,
paired-spine pushing the deficit into the child,
singleton-heavy/block families as secondary cases.
```

## Immediate Experiments

1. For each displayed hard set, compute the exact distribution of `(D, I_d(P,T))`.
2. Group by `(D, |T|, rank_child(U), multilevel profile)` and test whether the repair probability is
   stable across positions.
3. Add the quotient signature/root-line Hall invariant and test whether the deterministic repair
   failures are exactly the invisible/Hall-obstructed rows.
4. Add a root-line first-drop codimension profiler for Hall-OK rows.
5. If stable, promote those grouped probabilities to a finite profile recurrence.
6. If cross-ratio/projective data is needed to predict repair, the projective wall has reappeared in
   the fixed-set route.
