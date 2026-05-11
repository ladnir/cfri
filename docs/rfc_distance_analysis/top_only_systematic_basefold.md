# Top-Only Systematic BaseFold

This note analyzes the idea:

```text
only the top layer has k raw systematic coordinates;
after the first fold, the protocol switches back to an ordinary non-systematic RFC.
```

There are two separate questions:

1. Does this work in the BaseFold/Blaze protocol?
2. If it works, what distance can we certify?

## BaseFold Recap

An ordinary RFC layer splits the message as

```text
m = (l, r),  l,r in F^{k/2}.
```

Let the lower code be

```text
Enc_{d-1}: F^{k/2} -> F^{n/2}.
```

Write

```text
L = Enc_{d-1}(l)
R = Enc_{d-1}(r).
```

The next RFC codeword is made from coordinate pairs such as

```text
(L_j + T_j R_j, L_j + T'_j R_j).
```

Folding at challenge `alpha` gives

```text
L_j + alpha R_j
```

up to the local interpolation convention. Therefore the folded oracle is exactly

```text
Enc_{d-1}(l + alpha r).
```

This is what makes the recursive verifier and the multilinear evaluation semantics line up.

## Strict Top-Only Systematic Attempt

The desired top codeword has raw message coordinates:

```text
C_top(l,r) starts with (l, r).
```

A natural first thought is to pair `l_j` with `r_j` and fold them affinely:

```text
(l_j, r_j) -> (1-alpha)l_j + alpha r_j.
```

This produces the next folded message coordinate. But in BaseFold, every top pair must fold into
one coordinate of the next oracle. If the next oracle is supposed to be an ordinary non-systematic
RFC codeword

```text
Enc_{d-1}((1-alpha)l + alpha r),
```

then the outputs of the systematic pairs must be coordinates of that ordinary RFC codeword.
They are raw message coordinates instead.

So the strict construction only works if the lower code has those output coordinates as a
systematic information set in the same message basis:

```text
some lower code coordinates equal the raw lower message coordinates.
```

An ordinary RFC does not have that property in its natural recursive basis. Therefore:

```text
strict top-only systematic -> ordinary lower RFC
```

does not work directly with the usual BaseFold fold/evaluation semantics.

This is the main protocol obstruction.

## General Linear-Lift Variant

There is a more flexible linear-code construction.

Pick two linear maps

```text
A_0, A_1 : F^k -> F^{k/2}.
```

Construct the top pairs so that their two endpoints are

```text
Enc_{d-1}(A_0 m)
Enc_{d-1}(A_1 m).
```

Then the first fold lands in the ordinary lower RFC:

```text
Enc_{d-1}((1-alpha)A_0 m + alpha A_1 m).
```

If the lower generator has an invertible row submatrix, then by choosing `A_0` and `A_1`
appropriately, selected top endpoint coordinates can be forced to equal arbitrary raw message
coordinates. This can make the top codeword systematic while the folded codeword is ordinary RFC.

As a pure foldable linear code, this is plausible.

But it changes the folded message from

```text
(1-alpha)l + alpha r
```

to

```text
(1-alpha)A_0 m + alpha A_1 m.
```

That is not the standard multilinear variable fold. For the PCS/BaseFold evaluation protocol, this
breaks the direct link between folding challenges and the claimed multilinear evaluation unless we
add new machinery to track arbitrary linear projections of the original message.

For Blaze/RMLE, the same concern appears: the backend is not only proving proximity to some code;
it is proving an evaluation/relation statement about the original input oracle. Arbitrary
`A_alpha m` folds are not free.

Therefore the general linear-lift variant is promising as coding theory, but not immediately
protocol-compatible.

## Protocol Conclusion

Top-only systematic can work in the existing protocol only under a strong compatibility condition:

```text
the first fold of raw systematic coordinates must land in the lower codeword positions
corresponding to the ordinary multilinear folded message.
```

That is true if the lower code is systematic in the relevant message basis. It is not true for an
ordinary RFC lower layer in its natural basis.

So we have three options:

1. **Append-systematic RFC.**
   Protocol-compatible now, but has the one-child distance limiter.

2. **Recursive/mixed systematic foldable code.**
   Keep the lower layers systematic enough that raw affine folds land in valid lower codeword
   coordinates. Protocol-compatible, but it is not top-only.

3. **Top-only linear lift into ordinary RFC.**
   Potentially good distance, but changes folded-message semantics. It needs a new protocol
   argument for evaluation/RMLE claims.

For the current Blaze2/BaseFold backend goal, option 2 is the safer path if we want better distance
without redesigning the evaluation protocol. Option 3 is worth investigating, but it is not a drop-in
replacement.

Project decision: proceed with option 2, the systematic-at-all-levels mixed code. The distance hit is
real, but it is explicit and certifiable. The top-only alternatives are not currently worth the
protocol/accounting risk.

## Distance Implications

If strict top-only into ordinary RFC were possible, it would likely fix the append construction's
one-child weakness. A one-child message would still have its top systematic coordinates folded into
a lower RFC instance, and from the second layer onward the ordinary RFC distance recurrence would
apply. The systematic penalty would be paid only at the top boundary.

But because strict top-only requires a systematic lower information set, the honest distance target
depends on which option we choose.

### Option 2: Mixed Recursive Systematic Code

Distance should be analyzed with typed coordinate classes at every layer:

```text
systematic coordinates: affine/projective fold
parity RFC coordinates: RFC finite-point fold
```

The support-stratified recurrence from `docs/rfc_distance_analysis/systematic_rfc_distance_analysis.md` remains relevant,
but the affine `T,T+1` split behavior may change if systematic coordinates participate in every
recursive layer instead of sitting as a flat top append.

The new certificate object should track two zero profiles:

```text
S_i(s) = maximum systematic-coordinate zeros for support s
P_i(s) = maximum parity-coordinate zeros for support s
```

with total distance

```text
d_i >= min_s (N_i - S_i(s) - P_i(s)).
```

### Option 3: Linear-Lift Top Layer

Distance is a punctured/lifted-code problem. The top code is essentially

```text
(raw m, remaining coordinates of Enc_{d-1}(A_0 m), Enc_{d-1}(A_1 m)).
```

For random-looking `A_0,A_1` chosen subject to systematic endpoint constraints, the distance may be
close to a random systematic linear code plus an ordinary RFC lower code. This could be excellent.

But a distance proof alone is insufficient; the protocol must also prove that the transformed fold
still certifies the original RMLE/evaluation claim.

## Immediate Next Checks

1. Try to prove an impossibility lemma for strict top-only with natural RFC lower basis:

   ```text
   if raw systematic pair folds to raw lower message coordinates, then lower code has those
   coordinates systematic in the same basis.
   ```

2. Decide whether arbitrary `A_alpha m` folds can be reconciled with the sumcheck/RMLE relation
   without adding too much proof material.

3. If not, focus on option 2: a recursive mixed systematic code with typed fold rules and a
   support-stratified distance certificate.
