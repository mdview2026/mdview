# mdview 双击打开 .md 页面空白问题 — 排查与修复结论

日期：2026-07-18
环境：Ubuntu 24.04（Xorg，DISPLAY=:1）、WebKitGTK 2.52.3、wry 0.55 / tao 0.35

## 症状

双击 `.md` 文件后 mdview 正常启动：窗口弹出、标题正确（`xxx.md - mdview`）、
内置 HTTP 服务正常（`curl http://127.0.0.1:3456/` 能取到完整渲染结果），
但窗口页面**纯白**。

## 排查过程（按排除顺序）

1. **HTTP 层正常**：curl 取回的首页包含 `<h1>Hello World</h1>`，渲染管线（md4x FFI）
   没有问题 —— 与之前"渲染空白"的历史问题不同，那次是二进制陈旧导致内容为空，
   这次是**内容有但不显示**。

2. **排除 WebKitGTK / GPU 因素**：
   - 用系统自带的 `/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/MiniBrowser` 打开同一 URL，
     渲染完全正常；
   - `WEBKIT_DISABLE_DMABUF_RENDERER=1`、`WEBKIT_DISABLE_COMPOSITING_MODE=1` 均无效；
   - WebKit 网络进程确实抓取了页面（`~/.cache/mdview/WebKitCache` 中有完整响应）。
   结论：页面**加载了、JS 也跑了，但从未绘制**。

3. **定位到 wry 的 X11 嵌入方式**：wry 0.55 的 `WebViewBuilder::build(&window)`
   在 Linux/X11 下会把 tao 的 X 窗口用 `gdk_x11_window_foreign_new_for_display`
   包装成一个"外来" GtkWindow 再塞入 webview。在本机环境下该路径不绘制，整窗纯白。

4. **验证修复方向**：最小探针程序改用 wry 官方推荐的
   `WebViewBuilderExtUnix::build_gtk(window.default_vbox())`
   （把 webview 直接嵌入 tao 窗口自带的 GTK 控件树），同一 URL 渲染正常。

## 顺带发现的第二个 bug

无参数启动的欢迎窗口直接崩溃退出：`Error: the underlying handle is not available`。
原因：窗口以 `with_visible(false)` 创建时 GTK 窗口尚未 realize，而 `set_visible(true)`
是经事件循环异步生效的，`build(&window)` 取 raw window handle 时失败。

## 修复

`src/main.rs` 新增平台分流函数 `build_webview_for_window()`：

- **Windows / macOS**：维持 `builder.build(&window)`（WebView2 / WKWebView，完全不变）；
- **Linux/\*BSD**：改用 `builder.build_gtk(window.default_vbox())`，
  把 WebKitGTK webview 嵌入 tao 自己的 GTK vbox —— X11/Wayland 均能正确绘制，
  且不再需要 raw handle，欢迎窗口的崩溃一并解决。

cfg 目标列表与 wry 对 `WebViewBuilderExtUnix` 的门控保持一致（首版用
`#[cfg(not(windows))]` 会误伤 macOS 编译，已修正）。
`cargo check --target x86_64-pc-windows-gnu` 通过，Windows 侧代码路径不受影响。

三个 webview 创建点（预览 / 欢迎 / 设置窗口）全部改为走该函数。

## 验证（截图级）

- `gio open /tmp/mdview_test.md`（等价于双击，进程属主为桌面用户 harry）
  → 窗口正常渲染标题、列表、中文与 emoji，浮动编辑按钮出现（JS 正常）；
- 无参数启动 → 欢迎页正常显示（此前直接崩溃）；
- release 二进制已重新构建，`.desktop` 指向路径不变，无需重新注册。

## 经验教训

- **curl 正常 ≠ 窗口正常**：此前复查曾仅凭 HTTP 输出判定"渲染正常"，漏掉了
  webview 不绘制的问题。凡是 GUI 显示类问题，必须截图级验证。
- wry 在 Linux 上应优先使用 `build_gtk` 嵌入 tao 的 GTK 控件树，
  而不是 X11 外来窗口包装路径（wry 文档亦如此建议，且兼容 Wayland）。
