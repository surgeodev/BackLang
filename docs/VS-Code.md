# VS Code Extension

The BackLang VS Code extension provides a complete development environment with syntax highlighting, code intelligence, and an interactive debugger.

## Installation

```bash
code --install-extension vscode-extension/backlang-debug-1.0.0.vsix
```

Or via the one-line installer:

```bash
curl -fsSL https://raw.githubusercontent.com/surgeodev/BackLang/main/install.sh | bash
```

## Features

### 1. Syntax Highlighting

Full syntax highlighting for `.bl` and `.lb` files, including:
- Keywords (`let`, `const`, `function`, `server`, `endpoint`, `if`, `while`, `for`, etc.)
- HTTP methods (`GET`, `POST`, `PUT`, `DELETE`)
- Strings, numbers, booleans, null
- Comments (`//` and `/* */`)
- Special fields (`status`, `body`, `cors`, `port`)

### 2. Code Completion

Automatic suggestions as you type:
- **Keywords**: `let`, `const`, `if`, `while`, `for`, `function`, `return`, `server`, `endpoint`, `import`, etc.
- **Built-in functions**: `print`, `len`, `push`, `pop`, `keys`, `str`, `num`, `type`
- **Database functions**: `db.open`, `db.query`, `db.execute`, `db_open`, `db_query`, `db_execute`
- **Variables**: all `let`/`const` declarations in the current file
- **Functions**: all `function` declarations in the current file
- **Routes**: all endpoint paths in the current file

### 3. Hover Documentation

Hover over any keyword or built-in function to see its documentation:

```
`db.open(path)` — Open SQLite database connection
`len(val)` → length of string or array
`return` — `return {status: N, body: …}` — HTTP response
```

### 4. Inlay Hints

Shows inferred type information inline:

```bl
function add(a: Num, b: Num) → any     // return type hint
let x = "hello" : string               // inferred type
let count = 42 : num                   // inferred type
```

### 5. CodeLens

Inline action buttons above key structures:

- `▶ Run` — on `endpoint`, `function`, and standalone expressions
- `▶ Start Server` — on `server` declarations

### 6. Folding

Collapsible regions for:
- Curly brace blocks (`{}`)
- Comment regions (`// ──` sections)

### 7. Document Symbols (Breadcrumbs)

The outline and breadcrumb navigation show:
- `endpoint GET "/path"` — as functions
- `function name` — as functions  
- `server name` — as modules
- `let name` / `const name` — as variables

### 8. Rename (F2)

Rename variables and functions across the entire file with a single keypress.

### 9. Find References (Shift+F12)

Find all usages of any variable or function in the current file.

### 10. Real-Time Diagnostics

The extension runs `bl --check` every time you save a `.bl` file. Errors and warnings appear in the Problems panel and as squiggly underlines in the editor.

### 11. Code Snippets

12 snippets for rapid development:

| Prefix | Content |
|--------|---------|
| `server` | Server with CORS |
| `endpoint` | HTTP endpoint |
| `db.open` | SQLite open |
| `db.query` | SQLite query |
| `db.execute` | SQLite execute |
| `if` | If statement |
| `ifelse` | If/else |
| `while` | While loop |
| `for` | For-in loop |
| `function` | Function definition |
| `return` | Return value |
| `returnstatus` | Return with HTTP status |

### 12. Commands

Available commands in the command palette:

| Command | Keybind | Description |
|---------|---------|-------------|
| `BackLang: Run` | `Ctrl+F5` | Run the current file |
| `BackLang: Debug` | `Ctrl+Shift+D` | Start debugging |
| `BackLang: Build` | — | Run `cargo build --release` |
| `BackLang: Open REPL` | — | Open a terminal REPL |
| `!bl - Create Backend Base` | — | Generate a starter server |
| `!lb - Create HTML5 Base` | — | Generate an HTML template |

### 13. Status Bar

Shows `BackLang ✓` when the binary is found or `BackLang ✗` when the project needs to be built. Click to trigger a build.

## Debugger

BackLang has a **full interactive debugger** integrated into VS Code.

### Starting a Debug Session

1. Open a `.bl` file
2. Set breakpoints by clicking the gutter (left of line numbers)
3. Press `Ctrl+Shift+D` or click the ▶ Debug button in the editor title bar

### Debug Features

| Feature | Supported |
|---------|-----------|
| Breakpoints | ✅ Line breakpoints |
| Step Over (Next) | ✅ Skip function calls |
| Step In | ✅ Enter function calls |
| Step Out | ✅ Return from current function |
| Continue | ✅ Run to next breakpoint |
| Pause | ✅ (basic) |
| Variables | ✅ (locals scope) |
| Call Stack | ✅ (single thread) |
| Set Value | ❌ |

### Debug Protocol

The debugger uses stdin/stdout JSON-LD communication between the Node debug adapter and the Rust `bl` process:

1. VS Code sends breakpoints to the debug adapter (DAP)
2. Debug adapter spawns `bl --debug file.bl`
3. Debug adapter sends breakpoints via stdin to the Rust process
4. Rust process pauses at breakpoints, sends `paused` events on stderr
5. Debug adapter forwards user commands (continue, step) via stdin
6. Rust process resumes and emits `continued` events

### Debug Configuration

When you first debug, VS Code may ask for a launch configuration. Default configurations are provided:

```json
{
    "name": "▶️ Run BackLang File",
    "type": "backlang",
    "request": "launch",
    "program": "${file}",
    "stopOnEntry": false
}
```

Set `stopOnEntry: true` to pause immediately when the program starts.
