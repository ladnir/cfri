# Distance Proof For Systematic RFC

This document develops the proof for the selected systematic-at-all-levels BaseFold compiler code.
It is written as a proof skeleton with explicit lemmas, thresholds, and failure probabilities. The
goal is to make the final distance certificate a matter of instantiating the recurrence, not
inventing more proof structure.

## Construction

Fix a finite field `F`, parity expansion `c_p`, base message length `k_0`, and depth `d`.

For each layer `i`, let:

```text
k_i = k_0 2^i
n_i = c_p k_i
N_i = k_i + n_i = (1+c_p)k_i.
```

Let `P_i : F^{k_i} -> F^{n_i}` be the RFC parity encoder with the same recursive shape as the
BaseFold random foldable code, but with expansion `c_p`. The systematic compiler code is:

```text
C_i(m) = (m, P_i(m)) in F^{N_i}.
```

For `i > 0`, write a message as `m = (l,r)` with `l,r in F^{k_{i-1}}`. If

```text
L = P_{i-1}(l)
R = P_{i-1}(r)
D = R - L
```

then the parity block is built coordinatewise as:

```text
P_i(l,r) = (L + T_i D, L + T'_i D),
```

where each diagonal entry is sampled independently from the configured RFC distribution, and
`T'_i = T_i + 1`. This offset pairing has two properties used by the distance proof:

1. outside common zeros of `L` and `R`, a parity pair has at most two bad diagonal choices that
   create a zero;
2. outside common zeros, a parity pair cannot have both parent coordinates zero at once.

The second property is what lets the tail bound count one outside zero per bad pair. If a different
pairing is used, for example a pairing that can produce two zeros in one non-common pair, then the
tail exponent below must be weakened accordingly.

The fold rule is typed:

```text
systematic pair: (l_j, r_j) -> (1-alpha)l_j + alpha r_j
parity pair:     interpolate between L_j and R_j at alpha
```

Therefore:

```text
fold(C_i(l,r), alpha) = C_{i-1}((1-alpha)l + alpha r).
```

This establishes protocol correctness separately from distance.

## Distance Certificate Target

For `s in {1,...,k_i}`, let `B_i(s)` be an integer threshold. The desired good event is:

```text
Good_i(B_i):
  for every nonzero m in F^{k_i} with wt(m)=s,
  nzero(P_i(m)) < B_i(s).
```

If `Good_i(B_i)` holds, then the systematic code distance is at least:

```text
d_i >= min_{1 <= s <= k_i} (s + n_i - (B_i(s)-1)).
```

Equivalently:

```text
delta_i >= min_s (s + n_i - B_i(s) + 1) / (k_i+n_i).
```

We also define a zero-message threshold:

```text
B_i(0) = n_i + 1,
```

so that `nzero(P_i(0)) = n_i < B_i(0)`.

For positive supports, a useful certificate must use admissible thresholds:

```text
1 <= B_i(s) <= n_i  for 1 <= s <= k_i.
```

The upper bound is needed by the support-sensitive kernel lemma below: it ensures that when
`a < B_i(s)`, there are at least `B_i(s)-a` parity coordinates outside the fixed zero set.
The special value `B_i(0)=n_i+1` is only for the zero child and is never used as a positive-support
threshold.

## Base Layer

The proof only needs a base certificate `Good_0(B_0)`. For the current calculator and toy
certificates, `k_0 = 1` and the parity base code is the repetition-style map:

```text
P_0(x) = (c_1 x, c_2 x, ..., c_{n_0} x),
```

with all `c_j != 0`. Therefore every nonzero base message has no parity zeros, and the tight base
threshold is:

```text
B_0(1) = 1.
```

For any future `k_0 > 1` or non-repetition base code, this section should be replaced by an
explicit base threshold table satisfying `Good_0(B_0)`. The rest of the proof is unchanged once
that table is supplied and monotone over positive supports.

## Randomness Assumptions

