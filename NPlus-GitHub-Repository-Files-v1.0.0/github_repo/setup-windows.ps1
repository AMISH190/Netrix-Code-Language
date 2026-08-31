$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "  N+ v1.0 setup" -ForegroundColor Magenta
Write-Host "  Netrix Development" -ForegroundColor DarkGray
Write-Host ""

# Rustup normally puts Rust here. Add it for this process so a fresh terminal
# is not required when the user has just installed Rust.
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) {
    $env:Path = "$cargoBin;$env:Path"
}

$missing = @()
foreach ($cmd in @("rustc", "cargo", "node", "npm")) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        Write-Host "  ✓ $cmd found" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $cmd not found" -ForegroundColor Red
        $missing += $cmd
    }
}

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "Missing tools: $($missing -join ', ')" -ForegroundColor Red
    Write-Host "Install Rust from https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    Write-Host "Install Node.js from https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "  Building N+ compiler..." -ForegroundColor Cyan
Push-Location "$PSScriptRoot\compiler"
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "N+ compiler build failed with exit code $LASTEXITCODE."
    }
} finally {
    Pop-Location
}

$binary = Join-Path $PSScriptRoot "compiler\target\release\nplus.exe"
Write-Host ""
Write-Host "  ✓ Compiler built" -ForegroundColor Green
Write-Host "    $binary" -ForegroundColor DarkGray
Write-Host ""
Write-Host "  Next:" -ForegroundColor Cyan
Write-Host "    cd compiler" -ForegroundColor White
Write-Host "    cargo run -- run ..\examples\hello.npl" -ForegroundColor White
Write-Host ""
Write-Host "  VS Code extension source:" -ForegroundColor Cyan
Write-Host "    $PSScriptRoot\vscode-extension" -ForegroundColor DarkGray
Write-Host ""
