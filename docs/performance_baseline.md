# Performance Baseline

This repo treats prover and verifier performance as an invariant. These numbers are not production
SLAs, but they are regression guards: any substantial slowdown should be explained by a deliberate
protocol change, stronger security parameter, or better test coverage.

Run the baseline as a single release test:

```powershell
.\scripts\perf-baseline.ps1
```

The script runs:

```powershell
cargo test --release -p cfri --test perf_baseline -- --ignored --nocapture --test-threads=1
```

Do not run another benchmark or performance job at the same time. The output is line-oriented so it
can be copied into this file or parsed by later tooling:

```text
PERF|scheme=basefold|mode=transparent|num_vars=...|...
```

## Current Baseline

Captured on: 2026-05-02

Environment:

- Machine: 13th Gen Intel(R) Core(TM) i7-13700H
- OS: Microsoft Windows 11 Home 10.0.26200
- Rust: `rustc 1.94.1 (e408947bf 2026-03-25)`
- Command: `.\scripts\perf-baseline.ps1`

```text
PERF|kind=metadata|crate=cfri|profile=release|test=current_release_performance_baseline
PERF|scheme=basefold|mode=transparent|num_vars=10|poly_len=1024|setup_ms=1.455|trim_ms=0.191|commit_ms=1.573|eval_ms=0.151|open_ms=4.872|verify_ms=0.440|proof_bytes=22656
PERF|scheme=basefold|mode=hiding|num_vars=10|poly_len=1024|setup_ms=1.142|trim_ms=0.240|commit_ms=2.217|eval_ms=0.107|open_ms=6.140|verify_ms=0.429|proof_bytes=26624
PERF|scheme=blaze|mode=transparent|num_vars=8|poly_len=256|rows=64|queries=16|setup_ms=1.173|trim_ms=0.149|commit_ms=1.662|eval_ms=0.000|open_ms=41.365|verify_ms=36.303|proof_bytes=5720800
PERF|scheme=blaze|mode=hiding|num_vars=8|poly_len=256|rows=64|queries=16|setup_ms=1.407|trim_ms=0.324|commit_ms=1.821|eval_ms=0.000|open_ms=47.797|verify_ms=38.781|proof_bytes=6378160
```

## Latest Transparent Check

Captured on: 2026-05-02

Command:

```powershell
cargo test --release -p cfri --test perf_baseline current_release_transparent_performance_baseline -- --ignored --nocapture --test-threads=1
```

This capture includes the fully checked Blaze product relation path and the BaseFold batch-opening
deduplication for repeated polynomial indices.

```text
PERF|kind=metadata|crate=cfri|profile=release|test=current_release_transparent_performance_baseline
PERF|scheme=basefold|mode=transparent|num_vars=10|poly_len=1024|setup_ms=1.588|trim_ms=0.135|commit_ms=1.641|eval_ms=0.048|open_ms=4.372|verify_ms=0.382|proof_bytes=12096
PERF|scheme=blaze|mode=transparent|num_vars=8|poly_len=256|rows=64|queries=16|setup_ms=0.855|trim_ms=0.182|commit_ms=2.166|eval_ms=0.000|open_ms=108.563|verify_ms=28.498|proof_bytes=3425408
```

## Covered Cases

- `basefold`, transparent mode, `num_vars=10`.
- `basefold`, hiding mode, `num_vars=10`.
- `blaze`, transparent mode, `num_vars=8`, `rows=64`, `queries=16`.
- `blaze`, hiding mode, `num_vars=8`, `rows=64`, `queries=16`.

The baseline test intentionally excludes the old large ad hoc timing tests. Those are useful for
exploration, but this file is the stable checkpoint for current work.

## A/B Against Original

Run the current owned code against the frozen upstream snapshot:

```powershell
.\scripts\perf-ab.ps1
```

The script runs each side sequentially and only compares transparent, non-hiding cases. It does not
add the frozen snapshot to the top-level workspace; instead it creates a temporary copy under
`target/perf-ab`, injects a small perf example, and runs that example in the original workspace.
The temporary original workspace also gets an `original_pp` diagnostic verifier. This "original++"
path restores the skipped verifier checks as boolean checks, prints each result, does not panic or
return early on failed invariants, and returns one aggregate bool. Output includes normalized `PERF`
rows, `CHECK` rows, and `COMPARE` rows:

```text
PERF|impl=current|scheme=basefold|mode=transparent|...
PERF|impl=original|scheme=basefold|mode=transparent|...
CHECK|impl=original_pp|scheme=blaze|check=...|ok=...
PERF|impl=original_pp|scheme=blaze|mode=transparent|...
COMPARE|basefold|transparent|num_vars=10|poly_len=1024|metric=open_ms|current=...|original=...|current_over_original=...
```

Captured on: 2026-05-02

```text
PERF|impl=current|scheme=basefold|mode=transparent|num_vars=10|poly_len=1024|setup_ms=1.592|trim_ms=0.175|commit_ms=1.713|eval_ms=0.254|open_ms=5.811|verify_ms=0.414|proof_bytes=22656
PERF|impl=current|scheme=blaze|mode=transparent|num_vars=8|poly_len=256|rows=64|queries=16|setup_ms=1.095|trim_ms=0.182|commit_ms=1.442|eval_ms=0.000|open_ms=39.853|verify_ms=36.455|proof_bytes=5720800
PERF|impl=original|scheme=basefold|mode=transparent|num_vars=10|poly_len=1024|setup_ms=1.319|trim_ms=0.132|commit_ms=1.259|eval_ms=0.076|open_ms=3.722|verify_ms=0.226|proof_bytes=22656
PERF|impl=original|scheme=blaze|mode=transparent|num_vars=8|poly_len=256|rows=64|queries=16|setup_ms=0.804|trim_ms=0.147|commit_ms=0.968|eval_ms=0.000|open_ms=43.693|verify_ms=2.585|proof_bytes=5720784
PERF|impl=original_pp|scheme=blaze|mode=transparent|num_vars=8|poly_len=256|rows=64|queries=16|checks_ok=false|setup_ms=0.804|trim_ms=0.147|commit_ms=0.968|eval_ms=0.000|open_ms=43.693|verify_ms=37.883|proof_bytes=5720784
```

