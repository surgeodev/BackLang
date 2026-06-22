# BackLang

> High-performance scripting language with zero-overhead embedding, async HTTP runtime, and native SQLite engine.

**2.4 MB binary. Zero dependencies. 2ms startup. 120k req/s.**

Not a toy. Not a teaching language. BackLang is a production-grade scripting runtime for backend services, APIs, and embedded systems — powered by Rust.

## Why not Python / Node / Lua?

| | BackLang | Python | Node.js | Lua |
|---|---|---|---|---|
| **Binary size** | **2.4 MB** | ~40 MB + env | ~200 MB node_modules | ~5 MB |
| **Startup** | **2 ms** | ~50 ms | ~300 ms | ~1 ms |
| **HTTP throughput** | **~120k req/s** | ~15k (uvicorn) | ~40k (fastify) | ~20k (lapis) |
| **Embedding** | `extern "C"` ABI | CPython C API | napi-rs | C API |
| **SQLite** | Bundled, zero-copy | External pkg | better-sqlite3 | Bundled |

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

```bat
REM Windows
curl -fsSLo %TEMP%\install.bat https://raw.githubusercontent.com/surgeodev/BackLang/main/install.bat && %TEMP%\install.bat
```

## Quick Example

```bl
// REST API + SQLite — 20 lines, no dependencies
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

```
bl server.bl          # starts HTTP server
curl localhost:8080/api/items  # GET → JSON array
```

## Features

- **Rust engine** — Tree-walking interpreter, safe Rust, zero GC pauses, no JIT warmup
- **HTTP server** — Axum/tokio async runtime, CORS, path params, middleware, JSON body parsing
- **SQLite** — Bundled with FTS5, JSON1, R*Tree, zero-copy result rows
- **Standard library** — os, fs, math, string, random, db modules
- **Debugger** — DAP protocol: breakpoints, step over/in/out, stack trace, variable inspection
- **CLI** — `bl --check`, `bl --debug`, `bl --update`, `bl --install`
- **VS Code extension** — 13 features: syntax, completions, hover, inlay hints, folding, symbols, rename, references, diagnostics, CodeLens, snippets, debugger, templates
- **Self-contained** — One binary, zero deps. Copy it anywhere, it runs.

## Benchmarks

See the [Benchmarks](Benchmarks) page for full results and methodology.

## Docs

- [Installation](Installation)
- [Quick Start](Quick-Start)
- [Language Reference](Language)
- [Standard Library](Standard-Library)
- [HTTP Server](HTTP-Server)
- [VS Code Extension](VS-Code)
- [CLI Reference](CLI)
- [Benchmarks](Benchmarks)
- [Downloads](Downloads)
