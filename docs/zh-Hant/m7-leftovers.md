# M7 保留英文與後續追蹤

Milestone 7 以選單、命令選擇區、設定、鍵盤快速鍵、通知、對話框與錯誤提示為範圍。以下英文在本階段刻意保留，或因屬於較低頻/不同渲染管線而留給後續 milestone 追蹤。

## 本階段刻意保留

- 品牌與產品：Zed、Zed Pro、Codex、GitHub Copilot、Claude Agent
- 協定與技術縮寫：API、CLI、SSH、WSL、WSL2、MCP、ACP、LLM、LSP、JSON、Vim、GPUI
- 檔名與設定檔：settings.json、keymap.json、tasks.json、debug.json、devcontainer.json、.gitignore
- 使用者/外部資料：模型名稱、agent 名稱、provider 名稱、server 名稱、branch/repo 名稱、路徑、URL、版本號、錯誤詳細堆疊

## 已接線但仍需畫面抽查

- Keymap Editor：動作名稱、篩選器、表格欄位、建立/編輯鍵盤快速鍵 modal、按鍵錄製 tooltip 已接 `zh-Hant`。
- Which-key：顯示用 action humanized label 已改走 `l10n::text_or_original`。
- Settings：視窗標題、搜尋 placeholder、`settings.json` 編輯按鈕與搜尋索引已納入繁中詞條。
- Notifications/dialogs/errors：Agent error callout、Git askpass hint、commit modal 警告、URL modal、共用 prompt OK 按鈕已接繁中。

## 後續 milestone 追蹤

- Agent 設定 modal：MCP Server、LLM Provider、Profiles、Tools 設定頁仍有多個長表單文案。建議下一階段統一處理 Agent Settings / MCP。
- Git remote 與 clone flow：Fetch/Pull/Push toast、Pull Request / Merge Request、clone repository modal 仍需逐頁接線。
- Vim command prompt：命令列錯誤與寫入確認提示仍是獨立路徑，需保留 Vim 指令 token，只翻固定提示。
- JSON schema hover：`settings` / `keymap` schema 來自 doc comments 與 schema generation，不完全走一般 UI l10n，需另設 schema description 策略。
- Edit Prediction rating modal：屬低頻診斷/回饋工具，仍有 `Predicted Patch`、`Expected Patch`、`Bad Prediction` 等英文。
