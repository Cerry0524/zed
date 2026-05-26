# Zed 繁體中文術語表

本術語表延續 `docs/localization/zh-Hant-glossary.md`，並加入 Milestone 4 後續全面繁中化需要的 AI、Git、Settings、Remote、Debugger 詞彙。原則是盡量採用 VS Code 繁體中文常見譯法，同時保留 Zed、品牌、協定、檔名與程式語言 token。

## 翻譯原則

- 使用台灣繁體中文（`zh-Hant` / `zh-TW`）。
- 短按鈕與選單優先簡潔，長說明句優先自然與明確。
- 產品名、協定名、設定 key、檔名、指令、路徑、branch/repo/user-provided text 不翻。
- 一個概念只用一個主要譯法，避免同頁混用。
- `Stage` 與 `Stash` 必須清楚區分：`Stage` 是「暫存」，`Stash` 是「貯藏」。
- `Project` 是「專案」，`Workspace` 是「工作區」，不要互換。

## 核心 UI

| English | 繁體中文 | 類別 | 備註 | 是否保留英文 |
| --- | --- | --- | --- | --- |
| Command Palette | 命令選擇區 | Core UI | VS Code 繁中常見譯法 | 否 |
| Settings | 設定 | Core UI | 設定頁與 UI label | 否 |
| Settings Editor | 設定編輯器 | Core UI | 與 `settings.json` 區分 | 否 |
| Settings File | 設定檔 | Core UI | 指 JSON 設定檔 | 否 |
| Workspace | 工作區 | Core UI | 整個編輯工作區 | 否 |
| Project | 專案 | Core UI | Zed project/root model | 否 |
| Project Panel | 專案面板 | Core UI | 檔案樹側邊面板 | 否 |
| File | 檔案 | Core UI | 一般檔案 | 否 |
| Folder | 資料夾 | Core UI | directory | 否 |
| File Finder | 檔案尋找器 | Navigation | 檔案快速開啟 picker | 否 |
| Recent Projects | 最近專案 | Navigation | welcome/recents | 否 |
| Window | 視窗 | Core UI | macOS/app window | 否 |
| Pane | 窗格 | Core UI | split editor pane | 否 |
| Tab | 索引標籤 | Core UI | editor tabs | 否 |
| Panel | 面板 | Core UI | side/bottom panels | 否 |
| Dock | 停駐區 | Core UI | dock container | 否 |
| Sidebar | 側邊欄 | Core UI | left/right sidebar | 否 |
| Modal | 視窗 | Core UI | 若需避免和 app window 混淆，可用「對話框」 | 否 |
| Tooltip | 工具提示 | Core UI | 文件與開發註記用 | 否 |
| Search | 搜尋 | Search | 搜尋 UI | 否 |
| Replace | 取代 | Search | 搜尋/取代 | 否 |
| Filter | 篩選 | Search | filter UI | 否 |
| Match Case | 符合大小寫 | Search | 既有譯法 | 否 |
| Match Whole Words | 符合完整單字 | Search | 既有譯法 | 否 |
| Regular Expression | 規則運算式 | Search | regex | 否 |
| Open | 開啟 | Action | 通用 action | 否 |
| Close | 關閉 | Action | 通用 action | 否 |
| Save | 儲存 | Action | 通用 action | 否 |
| Save As | 另存新檔 | Action | 通用 action | 否 |
| New | 新增 | Action | 通用 action | 否 |
| Delete | 刪除 | Action | 刪除資料本體 | 否 |
| Remove | 移除 | Action | 從清單/設定移除 | 否 |
| Rename | 重新命名 | Action | 檔案/符號 | 否 |
| Copy | 複製 | Action | clipboard | 否 |
| Paste | 貼上 | Action | clipboard | 否 |
| Cut | 剪下 | Action | clipboard | 否 |
| Undo | 復原 | Action | edit history | 否 |
| Redo | 重做 | Action | edit history | 否 |
| Retry | 重試 | Action | error recovery | 否 |
| Dismiss | 關閉 | Action | 輕量關閉，不用「略過」 | 否 |
| Configure | 設定 | Action | 動詞 | 否 |
| Install | 安裝 | Action | extensions/tools | 否 |
| Uninstall | 解除安裝 | Action | extensions/tools | 否 |
| Update | 更新 | Action | software/extensions | 否 |
| Reload | 重新載入 | Action | window/project/settings | 否 |
| Select | 選取 | Action | selection / picker confirm | 否 |
| Learn More | 了解更多 | Action | docs link | 否 |

## 開發者工作流

