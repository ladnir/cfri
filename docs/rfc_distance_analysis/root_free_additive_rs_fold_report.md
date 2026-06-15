# Root-Free Additive RS Fold Report

Status: exploratory report. This is not a proof or implementation plan.

## Question

Can we get the RS distance story without roots of unity, while keeping FRI/BaseFold-style binary
folding?

Short answer:

```text
Yes in principle. Use additive/subspace Reed-Solomon folding, not random RFC tensor folding.
```

This is the root-free analogue of FRI. Over characteristic two it is naturally expressed through
Artin-Schreier/subspace polynomials.

## Core Algebra

Let `U` be a one-dimensional additive subspace:

```text
U = {0, beta}.
```

The quotient map that identifies each pair `{x, x+beta}` is the subspace polynomial:

```text
L_beta(X) = X^2 + beta X.
```

Indeed:

```text
L_beta(x + beta) = (x+beta)^2 + beta(x+beta)
                 = x^2 + beta x
                 = L_beta(x).
```

So the pair fibers are:

```text
{x, x+beta}.
```

No roots of unity are involved.

For `beta=1`, this is:

```text
L(X) = X^2 + X,
fiber {x, x+1}.
```

This is the binary-field analogue of the usual FRI squaring map `x -> x^2` with fibers `{x,-x}`.

## Degree-Preserving Fold

Every polynomial can be decomposed as:

```text
f(X) = f_0(L_beta(X)) + X f_1(L_beta(X)).
```

This is the key RS-preservation identity. If:

```text
deg f < k,
```

then approximately:

```text
deg f_0, deg f_1 < ceil(k/2).
```

Given evaluations of `f` on a pair `{x, x+beta}`, the verifier/prover can interpolate the local
line in `X`. Folding at challenge `rho` produces:

```text
g(L_beta(x)) = f_0(L_beta(x)) + rho f_1(L_beta(x)).
```

Then:

```text
deg g < ceil(k/2).
```

So the folded word is again an RS word on the quotient additive domain.

This is exactly the property RFC was trying to mimic with random local two-point folds, but here it
comes from a global univariate RS structure.

## Distance

If the code is:

```text
f with deg f < k,
evaluated on N distinct field points,
```

then it is Reed-Solomon and therefore MDS:

```text
distance = N - k + 1.
```

This is independent of the folding proof. The fold proof only needs to preserve the RS family
recursively.

The requirement is simply:

```text
N <= |F|.
```

For `B128`, this is not a problem for the target sizes:

```text
N = c*k = 8*2048 = 16384 << 2^128.
```

## Relation To Current RFC

Current RFC and additive RS both have pair shapes like:

```text
x, x+1
```

or more generally:

```text
x, x+beta.
```

But they differ in the global basis.

Current RFC:

```text
multilinear/tensor fold basis
local random or structured two-point transforms
distance proof sees tensor-line and mixed paired/singleton obstructions
```

Additive RS:

```text
univariate degree basis over additive/subspace domain
global RS/Vandermonde distance
folding via subspace polynomial quotient
```

So "choose nicer T's" is not enough if the code remains the same tensor/RFC ensemble. To inherit RS
distance cleanly, the code should actually be an additive-domain RS code or a code equivalent to
one.

## Existing Repo Evidence

The repository already has additive-domain machinery:

```text
crates/cfri/src/backend/code/binary_rs.rs
```

Relevant pieces:

```text
BinarySubspace<F>
OnTheFlyTwiddleAccess
PrecomputedTwiddleAccess
subspace_map(elem, constant) = elem^2 + constant*elem
```

The comments describe normalized subspace polynomials:

```text
W_i(X) vanishes on U_i,
W_{i+1}(X) = W_i(X)(W_i(X)+W_i(beta_i)).
```

This is precisely the additive/subspace polynomial tower.

The BaseFold backend also has:

```text
get_table_additive_binary(...)
query_point_binary_rs(...)
test_binary_rs_transform
test_binary_rs_table_weights
```

The test `test_binary_rs_transform` checks, for `B128`, `num_vars <= 5`, and `log_rate <= 2`, that
folding an encoded word over the additive binary table agrees with encoding the folded coefficients.

Important caveat:

```text
This does not by itself prove that the current BaseFold binary_rs path is a full global MDS RS
backend.
```

The current `evaluate_over_foldable_domain` path still has BaseFold/repetition-base structure in
places, and `encode_rs_basecode` is a separate base-code option. An encoding audit is needed to
determine whether the existing code path is:

```text
1. true additive RS over the whole domain,
2. RS basecode plus RFC-style recursive expansion,
3. or a fold-consistent additive table for the current tensor code.
```

The algebraic building blocks are present, but the global MDS claim depends on the exact encoded
code family.

## Why This Is Interesting

This gives a clean fork in the design space:

```text
Current RFC:
  field-agnostic-ish, pair-native, fast, but distance proof is hard and probably below RS proof
  cleanliness.

Additive RS/FRI:
  root-free, binary-field compatible, MDS by construction, and still pair-foldable.
```

This directly answers the "RS roots without roots of unity" concern:

```text
Use additive subspace roots instead of multiplicative roots of unity.
```

They are not roots of unity. They are elements of an additive subspace, with quotient maps given by
linearized/subspace polynomials.

## Practical Tradeoffs

Potential advantages:

```text
1. MDS distance is immediate from RS.
2. No roots of unity required.
3. Characteristic-two fields like B128 are natural.
4. Pair-shaped FRI/BaseFold queries remain natural.
5. The repo already has additive subspace/twiddle machinery.
```

Potential costs:

```text
1. The backend becomes RS/FRI-like rather than RFC-like.
2. Encoding must use additive FFT/subspace-polynomial machinery, not the current random RFC
   tensor-code proof.
3. The verifier/prover code needs an audit to ensure the committed code is exactly the MDS
   additive-RS code.
4. If Blaze depends on multilinear/tensor structure at the boundary, we need a clean map from
   multilinear evaluations to additive-RS coefficient/evaluation form.
```

## Concrete Next Step

Audit the existing `binary_rs` path as an encoding statement.

The audit should answer:

```text
For message dimension k and rate c, is the committed length-N word exactly evaluations of a
degree-<k univariate polynomial on N distinct additive-subspace points?
```

If yes:

```text
we already have an MDS-distance backend candidate, modulo protocol/security integration.
```

If no:

```text
the existing additive tables are still useful, but we need a new full additive-RS encoder path.
```

The minimal test is symbolic/small-field:

```text
1. instantiate a tiny binary field;
2. build the claimed additive RS domain;
3. encode basis messages through the backend path;
4. compare the resulting generator matrix to a Vandermonde/additive-RS generator matrix up to
   invertible row transformations and column ordering;
5. check all k-column minors for small k.
```

This is a much cleaner path than continuing to patch the RFC distance proof, because it attacks the
construction rather than the proof obstruction.
