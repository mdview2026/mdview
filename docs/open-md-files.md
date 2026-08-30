# How to Open .md Files on Windows and macOS — mdview, a Markdown Viewer That Opens on Double-Click

A `.md` file is a Markdown document — plain text with lightweight formatting symbols (`#` headings, `-` lists, `` `code` ``). READMEs, AI-generated answers, technical notes and changelogs are commonly written in Markdown. Open one in Notepad and you see the raw symbols; you need a **Markdown viewer** to see the rendered page — headings, tables, code highlighting, the works.

This guide compares the three common ways to open `.md` files, then shows how [mdview](https://www.mdview.top/) — a 2 MB viewer, not a 200 MB editor — makes "double-click and read" the default behavior.

**In mainland China?** The domestic edition is distributed by [www.mdview.cn](https://www.mdview.cn/) ([download page](https://www.mdview.cn/download.html), WeChat Pay, ¥5 Pro lifetime). Everything below works the same.

## Three ways to open a .md file

| | A full editor (Typora, VS Code, Obsidian) | A browser extension | **mdview** (dedicated viewer) |
|---|---|---|---|
| Install size | 100–400 MB | small, but needs the browser | **2.0 MB** |
| Double-click a .md file | 3–5 s of editor startup | drag or copy-paste each time | **opens instantly, rendered** |
| Reading experience | toolbars, panels, tabs | browser UI around the page | **clean full-page reading** |
| Built for | writing code / docs | occasional preview | **reading** |

Editors are great when you're *writing*. For the 90% of times you just want to *read* a README or an AI-generated document, a dedicated viewer is the lighter tool.

## Set .md files to open with mdview (one-time)

1. **Download and run the installer** from the [latest release](https://github.com/mdview2026/mdview/releases/latest) — or the [download page](https://www.mdview.top/download.html).
2. The installer **automatically associates `.md`** (and `.markdown`) files with mdview. No configuration needed.
3. Double-click any `.md` file — it opens rendered like a webpage, in under a second.

Other ways in:

- `Ctrl+O` inside mdview, or drag-and-drop a file onto the window
- Right-click a file → **Open with mdview** (add it permanently with `mdview --install`, remove with `mdview --uninstall`)
- Command line: `mdview path/to/file.md`

To undo the association later: right-click the page → **Settings**, or run `mdview --unbind`.

## Verify it works

After installing, save this as `test.md` and double-click it:

```markdown
# It works

- The heading above renders large
- **Bold** and *italic* show as styled text, not asterisks
- This code block has syntax highlighting
```

You should see a rendered page with a floating outline panel on the left. Press `Esc` to close — like closing a photo viewer.

## Troubleshooting

<details>
<summary><strong>Windows shows a blue SmartScreen warning</strong></summary>

Windows flags apps that haven't paid for an expensive code-signing certificate. Click **More info → Run anyway**. mdview runs entirely locally and uploads nothing — see [privacy and network behavior](privacy-network.md).
</details>

<details>
<summary><strong>The window is blank / won't render on Windows</strong></summary>

mdview uses the WebView2 runtime, preinstalled on Windows 11 and most Windows 10 PCs. If it's missing, get it from Microsoft: [WebView2 download](https://developer.microsoft.com/microsoft-edge/webview2/).
</details>

<details>
<summary><strong>The file shows # symbols and asterisks instead of formatting</strong></summary>

That's the Markdown *source*, not garbled text — whatever opened the file is showing raw text instead of rendering it. If `.md` got associated with Notepad or another editor, re-run the mdview installer or set the default app manually: right-click the file → **Open with → Choose another app → mdview → Always**.
</details>

<details>
<summary><strong>Chinese text shows as garbled characters (乱码)</strong></summary>

The file is likely saved in GBK/GB18030 instead of UTF-8. mdview has a built-in GBK/GB18030 encoding fallback and renders these files correctly in most cases.
</details>

## Further reading

- [How to Open .md Files on Windows? 3 Methods Compared](https://www.mdview.top/blog/how-to-open-md-file.html)
- [Why Does My .md File Show Only # Symbols?](https://www.mdview.top/blog/md-file-shows-symbols.html)
- [mdview facts: installer size, startup time, platform support](facts.md)
- [Privacy: network behavior and offline verification](privacy-network.md)
