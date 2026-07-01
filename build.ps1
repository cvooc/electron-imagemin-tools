# Build and Test Script for cvooc-imagemin-compressor

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
