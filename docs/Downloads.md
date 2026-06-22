# Downloads

Get the latest BackLang binary for your platform.

## Latest Release

[![GitHub release](https://img.shields.io/github/v/release/surgeodev/BackLang?style=for-the-badge)](https://github.com/surgeodev/BackLang/releases/latest)

### Linux (x86_64)

| File | Link |
|------|------|
| Binary | [backlang-x86_64-unknown-linux-gnu](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-unknown-linux-gnu) |

```bash
# Download
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-unknown-linux-gnu

# Make executable and install
chmod +x backlang-x86_64-unknown-linux-gnu
sudo mv backlang-x86_64-unknown-linux-gnu /usr/local/bin/bl
```

### macOS

| File | Link |
|------|------|
| Intel | [backlang-x86_64-apple-darwin](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-apple-darwin) |
| Apple Silicon | [backlang-aarch64-apple-darwin](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-aarch64-apple-darwin) |

```bash
# Intel
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-apple-darwin
chmod +x backlang-x86_64-apple-darwin
sudo mv backlang-x86_64-apple-darwin /usr/local/bin/bl

# Apple Silicon
curl -LO https://github.com/surgeodev/BackLang/releases/latest/download/backlang-aarch64-apple-darwin
chmod +x backlang-aarch64-apple-darwin
sudo mv backlang-aarch64-apple-darwin /usr/local/bin/bl
```

### Windows (x86_64)

| File | Link |
|------|------|
| Executable | [backlang-x86_64-pc-windows-msvc.exe](https://github.com/surgeodev/BackLang/releases/latest/download/backlang-x86_64-pc-windows-msvc.exe) |

**Download and add to PATH:**

1. Download the `.exe` file
2. Rename it to `bl.exe`
3. Move it to a folder in your PATH (e.g. `C:\Users\YourName\.cargo\bin\`)
4. Open a new terminal and run `bl --help`

### Checksums

```
sha256sum backlang-*
```

Available on the [GitHub Release](https://github.com/surgeodev/BackLang/releases/latest) page.

## Manual Build

If your platform is not listed above, see the [Installation](Installation) page for build instructions.
