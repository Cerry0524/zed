# Zed 繁體中文 UI 剩餘盤點

本文件是 Milestone 4 的只讀盤點結果，用來規劃後續 Zed 個人 fork 的全面繁體中文化。此階段不修改 production UI，只建立可執行的覆蓋地圖。

## 盤點來源

- 目前分支：`zh-hant-ui-slice`
- Fork remote：`https://github.com/Cerry0524/zed.git`
- 既有基線 commit：`b3efcbe806 ui: localize command and assistant surfaces in Traditional Chinese`
- 既有參考文件：
  - `docs/localization/zh-Hant-glossary.md`
  - `docs/localization/zh-Hant-ui-slice.md`
  - `docs/superpowers/plans/2026-05-26-zed-zh-hant-remaining-milestones.md`
- 主要掃描命令：

```bash
rg -n 'Label::new\("|Button::new\(|Tooltip::text\("|ContextMenuEntry::new\("|\.title\("|\.description\("' crates
```

另以較嚴格的 visible-English 掃描估算剩餘熱區：

```bash
rg -n 'Label::new\("|Button::new\([^\n]*"[A-Za-z]|Tooltip::text\("[A-Za-z]|ContextMenuEntry::new\("[A-Za-z]|\.title\("[A-Za-z]|\.description\("[A-Za-z]' crates --glob '!**/tests/**' --glob '!**/*test*'
```

`crates/ui/src/l10n.rs` 目前已有約 527 個翻譯 match 項目，包含第一至第三階段已接線的 Command Palette、搜尋、Git、Agent 高頻入口、設定描述、sidebar/title bar 等字串。

## 分類定義

| 分類 | 意義 | 後續處理 |
| --- | --- | --- |
| `translate` | 靜態或半靜態可見 UI 文案 | 加入 `ui::l10n` 詞條並接線 |
| `dynamic` | 來源是 action name、設定 metadata、registry、路徑或遠端資料 | 優先翻固定前後綴；動態內容保留原文 |
| `keep-English` | 品牌、協定、產品、檔名、命令或 code identifier | 只翻周邊說明，不翻 token 本身 |
| `log-test` | telemetry、log、debug、測試、fixture | 暫不列入 UI 翻譯 |
| `needs-design` | 需要複數、插值、安全文案或術語決策 | 先設計文案，再接線 |

## 高密度掃描熱區

以下是 visible-English 掃描中仍有較多候選項目的主要檔案。數字是掃描命中數，不等於一定需要翻譯的最終字串數。

| 檔案 | 命中數 | 主要 surface | 建議 milestone |
| --- | ---: | --- | --- |
| `crates/agent_ui/src/conversation_view/thread_view.rs` | 87 | Agent 對話、工具呼叫、權限、callout、context menu | M8 |
| `crates/collab_ui/src/collab_panel.rs` | 38 | 協作面板、通話、邀請、頻道 | M10 |
| `crates/recent_projects/src/remote_servers.rs` | 24 | 遠端伺服器、Dev Container、WSL/SSH | M6 |
| `crates/git_ui/src/git_panel.rs` | 22 | Git panel、commit、trust、branch diff | M7 |
| `crates/debugger_ui/src/debugger_panel.rs` | 20 | Debugger 面板與空狀態 | M7 |
| `crates/git_graph/src/git_graph.rs` | 16 | Git graph | M7 |
| `crates/git_ui/src/branch_picker.rs` | 16 | 分支切換、遠端建立、刪除 | M7 |
| `crates/recent_projects/src/recent_projects.rs` | 16 | 最近專案與 workspace picker | M6 |
| `crates/agent_ui/src/agent_configuration.rs` | 15 | Agent/MCP/provider 設定 | M8 |
| `crates/ai_onboarding/src/ai_onboarding.rs` | 14 | Zed AI onboarding / Pro upsell | M8 |
| `crates/settings_ui/src/pages/tool_permissions_setup.rs` | 14 | 工具權限設定與 regex 測試 | M9 |
| `crates/workspace/src/pane.rs` | 14 | 窗格/tab context menu | M6 |
| `crates/keymap_editor/src/keymap_editor.rs` | 14 | 鍵盤快速鍵編輯器 | M7 |
| `crates/rules_library/src/rules_library.rs` | 13 | Rules library | M8/M10 |
| `crates/settings_ui/src/settings_ui.rs` | 11 | Settings shell、搜尋、錯誤、trust banner | M9 |
| `crates/onboarding/src/basics_page.rs` | 11 | 首次設定流程 | M10 |

