# RFC Multi-PA Codimension Budget

Status: pressure check for how strong the multi-PA incidence theorem must be.

## Question

The multi-PA graph-contraction event is:

```text
rank{y_j} modulo U + span{x_j + alpha_j y_j} <= r.
```

A one-minor Schwartz-Zippel argument gives at most one q-factor once generic rank is larger than
`r`. That is safe but probably too weak. The question is how much codimension is actually needed
for the dominant flat-excess profiles.

## All-PA Stress Model

Use the dominant top profile:

```text
child k      = 1024
child n      = 8192
paired core  = 137
q            = 2^128
```

For a flat witness:

```text
a = 2r + F
```

consider the extreme all-mixed case where all `a` marked `A` coordinates are paired with `P`
siblings at the same child positions. A conservative profile count is:

```text
choose a mixed child positions
choose their sides
choose the remaining paired-core P positions elsewhere.
```

The helper:

```text
scripts/rfc_distance_analysis/rfc_multi_pa_codim_need.py
```

reports the q-dimension needed to beat this profile entropy at expected count at most `1/2`.

## Results

```text
(r,F)=(0,1):
  a                         = 1
  profile entropy           = 1007.791384 bits
  needed codim              = 7.881183 q-dimensions
  one-minor q^-1 log2 term  = +879.791384 bits
  random-like qdim          = 376

(r,F)=(1,1):
  a                         = 3
  profile entropy           = 1021.370700 bits
  needed codim              = 7.987271 q-dimensions
  one-minor q^-1 log2 term  = +893.370700 bits
  random-like qdim          = 754

(r,F)=(8,1):
  a                         = 17
  profile entropy           = 1087.528903 bits
  needed codim              = 8.504132 q-dimensions
  one-minor q^-1 log2 term  = +959.528903 bits
  random-like qdim          = 3456

(r,F)=(0,8):
  a                         = 8
  profile entropy           = 1048.878011 bits
  needed codim              = 8.202172 q-dimensions
  one-minor q^-1 log2 term  = +920.878011 bits
  random-like qdim          = 3064
```

## Meaning

A bare one-minor finite-root bound is not enough for all-mixed PA flat witnesses. But the required
codimension is tiny compared to the natural projection/random-like scale. For the first stress,
one-coordinate PA already gives about:

```text
376 q-dimensions.
```

So the local theorem does not need to prove the perfect random-matrix exponent. It likely suffices
to prove a much weaker combined statement:

```text
all-mixed PA rank loss costs at least 9 q-dimensions,
unless it routes to a child marked/projection rank-tail event.
```

More generally, an `O(log_q binom(N,p))` codimension lower bound, not an `Omega(k)` one, is enough
for these small-flat regimes.

## Updated Target

The multi-PA incidence theorem can be intentionally modest:

1. if rank loss is caused by child projection degeneracy, charge the child marked incremental tail;
2. otherwise show the root equations contribute a finite number of independent conditions;
3. prove that after projectivizing witnesses, the net codimension is at least about `9` q-dimensions
   for the target `c=8,k=2048,q=2^128` stress.

This makes the current distribution look more viable than a perfect-codimension target would
suggest.
