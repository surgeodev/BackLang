#!/bin/bash

# BackLang VS Code Extension Installation Script
# This script sets up and compiles the BackLang debug extension

set -e

echo "🚀 BackLang VS Code Extension - Installation Script"
echo "=================================================="
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js is not installed."
    echo "Please install Node.js 16+ from https://nodejs.org/"
    exit 1
fi

echo "✅ Node.js version: $(node --version)"
echo ""

# Navigate to extension directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "📦 Installing dependencies..."
npm install

echo ""
echo "🔨 Compiling TypeScript..."
npm run compile

echo ""
echo "✅ Extension compiled successfully!"
echo ""
echo "Next steps:"
echo "1. Open the extension folder in VS Code"
echo "2. Press F5 to run the extension in a debug window"
echo "3. Open a .bl file to test the extension"
echo ""
echo "To package for distribution:"
echo "  npm install -g vsce"
echo "  vsce package"
echo ""
