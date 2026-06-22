# BackLang

A fast, embeddable scripting language with built-in SQLite, HTTP server, and a full VS Code debugger.

[![Docs](https://img.shields.io/badge/docs-surgeodev.github.io/BackLang-blue)](https://surgeodev.github.io/BackLang)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Quick Install

**Linux / macOS** (one line):
```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

**Windows** (CMD — curl built-in):
```cmd
curl -fsSLo %TEMP%\install.bat https://raw.githubusercontent.com/surgeodev/BackLang/main/install.bat && %TEMP%\install.bat
```

**Or download binaries directly:** [surgeodev.github.io/BackLang/#/Downloads](https://surgeodev.github.io/BackLang/#/Downloads)

---

## Quick Example

```bl
// server.bl — REST API with SQLite
import "std.db"
let DB = "/tmp/app.db"
db.open(DB)
db.execute(DB, "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT)")

server app { port: 8080; cors: true }

endpoint GET "/api/items" {
    return {status: 200, body: db.query(DB, "SELECT * FROM items")}
}

endpoint POST "/api/items" {
    db.execute(DB, "INSERT INTO items (name) VALUES ('" + req.body.name + "')")
    return {status: 201, body: {ok: true}}
}
```

Run: `bl server.bl` → `curl http://localhost:8080/api/items`

## Features

- **HTTP Server** — Built-in Axum server with CORS, path params, middlewares
- **SQLite** — Full SQLite3 support (`db.open`, `db.query`, `db.execute`)
- **Standard Library** — os, fs, math, string, random modules
- **VS Code Extension** — Syntax highlighting, completions, hover, inlay hints, folding, symbols, rename, references, diagnostics, snippets, CodeLens
- **Interactive Debugger** — Breakpoints, step over/in/out, continue
- **Fast** — Compiled Rust interpreter with instant startup
- **Simple** — C-like syntax, dynamic typing, zero dependencies at runtime

## Documentation

Full documentation: **[surgeodev.github.io/BackLang](https://surgeodev.github.io/BackLang)**

## Usage

```
bl file.bl              Execute a program
bl --check file.bl      Syntax check only
bl --debug file.bl      Debug mode (for VS Code debugger)
```

## VS Code Extension

```bash
code --install-extension vscode-extension/backlang-debug-1.0.0.vsix
```

## License

MIT — see [LICENSE](LICENSE)
