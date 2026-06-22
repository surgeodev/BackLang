#!/bin/bash
# install_vscode_ext.sh - Installer l'extension VS Code BackLang

set -e

echo "🔧 Installation de l'extension VS Code BackLang..."

EXT_DIR="/home/surgeo/backlang-rust/vscode-extension"
cd "$EXT_DIR"

# Installer les dépendances npm
echo "📦 Installation des dépendances npm..."
npm install

# Compiler l'extension
echo "🔨 Compilation de l'extension..."
npm run compile

# Créer le package .vsix
echo "📦 Création du package VSIX..."
npm install -g vsce 2>/dev/null || true
vsce package --allow-missing-repository 2>/dev/null || {
    echo "⚠️  vsce n'est pas installé globalement"
    echo "Installation de vsce..."
    npm install -g vsce
    vsce package --allow-missing-repository
}

# Installer dans VS Code
echo "🚀 Installation dans VS Code..."
code --install-extension backlang-debug-1.0.0.vsix || {
    echo "⚠️  VS Code n'est peut-être pas dans le PATH"
    echo "Installez manuellement: code --install-extension $EXT_DIR/backlang-debug-1.0.0.vsix"
}

echo "✅ Extension installée avec succès!"
echo ""
echo "📋 Prochaines étapes:"
echo "   1. Redémarrez VS Code"
echo "   2. Ouvrez un fichier .bl"
echo "   3. Utilisez le bouton Play (▶️) dans la barre de titre pour exécuter"
echo "   4. Ou Ctrl+Shift+R pour exécuter"
echo "   5. Utilisez Ctrl+Shift+D pour déboguer"
