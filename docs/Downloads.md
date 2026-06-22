# Downloads

Get the latest BackLang binary for your platform.

## Latest Release

[![GitHub release](https://img.shields.io/github/v/release/surgeodev/BackLang?style=for-the-badge)](https://github.com/surgeodev/BackLang/releases/latest)

### Linux (x86_64)

**Direct download** (right-click → Save Link As):

[backlang-x86_64-unknown-linux-gnu](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-unknown-linux-gnu)

```bash
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-unknown-linux-gnu
chmod +x backlang-x86_64-unknown-linux-gnu
sudo mv backlang-x86_64-unknown-linux-gnu /usr/local/bin/bl
```

### macOS

**Intel:**
[backlang-x86_64-apple-darwin](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-apple-darwin)

```bash
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-apple-darwin
chmod +x backlang-x86_64-apple-darwin
sudo mv backlang-x86_64-apple-darwin /usr/local/bin/bl
```

**Apple Silicon:**
[backlang-aarch64-apple-darwin](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-aarch64-apple-darwin)

```bash
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-aarch64-apple-darwin
chmod +x backlang-aarch64-apple-darwin
sudo mv backlang-aarch64-apple-darwin /usr/local/bin/bl
```

### Windows (x86_64)

**Direct download:**
[backlang-x86_64-pc-windows-msvc.exe](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-pc-windows-msvc.exe)

1. Right-click the link → **Save Link As** → save as `bl.exe`
2. Create a folder for your binaries: `mkdir %USERPROFILE%\bin`
3. Move `bl.exe` to `%USERPROFILE%\bin\`
4. Add it to PATH: `setx PATH "%PATH%;%USERPROFILE%\bin"`
5. Open a **new** terminal and run `bl --help`

> **Note:** GitHub's browser download wraps files in a `.zip`. Use the direct links above or `curl` in a terminal to get the raw binary.

### VS Code Extension

[backlang-debug-1.0.0.vsix](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-debug-1.0.0.vsix)

1. Download the `.vsix` file
2. VS Code → Extensions (`Ctrl+Shift+X`) → `...` → **Install from VSIX...**
3. Select the file

Or via terminal:
```bash
code --install-extension backlang-debug-1.0.0.vsix
```

### Checksums

```
sha256sum backlang-*
```

Available on the [GitHub Release](https://github.com/surgeodev/BackLang/releases/latest) page.

## One-Line Install

Prefer the terminal? See the [Installation](Installation) page.

## Manual Build

If your platform is not listed above, see the [Installation](Installation) page for build instructions.
