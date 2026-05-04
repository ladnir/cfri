# RFC Rank Certificate Outline

This is the candidate route from the current experiments to a full distance certificate.

## 1. Column Algebra

Each parity column is a tree tensor:

```text
C_{a,path} = tensor_i u_{path_i}(T_{a,prefix_i})
```

with:

```text
u_0(T) = (1-T, T)
u_1(T) = (-T, T+1).
```

The non-systematic generator is a stack of independent copies of these columns. The systematic
generator is:

```text
G_sys = [I | P].
```

## 2. Rank-To-Distance Reduction

Distance greater than `D` follows if every zero set of size:

```text
z = N - D
```

has full column rank `k`.

For systematic zero sets:

```text
Z = S identity columns + P parity columns
```

the rank condition is:

```text
rank(I_S, P_Z) = k.
```

Equivalently, after quotienting out the identity coordinates in `S`, the selected parity columns
must span the remaining `k-|S|` dimensions.

## 3. Generic-Rank Statement

The main algebraic theorem should be phrased over indeterminates `T`, not over a sampled field:

```text
For every zero-set shape Z outside an explicitly defined recursive collision family B_d,
there exists a k x k minor of G_Z whose determinant is a nonzero polynomial in the T variables.
```

For the original non-systematic RFC, the conjectural collision family is empty at `|Z| >= k`:

```text
B_d^orig = empty.
```

For the systematic RFC, `B_d^sys` contains recursive identity/parity sibling-collision shapes.
The experiments show these are sparse and tree-structured.

## 4. Accidental-Zero Bound

If a shape has a nonzero determinant minor of total degree at most `Delta`, then by
Schwartz-Zippel over `T in (F^*)^m`:

```text
Pr[rank(G_Z) < k] <= Delta / (q-1).
```

This is useful for explaining sporadic tiny-field failures, but it is probably not strong enough by
itself for a final union bound over all zero-set shapes at production sizes. The final certificate
needs one of:

```text
1. a deterministic generic-rank theorem for all non-structural shapes, plus a fixed sampled
   challenge assignment that verifies the needed minors; or
2. a higher-power rank-loss tail, where losing rank r costs roughly q^-r; or
3. a shape-count refinement showing only a much smaller family needs accidental-zero charging.
```

The determinant degree is at most:

```text
Delta <= k * depth
```

for a `k x k` minor, because each column is multilinear with one factor per depth level.

## 5. Structural-Defect Count

Structural defects must be counted combinatorially:

```text
# {Z of size z in B_d^sys}.
```

The depth-3 exact rows suggest the right family is described by:

```text
complete systematic sibling subtrees
parity paths that collide under corresponding recursive projections
same expansion-copy reuse
```

A certificate can tolerate structural defects below the target zero count. For example, if all
structural defects have:

```text
|Z| <= k + e - 1,
```

and accidental failures are union-bounded, then:

```text
distance >= N - (k + e) + 1.
```

## 6. Current Empirical Targets

The data supports:

```text
original:
  generic MDS, i.e. e = 0

systematic:
  depth 2: e = 1 exactly in large-prime check
  depth 3: e = 2 in sampled/exact-row checks
  depth 4: no sampled failures at e = 2
```

So the next proof target is:

```text
systematic e <= 2 or e <= O(depth),
```

with an explicit recursive collision count. Even `e = O(depth)` would be dramatically stronger
than the threshold-distance certificate at the intended parameters.

## 7. Next Formal Step

Define a recursive collision score `rho_d(S,P)` such that:

```text
rho_d(S,P) = 0  =>  generic full rank
rho_d(S,P) > 0  =>  structural rank loss can occur
```

Then test on all exact depth-2 rows and exact depth-3 rows already enumerated. The score should
separate persistent structural defects from accidental finite-field determinant zeros.

The first useful finite-depth approximation is level-aware:

```text
I_l = number of complete systematic subtrees of size 2^l
P_l = number of parity collisions after projecting paths modulo level l
```

For the exact depth-3 rows, the feature tables in `docs/rank_tree_feature_row_*` show that
persistent defects are isolated by a small number of `(s_identity, z_parity, I_l, P_l)` buckets.
The regression script:

```text
python scripts/test_tree_collision_rule.py \
  docs/rank_tree_feature_row_s6_zp2_systematic_p65537_depth3_c8.csv \
  docs/rank_tree_feature_row_s5_zp3_systematic_p65537_depth3_c8.csv \
  docs/rank_tree_feature_row_s6_zp3_systematic_p65537_depth3_c8.csv \
  --out docs/rank_tree_collision_rule_depth3_exact_rows.csv
```

has:

```text
false positives: 0
false negatives: 35
```

and those 35 false negatives are precisely the accidental finite-field zeros that disappeared under
resampling. This makes the level-aware score the current best candidate for `rho_d`.

The same feature language catches the exact depth-2 obstruction:

```text
s_identity=2, z_parity=2:
  I_1=1 and P_1=1: 28/28 bad
  all other buckets: 0 bad
```

This gives the base case for the recursive collision family: a complete systematic sibling pair
combined with a parity sibling collision in the same projection class causes a deterministic rank
loss.

Sampled depth-4 feature rows also line up with this score. For example:

```text
python scripts/rank_tree_feature_row.py \
  --depth 4 \
  --total-expansion 8 \
  --prime 65537 \
  --seed 307 \
  --s-identity 11 \
  --z-parity 5 \
  --shape-samples 20000 \
  --out docs/rank_tree_feature_sample_s11_zp5_systematic_p65537_depth4_c8.csv
```

The sampled bad buckets all have multiple complete identity subtrees and parity projection
collisions across several levels. This is not exact enumeration, but it supports the same recursive
collision-score direction at one higher depth.