The `original_pp` check results for this capture were:

```text
CHECK|impl=original_pp|scheme=blaze|check=opening_point_matches_transcript|ok=true
CHECK|impl=original_pp|scheme=blaze|check=commitment_root_matches_proof|ok=true
CHECK|impl=original_pp|scheme=blaze|check=folded_eval_computes|ok=true
CHECK|impl=original_pp|scheme=blaze|check=raa_commitments_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=permutation_commitments_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=sumcheck_transcript_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=backend_evaluations_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=basefold_batch_verify|ok=true
CHECK|impl=original_pp|scheme=blaze|check=merkle_paths_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=query_leaves_read|ok=true
CHECK|impl=original_pp|scheme=blaze|check=merkle_paths_authenticate|ok=false
CHECK|impl=original_pp|scheme=blaze|check=query_linear_combos_match_folded_codeword|ok=false
CHECK|impl=original_pp|scheme=blaze|all=false
```

Important caveat: Blaze verifier timings are not semantically identical. The owned verifier includes
the newer evaluation-binding checks; the original verifier is much lighter here and should be read as
an upstream speed reference, not as the performance target for a fully checked verifier.

## PipFRI A/B Against Original

Run the current owned PipFRI code against the frozen upstream PipFRI snapshot:

```powershell
.\scripts\perf-pipfri-ab.ps1
.\scripts\perf-pipfri-ab.ps1 -NumVars 12
```

The script runs one side at a time. It snapshots `third_party/pipfri` into
`target/perf-pipfri-ab`, injects a small transparent PipFRI example, and compares it with the
owned implementation through the same low-level prover/verifier API shape. Current and original use
the same `StdRng` seeds for the polynomial, opening point, and setup coset. The default `cfri`
feature set keeps Ark's internal parallel FFT support off; enable `--features benchmark` for
explicit parallel experiments.

Captured on: 2026-05-02

```text
PERF|impl=current|scheme=pipfri|mode=transparent|num_vars=8|poly_len=256|sub_vars=3|setup_ms=0.091|trim_ms=0.000|commit_ms=0.242|eval_ms=0.003|open_ms=0.160|verify_ms=0.193|proof_bytes=11056
PERF|impl=original|scheme=pipfri|mode=transparent|num_vars=8|poly_len=256|sub_vars=3|setup_ms=0.091|trim_ms=0.000|commit_ms=0.212|eval_ms=0.001|open_ms=0.148|verify_ms=0.188|proof_bytes=12464
COMPARE|pipfri|transparent|num_vars=8|poly_len=256|sub_vars=3|metric=setup_ms|current=0.091|original=0.091|current_over_original=1.000
COMPARE|pipfri|transparent|num_vars=8|poly_len=256|sub_vars=3|metric=commit_ms|current=0.242|original=0.212|current_over_original=1.142
COMPARE|pipfri|transparent|num_vars=8|poly_len=256|sub_vars=3|metric=open_ms|current=0.16|original=0.148|current_over_original=1.081
COMPARE|pipfri|transparent|num_vars=8|poly_len=256|sub_vars=3|metric=verify_ms|current=0.193|original=0.188|current_over_original=1.027
COMPARE|pipfri|transparent|num_vars=8|poly_len=256|sub_vars=3|metric=proof_bytes|current=11056|original=12464|current_over_original=0.887

PERF|impl=current|scheme=pipfri|mode=transparent|num_vars=12|poly_len=4096|sub_vars=6|setup_ms=0.091|trim_ms=0.000|commit_ms=3.017|eval_ms=0.019|open_ms=1.165|verify_ms=0.647|proof_bytes=43056
PERF|impl=original|scheme=pipfri|mode=transparent|num_vars=12|poly_len=4096|sub_vars=6|setup_ms=0.092|trim_ms=0.000|commit_ms=3.296|eval_ms=0.017|open_ms=1.130|verify_ms=0.735|proof_bytes=41424
COMPARE|pipfri|transparent|num_vars=12|poly_len=4096|sub_vars=6|metric=setup_ms|current=0.091|original=0.092|current_over_original=0.989
COMPARE|pipfri|transparent|num_vars=12|poly_len=4096|sub_vars=6|metric=commit_ms|current=3.017|original=3.296|current_over_original=0.915
COMPARE|pipfri|transparent|num_vars=12|poly_len=4096|sub_vars=6|metric=open_ms|current=1.165|original=1.13|current_over_original=1.031
COMPARE|pipfri|transparent|num_vars=12|poly_len=4096|sub_vars=6|metric=verify_ms|current=0.647|original=0.735|current_over_original=0.880
COMPARE|pipfri|transparent|num_vars=12|poly_len=4096|sub_vars=6|metric=proof_bytes|current=43056|original=41424|current_over_original=1.039
```

The tiny `num_vars=8` case is close enough that timer granularity and randomized query shape matter.
The `num_vars=12` case is the better regression guard: current commit is slightly faster than
original, open is roughly flat, and verify is faster. The proof-size row is a useful artifact-size
smoke check, but query-list randomness still changes the exact Merkle path shape until the oracle is
made deterministic.
