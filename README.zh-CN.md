<p align="center">
  <img src="static/logo_big.png" width="96" alt="mdview logo">
</p>

<h1 align="center">mdview</h1>

<p align="center">
  <strong>Markdown,轻装上阵。</strong>一个 2.0 MB 的查看器,而不是又一个 200 MB 的编辑器 —— 双击任意 .md 文件即可像网页一样阅读,按 F2 原地编辑。<br>
  Windows · macOS · Android &nbsp;|&nbsp; 基于 Rust + WebView2 构建,无 Electron
</p>

<p align="center">
  <a href="https://www.mdview.cn/">官网</a> ·
  <a href="https://www.mdview.cn/download.html">下载</a> ·
  <a href="https://www.mdview.cn/changelog.html">更新日志</a> ·
  <a href="https://www.mdview.cn/blog/">博客</a> ·
  <a href="docs/facts.md">文档</a>
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a>
</p>

> 本页为**国内版**介绍——微信支付,Pro ¥5 永久解锁,下载与更新均由 mdview.cn 本站提供。海外用户请看 [英文版](README.md)(国际版官网 [www.mdview.top](https://www.mdview.top/),GitHub Releases 下载,Pro $6.99)。

---

大多数时候你打开一个 `.md` 文件,只是想**阅读**——一份 README、一些笔记、一段 AI 生成的回答。为此启动一个完整的编辑器未免小题大做。mdview 让它像网页一样瞬间打开;而当你确实需要编辑时,`F2` 会在同一个窗口里提供 Obsidian 式的所见即所得编辑——无需切换到其他应用。

- **应用极小。** 2.0 MB 安装包,不到一秒打开,零配置
- **界面极简。** 干净的整页阅读——没有工具栏,没有不请自来的侧边栏
- **原地编辑。** `F2` Obsidian 式所见即所得,`F3` 双栏实时预览
- **100% 本地运行**——无账号、无广告,文件永远不会离开你的电脑

<p align="center">
  <img src="web_en/img/shots125/gallery-01-hero.png" alt="mdview 像网页一样渲染 Markdown 文件:左侧大纲面板,标题、列表、表格和高亮代码块" width="1080">
</p>

## 下载(国内版)

| 平台 | 社区版(永久免费) | Pro 版(¥5 终身) |
|---|---|---|
| Windows 10/11 | [mdview-setup-latest.exe](https://www.mdview.cn/download/mdview-setup-latest.exe) | [mdview-setup-pro-latest.exe](https://www.mdview.cn/download/mdview-setup-pro-latest.exe) |
| macOS 11+ | [mdview-macos-latest.dmg](https://www.mdview.cn/download/mdview-macos-latest.dmg) | [mdview-macos-pro-latest.dmg](https://www.mdview.cn/download/mdview-macos-pro-latest.dmg) |
| Android 7.0+ | [mdview.apk](https://www.mdview.cn/download/mdview.apk) | — |

以上为**最新版**直链。各版本号历史存档、Windows 绿色免安装版(zip)与微软商店版,请见 [官网下载页](https://www.mdview.cn/download.html)。

两个版本是同一产品,安装时互相覆盖——随时切换。Android 仅有社区版。Pro 通过**微信支付 ¥5 永久解锁**,付款后即时生效,无需注册账号。

> **遇到 SmartScreen 警告?** Windows 会对没有昂贵代码签名证书的应用弹出提示。点击**更多信息 → 仍要运行**。mdview 完全本地运行,不上传任何数据。

## 为什么选 mdview

你双击 `.md` 文件是为了阅读——不是为了启动一个 IDE。把它理解为 **Markdown 的 Quick Look**:

| | mdview | Typora / VS Code | 浏览器扩展 |
|---|---|---|---|
| 安装体积 | **2.0 MB** | 100–400 MB | —(但需要浏览器) |
| 双击 .md 文件 | **瞬间打开** | 编辑器启动 3–5 秒 | 每次都要拖拽或复制粘贴 |
| 阅读优先的界面 | **零干扰** | 工具栏、面板、标签页 | 外面包着浏览器界面 |
| 为何而生 | **阅读** | 写代码 / 写文档 | 偶尔预览 |

它们互为补充——写作用编辑器,90% 只是阅读的场合交给 mdview。

## 功能

### 预览

- **双击打开**——安装包自动关联 `.md` 文件;也支持 `Ctrl+O`、拖放和资源管理器右键菜单
- **实时刷新**——在任意编辑器中保存,页面立即重新渲染(SSE 推送),并保持滚动位置
- **大纲面板**——悬浮目录,点击跳转、滚动同步,还有**历史记录**(按天分组)和**收藏**标签页
- **记忆阅读位置**——重新打开时回到上次读到的地方;每个文件单窗口
- **9 套配色主题 + 深色/浅色模式**——Glacier、Forest、Sunset、Typewriter……右键一下即可切换
- **排版控制**——任选已安装字体;`Ctrl+滚轮` 调字号,`Alt+滚轮` 调栏宽(真正的重排版,不是位图缩放)
- **完整渲染**——对齐表格、带回链的脚注、本地图片、语法高亮代码块(复制 / 自动换行 / 行号)
- **导出 PDF**、单换行即换行开关、GBK/GB18030 编码回退、自动检查更新

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-02a-outline.png" alt="mdview 大纲面板在滚动时高亮当前标题"></td>
    <td><img src="web_en/img/shots125/gallery-02b-history.png" alt="mdview 历史面板按天分组显示最近打开的 Markdown 文件"></td>
  </tr>
  <tr>
    <td align="center"><sub>大纲跟随你的滚动位置</sub></td>
    <td align="center"><sub>历史与收藏,按天分组</sub></td>
  </tr>
</table>

**9 套配色主题 + 深色模式**,右键一下即可切换:

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-default.png" alt="mdview 默认浅色主题"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-dark.png" alt="mdview 深色模式主题"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-glacier.png" alt="mdview Glacier 主题,冷蓝色调"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-typewriter.png" alt="mdview Typewriter 主题,等宽打字机风格"></td>
  </tr>
  <tr>
    <td align="center"><sub>Default</sub></td>
    <td align="center"><sub>Dark</sub></td>
    <td align="center"><sub>Glacier</sub></td>
    <td align="center"><sub>Typewriter</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-sunset.png" alt="mdview Sunset 主题,暖琥珀色调"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-forest.png" alt="mdview Forest 主题,沉静绿色调"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-editorial.png" alt="mdview Editorial 主题,杂志衬线风格"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-violet.png" alt="mdview Violet 主题,紫色调"></td>
  </tr>
  <tr>
    <td align="center"><sub>Sunset</sub></td>
    <td align="center"><sub>Forest</sub></td>
    <td align="center"><sub>Editorial</sub></td>
    <td align="center"><sub>Violet</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-lychee.png" alt="mdview Lychee 主题,柔和红色调"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-mint.png" alt="mdview Mint 主题,清新薄荷调"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-neon.png" alt="mdview Neon 主题,鲜艳高对比"></td>
    <td></td>
  </tr>
  <tr>
    <td align="center"><sub>Lychee</sub></td>
    <td align="center"><sub>Mint</sub></td>
    <td align="center"><sub>Neon</sub></td>
    <td></td>
  </tr>
</table>

### 编辑

| 按键 | 模式 | 版本 |
|---|---|---|
| `F2` | **实时编辑**——预览内 Obsidian 式所见即所得:格式、链接、表格(行/列操作)、标注块 | Pro |
| `F3` | **双栏编辑**——左侧源码,右侧实时预览,按节同步滚动;`Tab` 展开代码片段;粘贴图片/文件自动保存到 `.__assets/` 并插入 Markdown | Pro |
| `F4` / `Ctrl+E` | **外部编辑**——打开 VS Code / Sublime / Notepad++ / 记事本(自动检测),并定位到所选文字的精确行号 | 免费 |
| `Esc` / `Ctrl+W` | 关闭窗口 | 免费 |

<p align="center">
  <img src="web_en/img/shots125/gallery-04-editing.png" alt="mdview F3 双栏编辑:左侧 Markdown 源码,右侧实时渲染预览" width="1080">
</p>

### Pro 渲染

- **Mermaid 图表**(流程图 / 时序图 / 甘特图),双击进入全屏查看器——滚轮缩放、拖拽平移
- **KaTeX 数学公式**——行内 `$...$` 与块级 `$$...$$` LaTeX
- 渲染引擎首次使用时下载一次,之后完全离线可用

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-05a-mermaid.png" alt="mdview Pro 在 Markdown 文档中渲染的 Mermaid 流程图"></td>
    <td><img src="web_en/img/shots125/gallery-05b-katex.png" alt="mdview Pro 渲染的 KaTeX 数学公式,LaTeX 版 BM25 方程"></td>
  </tr>
  <tr>
    <td align="center"><sub>Mermaid 图表</sub></td>
    <td align="center"><sub>KaTeX 数学公式</sub></td>
  </tr>
</table>

## 价格(国内版)

| | 社区版 | Pro 版 |
|---|---|---|
| 价格 | **永久免费** | **¥5** 一次性永久解锁(微信支付) |
| 核心阅读功能 | ✅ 全部 | ✅ 全部 |
| Mermaid + KaTeX | — | ✅ |
| 实时编辑(F2)+ 双栏编辑(F3) | — | ✅ |
| 试用 | — | 100 次免费打开,之后是可忽略的"以后再说"提示——**永不硬性锁定** |

## 快速开始

1. 下载并运行安装包(或绿色免安装版)
2. 自动关联 `.md` 文件(随时解除:右键 → 设置,或 `mdview --unbind`)
3. 双击任意 Markdown 文件,开始阅读

## 命令行

```bash
mdview                     # 运行一次:自动关联 .md 文件
mdview <file.md>           # 预览指定文件
mdview --install           # 在资源管理器右键菜单中添加"使用 mdview 打开"
mdview --uninstall         # 移除右键菜单
mdview --settings          # 设置窗口(关联、最近文件、编辑器)
mdview --unbind            # 解除 .md 默认关联
mdview --help              # 帮助
```

## 环境变量

| 变量 | 说明 |
|---|---|
| `PORT` | HTTP 服务器端口(默认:随机空闲端口,回退 3456) |
| `MD_HTML=1` | 同时在 `.md` 文件旁输出渲染后的 `.html` |
| `MD_HTML_OUTPUT=<path>` | 将 `.html` 输出到指定路径 |
| `MD_EDITOR` | `F4` 使用的编辑器命令(如 `code --goto "{file}:{line}"`);未设置时自动检测 |

## 常见问题

<details>
<summary><strong>在 Windows 上如何打开 .md 文件?</strong></summary>

安装一次 mdview,它会自动为你关联 `.md` 文件。从此双击任意 Markdown 文件即可像网页一样渲染打开——无需编辑器、无需浏览器扩展、无需配置。
</details>

<details>
<summary><strong>mdview 是免费的吗?</strong></summary>

是的。社区版永久免费,包含完整的阅读体验。Pro 版新增 Mermaid 图表、KaTeX 数学公式和两种编辑模式(F2 实时编辑、F3 双栏),**¥5 一次性永久解锁(微信支付)**——先提供 100 次免费试用,之后只是可忽略的"以后再说"提示,永不硬性锁定。
</details>

<details>
<summary><strong>它和 Typora、VS Code 有什么不同?</strong></summary>

Typora 和 VS Code 是编辑器——功能强大,但只想阅读时太重。mdview 是专用查看器:2.0 MB 安装包,不到一秒打开,`Esc` 即关闭。把它理解为 Markdown 的 Quick Look。它们互为补充。
</details>

<details>
<summary><strong>mdview 会上传我的文件吗?</strong></summary>

不会。mdview 100% 本地运行。它只在 `localhost` 上启动一个临时服务器来渲染页面——你的 Markdown 永远不会离开你的电脑。无账号、无广告、无追踪。
</details>

<details>
<summary><strong>系统要求是什么?</strong></summary>

Windows 10/11(需要 WebView2 运行时,Windows 11 和多数 Windows 10 电脑已预装)· macOS 11 Big Sur 或更高 · Android 7.0 或更高。
</details>

<details>
<summary><strong>国内版和国际版有什么区别?</strong></summary>

同一产品、同一功能,区别只在分发与支付:国内版由 [mdview.cn](https://www.mdview.cn/) 提供下载与更新,Pro 走微信支付 ¥5;国际版由 [mdview.top](https://www.mdview.top/) 提供,下载托管在 GitHub Releases,Pro 走 Gumroad $6.99。两版安装包互相覆盖,数据共用。
</details>

## 技术栈

- **语言**:Rust(edition 2021)
- **GUI**:wry(WebView2)+ tao
- **HTTP**:axum + tokio(仅 localhost)
- **Markdown**:内嵌 md4c
- **文件监听**:notify + SSE 实时刷新
- **样式**:Tailwind CSS
