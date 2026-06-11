# RFC Distance Certificate Theorem

This note is the canonical statement for the original non-systematic RFC near-MDS distance
certificate path.

## Construction And Parameters

Use the determinant-`1` RFC fold. The clean theorem target currently assumes every root challenge:

```text
T in F^*
```

sampled uniformly and independently. Do not switch to the `T'=-T` algebra.

Implementation note: the in-repo table generation appears to sample field elements directly, so
the final certificate must either enforce the uniform-nonzero law above or insert the finite
normalization constant for uniform `T in F`. This is a construction constant, not a reason to
change the determinant-`1` fold law.

For depth `d` and expansion `c`:

```text
k = 2^d
N = c k
q = |F|
```

The original non-systematic RFC has length `N`. Systematic variants are out of scope for this push
except as an optional later comparison in the script.

## Distance Event

A nonzero codeword with at least `z` zero coordinates exists iff there is a zero set `Z`,
`|Z|=z`, such that:

```text
rank(G_Z) < k.
```

The sufficient first moment is a pair count:

```text
B_d(1,z)
  = E[# pairs (projective message line, Z) such that |Z|=z and the line vanishes on Z].
```

If:

```text
B_d(1,z) <= 2^-lambda,
```

then with probability at least `1-2^-lambda`:

```text
distance >= N - z + 1.
```

We set:

```text
z = k + e.
```

The certificate script searches for the smallest `e`.

## Local Lemma Package

The certificate currently depends on the following theorem package.

### L1. Support-Subcode Lemma

For a singleton block `S`, support `A subset S`, and child image `U <= F^S`:

```text
U_A = {u in U : supp(u) subset A}
delta(A) = dim U_A = rank(S) - rank(S \ A).
```

Any visible subspace supported in `A` lies in:

```text
U_A + U_A.
```

This proves the `tau=1` support-profile bound and the coarse Gaussian bound:

```text
# {tau-subspaces supported in A}
  <= GaussianBinomial(2 delta(A), tau)_q.
```

### L2. Root-Line Endpoint Theorem

For `U <= F^S`, define:

```text
K_A(ell) = { (x,y) in U_A + U_A : (x_j,y_j) in ell_j for every j in A }
kappa_A(ell) = dim K_A(ell).
```

Let:

```text
a     = |A|
delta = dim U_A
g     = generic dim K_A(ell)
comp  = comp(U_A).
```

The corrected proof target is the layer-codimension theorem. For:

```text
X_h(A)      = { ell in (P^1)^A : kappa_A(ell) >= h }
gamma_h(A)  = codim X_h(A) inside (P^1)^A,
```

the theorem must show:

```text
log_q E_A(2)
  <= |A| + max_{2 <= h <= delta(A)} (2h - 4 - gamma_h(A))
     + polylog_q(|A|),
```

where `E_A(2)` is the exact-support root-line count. The component/full-rank endpoint is expected
to be controlled by diagonal endomorphisms, but it is still a proof obligation until the standalone
component/full-kernel lemma is written with constants. The old full-variety component-only claim is
false for dense restrictions. A stronger two-endpoint shortcut using only `g` and `comp` is also
false in general: connected `a=5, delta=3, comp=1, g=1` rows have a codimension-one `kappa>=2`
layer. The main remaining local proof obligation is now to bound and charge these intermediate
rank-drop layers together with the full-kernel endpoint. The generic determinantal model predicts
`gamma_h >= (h-g)^2` for non-full intermediate layers, while the full-kernel layer requires the
component/full-kernel lemma `gamma_delta=a-comp`.

### L3. Tau-2 Weighted Exterior Bound

For root-line kernels:

```text
C_A(2) = sum_ell GaussianBinomial(kappa_A(ell), 2)_q.
```

Ordered bases inject into `V_2(U_A)`, so:
For exact support `A`, the root line at every coordinate is uniquely determined by the visible
2-plane. Therefore ordered bases for the exact-support count inject into `V_2(U_A)`, so:

```text
E_A(2) <= |V_2(U_A)| / |GL_2(F_q)|.
```

Assuming L2:

```text
E_A(2) * q^-|A|
  <= poly(|A|) q^theta_2(A),

theta_2(A) = max_{2 <= h <= delta(A)} (2h - 4 - gamma_h(A)).
```

### L3b. Exact-Support Incidence Grassmann Cap

Every tau-two exact-support quotient transition has an unconditional Grassmann cap that does not
depend on the genericity of `theta_2`. After fixing the child flag and kernel, let `E` be the
ambient quotient for `W/K`, with:

