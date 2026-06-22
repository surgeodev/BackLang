# Installation

## Linux & macOS (One-Line)

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

This will:
1. Install system dependencies (Linux: build-essential; macOS: Xcode CLI tools)
2. Install Rust (if not present)
3. Clone the BackLang repository
4. Build the release binary
5. Copy `bl` to `/usr/local/bin`
6. Install the VS Code extension (if VS Code is installed)

## Windows (One-Line)

```bat
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.bat | cmd
```

This will:
1. Install Rust via rustup (if not present)
2. Clone the BackLang repository
3. Build the release binary
4. Copy `bl.exe` to `%USERPROFILE%\bin\`
5. Add it to your PATH
6. Install the VS Code extension (if VS Code is installed)

## Manual Install

### Prerequisites

**Linux:**
```bash
# Debian / Ubuntu
sudo apt install build-essential curl

# Arch Linux
sudo pacman -S base-devel curl

# Fedora
sudo dnf install gcc curl
```

**macOS:**
```bash
# Install Xcode CLI tools
xcode-select --install
```

**Windows:**
- Install [Visual Studio Build Tools][vs] (or Visual Studio with "Desktop development with C++")

[vs]: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

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
mkdir %USERPROFILE%\bin -Force
copy target\release\bl.exe %USERPROFILE%\bin\
setx PATH "%PATH%;%USERPROFILE%\bin"
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
