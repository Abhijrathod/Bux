# PowerShell build script for bux

param(
    [string]$Target = "release",
    [switch]$Test = $false,
    [switch]$Clean = $false
)

$ErrorActionPreference = "Stop"

if ($Clean) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
}

Write-Host "Building bux ($Target)..." -ForegroundColor Green

if ($Target -eq "release") {
    cargo build --release
} else {
    cargo build
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

if ($Test) {
    Write-Host "Running tests..." -ForegroundColor Green
    cargo test
}

Write-Host "Build complete!" -ForegroundColor Green