The distance theorem uses a per-coordinate randomness condition in the same style as the original
BaseFold RFC proof. At each recursive layer:

1. The diagonal samples are independent across parity coordinate pairs.
2. For every pair of child values `(x,y)` not both zero, the parent pair has at most one zero, and
   the probability that it creates such a zero is at most:

   ```text
   2.002 / |F|.
   ```

For the `T' = T + 1` affine-interpolation pairing, the one-zero property follows because both
equations:

```text
x + T(y-x) = 0
x + (T+1)(y-x) = 0
```

imply `y-x = 0` and then `x = 0`, hence `y = 0`. Thus a non-common pair cannot produce two zeros.
For diagonal entries sampled uniformly from `F^*`, the probability bound follows from the fact that
at most two diagonal values can create a zero in the two affine parent coordinates, so the exact
one-coordinate probability is at most `2/(|F|-1)`. For `|F| >= 2^10`, this is at most
`2.002/|F|`. The proof and scripts use the latter expression as a simple conservative bound. The
proof below assumes `2.002/|F| <= 1/2`; again, `|F| >= 2^10` is more than enough.

## Lemma 1: Typed Fold Correctness

**Lemma.** For every `i > 0`, every `l,r in F^{k_{i-1}}`, and every folding challenge `alpha`,
typed folding of `C_i(l,r)` yields:

```text
C_{i-1}((1-alpha)l + alpha r).
```

**Proof.** The systematic coordinates are raw pairs `(l_j,r_j)`, so affine interpolation gives the
raw folded message coordinate `(1-alpha)l_j + alpha r_j`. The parity coordinates are RFC
`T,T+1` pairs built as affine evaluations between `P_{i-1}(l)` and `P_{i-1}(r)`, so interpolation at
`alpha` gives `(1-alpha)P_{i-1}(l)_j + alpha P_{i-1}(r)_j`. By linearity of `P_{i-1}`, this equals
`P_{i-1}((1-alpha)l + alpha r)_j`. Combining the two coordinate classes yields the claimed
systematic codeword. QED.

## Lemma 2: Support-Sensitive Kernel Bound

Fix layer `i` and assume `Good_i(B_i)`, monotone positive-support thresholds, and admissible
positive-support thresholds. For a support set `R subset [k_i]`, write:

```text
V_R = {m in F^{k_i} : supp(m) subseteq R}.
```

For a parity coordinate set `S subset [n_i]`, define:

```text
K_i(R,S) = {m in V_R : P_i(m)[S] = 0}.
```

**Lemma.** If `|R| = s` and `|S| = a < B_i(s)`, then:

```text
|K_i(R,S)| <= |F|^{B_i(s)-a}.
```

If `a >= B_i(s)`, then:

```text
K_i(R,S) = {0}.
```

**Proof.**

The restriction of `P_i[ S ]` to `V_R` is a linear map from a vector space of dimension at most `s`
to `F^a`. Its kernel is `K_i(R,S)`.

First suppose `a >= B_i(s)`. If there were a nonzero `m in K_i(R,S)`, then `wt(m) = s'` for some
`1 <= s' <= s`, and `P_i(m)` would have at least `a >= B_i(s)` zeros. This directly contradicts
`Good_i(B_i)` only when `B_i(s) >= B_i(s')`. Thus the certificate must enforce monotonicity:

```text
B_i(s') <= B_i(s) for all s' <= s.
```

Under this monotonicity condition, `a >= B_i(s) >= B_i(s')` contradicts `Good_i`, so the kernel is
zero.

Now suppose `a < B_i(s)`. Since `B_i(s) <= n_i`, choose any set `T subset [n_i] \ S` of size
`B_i(s)-a`. If a nonzero
vector lies in `K_i(R,S union T)`, the same monotonicity argument contradicts `Good_i`.
Therefore the restricted map on `S union T` has zero kernel over `V_R`; its rank is `dim(V_R)`.
The rows indexed by `T` can increase rank by at most `|T| = B_i(s)-a`, hence the nullity of the
map on `S` is at most `B_i(s)-a`. Therefore:

