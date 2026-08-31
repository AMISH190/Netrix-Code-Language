param(
    [Parameter(Mandatory=$true)]
    [string]$RepositoryUrl
)

$ErrorActionPreference = "Stop"

Write-Host "N+ v1.0 • GitHub publisher" -ForegroundColor Magenta
Write-Host "" 

git --version | Out-Null

if (-not (Test-Path ".git")) {
    git init
}

git branch -M main
git add .
git commit -m "N+ v1.0 initial publication"

git remote remove origin 2>$null
if ($LASTEXITCODE -ne 0) { $global:LASTEXITCODE = 0 }
git remote add origin $RepositoryUrl
git push -u origin main

git tag -f v1.0.0
git push -f origin v1.0.0

Write-Host ""
Write-Host "✓ N+ v1.0 published" -ForegroundColor Green
Write-Host "Repository: $RepositoryUrl" -ForegroundColor Cyan
Write-Host "Release tag: v1.0.0" -ForegroundColor Cyan
