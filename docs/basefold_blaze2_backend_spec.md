# Spec: Paper-Correct BaseFold Core And Blaze2 Backend

This document is the design contract for adapting BaseFold into the real Blaze2 backend.

Sources:

- BaseFold: <https://eprint.iacr.org/2023/1705.pdf>
- Blaze: <https://eprint.iacr.org/2024/1609.pdf>
- Local accounting: `docs/proof_size_accounting.md`
- Local rewrite target: `docs/blaze_rewrite.md`

## Philosophy

This project uses spec correctness first.

Correctness does not mean "the tests pass for a shape we know is temporary." Correctness means the
component implements the protocol shape, transcript binding, proof contents, and proof-size family
specified by the paper-level design.

We do not intentionally write implementation paths that we already know must be deleted later. If a
component is too large to implement in one step, split it into smaller components whose boundaries
are themselves paper-correct and testable. A partial component is acceptable only when it is a true
prefix or isolated primitive of the final protocol, not a stopgap protocol.

No extra proof objects are allowed. If a value is public parameter material, it is not proof data.
If a value can be derived from a compact public seed, the transcript may bind the seed and shape;
it should not absorb or serialize the expanded tables. Large public objects such as RAA
permutation tables, FFT tables, and encoded-code descriptions must not be hashed into every proof
unless the protocol explicitly requires that exact large object as communication.

Performance is part of correctness for this layer. The implementation must preserve the paper
asymptotics and the intended concrete proof-size family. It should avoid dynamic dispatch, type
erasure, heap-allocating callbacks, and generic runtime provider objects in hot paths. Shared code
should be monomorphized through concrete functions or compile-time generic parameters.

## Goal

Implement a low-level BaseFold core that is systematic/holographic by construction and can support
both:

1. Normal standalone BaseFold PCS openings.
2. Blaze2 openings where the RMLE/PRAA IOPP core asks for input-oracle positions and Blaze
   authenticates exactly those positions from the interleaved commitment.

The old monolithic BaseFold implementation is not a target architecture. It may remain temporarily
only as a frozen regression/A-B reference while the new core is built. The maintained BaseFold API
should become a wrapper around the systematic/holographic core. Blaze2 should call the same core
through a different monomorphized entrypoint. The two paths may share folding, sumcheck,
lower-layer opening, and verifier logic, but they must not force each other into the wrong
input-oracle policy.

## Phase 1: BaseFold Overhaul

Phase 1 is to overhaul BaseFold around systematic random foldable codes. This is not a Blaze-only
adapter. The BaseFold core contract is:

```text
input oracle y in F^k
systematic random foldable code C_sys : F^k -> F^N
C_sys(y) = (y || parity(y))
```

`C_sys` is the concrete foldable code passed to BaseFold. It must be systematic, and its systematic
layout must be verifier-known. This is a structural requirement on the BaseFold code object, not a
distributional requirement on `y`. The witness/input `y` may be arbitrary; the protocol still checks
proximity/evaluation against the virtual codeword `C_sys(y)`.

The conservative Phase 1 construction is **power-of-two systematic-augmented RFC**:

```text
E_i      : F^k_i -> F^n_i       paper random foldable code at layer i
C_sys_i : F^k_i -> F^(k_i+n_i)
C_sys_i(m) = (m, E_i(m))
n_i / k_i = c_parity = 2^r - 1
(k_i + n_i) / k_i = c_sys = 2^r
```

The notation `C_sys_i(m) = (m, E_i(m))` describes the logical address space. It does not require a
contiguous physical Merkle/tree layout. The implementation should use a fold-compatible recursive
physical layout and expose a deterministic address map:

```text
logical systematic index j  <-> physical codeword address a_j
logical parity index j      <-> physical codeword address b_j
```

`TopQuery.index` is a logical systematic index into the external oracle `y`; the backend code object
maps it to whatever physical address the fold/query schedule uses. This avoids forcing a contiguous
prefix layout that fights BaseFold's half-codeword pairing.

The systematic part is the identity copy of the message/oracle table in the logical address space.
The parity part is the normal RFC encoding. This is not an arbitrary systematic row reduction of the
RFC generator matrix; that could destroy the recursive foldable structure and low-overhead
encoding.

For the RFC/parity block, keep the project convention:

```text
T'[j] = T[j] + 1
```

This remains true over binary fields and is the fold rule the existing BaseFold code path is built
around.

For a raw systematic pair `(m_l[j], m_r[j])`, the finite points `T_sys = 0`, `T_sys' = 1` fold to:

```text
(1 - alpha) * m_l[j] + alpha * m_r[j]
```

