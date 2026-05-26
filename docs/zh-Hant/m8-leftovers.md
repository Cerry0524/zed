# M8 保留英文與後續追蹤

Milestone 8 聚焦 Agent、AI、Provider、MCP、Skills、Copilot 與工具權限流程 UI。本階段已優先接線高可見入口、設定 modal、權限確認流程與安全文案；以下英文刻意保留，或因屬於 provider 深層設定/外部錯誤來源而留給後續 milestone 追蹤。

## 本階段刻意保留

- 品牌與產品：Zed、Zed AI、Zed Pro、Zed Business、Zed Student、GitHub Copilot、Copilot Chat、Claude Agent、Codex
- 協定與技術縮寫：API、LLM、MCP、ACP、JSON、OAuth、IAM、SSO、AWS、WSL、WSL2
- 設定檔與規格名稱：settings.json、AGENTS.md、SKILL.md、parallel_tool_calls、prompt_cache_key、/chat/completions
- Provider 與模型名稱：OpenAI、OpenAI-compatible、Anthropic、OpenRouter、Ollama、LM Studio、Bedrock、Mistral、DeepSeek、xAI、Vercel AI Gateway、模型 ID
- 使用者/外部資料：agent 名稱、server 名稱、provider 名稱、profile 名稱、registry description、URL、email、env var、shell command、API error body

## 已接線但仍需畫面抽查

- AI onboarding：Zed AI plan 文案、API keys onboarding、Copilot fallback onboarding 已接 `zh-Hant`。
- Copilot auth：登入/重新安裝/裝置碼/訂閱錯誤/編輯預測與 chat 設定說明已接繁中。
- Agent settings：LLM Providers、MCP Server、External Agents、Agent Profiles、Tool picker 與主要 menu/tooltip 已接繁中。
- Tool permissions：工具清單、預設權限、Always Allow/Deny/Confirm、安全警告、regex 測試與無效 pattern 警告已接繁中。
- Skills / Rules：Skill Creator 基本欄位、scope/Front-matter/內容小標、Rules Library 搜尋/分類/新增/刪除/預設規則提示已接繁中。
- Provider：Zed-hosted model subscription CTA、OpenAI API key onboarding、Edit Prediction API key card 與 OpenAI-compatible 提示已接繁中。
- Edit Prediction：狀態按鈕、Provider 切換、Copilot Next Edit Suggestions、資料收集、排除檔案、用量/account/invoice 提示已接繁中。
- Agent selector/profile：Zed Agent、External Agents、新增代理、Profile selector、tools unsupported 提示已接繁中。

## 後續 milestone 追蹤

- Provider 深層設定：Anthropic、Google AI、Mistral、OpenRouter、DeepSeek、xAI、Vercel AI Gateway、Bedrock、Ollama、LM Studio、OpenCode 仍有 API key onboarding 與 credentials UI 可逐一抽出共用詞條。
- Edit Prediction：provider-specific 外部錯誤內容、staff-only experiment 名稱、動態 usage 數值與模型名稱保留原文或原格式。
- Agent registry：registry 來源的 agent description 保留原文；周邊 ID、repository、platform support 狀態可再細翻。
- MCP JSONC template：JSON key/value 必須保留英文；template comment 可另開 milestone 做 schema/comment 策略。
- Provider error bodies：外部 API、server、CLI 回傳錯誤保留原文，只翻 Zed 自己包的固定標題與導覽句。
