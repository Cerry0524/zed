# Zed 繁體中文 UI Milestone 覆蓋地圖

本文件把 Milestone 4 的盤點結果分配到後續大階段，讓每次 goal 都能有清楚邊界、驗收條件與 commit 範圍。

## 執行順序

| 順序 | Milestone | 主題 | 是否修改 production UI | 主要產出 |
| ---: | --- | --- | --- | --- |
| 1 | M5 | l10n 架構與測試整理 | 是 | 穩定 `ui::l10n` 結構、測試與詞條維護方式 |
| 2 | M6 | 核心工作區與檔案操作 | 是 | workspace、project panel、file finder、search、recent projects |
| 3 | M7 | 開發者工作流 | 是 | keymap、tasks、terminal、diagnostics、debugger、Git、extensions |
| 4 | M8 | Agent / AI / Provider / Copilot | 是 | agent panel、provider、MCP、Copilot、AI onboarding |
| 5 | M9 | Settings UI 與描述 | 是 | settings title/description/action link/tool permissions |
| 6 | M10 | 全域漏翻與 polish | 是 | collab、onboarding、language tools、dev container、quick action bar |
| 7 | M11 | Guardrails | 可選 script/docs | 漏翻檢查、allowlist、維護 checklist |
| 8 | M12 | 最終驗證與 release notes | 文件為主 | 最終 build/smoke/release notes |

## M5：l10n 架構與測試

### 範圍

- `crates/ui/src/l10n.rs`
- 可選拆分：
  - `crates/ui/src/l10n/mod.rs`
  - `crates/ui/src/l10n/zh_hant.rs`

### 來源需求

- `crates/ui/src/l10n.rs` 目前已累積約 527 個 match 項目，後續會快速膨脹。
- 需要保留既有 API：
  - `ui::l10n::text(...)`
  - `ui::l10n::text_or_original(...)`
  - `ui::l10n::text_for_locale(...)`

### 驗收

- `cargo test -p ui l10n --lib` 通過。
- 第三階段既有翻譯測試仍通過。
- 外部呼叫介面不變。
- commit：`ui: organize Traditional Chinese localization table`

## M6：核心工作區與檔案操作

### 主要檔案

- `crates/workspace/src/pane.rs`
- `crates/workspace/src/security_modal.rs`
- `crates/workspace/src/status_bar.rs`
- `crates/workspace/src/welcome.rs`
- `crates/workspace/src/workspace.rs`
- `crates/project_panel/src/project_panel.rs`
- `crates/file_finder/src/file_finder.rs`
- `crates/search/src/project_search.rs`
- `crates/search/src/buffer_search.rs`
- `crates/recent_projects/src/recent_projects.rs`
- `crates/recent_projects/src/sidebar_recent_projects.rs`
- `crates/recent_projects/src/remote_servers.rs`
- `crates/recent_projects/src/remote_connections.rs`
- `crates/recent_projects/src/disconnected_overlay.rs`
- `crates/recent_projects/src/dev_container_suggest.rs`
- `crates/sidebar/src/sidebar.rs`
- `crates/ui/src/l10n.rs`

### 字串來源

- File Finder：`Search project files...`、`Project Scan in Progress…`、`Filter Options`、`Split…`、`Open`
- Project Panel：`Search Inside`、`New Folder`、destructive prompt、download progress、replace prompt
- Sidebar：`Start New Agent Thread`、`Open Project in New Window`、`No threads match your search.`
- Remote/Recent：remote disconnect body、SSH failure prompt、Dev Container suggestion
- Workspace Security：`Unrecognized Project`、trust checkbox labels
- Search：unsaved prompt 與 dynamic headings

### 驗收

- 主工作區、檔案尋找、搜尋、專案面板、最近專案的高頻英文明顯下降。
- `SSH`、`WSL`、`Dev Container`、`.gitignore` 等保留英文 token。
- destructive prompt 經過文案設計，不只硬塞逐字翻譯。
- commit：`ui: localize core workspace and project surfaces`

## M7：開發者工作流

### 主要檔案

- `crates/command_palette/src/command_palette.rs`
- `crates/keymap_editor/src/keymap_editor.rs`
- `crates/keymap_editor/src/ui_components/keystroke_input.rs`
- `crates/tasks_ui/src/modal.rs`
- `crates/terminal_view/src/terminal_panel.rs`
- `crates/diagnostics/src/`
- `crates/outline_panel/src/outline_panel.rs`
- `crates/project_symbols/src/`
- `crates/debugger_ui/src/`
- `crates/git_ui/src/`
- `crates/git_graph/src/git_graph.rs`
- `crates/extensions_ui/src/`
- `crates/ui/src/l10n.rs`

### 字串來源

