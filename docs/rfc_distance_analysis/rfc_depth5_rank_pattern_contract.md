# RFC Depth-5 Rank-Pattern Contract

Scope: original non-systematic RFC, finite base seal for the `c=8,k=2048,e=71` distance proof.

Status: deterministic calibration and a now-audited non-theorem scalar contract.

Important update: `rfc_depth5_rank_pattern_audit.md` shows that the scalar
`q^{-r(s-c)}` singleton charge is not proof-safe for multi-coordinate low-visible-rank child
images. This note remains useful as the optimistic calibration and target trace, but the theorem
route must refine it to an exact-support/flag recurrence.

## Target

Seal the production floor state:

```text
depth = 5,
expansion = 8,
k = 32,
n = 256,
z = 34.
```

The desired base theorem is:

```text
B_5(1,34) <= 2^-80
```

where `B_d(r,z)` is the aggregate first moment over zero sets of size `z` of ordered `r`-tuples
vanishing on the set.

This would let the global recurrence stop at `child_k=32`, because the production zero floor from
`k=2048,e=71` is:

```text
ceil((2048+71)/64) = 34.
```

## Recurrence To Prove

For one fold, with parent replica count `r`, split a parent zero request of size `z` into:

```text
z = 2p + s,
```

where `p` child coordinates are paired and `s` are singleton-oriented. Let `c` be the number of
singleton child coordinates that are already common-zero for the child `2r`-tuple, and put:

```text
u = p + c.
```

The rank-pattern recurrence used by the calibration is:

```text
B_h(r,z)
  <= sum_{p,s,c}
       2^s
       binom(u,p)
       binom(n_child-u, s-c)
       q^{-r(s-c)}
       B_{h-1}(2r,u).
```

Interpretation:

```text
paired coordinate:
  child 2r-tuple is common-zero at that coordinate;

common-zero singleton:
  also contributes to u and costs no root equation;

non-common singleton:
  contributes a rank-one root-compatibility pattern and costs r aggregate equations.
```

The local proof obligation is exactly the finite-replica rank-pattern lemma:

```text
For the child RFC code and any selected singleton set E disjoint from the common-zero set,
the aggregate number of child 2r-tuples satisfying the root-compatible rank-one pattern on E
is at most q^{-r|E|} times the corresponding unrestricted child moment, up to the explicit
finite constants reserved for the base seal.
```

For `r=1`, this is already the one-root-per-singleton bound. The nontrivial finite base theorem
requires this aggregate rank-pattern bound for:

```text
r = 2,4,8,16,32
```

at depths `4,3,2,1,0`.

## Calibration

Command:

```text
python scripts/rfc_distance_analysis/rfc_replica_zero_moment.py \
  --depth 5 \
  --expansion 8 \
  --q-log2 128 \
  --security-bits 80 \
  --singleton-charge replica \
  --print-window 4 \
  --trace-z 34 \
  --explain-z 34 \
  --top-terms 25
```

Output:

```text
crossing_z = 34
crossing_excess = 2
log2 B_5(1,34) calibration = -115.10435419
slack to 2^-80 = 35.10435419 bits
```

Dominant trace:

```text
level 5: z=34, p=2, s=30, c=0, u=2
level 4: z=2,  p=0, s=2,  c=0, u=0
level 3: z=0
level 2: z=0
level 1: z=0
```

Top-level explanation for `z=34`:

```text
rank term_log2       gap_bits   p   s   c   u
1    -116.82450305   0.000000   2   30  0   2
2    -117.19373686   0.369234   1   32  0   1
3    -117.24450680   0.420004   3   28  0   3
4    -118.29697422   1.472471   4   26  0   4
5    -118.75573821   1.931235   0   34  0   0
6    -119.90396303   3.079460   5   24  0   5
7    -122.02425727   5.199754   6   22  0   6
8    -124.63807463   7.813572   7   20  0   7
9    -127.74064436   10.916141  8   18  0   8
10   -131.33968205   14.515179  9   16  0   9
```

The first term with `c=1` is more than `123` bits below the best term. Therefore the relevant
top-level mass is the `c=0` rank-pattern branch, not common-zero singleton leakage.

Depth-4 child values feeding the top-level recurrence:

```text
u   log2 B_4(2,u)
0   4096.00000000
1   3847.00000000
2   3596.98868469
3   3346.38100211
4   3095.34678639
5   2843.97905461
6   2592.33660661
7   2340.45998903
8   2088.37885227
9   1836.11581786
10  1583.68870753
11  1331.11191896
12  1078.39732118
13  825.55486246
14  572.59299759
15  319.51899700
16  66.33917597
17  -186.94093195
```

## Why This Helps The High-Defect Gap

The theta-chain hard-segment diagnostic sees a local gap for:

```text
child_k=32,
zeros=(34,39,44),
b=14.
```

The base seal does not close that local row. Instead, it says the global distance proof does not
need to continue the theta-chain recurrence once it reaches `child_k=32` and at least `34` zeros:

```text
F_5(1,34) is already below 2^-80.
```

Thus the remaining theorem can be split:

```text
depth above 5:
  use the flag/theta-chain recurrence, with the production zero floor guaranteeing z_child >= 34;

depth 5:
  use the finite rank-pattern base theorem.
```

## Current Risk

The recurrence above is still a theorem target. The known local aggregate identity:

```text
sum_{X,Y in F^r} Pr_T[X + T(Y-X)=0] = q^r
```

proves the one-coordinate cost, but the multi-coordinate version must survive dependencies in the
child RFC code. This is the exact rank-pattern induction problem in
`rfc_rank_pattern_induction_target.md`.

The base-seal route is attractive because it only requires that induction through depth five and
only needs about `35` bits of final slack.
