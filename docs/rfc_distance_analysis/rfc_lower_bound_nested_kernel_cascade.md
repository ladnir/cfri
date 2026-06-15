# RFC Lower-Bound: Nested-Kernel Cascade Attack

Scope: original non-systematic RFC only.

This is the third lower-bound/falsification iteration after both lanes agreed that the seven-copy
matched-core row is not the live obstruction once exact support sets and child flags are counted.

## Current Target

Production target under discussion:

```text
c = 8
k = 2048
q = 2^128
e = 71
z = k + e = 2119
```

The lower-bound question here is:

```text
Can a nested kernel K <= W ride the paired-compression spine and retain enough entropy under exact
support / flag counting to push the e=71 first moment above -80 bits?
```

I do not currently have such a corrected row.

## What Changed

The flag checkpoint:

```text
scripts/rfc_distance_analysis/rfc_flag_span_moment.py
docs/rfc_distance_analysis/rfc_flag_recurrence_checkpoint.md
```

records the state:

```text
pi(W) vanishes on P union (S \ A)
pi(K) vanishes on P union S
K = ker(ev_S | W)
```

The depth-4 toy crossing improves from scalar endpoint-tau2 excess `105` to flag excess `49`.
From the lower-bound side, the important point is that hidden kernel directions no longer float
through singleton-heavy layers for free.  A falsification must now build real nested flags, not just
large scalar spans.

## Paired-Compression Spine

The expected production spine is approximately:

```text
depth  k     z       excess
11     2048  2119    71
10     1024  1060    36
9      512   530     18
8      256   265     9
7      128   133     5
6      64    67      3
5      32    34      2
```

A nested-kernel cascade would have to add singleton structure along or near this spine.  At a
singleton layer, the visible quotient pays on `A`, while the kernel line/subspace receives the
stricter child request `P union S`.  So every time the cascade keeps a kernel alive, it also asks
that kernel to vanish on the entire singleton block.

## Structural Cost Of Kernel Growth

Existing near-kernel diagnostics point to a strong obstruction.  For a matched live row block and
an output core `C`, extra output leaves `E` increase the admissible kernel dimension only when they
complete full stride classes:

```text
dim <= 1 + floor(|E| / |C|).
```

Small-depth checks show:

```text
depth 4, m=4, extra outputs 2: all kernel_dim = 1
depth 4, m=4, extra outputs 3: all kernel_dim = 1
depth 4, m=4, extra outputs 4: 7872 rows kernel_dim = 1, 48 rows kernel_dim = 2
```

The dimension-2 rows are exactly the complete-extra-stride cases.  This is bad news for the
falsification attempt: isolated singleton extras have entropy, but they do not create nested kernel
dimension.  Complete stride-class extras create dimension, but they are rigid and expensive under
exact support/flag counting.

## Corrected b > 1 Support Scan

I added core-support filters to:

```text
scripts/rfc_distance_analysis/rfc_multicopy_falsification.py
```

and ran the exact-support b>1 check:

```text
python scripts/rfc_distance_analysis/rfc_multicopy_falsification.py \
  --depth 11 \
  --expansion 8 \
  --q-log2 128 \
  --target-excesses 71 \
  --support-count-model exact-size \
  --min-core-support 2 \
  --by-active
```

Best row:

```text
active copies r = 1
core support b = 2
extra support h = 0
one-copy support = 2
modeled log2 expected = -8663.29147157
```

All higher-active-copy b>1 rows are far more negative in this model.  So I do not see a corrected
multicopy b>1 row near `-80`, let alone above it.

This does not prove safety.  It only says that the next lower-bound obstruction is not visible in a
scalar exact-support sweep over `(r,b,h)`.

## Current Lower-Bound Status

The best exact-support scalar stress remains the broad b=1 one-copy row:

```text
e=70:    3.41617672
e=71: -121.83277193
e=72: -247.08241391
```

Thus, under exact support counting, I cannot currently produce an `e=71` modeled row above `-80`.
The remaining gap is not a visible support-count issue; it is a possible flag-intersection issue.

## Needed Exact Enumerator

The next falsification route needs an exact depth-4 flag-intersection enumerator.

For independent original RFC copies over a moderately large prime, enumerate child flags:

```text
L <= V
```

where:

```text
V = pi(W), zero on P union (S \ A)
L = pi(K), zero on P union S
```

Group by:

```text
dim V
dim L
outer zero count
inner zero count
visible support size |A|
support rank / stride class
number of complete extra stride classes
copy-pair or copy-triple intersection dimension
```

The falsifying signal would be:

```text
# exact flags * q^(observed common flag/intersection dimension)
```

exceeding the generic flag-intersection exponent by a production-relevant amount.

In particular, we should look for flags where:

```text
dim(L) stays positive through several paired levels,
dim(V/L) is only 1 or 2 at singleton layers,
and L is supported by complete stride classes that recur across independent copies.
```

This is the actual nested-kernel cascade.  A scalar support sweep cannot see whether those flags
are many distinct linear objects or duplicate certificates of the same child flag.

## Feedback For Upper-Bound Agent

The sharp remaining obstruction is:

```text
complete-stride nested flags may have more multiplicity than unmarked support sets reveal.
```

The upper-bound proof should explicitly prove one of the following:

```text
1. Exact child flags are counted with only generic Gaussian flag multiplicity once the supports
   and zero budgets are fixed.

2. Any excess multiplicity comes only from complete stride classes, and each such class is charged
   by the stricter pi(K) zero budget or by a paired-compression recursive term.

3. Multi-copy intersections of exact flags obey the generic projective/flag intersection exponent,
   except for paired-compression structure that preserves the same relative gap.
```

The depth-4 flag checkpoint is encouraging because it kills the scalar hidden-kernel trace.  But it
uses a coarse child-flag upper bound, not an exact flag enumerator.  From the falsification side, I
would not declare `e=71` robust until the exact depth-4 flag-intersection enumerator fails to find
a nested-kernel family above the generic flag exponent.

## Questions For Integrator

1. Should I build the exact depth-4 flag-intersection enumerator next, or should the proof lane
   first specify the exact flag type it wants counted?

2. Is `e=71` required to have full 80-bit slack after all constants, or is an `e=72` certificate
   acceptable if exact flag enumeration leaves less than about 20 bits of margin?

3. For the next exact enumerator, should we use large-prime sampled RFC instances or a symbolic
   support/stride-class enumerator first?
