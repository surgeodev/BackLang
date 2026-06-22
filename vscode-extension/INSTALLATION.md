# 🎉 BackLang VS Code Extension Complete!

Une extension VS Code complète et prête à l'emploi pour BackLang avec débogage, coloration syntaxique et générateur HTML5.

## ✨ Qu'est-ce qui a été créé

Une extension professionnelle VS Code contenant:

### 🎨 Coloration Syntaxique

- Support complet des fichiers `.bl`
- Mise en évidence des mots-clés, fonctions, chaînes
- Indentation automatique
- Support des commentaires

### 🐛 Débogage Intégré

- Points d'arrêt (breakpoints)
- Navigation pas à pas (step, step into, step out)
- Inspection des variables
- Vue de la pile d'appels

### 🚀 Commandes Utiles

- **Debug**: `Ctrl+Shift+D` - Déboguer le fichier courant
- **Run**: `Ctrl+Shift+R` - Exécuter le fichier
- **HTML5**: `Ctrl+Shift+P` → `!bl` - Créer base HTML5

### 📝 Validation Syntaxique

- Détection des erreurs en temps réel
- Validation des accolades, crochets, parenthèses
- Vérification des noms de variables
- Aide au survol (hover)

## 📁 Structure du Projet

```
vscode-extension/
├── src/
│   ├── extension.ts              # Code principal
│   └── debugAdapter.ts           # Adaptateur debug
├── syntaxes/
│   └── backlang.tmLanguage.json  # Coloration syntaxique
├── examples/
│   ├── demo.bl                   # Exemple simple
│   └── server_example.bl         # Exemple serveur
├── .vscode/
│   ├── launch.json               # Config debug
│   ├── settings.json             # Paramètres
│   └── tasks.json                # Tâches de build
├── package.json                  # Manifest
├── tsconfig.json                 # Config TypeScript
├── README.md                     # Documentation complète
├── QUICKSTART.md                 # Guide rapide
├── TESTING.md                    # Procédures de test
├── CHANGELOG.md                  # Historique des versions
├── CONTRIBUTING.md               # Guide de contribution
├── LICENSE                       # MIT License
└── publish.sh                    # Script de publication
```

## 🚀 Installation et Démarrage Rapide

### 1. Installer les Dépendances

```bash
cd vscode-extension
npm install
```

### 2. Compiler l'Extension

```bash
npm run compile
```

### 3. Tester dans VS Code

- Ouvrir le dossier `vscode-extension` dans VS Code
- Appuyer sur `F5` pour lancer une instance de test
- Les fichiers `.bl` auront la coloration et le debug

### Ou utiliser le script d'installation

```bash
# Unix/Linux/Mac
chmod +x install.sh
./install.sh

# Windows
install.bat
```

## 📖 Documentation

- **[README.md](README.md)** - Documentation complète avec toutes les fonctionnalités
- **[QUICKSTART.md](QUICKSTART.md)** - Guide de 5 minutes pour démarrer
- **[TESTING.md](TESTING.md)** - Procédures complètes de test
- **[FILE_STRUCTURE.md](FILE_STRUCTURE.md)** - Référence rapide des fichiers
- **[CHANGELOG.md](CHANGELOG.md)** - Historique des versions
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Guide pour contribuer

## 🎯 Utilisation

### Créer un Fichier BackLang

```backlang
let name = "BackLang"
let version = 1.0

function greet(msg: string) {
    print("Hello, " + msg)
}

greet(name)
```

### Exécuter le Fichier

1. Ouvrir le fichier dans VS Code
2. Appuyer sur `Ctrl+Shift+R`
3. Voir le résultat dans le terminal

### Déboguer le Fichier

1. Ajouter des points d'arrêt (cliquer à gauche)
2. Appuyer sur `Ctrl+Shift+D`
3. Utiliser F10, F11, F5 pour naviguer

### Générer une Base HTML5

1. `Ctrl+Shift+P` → `!bl`
2. Entrer un nom (ex: "index")
3. Un fichier HTML5 moderne est créé

## 🔧 Commandes npm