- Keymap：`This action is unbound`、`Filters`、`Edit in JSON`、`Create Keybinding`、`Edit Keystroke`
- Debugger：`New Session`、`Edit debug.json`、`Debugger Docs`、`Breakpoints`、`No Breakpoints Set`
- Git：`Commit SHA`、`Show in Git Graph`、`Trust Directory`、`Initialize Repository`
- Branch/Worktree/Stash：`Switch`、`Create`、`Open in New Window`、`Drop`、`Pop`、`Apply`
- Diagnostics：`Open File`、`No problems`
- Extensions：`Overridden by dev extension.`、docs/category buttons

### 驗收

- Git 術語保持一致：`Stage=暫存`、`Stash=貯藏`。
- `debug.json`、`tasks.json`、commit SHA、branch name 不翻。
- Command Palette action name 以詞條擴充，不改壞搜尋。
- commit：`ui: localize developer workflow surfaces`

## M8：Agent / AI / Provider / Copilot

### 主要檔案

- `crates/agent_ui/src/conversation_view/thread_view.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/agent_ui/src/agent_configuration.rs`
- `crates/agent_ui/src/agent_configuration/add_llm_provider_modal.rs`
- `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs`
- `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs`
- `crates/agent_ui/src/agent_registry_ui.rs`
- `crates/agent_ui/src/thread_import.rs`
- `crates/agent_ui/src/profile_selector.rs`
- `crates/agent_ui/src/ui/end_trial_upsell.rs`
- `crates/ai_onboarding/src/`
- `crates/copilot_ui/src/sign_in.rs`
- `crates/language_models/src/provider/`
- `crates/edit_prediction_ui/src/`
- `crates/rules_library/src/rules_library.rs`
- `crates/ui/src/l10n.rs`

### 字串來源

- AI onboarding：`Welcome to Zed AI`、`Try Zed Pro for Free`、`Start Free Trial`
- Copilot：`Use GitHub Copilot in Zed`、`Starting Copilot…`、`Authenticate To Use`
- Agent configuration：`Add Custom Server`、`Configure MCP Server`、`View Tools`、`Add Agent`
- Add LLM provider：`Models`、`Add Model`、`Remove Model`、`Save Provider`
- Agent profile：`Agent Profiles`、`Custom Profiles`、`Add New Profile`、`Configure Default Model`
- Agent thread：`Change Thinking Effort`、`Helpful Response`、`Copy This Agent Response`、`Stop This Command`
- Registry/import：`Loading registry...`、`No agents match your search.`、`Import External Agent Threads`
- Provider docs：`Loading credentials...`、`Create one by visiting`、`Connect`、`Refresh Models`

### 需先決定的術語

- `Thinking Effort`：暫定「推理強度」。
- `Profile`：Agent 語境用「代理設定檔」。
- `Default Permission` / `Default Action`：分別用「預設權限」/「預設動作」。

### 驗收

- Agent panel 主流程、provider setup、MCP/Copilot 權限流程可繁中使用。
- 安全與權限文字清楚，不改變風險語意。
- `Zed AI`、`Zed Pro`、`GitHub Copilot`、`MCP`、`ACP`、`LLM` 保留英文 token。
- commit：`ui: localize agent and AI workflows`

## M9：Settings UI 與描述

### 主要檔案

- `crates/settings_ui/src/settings_ui.rs`
- `crates/settings_ui/src/page_data.rs`
- `crates/settings_ui/src/pages/tool_permissions_setup.rs`
- `crates/settings_ui/src/pages/edit_prediction_provider_setup.rs`
- `crates/settings_ui/src/pages/audio_test_window.rs`
- `crates/settings_ui/src/pages/feature_flags.rs`
- `crates/settings_ui/src/components/`
- `crates/settings_content/src/`
- `crates/settings/src/`
- `crates/ui/src/l10n.rs`

### 字串來源

- Settings shell：`Edit in settings.json`、`Search settings…`、`View Other Projects`、`Manage Trust`
- Tool permissions：`Test Your Rules`、`Invalid Patterns`、`No patterns configured`、`Default Permission`、`Default Action`
- Metadata：`crates/settings_ui/src/page_data.rs` 大量 title/description
- Edit prediction provider：`Select which provider to use for edit predictions.`、`API Key`

### 驗收

- Settings title/description/action link/button 有一致繁中覆蓋。
- 設定 key、JSON key、code literal 不翻。
- long descriptions 自然，不機翻。
- commit：`ui: localize settings descriptions`

## M10：全域漏翻與 polish

### 主要檔案

- `crates/collab_ui/src/`
- `crates/onboarding/src/`
- `crates/language_tools/src/`
- `crates/dev_container/src/lib.rs`
- `crates/zed/src/zed/quick_action_bar.rs`
- `crates/zed/src/zed/quick_action_bar/repl_menu.rs`
- `crates/toolchain_selector/src/`
- `crates/zed/src/zed/open_url_modal.rs`
- `crates/zed/src/zed/migrate.rs`
- `crates/zed/src/zed/telemetry_log.rs`
- any remaining UI crate reported by global scan

### 字串來源