```text
|K_i(R,S)| <= |F|^{B_i(s)-a}.
```

QED.

**Important condition.** The monotonicity requirement can be enforced without loss by replacing
`B_i(s)` with the prefix maximum:

```text
B_i^mono(s) = max_{1 <= t <= s} B_i(t).
```

This may slightly weaken the distance certificate but makes the kernel lemma sound for
inside-support counting. The resulting positive-support thresholds must still satisfy
`B_i(s) <= n_i`; otherwise the table is not an admissible certificate.

## Lemma 3: Child-Count Bound

Assume `Good_i(B_i)` and monotone positive-support thresholds. For `u in {0,...,k_i}` and
`a in {0,...,n_i}`, define:

```text
M_i(u,a) =
  1                                      if u = 0,
  min(
    binom(k_i,u) * |F|^{B_i(u)-a},
    binom(k_i,u) * (|F|-1)^u
  )                                      if u > 0 and a < B_i(u),
  0                                      if u > 0 and a >= B_i(u).
```

**Lemma.** For any fixed parity coordinate set `A` with `|A|=a`, the number of child messages with
support size exactly `u` whose parity encoding vanishes on `A` is at most `M_i(u,a)`.

**Proof.** If `u=0`, there is only the zero child. If `u>0` and `a >= B_i(u)`, Lemma 2 says every
message supported inside any fixed `u`-set and vanishing on `A` is zero, so there are no exact
support-`u` messages.

Now take `u>0` and `a < B_i(u)`. Choose the exact support set in `binom(k_i,u)` ways. For each
chosen support, Lemma 2 gives at most `|F|^{B_i(u)-a}` messages supported inside that set and
vanishing on `A`. Independently, the total number of exact-support messages on the chosen support
is at most `(|F|-1)^u`. Taking the smaller of these two bounds and unioning over support sets gives
`M_i(u,a)`. QED.

## Definition: Support Failure Functional

Assume `Good_i(B_i)`, monotone positive-support thresholds, and admissible positive-support
thresholds.

For proposed parent threshold `b = B_{i+1}(s)`, define the support failure functional by summing
over all splits `u+v=s`, including one-child splits. For each common-zero size `a`, the term is:

```text
binom(n_i,a) * M_i(u,a) * M_i(v,a) * Tail_i(a,b),
```

where:

```text
Tail_i(a,b) =
  1                                             if b <= 2a,
  binom(n_i-a, b-2a) * (2.002/|F|)^{b-2a}       if 0 < b-2a <= n_i-a,
  0                                             if b-2a > n_i-a.
```

Thus:

```text
FailBound_{i+1}(s,b)
  =
  sum_{u+v=s}
  sum_{a=0}^{n_i}
    binom(n_i,a) * M_i(u,a) * M_i(v,a) * Tail_i(a,b).
```

This is the exact theorem-side quantity checked by `scripts/verify_systematic_rfc_certificate.py`.
One-child splits are not deterministic in the affine `T,T+1` construction: if one child parity
value is zero and the other is nonzero, a parent coordinate can still vanish when `T` hits one of
the two interpolation endpoints. The same random-root bound handles those cases.

## Lemma 4: One-Step Failure Bound

Assume `Good_i(B_i)` and the monotonicity condition above. Let `B_{i+1}(s)` be a proposed next-layer
threshold. For parent support size `s`, define:

```text
Fail_{i+1}(s)
```

as the probability over the fresh random diagonal that there exists a message
`m=(l,r)` with `wt(m)=s` and:

```text
nzero(P_{i+1}(m)) >= B_{i+1}(s).
```

Then:

```text
Fail_{i+1}(s) <= FailBound_{i+1}(s, B_{i+1}(s)).
```

**Proof.**

