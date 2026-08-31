param(
    [Parameter(Mandatory = $true)]
    [string]$RepositoryUrl
)

$ErrorActionPreference = 'Stop'

Write-Host "N+ v1.0.0 GitHub release preparation" -ForegroundColor Magenta

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw "Git was not found. Install Git first."
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "Cargo was not found. Install Rust first."
}

if (-not (Test-Path "compiler/Cargo.toml")) {
    throw "Run this script from the N+ project root."
}

Write-Host "Building N+..." -ForegroundColor Cyan
Push-Location compiler
cargo build --release
if ($LASTEXITCODE -ne 0) { Pop-Location; throw "N+ compiler build failed." }
Pop-Location

if (-not (Test-Path "compiler/target/release/nplus.exe")) {
    throw "nplus.exe was not produced."
}

Write-Host "Initializing Git repository..." -ForegroundColor Cyan
if (-not (Test-Path ".git")) {
    git init
}

git branch -M main

git add .
git commit -m "Release N+ v1.0.0" 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "No new commit was necessary." -ForegroundColor Yellow
}

$remoteExists = git remote get-url origin 2>$null
if (-not $remoteExists) {
    git remote add origin $RepositoryUrl
} elseif ($remoteExists -ne $RepositoryUrl) {
    git remote set-url origin $RepositoryUrl
}

git push -u origin main

git tag -f v1.0.0
git push -f origin v1.0.0

Write-Host "Done." -ForegroundColor Green
Write-Host "GitHub Actions will build the Windows .exe and create the v1.0.0 Release automatically." -ForegroundColor Green
