# CLI Reference

The `bl` binary is the BackLang compiler and runtime.

## Usage

```bash
bl <file.bl>                # Execute a BackLang program
bl --check <file.bl>        # Parse only, no execution
bl --debug <file.bl>        # Debug mode (used by VS Code debug adapter)
```

## Commands

### `bl <file.bl>`

Parses and executes the given BackLang source file. If the program declares a `server`, execution blocks and serves HTTP requests until the process is interrupted.

```
bl server.bl
```

### `bl --check <file.bl>`

Parses the file and validates syntax without executing any code. Prints `OK` on success or an error message on failure. Used internally by the VS Code extension for real-time diagnostics.

```
bl --check server.bl
# → OK
```

Exit code is `0` on success, non-zero on error.

### `bl --debug <file.bl>`

Starts the program in debug mode. The process communicates with the VS Code debug adapter via stdin/stdout JSON-LD protocol:

**Input (stdin):** Commands from the debug adapter
- `setBreakpoints <file> <line1> <line2> ...` — Set breakpoints
- `start` — Begin execution
- `continue` / `c` — Resume execution
- `next` / `n` — Step over
- `step` / `s` — Step into
- `finish` / `f` — Step out
- `terminate` / `exit` — Stop debugging

**Output (stderr):** Events to the debug adapter (JSON per line)
- `{"event":"paused","reason":"breakpoint","file":"...","line":5,"depth":0}`
- `{"event":"continued"}`
- `{"event":"terminated","exitCode":0}`

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | File not found, parse error, or runtime error |
