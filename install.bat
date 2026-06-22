@echo off
REM BackLang - Complete Installation Script (Windows)
REM One-command setup for BackLang VS Code Extension + Compiler

setlocal enabledelayedexpansion

echo.
echo BackLang - Complete Installation
echo ====================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust not found. Installing Rust...
    echo Download from: https://rustup.rs/
    echo Run the installer and try again.
    pause
    exit /b 1
)

echo Rust found
echo.

REM Get project directory
set PROJECT_DIR=%~dp0

echo Building BackLang Compiler...
cd /d "%PROJECT_DIR%"
call cargo build --release

if %errorlevel% neq 0 (
    echo Error building compiler
    pause
    exit /b 1
)

echo Compiler built successfully
echo.

REM Check if VS Code is installed
where code >nul 2>nul
if %errorlevel% equ 0 (
    echo Installing VS Code Extension...
    
    if exist "%PROJECT_DIR%vscode-extension\backlang-debug-1.0.0.vsix" (
        code --install-extension "%PROJECT_DIR%vscode-extension\backlang-debug-1.0.0.vsix"
        echo Extension installed successfully
    ) else (
        echo VSIX not found. Building extension...
        cd /d "%PROJECT_DIR%vscode-extension"
        call npm install
        call npm run compile
        call npx vsce package
        code --install-extension "%PROJECT_DIR%vscode-extension\backlang-debug-1.0.0.vsix"
        echo Extension installed successfully
    )
) else (
    echo VS Code not found. Install it from: https://code.visualstudio.com
    echo Then run: code --install-extension "%PROJECT_DIR%vscode-extension\backlang-debug-1.0.0.vsix"
)

echo.
echo ====================================
echo Installation Complete!
echo ====================================
echo.
echo Test BackLang:
echo   target\release\bl.exe test.bl
echo.
echo Open in VS Code:
echo   code .
echo.
echo Keyboard Shortcuts:
echo   Ctrl+Shift+R  - Run BackLang file
echo   Ctrl+Shift+D  - Debug BackLang file
echo   Ctrl+Shift+P  - Command palette (!bl for HTML5)
echo.
pause
