@echo off
echo === BackLang Installer ===

set "BIN_DIR=%USERPROFILE%\bin"
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

echo ^> Downloading bl.exe...
curl -fsSLo "%BIN_DIR%\bl.exe" https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-pc-windows-msvc.exe
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Download failed. Check your internet connection.
    pause
    exit /b 1
)

echo ^> Adding to PATH...
"%BIN_DIR%\bl.exe" --install

echo ^> Installing VS Code extension...
curl -fsSLo "%TEMP%\backlang-debug-1.0.0.vsix" https://github.com/surgeodev/BackLang/releases/latest/download/backlang-debug-1.0.0.vsix
where code >nul 2>nul
if %ERRORLEVEL% equ 0 (
    code --install-extension "%TEMP%\backlang-debug-1.0.0.vsix" --force
)
del "%TEMP%\backlang-debug-1.0.0.vsix" 2>nul

echo.
echo [OK] Done!
echo   bl --check file.bl
echo   bl file.bl
echo   bl --debug file.bl
echo.
echo Close and reopen your terminal, then try: bl --version