```text
dim E = m.
```

Before root compatibility, there are at most:

```text
[m choose 2]_q <= Gamma_q q^(2(m-2))
```

quotient two-planes. For exact visible support `A`, each coordinate in `A` accepts at most one
projective root line for a fixed two-plane, so root averaging gives:

```text
post-root quotient exponent <= 2(m-2)-|A|.
```

Equivalently, relative to the Gaussian quotient-lift factor, the local charge is at least `|A|`.
In the full-cover `K=L=0` subcase with `dim V=r`, this reads:

```text
E_{V,A}(2) q^-|A|
  <= Gamma_q q^(4r - 4 - |A|),
```

and can be combined with L2 as:

```text
E_{V,A}(2) q^-|A|
  <= poly(|A|,r) q^min(theta_2(V,A), 4r - 4 - |A|).
```

This closes the observed full-cover depth-5 neutral trace uniformly: for `r=4` and `|A|=59`, the
post-root exponent is `12 - 59 = -47`. The remaining hard rows are small-support tau-two layers,
where the `|A|` cap is weak and L2 must carry the charge.

### L3c. Rank-One Product Row: Audit Status

The smallest tau-two boundary row:

```text
|A| = 2,
delta = 2,
comp = 2,
K = L = 0,
```

has no local q-exponent slack after root averaging. The required charge is structural, not local:
the exact-support quotient splits as two rank-one components. If:

```text
A = {j_1,j_2},
```

then the two child component lines are zero on:

```text
P union (S \ A) union {j_2},
P union (S \ A) union {j_1},
```

respectively. The tempting product bound:

```text
poly(N) * F_child(1, outer_zeros + 1)^2
```

is not proof-safe by itself. The two lines live in the same child-code instance, so this multiplies
two first moments and would require an independence or negative-correlation theorem that we do not
have.

The proof-safe target is a joint marked-line child state, for example:

```text
poly(N) * (q+1) * F_child((2, outer_zeros), (1, outer_zeros + 1)),
```

or a sharper two-marked-line frame state. The `q+1` factor pays for the second line inside the
two-dimensional child span. This row is decomposable, but it is not closed by a product of first
moments.

The next boundary row:

```text
|A| = 3,
delta = 2,
comp = 1,
```

is the connected rank-two full-kernel/component endpoint. It contributes:

```text
theta_2 = comp + 2delta - 4 - |A| = -2,
```

and is not an intermediate-layer obstruction. This specific row should get a direct proof of the
component endpoint instead of relying only on the still-open general component lemma. That proof is
recorded in:

```text
docs/rfc_distance_analysis/rfc_u23_tau2_endpoint_lemma.md
```

The first local theorem row not explained by these structural reductions is:

```text
|A| = 5,
delta = 3,
comp = 1,
g = 1,
h = 2.
```

This row is now locally quantified by the `g=1` first-drop lemma:

```text
docs/rfc_distance_analysis/rfc_g1_first_drop_endpoint_lemma.md
```

It proves:

```text
gamma_2 >= 1
```

for the first-drop layer. The full row has:

```text
theta_2 = max(first-drop contribution, full-kernel contribution).
```

The first-drop contribution is `-1`. The full-kernel contribution is also at most `-1` once the
connected full-kernel/component endpoint is proved for this row. Until that endpoint is written as
a standalone lemma, the statement `theta_2=-1` for the whole row is conditional. Thus this row is
no longer an unexplained local-algebra blocker, but it still contributes two live obligations:

```text
1. close the connected full-kernel endpoint for the h=delta=3 layer;
2. control possible global chains of theta_2=-1 first-drop layers.
```

### L4. Finite-Replica Multi-Layer Flag Recurrence

The global recurrence tracks nested flag moments:

```text
F_h((t_0,z_0), ..., (t_m,z_m)),
V_0 >= V_1 >= ... >= V_m,
dim V_i = t_i,
|Z(V_i)| >= z_i.
```

The one-layer distance event is `F_d((1,k+e))`. The recurrence state records:

```text
replica span dimension t,
visible singleton dimension tau,
visible support A and |A|,
support subcode dimension delta(A),
component count comp(A),
kernel directions invisible on the singleton block.
```

Paired coordinates recurse exactly into the child. Singleton coordinates pay:

```text
root factor q^-|A|
visible-subspace count from L1/L3
recursive charge for invisible kernel directions.
```

