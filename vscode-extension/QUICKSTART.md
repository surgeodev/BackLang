# Guide de Démarrage Rapide - Extension BackLang

## Installation et Configuration (5 minutes)

### 1. Compiler l'Extension

```bash
cd vscode-extension
npm install
npm run compile
```

### 2. Tester l'Extension

Appuyer sur `F5` pour ouvrir une fenêtre VS Code de développement avec l'extension.

### 3. Créer un Fichier de Test

Créer un fichier `test.bl`:

```backlang
let x = 10
let y = 20
print("Hello BackLang!")
print(x + y)
```

## Utilisation de Base

### 🚀 Exécuter le Fichier

- `Ctrl+Shift+R` pour exécuter le fichier
- Le résultat s'affiche dans le terminal

### 🐛 Déboguer le Fichier

- `Ctrl+Shift+D` pour lancer le débogage
- Ajouter des breakpoints en cliquant à gauche des numéros de lignes
- Utiliser F10, F11, F5 pour naviguer

### 💻 Créer une Base HTML5

- `Ctrl+Shift+P` → `!bl` → Entrer un nom
- Un fichier HTML5 moderne sera créé

## Exemple Complet

### 1. Créer `app.bl`

```backlang
// Programme BackLang simple
let name = "BackLang"
let items = [1, 2, 3, 4, 5]

function double(n: num) {
    return n * 2
}

print("Application: " + name)
print("Items: " + items)

let doubled = double(10)
print("Double de 10: " + doubled)

if doubled > 10 {
    print("La valeur est grande!")
}
```

### 2. Lancer le Débogage

- Ouvrir `app.bl` dans VS Code
- Appuyer sur `Ctrl+Shift+D`
- Ajouter des breakpoints aux lignes intéressantes
- Appuyer sur F5 pour continuer, F10 pour étape suivante

### 3. Générer HTML5

- Commande `!bl` → `app`
- Un fichier `app.html` est créé avec design moderne

## Commandes Disponibles

| Commande | Raccourci              | Fonction             |
| -------- | ---------------------- | -------------------- |
| Debug    | `Ctrl+Shift+D`         | Déboguer le fichier  |
| Run      | `Ctrl+Shift+R`         | Exécuter le fichier  |
| HTML5    | `Ctrl+Shift+P` + `!bl` | Créer une base HTML5 |

## Structure des Fichiers BackLang

```
Variables:
let x = 10
let name = "test"

Tableaux:
let arr = [1, 2, 3]

Objets:
let obj = {name: "test", value: 42}

Fonctions:
function add(a, b) {
    return a + b
}

Contrôle:
if x > 5 { ... }
while x < 10 { ... }
for i in range(10) { ... }

Affichage:
print("Hello")
println("Hello with newline")
```

## Troubleshooting

### L'extension ne s'active pas

- Vérifier que le fichier a l'extension `.bl`
- Recharger VS Code (Ctrl+Shift+P → "Reload Window")

### Les breakpoints ne s'activent pas

- Vérifier que le débogage est lancé (devrait afficher la barre de débogage)
- Essayer de fermer et réouvrir le fichier

### Le HTML5 ne s'ouvre pas

- Vérifier que le workspace folder est défini
- Essayer avec un nom de fichier simple (ex: "index")

## Prochaines Étapes

1. **Lire la documentation complète**: Voir [README.md](./README.md)
2. **Explorez les exemples**: Essayer différentes fonctionnalités
3. **Contribuez**: Les contributions sont les bienvenues!

## Ressources

- [GitHub Repository](https://github.com/backlang/vscode-extension)
- [Documentation BackLang](https://backlang.dev/docs)
- [VS Code Extension API](https://code.visualstudio.com/api)

---

**Bienvenue dans BackLang! 🚀**
