# RFC Tensor Triple-Rank Codimension

Status: diagnostic/theorem target for the one-spill common-zero blocker.

## Purpose

The dominant one-spill row contains a lower paired event:

```text
depth = 4
three compressed RFC tensor columns
rank drops from 3 to 2.
```

The first one-spill ledger charged this event by the random-matrix exact-rank exponent:

```text
D0(p0-k1+D0) = 14.
```

This note records why that is probably too optimistic for RFC tensor columns and what must be
proved instead.

## Tensor Geometry

An RFC column at depth `d` is a rank-one tensor:

```text
u_{b_d}(T_d) tensor ... tensor u_{b_1}(T_1),
```

with local two-vectors:

```text
u_0(T) = (1-T, T)
u_1(T) = (-T, T+1).
```

Thus three columns live in the Segre variety:

```text
(P^1)^d -> P^{2^d-1}.
```

A random `16 x 3` matrix rank drop is not the right model. The columns are constrained rank-one
tensors whose dependencies are governed by low-dimensional tensor incidence conditions.

## Sampling Evidence

The sampler:

```text
scripts/rfc_distance_analysis/rfc_tensor_triple_rank_sampler.py
```

samples only the requested tensor columns. A compact run:

```text
python scripts/rfc_distance_analysis/rfc_tensor_triple_rank_sampler.py \
  --q 5 --q 7 --q 11 --q 31 --q 101 \
  --columns 0,11,25 --columns 0,32,74 --samples 10000
```

gave:

```text
triple 0,11,25:
  q=5:  97 / 10000 = 0.0097
  q=7:  22 / 10000 = 0.0022
  q=11:  5 / 10000 = 0.0005
  q=31:  0 / 10000
  q=101: 0 / 10000

triple 0,32,74:
  q=5:  53 / 10000 = 0.0053
  q=7:  10 / 10000 = 0.0010
  q=11:  3 / 10000 = 0.0003
  q=31:  0 / 10000
  q=101: 0 / 10000.
```

Multiplying the nonzero rates by `q^4` gives values on the order of a small constant:

```text
q=5:  about 3.3 to 6.1
q=7:  about 2.4 to 5.3
q=11: about 4.4 to 7.3.
```

This is much more consistent with a tensor-incidence codimension around `4` than with the random
matrix exponent `14`. The zero hits at `q=31,101` are also consistent with a codimension-four event:
at `q=31`, `5/q^4` is only about `5e-6`, so 10000 samples have a small expected hit count.

## Segre-Line Structural Classifier

A second diagnostic was added:

```text
scripts/rfc_distance_analysis/rfc_tensor_triple_segre_classifier.py
```

It uses the standard Segre-line fact:

```text
three distinct rank-one tensors are linearly dependent
  -> they agree projectively in all but one tensor factor.
```

For RFC local factors, projective equality has the simple form:

```text
u_b(T) projectively equals u_c(S)
  iff T+b = S+c.
```

Thus each non-free tensor level contributes the number of independent local `theta` equalities, or
is impossible if the same challenge variable would need two different offsets.

For all depth-4, expansion-8 triples:

```text
python scripts/rfc_distance_analysis/rfc_tensor_triple_segre_classifier.py \
  --depth 4 --expansion 8 --profile-all --top 20
```

gives:

```text
codim 3:   7168 triples
codim 4:  14336 triples
codim 5:  28672 triples
codim 6: 286720 triples
impossible: 4480 triples.
```

Representative triples:

```text
0,1,64  -> codim 3
0,1,32  -> codim 4
0,1,16  -> codim 5
0,1,2   -> codim 6
0,16,32 -> impossible.
```

Sampling these representatives matches the hierarchy qualitatively: the codim-3 example still
has hits at `q=31`, while codim-4/5 examples had no hits in the same compact run.

## One-Spill Impact

The one-spill ledger now supports:

```text
--triple-rank-codim-override 4
```

to stress the dominant row with this tensor-specific codimension. The deep-spill target run changes
from:

```text
random lower_q = 14:
  log2_sum = 2382.701824
  extra q-dimensions needed = 19.239858
```

to:

```text
tensor lower_q = 4:
  log2_sum = 3662.701824
  extra q-dimensions needed = 29.239858.
```

The dominant row is unchanged:

```text
h=864,F=41,z=224,lift_levels=5
h0=27,z0=7,p0=3,s0=1,D0=14.
```

The ledger also supports the structural profile:

```text
--triple-rank-structural-profile
```

which replaces arbitrary triple entropy by the count of triples in the minimum Segre-line
codimension stratum. For depth 4 this uses `7168` codim-3 triples. The same deep-spill target gives:

```text
log2_sum = 3785.128176
dominant row unchanged
lower_q = 3
lower_log2_count = log2(7168) = 12.807355
total_q = 34
extra q-dimensions needed = 30.196314.
```

## Proof Target

We need a theorem for three RFC tensor columns:

```text
Pr_T[rank(c1,c2,c3) <= 2] <= poly(d) q^{-gamma_3}
```

with an explicit `gamma_3` and a codimension-stratified triple count. Current evidence suggests:

```text
gamma_3 can be as low as 3 for some triples.
```

Using the structural triple count, the common-zero proof must recover about `30.2` q-dimensions
for the one-spill row from elsewhere:

```text
mixed root-line incidence,
full-common-zero maximality,
or top-profile compatibility.
```

If a stronger RFC-specific theorem proves a larger `gamma_3` for the exact triples appearing in
the canonical bucket, the required external saving decreases accordingly.

## Next Step

Turn the sampling evidence into a structural classification:

```text
three tensor columns dependent
  -> a finite list of local projective incidence patterns
  -> each pattern has an explicit number of independent root equations.
```

That classification would replace the random-matrix lower exponent in the one-spill recurrence and
make the mixed-branch certificate honest.
