param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\VANTA"
)

$ErrorActionPreference = "Stop"
$ExeName = "vanta.exe"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Split-Path -Parent $ScriptDir

$Candidates = @(
    (Join-Path $RepoRoot $ExeName),
    (Join-Path $RepoRoot "target\release\vanta.exe"),
    (Join-Path $ScriptDir $ExeName)
)

$SourceExe = $null
foreach ($candidate in $Candidates) {
    if (Test-Path $candidate) {
        $SourceExe = $candidate
        break
    }
}

if (-not $SourceExe) {
    throw "vanta.exe not found. Build with 'cargo build --release' or run from a release archive."
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$DestinationExe = Join-Path $InstallDir $ExeName
Copy-Item $SourceExe $DestinationExe -Force

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$Entries = @()
if ($UserPath) {
    $Entries = $UserPath.Split(";") | Where-Object { $_ -and $_.Trim() }
}

$NormalizedInstall = $InstallDir.TrimEnd("\")
$HasPath = $Entries | Where-Object { $_.TrimEnd("\") -ieq $NormalizedInstall }

if (-not $HasPath) {
    $UserPath = (($Entries + $InstallDir) -join ";")
    [Environment]::SetEnvironmentVariable("Path", $UserPath, "User")
    Write-Host "Added VANTA to user PATH."
}

# Refresh PATH in this PowerShell process. A parent shell cannot be mutated by a child process.
$MachinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$env:Path = "$MachinePath;$UserPath"

Write-Host ""
Write-Host "VANTA installed:"
Write-Host "  $DestinationExe"
Write-Host ""
& $DestinationExe --version
Write-Host ""
Write-Host "Installation verified."
