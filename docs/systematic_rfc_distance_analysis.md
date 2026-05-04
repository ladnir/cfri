# Systematic RFC Distance Analysis

This is the working target for a nontrivial distance certificate for systematic random foldable
codes. The certificate must use the systematic coordinates as real Hamming weight. It must not
derive distance by ignoring the systematic block and discounting the parity RFC distance by
`c_parity / (1 + c_parity)`.

## Decision

Use the **systematic-at-all-levels** construction as the BaseFold compiler code:

```text
C_i(m) = (m, P_i(m)).
```

Every recursive layer exposes a verifier-known systematic block, and systematic coordinates use
the affine fold rule

```text
(l_j, r_j) -> (1 - alpha) l_j + alpha r_j.
```

The parity block uses RFC `T,T+1` affine interpolation between the two child parity encodings. This
keeps the BaseFold/RMLE protocol semantics intact:

```text
fold(C_i(l,r), alpha) = C_{i-1}((1-alpha)l + alpha r).
```

This is the selected path because alternatives that are top-only systematic either break the
recursive evaluation semantics or need extra proof machinery. For Blaze, that extra machinery
likely reintroduces the input-query/proof-size expansion we are trying to avoid.

## Code Shape

For total expansion `c`, message length `k`, and parity length `n = (c - 1)k`, the systematic
compiler code is

```text
C_sys(m) = (m, P(m)) in F^(k+n).
```

The systematic part is raw caller data. The parity part `P` is the foldable RFC-style parity
encoder. The distance target is

```text
d_sys = min_{m != 0} wt(m) + wt(P(m)).
```

The useful object is therefore not only the ordinary minimum distance of `P`; it is the parity
zero profile conditioned on the support size of `m`.

## Support-Stratified Certificate

For `1 <= s <= k`, define

```text
Z_P(s) = max { zeros(P(m)) : m != 0 and wt(m) = s }.
```

Then

```text
d_sys >= min_s (s + n - Z_P(s))
delta_sys >= min_s (s + n - Z_P(s)) / (k+n).
```

This is the certificate shape we want. The systematic block contributes `s` directly, so sparse
messages cannot be treated the same as dense messages.

The old discount bound is recovered only by replacing every `Z_P(s)` with the single worst-case
zero bound for all nonzero messages. That loses the systematic structure and is not the target.

## Rank Form

Let `P` be represented by a `k x n` parity generator matrix. For a message support
`S subset [k]` with `|S| = s` and a parity zero set `Z subset [n]`, the condition

```text
P(m)_Z = 0
```

for some nonzero `m` supported on `S` is equivalent to the restricted matrix

```text
P[S, Z]
```

failing to have row rank `s`.

So a distance certificate can be stated as:

```text
For every support S of size s and every parity coordinate set Z of size tau_s,
rank(P[S, Z]) = s.
```

If this holds, then no nonzero message with support size `s` can have `tau_s` or more parity
zeros, and therefore

```text
Z_P(s) <= tau_s - 1
d_sys >= min_s (s + n - tau_s + 1).
```

This is the exact finite certificate we should aim to prove analytically for RFCs, and exhaustively
for small test instances.

## Random Linear Benchmark

For an ideal random linear parity matrix over `F_q`, a union-bound certificate is immediate. For
fixed `S`, fixed `Z`, and fixed nonzero `m` supported on `S`,

```text
Pr[P(m)_Z = 0] = q^(-|Z|).
```

Union bounding over messages, supports, and zero sets gives

```text
Pr[exists m with wt(m)=s and zeros(P(m)) >= tau]
  <= binom(k, s) * binom(n, tau) * (q^s - 1) * q^(-tau).
```

Thus, except with the sum of those failure probabilities over all `s`, the systematic code has

```text
d_sys >= min_s (s + n - tau_s + 1)
```

for any chosen thresholds `tau_s`.

This random-linear benchmark is not yet an RFC proof. It is the comparison point. A good systematic
RFC certificate should look like this with RFC-specific dependency losses, rather than like the
discarded parity-distance discount.

## RFC Proof Goal

The BaseFold RFC theorem proves a global upper bound on zero coordinates for every nonzero message.
For systematic distance we need the support-stratified analogue:

```text
For each support size s, bound the largest parity zero set Z such that
the restricted RFC parity equations on S can have a nonzero kernel.
```

Concretely, adapt the RFC rank-nullity proof to track support-restricted nullity:

```text
nullity(P[S, Z]) = 0
```

instead of only tracking the full-message case. The foldable recurrence must preserve this
support-sensitive invariant through each layer. If a layer splits the message as `(l, r)`, the
certificate should track the support sizes in the left and right halves, not just the total message
dimension.

The expected recurrence object is a table or function shaped like

