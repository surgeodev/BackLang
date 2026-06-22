# BackLang Debug Extension

Une extension VS Code puissante pour le développement, le débogage et l'extension des fichiers BackLang (.bl).

## Fonctionnalités

### 🎨 Coloration Syntaxique

- Coloration complète pour les fichiers `.bl`
- Support des mots-clés: `let`, `function`, `if`, `while`, `for`, etc.
- Coloration des chaînes, nombres et commentaires
- Support des fonctions natives: `print`, `push`, `pop`, `map`, `filter`, etc.

### 🐛 Débogage Avancé

- Débogage en temps réel des fichiers BackLang
- Points d'arrêt (breakpoints)
- Navigation par étapes (step, step into, step out)
- Inspection des variables et scope
- Pile d'appels (stack trace)

### 🚀 Commandes Utiles

#### Debug

```
Ctrl+Shift+D (Windows/Linux) / Cmd+Shift+D (Mac)
```

Lance le débogage du fichier BackLang actif avec points d'arrêt.

#### Exécution

```
Ctrl+Shift+R (Windows/Linux) / Cmd+Shift+R (Mac)
```

Exécute le fichier BackLang dans un terminal.

#### Créer une Base HTML5

```
Commande: !bl - Create HTML5 Base
```

Génère une page HTML5 moderne et prête à l'emploi avec:

- Design responsive avec gradient
- Styles modernes et professionnels
- Intégration JavaScript
- Boutons interactifs
- Footer avec information BackLang

### 📋 Validation Syntaxique

L'extension valide automatiquement:

- ✅ Appariement des accolades `{}`
- ✅ Appariement des crochets `[]`
- ✅ Appariement des parenthèses `()`
- ✅ Noms de variables valides
- ✅ Déclaration des fonctions

### 💡 Informations au Survol (Hover)

Survolez les variables et fonctions pour obtenir des informations contextuelles.

### ⚡ Intégration Complète

- Support des points d'arrêt conditionnels
- Consoles de débogage intégrée
- Évaluation d'expressions
- Inspection des structures de données

## Installation

### Prerequisites

- VS Code 1.70.0 ou supérieur
- Node.js 16+ pour compiler l'extension

### Installation de l'Extension

1. **Depuis VS Code Marketplace** (une fois publiée)

   ```
   Ouvrir Extensions (Ctrl+Shift+X)
   Rechercher "BackLang Debug"
   Cliquer "Install"
   ```

2. **Installation Locale**
   ```bash
   cd vscode-extension
   npm install
   npm run compile
   vsce package
   code --install-extension backlang-debug-1.0.0.vsix
   ```

## Configuration

### Fichier de Lancement .vscode/launch.json

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "BackLang Debug",
      "type": "backlang",
      "request": "launch",
      "program": "${workspaceFolder}/${relativeFile}",
      "stopOnEntry": true
    }
  ]
}
```

## Utilisation

### Exemple: Créer et Déboguer un Programme BackLang

1. **Créer un fichier** `hello.bl`:

```backlang
let name = "BackLang"
let version = 1.0

function greet(msg: string) {
    print("Hello, " + msg)
}

greet(name)
print("Version: " + version)
```

2. **Ajouter des Points d'Arrêt**
   - Cliquer sur la numérotation des lignes à gauche de l'éditeur
   - Des points rouges apparaissent

3. **Lancer le Débogage**
   - Appuyer sur `Ctrl+Shift+D` (ou cmd+shift+D sur Mac)
   - Le débogueur démarre et s'arrête au premier point d'arrêt

4. **Utiliser les Contrôles de Débogage**
   - Continue (F5)
   - Step Over (F10)
   - Step Into (F11)
   - Step Out (Shift+F11)
   - Pause (Ctrl+Pause)

### Générer une Base HTML5

1. Ouvrir la palette de commandes (`Ctrl+Shift+P`)
2. Taper `!bl` ou chercher "Create HTML5 Base"
3. Entrer le nom du fichier (ex: "index")
4. Un fichier `index.html` est créé et ouvert

## Structure de l'Extension

```
vscode-extension/
├── src/
│   ├── extension.ts          # Point d'entrée principal
│   └── debugAdapter.ts       # Adaptateur de débogage
├── syntaxes/
│   └── backlang.tmLanguage.json  # Coloration syntaxique
├── media/                    # Ressources (icônes, images)
├── package.json              # Configuration de l'extension
├── language-configuration.json
├── tsconfig.json
└── README.md
```

## Compilation et Développement

### Compilation

```bash
cd vscode-extension
npm install
npm run compile
```

### Watch Mode (Recompilation Automatique)

```bash
npm run watch
```

### Tests

```bash
npm run test
```

### Linting

```bash
npm run lint
```

### Packaging

```bash
npm install -g vsce
vsce package
```

## Dépannage

### L'extension ne se charge pas

- Vérifier que le fichier est un `.bl`
- Recharger VS Code (`Ctrl+Shift+P` > "Developer: Reload Window")
- Vérifier les logs: Output > BackLang Debug

### Les points d'arrêt ne fonctionnent pas

- S'assurer que le binaire `bl` est accessible
- Vérifier le chemin dans `launch.json`
- Consulter la console de débogage pour les erreurs

### Erreurs de syntaxe non détectées

- L'extension valide les erreurs basiques
- Pour une validation complète, utiliser le compilateur BackLang directement
- Consulter la palette de commandes pour les diagnostics

## Raccourcis Clavier

| Action    | Windows/Linux  | Mac           |
| --------- | -------------- | ------------- |
| Debug     | `Ctrl+Shift+D` | `Cmd+Shift+D` |
| Run       | `Ctrl+Shift+R` | `Cmd+Shift+R` |
| Continue  | `F5`           | `F5`          |
| Step Over | `F10`          | `F10`         |
| Step Into | `F11`          | `F11`         |
| Step Out  | `Shift+F11`    | `Shift+F11`   |
| Pause     | `Ctrl+Pause`   | `Cmd+Pause`   |

## API de Débogage

### Commandes Disponibles

- `backlang.debug` - Déboguer le fichier courant
- `backlang.run` - Exécuter le fichier courant
- `backlang.createHtml5Base` - Créer une base HTML5

### Événements de Débogage

- `initialized` - Débogueur initialisé
- `stopped` - Exécution arrêtée (breakpoint, step, pause)
- `terminated` - Débogage terminé

## Contribution

Les contributions sont les bienvenues! Pour contribuer:

1. Fork le repository
2. Créer une branche feature (`git checkout -b feature/amazing-feature`)
3. Commit les changements (`git commit -m 'Add amazing feature'`)
4. Push vers la branche (`git push origin feature/amazing-feature`)
5. Ouvrir une Pull Request

## License

MIT License - Voir LICENSE pour plus de détails

## Support

Pour les problèmes ou suggestions:

- 📧 Email: support@backlang.dev
- 🐛 Issues: https://github.com/backlang/vscode-extension/issues
- 💬 Discussions: https://github.com/backlang/discussions

## Changelog

### v1.0.0 (Initial Release)

- ✨ Coloration syntaxique complète
- 🐛 Débogage avec breakpoints
- 🚀 Commande HTML5 !bl
- 📝 Validation syntaxique
- 💡 Hover information
- ⚡ Support multi-fichier

---

**Profitez du développement avec BackLang!** 🎉
