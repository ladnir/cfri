# RFC Fixed-Survivor Rank-Tail Go/No-Go Checkpoint

Status: active but in danger of becoming a local-invariant grind.

This is the hard checkpoint for the fixed-survivor rank-tail route. It is intentionally more
skeptical than the diagnostic notes. The point is to separate real proof movement from the pattern
of discovering one more local invariant after another.

## Route Being Judged

For a fixed survivor coordinate set `S`, prove a tail bound for:

```text
Pr_T[rank(G_S(T)) < k]
```

or for the stronger moment:

```text
E_T[q^(k-rank(G_S(T))) - 1].
```

This is a fixed-set erasure-recovery route for the original non-systematic RFC. It is not the older
aggregate replica moment, not a fixed-set MDS proof, and not yet the systematic RFC route.

The top fold splits child positions into:

```text
P = paired child coordinates, both siblings survive
T = singleton child coordinates, exactly one sibling survives
```

The central identity is:

```text
rank_parent(S) = 2 rank_child(P) + singleton_increment(P,T).
```

With child pair deficit:

```text
D = k_child - rank_child(P),
```

parent full rank is equivalent to:

```text
singleton_increment(P,T) >= 2D.
```

This identity is the reason the route is still worth considering. It turns the bad event into a
recursive child rank-tail plus a local root-line repair problem.

## Evidence That This Is Real Progress

The paired compression identity has held in all exact and sampled checks and is now proved in
`rfc_paired_compression_rank_lemma.md` by the determinant-1 local coordinate transform.

The old scalar summaries failed for a useful reason. The failure was not random noise; it exposed
the actual local object:

```text
K_P = ker(child evaluation on P),
K_P restricted to singleton coordinates T.
```

For singleton rows, the repair equations on `K_P x K_P` have the root-line form:

```text
ell_j(x) + alpha_j ell_j(y) = 0.
```

The side domains are fixed by the construction:

```text
left singleton:  alpha in F_q^*
right singleton: alpha in F_q \ {1}
```

For `D=2`, the projective parallel-class signature of `K_P|_T` appears to predict the exact
root-line repair failure probability:

```text
zero singleton columns,
multiset of (left_count,right_count) over projective parallel classes.
```

This is stronger than the rank/Hall histogram, which fails on mixed-side examples. The diagnostic
separation is important:

```text
rank/Hall ambiguity on mixed sides: many ambiguous signatures
D=2 parallel-class ambiguity:      zero ambiguous signatures in tested samples
canonical D=2 formula mismatches:  zero in tested samples
```

The standalone cross-ratio self-test also survived all small checked signatures through total
singleton count five over `GF(5)`, plus selected `GF(7)` cases. This specifically attacks the
main possible leak: dependence on the cross-ratio of projective directions rather than only their
parallel classes and side counts.

There is now a safer proof fallback for this leak. In the no-active `D=2` case, rank failure for
four or more inactive projective classes means the points `(ell_i,alpha_i)` lie on a `(1,1)` divisor
in `P^1 x P^1`. Counting fractional-linear maps gives at most `q(q^2-1)+q = q^3` bad common-alpha
assignments before side-domain restrictions. Thus the exact count may have cross-ratio lower-order
terms, but the conditional exponent is uniformly `O(q^-1)` for four single-row inactive classes and
`O(q^(3-m))` for `m` inactive classes. Duplicate singleton rows in a class only improve the
unconditional exponent. This may be theorem-grade even if exact cross-ratio independence stays
annoying.

The explicit version of this fallback is now recorded in:

```text
rfc_d2_rootline_pgl2_envelope.md
```

## Evidence That We Are Partly Spinning

The route has repeatedly needed sharper state:

```text
top P/S/E/T profile
child rank on P and U
singleton increment histograms
rank/Hall signatures of K_P|_T
D=2 projective parallel-class side counts
```

That sequence is not automatically bad. But it becomes bad if each new state only explains the
last counterexample while the next `D` or mixed-side case breaks it again.

The current route does not yet have:

```text
1. a fully integrated D=2 root-line table/envelope in the recurrence;
2. a proved surplus-sensitive repair codimension theorem for high D;
3. a global recurrence that counts fixed survivor sets without reintroducing loose aggregate
   overcount;
4. production parameter constants for c=8,k=2048,q=2^128,lambda=80.
```

So the route is not close to a certificate yet. It is close to a useful local theorem, not close to
the final distance theorem.

## Go Theorem

The route becomes a serious proof route if we can prove the following package.

### 1. Paired Compression

For every survivor set `S` with top paired child set `P`:

```text
rank(parent restricted to paired coordinates over P) = 2 rank_child(P).
```

This is now written as an exact construction-level lemma in:

```text
rfc_paired_compression_rank_lemma.md
```

### 2. Local Root-Line Repair Table

Given `K_P` of dimension `D` and singleton restrictions `K_P|_T`, prove a finite-field upper bound:

```text
Pr_alpha[rank root-line equations on K_P x K_P < 2D] <= LocalFail(D, signature(K_P|_T)).
```

For `D=2`, the desired signature is the projective parallel-class side-count data. For higher `D`,
the signature must not explode into the full projective configuration unless a strong counting
compression is found.

### 3. Recursive Fixed-Set Tail

For each fixed survivor set:

```text
Pr[R_d(S) < k_d]
  <= sum_D Pr[R_{d-1}(P)=k_{d-1}-D] * LocalFail(D, signature(K_P|_T)).
```

The recurrence must count or dominate the signatures of `K_P|_T` using child rank-tail data, not
empirical sampling.

### 4. Survivor-Set Union