We split on `u = wt(l)` and `v = wt(r)`, so `u+v=s`.

Let:

```text
A = {j : P_i(l)[j] = 0 and P_i(r)[j] = 0}
```

be the common parity zero set, with `|A|=a`.

### Split Analysis

Common zeros in `A` give `2a` forced parent zeros. Outside `A`, at least one of the
two child parity values is nonzero. For each coordinate `j notin A`, the two parent parity
coordinates are:

```text
L_j + T_j(R_j - L_j)
L_j + (T_j+1)(R_j - L_j).
```

There are at most two diagonal values that make at least one of the two parent coordinates zero.
For uniform sampling from `F^*`, this has probability at most `2/(|F|-1)`, which is at most:

```text
2.002 / |F|
```

for `|F| >= 2^10`.
The diagonal entries are independent across `j`, so for any fixed set `H subseteq [n_i] \ A` of
outside pairs:

```text
Pr[every pair in H contributes a zero] <= (2.002/|F|)^{|H|}.
```

Since each outside pair contributes at most one zero, reaching `b` total parent zeros requires at
least `h = b-2a` bad outside pairs. If `h > n_i-a`, this is impossible. Otherwise, the event implies
that there exists an `h`-subset of outside pairs that are all bad, so union bounding over those
subsets gives:

```text
binom(n_i-a,h) * (2.002/|F|)^h,
```

with `h = b-2a`. If `b <= 2a`, the forced zeros alone may already cross the threshold, so the safe
tail bound is `1`.

For a fixed split `(u,v)` and a fixed set `A` of size `a`, the number of candidate child pairs whose
actual common-zero set is exactly `A` is at most:

```text
M_i(u,a) * M_i(v,a),
```

using Lemma 3 for the fixed common-zero set `A`. Lemma 3 only requires both children to vanish on
`A`; it does not enforce that `A` is exact. That is safe because vanishing on `A` is a superset of
having exact common-zero set `A`. In the union bound, the failure event is partitioned by the actual
common-zero set, while the number of pairs in each part is bounded by this larger vanishing-on-`A`
count.

Union bounding over splits, common-zero sets `A`, candidate child pairs, and bad outside-pair sets
is exactly `FailBound_{i+1}(s,b)`. QED.

## Theorem: Support-Stratified Systematic RFC Distance

Let `epsilon_i(s)` be nonnegative failure budgets. Suppose:

1. `Good_0(B_0)` holds for the base code.
2. Each `B_i` is monotone under prefix maximum over positive supports.
3. Each positive-support threshold is admissible: `1 <= B_i(s) <= n_i`.
4. For every layer `i < d` and support size `s`, the threshold `B_{i+1}(s)` satisfies:

   ```text
   FailBound_{i+1}(s, B_{i+1}(s)) <= epsilon_{i+1}(s).
   ```

Then with probability at least:

```text
1 - sum_{i=1}^d sum_{s=1}^{k_i} epsilon_i(s),
```

the depth-`d` systematic code has distance:

```text
d_sys >= min_s (s + n_d - B_d(s) + 1).
```

**Proof.** Induct on `i`. The base case is `Good_0`. Assuming `Good_i`, Lemmas 2 through 4 bound the
probability that `Good_{i+1}` fails at each support size:

```text
Pr[not Good_{i+1}(B_{i+1}) | Good_i(B_i)]
  <= sum_{s=1}^{k_{i+1}} FailBound_{i+1}(s, B_{i+1}(s))
  <= sum_{s=1}^{k_{i+1}} epsilon_{i+1}(s).
```

Telescoping over layers and union bounding the bad transition events gives the claimed success
probability. When `Good_d` holds, the distance formula follows from:

```text
wt(C_d(m)) = wt(m) + wt(P_d(m))
           = s + n_d - nzero(P_d(m))
           >= s + n_d - B_d(s) + 1.
```

QED.

## Failure-Budget Accounting

