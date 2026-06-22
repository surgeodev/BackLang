@echo off
echo === BackLang Installer ===

:: Check Rust
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo ^> Installing Rust...
    curl -fsSLo "%TEMP%\rustup-init.exe" https://win.rustup.rs/x86_64
    "%TEMP%\rustup-init.exe" -y
    del "%TEMP%\rustup-init.exe"
    set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
)

:: Source
set "SRC=%USERPROFILE%\backlang"
if exist "Cargo.toml" set "SRC=%CD%"
if not exist "%SRC%\Cargo.toml" (
    echo ^> Downloading BackLang from GitHub...
    rmdir /s /q "%SRC%" 2>nul
    git clone --depth 1 https://github.com/surgeodev/BackLang.git "%SRC%"
)

cd /d "%SRC%"

echo ^> Building BackLang (this may take a few minutes)...
cargo build --release

echo ^> Installing binary...
set "BIN_DIR=%USERPROFILE%\bin"
if exist "%USERPROFILE%\.cargo\bin" set "BIN_DIR=%USERPROFILE%\.cargo\bin"
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"
copy /y "target\release\bl.exe" "%BIN_DIR%\bl.exe" >nul

:: Add to PATH
for /f "skip=2 tokens=3*" %%A in ('reg query "HKCU\Environment" /v Path 2^>nul') do set "USER_PATH=%%A %%B"
if "%USER_PATH%"=="" set "USER_PATH=%PATH%"
echo %USER_PATH% | findstr /C:"%BIN_DIR%" >nul
if %ERRORLEVEL% neq 0 (
    setx PATH "%USER_PATH%;%BIN_DIR%"
    echo ^> Added %BIN_DIR% to PATH
)

:: VS Code extension
if exist "vscode-extension\backlang-debug-1.0.0.vsix" (
    where code >nul 2>nul
    if %ERRORLEVEL% equ 0 (
        echo ^> Installing VS Code extension...
        code --install-extension "vscode-extension\backlang-debug-1.0.0.vsix" --force
    )
)

echo.
echo [OK] Done!
echo   bl --check file.bl
echo   bl file.bl
echo   bl --debug file.bl
