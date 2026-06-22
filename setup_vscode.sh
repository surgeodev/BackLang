#!/bin/bash
# setup_vscode.sh - Préparer l'environnement VS Code

set -e

echo "🔧 Préparation de l'environnement VS Code pour BackLang..."

# 1. Compiler le projet Rust
echo "1. Compilation de BackLang (debug)..."
cd /home/surgeo/backlang-rust
cargo build
echo "   ✅ Binaire: target/debug/bl"

echo "2. Compilation de BackLang (release)..."
cargo build --release
echo "   ✅ Binaire: target/release/bl"

# 2. Compiler l'extension VS Code
echo "3. Compilation de l'extension VS Code..."
cd vscode-extension
npm install 2>/dev/null || true
npm run compile
echo "   ✅ Extension compilée"

# 3. Créer le package .vsix
echo "4. Création du package VSIX..."
if ! command -v vsce &> /dev/null; then
    echo "   Installation de vsce..."
    npm install -g vsce
fi

vsce package --allow-missing-repository 2>/dev/null || true
echo "   ✅ Package créé"

# 4. Créer un workspace de test
echo "5. Création d'un workspace de test..."
cd /home/surgeo/backlang-rust
cat > test_backlang.code-workspace << 'EOF'
{
    "folders": [
        {
            "path": ".",
            "name": "BackLang"
        }
    ],
    "settings": {
        "backlang.debug.blPath": "./target/debug/bl",
        "editor.fontSize": 14
    },
    "extensions": [
        "backlang.backlang-debug"
    ]
}
EOF
echo "   ✅ Workspace: test_backlang.code-workspace"

echo ""
echo "✅ Préparation terminée!"
echo ""
echo "📋 Instructions :"
echo "   1. Ouvrez VS Code: code /home/surgeo/backlang-rust"
echo "   2. Si l'extension n'est pas installée:"
echo "      code --install-extension ./vscode-extension/backlang-debug-1.0.0.vsix"
echo "   3. Rechargez VS Code: Ctrl+Shift+P → 'Developer: Reload Window'"
echo "   4. Ouvrez un fichier .bl (ex: test_simple.bl)"
echo "   5. Le bouton ▶️ devrait apparaître dans la barre de titre"
echo "   6. Ctrl+Shift+R pour exécuter"
echo "   7. Ctrl+Shift+D pour déboguer"
echo ""
echo "🔍 Pour vérifier si le bouton apparaît:"
echo "   - Ouvrez test_simple.bl"
echo "   - Regardez en haut à droite de l'éditeur (barre de titre)"
echo "   - Vous devriez voir ▶️ et 🐛"
