<p align="center">
  <img src="static/logo_big.png" width="96" alt="mdview ロゴ">
</p>

<h1 align="center">mdview</h1>

<p align="center">
  <strong>Markdown を、余計なものなしに。</strong>200 MB のエディターではなく、2.0 MB のビューアー —— .md ファイルをダブルクリックするだけでウェブページのように読め、F2 キーでその場で編集できます。<br>
  Windows · macOS · Android &nbsp;|&nbsp; Rust + WebView2 製、Electron 不使用
</p>

<p align="center">
  <a href="https://www.mdview.top/">公式サイト</a> ·
  <a href="https://github.com/mdview2026/mdview/releases/latest">ダウンロード</a> ·
  <a href="https://www.mdview.top/changelog.html">更新履歴</a> ·
  <a href="https://www.mdview.top/blog/">ブログ</a> ·
  <a href="docs/facts.md">ドキュメント</a>
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a>
</p>

---

`.md` ファイルを開くとき、その大半はただ**読みたいだけ**——README、メモ、AI が生成した回答。そのためにフル機能のエディターを起動するのは大げさです。mdview ならウェブページのように一瞬で開き、編集が必要になったときは `F2` で同じウィンドウ内で Obsidian スタイルの WYSIWYG 編集ができます——他のアプリへ切り替える必要はありません。

- **超軽量。** 2.0 MB のインストーラー、1 秒未満で起動、設定不要
- **余計なものゼロ。** クリーンな全画面読書——ツールバーも、勝手に出るサイドバーもありません
- **その場で編集。** `F2` Obsidian スタイルの WYSIWYG、`F3` ライブプレビュー付き 2 段組編集
- **100% ローカル**——アカウント不要、広告なし、ファイルがあなたの PC の外に出ることはありません

<p align="center">
  <img src="web_en/img/shots125/gallery-01-hero.png" alt="mdview が Markdown ファイルをウェブページのようにレンダリング——左にアウトラインパネル、見出し、リスト、表、ハイライトされたコードブロック" width="1080">
</p>

## ダウンロード

