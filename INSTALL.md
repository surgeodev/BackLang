# 🚀 BackLang - Installation Simplifiée

Installation complète en **1 commande**!

## ⚡ Installation Rapide

### Mac / Linux

```bash
bash install.sh
```

### Windows

```bash
install.bat
```

C'est tout! L'installateur va:

✅ Compiler le compilateur BackLang  
✅ Installer l'extension VS Code  
✅ Configurer tout automatiquement

## 🎯 Après Installation

Ouvrez VS Code et créez un fichier `hello.bl`:

```backlang
print("Hello BackLang!")

let x = 10
let y = 20
print(x + y)
```

Puis:

- **Exécuter**: `Ctrl+Shift+R`
- **Déboguer**: `Ctrl+Shift+D`
- **Créer HTML5**: `Ctrl+Shift+P` → `!bl`

## 📋 Configuration Requise

- **Rust**: Installé automatiquement si absent
- **VS Code**: Recommandé (détecté automatiquement)
- **Node.js**: Optionnel (pour modifier l'extension)

## 🧪 Tester

```bash
# Depuis n'importe où
bl hello.bl

# Ou avec le chemin complet
./target/release/bl hello.bl
```

## 📞 Support

- 📖 [Documentation Complète](./vscode-extension/README.md)
- 🐛 [Signaler un Bug](./vscode-extension/CONTRIBUTING.md)
- 💬 Créer une issue GitHub

## 📄 License

MIT - Gratuit pour tous!

---

**C'est prêt! Exécutez l'installateur et codez en BackLang! 🎉**
