# Generate release artifacts

param(
    [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"

Write-Host "Generating release artifacts for version $Version..." -ForegroundColor Green

# Build release binaries
cargo build --release

# Create release directory
$ReleaseDir = "release/v$Version"
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# Copy binaries (platform-specific)
# Windows
if (Test-Path "target/release/bux.exe") {
    Copy-Item "target/release/bux.exe" "$ReleaseDir/bux-windows-x64.exe"
}

# Generate checksums
Write-Host "Generating checksums..." -ForegroundColor Yellow
Get-ChildItem "$ReleaseDir/*" | ForEach-Object {
    $hash = Get-FileHash $_.FullName -Algorithm SHA256
    "$($hash.Hash)  $($_.Name)" | Out-File -FilePath "$($_.FullName).sha256" -Encoding utf8
}

Write-Host "Release artifacts generated in $ReleaseDir" -ForegroundColor Green

