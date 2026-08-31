$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$compiler = Join-Path $root "compiler\target\debug\nplus.exe"
if (-not (Test-Path $compiler)) {
    $compiler = Join-Path $root "compiler\target\release\nplus.exe"
}
if (-not (Test-Path $compiler)) {
    Write-Host "N+ compiler not built. Run .\setup-windows.ps1 first." -ForegroundColor Yellow
    exit 1
}

Get-ChildItem (Join-Path $root "examples") -Filter *.npl | ForEach-Object {
    Write-Host ""
    Write-Host "=== $($_.Name) ===" -ForegroundColor Magenta
    & $compiler run $_.FullName
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
