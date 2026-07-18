# mdview Linux 移植记录

## 已完成的工作

1. **依赖项平台化** (`Cargo.toml`)
   - 将 `winreg`、`windows-sys` 移到 `[target.'cfg(windows)'.dependencies]`
   - 将 `winres` 移到 `[target.'cfg(windows)'.build-dependencies]`

2. **构建脚本跨平台化** (`build.rs`)
   - 不再硬编码 `x86_64-windows-msvc`
   - 通过 `rustc -vV` 获取 host target，并转换为 zig 可识别的 triple（如 `x86_64-linux-gnu`）
   - 仅在 Windows 下对 libyaml 定义 `-Dstrdup=_strdup`
   - Windows 输出 `md4x.lib`，Linux 输出 `libmd4x.a`

3. **主程序平台适配** (`src/main.rs`)
   - `with_default_context_menus(false)` 是 Windows-only API，已包进 `#[cfg(windows)]` 块

4. **Linux 桌面入口注册脚本**
   - 新增 `static/mdview.desktop`
   - 新增 `install-linux-mime.sh`，用于将 mdview 注册为 `.md` 默认打开方式

## 验证结果

### 可执行文件

```bash
/data/study/mdview/target/release/mdview
```

- `mdview --help` 正常输出
- 文件大小约 3.0MB，ELF 64-bit LSB pie executable

### GUI 运行

在 X11 桌面环境（`DISPLAY=:1`）下：

```bash
./target/release/mdview /path/to/file.md
```

窗口可以正常打开，HTTP 服务器正常启动（如 `http://127.0.0.1:3456`）。

### 文件关联

运行 `./install-linux-mime.sh` 后：

```bash
xdg-mime query default text/markdown   # -> mdview.desktop
xdg-mime query default text/x-markdown # -> mdview.desktop
```

双击 `.md` 文件会调用 mdview。

2026-07-18 端到端复验通过（GNOME/Nautilus）：`gio open /tmp/test.md`（等价于双击）
→ mdview 启动、HTTP 服务正常、窗口 `test.md - mdview` 正常弹出，页面渲染 `<h1>Hello World</h1>`。
注意：`Exec` 指向 `target/release/mdview`，`cargo clean` 或移动二进制后需重跑 `install-linux-mime.sh`。

**多用户坑（2026-07-18 实踩）**：MIME 关联是**按用户**的。桌面会话用户是 harry，
而以 root 身份运行 `install-linux-mime.sh` 只写入了 `/root/.local/share/applications/` 和 root 的
mimeapps.list —— root 下 `gio open` 正常，但 harry 双击仍回落到文本编辑器（harry 对 text/markdown
没有任何默认应用）。修复方式：

1. 桌面文件安装到系统目录 `/usr/local/share/applications/mdview.desktop`（Exec/Icon 路径全局可读）
2. 以桌面用户身份注册默认应用：
   `sudo -u harry env HOME=/home/harry xdg-mime default mdview.desktop text/markdown`（text/x-markdown 同理）

已按此修复并以 harry 身份 `gio open` 验证通过（进程属主 harry、窗口弹出、渲染正常）。

## 已解决：窗口纯白（页面已加载但不绘制，2026-07-18 下午）

### 症状

双击 `.md` 文件 → mdview 窗口弹出、标题正确，但页面**纯白**。curl 拉取 HTTP 输出内容完全正常。

> 注意：上一节"Markdown 内容渲染空白（2026-07-18 复查）"的结论只验证了 HTTP 层
> （curl 输出正确），并未覆盖"窗口是否真的绘制"。curl 正常 ≠ 窗口正常，排查此类问题必须截图验证。

### 根因

wry 0.55 的 `WebViewBuilder::build(&window)` 在 Linux/X11 下走的是"外来窗口包装"路径：
把 tao 的 X 窗口用 `gdk_x11_window_foreign_new_for_display` 包成一个 GtkWindow 再塞 webview。
在本机（Xorg + WebKitGTK 2.52.3）该路径下页面**能加载**（WebKitNetworkProcess 有请求、
`~/.cache/mdview/WebKitCache` 里有完整响应、JS 也在跑）但**从不绘制**，整窗纯白。