```bash
npm install              # Installer les dépendances
npm run compile          # Compiler TypeScript
npm run watch           # Surveiller les changements
npm run lint            # Vérifier le code
npm run test            # Exécuter les tests
npm run package         # Créer un package .vsix
```

## 📦 Publier l'Extension

### Créer un Package

```bash
npm install -g vsce
vsce package
```

### Publier sur le Marketplace

```bash
vsce publish
```

Ou utiliser le script de publication:

```bash
chmod +x publish.sh
./publish.sh
```

## 🌟 Fonctionnalités Clés

### ✅ Coloration Syntaxique Complète

- Mots-clés réservés
- Fonctions natives
- Chaînes et nombres
- Commentaires

### ✅ Débogage Puissant

- Breakpoints sur n'importe quelle ligne
- Step over, step into, step out
- Inspection des variables
- Vue de la pile d'appels

### ✅ Générateur HTML5

- Design moderne et responsive
- Gradient CSS
- Boutons interactifs
- JavaScript intégré

### ✅ Validation Syntaxique

- Détection d'erreurs en temps réel
- Messages clairs
- Suggestions de correction

## 💻 Raccourcis Clavier

| Action    | Raccourci      |
| --------- | -------------- |
| Debug     | `Ctrl+Shift+D` |
| Run       | `Ctrl+Shift+R` |
| Continue  | `F5`           |
| Step Over | `F10`          |
| Step Into | `F11`          |
| Step Out  | `Shift+F11`    |

## 🧪 Tester l'Extension

1. Lire [TESTING.md](TESTING.md)
2. Ouvrir `examples/demo.bl`
3. Tester chaque fonctionnalité
4. Ajouter vos propres fichiers `.bl`

## 🐛 Dépannage

### L'extension ne se charge pas

- Vérifier que le fichier a l'extension `.bl`
- Recharger VS Code: `Ctrl+Shift+P` → "Reload Window"

### Les breakpoints ne fonctionnent pas

- Vérifier que le débogage est lancé
- Consulter la console de débogage

### HTML5 ne s'ouvre pas

- Vérifier que le workspace folder existe
- Utiliser un nom de fichier simple

## 📞 Support

- 📧 Email: support@backlang.dev
- 🐛 Issues: GitHub Issues
- 💬 Discussions: GitHub Discussions

## 🤝 Contribuer

Les contributions sont les bienvenues! Voir [CONTRIBUTING.md](CONTRIBUTING.md) pour:

- Signaler les bugs
- Suggérer des fonctionnalités
- Soumettre des pull requests

## 📄 License

MIT License - Gratuit pour un usage personnel et commercial

## 🎉 Prochaines Étapes

1. **Lire la documentation**: [README.md](README.md)
2. **Installer les dépendances**: `npm install`
3. **Compiler**: `npm run compile`
4. **Tester**: Appuyer sur `F5` dans VS Code
5. **Explorer**: Ouvrir `examples/demo.bl`
6. **Personnaliser**: Adapter pour vos besoins

## 📊 Statistiques du Projet

- **Fichiers Source**: 2 (extension.ts, debugAdapter.ts)
- **Fichiers de Configuration**: 5+
- **Fichiers de Documentation**: 7+
- **Fichiers d'Exemple**: 2
- **Dépendances**: 3 principales
- **Lignes de Code**: 1000+

## 🌐 Intégration avec BackLang Rust

Cette extension VS Code s'intègre parfaitement avec le projet Rust BackLang:

- ✅ Support du binaire `bl` du compilateur Rust
- ✅ Débogage des fichiers `.bl`
- ✅ Exécution via le terminal
- ✅ Validation compatible avec le compilateur

## 🎓 Pour Apprendre

Consultez ces ressources:

- Documentation VS Code: https://code.visualstudio.com/api
- BackLang Documentation: https://backlang.dev
- Extension Examples: `examples/` folder

## 🎊 Résumé

Vous avez maintenant une **extension VS Code complète et professionnelle** pour BackLang avec:

✅ Coloration syntaxique  
✅ Débogage intégré  
✅ Générateur HTML5  
✅ Documentation complète  
✅ Prête pour la publication

**Profitez du développement avec BackLang! 🚀**

---

Pour toute question ou problème, consultez la documentation ou ouvrez un issue sur GitHub.
