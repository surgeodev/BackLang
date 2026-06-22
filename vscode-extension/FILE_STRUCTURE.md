# BackLang Extension - Quick Reference

## 📍 Where to Find Everything

### Extension Files

- **Main Code**: [src/extension.ts](src/extension.ts)
- **Debug Adapter**: [src/debugAdapter.ts](src/debugAdapter.ts)
- **Syntax Rules**: [syntaxes/backlang.tmLanguage.json](syntaxes/backlang.tmLanguage.json)

### Configuration

- **Extension Config**: [package.json](package.json)
- **TypeScript Config**: [tsconfig.json](tsconfig.json)
- **Language Config**: [language-configuration.json](language-configuration.json)
- **Debug Config**: [.vscode/launch.json](.vscode/launch.json)

### Documentation

- **Full Guide**: [README.md](README.md)
- **Quick Start**: [QUICKSTART.md](QUICKSTART.md)
- **Testing**: [TESTING.md](TESTING.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)

### Examples

- **Basic Demo**: [examples/demo.bl](examples/demo.bl)
- **Server Example**: [examples/server_example.bl](examples/server_example.bl)

## 🎯 Common Tasks

### Setup & Build

```bash
npm install              # Install dependencies
npm run compile          # Build extension
npm run watch           # Watch for changes
```

### Development

```bash
# Press F5 in VS Code to test extension
# Open a .bl file to test features
```

### Testing

```bash
npm run test            # Run tests
npm run lint            # Check code style
```

### Packaging

```bash
npm install -g vsce     # Install packaging tool
vsce package            # Create .vsix file
```

### Publishing

```bash
chmod +x publish.sh
./publish.sh            # Publish to marketplace
```

## 🔍 Main Features Quick Links

| Feature             | How to Use             | File                                                          |
| ------------------- | ---------------------- | ------------------------------------------------------------- |
| Syntax Highlighting | Open `.bl` file        | [backlang.tmLanguage.json](syntaxes/backlang.tmLanguage.json) |
| Debug               | `Ctrl+Shift+D`         | [extension.ts](src/extension.ts)                              |
| Run File            | `Ctrl+Shift+R`         | [extension.ts](src/extension.ts)                              |
| HTML5 Template      | `Ctrl+Shift+P` → `!bl` | [extension.ts](src/extension.ts)                              |
| Validation          | Auto on save           | [extension.ts](src/extension.ts)                              |

## 📦 Dependencies

- `@vscode/debugadapter` - Debug protocol
- `@vscode/debugprotocol` - Debug messages
- `typescript` - Language compilation

## 🚀 First Time Users

1. Read [QUICKSTART.md](QUICKSTART.md) - 5 minute overview
2. Run `npm install && npm run compile`
3. Press `F5` to test in VS Code
4. Open [examples/demo.bl](examples/demo.bl)
5. Try all the features!

## 💡 Development Tips

- **Make changes**: Edit files in `src/`
- **Recompile**: `npm run compile` or `npm run watch`
- **Test**: Press `Ctrl+Shift+P` → "Reload Window"
- **Debug**: Check Output panel for errors
- **Format**: `npm run lint` to check code

## 🎨 Customization

### Add Syntax Highlighting

Edit: [syntaxes/backlang.tmLanguage.json](syntaxes/backlang.tmLanguage.json)

### Add Commands

Edit: [src/extension.ts](src/extension.ts) and [package.json](package.json)

### Change HTML5 Template

Edit: `generateHtml5Base()` function in [src/extension.ts](src/extension.ts)

## 📞 Need Help?

1. Check [README.md](README.md) for full documentation
2. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
3. Review [TESTING.md](TESTING.md) for test procedures
4. Open an issue on GitHub

## 🎯 Next Steps

- [ ] Read the documentation
- [ ] Install dependencies
- [ ] Compile the extension
- [ ] Test in VS Code
- [ ] Try example programs
- [ ] Customize for your needs

---

**Happy coding! 🚀**
