# RFC Surplus Repair Codimension Target

Status: new global-tolerance checkpoint for the fixed-survivor rank-tail route.

The local one-minor Schwartz-Zippel envelope is true but far too weak for near-MDS. The top-profile
tolerance check shows the proof needs a surplus-sensitive repair codimension theorem.

## Calculator

Script:

```text
scripts/rfc_distance_analysis/rfc_one_step_repair_tolerance.py
```

This is not a certificate. It only counts top-level survivor profiles with too few paired child
positions:

```text
p < k_child.
```

For a parent survivor size:

```text
s = k + e,
```

a top profile has:

```text
2p + t = s,
D_min = k_child - p.
```

The profile count is:

```text
binom(n_child,p) binom(n_child-p,t) 2^t.
```

The script compares two local repair exponent models:

```text
single_minor:  one q-factor from D/(q-1) Schwartz-Zippel
surplus_rank:  q^{-(t-2D+1)}
```

The second model is the codimension we would expect from a full-rank random row system in dimension
`2D`, and it matches the D=1 and D=2 root-line behavior:

```text
D=1: failure means all alpha values agree, exponent t-1.
D=2: PGL2/no-active envelope gives exponent t-3 in the generic distinct-class case.
```

## Target Config Result

For:

```text
depth=11
expansion=8
k=2048
n=16384
q=2^128
success target: expected bad top-profile count <= 1/2
```

the one-minor determinant envelope fails badly:

```text
python -B scripts/rfc_distance_analysis/rfc_one_step_repair_tolerance.py \
  --depth 11 --expansion 8 --q-log2 128 \
  --model single_minor --min-excess 69 --max-excess 71
```

Rows:

```text
e=69: log2_bad_top_deficit = 8972.612331
e=70: log2_bad_top_deficit = 8975.364028
e=71: log2_bad_top_deficit = 8978.114943
```

So a single `q^-1` repair factor is not remotely enough.

The surplus-rank model crosses at the old near-MDS scale:

```text
python -B scripts/rfc_distance_analysis/rfc_one_step_repair_tolerance.py \
  --depth 11 --expansion 8 --q-log2 128 \
  --model surplus_rank --min-excess 65 --max-excess 76
```

Rows around the crossing:

```text
e=69: log2_bad_top_deficit =  130.819076
e=70: log2_bad_top_deficit =    5.570983
e=71: log2_bad_top_deficit = -119.677892
e=72: log2_bad_top_deficit = -244.927548
```

Thus for relaxed success probability `1/2`, this top-profile stress crosses at:

```text
e = 71.
```

The dominant `e=71` top profile is:

```text
p     = 137
t     = 1845
D_min = 887
q exponent = t - 2D_min + 1 = 72
```

This is a high-deficit, high-surplus repair case. It is not controlled by D=2 exact algebra.

## New Proof Target

The needed local theorem is:

```text
For Hall-OK root-line repair with dim K_P = D and t singleton rows,
Pr[rank < 2D] <= poly(D,t) q^{-(t-2D+1)}
```

over the RFC side domains, or at least an exponent close enough to `t-2D+1` to survive the profile
count.

Equivalently, the bad-root variety should have codimension:

```text
t - 2D + 1
```

inside the `t` root variables, once the singleton functionals are sufficiently generic/Hall-OK.

This is the standard determinantal codimension for an unconstrained `t x 2D` matrix, but the
root-line rows:

```text
(ell_i, alpha_i ell_i)
```

are highly structured. The main open question is whether Hall-OK plus enough singleton surplus
forces the same codimension, or whether projective/root-line structure creates larger bad strata.

The incidence stratification in:

```text
rfc_surplus_incidence_stratification.md
```

shows that Hall-OK alone is not enough. The corrected exponent is:

```text
t - 2D + 1 - flat_excess
```

where:

```text
flat_excess = max_{A: rank(A)<D} ( |A| - 2 rank(A) ).
```

Thus the clean surplus target holds under a flat-sparsity condition `|A|<=2rank(A)` for every
proper-rank flat, or if positive flat excess can be charged globally.

## Meaning

This is a notable change in understanding:

```text
The one-minor Schwartz-Zippel envelope is a valid local lemma but not a near-MDS proof tool.
Near-MDS requires a surplus-sensitive rank-defect codimension theorem plus flat-excess control.
```

The good news is that the required exponent reproduces the original `e ~= 71` crossing in the
coarse top-profile stress test. The bad news is that proving this exponent for general `D` is a
substantial algebraic geometry/matroid problem.

## Next Step

Attack the surplus codimension theorem locally:

```text
1. Prove it for D=1 and restate D=2 PGL2 as the D=2 case.
2. Probe D=3 for codimension t-5 rather than exact probabilities.
3. Try to prove the general Hall-OK case by determinantal/incidence projection:
   count alpha assignments for which there exists a nonzero left/right kernel vector
   annihilated by all rows.
```

The first D=3 codimension probe is recorded in:

```text
rfc_d3_surplus_codimension_signal.md
```

It does not prove the theorem, but it supports the revised target: exact probabilities vary, while
the observed finite-field failures are within moderate constants of the `q^{-(t-5)}` prediction in
small Hall-OK samples.
