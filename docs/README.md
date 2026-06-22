# BackLang

**BackLang** is a fast, embeddable scripting language with first-class SQLite support, a built-in HTTP server, and a rich VS Code extension with a full interactive debugger.

```
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

## Design Principles

- **Fast**: Compiled Rust interpreter — instant startup, no VM overhead.
- **Practical**: SQLite, HTTP, JSON, file I/O — all built-in, no dependencies.
- **Toolable**: Full VS Code extension with completions, hover docs, inlay hints, folding, symbols, rename, references, diagnostics, and a step debugger.
- **Simple**: Familiar C-style syntax, minimal keywords, no macros, no generics.

## Feature Overview

| Feature | Status |
|---------|--------|
| C-style syntax (let, const, if, while, for, function) | ✅ |
| Dynamic typing (null, bool, num, str, array, object, function) | ✅ |
| Assignment (+=, -=, *=, /=) | ✅ |
| Comparison (==, !=, <, >, <=, >=) | ✅ |
| Logic (&&, \|\|, !) | ✅ |
| Object/array indexing and member access | ✅ |
| Functions with type annotations | ✅ |
| **HTTP server** with CORS | ✅ |
| **SQLite** (db.open, db.query, db.execute) | ✅ |
| **Standard library** (os, fs, math, string, random) | ✅ |
| **VS Code extension** (completions, hover, inlay hints, folding, symbols, rename, references, diagnostics, snippets, CodeLens) | ✅ |
| **Interactive debugger** (breakpoints, step over/in/out, continue) | ✅ |
| **CLI** (run, --check, --debug) | ✅ |
| Module/import system | ✅ |
| Middleware pipeline for HTTP | ✅ |
| Error handling via `Result` return | ❌ (planned) |
| Package manager (`bl-pkg`) | ❌ (planned) |

## Project Structure

```
BackLang/
├── src/              # Rust source
│   ├── main.rs       # CLI entry point
│   ├── lexer.rs      # Tokenizer
│   ├── parser.rs     # AST builder
│   ├── interpreter.rs# Tree-walking interpreter
│   ├── debugger.rs   # Debug protocol (stdin/stdout JSON-LD)
│   └── server.rs     # Axum HTTP server
├── std/              # Standard library (.bl files)
├── docs/             # Documentation site (GitHub Pages)
└── vscode-extension/ # VS Code extension
```

## Quick Links

- **[Installation](/#/Installation)**
- **[Language Reference](/#/Language)**
- **[Standard Library](/#/Standard-Library)**
- **[HTTP Server Guide](/#/HTTP-Server)**
- **[VS Code Extension](/#/VS-Code)**
- **[Examples](/#/Examples)**
- **[GitHub Repository](https://github.com/surgeodev/BackLang)**
