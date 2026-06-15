# Upgrading The Original Binary RFC Proof Strategy

Question: after the later obstruction work, can we return to the original proof strategy and
upgrade it instead of replacing it?

## Short Answer

Yes, this is probably the cleanest binary path now.

The upgrade should keep the original fixed-survivor proof skeleton:

```text
fix a survivor/zero set
split top coordinates into paired and singleton children
paired coordinates compress exactly
singletons repair the paired deficit through random root lines
union/first-moment over survivor profiles
```

The new work upgrades the singleton repair step. The original-style argument can use:

```text
paired compression identity
root-line generic rank by matroid union
surplus root-incidence exponent
flat-excess correction
flat-excess control through child zero-set, closure-tail, and PA-chain reductions
```

This is more promising than continuing row-by-row diagram patches, because it uses the same core
logic as the paper but replaces the crude local probability bound by the sharper structure we have
learned.

## Original Skeleton

For a fixed parent survivor set `S`, split child positions into:

```text
P = positions where both siblings survive
T = positions where exactly one sibling survives
```

Let:

```text
R_P = child rank on P
D   = k_child - R_P
```

Paired compression is exact:

```text
rank(parent on paired P) = 2 R_P.
```

So the singleton part must add:

```text
2D
```

rank modulo the paired block.

The singleton rows have root-line form:

```text
(ell_i, alpha_i ell_i) in K_P^* plus K_P^*
```

where `K_P` is the child kernel on the paired coordinates and `dim K_P=D`.

The original proof style then asks whether these singleton root lines repair the `2D` missing
rank.

## New Local Upgrade

The generic rank of singleton repair is now clean:

```text
rank_generic = min_A ( |T|-|A| + 2 r(A) )
```

where:

```text
r(A) = rank{ell_i : i in A}.
```

This is the matroid-union/root-line generic rank lemma.

If:

```text
r(T)=D
and
min_A ( |T|-|A| + 2r(A) ) >= 2D,
```

then generic repair succeeds.

The old Schwartz-Zippel version only used the fact that one determinant is nonzero, giving a weak
failure probability like:

```text
O(D/q).
```

Our new incidence stratification gives the stronger exponent:

```text
Pr[repair failure]
  <= poly(D,t) q^{-(t - 2D + 1 - flat_excess)}
```

where:

```text
t = |T|
flat_excess = max_{A: r(A)<D} ( |A| - 2r(A) ).
```

This is the main upgrade.

## What This Buys

If `flat_excess=0`, the local exponent is:

```text
t - 2D + 1.
```

This is exactly the exponent that made the top-profile tolerance calculator cross near:

```text
c=8,k=2048,q=2^128: e ~= 71.
```

If `flat_excess=F`, the crossing shifts roughly to:

```text
e ~= 71 + F.
```

So we do not need to prove perfect behavior everywhere. A bounded or chargeable flat excess still
gives a strong near-MDS statement.

## Where The New Obstruction Work Fits

The late-stage obstruction work should be treated as a support package for flat excess, not as a
replacement proof.

### 1. Flat-Excess Control Stack

Positive flat excess means there is an overloaded low-rank set:

```text
|A| > 2 r(A).
```

The first bridge is the marked rank identity:

```text
r(A) = rank(P union A)-rank(P).
```

But the current proof stack is narrower than a full marked-pair recurrence. It splits flat-excess
witnesses into:

```text
high-rank endpoint:
  ordinary child line-zero/rank-tail event;

low-rank endpoint:
  marked closure event A subset cl(P);

multi-PA closure chains:
  safe child one-rank marked tail,
  reduced by circuit union to one-mark closure.
```

The current map is recorded in:

```text
rfc_flat_excess_control_stack.md
```

### 2. PA And Mixed-Profile Repairs

For pure all-mixed PA blocks, the corrected identity says:

```text
A_alpha(J) = Full(J) - P_alpha(J).
```

For pure all-mixed PA, low marked rank is therefore a full two-copy span deficiency, not a rare
bad-root event. This is good for the upgraded proof because it routes the scary mixed case back to
paired-compression/child-rank style structure.

So all-mixed PA is not a separate finite-root disaster. It is a recursive full-span deficiency
charge.

For the low-rank closure endpoint, multi-PA chains are handled by:

```text
parent PA closure
  -> child rank-increment drop
  -> minimal circuit
  -> one-mark child closure with enlarged core.
```

This avoids relying on the false strong statement that all PA marks are individually in child
closure of the non-PA core.

### 3. Diagram-State Work

The diagram-state work remains useful, but it should be demoted from the main architecture to a
finite/local verifier for the marked incremental lemma. It tells us which small support profiles
can break naive bounds and what extra flags are needed.

The main proof statement should not expose all those flags unless necessary.

## Proposed Upgraded Theorem

A plausible theorem package is:

```text
Theorem A: paired compression exactness.
Theorem B: root-line generic repair rank equals matroid-union Hall value.
Theorem C: Hall-OK repair failure has incidence exponent
           t - 2D + 1 - flat_excess.
Theorem D: flat_excess F in the singleton quotient is charged by the flat-excess
           control stack: high-rank child line-zero, low-rank closure-tail, and
           PA-chain one-rank/circuit reduction.
Theorem E: pure all-mixed PA routes to full-span deficiency, not finite-root
           coincidence.
```

Then the certificate becomes:

```text
B_d(1,k+e)
  <= sum_survivor_profiles
       paired_child_profile_count
       * q^{-(singleton_surplus - charged_flat_excess)}
       * recursive_marked_tail_terms
```

For `c=8,k=2048,q=2^128`, this should target:

```text
e = 71 + F_budget
```

where `F_budget` is the proven bound or charged effective flat excess. Even `F_budget <= 16` would
still be excellent.

## Why This Is Better Than The Pure New Route

The pure new route tried to prove a fully general marked-pair recurrence from scratch. That became
state-heavy.

The upgraded original route only needs extra machinery for the cases where the original surplus
exponent degrades:

```text
flat_excess > 0.
```

The latest reduction makes this target smaller still: for the low-rank PA endpoint, it needs
one-mark closure-tail bounds plus a circuit union factor, not a full arbitrary marked-rank theorem.

The proof would still look like the original BaseFold proof:

```text
survivor set -> paired compression -> singleton repair -> first moment.
```

The difference is that singleton repair is no longer treated with a one-minor bound. It gets the
full incidence exponent, corrected by a flat-excess term that is then recursively charged.

## Main Remaining Blocker

The blocker is now precise:

```text
prove the closure-tail recurrence and integrate it with the flat-excess stack.
```

A strong version:

```text
dominant singleton quotient profiles have flat_excess = O(1).
```

A more flexible version:

```text
profiles with flat_excess = F pay through endpoint-specific charges
```

via ordinary child rank-tail, closure-tail, PA-chain circuit reduction, and full-span deficiency.

If this fails, then the original proof cannot be upgraded to near-MDS by local repair alone. The
bad structures would be real: survivor sets would concentrate singleton functionals in overloaded
flats often enough to erase the surplus exponent.

## Recommendation

Return to the original proof skeleton, but with the upgraded local theorem:

```text
singleton repair exponent = t - 2D + 1 - flat_excess.
```

Then attack only the flat-excess charge. This is a cleaner route than trying to finish the entire
diagram grammar, and it directly answers whether the binary RFC is near-MDS for the same reason the
paper's proof almost shows it.
