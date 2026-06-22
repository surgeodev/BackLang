# BackLang Installer for Windows
$ErrorActionPreference = "Stop"
Write-Host "=== BackLang Installer ===" -ForegroundColor Cyan

# Check for Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "-> Installing Rust..." -ForegroundColor Yellow
    $url = "https://win.rustup.rs/x86_64"
    $out = "$env:TEMP\rustup-init.exe"
    curl -fsSLo $out $url
    Start-Process -Wait -FilePath $out -ArgumentList "-y"
    Remove-Item $out
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
}

# Source directory
$src = "$env:USERPROFILE\backlang"
if (Test-Path "$PSScriptRoot\Cargo.toml") {
    $src = $PSScriptRoot
} else {
    Write-Host "-> Downloading BackLang from GitHub..." -ForegroundColor Yellow
    if (Test-Path $src) { Remove-Item -Recurse -Force $src }
    git clone --depth 1 https://github.com/surgeodev/BackLang.git $src
}

Set-Location $src

Write-Host "-> Building BackLang..." -ForegroundColor Yellow
cargo build --release

Write-Host "-> Installing binary..." -ForegroundColor Yellow
$binDir = "$env:USERPROFILE\bin"
if (Test-Path "$env:USERPROFILE\.cargo\bin") {
    $binDir = "$env:USERPROFILE\.cargo\bin"
}
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Copy-Item "target\release\bl.exe" "$binDir\bl.exe" -Force

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$binDir", "User")
    Write-Host "-> Added $binDir to PATH" -ForegroundColor Yellow
}

# VS Code extension
$vsix = "vscode-extension\backlang-debug-1.0.0.vsix"
if ((Get-Command code -ErrorAction SilentlyContinue) -and (Test-Path $vsix)) {
    Write-Host "-> Installing VS Code extension..." -ForegroundColor Yellow
    code --install-extension $vsix --force
}

Write-Host ""
Write-Host "[OK] Done!" -ForegroundColor Green
Write-Host "  bl --check file.bl"
Write-Host "  bl file.bl"
Write-Host "  bl --debug file.bl"