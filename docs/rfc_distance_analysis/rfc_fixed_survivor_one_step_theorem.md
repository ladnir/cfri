# RFC Fixed-Survivor One-Step Rank Theorem

Status: local recurrence theorem for the original non-systematic RFC.

This packages the local bricks:

```text
paired compression,
root-line generic rank,
Schwartz-Zippel repair envelope.
```

It is a one-step theorem for a fixed survivor set. It is not yet the global near-MDS certificate,
because the final proof still needs survivor-set counting or a profile grammar.

## Setup

Fix a parent-depth survivor set `S`. At the top fold, split child positions into:

```text
P = child positions where both siblings survive,
T = child positions where exactly one sibling survives.
```

Condition on all lower-level RFC challenges. Let:

```text
R_P = rank(child restricted to P),
D   = k_child - R_P,
K_P = ker(child evaluation on P).
```

Thus:

```text
dim K_P = D.
```

For each singleton position `j in T`, let:

```text
ell_j in K_P^*
```

be the evaluation functional restricted to `K_P`.

## Paired Part

By `rfc_paired_compression_rank_lemma.md`, the paired parent block has rank:

```text
2 R_P = 2(k_child-D).
```

Therefore parent full rank requires the singleton block to add:

```text
2D
```

new rank modulo the paired block.

## Root-Line Singleton Part

Modulo the paired block, singleton rows are:

```text
(ell_j, alpha_j ell_j) in K_P^* plus K_P^*.
```

The side domains are:

```text
left singleton:  alpha_j in F_q^*
right singleton: alpha_j in F_q \ {1}.
```

Define the Hall value:

```text
H(P,T) = min_{A subseteq T} ( |T|-|A| + 2 rank{ell_j : j in A} ).
```

By `rfc_rootline_generic_rank_lemma.md`, the generic singleton rank is:

```text
min(D*2, H(P,T))
```

assuming `rank{ell_j : j in T}` is capped by `D`.

In particular, if:

```text
rank{ell_j : j in T} = D
and
H(P,T) >= 2D,
```

then some `2D x 2D` singleton repair determinant is a nonzero polynomial.

## Conditional Failure Bound

If `D=0`, the parent is already full rank on paired coordinates.

If `D>0` and either:

```text
rank{ell_j : j in T} < D
or
H(P,T) < 2D,
```

then the top singleton set is generically unable to repair the child pair deficit. This is a
deterministic structural failure for this one-step survivor profile.

If `D>0`, `rank{ell_j : j in T}=D`, and `H(P,T) >= 2D`, then the parent full-rank failure
probability over the top fold roots is bounded by:

```text
Pr_top[parent restricted to S is rank-deficient | lower challenges]
  <= D/(q-1).
```

This is the determinant-envelope bound from `rfc_rootline_schwartz_zippel_envelope.md`.

For `D=2`, one may replace `D/(q-1)` by the sharper PGL2/parallel-class envelope from:

```text
rfc_d2_rootline_pgl2_envelope.md.
```

## One-Step Recurrence Form

Let `BadStruct(P,T)` be the event, depending on lower challenges, that:

```text
D>0
and
(
  rank{ell_j : j in T} < D
  or
  H(P,T) < 2D
).
```

Then for fixed `S`:

```text
Pr[parent rank on S < k_parent]
  <= Pr_lower[BadStruct(P,T)]
     + E_lower[ 1_{D>0 and not BadStruct(P,T)} * D/(q-1) ].
```

This is the first theorem-shaped fixed-survivor rank-tail recurrence.

## Strength Warning

The `D/(q-1)` term is a valid local one-minor envelope, but it is too weak for near-MDS global
counting. The top-profile tolerance check in:

```text
rfc_surplus_repair_codimension_target.md
```

shows that target parameters need the singleton surplus to pay approximately:

```text
q^{-(|T|-2D+1)}
```

in Hall-OK cases, not merely one q-factor.

## What Remains Global

This theorem does not yet count how many survivor sets have a bad structural profile. To turn it
into a near-MDS distance statement, we still need one of:

```text
1. a profile grammar that upper-bounds the number of fixed survivor sets feeding each child
   structural event;
2. a first-moment recurrence for the expected number of bad survivor sets;
3. a stronger local envelope that gives enough q-exponent to union over all fixed sets directly.
```

The current local theorem says where random root factors appear. It does not by itself prove that
there are enough such factors to beat the entropy of all near-MDS survivor sets.