not `m_l[j] + alpha * m_r[j]`. Therefore the spec must not silently reuse the old linear message
fold equation for raw systematic coordinates. There are only two acceptable implementation choices:

```text
1. typed affine fold for logical systematic coordinates:
   (m_l[j], m_r[j]) -> (1 - alpha) * m_l[j] + alpha * m_r[j]

2. transformed systematic coordinates compatible with the old linear fold:
   (m_l[j], m_l[j] + m_r[j]) -> m_l[j] + alpha * m_r[j]
```

Choice 2 is not directly Blaze-friendly because the second coordinate is not a raw external oracle
entry. For Blaze, use choice 1 unless a later same-rate systematic RFC construction proves a better
layout. The RFC/parity block still uses the sampled RFC diagonal schedule with `T' = T + 1`.
With choice 1, the parity pair for child encodings `L = P(m_l)` and `R = P(m_r)` must interpolate
between `L` and `R`:

```text
(L[j] + T[j] * (R[j] - L[j]), L[j] + (T[j] + 1) * (R[j] - L[j]))
```

not between `L` and a raw `R` direction. This is what makes the parity fold land on
`P((1-alpha) * m_l + alpha * m_r)`.
The power-of-two restriction below avoids non-power-of-two codeword plumbing; it does not license
the old linear fold equation for raw systematic coordinates.

Distance and query counts must use the augmented code, not the old RFC alone. The conservative
parameter claim is exactly this:

```text
parity RFC:
  E_i : F^k_i -> F^n_i
  c_parity = n_i / k_i
  rate_parity = k_i / n_i = 1 / c_parity
  relative distance >= delta_rfc

systematic-augmented code:
  C_sys_i : F^k_i -> F^(k_i + n_i)
  C_sys_i(m) = (m, E_i(m))
  c_sys = (k_i + n_i) / k_i = 1 + c_parity
  rate_sys = k_i / (k_i + n_i) = 1 / (1 + c_parity)
```

For every nonzero `m`, the systematic identity block can only help absolute Hamming weight:

```text
wt(C_sys_i(m)) = wt(m) + wt(E_i(m)) >= wt(E_i(m))
```

Therefore the conservative relative-distance claim is:

```text
delta_sys >= (c_parity / (1 + c_parity)) * delta_rfc
```

Because Phase 1 chooses the power-of-two implementation path, allowed conservative parity
expansions are:

```text
c_parity in {1, 3, 7, 15, ...}
c_sys = 1 + c_parity in {2, 4, 8, 16, ...}
```

Example: if the parity RFC has expansion `c_parity = 3`, then the final systematic-augmented code
has `rate_sys = 1/4` and `delta_sys >= (3/4) * delta_rfc`. If the parity RFC has expansion
`c_parity = 7`, then the final code has `rate_sys = 1/8` and
`delta_sys >= (7/8) * delta_rfc`.

This distance penalty is real and must be reflected in BaseFold/backend query counts. The benefit is
that parity encoding is exactly the RFC encoder:

```text
encode_parity(y, out) = E_i(y)
```

so it keeps the RFC encoding cost and writes into caller-owned `out`. If this distance/profile is
not acceptable for a parameter set, the response is to choose a different explicitly specified
systematic foldable code, not to use an unspecified systematic transform.

The code object must expose enough information for the prover and verifier to classify every query
without runtime-erased callbacks:

```rust
pub enum CodewordPart {
    Systematic,
    Parity,
}

pub struct CodewordAddress {
    pub part: CodewordPart,
    pub local_index: usize,
}

pub struct FoldPair {
    pub left: usize,
    pub right: usize,
    pub out: usize,
}

pub struct CodeLayout {
    pub compiler_message_len: usize,
    pub compiler_parity_len: usize,
    pub compiler_codeword_len: usize,
    pub num_rounds: usize,
}

pub struct SystematicFoldableCodeSpec {
    pub version: u32,
    pub field_id: FieldId,
    pub compiler_message_len: usize,
    pub compiler_systematic_len: usize,
    pub compiler_parity_len: usize,
    pub compiler_codeword_len: usize,
    pub parity_expansion_factor: usize,
    pub seed: CodeSeed,
    pub layout: CodeLayout,
    pub folding_schedule: FoldingScheduleSpec,
}
```

The concrete code implementation must provide these operations in monomorphized, allocation-aware
form:

```rust
message_len()
codeword_len()
systematic_len()
parity_len()
physical_to_logical(physical_index) -> CodewordAddress
systematic_to_physical(logical_index) -> usize
parity_to_physical(logical_index) -> usize
fold_pair(round, output_index) -> FoldPair
encode_parity(input, parity_out)
folding_schedule()
```

