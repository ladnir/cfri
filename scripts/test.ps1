[CmdletBinding()]
param(
    [switch]$Run
)

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")

function Invoke-CargoStep {
    param(
        [string]$Name,
        [string]$WorkingDirectory,
        [string[]]$CargoArgs
    )

    $Executable = "cargo"

    Write-Host "==> ${Name}: $Executable $($CargoArgs -join ' ')"
    Push-Location $WorkingDirectory
    try {
        & $Executable @CargoArgs
        if ($LASTEXITCODE -ne 0) {
            throw "$Name failed with exit code $LASTEXITCODE"
        }
    } finally {
        Pop-Location
    }
}

$TestSuffix = @()
if (-not $Run) {
    $TestSuffix = @("--no-run")
}

Invoke-CargoStep `
    -Name "cfri core tests" `
    -WorkingDirectory $RepoRoot `
    -CargoArgs (@("test", "-p", "cfri") + $TestSuffix)
