[CmdletBinding()]
param(
    [switch]$Release
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

$BuildArgs = @("build")
if ($Release) {
    $BuildArgs += "--release"
}

Invoke-CargoStep `
    -Name "cfri core" `
    -WorkingDirectory $RepoRoot `
    -CargoArgs ($BuildArgs + @("-p", "cfri"))

Invoke-CargoStep `
    -Name "cfri Blaze imports" `
    -WorkingDirectory $RepoRoot `
    -CargoArgs ($BuildArgs + @("-p", "cfri-blaze")) `
    -Toolchain "nightly"

Invoke-CargoStep `
    -Name "Blaze / plonkish_backend" `
    -WorkingDirectory (Join-Path $RepoRoot "third_party/blaze/plonkish") `
    -CargoArgs ($BuildArgs + @("-p", "plonkish_backend")) `
    -Toolchain "nightly"
