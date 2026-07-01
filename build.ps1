# Build and Test Script for cvooc-imagemin-compressor

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