For a parent layer `V_i`, the singleton kernel produces:

```text
pi(K_i) <= pi(V_i)
```

in the child code. The child recurrence state is the merged chain obtained by inserting all such
kernels into the projected parent flag. The transition is proof-safe only after conditioning on one
depth-`h-1` child code and counting one merged child flag. In particular, the recurrence must not
replace a joint child event by:

```text
F_child(upper event) * F_child(lower event)
```

unless those events live over independent child-code randomness, which they do not inside one fold.
The current target theorem is:

```text
docs/rfc_distance_analysis/rfc_multilayer_flag_transition_theorem.md
```

That theorem must dominate the exact first moment `B_d(1,z)`, include the dominant layer value
`theta_2(A)` or an equivalent charge for intermediate rank-drop layers, and apply the
exact-support incidence cap inside the represented quotient `pi(V_i)/pi(K_i)` before spending any
quotient-lift factor.

The L4 recurrence also needs an explicit flag-gap routing rule. In a kernel-following chain, an
ancestor `V_i` above a lower child flag `V_{i+1}` is not chosen inside the full child message space;
it is chosen inside the shortened ambient:

```text
H(B_i) = {messages vanishing on the child zero witness B_i}.
```

For fixed `B_i`, the lift factor is therefore:

```text
GaussianBinomial(dim H(B_i)-t_{i+1}, t_i-t_{i+1})_q.
```

If `dim H(B_i)` is near the MDS value `max(k_child-|B_i|,0)`, this shortened-ambient factor is the
normal branch and should be paid directly by the accumulated local zero requests. If `dim H(B_i)`
is larger, the transition must expose the enlarged shortened ambient as a recursive child
shortened-kernel rank event:

```text
R_{h-1}(dim H(B_i), |B_i|)
  = Pr_child[dim H(B_i) is at least this large].
```

The rank-tail codimension

```text
dim H(B_i) * (|B_i| - k_child + dim H(B_i))
```

is only the generic-rank calibration target for this exposed child rank event. The actual theorem
must use the RFC shortened-kernel rank recurrence:

```text
rho_{h-1}(D,z) = -log_q Pr[dim H_{h-1}(B) >= D].
```

A raw child flag moment `F_{h-1}((D,|B_i|))` is too loose for this purpose because it counts all
`D`-subspaces inside the shortened ambient; the kernel-chain recurrence already counts ancestor
flags inside `H(B_i)` by the Gaussian shortened-ambient factor above. The exact all-paired branch
of `rho` compresses to the child by `D -> ceil(D/2)`, so exposed defects cannot be certified by
blindly plugging in the generic rank-tail exponent.

## Conditional Certificate Bound

Assuming L1-L4 and absorbing theorem constants into a factor `C(d,N)`:

```text
B_d(1,k+e)
  <= C(d,N) * binom(N,k+e) * q^-(e+1)
```

for the original RFC.

Therefore the certificate script searches for the smallest `e` satisfying:

```text
log2 binom(N,k+e) + log2 C(d,N) - (e+1) log2(q) <= -lambda.
```

The script still has an optional `systematic_all_levels` mode for later comparison, but the default
and this theorem statement are original/non-systematic only.

## Output Contract

The driver:

```text
scripts/rfc_distance_analysis/rfc_distance_certificate.py
```

emits a final-shape conditional calculation as CSV to stdout and optional JSON/CSV files containing:

```text
config,
conditional status,
bound model,
certified excess e,
distance lower bound,
relative distance,
gap to MDS,
slack bits,
theorem assumptions used,
paired-compression trace.
```

The driver does not implement the full recurrence state from L4. It evaluates the final theorem
shape after L1-L4 are assumed and reports that status explicitly. The output is therefore an
idealized conditional certificate until L2, the conditional full-kernel endpoint in L3, L4, and the
polynomial factor `C(d,N)` are completed.

## Default Target

For:

```text
c = 8
k = 2048
q = 2^128
lambda = 80
```

with `log2 C(d,N)=0`, the expected crossing is:

```text
e = 71.
```

The all-paired branch is compatible with the same relative gap because both the effective dimension
and zero-set size compress by the same factor.

Secondary idealized crossings from the same driver:

```text
c=4, k=2048, q=2^128: e=53
c=8, k=2048, q=2^256: e=35
```

Systematic adaptation is deliberately deferred. When we return to it, systematic coordinates must be
included as real zero-request coordinates in the recurrence, not handled by discounting a prefix.
