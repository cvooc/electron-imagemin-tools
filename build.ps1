# Build and Test Script for cvooc-imagemin-compressor

$ErrorActionPreference = "Stop"

# Change to script directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptPath

$env:PATH = "C:\Users\liule\AppData\Local\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin;" + $env:PATH

Write-Host "==================== Running Tests ====================" -ForegroundColor Cyan
cargo test -p imagemin-core
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== Building Release ====================" -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== Copying Binary ====================" -ForegroundColor Cyan
Copy-Item -Path "target\release\ui.exe" -Destination "dist\cvooc-imagemin-compressor.exe" -Force
if ($LASTEXITCODE -ne 0) {
    Write-Host "Copy failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== Build Complete ====================" -ForegroundColor Green
$file = Get-Item "dist\cvooc-imagemin-compressor.exe"
Write-Host "Output: dist\cvooc-imagemin-compressor.exe"
Write-Host ("Size: {0:N2} MB" -f ($file.Length / 1MB))