## M6：核心工作區、檔案、搜尋、最近專案

### `translate`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| File Finder placeholder | `crates/file_finder/src/file_finder.rs:1474` | `Search project files...` | 高頻入口，應接 `l10n::text` |
| File Finder footer/filter | `crates/file_finder/src/file_finder.rs:1850`, `1892`, `1909`, `1911`, `1942`, `1962-1974`, `1983` | `Project Scan in Progress…`, `Filter Options`, `Include Ignored Files`, `Split…`, `Split Left`, `Open` | footer、tooltip、split menu |
| File Finder result labels | `crates/file_finder/src/file_finder.rs:1246`, `1250` | `Channel Notes`, `Create file: ...` | 翻固定前綴，路徑保持動態 |
| Recent Projects placeholder | `crates/recent_projects/src/sidebar_recent_projects.rs:137` | `Search recent projects…` | footer buttons 已部分繁中化 |
| Sidebar project/thread chrome | `crates/sidebar/src/sidebar.rs:2172`, `2345`, `2383-2385`, `2431`, `2555-2586` | `Start New Agent Thread`, `Open Project in New Window`, `Focus Project`, `Open Worktrees`, `Move Up`, `Remove` | 專案 header menu 與 threads sidebar |
| Sidebar terminal/recent/search | `crates/sidebar/src/sidebar.rs:5625`, `5704`, `6370-6382`, `6573-6578` | `Close Terminal`, `Add Project`, `No threads match your search.`, `Show Thread History` | 鄰近字串已使用 `l10n`，可延續 |
| Thread import banners | `crates/sidebar/src/sidebar.rs:6654-6703` | `Looking for threads from external agents?`, `Import Threads`, `Threads found from other channels` | 翻靜態段落，channel 名稱保留動態 |
| Project Panel menu | `crates/project_panel/src/project_panel.rs:1080`, `1083-1084` | `Search Inside`, `New File`, `New Folder` | 同段已有多個 `l10n::text`，剩餘硬編碼可補齊 |
| Project Panel prompts | `crates/project_panel/src/project_panel.rs:2235-2236`, `2381-2444`, `4373-4388` | `Discard changes to ...?`, `Trash`, `Delete`, `This cannot be undone.`, `Replace`, `Cancel` | destructive prompt 需注意語氣與複數 |
| Project Panel download/drag | `crates/project_panel/src/project_panel.rs:3346`, `3361-3414`, `7228` | `Download`, `Downloading ... files...`, `Downloaded ... files`, `... entries` | 需要插值/複數處理 |
| Remote connection prompts | `crates/recent_projects/src/remote_connections.rs:318-329`, `379-390` | `Failed to connect over SSH`, `Retry`, `Cancel` | 保留 SSH |
| Disconnected modal body | `crates/recent_projects/src/disconnected_overlay.rs:151-168` | `Your connection to the remote project has been lost.`, `Unsaved changes are stored locally.` | header/buttons 已繁中化，body 尚未 |
| Dev Container suggestion | `crates/recent_projects/src/dev_container_suggest.rs:117-137` | `Would you like to re-open it in a container?`, `Yes, Open in Container`, `Don't Show Again` | Dev Container 保留英文產品名 |
| Security/trust modal | `crates/workspace/src/security_modal.rs:82-84`, `177`, `293-302` | `Unrecognized Project`, `Untrusted projects are opened...`, `Trust all projects...` | 部分 body/buttons 已繁中化 |
| Status bar toggles | `crates/workspace/src/status_bar.rs:230`, `279` | `Open Threads Sidebar`, `Hide Button` | 高頻 tooltip/menu |
| Welcome page constants | `crates/workspace/src/welcome.rs:165-210`, `449-451` | `Get Started`, `New File`, `Configure`, `Welcome to Zed` | 部分 render path 已接線，仍需確認 constants |
| Workspace prompts | `crates/workspace/src/workspace.rs:9430-9455`, `10430-10432` | `Please sign in to continue.`, `Failed to join channel`, `Are you sure you want to restart?`, `Restart`, `Cancel` | prompt copy/buttons |