| English | 繁體中文 | 類別 | 備註 | 是否保留英文 |
| --- | --- | --- | --- | --- |
| Extensions | 延伸模組 | Extensions | 延續既有 glossary；不採「擴充功能」 | 否 |
| Keybindings | 鍵盤快速鍵 | Input | 同 Keyboard Shortcuts | 否 |
| Keyboard Shortcuts | 鍵盤快速鍵 | Input | 同 Keybindings | 否 |
| Keymap | 按鍵對應 | Input | 檔案或 keymap 概念 | 否 |
| Keystroke | 按鍵 | Input | 輸入記錄 UI | 否 |
| Terminal | 終端機 | Terminal | 內建終端機 | 否 |
| Task | 工作 | Task | Zed tasks | 否 |
| Run | 執行 | Action | task/command/debug | 否 |
| Spawn | 產生 | Task | 既有 Zed task wording | 否 |
| Diagnostics | 診斷 | LSP | LSP diagnostics | 否 |
| Debugger | 偵錯工具 | Debugger | UI surface | 否 |
| Debug | 偵錯 | Debugger | 動詞/形容詞 | 否 |
| Breakpoint | 中斷點 | Debugger | debugging | 否 |
| Stack Frame | 堆疊框架 | Debugger | debugger stack frame | 否 |
| Console | 主控台 | Debugger | debugger console | 否 |
| Outline | 大綱 | Navigation | document outline | 否 |
| Symbol | 符號 | Navigation | code symbol | 否 |
| References | 參考 | Navigation | find references | 否 |
| Definition | 定義 | Navigation | go to definition | 否 |
| Declaration | 宣告 | Navigation | go to declaration | 否 |
| Type Definition | 型別定義 | Navigation | go to type definition | 否 |
| Editor | 編輯器 | Editor | code editor | 否 |
| Buffer | 緩衝區 | Editor | technical UI; 若句子可用「檔案」則優先自然 | 否 |
| Selection | 選取範圍 | Editor | selected text/range | 否 |
| Cursor | 游標 | Editor | text cursor | 否 |
| Inline | 行內 | Editor | inline assist/diagnostics | 否 |
| Completion | 自動完成 | Editor | completion UI | 否 |
| Prediction | 預測 | Editor/AI | edit prediction | 否 |
| REPL | REPL | Developer | 技術縮寫保留 | 是 |
| JSON | JSON | Format | 格式名保留 | 是 |
| `settings.json` | `settings.json` | File | 檔名保留 | 是 |
| `debug.json` | `debug.json` | File | 檔名保留 | 是 |
| `tasks.json` | `tasks.json` | File | 檔名保留 | 是 |

## Git / Version Control

| English | 繁體中文 | 類別 | 備註 | 是否保留英文 |
| --- | --- | --- | --- | --- |
| Git | Git | Version control | 產品/工具名保留 | 是 |
| Version Control | 版本控制 | Version control | 類別名 | 否 |
| Repository | 儲存庫 | Version control | repo | 否 |
| Remote | 遠端 | Version control | remote repo/server | 否 |
| Commit | 提交 | Version control | 既有 l10n | 否 |
| Commit Message | 提交訊息 | Version control | commit message | 否 |
| Commit SHA | Commit SHA | Version control | label 中可保留 SHA | 是 |
| Branch | 分支 | Version control | 既有 l10n | 否 |
| Current Branch | 目前分支 | Version control | branch picker | 否 |
| Selected Branch | 已選取分支 | Version control | branch picker | 否 |
| Diff | 差異 | Version control | View Diff = 檢視差異 | 否 |
| Branch Diff | 分支差異 | Version control | 既有 l10n | 否 |
| Hunk | 變更區塊 | Version control | diff hunk | 否 |
| Stage | 暫存 | Version control | Git stage | 否 |
| Unstage | 取消暫存 | Version control | Git unstage | 否 |
| Stash | 貯藏 | Version control | 不用「暫存」，避免和 Stage 混淆 | 否 |
| Pop Stash | 取出貯藏 | Version control | stash pop | 否 |
| Apply Stash | 套用貯藏 | Version control | stash apply | 否 |
| Drop Stash | 刪除貯藏 | Version control | stash drop | 否 |
| Worktree | 工作樹 | Version control | Git worktree | 否 |
| Blame | Blame | Version control | 可保留 Git 術語；必要時「檢視 Blame」 | 是 |
| Pull Request | Pull Request | Version control | 若出現 GitHub 產品語境保留 | 是 |

## Agent / AI

