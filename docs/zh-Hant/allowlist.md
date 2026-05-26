# 繁體中文 UI 漏翻 Allowlist

本文件記錄 M10 全域掃描後刻意保留英文或不納入 UI 翻譯的類型。後續 M11 可以把這些分類轉成檢查腳本的 allowlist。

## 腳本可讀 Allowlist

`script/check-zh-hant-ui-strings` 會讀取下列表格。每列必須有三欄：

- `path`：Rust 檔案路徑的 regular expression。
- `pattern`：可見文字內容的 regular expression。
- `reason`：必須對應下方分類，說明為什麼不翻。

| path | pattern | reason |
| --- | --- | --- |
| `crates/.*/tests?/.*` | `.*` | telemetry/log/test |
| `crates/.*/fixtures/.*` | `.*` | telemetry/log/test |
| `crates/.*/snapshots/.*` | `.*` | telemetry/log/test |
| `crates/.*/test_data/.*` | `.*` | telemetry/log/test |
| `crates/zed/src/zed/open_url_modal\.rs` | `^zed://\.\.\.$` | brand/protocol |
| `crates/recent_projects/src/remote_servers\.rs` | `^ssh user@example -p 2222$` | code identifier |
| `crates/zed/src/visual_test_runner\.rs` | `.*` | visual fixture/sample |
| `crates/ui/src/components/.*` | `.*` | visual fixture/sample |
| `crates/workspace/src/theme_preview\.rs` | `.*` | visual fixture/sample |

不要為了讓掃描通過而把真實使用者會看到的靜態 UI 文案加入 allowlist；那類字串應改成 `l10n::text(...)` 並補 `crates/ui/src/l10n/zh_hant.rs` 詞條。

## 品牌與協定

- `Zed`
- `Zed Agent`
- `Zed AI`
- `Zed Pro`
- `GitHub`
- `GitHub Copilot`
- `Jupyter`
- `SSH`
- `WSL`
- `Dev Container`
- `MCP`
- `ACP`
- `LSP`
- `REPL`
- `RPC`
- `WebRTC`

處理原則：品牌、協定、產品名與常見縮寫保留英文，周邊說明文字翻譯。

## 程式識別字

- `settings.json`
- `.zed/settings.json`
- `tasks.json`
- `debug.json`
- `.gitignore`
- `ipykernel`
- `zed://...`
- `AGENTS.md`
- command/action id
- enum、schema、setting key

處理原則：檔名、設定 key、程式 token 不翻，固定提示與按鈕翻譯。

## 動態外部內容

- repository name
- branch name
- commit SHA
- file path
- channel/user/project name
- registry description
- language server diagnostic body
- model/provider name
- agent/tool output

處理原則：只翻固定前後綴與 action label，來源內容保持原文。

## Telemetry、Log、Debug、Test

- telemetry event name
- log level
- protocol trace
- test fixture
- visual test sample
- eval fixture
- parser/debug output

處理原則：不列入使用者 UI 翻譯範圍，除非同一字串也直接出現在可見 UI。

## 掃描假陽性

- `IconButton::new("...")` 的 element id
- `Button::new("id", dynamic_label)` 的 id
- `Label::new(variable)` 的動態值
- `CopyButton::new("id", value)` 的 id
- `editor.set_placeholder_text("zed://...", ...)` 這類協定範例

處理原則：M10 手動檢視後不翻，M11 可在 script 中分類忽略。