```text
B_i(s) = certified maximum number of parity zeros after i foldable layers
         for a nonzero message of support size s.
```

The final certificate is then

```text
d_sys >= min_s (s + n_d - B_d(s)).
```

## Adapting The BaseFold Proof

The original BaseFold distance proof is close, but it uses one global threshold `t_i`.
The new proof needs a threshold per support size:

```text
Good_i:
  for every nonzero m in F^{k_i} with wt(m)=s,
  nzero(Enc_i(m)) < B_i(s).
```

Set `Z_i(0) = n_i`, because the zero child message has every parity coordinate zero. For
nonzero support size `s`, the certified maximum zero count is `Z_i(s) = B_i(s) - 1`.

The RFC recurrence writes a parent message as

```text
m = (m_l, m_r),  wt(m_l)=u, wt(m_r)=v, u + v = s.
```

Let

```text
S = {j : Enc_i(m_l)[j] = 0 and Enc_i(m_r)[j] = 0}.
```

Every coordinate in `S` creates two parent zeros before randomness enters. Outside `S`, at least
one child value is nonzero. In the local RFC fold

```text
(A_j + T_j(B_j - A_j), A_j + (T_j + 1)(B_j - A_j))
```

at most two choices of the random diagonal entry can make a zero in the pair. Also, outside common
zeros, both parent coordinates cannot be zero at once. For `T` uniform in `F^*`, the per-pair bad
probability is at most `2/(|F|-1)`, and the scripts use the conservative upper bound
`2.002/|F|` for `|F| >= 2^10`. Thus the BaseFold Lemma 3 Bernoulli argument still applies, with
one outside zero per bad pair.

The new work is the support-sensitive replacement for BaseFold Lemma 2.

### Support-Sensitive Kernel Bound

Fix a support set `R subset [k_i]` with `|R| = u`, and a coordinate set
`S subset [n_i]` with `|S| = a`. Under `Good_i`, if `u > 0` and `a < B_i(u)`,
then

```text
|{x supported inside R : Enc_i(x)[S] = 0}| <= |F|^{B_i(u)-a}.
```

If `a >= B_i(u)`, the set contains only the zero vector.

This is the same rank-nullity argument as BaseFold Lemma 2, restricted to the columns indexed by
`R`: if the nullity on `S` were larger than `B_i(u)-a`, then adding `B_i(u)-a` more zero
coordinates would still leave a nonzero vector supported in `R`, contradicting `Good_i`.

Unioning over support sets gives the coarse but useful bound

```text
M_i(u, a) =
  1                                      if u = 0
  min(
    binom(k_i, u) * |F|^{B_i(u)-a},
    binom(k_i, u) * (|F|-1)^u
  )                                      if u > 0 and a < B_i(u)
  0                                      if u > 0 and a >= B_i(u)
```

for the number of child messages of exact support `u` whose encoding vanishes on a fixed `a`-set.
The first term is the support-sensitive kernel bound; the second is the exact-support universe cap.

### One-Step Failure Bound

Suppose `Good_i` holds. For a proposed parent threshold `b = B_{i+1}(s)`, the probability over
the fresh random diagonal that there exists a support-`s` parent message with at least `b` zeros is
bounded by

```text
Fail_{i+1}(s, b)
 <= sum_{u+v=s}
    sum_{a=0}^{n_i}
      binom(n_i, a)
    * M_i(u, a) * M_i(v, a)
    * Tail_i(a,b)

Tail_i(a,b) =
  1                                             if b <= 2a
  binom(n_i-a, b-2a) * (2.002 / |F|)^{b-2a}     if 0 < b-2a <= n_i-a
  0                                             if b-2a > n_i-a.
```

Terms with `b <= 2a` are deliberately treated as probability `1`, not as a negative exponent.

This recurrence is the direct support-stratified analogue of BaseFold Lemmas 2, 3, and 4. The
certificate chooses the smallest `B_{i+1}(s)` such that

```text
sum_s Fail_{i+1}(s, B_{i+1}(s)) <= 2^{-lambda_i}
```

or assigns a per-support budget such as `2^{-lambda_i}/k_{i+1}`.

### Why This Can Beat The Trivial Bound

The old global proof pays for all nonzero messages equally. In the support recurrence:

1. sparse messages have few support choices and contribute `s` systematic nonzeros directly;
2. dense messages have many support choices, but the systematic block already contributes many
   nonzeros;
3. the parity zero threshold `B_i(s)` only needs to be strong where `s + n_i - B_i(s)` could be
   the minimum.

This is exactly the information discarded by the trivial discount.

### Remaining Work

The recurrence above is still a first certificate shape. To make it tight enough for parameters, we
need to improve two overcounts:

1. `M_i(u, a)` still uses an inside-support kernel bound before capping by the exact-support
   universe size. It does not exploit inclusion-exclusion among smaller support subsets.
