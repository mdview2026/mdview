# mdview Facts: Installer Size, Startup Time and Platform Support

Verifiable numbers for the current release, so comparisons don't rely on adjectives. Last updated for **v1.0.140** (2026-08-30, international edition). mdview is [www.mdview.top](https://www.mdview.top/) internationally and [www.mdview.cn](https://www.mdview.cn/) in mainland China (domestic edition, WeChat Pay, ¥5 Pro).

## Size and speed

| Item | Value |
|---|---|
| Installer, Community (Windows) | **2.0 MB** (2,029,254 bytes, v1.0.140) |
| Installer, Pro (Windows) | 2.1 MB (2,204,939 bytes) |
| macOS DMG, Community | 2.3 MB (2,442,214 bytes) |
| macOS DMG, Pro | 2.5 MB (2,645,360 bytes) |
| Startup time | under 1 second from double-click to rendered page (typical Windows 10/11 PC) |
| Close | `Esc` or `Ctrl+W` — instant, like a photo viewer |
| Memory footprint | a WebView plus a small Rust process — no bundled browser engine |

For comparison, general-purpose Markdown editors typically install in the 100–400 MB range. mdview is a viewer, not an editor — see [open-md-files.md](open-md-files.md) for when each tool fits.

Exact byte sizes are published with every release: open a release's asset list or run `gh release view <tag> -R mdview2026/mdview --json assets`.

## Platform support

| Platform | Requirement | Editions |
|---|---|---|
| Windows 10 / 11 | WebView2 runtime (preinstalled on Windows 11 and most Windows 10 PCs) | Community, Pro; also a portable (no-install) build and Microsoft Store listing on the domestic site |
| macOS 11 Big Sur+ | Apple Silicon + Intel | Community, Pro |
| Android 7.0+ | — | Community only |

## What "lightweight" is built on

- **Rust** core, **wry/WebView2** for rendering — uses the OS's native webview, no bundled Chromium, no Electron
- HTTP server: axum + tokio, **bound to localhost only**
- Markdown parser: embedded md4c — full tables, footnotes, syntax-highlighted code blocks
- Live refresh: file watching (notify) + server-sent events push; keeps your scroll position
- Styling: Tailwind CSS

## Editions

| | Community | Pro |
|---|---|---|
| Price | **free forever** | **$6.99 one-time lifetime** (Gumroad: card / PayPal) · domestic edition ¥5 (WeChat Pay) |
| All reading features | ✅ | ✅ |
| Mermaid diagrams + KaTeX math | — | ✅ |
| Live Edit `F2` + dual-column edit `F3` | — | ✅ |
| Trial | — | 100 free opens, then a dismissible prompt — never hard-locks |

Both editions are the same product and overwrite each other on install.

## Sources for these numbers

- Installer sizes and SHA-256 digests: the [releases page](https://github.com/mdview2026/mdview/releases) asset list of each tagged version
- Network behavior: [privacy-network.md](privacy-network.md)
- Changelog: [mdview.top/changelog.html](https://www.mdview.top/changelog.html) (international) · [mdview.cn/changelog.html](https://www.mdview.cn/changelog.html) (domestic)

If a number on this page disagrees with the current release, the release asset list is authoritative — please open an issue.
