# RFC Systematic Distance Proof Status

This is the current proof snapshot for the all-level systematic RFC distance certificate.

## Target

Parameters:

```text
c = 8
c_p = 7
d = 11
k = 2048
N = 16384
q = 2^128
```

The collapse family gives the upper bound:

```text
d_sys <= 12384
delta_sys <= 12384 / 16384 = 0.755859375.
```

So the certificate target is:

```text
d_sys >= 12384
```

up to the usual random-challenge failure probability.

## Current Certificate Shape

The proof target is no longer literal actual-core containment. The working theorem is a
defect-coupled virtual-core statement:

```text
wt(x) = m,
wt(Ax) = k/m + e

=> supp(Ax) = (C \ H) union E
   where C is a matched stride core of size k/m,
         H subset C,
         E subset [k]\C,
         |E| = e + |H|,
         |H| <= 2e.
```

This is the right replacement because unbalanced row splits can create holes in the selected
parent residue, even when they are still accompanied by proportional outside support.

## Count

The depth-11 systematic count with virtual holes coupled to true output defect gives:

```text
alpha=2: Pr[d_sys < 12384] <= 2^-99.60689084.
```

The same audited count remains negative through:

```text
alpha=5: log2 failure bound = -21.53925283
```

and breaks at:

```text
alpha=6: log2 failure bound = 25.89359792.
```

So the proof target is `alpha=2`, with slack up to but not including `alpha=6`.

The count includes:

```text
k * sum_{h <= alpha e} binom(k/m, h) binom(k-k/m, e+h) * 2^(B e)
```

with `B=64`, and the near-kernel dimension factor:

```text
q^floor((e+h)/(k/m)).
```

The key artifacts are:

```text
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le2extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le5extra.csv
docs/rfc_near_extremizer_total_union_depth11_c8_e128_stride_dim_overhead64_holes_le6extra.csv
```

## What Is Reduced

The one-child case is reduced:

```text
child virtual core lifts to both parent siblings;
holes and defect both double.
```

The balanced two-child case is reduced:

```text
after overlap and paid cancellation charges, no-early-gluing leaves an actual parent core.
```

The unbalanced two-child case is reduced to a local alpha-2 theorem:

```text
new parent virtual holes <= 2 * new true output-defect leaves.
```

The deterministic size part of this theorem is written. At an unbalanced node:

```text
M = a+b, a != b,
L = K/M,
R = L/2,
C = child projection of selected parent residue,
A = |C \ (U union V)|,
O = |(U union V) \ C|.
```

One-copy uncertainty gives:

```text
O >= R + A.
```

After all paid child dimensions and paid cancellations are removed, the residual skeleton should
have:

```text
c_C + c_O <= 1.
```

Then:

```text
h_local = 2A + c_C,
e_local >= 2O - c_O - h_local
        >= 2R - (c_C+c_O)
        >= 2R - 1,
```

and for `R>=2`:

```text
h_local <= 2 e_local.
```

The integer small cases are covered by:

```text
scripts/rfc_check_alpha2_local_inequality.py --max-r 64
```

which reports:

```text
checked_R=1..64 failures=0.
```

## Global Composition

The global bookkeeping is no longer the main gap. Fix the final virtual core `C_root`; every
outside leaf has a unique highest first-divergence node.

For each unbalanced node `v`, split the outside leaves first diverging at `v` into:

```text
R_v = replacement leaves, one per new virtual hole;
D_v = true defect leaves.
```

The local theorem gives:

```text
|R_v| = h_v,
h_v <= 2 |D_v|.
```

The first-divergence sets are disjoint across nodes, so:

```text
|H_root| = sum_v h_v
        <= 2 sum_v |D_v|
        <= 2e.
```

Balanced-node charges create no holes and only add true defect leaves. Complete extra stride-class
dimensions are rank payments handled by the factor `q^floor((e+h)/|C|)`, not additional hole
payments.

## Remaining Algebraic Lemma

The remaining serious proof obligation is the rank-cancellation exchange:

```text
after paying child rank dimensions, at most one sibling cancellation remains uncharged.
```

Local formulation:

```text
r_0 = 1 + s_0,
r_1 = 1 + s_1,
s = s_0+s_1.
```

If `t` sibling cancellations occur at the parent node, then generically:

```text
t cancellations impose t equations,
one equation is absorbed by the relative scalar,
each additional absorbed equation consumes one paid child-rank parameter.
```

So the intended exchange is:

```text
pay min(s,t-1) cancellations by rank.
```

If `t>s+1`, the remaining `t-s-1` cancellations must be paid by the usual no-early-gluing
probability loss. In first-moment form this is:

```text
q^s * q^(-max(0,t-s-1)).
```

After rank payments and probability-loss payments are removed from the structural case under
consideration, at most one cancellation remains visible to the alpha-2 local size lemma.

Equivalently, for every fixed child support/rank pattern and cancellation set `J`, the actual RFC
cancellation matrix should have generic rank at least:

```text
|J|-1
```

over the paid child-rank quotient. This must be proved for the actual fold equations with `T`
uniform nonzero.

This is tracked in:

```text
docs/rfc_rank_cancellation_exchange.md
```

## Current Honesty Level

We have a plausible full proof architecture and the counting already supports the desired distance
bound with real slack. We do not yet have a complete proof, because the rank-cancellation exchange
is still an algebraic lemma, not a finished theorem.

If the exchange lemma is proved, the proof chain closes as:

```text
rank-cancellation exchange
  => residual c_C+c_O<=1
  => local alpha-2 hole/defect coupling
  => first-divergence global |H|<=2e
  => defect-coupled virtual-core count
  => Pr[d_sys < 12384] <= 2^-99.60689084.
```

The most important audit questions are:

```text
1. Is the local inequality using O >= R+A correctly in every unbalanced residue?
2. Does first-divergence really prevent double-spending between replacement leaves and true defect
   leaves?
3. Is the rank-cancellation exchange stated with the correct quotient and dimension payment?
4. Does the first-moment count pay for every rank parameter and every charged-tree label exactly
   once, without undercounting virtual holes?
```