| プラットフォーム | Community(永久無料)| Pro($6.99 買い切り)|
|---|---|---|
| Windows 10/11 | [mdview-setup-en-latest.exe](https://github.com/mdview2026/mdview/releases/latest/download/mdview-setup-en-latest.exe) | [mdview-setup-pro-en-latest.exe](https://github.com/mdview2026/mdview/releases/latest/download/mdview-setup-pro-en-latest.exe) |
| macOS 11+ | [mdview-macos-en-latest.dmg](https://github.com/mdview2026/mdview/releases/latest/download/mdview-macos-en-latest.dmg) | [mdview-macos-pro-en-latest.dmg](https://github.com/mdview2026/mdview/releases/latest/download/mdview-macos-pro-en-latest.dmg) |
| Android 7.0+ | [mdview.apk](https://www.mdview.top/download/mdview.apk) | — |

両エディションは同じ製品で、インストール時に相互に上書きされます——いつでも切り替え可能。Android は Community のみです。

> **SmartScreen の警告が出る?** Windows は高額なコード署名証明書を持たないアプリに警告を出します。**「詳細情報」→「実行する」**をクリックしてください。mdview は完全にローカルで動作し、何もアップロードしません。

## mdview を選ぶ理由

あなたが `.md` ファイルをダブルクリックするのは読むため——IDE を起動するためではありません。**Markdown 版 Quick Look** と思ってください:

| | mdview | Typora / VS Code | ブラウザー拡張 |
|---|---|---|---|
| インストールサイズ | **2.0 MB** | 100–400 MB | —(ただしブラウザーが必要) |
| .md をダブルクリック | **一瞬** | エディター起動に 3–5 秒 | 毎回ドラッグ&ドロップかコピペ |
| 読書優先の UI | **余計な UI なし** | ツールバー、パネル、タブ | 周りにブラウザーの UI |
| ために作られた | **読むこと** | コード / ドキュメント執筆 | たまのプレビュー |

互いに補い合う関係です——執筆はエディターで、ただ読むだけの 90% の場面は mdview で。

## 機能

### プレビュー

- **ダブルクリックで開く**——インストーラーが `.md` ファイルを自動関連付け。`Ctrl+O`、ドラッグ&ドロップ、エクスプローラーの右クリックメニューにも対応
- **ライブリフレッシュ**——任意のエディターで保存するとページが即座に再レンダリング(SSE プッシュ)、スクロール位置も保持
- **アウトラインパネル**——クリックでジャンプ・スクロール同期のフローティング目次に加え、日付別の**履歴**と**お気に入り**タブ
- **閲覧位置の記憶**——前回読み終わった場所から再開。ファイルごとに単一ウィンドウ
- **9 つのカラーテーマ + ダーク/ライトモード**——Glacier、Forest、Sunset、Typewriter……右クリック 1 回で切替
- **タイポグラフィ制御**——インストール済みフォントを自由に選択。`Ctrl+スクロール` で文字サイズ、`Alt+スクロール` で段幅(ビットマップ拡大ではなく本当の再組版)
- **完全なレンダリング**—— Alignment 対応の表、逆リンク付き脚注、ローカル画像、シンタックスハイライト付きコードブロック(コピー / 折り返し / 行番号)
- **PDF エクスポート**、単一改行を改段として扱うトグル、GBK/GB18030 エンコーディングのフォールバック、自動アップデート確認

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-02a-outline.png" alt="mdview のアウトラインパネルがスクロール中の現在の見出しをハイライト"></td>
    <td><img src="web_en/img/shots125/gallery-02b-history.png" alt="mdview の履歴パネルが最近開いた Markdown ファイルを日付別にグループ化"></td>
  </tr>
  <tr>
    <td align="center"><sub>アウトラインがスクロール位置を追跡</sub></td>
    <td align="center"><sub>履歴とお気に入り、日付別にグループ化</sub></td>
  </tr>
</table>

**9 つのカラーテーマ + ダークモード**、右クリック 1 回で切替:

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-default.png" alt="mdview デフォルトのライトテーマ"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-dark.png" alt="mdview ダークモードのテーマ"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-glacier.png" alt="mdview Glacier テーマ、クールな青系"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-typewriter.png" alt="mdview Typewriter テーマ、等幅タイプライター風"></td>
  </tr>
  <tr>
    <td align="center"><sub>Default</sub></td>
    <td align="center"><sub>Dark</sub></td>
    <td align="center"><sub>Glacier</sub></td>
    <td align="center"><sub>Typewriter</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-sunset.png" alt="mdview Sunset テーマ、温かいアンバー系"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-forest.png" alt="mdview Forest テーマ、落ち着いた緑系"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-editorial.png" alt="mdview Editorial テーマ、雑誌風セリフ"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-violet.png" alt="mdview Violet テーマ、紫系"></td>
  </tr>
  <tr>
    <td align="center"><sub>Sunset</sub></td>
    <td align="center"><sub>Forest</sub></td>
    <td align="center"><sub>Editorial</sub></td>
    <td align="center"><sub>Violet</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-lychee.png" alt="mdview Lychee テーマ、柔らかな赤系"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-mint.png" alt="mdview Mint テーマ、爽やかなミント系"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-neon.png" alt="mdview Neon テーマ、鮮やかなハイコントラスト"></td>
    <td></td>
  </tr>
  <tr>
    <td align="center"><sub>Lychee</sub></td>
    <td align="center"><sub>Mint</sub></td>
    <td align="center"><sub>Neon</sub></td>
    <td></td>
  </tr>
</table>

### 編集

| キー | モード | エディション |
|---|---|---|
| `F2` | **ライブ編集**——プレビュー内で Obsidian スタイルの WYSIWYG:書式、リンク、表(行/列操作)、コールアウト | Pro |
| `F3` | **2 段組編集**——左にソース、右にライブプレビュー、セクション単位でスクロール同期。`Tab` でスニペット展開、画像/ファイルの貼り付けは `.__assets/` に自動保存され Markdown が挿入されます | Pro |
| `F4` / `Ctrl+E` | **外部エディターで編集**——VS Code / Sublime / Notepad++ / メモ帳(自動検出)を選択テキストの正確な行で開きます | 無料 |
| `Esc` / `Ctrl+W` | ウィンドウを閉じる | 無料 |

<p align="center">
  <img src="web_en/img/shots125/gallery-04-editing.png" alt="mdview の F3 2 段組編集:左に Markdown ソース、右にライブレンダリングされたプレビュー" width="1080">
</p>

### Pro レンダリング

- **Mermaid ダイアグラム**(フローチャート / シーケンス / ガント)——ダブルクリックで全画面ビューアー、ホイールズーム・ドラッグパン対応
- **KaTeX 数式**——インライン `$...$` とブロック `$$...$$` の LaTeX
- レンダリングエンジンは初回使用時に 1 回だけダウンロード、以降は完全オフラインで動作

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-05a-mermaid.png" alt="mdview Pro が Markdown ドキュメント内でレンダリングした Mermaid フローチャート"></td>
    <td><img src="web_en/img/shots125/gallery-05b-katex.png" alt="mdview Pro がレンダリングした KaTeX 数式、LaTeX の BM25 方程式"></td>
  </tr>
  <tr>
    <td align="center"><sub>Mermaid ダイアグラム</sub></td>
    <td align="center"><sub>KaTeX 数式</sub></td>
  </tr>
</table>

## 価格

| | Community | Pro |
|---|---|---|
| 価格 | **永久無料**(任意でスポンサー)| **$6.99** 一回限りの買い切り(Gumroad:カード / PayPal) |
| コアな読書機能 | ✅ すべて | ✅ すべて |
| Mermaid + KaTeX | — | ✅ |
| ライブ編集(F2)+ 2 段組編集(F3) | — | ✅ |
| 体験版 | — | 100 回無料でオープン、その後は閉じられる「後で」プロンプトのみ——**ロックされることはありません** |

## はじめ方

1. インストーラー(またはポータブル版 `mdview.exe`)をダウンロードして実行
2. `.md` ファイルが自動的に関連付けされます(解除はいつでも:右クリック → 設定、または `mdview --unbind`)
3. 任意の Markdown ファイルをダブルクリックして読むだけ

## コマンドライン

```bash
mdview                     # 一度実行:.md ファイルを自動関連付け
mdview <file.md>           # 指定ファイルをプレビュー
mdview --install           # エクスプローラーの右クリックメニューに「mdview で開く」を追加
mdview --uninstall         # 右クリックメニューから削除
mdview --settings          # 設定ウィンドウ(関連付け、最近のファイル、エディター)
mdview --unbind            # .md のデフォルト関連付けを解除
mdview --help              # ヘルプ
```

## 環境変数

| 変数 | 説明 |
|---|---|
| `PORT` | HTTP サーバーのポート(デフォルト:ランダムな空きポート、フォールバック 3456) |
| `MD_HTML=1` | `.md` ファイルの隣にレンダリング済み `.html` も出力 |
| `MD_HTML_OUTPUT=<path>` | `.html` を指定パスに出力 |
| `MD_EDITOR` | `F4` 用のエディターコマンド(例:`code --goto "{file}:{line}"`);未設定なら自動検出 |

## よくある質問

<details>
<summary><strong>Windows で .md ファイルを開くには?</strong></summary>

mdview を一度インストールすれば、`.md` ファイルが自動的に関連付けされます。以後、任意の Markdown ファイルをダブルクリックするだけでウェブページのようにレンダリングされて開きます——エディターもブラウザー拡張も設定も不要です。
</details>

<details>
<summary><strong>mdview は無料ですか?</strong></summary>

はい。Community エディションは完全な読書体験を含めて永久無料です。Pro エディションは Mermaid ダイアグラム、KaTeX 数式、2 つの編集モード(F2 ライブ編集、F3 2 段組)を $6.99 の買い切りで追加します——まず 100 回の無料体験があり、その後も閉じられる「後で」プロンプトが表示されるだけで、ロックされることはありません。
</details>

<details>
<summary><strong>Typora や VS Code と何が違いますか?</strong></summary>

Typora と VS Code は**エディター**です——高機能ですが、ただ読みたいだけのときには重すぎます。mdview は専用ビューアー:2.0 MB のインストーラー、1 秒未満で起動、`Esc` で閉じます。Markdown 版 Quick Look と考えてください。互いに補い合う関係です。
</details>

<details>
<summary><strong>mdview はファイルをアップロードしますか?</strong></summary>

いいえ。mdview は 100% ローカルで動作します。ページのレンダリングのために `localhost` 上に一時的なサーバーを起動するだけです——あなたの Markdown が PC の外に出ることはありません。アカウント不要、広告なし、トラッキングなし。
</details>

<details>
<summary><strong>システム要件は?</strong></summary>

Windows 10/11(WebView2 ランタイム。Windows 11 とほとんどの Windows 10 PC にプリインストール済み)· macOS 11 Big Sur 以降 · Android 7.0 以降。
</details>

## 技術スタック

- **言語**: Rust(edition 2021)
- **GUI**: wry(WebView2)+ tao
- **HTTP**: axum + tokio(localhost のみ)
- **Markdown**: 組み込み md4c
- **ファイル監視**: notify + SSE ライブリロード
- **スタイリング**: Tailwind CSS
