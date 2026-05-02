[CmdletBinding()]
param(
    [int]$NumVars = 8
)

$ErrorActionPreference = "Stop"
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$TempRoot = Join-Path $RepoRoot "target\perf-pipfri-ab"
$OriginalRoot = Join-Path $TempRoot "original"
$OriginalWorkspace = Join-Path $OriginalRoot "third_party\pipfri"
$OriginalExampleDir = Join-Path $OriginalWorkspace "pip_fri\examples"
$OriginalExamplePath = Join-Path $OriginalExampleDir "cfri_pipfri_ab_perf.rs"

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
    if ($Row.ContainsKey("sub_vars")) {
        $keyParts += "sub_vars=$($Row["sub_vars"])"
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

    $archive = Join-Path $TempRoot "original-pipfri.tar"
    if (Test-Path $archive) {
        Remove-Item -Force -LiteralPath $archive
    }

    Push-Location $RepoRoot
    try {
        & git archive -o $archive HEAD third_party/pipfri
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
use ark_ff::UniformRand;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use pip_fri::{prover::Prover, verifier::Verifier};
use rand::{rngs::StdRng, SeedableRng};
use std::{
    hint::black_box,
    mem::size_of,
    time::{Duration, Instant},
};
use utils::{
    fiat_shamir::RandomOracle,
    goldilocks::Goldilocks as T,
    helper::{Helper, MultilinearPolynomial},
    interpolate_vecs_value::{get_sub_variable_num, get_tensor},
    merkle_tree::MERKLE_ROOT_SIZE,
    CODE_RATE, SECURITY_BITS,
};

#[derive(Clone, Copy)]
struct Timed<TValue> {
    value: TValue,
    elapsed: Duration,
}

fn timed<TValue>(f: impl FnOnce() -> TValue) -> Timed<TValue> {
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

fn polynomial(num_vars: usize, seed: u64) -> MultilinearPolynomial<T> {
    let mut rng = StdRng::seed_from_u64(seed);
    let coefficients = (0..(1usize << num_vars))
        .map(|_| T::rand(&mut rng))
        .collect();
    MultilinearPolynomial::new(coefficients)
}

fn point(num_vars: usize, seed: u64) -> Vec<T> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..num_vars).map(|_| T::rand(&mut rng)).collect()
}

fn print_perf_line(
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
        "PERF|impl=original|scheme=pipfri|mode=transparent|{params}|setup_ms={:.3}|trim_ms={:.3}|commit_ms={:.3}|eval_ms={:.3}|open_ms={:.3}|verify_ms={:.3}|proof_bytes={proof_bytes}",
        ms(setup),
        ms(trim),
        ms(commit),
        ms(eval),
        ms(open),
        ms(verify),
    );
}

fn run_pipfri(num_vars: usize) {
    let poly_size = 1 << num_vars;
    let polynomial = polynomial(num_vars, 41);
    let point = point(num_vars, 42);
    let eval = timed(|| polynomial.evaluate(&point));

    let sub_variable_num = get_sub_variable_num(&polynomial);
    let (sub_open_point, remaining_var) = point.split_at(sub_variable_num);
    let sub_open_point = sub_open_point.to_vec();
    let tensor = get_tensor(&remaining_var.to_vec());

    let setup = timed(|| {
        let mut rng = StdRng::seed_from_u64(43);
        let mut interpolate_cosets = vec![GeneralEvaluationDomain::new_coset(
            1 << (sub_variable_num + CODE_RATE),
            T::rand(&mut rng),
        )
        .unwrap()];
        for i in 1..sub_variable_num {
            interpolate_cosets.push(Helper::pow(&interpolate_cosets[i - 1], 2));
        }
        let oracle = RandomOracle::new(sub_variable_num, SECURITY_BITS / CODE_RATE);
        (interpolate_cosets, oracle)
    });
    let (interpolate_cosets, oracle) = setup.value;

    let commit = timed(|| {
        let mut prover = Prover::new(
            sub_variable_num,
            &interpolate_cosets,
            polynomial,
            &oracle,
            &tensor,
        );
        let commitment = prover.commit_polynomial();
        (prover, commitment)
    });
    let (mut prover, commitment) = commit.value;
    black_box(&commitment);

    let open = timed(|| {
        let mut verifier = Verifier::new(
            sub_variable_num,
            commitment,
            &interpolate_cosets,
            &oracle,
            &sub_open_point,
            &tensor,
        );
        let proofs = prover.open(&sub_open_point, &mut verifier);
        (verifier, proofs)
    });
    let (verifier, (polynomial_proof, folding_proof, function_proof)) = open.value;

    let verify = timed(|| {
        assert!(verifier.verify(
            &polynomial_proof,
            &folding_proof,
            &function_proof,
            eval.value,
        ));
    });

    let proof_bytes = folding_proof.iter().map(|x| x.proof_size()).sum::<usize>()
        + polynomial_proof.proof_size()
        + function_proof.iter().map(|x| x.proof_size()).sum::<usize>()
        + (2 * sub_variable_num).saturating_sub(3) * MERKLE_ROOT_SIZE
        + 2 * size_of::<T>();

    print_perf_line(
        &format!("num_vars={num_vars}|poly_len={poly_size}|sub_vars={sub_variable_num}"),
        setup.elapsed,
        Duration::ZERO,
        commit.elapsed,
        eval.elapsed,
        open.elapsed,
        verify.elapsed,
        proof_bytes,
    );
}

fn main() {
    let num_vars = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(8);
    println!("PERF|kind=metadata|impl=original|source=third_party/pipfri|profile=release");
    run_pipfri(num_vars);
}
'@
}

Write-Host "==> PiPFRI A/B perf comparison runs sequentially. Do not run another benchmark at the same time."
Write-Host "==> PiPFRI A/B num_vars=$NumVars"

$currentOutput = Invoke-Captured `
    -Name "current cfri PiPFRI perf baseline" `
    -WorkingDirectory $RepoRoot `
    -Executable "cargo" `
    -Arguments @("run", "--quiet", "--release", "-p", "cfri", "--example", "pipfri_ab_perf", "--", "$NumVars")

$currentRows = @($currentOutput | Where-Object { $_ -like "PERF|impl=current|scheme=pipfri|mode=transparent|*" })
$currentRows | ForEach-Object { Write-Host $_ }

Initialize-OriginalWorkspace

$originalOutput = Invoke-Captured `
    -Name "original upstream PiPFRI perf baseline" `
    -WorkingDirectory $OriginalWorkspace `
    -Executable "cargo" `
    -Arguments @("run", "--quiet", "--release", "-p", "pip_fri", "--example", "cfri_pipfri_ab_perf", "--", "$NumVars")

$originalRows = @($originalOutput | Where-Object { $_ -like "PERF|impl=original|scheme=pipfri|mode=transparent|*" })
$originalRows | ForEach-Object { Write-Host $_ }

Write-ComparisonRows -PerfRows (@($currentRows) + @($originalRows))