### `dynamic`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| Project Search | `crates/search/src/project_search.rs:566-575`, `980-1055`, `1728-1788`, `2325-2405` | headings、placeholders、option buttons | 大多已使用 `l10n::text`；後續驗證 dynamic heading |
| Search unsaved prompt | `crates/search/src/project_search.rs:1331-1337` | `Save`, `Don't Save`, `Cancel` 與 prompt sentence | 未完整接線，需 prompt/button 設計 |
| Buffer Search | `crates/search/src/buffer_search.rs:137-194`, `249-312`; `crates/search/src/search.rs:138-203` | `Unified`, `Split`, `Search…`, `Replace with…`, `No more matches` | 多數已繁中，可作為 nearby pattern |
| Workspace notifications | `crates/workspace/src/notifications.rs` | action title/message | 多數來自呼叫端或外部資料；翻固定 action label |

### `keep-English`

- `SSH`
- `WSL`
- `Dev Container`
- `.gitignore`
- `.zed/settings.json`
- `MCP Server`
- `Zed`
- `ACP`
- 外部 agent 名稱，例如 `Claude Agent`、`Codex`

### `needs-design`

- `crates/workspace/src/security_modal.rs:293-302`：trust scope 需要決定「信任父資料夾」與「信任所有專案」語氣。
- `crates/project_panel/src/project_panel.rs:2381-2444`：多檔案刪除/丟垃圾桶提示需要複數與不可復原語氣。
- `crates/search/src/project_search.rs:1331-1337`：未儲存變更 prompt 需搭配 `Save` / `Don't Save` / `Cancel` 三按鈕。

## M7：開發者工作流

### `translate`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| Keymap Editor | `crates/keymap_editor/src/keymap_editor.rs:1155`, `1680`, `2069`, `2083`, `3093-3180` | `This action is unbound`, `Filters`, `Edit in JSON`, `Create Keybinding`, `Edit Keystroke`, `Edit Arguments`, `View`, `Cancel`, `Save` | 鍵盤快速鍵核心 flow |
| Keystroke input | `crates/keymap_editor/src/ui_components/keystroke_input.rs:505`, `523` | `REC`, `SEARCH` | 可視 UI badge，需確認是否保留短英文 |
| Debugger panel | `crates/debugger_ui/src/debugger_panel.rs:644-670`, `1823-1918` | `Edit debug.json`, `Open Documentation`, `Open Debug Adapter Logs`, `New Session`, `Debugger Docs`, `Breakpoints`, `No Breakpoints Set` | 保留 `debug.json` |
| Debug process modal | `crates/debugger_ui/src/new_process_modal.rs:736`, `753`, `783`, `936`, `1488-1520` | `Edit in debug.json`, `Start`, `Debugger:`, `Launch Custom` | debug 設定流程 |
| Diagnostics | `crates/diagnostics/src/buffer_diagnostics.rs:917-944`, `crates/diagnostics/src/diagnostics.rs:769` | `Open File`, `No problems` | LSP 診斷 |
| Outline Panel | `crates/outline_panel/src/outline_panel.rs:4568`, `5067` | `Toggle Panel With`, `Searching:` | 大綱面板 |
| Tasks UI | `crates/tasks_ui/src/modal.rs:669-719` | task edit/spawn/run labels | 部分為 dynamic label，需翻固定 label |
| Terminal panel | `crates/terminal_view/src/terminal_panel.rs` | plus/split/zoom/dropdown tooltips | 部分已由 workspace/pane pattern 覆蓋 |
| Extensions UI | `crates/extensions_ui/src/components/extension_card.rs:56`, `crates/extensions_ui/src/extensions_ui.rs:1457`, `1738` | `Overridden by dev extension.`, `View Documentation`, `All` | 部分已接 `l10n` |
| Git conflict view | `crates/git_ui/src/conflict_view.rs:306`, `325`, `344`, `365` | `Use ...`, `Use Both`, `Resolve with Agent` | branch 名稱保持動態 |
| Commit view/panel | `crates/git_ui/src/commit_view.rs:634`, `1197`; `crates/git_ui/src/git_panel.rs:4900-5578` | `Commit SHA`, `Show in Git Graph`, `This will update your most recent commit.`, `Trust Directory`, `No Git Repositories`, `Initialize Repository` | Git 高頻 flow |
| Branch/worktree/stash pickers | `crates/git_ui/src/branch_picker.rs`, `worktree_picker.rs`, `stash_picker.rs` | `Create`, `Delete`, `Open in New Window`, `Switch`, `Drop`, `Pop`, `Apply` | 需避開 Stage/暫存 與 Stash/貯藏混淆 |

