#!/usr/bin/env bash
set -euo pipefail

REPO="surgeodev/BackLang"
BIN="$HOME/.local/bin/bl"

detect_arch() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        *)      echo "Unsupported OS: $os"; exit 1 ;;
    esac
    case "$arch" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) echo "Unsupported arch: $arch"; exit 1 ;;
    esac
    echo "${arch}-${os}"
}

echo "=== BackLang Installer ==="

TARGET=$(detect_arch)
URL="https://github.com/$REPO/releases/latest/download/backlang-${TARGET}"
if [ "$TARGET" = "x86_64-apple-darwin" ] || [ "$TARGET" = "aarch64-apple-darwin" ]; then
    : # no .exe
fi

echo "→ Downloading BackLang for ${TARGET}..."
mkdir -p "$HOME/.local/bin"
curl -fsSL "$URL" -o "$BIN"
chmod +x "$BIN"

# Add to PATH if not already present
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    RC="${HOME}/.zshrc"
    if [ ! -f "$RC" ] && [ -f "$HOME/.bashrc" ]; then
        RC="$HOME/.bashrc"
    fi
    echo "" >> "$RC"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$RC"
    echo "→ Added ~/.local/bin to PATH in $RC"
fi

echo ""
echo "✓ BackLang installed!"
echo "  Run: source ~/.zshrc  (or open a new terminal)"
echo "  Then: bl --snake"