The fixed-set bound must then union over all `S` of the target size, or use a profile grammar whose
count is explicitly bounded. This is where a good local theorem can still die: if the profile count
cost is too large, the fixed-set tail is not enough.

## No-Go Criteria

This route should be demoted if any of these happen.

1. The `D=2` parallel-class lemma is false over some finite field or side-count signature.

   This would mean even the first compact local table depends on hidden cross-ratio geometry.

2. The `D=2` lemma is true but only by a formula too case-heavy to compose.

   A long finite table is acceptable for a diagnostic. It is not acceptable for a clean distance
   certificate unless it has monotone envelopes.

3. `D=3` already requires full projective configuration data.

   If repair probabilities depend on genuine matroid/projective moduli at `D=3`, the state space
   likely becomes too large for a theorem-grade recurrence.

4. The fixed-set recurrence proves only weak tails after survivor-set union.

   If the local repair exponents are real but the number of fixed survivor sets eats all savings,
   this route may still explain the structure while failing as the certificate path.

5. The same failure pattern repeats: each stress run only produces a new invariant with no
   monotone theorem target.

   This is the spinning condition.

## Current Verdict

Current verdict: yellow, not green.

This is not a failed route yet, because the paired-compression identity and the `D=2`
parallel-class signal are genuinely sharper than previous abstractions. They expose a real local
algebraic object instead of just fitting rows.

But it is also not a straight-line proof route. We should stop expanding diagnostics unless they
answer one of two questions:

```text
Can we prove the D=2 parallel-class formula?
Does D=3 admit a comparably compact signature or a monotone upper bound?
```

If the next work does not move one of those two questions, then we are spinning.

The D=2 question should now be interpreted generously: an explicit monotone upper envelope with
finite constants is enough. Exact local probabilities are useful for diagnostics but are not
required for the distance certificate.

The first D=3 check is mixed. Exact table compression is now suspect: over `GF(5)`, mixed-side
six-row examples split even under a side-colored matroid signature. This is a warning, but not an
immediate no-go, because the certificate can use the generic determinant envelope:

```text
if a 2D-row repair minor is a nonzero polynomial,
Pr[repair failure] <= D/(q-1).
```

The next real theorem is therefore not exact D=3 probability classification. The local theorem
route is Hall/generic full rank plus a determinant envelope.

That local Hall/generic-rank theorem is now written in:

```text
rfc_rootline_generic_rank_lemma.md
```

So the remaining question is not whether Hall-OK gives a nonzero determinant. It does. The question
is whether the resulting `D/(q-1)` repair factor is strong enough after the fixed-survivor
recurrence and survivor-set counting.

The first tolerance check says no: `D/(q-1)` is thousands of bits too weak at the target scale.
Near-MDS needs a surplus-sensitive repair exponent, roughly:

```text
q^{-(|T|-2D+1)}.
```

This is recorded in:

```text
rfc_surplus_repair_codimension_target.md
```

The incidence stratification further corrects this target:

```text
codim >= |T|-2D+1-flat_excess,
flat_excess=max_{A:rank(A)<D}(|A|-2rank(A)).
```

See:

```text
rfc_surplus_incidence_stratification.md
```

Thus the current blocker is flat-excess control in the child quotient `K_P|_T`, not the root-line
generic rank lemma.

The tolerance calculation in:

```text
rfc_flat_excess_tolerance.md
```

shows this blocker is not all-or-nothing. In the top-profile model for
`c=8,k=2048,q=2^128`, bounded flat excess `F` shifts the crossing approximately from:

```text
e=71 to e=71+F.
```

So exact `e=71` needs `F=0`, but a small constant `F` still gives a near-MDS result.

The small-rank side of flat-excess control has been reframed in:

```text
rfc_small_flat_subcode_charge.md
```

A rank-`r` overloaded quotient flat gives an `(D-r)`-dimensional child subcode vanishing on
`|P|+2r+F` coordinates. In the dominant profile, the random-code first-moment exponent is roughly:

```text
-(D-r)(r+F),
```

which is enormous for small `r` and `F>0`. Thus the small-flat blocker becomes an RFC generalized
subcode-zero theorem, not a root-line theorem.

## Concrete Obstruction Shape

The clearest bad shape is a child pair deficit with just enough visible singleton positions to
repair generically, but with root-line rows arranged in few projective classes.

For `D=2`, four inactive singleton classes produce rows:

```text
(ell_i, alpha_i ell_i) in K_P^* plus K_P^*
```

The determinant condition can be written as:

```text
det rows [1, x_i, alpha_i, x_i alpha_i] = 0.
```

Equivalently, in nondegenerate coordinates:

```text
cr(x_1,x_2;x_3,x_4) = cr(alpha_1,alpha_2;alpha_3,alpha_4).
```

This is exactly where hidden projective moduli could enter. The current evidence says the special
alpha domains `F_q^*` and `F_q \ {1}` may wash out that cross-ratio dependence after counting. But
until this is proved, this is the local place where the proof can still break.

For `D>2`, the analogous obstruction is worse: inactive rows are decomposable tensors

```text
ell_i tensor (1,alpha_i),
```

and rank failure is controlled by secant/determinantal conditions in a Segre-type variety. If those
counts depend on the full projective arrangement of the `ell_i`, then a compact recurrence is
unlikely.

## Next Move

Do not add another broad profiler first.

The next useful task is one of:

```text
1. prove an RFC generalized subcode-zero bound for small-rank overloaded flats;
2. prove/charge the large-rank flat regime recursively as child rank-tail;
3. turn the one-step theorem plus corrected surplus bound into a survivor-set counting/profile
   recurrence.
```

If item 1 fails badly, the fixed-survivor route becomes a local theorem plus a likely dead global
proof for near-MDS.
