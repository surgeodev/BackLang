# Installation

## Linux & macOS (One-Line)

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

This will:
1. Install system dependencies (Linux: libsqlite3-dev, pkg-config; macOS: Xcode CLI tools)
2. Install Rust (if not present)
3. Clone the BackLang repository
4. Build the release binary
5. Copy `bl` to `/usr/local/bin`
6. Install the VS Code extension (if VS Code is installed)

## Windows (One-Line PowerShell)

Run as Administrator:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
iwr -Uri https://raw.githubusercontent.com/surgeodev/BackLang/main/install.ps1 -OutFile "$env:TEMP\install.ps1"
& "$env:TEMP\install.ps1"
```

Or run directly:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "iwr -Uri https://raw.githubusercontent.com/surgeodev/BackLang/main/install.ps1 -OutFile $env:TEMP\install.ps1; & $env:TEMP\install.ps1"
```

This will:
1. Install Rust via rustup (if not present)
2. Clone the BackLang repository
3. Build the release binary
4. Copy `bl.exe` to `~/.cargo/bin`
5. Add it to your PATH
6. Install the VS Code extension (if VS Code is installed)

## Manual Install

### Prerequisites

**Linux:**
```bash
# Debian / Ubuntu
sudo apt install libsqlite3-dev pkg-config build-essential curl

# Arch Linux
sudo pacman -S sqlite pkg-config base-devel curl

# Fedora
sudo dnf install libsqlite3x-devel pkgconfig gcc curl
```

**macOS:**
```bash
# Install Xcode CLI tools
xcode-select --install

# Optional: pkg-config via Homebrew
brew install pkg-config
```

**Windows:**
- Install [Visual Studio Build Tools][vs] (or Visual Studio with "Desktop development with C++")
- Or install [LLVM + clang][llvm] and use `-x clang` with Rust (see [rustup docs][rustup-msvc])

[vs]: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
[llvm]: https://github.com/llvm/llvm-project/releases
[rustup-msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

On Windows: download from [rustup.rs](https://rustup.rs/)

### Build from Source

```bash
git clone https://github.com/surgeodev/BackLang
cd BackLang
cargo build --release
```

**Linux / macOS:**
```bash
sudo cp target/release/bl /usr/local/bin/bl
```

**Windows:**
```powershell
copy target\release\bl.exe %USERPROFILE%\.cargo\bin\
```

### Verify

```bash
bl --check file.bl    # should print "OK" for valid files
```

## VS Code Extension

If you already have VS Code installed, the extension can be installed manually:

```bash
code --install-extension vscode-extension/backlang-debug-1.0.0.vsix
```

The extension provides:
- Syntax highlighting
- Code completion
- Hover documentation
- Inlay type hints
- CodeLens (Run / Start Server buttons)
- Folding ranges
- Document symbols & breadcrumbs
- Rename & find references
- Real-time diagnostics on save
- 12 code snippets
- Interactive debugger