`encode_parity(input, parity_out)` writes into caller-owned storage. It must not allocate a full
temporary codeword in hot paths unless the caller explicitly chose that representation. If a
particular foldable code cannot be made systematic without breaking the BaseFold algebra or
performance target, it is not accepted as the Phase 1 BaseFold code.

`CodeLayout` owns physical address mapping and fold-pair locality. `folding_schedule()` owns the
algebraic fold parameters such as `T`, `T'`, and the typed affine-vs-finite fold rule. Keep this
split explicit: do not hide physical address mapping behind a runtime-erased oracle provider or a
callback in the hot path. Implementations may precompute compact layout tables when that is faster,
but the tables are public parameters derived from the compact code spec and are not proof data.

The old standalone BaseFold API should be reimplemented as:

```text
commit:
  commit/authenticate the systematic input part y
  compute and commit/authenticate parity(y) and backend proof oracles

open:
  run the shared query schedule
  answer systematic queries from the standalone systematic commitment
  answer parity/proof-oracle queries from backend proof commitments
```

Blaze uses the same core, but supplies the systematic `y = c_star` authentication through
interleaved column openings rather than through a standalone BaseFold top commitment.

## Blaze2 Application

Blaze commits to an interleaved packed RAA codeword. Let:

```text
m in F^(t x k)
c = PRAA(m) in F^(t x n_code)
z = (z_row, z_col)
z_row in F^log(t)
z_col in F^log(k)
```

Here `k` is the per-row message length, `n_code` is the per-row RAA codeword length, and `t` is
the interleaving width/number of rows. In the Blaze paper's Section 8.2 notation, `n = t *
n_code`; the outer Merkle commitment has `n / t = n_code` leaves, one leaf per interleaved column.
The total committed field-element matrix has `t * n_code` elements.

The prover claims the multilinear evaluation:

```text
m(z_row, z_col) = y
```

Blaze first sends the row-evaluation vector:

```text
u_i = m_i(z_col), for i in 0..t
```

The verifier checks:

```text
u(z_row) = y
```

After `u` is bound, the transcript samples folding challenges:

```text
rho in F^t
m_star = sum_i rho_i m_i
c_star = sum_i rho_i c_i
```

Because packed RAA is linear:

```text
c_star = PRAA(m_star)
```

The backend must prove the folded relation:

```text
m_star(z_col) = <rho, u>
c_star = PRAA(m_star)
```

Blaze supplies oracle access to `c_star` at the input positions requested by the inner IOPP. Each
input query is a single column query:

```text
index in [0, n_code)
```

Blaze authenticates one interleaved column `c[:, index]`, then computes:

```text
c_star[index] = sum_i rho_i c_i[index]
```

The backend receives exactly those indices and values in its query phase. The Blaze-facing
input-oracle boundary is a list of authenticated single-position values. If a backend equation needs
two input-oracle positions, both positions must appear as two entries in that list and both consume
the Blaze column-opening budget.

Paper interpretation: the column query above is an **input query** to the IOPP for
`RMLE[PRAA^t]`. Lemma 6.1 lifts one input query to the underlying `RMLE[PRAA]` IOPP into one
query over alphabet `F^t`, i.e. one `t`-length interleaved column. This is distinct from the
BaseFold/FRI-style **proof queries** made to proof oracles inside the compiled IOPP. Proof-oracle
queries may have sibling/folding structure, but those bytes are backend proof bytes, not additional
openings of the Blaze input commitment.

BaseFold Protocol 3 is the concrete trap to avoid. Its raw folding query samples one position in a
half-size domain, then reads the pair:

```text
pi_{i+1}[mu], pi_{i+1}[mu + n_i]
```

For a standalone BaseFold PCS, those are two top-oracle reads in that round. For Blaze, if the top
oracle being read is the interleaved `c_star` input oracle, those two reads are two Blaze
interleaved-column openings. They are not one query plus a free sibling. The paper-compatible
counter is therefore total input symbols opened from `c_star`, not the number of binary-fold query
rounds. This is how the BaseFold pair check connects to Blaze Section 8.2's `1059` columns.

The deeper connector is Lemma 4.2. Blaze does not use the standalone BaseFold PCS commitment to
`c_star` directly. It uses a holographic MLIOP-to-IOPP compiler with a separate systematic
encoding code, call it `C_sys`. This is the code whose `RMLE[C_sys]` IOPP is supplied by
BaseFold/FRI, e.g. a systematic Reed-Solomon or other systematic foldable code. This is not PRAA.
PRAA/RAA is the relation code being checked and is not systematic as stated in the Blaze paper.
For the implicit input `y = c_star`, the compiler prover sends only the non-systematic part of
`C_sys(y)`:

```text
C_sys(y) = (y || y_tilde)
```

The verifier's oracle access to `C_sys(y)` is therefore split:

```text
compiler-systematic positions      -> read `y = c_star`, authenticated by Blaze column openings
compiler-non-systematic positions  -> read `y_tilde`, authenticated inside the backend proof
```

This is why a BaseFold/FRI pair-shaped query does not automatically double the Blaze outer proof.
Only queried positions in the compiler-systematic copy of the implicit input consume the `Q_RAA`
interleaved-column budget. Compiler-non-systematic or auxiliary proof-oracle positions are part of
the BaseFold/backend proof term. An implementation that treats the entire top BaseFold oracle as
`c_star` has already lost the paper accounting.

## Required Backend Boundary

The backend should be index-native and holographic. The inner RMLE/PRAA IOPP determines its query
schedule after all prequery commitments/messages are bound. Blaze authenticates the subset of those
queries that touch the compiler-systematic copy of the input oracle `c_star`.

Use separate prover and verifier requests. The prover request includes the folded-message witness
needed to construct MLIOP/backend proof oracles. The verifier request contains only public claims,
authenticated input queries, and compact public context. The backend proof is passed as a separate
argument to verification, not embedded in the verifier request.

```rust
pub struct TopQuery<T> {
    pub index: usize,
    pub value: T,
}