### `dynamic`

- Command Palette 已在 `crates/command_palette/src/command_palette.rs` 使用 `l10n::text_or_original`，後續需擴充 action name 詞條而非大改接線。
- `diagnostics` 的 LSP 訊息多由語言伺服器提供，不應翻動態錯誤內容，只翻按鈕與標題。
- Git branch、remote、commit SHA、path、repo name 保持動態原文。

### `keep-English`

- `JSON`
- `debug.json`
- `tasks.json`
- `Git`
- `GitHub`
- commit SHA / branch name / remote URL
- language server error body

## M8：Agent、AI、Provider、Copilot、MCP

### `translate`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| AI onboarding / plan upsell | `crates/ai_onboarding/src/ai_onboarding.rs:140`, `157`, `159`, `165`, `187`, `206`, `224`, `239`, `259`, `268`, `290-341` | `Welcome to Zed AI`, `Try Zed Pro for Free`, `Get Started`, `Current Plan`, `Start Free Trial`, `Here's what you get:` | 保留 Zed AI / Zed Pro |
| API keys onboarding | `crates/ai_onboarding/src/agent_api_keys_onboarding.rs:102` | `Start now using API keys from your environment for the following providers:` | provider 名稱保持原文 |
| Copilot auth/onboarding | `crates/copilot_ui/src/sign_in.rs:251`, `253`, `258`, `535`, `537`, `558`, `560`, `588`, `590`, `625`, `635`, `666` | `Use GitHub Copilot in Zed`, `Starting Copilot…`, `Sign in to use GitHub Copilot`, `Authenticate To Use` | 保留 GitHub Copilot |
| Agent configuration | `crates/agent_ui/src/agent_configuration.rs:545`, `550`, `717`, `730`, `755`, `774`, `785`, `863`, `894`, `900`, `929`, `935`, `1060`, `1072`, `1077`, `1093`, `1208`, `1231`, `1259` | `Add Custom Server`, `Install from Extensions`, `Configure MCP Server`, `View Tools`, `Log Out`, `Authenticate`, `Add Agent`, `Install from Registry` | MCP/ACP 保留英文縮寫 |
| Add LLM provider modal | `crates/agent_ui/src/agent_configuration/add_llm_provider_modal.rs:346`, `348`, `455`, `523`, `526`, `566`, `580` | `Models`, `Add Model`, `Remove Model`, `Add LLM Provider`, `Save Provider` | LLM 保留 |
| Agent profile modal | `crates/agent_ui/src/agent_configuration/manage_profiles_modal.rs:187`, `545`, `575`, `589`, `618`, `728`, `769`, `810`, `847`, `884`, `918` | `Profile name`, `Agent Profiles`, `Custom Profiles`, `Add New Profile`, `Configure Default Model`, `Delete Profile`, `Go Back` | Profile 建議譯「設定檔」或「描述檔」，需術語統一 |
| Agent thread view | `crates/agent_ui/src/conversation_view/thread_view.rs:4312`, `4328`, `4351`, `5032`, `5281`, `5529`, `5532`, `6214`, `6229`, `6238`, `6265`, `6573`, `6619`, `7215`, `7825`, `7930`, `8433`, `8443`, `9230`, `9270` | `Change Thinking Effort`, `Restore Checkpoint` tooltip body, `Helpful Response`, `Copy This Agent Response`, `Scroll to Bottom`, `Stop This Command`, `Truncated`, `Open File` | Agent flow 最大熱區 |
| Thread import | `crates/agent_ui/src/thread_import.rs:393`, `395`, `411`, `429` | `Import External Agent Threads`, `No ACP agents available.`, `Import Threads` | ACP 保留 |
| Agent registry | `crates/agent_ui/src/agent_registry_ui.rs:294-318`, `385`, `420`, `440`, `459` | `Loading registry...`, `No agents match your search.`, `Missing registry entry.`, `Visit Agent Repository`, `Not supported on this platform` | registry 動態資料保留 |
| Rules Library | `crates/rules_library/src/rules_library.rs:386`, `398`, `1093`, `1196`, `1214` | `Remove from Default Rules`, `Delete Rule`, `New Rule` | Agent rules |
| Language model providers | `crates/language_models/src/provider/*` | `Loading credentials...`, `Create one by visiting`, `Reset API Key`, `Connect`, `Refresh Models` | provider 名稱保留，固定導覽文案可翻 |
| Edit Prediction UI | `crates/edit_prediction_ui/src/edit_prediction_button.rs` | `Configure Providers`, `This Buffer`, `All Files`, `Eager`, `Subtle`, `Training Data Collection`, `View Docs` | 與 AI/edit prediction flow 相關 |

