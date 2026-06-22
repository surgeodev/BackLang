# CHANGELOG

All notable changes to the BackLang VS Code Debug Extension will be documented in this file.

## [1.0.0] - 2025-04-28

### Added

- ✨ Initial release of BackLang Debug Extension
- 🎨 Complete syntax highlighting for .bl files
  - Keywords: let, function, if, while, for, etc.
  - Built-in functions: print, push, pop, map, filter, etc.
  - String, number, and comment highlighting
- 🐛 Debug Support
  - Breakpoint management
  - Step over, step into, step out navigation
  - Call stack inspection
  - Variable and scope inspection
  - Debug console output
- 🚀 Command Features
  - `backlang.debug` - Debug current file (Ctrl+Shift+D)
  - `backlang.run` - Run current file (Ctrl+Shift+R)
  - `backlang.createHtml5Base` - Create HTML5 template (!bl command)
- 📝 Code Quality
  - Real-time syntax validation
  - Bracket/brace/parenthesis matching
  - Variable name validation
  - Function declaration checking
- 💡 Developer Experience
  - Hover information for variables and functions
  - Code actions for quick fixes
  - Auto-completion support
  - Proper indentation handling
- 📋 Documentation
  - Comprehensive README
  - Quick Start Guide
  - Example programs
  - Installation guide

### Features Details

#### Syntax Highlighting

- Keywords (control flow, declarations)
- Data types and literals
- Comments (line and block)
- String interpolation
- Operators

#### Debugging Features

- Line breakpoints
- Conditional breakpoints (planned for v1.1)
- Multi-threaded debugging
- Debug console evaluation
- Variable introspection

#### HTML5 Template

Modern, responsive HTML5 base with:

- CSS Grid layout
- Responsive design
- Modern styling
- Interactive buttons
- JavaScript integration

### Known Limitations

- Conditional breakpoints not yet supported
- Remote debugging not available
- Limited expression evaluation in console

### Dependencies

- @vscode/debugadapter: ^1.58.0
- @vscode/debugprotocol: ^1.58.0
- TypeScript: ^4.7.0

---

## [Upcoming]

### v1.1.0 - Conditional Breakpoints

- Conditional breakpoint support
- Logpoints for debugging without breakpoints
- Better variable inspection

### v1.2.0 - Code Formatting

- BackLang code formatter
- Linter integration
- Automatic formatting options

### v2.0.0 - Language Server

- Full Language Server Protocol (LSP) support
- IntelliSense and autocomplete
- Go to definition and references
- Refactoring support

---

## Version History

### Development Build

- All features from v1.0.0
- Bug fixes and improvements
- Testing updates

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

MIT - See LICENSE file for details
