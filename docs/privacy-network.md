# mdview Privacy: Local-First Markdown Viewing — Network Behavior and Offline Verification

mdview is a **local-first** Markdown viewer: your documents are rendered on your own machine and their contents never leave your computer. This page lists *every* network behavior the app performs, so you can verify the claims yourself — including a try-it-at-home offline test.

**In mainland China?** The domestic edition (from [www.mdview.cn](https://www.mdview.cn/)) uses the same local rendering architecture; its few network calls go to `www.mdview.cn` / `ml-smarttech.com` (update check, anonymous device report, WeChat payment) instead of the international endpoints listed below.

## The short version

| Question | Answer |
|---|---|
| Are my Markdown files uploaded anywhere? | **No.** Rendering happens via a temporary server on `localhost` only |
| Do I need an account? | **No.** There is no login, no registration |
| Is there advertising or third-party tracking? | **No** |
| Does it work offline? | **Yes** — all reading features work with no network at all |
| What network calls does it make at all? | Update check + one anonymous device report at startup; Pro purchase and feedback only when *you* act |

## Document rendering: localhost only

When mdview opens a file, it starts an HTTP server **bound to localhost** and renders the page in an embedded WebView. The file's content is served from your own disk to your own process — it is never sent to the internet. Closing the window shuts the server down.

## The complete list of network behavior

1. **Update check.** mdview periodically checks a version file (`api/latest.json`) to tell you when a new release is available. It downloads nothing without your action. International edition: `mdview-web-en.pages.dev` / `www.mdview.top`; domestic edition: `www.mdview.cn`. GitHub (`github.com`) serves release downloads.
2. **Anonymous startup report (international: `api.mdview.top`; domestic: `ml-smarttech.com`).** One small POST at launch containing an anonymous device id, open counter, and build edition — used for license verification and aggregate usage stats. It contains **no file contents, no file names, no file paths, and no personal data**. If the request fails, times out, or the machine is offline, mdview silently continues — features never lock up because a report didn't arrive.
3. **Pro purchase.** Only when you choose to buy: international edition uses Gumroad (card / PayPal), domestic edition uses WeChat Pay.
4. **User-initiated feedback.** Only when you submit something from the in-app feedback form, including attachments you explicitly attach.
5. **Pro rendering engines (Mermaid / KaTeX).** Downloaded once on first use, then cached — after that they work fully offline.

That's the whole list. There is no telemetry SDK, no analytics script, no ad network.

## Verify it yourself: the offline test

1. Install mdview and open a few `.md` files, including ones with tables, images and code blocks.
2. **Disconnect from the network entirely** (turn off Wi-Fi / unplug the cable).
3. Open any local `.md` file. It renders exactly the same — themes, outline, fonts, export to PDF all keep working.

You can also watch the traffic: any firewall or proxy tool (e.g. Wireshark, Glasswire, Little Snitch) will confirm the only outbound connections are the endpoints listed above, and that no document content appears in them.

## Local data storage

mdview stores a small config file (theme, window size, reading positions, favorites) on your own disk. Nothing is stored in the cloud.

## Further reading

- [mdview facts: installer size, startup time, platform support](facts.md)
- [How to open .md files on Windows and macOS](open-md-files.md)
- [Privacy policy (Pro edition)](https://www.mdview.top/privacy.html)
