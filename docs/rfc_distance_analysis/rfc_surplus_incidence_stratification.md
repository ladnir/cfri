# RFC Surplus Repair Incidence Stratification

Status: correction to the surplus codimension target.

The previous target asked for a Hall-OK bound:

```text
Pr[root-line repair rank < 2D] <= poly(D,t) q^{-(t-2D+1)}.
```

The incidence calculation shows this is too optimistic under Hall alone. Hall proves generic full
rank, but surplus codimension also depends on how many singleton functionals lie in low-rank flats.

## Bad-Root Witness

Root-line repair fails iff there exists a nonzero witness:

```text
(x,y) in K_P plus K_P
```

such that for every singleton row `i`:

```text
ell_i(x) + alpha_i ell_i(y) = 0.
```

For fixed `(x,y)`, write:

```text
a_i = ell_i(x)
b_i = ell_i(y).
```

Then:

```text
b_i != 0:  alpha_i is fixed to -a_i/b_i, if allowed by the side domain;
b_i = 0, a_i != 0: no alpha_i works;
a_i = b_i = 0: alpha_i is free.
```

Thus the only way to keep many root variables free is for many rows to vanish on both `x` and `y`.

## Stratification By Common-Zero Set

For a subset `A` of singleton rows, suppose:

```text
ell_i(x)=ell_i(y)=0 for all i in A.
```

Let:

```text
r(A)=rank{ell_i : i in A}.
```

The space of pairs `(x,y)` killed by all rows in `A` has affine dimension:

```text
2(D-r(A)).
```

Projectively, this contributes at most:

```text
q^{2(D-r(A))-1}
```

witness classes when `r(A)<D`; if `r(A)=D`, only `(x,y)=(0,0)` is killed, so there is no nonzero
witness stratum.

Rows in `A` leave their alpha variables free. Rows outside `A` have at most one allowed alpha value
for each fixed witness. Therefore this stratum contributes at most:

```text
q^{2(D-r(A))-1} (q-1)^{|A|}
```

bad root assignments, out of roughly:

```text
(q-1)^t.
```

So the codimension supplied by this stratum is:

```text
t - |A| - 2D + 2r(A) + 1.
```

## Corrected Local Exponent

The incidence union bound gives the local exponent:

```text
codim >= min_{A: r(A)<D} ( t - |A| - 2D + 2r(A) + 1 )
```

up to polynomial factors.

Equivalently:

```text
codim >= t - 2D + 1 - max_{A: r(A)<D} ( |A| - 2r(A) ).
```

Define the flat excess:

```text
flat_excess = max_{A: r(A)<D} ( |A| - 2r(A) ).
```

Then:

```text
Pr[repair failure] <= poly(D,t) q^{-(t-2D+1-flat_excess)}.
```

This explains the role of Hall:

```text
Hall-OK implies t-|A|+2r(A) >= 2D for all A,
so the corrected exponent is at least 1.
```

But Hall alone does not force the full surplus exponent `t-2D+1`; it only prevents the exponent
from dropping below one.

## When The Original Surplus Target Holds

The clean target:

```text
t - 2D + 1
```

holds if:

```text
|A| <= 2r(A)
```

for every proper-rank flat `A`.

For `D=3`, this says:

```text
no projective point has multiplicity > 2,
no projective line contains more than 4 singleton rows.
```

The current D=3 probes used distinct projective points, so rank-one flat excess is absent. The main
possible exponent loss is from lines carrying five or more sampled singleton rows.

## Meaning For The Near-MDS Route

This is a notable correction:

```text
Near-MDS does not merely require Hall-OK repair.
It requires enough surplus plus a bound on low-rank flat overloading in K_P|_T.
```

The top-profile tolerance calculation assumes `flat_excess=0`. That is plausible for generic
large-dimensional child quotient data, but it is not guaranteed by the Hall condition.

The global proof must therefore count or charge overfull flats. There are two possible routes:

```text
1. Prove child quotient restrictions K_P|_T are flat-sparse for the survivor profiles that dominate.
2. Add flat_excess as a state variable and show profiles with positive flat_excess are few or pay
   elsewhere in the recurrence.
```

This is the current local blocker.

The top-profile tolerance to bounded flat excess is recorded in:

```text
rfc_flat_excess_tolerance.md
```

The main numerical message is that each unit of flat excess costs about one unit of final excess
`e` in the `c=8,k=2048,q=2^128` top-profile model.
