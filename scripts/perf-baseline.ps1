[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$CargoArgs = @(
    "test",
    "--release",
    "-p",
    "cfri",
    "--test",
    "perf_baseline",
    "--",
    "--ignored",
    "--nocapture",
    "--test-threads=1"
)

Write-Host "==> cfri perf baseline: cargo $($CargoArgs -join ' ')"
Push-Location $RepoRoot
try {
    & cargo @CargoArgs
    if ($LASTEXITCODE -ne 0) {
        throw "cfri perf baseline failed with exit code $LASTEXITCODE"
    }
} finally {
    Pop-Location
}