pub struct Blaze2BackendProverRequest<'a> {
    pub code: &'a Blaze2Code,
    pub folded_message: &'a [B128],
    pub point: &'a [B128],
    pub folded_eval: B128,
    pub top_queries: &'a [TopQuery<B128>],
}

pub struct Blaze2BackendVerifierRequest<'a> {
    pub code: &'a Blaze2Code,
    pub point: &'a [B128],
    pub folded_eval: B128,
    pub top_queries: &'a [TopQuery<B128>],
}
```

Each top query means:

```text
PRAA(m_star)[index] = value
```

`folded_eval` means `<rho, u> = m_star(point)`.

The request boundary above is the query-phase boundary, not the full prequery state. The backend
also has prequery proof material for the non-systematic part of `C_sys(c_star)` and for auxiliary
MLIOP oracles. Those objects are backend proof communication. They are not extra Blaze input
columns.

The backend may borrow an expanded code object for performance. The expanded object is public
parameter material, not proof data. The transcript should bind the compact code spec:

```rust
pub struct Blaze2CodeSpec {
    pub version: u32,
    pub field_id: FieldId,
    pub hash_id: HashId,
    pub raa_variant: RaaVariant,
    pub packing: Blaze2PackingLayout,
    pub leaf_layout: Blaze2LeafLayout,
    pub praa_message_len: usize,
    pub praa_expansion_factor: usize,
    pub praa_codeword_len: usize,
    pub seed: Blaze2CodeSeed,
}

pub struct Blaze2BaseFoldBackendSpec {
    pub praa: Blaze2CodeSpec,
    pub compiler_code: SystematicFoldableCodeSpec,
    pub q_raa_input: usize,
    pub q_backend_proof: usize,
    pub auxiliary_oracle_len: usize,
}

pub struct Blaze2Code {
    pub spec: Blaze2CodeSpec,
    pub packed: PackedRaaCode,
}
```

`Blaze2Code` must be deterministically constructible from `Blaze2CodeSpec`, so the verifier is not
trusting arbitrary expanded permutations while hashing only a seed. The backend compiler code must
be deterministically constructible from `SystematicFoldableCodeSpec` for the same reason.

`praa_expansion_factor` means `praa_codeword_len / praa_message_len`. It is not the reciprocal
coding-theory rate. The constructor must reject the spec unless all of these equalities hold:

```text
praa_message_len * praa_expansion_factor == praa_codeword_len
compiler_code.compiler_message_len == praa_codeword_len
compiler_code.compiler_systematic_len == praa_codeword_len
compiler_code.compiler_systematic_len + compiler_code.compiler_parity_len
    == compiler_code.compiler_codeword_len
compiler_code.compiler_parity_len
    == compiler_code.compiler_message_len * compiler_code.parity_expansion_factor
