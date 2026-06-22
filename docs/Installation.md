# Installation

## One-Line Install (Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

This will:
1. Install system dependencies (libsqlite3-dev, pkg-config, build-essential)
2. Install Rust (if not present)
3. Clone the BackLang repository
4. Build the release binary
5. Copy `bl` to `/usr/local/bin`
6. Install the VS Code extension (if VS Code is installed)

## Manual Install

### Prerequisites

```bash
# Debian / Ubuntu
sudo apt install libsqlite3-dev pkg-config build-essential curl

# Arch Linux
sudo pacman -S sqlite pkg-config base-devel curl

# Fedora
sudo dnf install libsqlite3x-devel pkgconfig gcc curl
```

Install Rust if you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Build

```bash
git clone https://github.com/surgeodev/BackLang
cd BackLang
cargo build --release
sudo cp target/release/bl /usr/local/bin/bl
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
