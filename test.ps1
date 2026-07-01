# Test Script for cvooc-imagemin-compressor

$ErrorActionPreference = "Stop"

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptPath

# 确保 NASM 在 PATH 中
if (-not (Get-Command "nasm" -ErrorAction SilentlyContinue)) {
    $nasmDir = $null
    $p = Resolve-Path "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\*\*\mingw64\bin\nasm.exe" -ErrorAction SilentlyContinue
    if ($p) {
        $nasmDir = Split-Path ($p | Select-Object -First 1)
    } elseif (Test-Path "C:\Program Files\NASM\nasm.exe") {
        $nasmDir = "C:\Program Files\NASM"
    }
    if ($nasmDir) {
        $env:PATH = "$nasmDir;" + $env:PATH
    }
}

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