compiler_code.parity_expansion_factor + 1 is a power of two
auxiliary_oracle_len == praa_codeword_len * backend_auxiliary_row_count
```

For the current Blaze2 BaseFold backend, `backend_auxiliary_row_count = 3`: the three PRAA relation
rows `u2`, `u3`, and `u4`. `folded_eval` is bound by the global eval-binding sumcheck, not by an
extra request-dependent auxiliary row. `auxiliary_oracle_len = 0` is not an accepted Blaze2 backend
configuration, because it skips the folded-relation checks. A standalone/generic scheduler may still
model a no-auxiliary query domain for isolated tests, but the Blaze2 backend constructor must reject
it.

For the initial transparent Blaze2 path, `praa_message_len`, `praa_codeword_len`,
`compiler_message_len`, `compiler_codeword_len`, and `t` must be non-zero powers of two unless an
explicit padding/layout spec says otherwise. `point.len()` must be `log2(praa_message_len)` for the
PRAA message claim, and the compiler RMLE point for `c_star` lives in
`F^log2(compiler_message_len)`. Non-power-of-two inputs require an explicit padding/layout spec
with its own proof accounting; they are not implicit.

Backend-specific BaseFold parameters, such as folding schedule, base-code choice, and final clear
oracle size, belong in a separate backend parameter spec that is also transcript-bound by compact
identifiers and scalar parameters.

This spec is initially transparent-only. Hiding/zero-knowledge variants must get their own proof
accounting before entering this path; they should not be folded into the transparent backend API as
optional proof ballast.

## Query Canonicalization

Query indices are transcript ordered. Do not sort them. Duplicates are allowed unless a future
security proof and accounting model explicitly introduces deduplication.

Every full backend query schedule entry must satisfy:

```text
index < compiler_codeword_len
```

Every Blaze-supplied input query must be a schedule entry classified as a compiler-systematic
`c_star` position:

```text
top_query.index < compiler_systematic_len
top_queries.len() == number_of_systematic_c_star_entries_in_schedule
```

Sampling should be unbiased. If the query domain size is a power of two, masking is acceptable. For
non-power-of-two domains, use rejection sampling from transcript bytes/field elements. Do not use a
small fixed-width modulo conversion such as `u32 % domain`.

`q_raa_input` means the exact number of authenticated input-oracle symbols opened from the Blaze
interleaved commitment. If an internal BaseFold/RMLE query equation needs two compiler-systematic
input symbols, both symbols must appear as separate `TopQuery` entries and both count against
`q_raa_input`.

The query schedule must sample typed query sets with fixed configured counts:

```text
input_queries.len() == q_raa_input
proof_queries.len() == q_backend_proof
```

At least one backend proof query must be in the compiler-parity domain, so every accepted Blaze2
BaseFold proof includes a parity fold path tying the clear terminal base-code word back to committed
folded parity layers. The current schedule uses the first backend proof query for this parity
fold-chain guard, then samples the remaining backend proof queries over the parity and auxiliary
proof-oracle domains.

The eval-binding path must include a global degree-2 sumcheck for:

```text
sum_i eval_weight[i] * c_star[i] == folded_eval
```

where `eval_weight` is verifier-derived from the PRAA code and the multilinear opening point. The
round polynomials are backend prequery messages. Their Fiat-Shamir challenges are also the
systematic fold-chain challenges, so the sumcheck terminal check uses the clear terminal systematic
symbol of `C_sys(c_star)` as `c_star(r)`. The first RAA final-accumulator spot is fixed to the
terminal transition `(n_praa - 2, n_praa - 1)` as a boundary guard for the `u4` relation; the
remaining RAA final spots are sampled by the transcript. This still consumes exactly two
`input_queries` per final-accumulator spot and preserves the configured `q_raa_input`.

Do not sample one untyped list from the full compiler codeword and then hope the number of
systematic hits is right. If a protocol step needs `q_raa_input` systematic input checks, sample
from the systematic domain directly. If it needs parity/proof-oracle checks, sample those from the
parity/proof domains with their own configured counts. Rejection/extension loops are not acceptable
for the normal prover path because they make proof size and transcript timing harder to reason
about.

Do not conflate input-query count with proof-query count. The Blaze paper's `q_RAA = 1059` is the
input-query count that determines the extra interleaved-column payload and outer Merkle paths.
Backend proof-query complexity, such as BaseFold/FRI folded-oracle queries, is counted inside the
backend proof term.

## BaseFold Core Split

The current high-level BaseFold PCS API owns too much protocol policy. It commits, samples query
indices, serializes proof bytes through transcript read/write APIs, opens the top oracle, and
verifies everything in one path. That path is not the object described by Blaze Section 6/7.

The reusable object is not that high-level PCS API. For Blaze, the reusable backend object is the
IOPP for `RMLE[PRAA]` obtained by the paper's Section 5 MLIOP plus Section 4.1 MLIOP-to-IOPP
compiler, using BaseFold/FRI-style machinery for the proof oracles. Given an RMLE claim and input
oracle access to `y = PRAA(m)`, it proves that `m(z) = v` and that the supplied input-oracle values
are consistent with the protocol.

The new inner core should therefore separate RMLE input-oracle queries from backend proof-oracle
queries. The core may have its own internal folded proof-oracle queries and sibling data, but the
external input-oracle boundary must be an explicit list of single-position reads:

```rust
pub struct InputQuery<F> {
    pub index: usize,
    pub value: F,
}
```

For standalone BaseFold, a PCS wrapper obtains `InputQuery` values by opening its own top Merkle
commitment. For Blaze2, Blaze obtains `InputQuery` values by opening interleaved columns and folding
them into `c_star[index]`. The core must not require implicit sibling input-oracle values. If the
core needs both halves of a BaseFold pair and both halves are compiler-systematic `c_star`
positions, it must request two explicit `InputQuery` entries and the Blaze proof must open two
interleaved columns. If one half is a compiler-non-systematic `y_tilde` or auxiliary proof-oracle
position, that half is authenticated inside the backend proof instead.

The implementation should expose explicit phases:

```rust
prove_prequery(...) -> (prequery_proof, prover_state)
sample_oracle_queries(transcript, prover_state) -> QuerySchedule
prove_queries(prover_state, input_queries) -> query_proof
verify(preproof, input_queries, query_proof) -> Result<(), Error>
```

This phase split is part of the spec. It prevents sampling query indices before the commitments and
messages they check have been bound, and it gives Blaze one clean place to supply externally
authenticated values for the input-oracle positions requested by the backend.

The single-position boundary does not mean that one input symbol magically replaces a binary fold
equation that needs two input symbols. It means the core declares every input-oracle symbol it
needs as an explicit single-position query:

```text
input_queries = [
    InputQuery { index: j0, value: y[j0] },
    InputQuery { index: j1, value: y[j1] },
    ...
]
```

The verifier then checks the core's query equations using only values present in this list. For
example, if an input-oracle equation genuinely needs `y[j0]` and `y[j1]`, then both `j0` and `j1`
must be in `input_queries`, and the Blaze proof must open both corresponding interleaved columns.
There is no unauthenticated sibling lookup and no sibling value hidden inside the backend proof. In
the paper construction, the `q_RAA` interleaved-column queries are the total input-oracle symbols.
Sibling queries that arise from BaseFold/FRI folding of backend proof oracles are a different class
of query and must be serialized/accounted inside the backend proof, not as hidden input queries.

### Shared Core Responsibilities

The RMLE/PRAA backend core handles:

1. The systematic random foldable code layout for `C_sys(y) = (y || parity(y))`.
2. Query schedule generation and classification into systematic, parity, and auxiliary proof-oracle
   entries.
3. The MLIOP checks for the PRAA relation: message evaluation, repetition/permutation, and
   accumulation constraints.
4. The MLIOP-to-IOPP batching that reduces multilinear evaluation claims to backend proof-oracle
   checks.
5. BaseFold/FRI-style commitments and queries for those backend proof oracles.
6. Verification against the explicit `InputQuery` values supplied by the caller.
7. Final/base-code checks required by the backend proof.

These pieces are shared by standalone BaseFold and Blaze2.

### Standalone BaseFold Wrapper

The standalone PCS wrapper handles:

1. Commit/authenticate the systematic input vector `y`.
2. Compute `parity(y)` with the systematic random foldable code and commit/authenticate parity and
   backend proof oracles.
3. Bind all public roots and backend prequery messages.
4. Sample the shared query schedule after all objects being queried are bound.
5. Open systematic entries from the standalone systematic commitment.
6. Open parity and auxiliary entries from backend proof commitments.
7. Call the inner BaseFold core with the same classified query schedule.

This wrapper may preserve a convenient public commit/open API, but it must not preserve the old
monolithic proof shape internally. The implementation should delegate to the systematic/holographic
core instead of duplicating folding logic.

### Blaze2 BaseFold Backend

The Blaze2 backend handles:

1. Emit backend prequery commitments required by the RMLE IOPP core, such as roots for the
   non-systematic part of `C_sys(c_star)`, folded-layer roots, auxiliary-oracle roots, or sumcheck
   messages.
2. Emit the eval-binding sumcheck messages before folded-layer roots when those sumcheck
   challenges drive the fold chain.
3. Sample the full query schedule after those objects are bound.
4. Ask Blaze2 to authenticate the schedule entries that touch the compiler-systematic input copy
   `c_star`.
5. Authenticate compiler-non-systematic and auxiliary proof-oracle entries inside the backend
   proof.
6. Call the inner RMLE/PRAA IOPP core for the folded relation using the same explicit query
   schedule.

`backend_prequery_commitments` must not be a PCS commitment to the full folded codeword `c_star`,
and must not be a commitment whose only purpose is to reopen `c_star[index]`. It is the compact set
of internal proof commitments/messages that the BaseFold/RMLE core needs before query sampling.

The Blaze2 backend must not require hidden sibling input-oracle values from Blaze. The current
code's binary-pair compact query convention is an implementation detail of the standalone PCS path.
If that convention is reused, every top-pair element that belongs to the compiler-systematic
`c_star` copy must be represented as an explicit `InputQuery` and counted as a Blaze column
opening. Compiler-non-systematic or auxiliary top-pair elements must be authenticated by the
backend proof.

The Blaze2 backend must not add a second commitment to the full folded codeword if that commitment
only exists to reopen the same top values. That would recreate the old proof-size failure.

## Monomorphized Implementation Rule

There must be no `dyn` provider objects, virtual dispatch, boxed callbacks, or runtime-erased
oracle interfaces in this hot path.

Use one of these shapes:

```rust
fn prove_standalone_basefold_core<F, H, V>(...) -> Result<BaseFoldProof<F, H>, Error>;