The theorem is parameterized by explicit budgets `epsilon_i(s)`. A convenient schedule for a target
global failure probability `2^-lambda` is:

```text
epsilon_i(s) = 2^-lambda / (d * k_i)
```

for every transition layer `i in {1,...,d}` and every positive support `s <= k_i`. Then:

```text
sum_{i=1}^d sum_{s=1}^{k_i} epsilon_i(s) <= 2^-lambda.
```

The current research scripts use a per-transition setting:

```text
epsilon_i(s) = 2^-lambda_round / k_i.
```

So a depth-`d` run with `--security-bits lambda_round` has total failure probability at most:

```text
d * 2^-lambda_round.
```

To get global `lambda` bits with those scripts, use:

```text
lambda_round = lambda + ceil(log2(d)).
```

For example, a depth-8 global 80-bit certificate can be checked by running the scripts with
`--security-bits 83`.

## Certificate Contract

A certificate artifact consists of:

1. a full-threshold CSV,
2. declared parameters `(d, c_p, |F|, lambda_round)`,
3. the verifier command that checks the CSV under those parameters.

The full-threshold CSV must contain, for every round `i` and positive support `s`, the tuple:

```text
(i, k_i, n_i, s, B_i(s)).
```

The independent verifier checks:

1. The field is in the theorem regime, currently `field_bits >= 10`.
2. Supports are contiguous for each round.
3. Every row's declared `k_i` and `n_i` matches the round shape.
4. Round numbers are contiguous from `0`.
5. `k_i` and `n_i` double each round.
6. The parity expansion `n_i/k_i` is integral and constant.
7. Positive-support thresholds are monotone.
8. Positive-support thresholds are admissible: `1 <= B_i(s) <= n_i`.
9. The support failure functional satisfies the configured per-support budget.

The generator is also required to fail rather than emit a positive-support threshold outside
`1..n_i`. Such a row would not satisfy the kernel lemma and therefore is not a valid theorem
certificate, even though it would give a formal Hamming-distance expression.

Therefore, if the verifier accepts the threshold CSV under the declared parameters with per-round
budget `lambda_round`, then with probability at least:

```text
1 - d * 2^-lambda_round,
```

the systematic RFC code at the final round has distance at least:

```text
min_s (s + n_d - B_d(s) + 1).
```

This is the exact distance certificate shape we need. Productionizing the certificate means
replacing the toy `lgamma` binomial arithmetic with conservative entropy/Stirling bounds, while
preserving this verifier contract.

## Checked Toy Certificates

This proof is now structurally complete for the selected recurrence, and the toy certificate
pipeline has three pieces:

1. `scripts/systematic_rfc_bound.py` emits threshold tables.
2. `scripts/verify_systematic_rfc_certificate.py` independently checks the theorem conditions from
   the emitted threshold table.
3. `tools/systematic_rfc_cert_cpp/systematic_rfc_cert` is a standalone C++ generator/verifier for
   the same recurrence, intended for deeper runs.

The file `docs/systematic_rfc_c8_depth8_certificate.csv` is the main small checked toy certificate.
It was emitted with:

```text
python scripts/systematic_rfc_bound.py \
  --depth 8 \
  --parity-expansion 7 \
  --field-bits 128 \
  --security-bits 40 \
  --compare \
  --monotone-thresholds \
  --emit-certificate \
  --certificate-path docs/systematic_rfc_c8_depth8_certificate.csv \
  --full-thresholds-path docs/systematic_rfc_c8_depth8_thresholds.csv
```

and independently verified with:

```text
python scripts/verify_systematic_rfc_certificate.py \
  docs/systematic_rfc_c8_depth8_thresholds.csv \
  --field-bits 128 \
  --security-bits 40
```

The verifier reports:

```text
ok=true
rounds=9
final_round=8
final_k=256
final_parity_n=1792
parity_expansion=7
final_distance=1266
final_relative_distance=0.618164062500
final_worst_distance_support=16
worst_log2_slack=-0.624803701828
worst_slack_round=8
worst_slack_support=42
```

