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
        [string[]]$CargoArgs,
        [string]$Toolchain
    )

    $Executable = "cargo"
    $PrefixArgs = @()
    if ($Toolchain) {
        $Executable = "rustup"
        $PrefixArgs = @("run", $Toolchain, "cargo")
    }

    Write-Host "==> ${Name}: $Executable $(($PrefixArgs + $CargoArgs) -join ' ')"
    Push-Location $WorkingDirectory
    try {
        & $Executable @($PrefixArgs + $CargoArgs)
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

Invoke-CargoStep `
    -Name "Blaze / plonkish_backend tests" `
    -WorkingDirectory (Join-Path $RepoRoot "third_party/blaze/plonkish") `
    -CargoArgs (@("test", "-p", "plonkish_backend", "--lib") + $TestSuffix) `
    -Toolchain "nightly"

Invoke-CargoStep `
    -Name "PIP_FRI workspace tests" `
    -WorkingDirectory (Join-Path $RepoRoot "third_party/pipfri") `
    -CargoArgs (@("test", "--workspace") + $TestSuffix)
