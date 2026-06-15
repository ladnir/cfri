# One-Spill Minority-Label Local No-Go

Status: local no-go for the dominant one-spill row.

## Purpose

The label-distribution profiler showed that the dominant one-spill row would close if we could
charge about:

```text
3.25 q-dimensions per minority P/C label
```

inside the seven lifted blocks. This note checks whether that charge can come from local root-line
algebra inside the lifted zero set.

Conclusion:

```text
No. P/C labels inside Z=P union C are locally algebraically invisible once x and y vanish on Z.
```

Any real saving must therefore come from a global/canonical source:

```text
full-common-zero maximality outside Z,
top survivor-profile coupling,
or a different witness-counting normalization.
```

## Setup

In the common-zero replacement theorem, for a fixed top profile:

```text
P = paired child positions
T = singleton child positions
C = C(x,y) subset T
Z = P union C.
```

The child subcode-zero event is:

```text
x_j = y_j = 0 for all j in Z.
```

The dominant one-spill label pattern has:

```text
z = |Z| = 224
|P| = 137
|C| = 87
lift block size = 32.
```

The entropy term:

```text
log2 binom(224,87)
```

counts which positions of `Z` are singleton common-zero positions `C` rather than paired positions
`P`.

## Local Algebra Check

For `j in P`, both parent siblings are requested zero. Since:

```text
left_j  = x_j + alpha_j y_j
right_j = x_j + beta_j y_j
```

with an invertible local fold matrix, the paired condition is equivalent to:

```text
x_j = y_j = 0.
```

For `j in C`, the position is a singleton position in the parent profile, but by definition of the
full common-zero set:

```text
x_j = y_j = 0.
```

Therefore, after conditioning on `Z=P union C`, the local equations inside `Z` are identical for
`P` and `C` labels:

```text
j in P: x_j = y_j = 0
j in C: x_j = y_j = 0.
```

The sampled root value for the singleton coordinate also disappears:

```text
x_j + alpha_j y_j = 0
```

is automatic when `x_j=y_j=0`.

Thus, for fixed `Z`, the child kernel:

```text
H_Z = ker(ev_Z)
```

and the witness space:

```text
H_Z plus H_Z
```

do not depend on the `P/C` labeling of positions inside `Z`.

## Consequence

A theorem of the form:

```text
each minority P/C label inside a mixed lifted block costs q^{-c}
```

cannot be proved from local root-line equations inside `Z`. Those equations see only `Z`, not the
labeling.

The only label-dependent quantities in the current ledger are:

```text
1. the survivor-profile counting term;
2. the size a=|C|, hence the residual root exponent t-a-2h+1;
3. full-common-zero maximality outside C, if we enforce exact C(x,y)=C rather than C subset C(x,y).
```

For the dominant row, `a=87` is fixed. Rearranging the `87` C-labels across the seven blocks changes
the combinatorial count but not the current q-exponent.

## What This Rules Out

The label-distribution target:

```text
3.25 q-dimensions per minority label
```

is not a local spill-block root-incidence theorem.

The block-constant label test was already impossible because:

```text
|P| = 137
```

is not divisible by the lift size `32`. This note strengthens the lesson: even for nonconstant
labels, the P/C arrangement inside `Z` is algebraically invisible to the conditioned child-zero
event.

## Remaining Sources Of Saving

To close the dominant one-spill row, the missing `30.196314` q-dimensions must come from one of:

```text
1. exact full-common-zero maximality:
   require T\C positions not to be common-zero for the same witness;

2. top survivor-profile coupling:
   count (P,C,T\C) jointly with the lower one-spill structure rather than multiplying independent
   label and outside-singleton choices;

3. witness de-duplication:
   count canonical witness subspaces or full common-zero sets once, instead of counting many
   compatible labels for the same witness family;

4. a stronger lower tensor-rank theorem for the exact canonical triple stratum.
```

This refines the next target: stop looking for a local minority-label root charge inside `Z`; look
at exact maximality and global witness-counting instead.