### `dynamic`

- `crates/agent_ui/src/agent_registry_ui.rs:472`, `480`：registry agent id/description 來自 registry。可翻 `ID:` 或 surrounding labels，description 保留來源語言。
- `crates/agent_ui/src/conversation_view/thread_view.rs`：tool command、檔名、agent 回覆、模型名稱、session id 不翻；只翻 chrome、tooltip、permission prompt。
- `language_models/src/provider/*`：provider/product names 保留，固定說明句可翻。

### `keep-English`

- `Zed`
- `Zed AI`
- `Zed Pro`
- `GitHub Copilot`
- `Copilot Chat`
- `OpenAI`
- `OpenAI-compatible`
- `LLM`
- `MCP`
- `ACP`
- `WSL2`
- `AGENTS.md`
- `settings.json`
- provider names：Anthropic、OpenRouter、Ollama、LM Studio、Bedrock、Mistral、DeepSeek、xAI

### `needs-design`

- `Thinking Effort`：需要全域術語。建議暫定「推理強度」，Milestone 8 實作前再檢查使用語境。
- Agent tool permission：`Confirm`、`Allow`、`Deny`、`Default Permission`、`Default Action`、tool input 需要和 Settings tool permissions 統一。
- 外部來源 prompt/safety callouts：如 `Review before sending`、`External Agents currently don't support multi-root workspaces`，需避免誤導安全語意。

## M9：Settings UI 與描述

### `translate`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| Settings shell | `crates/settings_ui/src/settings_ui.rs:438`, `1514`, `2552`, `2565`, `3372`, `3389`, `3397`, `3439`, `3450` | `Edit in settings.json`, `Search settings…`, `View Other Projects`, `Failed to load your settings…`, `Restricted Mode`, `Manage Trust` | shell 與錯誤狀態 |
| Tool permissions | `crates/settings_ui/src/pages/tool_permissions_setup.rs:17`, `19`, `25-27`, `376`, `462`, `537`, `557`, `789`, `819`, `957`, `1083`, `1103`, `1139` | `Terminal`, `Commands executed in the terminal`, `Dismiss`, `Test Your Rules`, `No regex matches...`, `Invalid Patterns`, `Default Permission`, `Confirm/Allow/Deny` | 需和 Agent permission 統一 |
| Edit prediction provider setup | `crates/settings_ui/src/pages/edit_prediction_provider_setup.rs:158`, `160`, `251`, `262`, `303` | `Provider`, `Select which provider to use for edit predictions.`, `Visit the`, `to generate an API key.`, `API Key` | 固定說明翻譯 |
| Audio test window | `crates/settings_ui/src/pages/audio_test_window.rs:218`, `224` | `Output Device`, `Input Device` | 設定頁 |
| Feature flags | `crates/settings_ui/src/pages/feature_flags.rs:62`, `71` | `enabled for all`, `Reset` | 低頻但 visible |

### `dynamic`

- `crates/settings_ui/src/page_data.rs`：設定 metadata 來源很大；render path 已在 `crates/settings_ui/src/settings_ui.rs:1201`, `1242` 使用 `l10n::text(setting_item.title)` / `l10n::text(setting_item.description)`。Milestone 9 應主攻詞條覆蓋，不必每個 call site 重接線。
- 設定 key、JSON key、enum literal、檔名保持原文。

### `needs-design`

- `Profile` 在 Agent 設定與 Settings 之間需統一為「設定檔」或「描述檔」。建議採「設定檔」，但需避開 `Settings File` 的「設定檔」混淆時可用「代理設定檔」。
- tool permission 的 `Default Permission` / `Default Action` 需明確區分「預設權限」與「預設動作」。

## M10：其他全域 surface 與 polish

### `translate`

