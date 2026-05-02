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

$BuildArgs = @("build")
if ($Release) {
    $BuildArgs += "--release"
}

Invoke-CargoStep `
    -Name "cfri core" `
    -WorkingDirectory $RepoRoot `
    -CargoArgs ($BuildArgs + @("-p", "cfri"))
