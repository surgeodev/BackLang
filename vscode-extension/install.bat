@echo off
REM BackLang VS Code Extension Installation Script (Windows)
REM This script sets up and compiles the BackLang debug extension

echo.
echo BackLang VS Code Extension - Installation Script
echo ==================================================
echo.

REM Check if Node.js is installed
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Node.js is not installed.
    echo Please install Node.js 16+ from https://nodejs.org/
    exit /b 1
)

echo Node.js version:
node --version
echo.

echo Installing dependencies...
call npm install

echo.
echo Compiling TypeScript...
call npm run compile

echo.
echo Extension compiled successfully!
echo.
echo Next steps:
echo 1. Open the extension folder in VS Code
echo 2. Press F5 to run the extension in a debug window
echo 3. Open a .bl file to test the extension
echo.
echo To package for distribution:
echo   npm install -g vsce
echo   vsce package
echo.
pause
