# BackLang

A fast, embeddable scripting language with SQLite support and HTTP server capabilities.

📖 Documentation complète : https://surgeodev.github.io/BackLang

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

## Usage

```bash
bl --check file.bl        # syntax check
bl file.bl                # run
bl --debug file.bl        # debug (VS Code)
```

## Features

- SQLite persistence (`db.open`, `db.query`, `db.execute`)
- HTTP server with CORS (`server`, `endpoint GET/POST/...`)
- Rich VS Code extension (debugger, completions, hover, inlay hints, folding)
- Standard library: `std.os`, `std.random`, `std.math`, `std.fs`, `std.string`