fn prove_blaze2_basefold_core<H, V>(...) -> Result<Blaze2BaseFoldProof<H>, Error>;
```

or a generic compile-time policy:

```rust
fn prove_basefold_core<F, H, V, P>(..., policy: P) -> Result<BaseFoldCoreProof<F, H>, Error>
where
    P: TopLayerPolicy<F, H>;
```

The policy form is acceptable only if it monomorphizes and does not allocate or dispatch in inner
loops. Prefer two thin concrete entrypoints when that keeps generated code clearer.

## Proof-Size Requirements

### BaseFold Core

A paper-shaped BaseFold proof contains:

```text
sumcheck messages
systematic input root(s), when the standalone wrapper owns them
parity/proof-oracle roots below the public systematic input oracle
final/base oracle material
query values with carried-value reuse
Merkle paths for authenticated parity and proof-oracle layers
```

It must not contain:

```text
a fresh full-size commitment per query
duplicated top values that are already carried
duplicated Merkle paths for values authenticated by another layer
systematic input values re-authenticated by parity/proof commitments
full oracle vectors except for the intended final/base oracle
```

For one opening, the old monolithic reference accounting is:

```text
basefold_bytes ~= Q_BF * sum_i log2(N_i) * hash_bytes
               + Q_BF * (D + 1) * field_bytes
               + 3 * D * field_bytes
               + folded_roots
               + final_codeword