| English | 繁體中文 | 類別 | 備註 | 是否保留英文 |
| --- | --- | --- | --- | --- |
| Agent | 代理 | AI | 既有 l10n | 否 |
| Assistant | 助理 | AI | 既有 glossary | 否 |
| Thread | 對話串 | AI | AI conversation thread | 否 |
| Prompt | 提示 | AI | user prompt | 否 |
| Context | 內容 | AI | 若指 attached context 可用「內容」 | 否 |
| Rules | 規則 | AI | Agent rules | 否 |
| Skill | 技能 | AI | Zed skills | 否 |
| Tool | 工具 | AI | tool call / permission | 否 |
| Tool Call | 工具呼叫 | AI | tool execution card | 否 |
| Permission | 權限 | AI/Security | tool permission | 否 |
| Default Permission | 預設權限 | AI/Security | Settings tool permissions | 否 |
| Default Action | 預設動作 | AI/Security | fallback action | 否 |
| Allow | 允許 | AI/Security | permission button | 否 |
| Deny | 拒絕 | AI/Security | permission button | 否 |
| Ask | 詢問 | AI/Security | permission mode | 否 |
| Approve | 核准 | AI/Security | approval action | 否 |
| Reject | 拒絕 | AI/Security | reject change/action | 否 |
| Provider | 提供者 | AI | LLM provider | 否 |
| Model | 模型 | AI | model selector | 否 |
| Profile | 代理設定檔 | AI | Agent profile；若短 label 可用「設定檔」 | 否 |
| Thinking Effort | 推理強度 | AI | Milestone 8 前再確認語境 | 否 |
| Token | token | AI | 技術單位可保留小寫 | 是 |
| Cost | 費用 | AI | token cost | 否 |
| MCP | MCP | Protocol | 保留縮寫；可寫 MCP 伺服器 | 是 |
| ACP | ACP | Protocol | 保留縮寫；可寫 ACP 代理/登錄檔 | 是 |
| LLM | LLM | AI | 保留縮寫；LLM 提供者 | 是 |
| Copilot | Copilot | Brand | 保留產品名 | 是 |
| GitHub Copilot | GitHub Copilot | Brand | 保留產品名 | 是 |
| Zed AI | Zed AI | Brand | 保留產品名 | 是 |
| Zed Pro | Zed Pro | Brand | 保留產品名 | 是 |
| OpenAI | OpenAI | Brand | 保留產品名 | 是 |
| OpenAI-compatible | OpenAI 相容 | AI | 周邊詞可翻 | 部分 |
| Anthropic | Anthropic | Brand | provider name | 是 |
| OpenRouter | OpenRouter | Brand | provider name | 是 |
| Ollama | Ollama | Brand | provider name | 是 |
| LM Studio | LM Studio | Brand | provider name | 是 |
| Bedrock | Bedrock | Brand | provider name | 是 |
| Mistral | Mistral | Brand | provider name | 是 |
| DeepSeek | DeepSeek | Brand | provider name | 是 |
| xAI | xAI | Brand | provider name | 是 |

## Security / Remote / Collaboration

| English | 繁體中文 | 類別 | 備註 | 是否保留英文 |
| --- | --- | --- | --- | --- |
| Restricted Mode | 受限模式 | Security | 既有 l10n | 否 |
| Trust | 信任 | Security | trust project/folder | 否 |
| Trusted | 已信任 | Security | state | 否 |
| Untrusted | 未信任 | Security | state | 否 |
| Unrecognized Project | 無法識別的專案 | Security | Security modal title | 否 |
| Remote Project | 遠端專案 | Remote | 既有 l10n | 否 |
| Remote Server | 遠端伺服器 | Remote | SSH/WSL server UI | 否 |
| SSH | SSH | Protocol | 保留 | 是 |
| WSL | WSL | Platform | 保留 | 是 |
| WSL2 | WSL2 | Platform | 保留 | 是 |
| Dev Container | Dev Container | Product/feature | 保留，周邊中文 | 是 |
| Channel | 頻道 | Collaboration | collab channel | 否 |
| Contact | 聯絡人 | Collaboration | contacts | 否 |
| Call | 通話 | Collaboration | voice/screen call | 否 |
| Screen | 螢幕 | Collaboration | shared screen | 否 |
| Invite | 邀請 | Collaboration | invite action/state | 否 |
| Guest | 訪客 | Collaboration | collab role | 否 |
| Admin | 管理員 | Collaboration | collab role | 否 |
| Member | 成員 | Collaboration | collab role | 否 |

## 需要後續確認的術語

| English | 暫定譯法 | 問題 | 建議處理 |
| --- | --- | --- | --- |
| Thinking Effort | 推理強度 | 可能出現在 model selector、tooltip、menu header；需確認是否比「思考強度」自然 | Milestone 8 前用 UI 語境審稿 |
| Profile | 代理設定檔 | Settings File 也譯「設定檔」，短 label 可能混淆 | Agent context 用「代理設定檔」 |
| Tool Input | 工具輸入 | Permission regex UI 可能指 command input | Milestone 9 和 Agent permission 一起統一 |
| Default Permission | 預設權限 | 與 Default Action 同頁出現 | 保持兩詞分離 |
| Default Action | 預設動作 | 與 Default Permission 同頁出現 | 保持兩詞分離 |
| Trust all projects under parent folder | 信任父資料夾下的所有專案 | 句子偏長，checkbox 可能擠壓 | Milestone 6 檢查版面 |
| Delete permanently | 永久刪除 | destructive prompt 需要安全語氣 | Milestone 6 設計完整句 |
| Trash | 丟到垃圾桶 | macOS familiar wording | 已有 l10n，繼續使用 |

## 保留英文原則

以下類型預設保留英文，但可翻周邊文案：

- 品牌：`Zed`、`GitHub Copilot`、`OpenAI`、`Ollama`、`LM Studio`
- 協定/縮寫：`MCP`、`ACP`、`LLM`、`SSH`、`WSL`
- 檔名與設定 key：`settings.json`、`debug.json`、`tasks.json`、`.gitignore`、`AGENTS.md`
- 程式語言、package、command、model id、branch name、commit SHA、URL、路徑
- 外部 provider/registry 回傳的名稱或描述，除非後續有正式 i18n source
