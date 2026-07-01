# Test Script for cvooc-imagemin-compressor

$ErrorActionPreference = "Stop"

# Change to script directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptPath

# 确保 NASM 在 PATH 中（ravif/rav1e 需要），尝试常见安装路径
if (-not (Get-Command "nasm" -ErrorAction SilentlyContinue)) {
    $nasmPaths = @(
        "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\*\*\mingw64\bin",
        "C:\Program Files\NASM",
        "$env:ProgramFiles\NASM",
        "${env:ProgramFiles(x86)}\NASM"
    )
    foreach ($p in $nasmPaths) {
        $resolved = Resolve-Path "$p\nasm.exe" -ErrorAction SilentlyContinue
        if ($resolved) {
            $env:PATH = "$(Split-Path $resolved);$env:PATH"
            break
        }
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
