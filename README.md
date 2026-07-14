# mdview

A lightweight Markdown live-preview tool for Windows, Linux and macOS, built with Rust (Windows: WebView2 · Linux: WebKitGTK · macOS: WKWebView).

🌐 **Official website**: [https://www.mdview.top](https://www.mdview.top)

![icon](static/logo_big.png)

## Features

- **Double-click to open** — binds itself as the default `.md` handler on first run, then any Markdown file opens with a double-click
- **Live refresh** — re-renders automatically on save, no manual refresh needed
- **Dark / light themes** — toggle from the right-click menu; TOC and dialogs adapt
- **Table of contents (TOC)** — auto-generated outline, click to jump, auto-highlights on scroll
- **Edit source** — select text and press `Ctrl+E` to open the file in your editor at the matching line
- **Local images** — loads images relative to the Markdown file
- **Export to PDF** — print / export from the right-click menu
- **No ads, no telemetry** — runs locally, your files never leave the machine

## Quick start

1. Download `mdview.exe` from [mdview.top](https://www.mdview.top) and drop it anywhere
2. **Double-click it** — it registers itself as the default `.md` handler
3. **Double-click any `.md` file** to preview it

> To undo the default binding later: right-click in a preview → Settings → Unbind.

## Usage

### Preview window

| Action | Description |
|--------|-------------|
| `ESC` | Close the preview |
| `Ctrl + E` | With text selected, open the source file in your editor at that line |
| Right-click | Copy / Select all / Edit source / Export PDF / Toggle theme / Show-hide TOC |

### Themes

Right-click the preview → "Switch to dark theme" / "Switch to light theme". The choice is saved automatically.

### Table of contents (TOC)

- A floating outline appears on the left
- Click an entry to smooth-scroll to that section
- The current section is highlighted as you scroll
- Drag to resize; switch position (far-left / left-of-text / right-of-text / far-right)
- Toggle visibility from the right-click menu

## Command line

```bash
mdview                     Run with no args (auto-binds the .md association)
mdview <file.md>           Preview a specific Markdown file
mdview --install           Install the right-click "Open with mdview" entry
mdview --uninstall         Remove the right-click entry
mdview --settings          Open the settings window (binding state, recent files, editor)
mdview --unbind            Remove the .md default association
mdview --help              Show help
```

## Environment variables

| Variable | Description |
|----------|-------------|
| `PORT` | HTTP server port (default: random available port, falls back to 3456) |
| `MD_HTML=1` | Also write a `.html` file next to the Markdown file |
| `MD_HTML_OUTPUT=<path>` | Write the `.html` to a specific path |
| `MD_EDITOR` | Editor command for `Ctrl+E` (auto-detected if unset) |

## Tech stack

- **Language**: Rust (edition 2021)
- **GUI**: wry (WebView2) + tao
- **HTTP server**: axum + tokio
- **Markdown rendering**: bundled [md4x](https://github.com/unjs/md4x) (C, MIT)
- **File watching**: notify
- **Styling**: Tailwind CSS

## Building

Pre-built binaries for Windows, Linux and macOS are produced by the [Build & Release](.github/workflows/release.yml) workflow on every tagged release.

To build locally you need Rust and [zig](https://ziglang.org/). `build.rs` invokes zig to compile the md4x / libyaml C sources under `csrc/` into a static library (zig must be on PATH, or at `D:\zig\` / `C:\zig\`).

```bash
cargo build --release
```

Per-platform extras:

- **Windows** — Rust MSVC toolchain. Binary: `target/release/mdview.exe`.
- **Linux** — WebKitGTK dev headers: `sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev build-essential libssl-dev`. Binary: `target/release/mdview`.
- **macOS** — Xcode Command Line Tools (WKWebView is system-provided). For a universal binary, build both targets and combine them with `lipo -create target/{aarch64,x86_64}-apple-darwin/release/mdview -output mdview`. Binary: `target/release/mdview`.

> File-association and the right-click "Open with mdview" commands (`--install`, `--unbind`, …) are Windows-only; on Linux/macOS they print a message and exit.

## Project layout

```
src/
  main.rs      # entry, CLI parsing, window creation
  server.rs    # HTTP server, routes, settings/welcome pages
  renderer.rs  # Markdown rendering, HTML wrapper (theme/TOC/context menu)
  watcher.rs   # file watching + SSE live refresh
  config.rs    # config read/write, recent files, window state
  registry.rs  # file association, context menu, registry ops
  editor.rs    # editor detection and open-at-line
csrc/
  md4x/        # md4x Markdown parser (MIT, bundled)
  libyaml/     # libyaml YAML parser (MIT, bundled)
static/
  icon.ico     # application icon
  logo_big.png # app logo
  style.css    # compiled Tailwind styles
```

## License

Released under the [MIT License](./LICENSE). Contributions and pull requests welcome.

## Acknowledgements

mdview's Markdown parsing is built on these open-source projects:

- **[md4x](https://github.com/unjs/md4x)** — a C Markdown parser (MIT License, Copyright © 2016-2024 Martin Mitáš). Bundled under `csrc/md4x/` and compiled into a static library by `build.rs` via zig.
- **[libyaml](https://github.com/yaml/libyaml)** — a YAML parser (MIT License, Copyright © 2017 Kirill Simonov), used by md4x for YAML front matter. Bundled under `csrc/libyaml/`.

Full license text for each component lives in the `LICENSE` file under its directory.