```

This formula is useful as an A/B upper/reference model for the current code, but it is not the
target holographic BaseFold proof shape. For the target shape, account over typed query schedules:

```text
holographic_basefold_bytes =
    backend_roots
  + final_codeword
  + sumcheck_bytes
  + systematic_opening_bytes_owned_by_wrapper
  + parity_and_aux_query_values
  + parity_and_aux_merkle_paths
```

For Blaze2, `systematic_opening_bytes_owned_by_wrapper` is zero inside the backend term because
systematic `c_star` values are supplied by Blaze interleaved column openings. The backend proof must
not re-add those bytes under another name.

The backend proof may include internal folded-layer sibling values when those are part of the
BaseFold/FRI-style proof-oracle proximity check. It must not ask Blaze for hidden or unaccounted
sibling values of the original input oracle `c_star`; every compiler-systematic `c_star` value
requested by the query schedule consumes one Blaze column opening. It must not serialize a second
authentication path for `c_star[index]`.

### Blaze2 Full Proof

The Blaze2 proof consists of:

```text
row evaluation vector u
one backend proof for the folded relation
Q_RAA interleaved columns from the outer commitment
Q_RAA Merkle paths for those columns
small scalar/root overhead
```

For `B128` and 32-byte hashes, the Blaze-specific outer proof budget before backend proof is:

```text
blaze2_outer_bytes =
    t * 16
  + backend_commitment_bytes
  + Q_RAA * (t * 16 + log2(n_code) * 32)
```

where:

`folded_eval = <rho, u>` is verifier-derived after `u` is bound and `rho` is squeezed. It is an API
input to the backend verifier, not serialized proof data, and contributes zero bytes unless a
future backend has a separately justified protocol reason to send it. `backend_commitment_bytes`
means backend prequery commitments/messages that are actual proof communication.

This is the required Blaze paper budget: Section 8.2 gives the additional field payload as
`Q_RAA * t * field_bytes` and says there are `Q_RAA` Merkle paths for a tree with `n/t = n_code`
leaves. Here `Q_RAA` is the total number of input-oracle column openings. The backend proof term
may include BaseFold/FRI proof-oracle query data, but those are not openings of the interleaved
input commitment and must already be included in the holographic BaseFold backend term.

More explicitly for the holographic compiler:

```text
PRAA message length             = praa_message_len = k
PRAA/input oracle length        = praa_codeword_len = n_code
compiler message/systematic len = compiler_message_len = compiler_systematic_len = n_code
compiler parity length          = compiler_parity_len
compiler codeword length        = n_code + compiler_parity_len
```

The Blaze outer term depends on `praa_codeword_len` because that is the number of leaves in the
interleaved PRAA commitment. The backend BaseFold term depends on `compiler_codeword_len` and on
the distance/query parameters of `C_sys`, including the systematic-augmentation distance penalty.

The full Blaze2 proof target is:

```text
blaze2_bytes =
    blaze2_outer_bytes
  + basefold_backend_bytes(
        compiler_message_len = n_code,
        compiler_codeword_len = n_code + compiler_parity_len
    )