The same depth-8 certificate at an 80-bit failure target is in
`docs/systematic_rfc_c8_depth8_security80_certificate.csv` and
`docs/systematic_rfc_c8_depth8_security80_thresholds.csv`. It verifies with:

```text
ok=true
rounds=9
final_round=8
final_k=256
final_parity_n=1792
parity_expansion=7
final_distance=1265
final_relative_distance=0.617675781250
final_worst_distance_support=24
worst_log2_slack=-0.008407883007
worst_slack_round=4
worst_slack_support=1
```

Since the scripts allocate security per transition, the corresponding depth-8 global 80-bit
certificate uses `--security-bits 83`. It is in
`docs/systematic_rfc_c8_depth8_global80_certificate.csv` and
`docs/systematic_rfc_c8_depth8_global80_thresholds.csv`. The verifier command is:

```text
python scripts/verify_systematic_rfc_certificate.py \
  docs/systematic_rfc_c8_depth8_global80_thresholds.csv \
  --field-bits 128 \
  --security-bits 83
```

It reports:

```text
ok=true
rounds=9
final_round=8
final_k=256
final_parity_n=1792
parity_expansion=7
final_distance=1233
final_relative_distance=0.602050781250
final_worst_distance_support=16
worst_log2_slack=-0.665423591256
worst_slack_round=8
worst_slack_support=178
```

The same global 80-bit accounting one layer deeper uses `--security-bits 84`, since
`ceil(log2(9)) = 4`. The depth-9 artifact is in
`docs/systematic_rfc_c8_depth9_global80_certificate.csv` and
`docs/systematic_rfc_c8_depth9_global80_thresholds.csv`. The verifier command is:

```text
python scripts/verify_systematic_rfc_certificate.py \
  docs/systematic_rfc_c8_depth9_global80_thresholds.csv \
  --field-bits 128 \
  --security-bits 84
```

It reports:

```text
ok=true
rounds=10
final_round=9
final_k=512
final_parity_n=3584
parity_expansion=7
final_distance=2418
final_relative_distance=0.590332031250
final_worst_distance_support=32
worst_log2_slack=-1.559818369695
worst_slack_round=8
worst_slack_support=15
```

The standalone C++ tool reaches depth 10 under the same global 80-bit accounting, again using
`--security-bits 84` since `ceil(log2(10)) = 4`. The depth-10 artifact is in
`docs/systematic_rfc_c8_depth10_global80_certificate.csv` and
`docs/systematic_rfc_c8_depth10_global80_thresholds.csv`. The verifier command is:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe \
  --verify docs/systematic_rfc_c8_depth10_global80_thresholds.csv \
  --field-bits 128 \
  --security-bits 84