对照实验：`/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/MiniBrowser http://127.0.0.1:3456/`
渲染完全正常 → WebKitGTK/GPU 本身没问题，问题在 wry 的 X11 嵌入方式。
（`WEBKIT_DISABLE_DMABUF_RENDERER=1`、`WEBKIT_DISABLE_COMPOSITING_MODE=1` 均无效，已排除。）

顺带发现的第二个 bug：欢迎窗口（无参数启动）直接崩溃退出
`Error: the underlying handle is not available`。原因：窗口 `with_visible(false)` 创建时
GTK 窗口尚未 realize，`set_visible` 又是经事件循环异步生效的，导致 `build(&window)` 取不到
raw window handle。

### 修复

`src/main.rs` 新增 `build_webview_for_window()`：Windows/macOS 仍用 `build(&window)`；
Linux/\*BSD 改用 wry 官方推荐的 `WebViewBuilderExtUnix::build_gtk(window.default_vbox())`
（tao 窗口自带的 vbox，直接嵌入 tao 自己的 GTK 控件树，X11/Wayland 都正确绘制，也不再需要
raw handle，欢迎窗口的崩溃一并解决）。三个 webview 创建点（预览/欢迎/设置）全部走该函数。
cfg 目标列表与 wry 对 `WebViewBuilderExtUnix` 的门控一致
（不能图省事写 `#[cfg(not(windows))]`，否则 macOS 编译会挂——`tao::platform::unix`
和 `WebViewBuilderExtUnix` 在 macOS 上不存在）。
`cargo check --target x86_64-pc-windows-gnu` 通过，Windows 侧不受影响。

### 验证（截图级）

- `gio open /tmp/mdview_test.md`（等价双击，进程属主 harry）→ 窗口正常渲染 `<h1>Hello World</h1>`
- 无参数启动 → 欢迎页正常显示（此前直接崩溃）
- MiniBrowser 与 mdview 同 URL 对照渲染一致

## 已解决：Markdown 内容渲染空白（2026-07-18 复查）

### 结论

当前工作区代码不存在该问题。用当前源码重新构建后，之前所有失败用例均渲染正常。
当时的空白现象来自移植过程中某个中间/过期的构建产物（或端口 3456 上残留的旧服务进程），
并非当前源码的缺陷。

### 验证矩阵（release 二进制 + curl 直接验证 HTTP 输出）

| 用例 | 结果 |
|------|------|
| 纯 ASCII（`# Hello World` + 列表） | ✅ `<h1>Hello World</h1>` |
| 中文 + emoji 🌐 + 符号 `—` `→` | ✅ `<h1>你好世界</h1>` |
| UTF-8 BOM 文件 | ✅ |
| CRLF 换行文件 | ✅ |
| frontmatter 文件 | ✅（frontmatter 被正确抑制，正文正常渲染） |
| `README.md` | ✅ `<h1>mdview</h1>` |

FFI 单元测试（`src/md4x.rs`，`cargo test --bin mdview md4x`）在 debug 与 release profile 下均全部通过。

### 排查记录

- `md_html_ex` 的 Rust FFI 声明与 `md4x-html.h` 完全一致（`MD_CHAR=char`、`MD_SIZE=unsigned`），
  parser/renderer 标志位值也与 `md4x.h` 逐一核对无误
- 重新加入 `-funsigned-char` 构建验证：渲染依然正常，与当时"全部空白"的观察无关，该线索排除
- 最可能的解释：回滚 `-funsigned-char` 后的重建并未真正成功（见下方 zig PATH 问题，
  构建脚本 panic 时旧二进制会原样保留），导致后续一直在测试实验窗口期的旧二进制

## 注意事项：zig 不在默认 PATH 上

zig 0.13.0 位于 `/tmp/zig/zig`，不在默认 PATH。一旦 build.rs 需要重跑（修改 build.rs/csrc 或 clean 构建），
必须先 `export PATH=/tmp/zig:$PATH`，否则构建脚本会 panic（"zig not found"），且**旧二进制原样保留**，
容易造成"改了代码但没生效"的假象。建议将 zig 安装到固定位置并写入 PATH（/tmp 重启后会丢失）。

## 环境信息

- OS: Ubuntu 24.04.4 LTS
- Rust: 1.95.0
- zig: 0.13.0（用于编译 C 源文件，路径 `/tmp/zig/zig`）
- GUI: X.Org 21.1.11 (`DISPLAY=:1`)
- WebKitGTK: libwebkit2gtk-4.1-dev / libwebkit2gtk-4.1-0
