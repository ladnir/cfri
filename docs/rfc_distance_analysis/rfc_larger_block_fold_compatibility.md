# Larger-Block RFC Folding Compatibility

Question: if we replace the binary RFC local block by larger blocks, especially 4-to-4 blocks,
does the foldable-code protocol still work, and would this help the distance proof?

## Current Binary Contract

The current BaseFold/Blaze backend contract is binary. A round splits the codeword into two halves
and exposes pairs:

```text
fold_pair(round, output_index) -> (left, right, out)
```

The non-systematic RFC/parity block uses two local points. In the project convention these are
linked as:

```text
T' = T + 1
```

For a child coordinate pair `(L_j, R_j)`, the parent stores two affine evaluations of the line
through the child values. Folding at verifier challenge `alpha` evaluates the same line at
`alpha`, so the folded word is the lower code applied to the folded message.

This is why the existing Rust and spec are pair-shaped:

```text
left  = output_index
right = output_index + half_len
```

and the FRI verifier paths read `j` and `j + domain_size / 2`.

The Blaze boundary is also explicit about query accounting: if a fold equation needs two
compiler-systematic input positions, both positions must be supplied as input queries. A larger
block would similarly need every queried systematic sibling to be explicitly opened.

## True Arity-4 Foldable RFC

A genuine 4-to-4 version is algebraically compatible with folding.

At level `i`, split the message into four children:

```text
m = (m_0, m_1, m_2, m_3)
```

Let the lower encoder be `C_{i-1}`. For each lower coordinate `j`, form the four child values:

```text
v_s(j) = C_{i-1}(m_s)[j],  s in {0,1,2,3}.
```

Choose four distinct local domain points:

```text
beta_0, beta_1, beta_2, beta_3
```

and define the unique degree-at-most-3 polynomial `p_j(X)` satisfying:

```text
p_j(beta_s) = v_s(j).
```

Then the arity-4 parent block may store:

```text
p_j(theta_0), p_j(theta_1), p_j(theta_2), p_j(theta_3)
```

for four verifier-known, distinct output points `theta_h`. The `theta_h` can be random per
coordinate, conditioned on distinctness and on invertibility of the local interpolation matrix.

Folding at challenge `alpha` computes:

```text
p_j(alpha) = sum_s lambda_s(alpha) C_{i-1}(m_s)[j],
```

where `lambda_s` are the Lagrange coefficients for the `beta_s`. By linearity of `C_{i-1}`:

```text
p_j(alpha) = C_{i-1}(sum_s lambda_s(alpha) m_s)[j].
```

So the folded word is exactly:

```text
C_{i-1}(m_alpha),
where m_alpha = sum_s lambda_s(alpha) m_s.
```

This is the direct arity-4 analogue of the binary RFC fold.

## Compatibility Cost

This is not drop-in compatible with the current implementation or verifier interface.

Required protocol/API changes:

```text
fold_pair(...)                 -> fold_block(..., arity=4)
FoldPair                       -> FoldBlock { inputs: [usize; 4], out: usize }
binary half-domain query       -> quarter-domain sibling query
one fold challenge per bit     -> one arity-4 interpolation challenge, or two binary variables
two local values in query path -> four local values in query path
line interpolation             -> cubic interpolation
```

For Blaze systematic input positions, a top query that touches an arity-4 systematic block can cost
up to four authenticated interleaved-column openings, unless some siblings are non-systematic
backend-proof positions. This is not hidden inside the backend proof; it has to be counted exactly.

The total sibling-value count per path is not necessarily worse asymptotically. Binary folding uses
about:

```text
2 * log2(k)
```

local values across all rounds. Arity-4 folding uses about:

```text
4 * log4(k) = 2 * log2(k)
```

local values. The number of committed/folded layers is halved, but each local check is wider and
uses higher-degree interpolation.

## Tensorized 4-to-4 Is Mostly Cosmetic

There is a cheap way to get a 4-symbol block: batch two binary folds. This views a 4-block as a
2-by-2 tensor of the existing line folds.

That is highly compatible with the multilinear/RMLE picture: it folds two Boolean variables at
once. It is also much easier to implement because it can be decomposed into existing pair checks.

But it probably does not help the distance proof much. The obstruction geometry factorizes into
the same binary root-line events we have been fighting. It reduces round count, but it does not
create a genuinely new local random subspace.

## True 4-Point Blocks May Help The Proof

The proof-friendly version is the true degree-3 local interpolation block, not a tensor product of
two binary folds.

Potential benefits:

```text
1. bad local rank-one line configurations become rank-at-most-3 interpolation configurations;
2. a zero request sees a random 4-point MDS local transform instead of a linked two-point line;
3. all-mixed PA stress should no longer be expressible as a single slope-line contraction;
4. common-zero/all-paired compression still recurses cleanly, but with arity 4.
```

The likely new proof object is not the old root-line geometry. It is a bounded-degree local
Reed-Solomon block over four child copies, with bad events controlled by ranks of degree-3
interpolation restrictions.

That could be cleaner for a first-moment proof, but it is a real new code family. We should not
call it a proof of the existing binary RFC distance.

## Systematic Coordinates

Arity 4 can support systematic coordinates if the systematic block stores the four child message
values as the four interpolation values at `beta_0,...,beta_3`. Folding then maps the raw
systematic block to:

```text
sum_s lambda_s(alpha) m_s,
```

which is the natural arity-4 multilinear fold over two variables.

This is cleaner than top-only systematic followed by ordinary non-systematic RFC, because the lower
code is still in the same folded message basis. But it means the whole recursive code must agree on
the arity-4 fold semantics. It is not a top-layer-only patch.

## Local Seal Alternative

A less invasive compromise is a non-recursive local seal layer.

Idea:

```text
binary RFC core -> apply a random local 4-to-4 MDS/mixing layer at the top or at selected levels
```

This could improve the visible local geometry used in the distance proof while leaving the
recursive fold core binary. The price is that the seal must be opened/checked as an extra local
constraint, and a seal-only layer does not change the deepest recursive all-paired obstruction.

This is the best candidate if we want a small protocol experiment without redesigning BaseFold.
It is not the same as an arity-4 foldable core.

## Recommendation

For the existing BaseFold/Blaze backend, larger blocks are not drop-in. The current code and spec
are pair-native.

For a proof-friendly RFC variant, true arity-4 interpolation is the version worth exploring. It is
mathematically fold-compatible and may replace root-line obstructions by higher-rank local MDS
geometry. The tensorized 4-to-4 version is protocol-friendly but probably proof-neutral.

Concrete next steps:

```text
1. Define an arity-4 non-systematic RFC ensemble using four distinct output points per coordinate.
2. Prove the fold identity by Lagrange interpolation.
3. Write a small sampler comparing binary RFC, tensorized 4-to-4, and true 4-point arity-4 blocks.
4. Track first-moment zero-set exponents for the top stress profiles, especially all-mixed PA.
5. Only after that, decide whether the BaseFold/Blaze protocol cost is worth the proof simplification.
```
