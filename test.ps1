# Test Script for cvooc-imagemin-compressor

$ErrorActionPreference = "Stop"

# Change to script directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptPath

$env:PATH = "C:\Users\liule\AppData\Local\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin;" + $env:PATH

Write-Host "==================== Running Unit Tests ====================" -ForegroundColor Cyan
cargo test -p imagemin-core --lib
if ($LASTEXITCODE -ne 0) {
    Write-Host "Unit tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== Running Integration Tests ====================" -ForegroundColor Cyan
cargo test -p imagemin-core --test integration
if ($LASTEXITCODE -ne 0) {
    Write-Host "Integration tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== Running All Tests ====================" -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "==================== All Tests Passed ====================" -ForegroundColor Green
