param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\VANTA"
)

$ErrorActionPreference = "Stop"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath) {
    $NormalizedInstall = $InstallDir.TrimEnd("\")
    $Entries = $UserPath.Split(";") | Where-Object {
        $_ -and $_.Trim() -and $_.TrimEnd("\") -ine $NormalizedInstall
    }
    [Environment]::SetEnvironmentVariable("Path", ($Entries -join ";"), "User")
}

if (Test-Path $InstallDir) {
    Remove-Item $InstallDir -Recurse -Force
}

Write-Host "VANTA uninstalled."
Write-Host "Open a new terminal to refresh PATH."
