#!/usr/bin/env bash
set -euo pipefail

echo "=== BackLang Installer ==="

# System deps
if command -v apt &>/dev/null; then
    sudo apt update && sudo apt install -y libsqlite3-dev pkg-config build-essential curl
elif command -v pacman &>/dev/null; then
    sudo pacman -S --needed sqlite pkg-config base-devel curl
elif command -v dnf &>/dev/null; then
    sudo dnf install -y libsqlite3x-devel pkgconfig gcc curl
fi

# Rust
if ! command -v cargo &>/dev/null; then
    echo "→ Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Source: local dir or clone from GitHub
SCRIPT_DIR="$(cd "$(dirname "$0")" 2>/dev/null && pwd || echo "")"
if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    SRC="$SCRIPT_DIR"
else
    SRC="$HOME/backlang"
    echo "→ Downloading BackLang from GitHub..."
    git clone --depth 1 https://github.com/surgeodev/backlang "$SRC"
fi

cd "$SRC"

echo "→ Building BackLang..."
cargo build --release

echo "→ Installing binary..."
sudo cp target/release/bl /usr/local/bin/bl

VSIX="vscode-extension/backlang-debug-1.0.0.vsix"
if command -v code &>/dev/null && [ -f "$VSIX" ]; then
    echo "→ Installing VS Code extension..."
    code --install-extension "$VSIX" --force
fi

echo ""
echo "✓ Done!"
echo "  bl --check file.bl"
echo "  bl file.bl"
echo "  bl --debug file.bl"