2. The sum over `a` and all splits may be pessimistic for large `k`; the production certificate
   should use entropy/log-domain bounds rather than literal binomial enumeration.

The proof obligation is nevertheless clear: use the support-sensitive kernel lemma, implement the
recurrence in log space, and compare

```text
min_s (s + n_d - B_d(s)) / (k_d+n_d)
```

against the non-systematic RFC distance at the same total expansion.

## First Numerical Signal

The script `scripts/systematic_rfc_bound.py` implements the first support-stratified recurrence in
log space. It also reports:

```text
old non-systematic:  BaseFold global recurrence at total expansion c
dense heuristic:     (1 + (c-1) * delta_RFC(c-1)) / c
```

Small examples over a 128-bit field with a 40-bit toy failure budget:

```text
total c=4, parity expansion=3, depth=6:
  systematic support recurrence: 0.31250000
  old non-systematic recurrence: 0.49218750
  dense heuristic:               0.49609375

total c=8, parity expansion=7, depth=6:
  systematic support recurrence: 0.65039062
  old non-systematic recurrence: 0.74609375
  dense heuristic:               0.74609375
```

Interpretation:

1. The support recurrence is already better than discarding the systematic block, because the best
   support size is not always `s=1`; it moves through `s=1,2,4,8,...` as depth grows.
2. The recurrence is still far below the dense heuristic. This is probably slack from counting all
   child supports independently in `M_i(u,a) * M_i(v,a)` and from using inside-support rather than
   exact-support kernel counts.
3. Larger parity expansion helps a lot. If we want systematic access with high distance, total
   expansion `c=8` looks much more forgiving than `c=4`.

The next proof-tightening target is an exact-support kernel bound:

```text
K_i^exact(R, a) = number of vectors with exact support R and Enc_i(x)[S] = 0,
```

or an inclusion-exclusion/log-entropy upper bound for exact support. The current inside-support
bound counts vectors supported on subsets of `R`, which is safe but pessimistic and likely explains
part of the distance gap.

Dumping the final support profile for the `c=8`, depth-6 toy recurrence shows the dip is localized
around small powers of two, not spread over all supports:

```text
best support s=8:   333 / 512 = 0.650391
support s=4:        337 / 512 = 0.658203
support s=16:       337 / 512 = 0.658203
```

That profile matches the recursive structure: the valley is in medium-small supports. In the
affine `T,T+1` form, one-child splits are not deterministic, because a nonzero child coordinate can
vanish when the sampled interpolation point hits an endpoint. They are therefore included in the
same random-root union bound as every other split.

With cached combinatorial terms, the same toy recurrence reaches depth 7:

```text
total c=8, parity expansion=7, depth=7:
  systematic support recurrence: 645 / 1024 = 0.62988281
  old non-systematic recurrence:             0.7402344
  dense heuristic:                           0.7402344
```

The depth-8 toy certificate has also been generated and independently checked:

```text
total c=8, parity expansion=7, depth=8:
  systematic support recurrence: 1266 / 2048 = 0.61816406
  old non-systematic recurrence:              0.73486328
  dense heuristic:                            0.73535156
```

At an 80-bit failure target over the same 128-bit field, the checked depth-8 certificate becomes:

```text
total c=8, parity expansion=7, depth=8, security=80:
  systematic support recurrence: 1265 / 2048 = 0.61767578
  old non-systematic recurrence:              0.72363281
  dense heuristic:                            0.72607422
```

The script's security target is per transition, so this gives total failure at most
`8 * 2^-80` for depth 8. A global 80-bit certificate uses `--security-bits 83`, which costs some
distance:

```text
total c=8, parity expansion=7, depth=8, global security=80:
  systematic support recurrence: 1233 / 2048 = 0.60205078
```

At depth 8 the certified minimum occurs at support `16` for both the 40-bit toy run and the global
80-bit run. The tightest verifier row is a separate probability-budget margin; it does not
necessarily occur at the distance-minimizing support.

With split/common-zero precomputation in the generator and verifier, the same affine recurrence
reaches depth 9 under global 80-bit accounting:

```text
total c=8, parity expansion=7, depth=9, global security=80:
  systematic support recurrence: 2418 / 4096 = 0.59033203
  old non-systematic recurrence:              0.70288086
  dense heuristic:                            0.72119141
```

A standalone C++ implementation of the same recurrence lives in
`tools/systematic_rfc_cert_cpp`. It does not modify or link against `libOTe`; it only follows the
same enumerator-style organization with log-domain tables and an independent verifier mode. On the
same `c=8` parameters it reaches depth 10 comfortably:

```text
total c=8, parity expansion=7, depth=10, global security=80:
  systematic support recurrence: 4771 / 8192 = 0.58239746
  old non-systematic recurrence:              0.69787598
  dense heuristic:                            0.71667480
```