- Collab：`Calling`、`Guest`、`Mic only`、`Open Shared Screen`、`Create Channel`、`Accept`、`Decline`
- Onboarding：`Theme`、`Base Keymap`、`Import Settings`、`Agent Setup`、`Finish Setup`
- Language tools：`Keyboard Context`、`Current Context Stack`、`No highlights found`
- Dev Container：`Select Feature`、`Confirm Selections`、`Search for Dev Container Templates`
- Quick action bar：`Selection Controls`、`Editor Controls`、`Inline Diagnostics`
- REPL：`Interrupt`、`Clear Outputs`、`Select Kernel`

### 驗收

- 跑全域 visible-English scan，剩餘項目都有 allowlist 理由。
- 修掉過長/語氣不自然的繁中。
- 不翻 telemetry、tests、debug-only sample、外部資料。
- commit：`ui: polish Traditional Chinese localization coverage`

### M10 執行摘要

- 補齊 Quick Action Bar、REPL、Telemetry、Migration、Collab、Onboarding、Language tools、Dev Container、Toolchain selector 的靜態 UI 詞條。
- 建立 `docs/zh-Hant/allowlist.md` 作為 M11 guardrail 腳本的分類來源。
- M10 掃描中仍保留英文的項目以品牌/協定、程式識別字、動態外部內容、log/test/fixture、掃描假陽性為主。

## M11：Guardrails

### 主要檔案

- `docs/zh-Hant/localization-checklist.md`
- `docs/zh-Hant/allowlist.md`
- 可選：`script/check-zh-hant-ui-strings`

### 內容

- 建立一條可手動執行的 visible-English scan。
- 建立 allowlist 分類：
  - brand/protocol
  - code identifier
  - dynamic external content
  - telemetry/log/test
  - visual fixture/sample
- 建立 upstream merge 後如何更新詞條與掃描的 checklist。

### 驗收

- 有一條命令可找出疑似未翻 UI。
- allowlist 可維護，避免未來掃描噪音太大。
- commit：`test: add Traditional Chinese localization guardrails`

### M11 執行摘要

- 新增 advisory guardrail：`script/check-zh-hant-ui-strings`。
- 新增 guardrail 測試：`python3 script/test-check-zh-hant-ui-strings`。
- 預設掃描已繁中化與 M10 盤點過的 UI surface；`--all` 可用於全 repo inventory。
- `docs/zh-Hant/allowlist.md` 含腳本可讀的 allowlist 表格。
- `docs/zh-Hant/localization-checklist.md` 記錄 upstream merge 後的檢查流程。

## M12：最終驗證與 release notes

### 主要檔案

- `docs/zh-Hant/release-notes.md`
- `docs/zh-Hant/ui-localization-audit.md`
- `docs/zh-Hant/localization-checklist.md`

### 驗證命令

```bash
git status --short --branch
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo fmt --check
git diff --check
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-final --system-specs
```

### GUI smoke screens

- Welcome
- Command Palette
- Settings
- Project Panel
- Search
- Recent Projects
- Agent Panel
- Provider/Copilot screens
- Theme selector
- Git/Diff/Commit screens

### 驗收

- build/test/smoke 通過。
- GUI smoke 完成或交由使用者逐項確認。
- release notes 說明完成範圍、驗證命令、已知保留英文。
- branch clean and pushed。
- commit：`docs: summarize Traditional Chinese UI localization`

## Goal 呼叫對照

| Milestone | 建議 goal |
| --- | --- |
| M5 | `按照 Milestone 5 執行：整理 Zed 個人 fork 的繁中 l10n 架構與測試，保留既有 API，相容第三階段成果。完成後驗證、commit 並 push。` |
| M6 | `按照 Milestone 6 執行：全面繁中化核心工作區、專案面板、檔案操作、搜尋與最近專案等主畫面 UI。完成後測試、build、smoke、commit 並 push。` |
| M7 | `按照 Milestone 7 執行：全面繁中化開發者工作流 UI，包含命令、鍵盤快速鍵、工作、終端機、診斷、大綱、符號、偵錯、Git、延伸模組。完成後驗證、commit 並 push。` |
| M8 | `按照 Milestone 8 執行：深度繁中化 Agent、AI、Provider、MCP、Skills、Copilot 與工具權限流程 UI。完成後驗證、commit 並 push。` |
| M9 | `按照 Milestone 9 執行：全面繁中化 Settings UI、設定描述、ActionLink、SubPageLink、JSON 設定說明與修改來源標示。完成後驗證、commit 並 push。` |
| M10 | `按照 Milestone 10 執行：進行全域英文 UI 漏翻掃描、allowlist 分類、繁中文字長度與語氣修整。完成後驗證、commit 並 push。` |
| M11 | `按照 Milestone 11 執行：建立繁中 UI 漏翻檢查 guardrails、allowlist 與維護 checklist，避免後續 upstream merge 後大量英文回流。完成後驗證、commit 並 push。` |
| M12 | `按照 Milestone 12 執行：完成 Zed 繁體中文 UI 分支最終驗證、GUI smoke、release notes 與分支整理。完成後 commit、push，並標記 goal complete。` |
