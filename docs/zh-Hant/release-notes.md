# Zed 個人 Fork 繁體中文 UI Release Notes

本分支：`zh-hant-ui-slice`

本文件記錄目前個人 fork 的繁體中文 UI 覆蓋範圍、最終驗證方式與刻意保留英文的邊界，供後續 upstream merge、PR 或本機安裝前檢查。

## 完成範圍

- 核心入口：menu bar、Welcome、Onboarding、Command Palette placeholder/footer、File Finder、Theme selector。
- 工作區與專案：Project Panel、Search、Recent Projects、Welcome project actions、remote/dev-container/WSL 相關固定 UI。
- 開發者工作流：Settings shell 與主要分類、Keymap/command 高頻詞條、Debugger panel、Debugger launch modal、generic picker empty state。
- Agent/AI：Agent Panel、Provider、MCP、Copilot onboarding 與 tool permissions 的主要固定 UI。
- 全域 polish：Collab、Language tools、Dev Container、Quick Action Bar、REPL、Telemetry log、Migration/open-url 等 M10 surface。
- Guardrails：`script/check-zh-hant-ui-strings`、`docs/zh-Hant/allowlist.md`、`docs/zh-Hant/localization-checklist.md`。

## GUI Smoke 結果

已用本機 `target/debug/zed` 暫時打包成一次性路徑 `/private/tmp/zed-zh-hant-smoke-app.K0Of1k/ZedZhHantSmoke.app` 做 GUI smoke。臨時 bundle 的核心步驟是把 `target/debug/zed` 複製到 `Contents/MacOS/zed`，再用 `/usr/bin/open -n ... --args --user-data-dir /private/tmp/zed-zh-hant-final-app /private/tmp/zed-zh-hant-smoke-project` 啟動。

已完成或部分完成的 GUI smoke：

- menu bar：`檔案`、`編輯`、`選取範圍`、`檢視`、`移至`、`執行`、`視窗`、`說明`
- Onboarding：`歡迎使用 Zed`、`佈景主題`、`基底按鍵對應`、`代理設定`、`匯入設定`、Vim/trust/telemetry 說明
- Welcome：`歡迎使用 Zed`、`新增檔案`、`開啟專案`、`複製儲存庫`、`開啟命令選擇區`
- Project Panel：專案樹與底部 panel 可正常顯示繁中 chrome
- Search：`專案搜尋`、`搜尋所有檔案...`、搜尋選項
- Recent Projects：`搜尋專案...`、`最近專案`、`新增`、`新增視窗`、`開啟`、`動作`
- Settings：`設定`、`搜尋設定...`、分類如 `一般`、`外觀`、`編輯器`、`版本控制`、`協作`、`開發人員`
- Debugger panel/modal：`中斷點`、`尚未設定中斷點`、`新增工作階段`、`偵錯工具文件`、`執行`、`偵錯`、`附加`、`啟動`
- Command Palette：placeholder/footer 已繁中；部分 action id 仍是 known backlog

未完整逐項 GUI smoke 的 M12 候選畫面：

- Agent Panel、Provider/Copilot、Git/Diff/Commit：本輪以 M8/M7/M9 的 l10n 測試、guardrail 與既有 surface 掃描覆蓋，仍建議正式安裝前由使用者逐項人工確認。
- Theme selector：theme 名稱保留英文；selector action id 仍可能以英文出現在 Command Palette。

## 已知保留英文

- 品牌、產品、協定與檔案名：`Zed`、`Zed AI`、`Zed Pro`、`Git`、`GitHub`、`GitHub Copilot`、`Copilot Chat`、`OpenAI`、`OpenAI-compatible`、`MCP`、`ACP`、`LLM`、`LSP`、`REPL`、`WSL`、`SSH`、`Dev Container`、`AGENTS.md`、`debug.json`、`tasks.json`、`settings.json`
- 第三方 keymap/editor 名稱：`VS Code`、`JetBrains`、`Sublime Text`、`Atom`、`Cursor`
- theme 名稱：`One`、`Ayu`、`Gruvbox`
- Command Palette 中仍有部分 action id 或外部 registry 來源字串顯示英文，例如 `agent: add context server`。這類字串會在後續 action-name 詞條或 upstream i18n 架構中繼續收斂。
- Settings 中部分 enum value 仍可能是英文，例如 `Platform Default`。這類值多來自設定 schema 或 enum literal，需另行評估是否翻譯會影響搜尋與設定語意。

## 分支狀態

本文件不預先宣稱 branch clean 或 pushed。M12 執行者必須在 commit/push 後以 `git status --short --branch`、commit hash 與 push 結果在最終回報中確認。

## 驗證命令

M12 最終驗證使用：

```bash
git status --short --branch
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo fmt --check
git diff --check
python3 script/test-check-zh-hant-ui-strings
script/check-zh-hant-ui-strings
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-final --system-specs
```

另做 GUI smoke：啟動臨時 `.app` bundle 並逐項檢查 Welcome、Onboarding、Settings、Search、Recent Projects、Debugger panel/modal。

## 打包注意

`script/bundle-mac -d` 在本機曾於 `cargo bundle` 階段遇到 term color unwrap panic。M12 GUI smoke 因此改用暫時 `.app` bundle 驗證最新 `target/debug/zed`，避免誤開 `/Applications/Zed.app` stable 版。
