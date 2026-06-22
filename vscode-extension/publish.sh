#!/bin/bash

# Publish BackLang Extension to VS Code Marketplace
# This script builds and publishes the extension

set -e

echo "🚀 Publishing BackLang Debug Extension"
echo "======================================"
echo ""

# Check if vsce is installed
if ! command -v vsce &> /dev/null; then
    echo "Installing vsce globally..."
    npm install -g vsce
fi

# Check if git is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: You have uncommitted changes"
    read -p "Continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Get version from package.json
VERSION=$(node -pe "require('./package.json').version")
echo "📦 Publishing version: $VERSION"

# Compile
echo "🔨 Compiling TypeScript..."
npm run compile

# Package
echo "📦 Packaging extension..."
vsce package

# Publish
echo "🌐 Publishing to marketplace..."
vsce publish

echo ""
echo "✅ Successfully published version $VERSION!"
echo ""
echo "🎉 Your extension is now available on the VS Code Marketplace"
echo ""
echo "Link: https://marketplace.visualstudio.com/items?itemName=backlang.backlang-debug"
