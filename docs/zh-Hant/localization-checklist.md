# 繁體中文 UI 維護 Checklist

本 checklist 用於 upstream merge、rebase，或新增 UI surface 後，避免已繁中化的 Zed fork 大量回流英文。

## 日常 Guardrail

從 repo root 執行：

```bash
script/check-zh-hant-ui-strings
```

預設會掃描已完成繁中化與 M10 盤點過的 UI surface：

- `crates/collab_ui`
- `crates/onboarding`
- `crates/language_tools`
- `crates/dev_container`
- `crates/toolchain_selector`
- `crates/repl`
- `crates/zed/src/zed/quick_action_bar.rs`
- `crates/zed/src/zed/quick_action_bar/repl_menu.rs`
- `crates/zed/src/zed/open_url_modal.rs`
- `crates/zed/src/zed/migrate.rs`
- `crates/zed/src/zed/telemetry_log.rs`

如果要盤點整個 repo 的剩餘 backlog，可執行：

```bash
script/check-zh-hant-ui-strings --all
```

`--all` 是 inventory 模式，可能會列出尚未納入保護範圍的舊 UI 英文；不要把它和預設 guardrail 混為一談。

## Upstream Merge 後流程

1. 完成 upstream merge 或 rebase。
2. 跑 `cargo fmt`。
3. 跑 `script/check-zh-hant-ui-strings`。
4. 對每個 `visible-english`：
   - 真實可見 UI：改為 `l10n::text("English Source")`。
   - 新增繁中詞條到 `crates/ui/src/l10n/zh_hant.rs`。
   - 品牌、協定、檔名、動態外部內容：只在有明確理由時更新 `docs/zh-Hant/allowlist.md`。
5. 對每個 `missing-l10n-entry`：
   - 補 `crates/ui/src/l10n/zh_hant.rs` 詞條。
   - 若是品牌或協定 token，改寫呼叫端，讓固定句子翻譯、token 保留英文。
6. 跑測試與建置：

```bash
python3 script/test-check-zh-hant-ui-strings
cargo test -p ui l10n::zh_hant --lib
cargo build -p zed
```

如果本機 toolchain 因 Metal SDK 選擇失敗，再於 `cargo` 前加上目前可用的 `TOOLCHAINS=...`。

7. 做一次 CLI smoke：

```bash
target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-guardrail --system-specs
```

8. commit 前跑：

```bash
git diff --check
git status --short
```

## Allowlist 規則

Allowlist 只用於不應翻譯或不是一般 UI 的內容。新增 row 時，必須能歸入下列分類之一：

- `brand/protocol`
- `code identifier`
- `dynamic external content`
- `telemetry/log/test`
- `visual fixture/sample`
- `file/path/schema/setting key`

新增 allowlist row 前先問自己：

- 使用者是否會在正常 UI flow 看見這個英文？
- 這是否只是檔名、設定 key、路徑、協定名稱或品牌？
- 固定前後綴是否仍可翻譯？
- 這個 row 是否太寬，會把未來真正漏翻的 UI 一起藏掉？

如果答案不明確，不要 allowlist，先翻譯或縮小掃描範圍。

## 詞條品質檢查

- 優先採用 VS Code 繁中慣用詞。
- `Command Palette` 用「命令選擇區」。
- `Settings` 用「設定」。
- `Workspace` 用「工作區」。
- `Project` 用「專案」。
- `Extensions` 用「延伸模組」。
- `Diff` 用「差異」。
- `Stage` 用「暫存」。
- `Stash` 用「貯藏」。
- `Thread` 依 Agent 語境用「對話串」。
- `Agent` 用「代理」。
- `MCP`、`ACP`、`LSP`、`REPL` 保留英文縮寫。

中文文案要自然短句；避免逐字保留 `Pattern`、`Scope`、`Repository`、`Modal`、`Proxy` 等英文，除非是設定 key 或協定 token。