In a 10-minute exploratory run the C++ tool completed depth 11, with a very small verifier margin:

```text
total c=8, parity expansion=7, depth=11, global security=80:
  systematic support recurrence: 9418 / 16384 = 0.57482910
  verifier slack:                           -0.017877 bits
```

## Tiny Exhaustive Checks

The script `scripts/exhaustive_systematic_rfc.py` builds paper-style RFC generator matrices over
small prime fields and exhaustively checks all messages. It is only for tiny `p^k`; it is not a
production estimator.

For one GF(5), depth-3 sample:

```text
total c=8, k=8:
  non-systematic RFC distance: 25 / 64 = 0.390625
  systematic distance:         22 / 64 = 0.343750

total c=4, k=8:
  non-systematic RFC distance:  9 / 32 = 0.281250
  systematic distance:         11 / 32 = 0.343750
```

These tiny-field numbers should not be extrapolated to GF(2^128), but they do show an important
qualitative point: the identity block moves the true minimum to a different support regime. In the
`c=4` sample it more than pays for using parity expansion `c-1`; in the `c=8` sample it does not.
For real parameters we should trust the high-probability certificate, not sampled tiny-field
optimism. The value of the exhaustive script is to validate or falsify recurrence guesses on small
instances where the true answer is known.

## Comparison To Non-Systematic RFC

For the same total expansion `c`:

```text
non-systematic RFC:       E_c : F^k -> F^(ck)
systematic RFC candidate: C_sys(m) = (m, P_{c-1}(m)) in F^(ck)
```

The comparison must be:

```text
delta_non_systematic(c)
  versus
min_s (s + (c-1)k - B_{c-1}(s)) / (ck).
```

This is the quantity that decides whether systematic access costs real distance, and by how much.
The answer may depend on field size, base dimension, total expansion, and the RFC dependency losses.

## Implementation Acceptance Bar

Do not plug systematic RFC parameters into backend query counts until we have one of:

1. an analytic support-stratified RFC theorem with explicit `B_d(s)` values, or
2. a verifier-checkable rank certificate for the concrete code sizes we use.

Small tests should exhaustively validate the rank form on toy fields and toy layouts. Large
production parameters need the analytic certificate; brute-force support enumeration is only a
research sanity check.

## Endgame For The Distance Certificate

The target artifact is a parameterized certificate for the selected construction:

```text
C_i(m) = (m, P_i(m))
```

with total expansion:

```text
c = 1 + c_parity.
```

For each layer `i` and support size `s`, the certificate must output an integer threshold

```text
B_i(s)
```

such that, except with the configured failure probability,

```text
for every m with wt(m)=s:
  nzero(P_i(m)) < B_i(s).
```

Then the certified distance is:

```text
d_i = min_{1 <= s <= k_i} (s + n_i - (B_i(s)-1))
delta_i = d_i / (k_i + n_i).
```

### Proof Items

1. **Typed fold correctness.**
   Prove by induction that affine systematic folding and RFC parity folding give:

   ```text
   fold(C_i(l,r), alpha) = C_{i-1}((1-alpha)l + alpha r).
   ```

2. **Support-sensitive kernel lemma.**
   For fixed support `R` with `|R|=s` and parity zero set `S`, prove the restricted kernel bound:

   ```text
   |{m supported in R : P_i(m)[S] = 0}| <= |F|^{B_i(s)-|S|}
   ```

   when `|S| < B_i(s)`, with the zero-only case when `|S| >= B_i(s)`.

3. **Random split case.**
   For every split `u+v=s`, including one-child splits, adapt BaseFold Lemma 3 with common-zero set
   size `a` and Bernoulli root probability at most `2.002/|F|`.

4. **Union-bound schedule.**
   Allocate failure probability across layers and support sizes, then choose the minimum `B_i(s)`
   satisfying the bound.

### Calculator Items

The current `scripts/systematic_rfc_bound.py` is a research calculator. To become a certificate
generator, it needs:

1. log-domain computation for production-sized `k_i`;
2. entropy/Stirling bounds instead of literal binomial tables for large `n_i`;
3. explicit failure-budget accounting per layer/support;
4. output of `(B_i(s), d_i, delta_i)` plus the worst support;
5. reproducible parameter files checked into docs or tests;
6. cross-checks against exhaustive tiny-field scripts.

### Parameter Direction

Current toy certificates indicate:

```text
rate 1/4: distance hit is large; use only if proof-size pressure dominates.
rate 1/8: distance hit is materially smaller and looks like the safer default.
```

The working expectation is that Blaze2 should prefer `c=8` systematic compiler code unless a final
proof-size/query-count calculation shows `c=4` is still better overall.