| Surface | 檔案位置 | 代表字串 | 備註 |
| --- | --- | --- | --- |
| Collab panel | `crates/collab_ui/src/collab_panel.rs` | `Calling`, `Guest`, `Mic only`, `Open Shared Screen`, `Open Channel Notes`, `Search for new contact`, `Create Channel`, `Accept`, `Decline`, `Close` | 協作面板集中在 M10 |
| Collab modals/notifications | `crates/collab_ui/src/collab_panel/channel_modal.rs`, `contact_finder.rs`, `incoming_call_notification.rs`, `project_shared_notification.rs` | `Copy Link`, `Manage Members`, `Invite Members`, `Contacts`, `Accept`, `Dismiss` | 低於核心工作流但 visible |
| Onboarding | `crates/onboarding/src/basics_page.rs`, `onboarding.rs`, `multibuffer_hint.rs` | `Theme`, `Base Keymap`, `Import Settings`, `Agent Setup`, `Finish Setup`, `Learn More` | 首次啟動體驗 |
| Language tools | `crates/language_tools/src/key_context_view.rs`, `syntax_tree_view.rs`, `highlights_tree_view.rs`, `lsp_button.rs` | `Keyboard Context`, `Current Context Stack`, `Last Keystroke`, `No highlights found`, `Language Servers can't run...` | 開發者低頻工具 |
| Dev Container | `crates/dev_container/src/lib.rs` | `Select Feature`, `Confirm Selections`, `Search for Dev Container Templates`, `Overwrite`, `Querying template registry...` | 保留 Dev Container |
| Quick action bar / REPL | `crates/zed/src/zed/quick_action_bar.rs`, `quick_action_bar/repl_menu.rs` | `Selection Controls`, `Editor Controls`, `Edit Predictions`, `Inline Diagnostics`, `Interrupt`, `Clear Outputs`, `Select Kernel` | M10/M7 交界 |
| Toolchain selector | `crates/toolchain_selector/src/toolchain_selector.rs` | `Select Toolchain Path`, `Scope`, `Select` | M10 |
| Open URL / migrate / telemetry | `crates/zed/src/zed/open_url_modal.rs`, `migrate.rs`, `telemetry_log.rs` | `Paste a URL to open.`, `Backup and Update`, `Clear Events`, `Open Raw Log File` | 低頻 visible |

### M10 執行更新

- 已補上 Quick Action Bar、REPL menu、Telemetry log、Migration banner 的固定 UI 文案。
- 已補上 Collab panel、channel modal、contact finder、incoming/project shared notification、call diagnostics 的主要可見文案。
- 已補上 Onboarding、Multibuffer hint、Language tools、Dev Container、Toolchain selector、REPL outputs/sessions/notebook 的剩餘固定文案。
- 已建立 `docs/zh-Hant/allowlist.md`，將品牌/協定、程式識別字、動態外部內容、telemetry/log/test、掃描假陽性分開管理。
- M10 掃描後仍可接受的典型殘留：`IconButton::new("...")` element id、動態 label、`zed://...` 協定範例、測試/fixture/log 內容。

### `log-test`

- `*_tests.rs`
- telemetry event names
- assertion messages
- visual test runner sample labels
- debug-only syntax/highlight tree content that is not normal UI

## 已完成或部分完成的區域

- `crates/ui/src/l10n.rs` 已建立 `text`、`text_for_locale`、`text_or_original` 與 locale 判斷。
- Command Palette placeholder、action humanization 與 intercept result 已接 `l10n`。
- Project Search / Buffer Search 大量高頻控制已接 `l10n`。
- Title bar、security modal、sidebar、theme selector、recent projects footer、welcome page部分內容已接 `l10n`。
- Agent thread view 已完成第三階段高頻入口，但仍是最大剩餘熱區。

## 後續使用方式

1. Milestone 5 先整理 `l10n` 架構，避免翻譯表繼續膨脹到難維護。
2. Milestone 6 以本文件 M6 表格為工作清單。
3. Milestone 7 以 developer workflow 表格為工作清單。
4. Milestone 8 聚焦 Agent/AI，並先決定 `Thinking Effort`、tool permission 相關術語。
5. Milestone 9 用 settings metadata 方式補大量 description 詞條。
6. Milestone 10 重新跑 visible-English scan，更新本文件為剩餘 allowlist。
7. Milestone 11 後，日常回歸檢查使用 `script/check-zh-hant-ui-strings`；全 repo inventory 使用 `script/check-zh-hant-ui-strings --all`。
