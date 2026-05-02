[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$TempRoot = Join-Path $RepoRoot "target\perf-ab"
$OriginalRoot = Join-Path $TempRoot "original"
$OriginalWorkspace = Join-Path $OriginalRoot "third_party\blaze\plonkish"
$OriginalExampleDir = Join-Path $OriginalWorkspace "plonkish_backend\examples"
$OriginalExamplePath = Join-Path $OriginalExampleDir "cfri_ab_perf.rs"
$OriginalBlazeSourcePath = Join-Path $OriginalWorkspace "plonkish_backend\src\pcs\multilinear\blaze.rs"

function Invoke-Captured {
    param(
        [string]$Name,
        [string]$WorkingDirectory,
        [string]$Executable,
        [string[]]$Arguments,
        [hashtable]$Environment = @{}
    )

    Write-Host "==> ${Name}: $Executable $($Arguments -join ' ')"
    Push-Location $WorkingDirectory
    $oldValues = @{}
    $oldErrorActionPreference = $ErrorActionPreference
    try {
        foreach ($key in $Environment.Keys) {
            $oldValues[$key] = [Environment]::GetEnvironmentVariable($key, "Process")
            [Environment]::SetEnvironmentVariable($key, [string]$Environment[$key], "Process")
        }

        $ErrorActionPreference = "Continue"
        $output = & $Executable @Arguments 2>&1
        $exitCode = $LASTEXITCODE
        if ($exitCode -ne 0) {
            $output | ForEach-Object { Write-Host $_ }
            throw "$Name failed with exit code $exitCode"
        }
        return @($output | ForEach-Object { "$_" })
    } finally {
        $ErrorActionPreference = $oldErrorActionPreference
        foreach ($key in $Environment.Keys) {
            [Environment]::SetEnvironmentVariable($key, $oldValues[$key], "Process")
        }
        Pop-Location
    }
}

function Get-PerfRows {
    param(
        [string[]]$Lines,
        [string]$Impl
    )

    $rows = @()
    foreach ($line in $Lines) {
        if ($line -like "PERF|scheme=*|mode=transparent|*") {
            $rows += "PERF|impl=$Impl|$($line.Substring(5))"
        }
    }
    return $rows
}

function Convert-PerfRow {
    param([string]$Line)

    $row = @{}
    foreach ($part in $Line.Split("|")) {
        if ($part -eq "PERF") {
            continue
        }
        $idx = $part.IndexOf("=")
        if ($idx -lt 0) {
            continue
        }
        $row[$part.Substring(0, $idx)] = $part.Substring($idx + 1)
    }
    return $row
}

function Get-RowKey {
    param([hashtable]$Row)

    $keyParts = @(
        $Row["scheme"],
        $Row["mode"],
        "num_vars=$($Row["num_vars"])",
        "poly_len=$($Row["poly_len"])"
    )
    if ($Row.ContainsKey("rows")) {
        $keyParts += "rows=$($Row["rows"])"
    }
    if ($Row.ContainsKey("queries")) {
        $keyParts += "queries=$($Row["queries"])"
    }
    return ($keyParts -join "|")
}

function Write-ComparisonRows {
    param([string[]]$PerfRows)

    $current = @{}
    $original = @{}
    foreach ($line in $PerfRows) {
        $row = Convert-PerfRow $line
        $key = Get-RowKey $row
        if ($row["impl"] -eq "current") {
            $current[$key] = $row
        } elseif ($row["impl"] -eq "original") {
            $original[$key] = $row
        }
    }

    $metrics = @("setup_ms", "trim_ms", "commit_ms", "eval_ms", "open_ms", "verify_ms", "proof_bytes")
    foreach ($key in ($current.Keys | Sort-Object)) {
        if (-not $original.ContainsKey($key)) {
            continue
        }
        foreach ($metric in $metrics) {
            if (-not $current[$key].ContainsKey($metric) -or -not $original[$key].ContainsKey($metric)) {
                continue
            }
            $a = [double]$current[$key][$metric]
            $b = [double]$original[$key][$metric]
            $ratio = if ($b -eq 0.0) { "inf" } else { "{0:F3}" -f ($a / $b) }
            Write-Host "COMPARE|$key|metric=$metric|current=$a|original=$b|current_over_original=$ratio"
        }
    }
}

function Initialize-OriginalWorkspace {
    if (Test-Path $OriginalRoot) {
        Remove-Item -Recurse -Force -LiteralPath $OriginalRoot
    }
    New-Item -ItemType Directory -Force -Path $OriginalRoot | Out-Null

    $archive = Join-Path $TempRoot "original-blaze.tar"
    if (Test-Path $archive) {
        Remove-Item -Force -LiteralPath $archive
    }

    Push-Location $RepoRoot
    try {
        & git archive -o $archive HEAD third_party/blaze/plonkish
        if ($LASTEXITCODE -ne 0) {
            throw "git archive failed with exit code $LASTEXITCODE"
        }
    } finally {
        Pop-Location
    }

    & tar -xf $archive -C $OriginalRoot
    if ($LASTEXITCODE -ne 0) {
        throw "tar extraction failed with exit code $LASTEXITCODE"
    }
    Remove-Item -Force -LiteralPath $archive

    New-Item -ItemType Directory -Force -Path $OriginalExampleDir | Out-Null
    Set-Content -Path $OriginalExamplePath -Value @'
use blake2::Blake2s256;
use num_traits::Zero;
use plonkish_backend::{
    halo2_curves::bn256::Fr,
    pcs::{
        multilinear::{blaze, Basefold, BasefoldExtParams},
        PolynomialCommitmentScheme,
    },
    poly::multilinear::MultilinearPolynomial,
    util::{
        avx_int_types::u64::Blazeu64,
        binary_extension_fields::B128,
        blaze_transcript::BlazeBlake2sTranscript,
        hash::Blake2s,
        transcript::{
            Blake2sTranscript, FieldTranscript, FieldTranscriptRead, FieldTranscriptWrite,
            InMemoryTranscript,
        },
    },
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct PerfRandomCode;

impl BasefoldExtParams for PerfRandomCode {
    fn get_reps() -> usize {
        5
    }

    fn get_rate() -> usize {
        1
    }

    fn get_basecode_rounds() -> usize {
        0
    }

    fn get_rs_basecode() -> bool {
        false
    }

    fn get_code_type() -> String {
        "random".to_string()
    }
}

type BasefoldPcs = Basefold<Fr, Blake2s256, PerfRandomCode>;

#[derive(Clone, Copy)]
struct Timed<T> {
    value: T,
    elapsed: Duration,
}

fn timed<T>(f: impl FnOnce() -> T) -> Timed<T> {
    let start = Instant::now();
    let value = f();
    Timed {
        value,
        elapsed: start.elapsed(),
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn print_perf_line(
    impl_name: &str,
    scheme: &str,
    params: &str,
    setup: Duration,
    trim: Duration,
    commit: Duration,
    eval: Duration,
    open: Duration,
    verify: Duration,
    proof_bytes: usize,
) {
    println!(
        "PERF|impl={impl_name}|scheme={scheme}|mode=transparent|{params}|setup_ms={:.3}|trim_ms={:.3}|commit_ms={:.3}|eval_ms={:.3}|open_ms={:.3}|verify_ms={:.3}|proof_bytes={proof_bytes}",
        ms(setup),
        ms(trim),
        ms(commit),
        ms(eval),
        ms(open),
        ms(verify),
    );
}

fn run_basefold(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([21; 32]);
    let setup = timed(|| BasefoldPcs::setup(poly_size, 1, &mut rng).unwrap());
    let trim = timed(|| BasefoldPcs::trim(&setup.value, poly_size, 1).unwrap());
    let (pp, vp) = trim.value;
    let poly = MultilinearPolynomial::rand(num_vars, &mut rng);

    let mut transcript = Blake2sTranscript::new(());
    let commit = timed(|| BasefoldPcs::commit_and_write(&pp, &poly, &mut transcript).unwrap());
    black_box(&commit.value);
    let point = transcript.squeeze_challenges(num_vars);
    let eval = timed(|| poly.evaluate(&point));
    transcript.write_field_element(&eval.value).unwrap();
    let open = timed(|| {
        BasefoldPcs::open(
            &pp,
            &poly,
            &commit.value,
            &point,
            &eval.value,
            &mut transcript,
        )
        .unwrap()
    });
    black_box(open.value);
    let proof = transcript.into_proof();

    let verify = timed(|| {
        let mut transcript = Blake2sTranscript::from_proof((), proof.as_slice());
        let comm = BasefoldPcs::read_commitment(&vp, &mut transcript).unwrap();
        let point = transcript.squeeze_challenges(num_vars);
        let eval = transcript.read_field_element().unwrap();
        BasefoldPcs::verify(&vp, &comm, &point, &eval, &mut transcript).unwrap();
    });

    print_perf_line(
        "original",
        "basefold",
        &format!("num_vars={num_vars}|poly_len={poly_size}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        eval.elapsed,
        open.elapsed,
        verify.elapsed,
        proof.len(),
    );
}

fn blaze_input(num_rows: usize, poly_size: usize) -> Vec<Vec<Blazeu64>> {
    (0..num_rows)
        .map(|row| {
            (0..poly_size)
                .map(|col| Blazeu64 {
                    value: (0x9e37_79b9u64)
                        .wrapping_mul((row as u64) + 1)
                        .wrapping_add((0xd1b5_4a32u64).wrapping_mul((col as u64) + 3)),
                })
                .collect()
        })
        .collect()
}

fn run_blaze(num_vars: usize, num_rows: usize, num_queries: usize) {
    let poly_size = 1 << num_vars;
    let mut rng = ChaCha8Rng::from_seed([31; 32]);
    let setup = timed(|| {
        blaze::setup::<Blake2s>(poly_size, 1, &mut rng, Some(num_rows), Some(num_queries))
    });
    let trim = timed(|| blaze::trim::<Blake2s>(&setup.value, poly_size, 1));
    let (pp, vp) = trim.value;
    let data = blaze_input(num_rows, poly_size);

    let mut blaze_transcript = BlazeBlake2sTranscript::new(());
    let commit =
        timed(|| blaze::commit_and_write::<Blazeu64, Blake2s>(&pp, &data, &mut blaze_transcript));
    black_box(&commit.value);
    let mut b128_transcript = Blake2sTranscript::new(());
    let point = b128_transcript.squeeze_challenges(num_vars);
    let zero = B128::zero();
    let open = timed(|| {
        blaze::open(
            &pp,
            &data,
            &commit.value,
            &point,
            &zero,
            &mut blaze_transcript,
            &mut b128_transcript,
        )
        .unwrap()
    });
    black_box(open.value);
    let blaze_proof = blaze_transcript.into_proof();
    let b128_proof = b128_transcript.into_proof();

    let verify = timed(|| {
        let mut blaze_transcript =
            BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        let eval_marker = Blazeu64 { value: 0 };
        blaze::verify(
            &vp,
            &commit.value,
            &point,
            &eval_marker,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
        .unwrap();
    });

    let verify_original_pp = timed(|| {
        let mut blaze_transcript =
            BlazeBlake2sTranscript::from_proof((), blaze_proof.as_slice());
        let mut b128_transcript = Blake2sTranscript::from_proof((), b128_proof.as_slice());
        blaze::verify_original_pp(
            &vp,
            &commit.value,
            &point,
            &mut b128_transcript,
            &mut blaze_transcript,
        )
    });
    println!(
        "CHECK|impl=original_pp|scheme=blaze|all={}",
        verify_original_pp.value
    );

    print_perf_line(
        "original",
        "blaze",
        &format!("num_vars={num_vars}|poly_len={poly_size}|rows={num_rows}|queries={num_queries}"),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        Duration::ZERO,
        open.elapsed,
        verify.elapsed,
        blaze_proof.len() + b128_proof.len(),
    );

    print_perf_line(
        "original_pp",
        "blaze",
        &format!("num_vars={num_vars}|poly_len={poly_size}|rows={num_rows}|queries={num_queries}|checks_ok={}", verify_original_pp.value),
        setup.elapsed,
        trim.elapsed,
        commit.elapsed,
        Duration::ZERO,
        open.elapsed,
        verify_original_pp.elapsed,
        blaze_proof.len() + b128_proof.len(),
    );
}

fn main() {
    println!("PERF|kind=metadata|impl=original|source=third_party/blaze|profile=release");
    run_basefold(10);
    run_blaze(8, 64, 16);
}
'@

    Add-Content -Path $OriginalBlazeSourcePath -Value @'

fn original_pp_record_check(name: &str, ok: bool, all: &mut bool) {
    println!("CHECK|impl=original_pp|scheme=blaze|check={name}|ok={ok}");
    *all &= ok;
}

fn original_pp_eval_folded_blaze_poly(folded_poly: &[B128], point: &[B128]) -> Option<B128> {
    if point.len() >= usize::BITS as usize {
        return None;
    }
    if folded_poly.len() != (1usize << point.len()) {
        return None;
    }

    let mut poly = Type2Polynomial {
        poly: folded_poly.to_vec(),
    };
    multilinear_evaluation_ztoa(&mut poly, &point.to_vec());
    poly.poly.first().copied()
}

fn original_pp_authenticate_merkle_path<H: Hash>(
    path: &Vec<Vec<Output<H>>>,
    mut x_index: usize,
    root: &Output<H>,
) -> bool {
    if path.is_empty() {
        return false;
    }

    let mut ok = true;
    for i in 0..path.len() {
        if i + 1 == path.len() {
            ok &= path[i].len() == 1 && &path[i][0] == root;
            break;
        }

        if path[i].len() != 2 {
            ok = false;
            x_index >>= 1;
            continue;
        }

        let mut hasher = H::new();
        let mut hash = Output::<H>::default();
        hasher.update(&path[i][0]);
        hasher.update(&path[i][1]);
        hasher.finalize_into_reset(&mut hash);

        let parent_index = (x_index >> 1) % 2;
        ok &= path[i + 1]
            .get(parent_index)
            .map(|parent| parent == &hash)
            .unwrap_or(false);
        x_index >>= 1;
    }
    ok
}

pub fn verify_original_pp<F: BlazeField, H: Hash>(
    vp: &BlazeVerifierParam,
    comm: &BlazeCommitment<F, H>,
    point: &Vec<B128>,
    b128transcript: &mut impl TranscriptRead<CommitmentChunk<H>, B128>,
    blazetranscript: &mut impl TranscriptRead<CommitmentChunk<H>, F>,
) -> bool {
    #[derive(Debug)]
    pub struct Five {};
    impl BasefoldExtParams for Five {
        fn get_reps() -> usize {
            402
        }

        fn get_rate() -> usize {
            BASEFOLD_RATE
        }

        fn get_basecode_rounds() -> usize {
            0
        }

        fn get_rs_basecode() -> bool {
            false
        }

        fn get_code_type() -> String {
            "binary_rs".to_string()
        }
    }

    type Pcs<H> = Basefold<B128, H, Five>;

    let mut all_ok = true;

    let transcript_point = b128transcript.squeeze_challenges(vp.num_vars);
    original_pp_record_check("opening_point_matches_transcript", &transcript_point == point, &mut all_ok);

    let blaze_root = blazetranscript.read_commitment();
    let root_ok = blaze_root
        .as_ref()
        .map(|root| root == <BlazeCommitment<F, H> as AsRef<Output<H>>>::as_ref(comm))
        .unwrap_or(false);
    original_pp_record_check("commitment_root_matches_proof", root_ok, &mut all_ok);

    let challenges: Vec<B128> =
        bf_to_b128_vec(&blazetranscript.squeeze_challenges(vp.num_rows >> 1));
    let folded_poly_b128 = blazefield_linear_combo_even_faster(&challenges, &comm.bh_evals, 128);
    let eval_ok = original_pp_eval_folded_blaze_poly(&folded_poly_b128, point).is_some();
    original_pp_record_check("folded_eval_computes", eval_ok, &mut all_ok);

    let num_split_chunks = 1 << vp.log_num_chunks;
    let num_raa_comms = 3 * num_split_chunks;
    let num_perm_comms = 4 * num_split_chunks;

    let basefold_comms1 = Pcs::<H>::read_commitments(
        &vp.split_basefold_verifier_param,
        num_raa_comms,
        b128transcript,
    );
    let basefold_comms1_ok = basefold_comms1.is_ok();
    original_pp_record_check("raa_commitments_read", basefold_comms1_ok, &mut all_ok);

    let (_alpha, _beta) = (
        b128transcript.squeeze_challenge(),
        b128transcript.squeeze_challenge(),
    );

    let basefold_comms2 = Pcs::<H>::read_commitments(
        &vp.split_basefold_verifier_param,
        num_perm_comms,
        b128transcript,
    );
    let basefold_comms2_ok = basefold_comms2.is_ok();
    original_pp_record_check("permutation_commitments_read", basefold_comms2_ok, &mut all_ok);

    let rand_point = b128transcript.squeeze_challenges(vp.reg_basefold_verifier_param.num_vars);

    let _coeffs = b128transcript.squeeze_challenges(4);
    let mut sumcheck_transcript_ok = true;
    for _ in 0..3 {
        for _ in 0..vp.reg_basefold_verifier_param.num_rounds {
            sumcheck_transcript_ok &= b128transcript.read_field_elements(3).is_ok();
            b128transcript.squeeze_challenge();
        }
        sumcheck_transcript_ok &= b128transcript.read_field_elements(3).is_ok();
    }
    original_pp_record_check("sumcheck_transcript_read", sumcheck_transcript_ok, &mut all_ok);

    let evaluations = b128transcript.read_field_elements(num_raa_comms + num_perm_comms);
    let evaluations_ok = evaluations.is_ok();
    original_pp_record_check("backend_evaluations_read", evaluations_ok, &mut all_ok);

    let mut basefold_batch_ok = false;
    if let (Ok(basefold_comms1), Ok(basefold_comms2), Ok(evaluations)) =
        (basefold_comms1, basefold_comms2, evaluations)
    {
        let basefold_comms = chain(basefold_comms1.iter(), basefold_comms2.iter()).collect::<Vec<_>>();
        let evals_al = evaluations
            .iter()
            .enumerate()
            .map(|(i, v)| Evaluation {
                poly: i,
                point: i,
                value: v.clone(),
            })
            .collect::<Vec<_>>();
        let al_rand_point = &rand_point.clone()[0..vp.split_basefold_verifier_param.num_vars];
        let points: Vec<Vec<B128>> = basefold_comms
            .par_iter()
            .map(|_| al_rand_point.to_vec())
            .collect::<Vec<_>>();

        basefold_batch_ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Pcs::<H>::batch_verify(
                &vp.split_basefold_verifier_param,
                basefold_comms,
                &points,
                &evals_al,
                b128transcript,
            )
            .is_ok()
        }))
        .unwrap_or(false);
    }
    original_pp_record_check("basefold_batch_verify", basefold_batch_ok, &mut all_ok);

    let q_challenges = b128transcript.squeeze_challenges(vp.num_queries);
    let row_len = 1 << vp.num_vars;

    let mut paths = Vec::new();
    let mut paths_read_ok = true;
    for _ in 0..vp.num_queries {
        match blazetranscript.read_commitments(2 * vp.num_vars) {
            Ok(path) => paths.push(path.chunks(2).map(|c| c.to_vec()).collect::<Vec<_>>()),
            Err(_) => {
                paths_read_ok = false;
                paths.push(Vec::new());
            }
        }
    }
    original_pp_record_check("merkle_paths_read", paths_read_ok, &mut all_ok);

    let queries = blazetranscript.read_field_elements(vp.num_queries * (vp.num_rows >> 1));
    let queries_ok = queries.is_ok();
    original_pp_record_check("query_leaves_read", queries_ok, &mut all_ok);

    let queries_usize: Vec<usize> = q_challenges
        .par_iter()
        .map(|x_index| {
            let x_rep = (*x_index).to_repr();
            let x: &[u8] = x_rep.as_ref();
            let (int_bytes, _) = x.split_at(std::mem::size_of::<u32>());
            let x_int: u32 = u32::from_be_bytes(int_bytes.try_into().unwrap());
            (x_int as usize) % row_len
        })
        .collect::<Vec<_>>();

    let merkle_ok = blaze_root
        .as_ref()
        .map(|root| {
            (0..vp.num_queries)
                .into_par_iter()
                .map(|q| original_pp_authenticate_merkle_path::<H>(&paths[q], queries_usize[q], root))
                .all(|ok| ok)
        })
        .unwrap_or(false);
    original_pp_record_check("merkle_paths_authenticate", merkle_ok, &mut all_ok);

    let mut query_linear_combo_ok = false;
    if let Ok(queries_flat) = queries {
        query_linear_combo_ok = queries_flat
            .par_chunks_exact(vp.num_rows >> 1)
            .zip(queries_usize.par_iter())
            .map(|(query, x_index)| {
                let query_b128 = bf_to_b128_vec(&query.to_vec());
                let sum = query_b128
                    .iter()
                    .zip(challenges.iter())
                    .fold(B128::zero(), |acc, (query, challenge)| acc + (*query * *challenge));
                folded_poly_b128
                    .get(*x_index)
                    .map(|expected| *expected == sum)
                    .unwrap_or(false)
            })
            .all(|ok| ok);
    }
    original_pp_record_check("query_linear_combos_match_folded_codeword", query_linear_combo_ok, &mut all_ok);

    all_ok
}
'@
}

Write-Host "==> A/B perf comparison runs sequentially. Do not run another benchmark at the same time."

$currentOutput = Invoke-Captured `
    -Name "current cfri perf baseline" `
    -WorkingDirectory $RepoRoot `
    -Executable "cargo" `
    -Arguments @("test", "--release", "-p", "cfri", "--test", "perf_baseline", "current_release_transparent_performance_baseline", "--", "--ignored", "--nocapture", "--test-threads=1")

$currentRows = Get-PerfRows -Lines $currentOutput -Impl "current"
$currentRows | ForEach-Object { Write-Host $_ }

Initialize-OriginalWorkspace

$originalOutput = Invoke-Captured `
    -Name "original upstream perf baseline" `
    -WorkingDirectory $OriginalWorkspace `
    -Executable "cargo" `
    -Arguments @("run", "--quiet", "--release", "-p", "plonkish_backend", "--example", "cfri_ab_perf") `
    -Environment @{ RUSTC_BOOTSTRAP = "1" }

$originalOutput | Where-Object { $_ -like "CHECK|impl=original_pp|*" } | ForEach-Object { Write-Host $_ }
$originalRows = @($originalOutput | Where-Object { $_ -like "PERF|impl=original*|scheme=*|mode=transparent|*" })
$originalRows | ForEach-Object { Write-Host $_ }

Write-ComparisonRows -PerfRows (@($currentRows) + @($originalRows))