```

The implementation is not paper-correct if it includes terms like:

```text
Q_RAA * t * log(n/t) field elements
Q_RAA * number_of_auxiliary_polynomials * Merkle_path_length
Q_RAA * n
full row-combination vector
BaseFold batch openings for RAA helper polynomials
separate folded-codeword commitment plus openings duplicating Blaze column openings
```

## Transcript Requirements

The transcript must bind all public protocol choices needed for soundness:

```text
domain separator
field/code/backend identifiers
systematic random foldable code spec
message length
expansion factor and codeword length
systematic length and layout
code seed
number of rows t
number of queries
public commitment roots
claimed point and value
row evaluation vector u
backend commitment
derived folded evaluation
BaseFold folded-layer roots
BaseFold sumcheck messages
```

The logical Fiat-Shamir order for Blaze2 with a BaseFold backend is:

1. Absorb domain separators, field/hash/backend identifiers, compact code spec, backend parameter
   spec, systematic foldable compiler-code spec, outer interleaved commitment root,
   claim point/value, `q_raa_input`, and `q_backend_proof`.
2. Absorb the row-evaluation vector `u`.
3. Squeeze `rho`.
4. Derive `folded_eval = <rho, u>`, then absorb the backend prequery commitments/messages that
   determine the oracles to be queried, including commitments to non-systematic encodings and
   auxiliary proof oracles. These commitments/messages are not a fresh PCS commitment whose purpose
   is to reopen all of `c_star`.
5. Squeeze the full backend query schedule.
6. Prover opens Blaze interleaved columns for the schedule entries that touch
   compiler-systematic `c_star` positions and emits the backend query proof for
   compiler-non-systematic and auxiliary proof-oracle entries tied to the same schedule.

No query index may be sampled before the commitments/messages for the objects it checks are bound.

The transcript must not absorb expanded public tables when a compact seed/spec is sufficient:

```text
full RAA permutation arrays
full encoding tables
full FFT tables
full Merkle trees
full public codewords
```

Large objects may be stored in prover/verifier parameters and borrowed by hot code. They are not
proof bytes and should not be repeatedly hashed into the Fiat-Shamir state.

## Test Requirements

Every extracted component must have tests that match its spec boundary:

1. `SystematicFoldableCodeSpec -> C_sys` determinism, bidirectional logical/physical address
   mapping, fold-pair locality, and parity encoding.
2. Holographic BaseFold query schedules that classify systematic, parity, and auxiliary entries.
3. Standalone BaseFold wrapper using the systematic/holographic core.
4. `Blaze2CodeSpec -> Blaze2Code` determinism.
5. RAA encoding and linearity spot checks.
6. Interleaved column authentication and folding into `c_star[index]`.
7. BaseFold/RMLE core with a backend-sampled query schedule and externally authenticated input
   values.
8. Blaze2 backend using the same core without re-authenticating systematic `c_star` entries.
9. Proof-size accounting tests for each public proof struct.

Negative tests must mutate:

```text
query index
query value
Merkle path
systematic/parity query classification
row evaluation vector u
folding challenge dependent value
backend commitment/proof
code spec seed, version, systematic layout, or shape
claimed evaluation
```

Default tests should stay small. No individual release-mode test should take longer than the
project's configured test-time budget. Larger performance comparisons belong in ignored benchmarks
or explicit performance tests, never in normal unit tests.

## Acceptance Criteria

A BaseFold core or Blaze2 backend change is acceptable only when:

1. The proof objects match the component accounting above.
2. The transcript binds compact public specs and commitments, not expanded public tables.
3. The implementation is monomorphized in hot paths.
4. The BaseFold code object is a systematic random foldable code with verifier-known layout.
5. The standalone BaseFold PCS path is a wrapper around the systematic/holographic RMLE IOPP core,
   not the old monolithic core.
6. Blaze2 uses the same RMLE IOPP core through externally authenticated single-position input
   queries for compiler-systematic `c_star` entries.
7. Tests exercise correctness at the boundary being introduced.
8. No known-to-be-deleted protocol path is added as a stepping stone.
