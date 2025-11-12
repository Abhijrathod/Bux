# Windows installer script for bux

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\bux"
)

$ErrorActionPreference = "Stop"

Write-Host "Installing bux..." -ForegroundColor Green

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallPath | Out-Null

# Build bux
Write-Host "Building bux..." -ForegroundColor Yellow
cargo build --release

# Copy binary
$BinaryPath = "target\release\bux.exe"
if (Test-Path $BinaryPath) {
    Copy-Item $BinaryPath "$InstallPath\bux.exe"
    Write-Host "Installed to $InstallPath\bux.exe" -ForegroundColor Green
} else {
    Write-Host "Binary not found! Build failed." -ForegroundColor Red
    exit 1
}

# Add to PATH (optional)
$AddToPath = Read-Host "Add bux to PATH? (y/n)"
if ($AddToPath -eq "y") {
    $CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($CurrentPath -notlike "*$InstallPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallPath", "User")
        Write-Host "Added to PATH. Restart your terminal." -ForegroundColor Green
    }
}

Write-Host "Installation complete!" -ForegroundColor Green