```

It reports:

```text
ok=true
rounds=11
final_round=10
final_k=1024
final_parity_n=7168
parity_expansion=7
final_distance=4771
final_relative_distance=0.582397460938
final_worst_distance_support=64
worst_log2_slack=-0.649539308978
worst_slack_round=10
worst_slack_support=827
```

A 10-minute exploratory C++ run also completed depth 11 before the time limit and was verified.
The depth-11 artifact is in `docs/systematic_rfc_c8_depth11_global80_certificate.csv` and
`docs/systematic_rfc_c8_depth11_global80_thresholds.csv`. It reports:

```text
ok=true
rounds=12
final_round=11
final_k=2048
final_parity_n=14336
parity_expansion=7
final_distance=9418
final_relative_distance=0.574829101562
final_worst_distance_support=64
worst_log2_slack=-0.0178770652584
worst_slack_round=11
worst_slack_support=1666
```

The final distance bottleneck is not the same as the tightest probability row. At depth 8 global
80-bit security, support `16` has threshold `576`, giving:

```text
16 + 1792 - (576 - 1) = 1233.
```

Unlike the earlier linear-direction analysis, this is not a deterministic one-child inherited
threshold. In the affine systematic-correct form, one-child splits are part of the same random-root
failure sum as all other splits.

For reference, the depth-8 power-of-two support profile is:

```text
support  distance / 2048
1        1499 / 2048 = 0.73193359375
2        1367 / 2048 = 0.66748046875
4        1302 / 2048 = 0.63574218750
8        1242 / 2048 = 0.60644531250
16       1233 / 2048 = 0.60205078125
32       1233 / 2048 = 0.60205078125
64       1257 / 2048 = 0.61376953125
128      1315 / 2048 = 0.64208984375
256      1437 / 2048 = 0.70166015625
```

## Remaining Gaps

There are still certificate-engine obligations before this is a production parameter certificate:

1. Decide whether inside-support counting is acceptable or replace it with exact-support counting.
2. Replace literal binomial sums with entropy/log bounds for production sizes.
3. Emit final parameter tables for the selected `c`, `k`, field size, and security budget.

The generator and verifier now precompute the split/common-zero log-sums for each transition. This
does not change the recurrence. The Python engine is usable through depth 9 for `c=8`; the
standalone C++ engine has been checked through depth 11.

The research calculator now has `--monotone-thresholds` to mirror the proof condition. For the
current `c=8`, depth-6 toy run, enabling it does not change the certificate, which suggests the
computed thresholds are already monotone over positive support sizes in that regime.

The file `docs/systematic_rfc_c8_depth6_certificate.csv` is an example certificate summary emitted
by:

```text
python scripts/systematic_rfc_bound.py \
  --depth 6 \
  --parity-expansion 7 \
  --field-bits 128 \
  --security-bits 40 \
  --compare \
  --monotone-thresholds \
  --emit-certificate \
  --certificate-path docs/systematic_rfc_c8_depth6_certificate.csv \
  --full-thresholds-path docs/systematic_rfc_c8_depth6_thresholds.csv
```

The columns are:

```text
round                  RFC/systematic layer
k                      message length
parity_n               parity length
support                worst support for distance at this layer
threshold              B_i(s) at that worst support
distance               certified systematic distance at this layer
relative_distance      distance / (k + parity_n)
worst_log2_slack       max_s(log2 Fail_i(s) - log2 budget_i(s))
worst_support          support size attaining worst_log2_slack
```

For this toy certificate, every `worst_log2_slack` after round 0 is negative, so the thresholds pass
the theorem checker under the toy 40-bit budget. The tightest verifier row has about `3.4` bits of
slack.

The companion `docs/systematic_rfc_c8_depth6_thresholds.csv` contains every threshold `B_i(s)` used
by the certificate. For depth 6, it has `1+2+4+8+16+32+64 = 127` threshold rows.

The threshold file can be independently checked with:

```text
python scripts/verify_systematic_rfc_certificate.py \
  docs/systematic_rfc_c8_depth6_thresholds.csv \
  --field-bits 128 \
  --security-bits 40
```

For the checked-in toy certificate, the verifier reports:

```text
ok=true
rounds=7
final_round=6
final_k=64
final_parity_n=448
parity_expansion=7
final_distance=333
final_relative_distance=0.650390625000
final_worst_distance_support=8
worst_log2_slack=-3.367046167106
worst_slack_round=5
worst_slack_support=1
```

Tiny exact checks are available through `scripts/exhaustive_systematic_rfc.py`. For example, for
GF(5), depth 3:

```text
python scripts/exhaustive_systematic_rfc.py \
  --p 5 \
  --depth 3 \
  --total-expansion 8 \
  --seed 1 \
```

The tiny-field script is not a certificate generator. In fact, after switching to the affine
systematic-correct recurrence, the threshold generator may fail to find admissible tables over
GF(5), which is below the theorem field-size regime. These runs are only sanity checks for concrete
sampled codes.
