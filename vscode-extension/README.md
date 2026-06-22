# BackLang for VS Code

[![Visual Studio Marketplace Version](https://img.shields.io/visual-studio-marketplace/v/surgeodev.backlang-debug)](https://marketplace.visualstudio.com/items?itemName=surgeodev.backlang-debug)

Full language support and debugger for [BackLang](https://github.com/surgeodev/BackLang) (.bl files).

## Features

### 🎨 Syntax Highlighting
Complete syntax coloring for `.bl` files — keywords, strings, numbers, comments, and built-in functions.

### 🐛 Interactive Debugger
Set breakpoints, step over/in/out, inspect variables, and view the call stack — all inside VS Code.

| Action | Windows/Linux | Mac |
|--------|---------------|-----|
| Start Debug | `Ctrl+Shift+D` | `Cmd+Shift+D` |
| Run File | `Ctrl+Shift+R` | `Cmd+Shift+R` |
| Continue | `F5` | `F5` |
| Step Over | `F10` | `F10` |
| Step Into | `F11` | `F11` |
| Step Out | `Shift+F11` | `Shift+F11` |

### ✨ Language Features
- **Code completion** — suggest keywords, built-in functions, variable names
- **Hover info** — type and value information on hover
- **Inlay hints** — see inferred types inline after `let` and `function`
- **Folding** — collapse blocks, functions, servers, comments
- **Document symbols** — navigate via breadcrumbs and outline (endpoints, functions, server, vars)
- **Rename** (F2) — rename variables across the file
- **Find references** (Shift+F12) — find all references to a variable
- **Diagnostics** — real-time errors on save (parsed via `bl --check`)
- **CodeLens** — Run / Start Server buttons above endpoints and server blocks
- **Snippets** — 12 shortcuts for `let`, `function`, `if`, `for`, `while`, `server`, `endpoint`, `import`, and more

### 🚀 Commands
| Command | Description |
|---------|-------------|
| `BackLang: Debug File` | Debug the current `.bl` file |
| `BackLang: Run File` | Execute the current `.bl` file |
| `BackLang: Build` | Build `bl` binary from source |
| `BackLang: Open REPL` | Start interactive BackLang session |
| `!bl - Create Backend Base` | Generate a `.lb` backend scaffold |
| `!bl - Create HTML5 Base` | Generate a modern HTML5 page |

## Requirements

- VS Code 1.70+
- The `bl` binary installed (run `BackLang: Build` or install via [backlang.dev](https://surgeodev.github.io/BackLang))

## Extension Settings

This extension contributes the following settings:

* `backlang.blPath`: Path to the `bl` binary (auto-detected from PATH)
* `backlang.autoRunCheck`: Run `bl --check` on save (default: `true`)

## Release Notes

### 1.0.0

- Initial release: syntax highlighting, debugger, language server features, snippets, commands

## Repository

[github.com/surgeodev/BackLang](https://github.com/surgeodev/BackLang)
