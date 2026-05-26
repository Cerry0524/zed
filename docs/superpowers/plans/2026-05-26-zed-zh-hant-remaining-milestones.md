# Zed Traditional Chinese UI Remaining Milestones Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the remaining Traditional Chinese UI localization work for the user's Zed fork, with each large milestone controlled by a separate user-created goal.

**Architecture:** Keep the existing `ui::l10n` call pattern stable while expanding coverage in scoped, reviewable milestones. Use subagents for independent inventory, terminology review, and leak checks; keep production edits integrated by the main worker to avoid conflicting Rust changes.

**Tech Stack:** Zed Rust workspace, GPUI UI crates, `ui::l10n`, Cargo tests/check/build, git commits pushed to `origin/zh-hant-ui-slice`.

---

## Shared Execution Rules

- [ ] Start every milestone by running `git status --short --branch` in `/Users/cerry/.codex-accounts/personal-home/Documents/Codex/2026-05-26/fork-zed-mac-vs-code/zed`.
- [ ] Confirm the branch is `zh-hant-ui-slice` and the remote is `origin=https://github.com/Cerry0524/zed.git`.
- [ ] Do not modify unrelated user changes. If unrelated changes exist, record them and work around them.
- [ ] Prefer subagents for read-only audit tasks. Main worker owns final edits, tests, commits, and push.
- [ ] Each milestone must end with a dedicated commit and `git push origin zh-hant-ui-slice`.
- [ ] Do not mark a goal complete until all milestone acceptance criteria pass.
- [ ] Use Traditional Chinese in user-facing summaries and docs.

## Shared Verification Commands

Use these commands as applicable to each milestone:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo fmt --check
```

Expected: exits 0.

```bash
git diff --check
```

Expected: exits 0.

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
```

Expected: all `l10n` tests pass.

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
```

Expected: exits 0.

```bash
target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-final --system-specs
```

Expected: exits 0 and prints Zed system specs. A log-file permission fallback to stdout is acceptable if the command exits 0.

---

## Milestone 4: Full UI String Audit and Coverage Map

### Goal Command To Use

```text
按照 Milestone 4 執行：建立 Zed 剩餘 UI 繁體中文化完整盤點、術語表與 milestone 覆蓋地圖。只做盤點與文件，不接線 production UI。完成後 commit 並 push。
```

### Objective

Create a complete remaining-string inventory so later milestones are driven by evidence, not guesswork.

### Files

- Create: `docs/zh-Hant/ui-localization-audit.md`
- Create: `docs/zh-Hant/terminology.md`
- Create: `docs/zh-Hant/milestone-map.md`
- Modify: no production Rust files in this milestone

### Subagents

- [ ] Subagent A: Audit `crates/*_ui`, `crates/workspace`, `crates/project_panel`, `crates/file_finder`, `crates/search`, `crates/recent_projects` for visible UI strings.
- [ ] Subagent B: Audit Agent, Assistant, Settings, Command Palette, Picker, Modal, Toast, Tooltip, and ContextMenu surfaces.
- [ ] Subagent C: Compare likely terminology against VS Code Traditional Chinese conventions and produce a terminology recommendation table.

### Main Worker Tasks

- [ ] Run `rg -n 'Label::new\("|Button::new\(|Tooltip::text\("|ContextMenuEntry::new\("|\.title\("|\.description\("' crates` and save categorized findings into `docs/zh-Hant/ui-localization-audit.md`.
- [ ] Mark each finding as `translate`, `keep-English`, `dynamic`, `brand/protocol`, `log/test`, or `needs-design`.
- [ ] Write `docs/zh-Hant/terminology.md` with fixed terms such as Command Palette, Settings, Workspace, Project, Extensions, Terminal, Agent, Thread, Provider, Model, Diff, Commit, Branch, and Stash.
- [ ] Write `docs/zh-Hant/milestone-map.md` assigning remaining UI areas to Milestones 6-12.
- [ ] Run `git diff --check`.
- [ ] Commit with:

```bash
git add docs/zh-Hant/ui-localization-audit.md docs/zh-Hant/terminology.md docs/zh-Hant/milestone-map.md
git commit -m "docs: audit remaining Traditional Chinese UI coverage"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Audit doc lists remaining visible English strings by crate/surface.
- [ ] Terminology doc has stable Traditional Chinese equivalents and notes for terms intentionally kept in English.
- [ ] Milestone map routes every high-frequency surface to a later milestone.
- [ ] No production code changed.
- [ ] Commit pushed.

---

## Milestone 5: l10n Architecture and Guarded Vocabulary

### Goal Command To Use

```text
按照 Milestone 5 執行：整理 Zed 個人 fork 的繁中 l10n 架構與測試，保留既有 API，相容第三階段成果。完成後驗證、commit 並 push。
```

### Objective

Make the localization layer easier to maintain before adding many more translations.

### Files

- Modify: `crates/ui/src/l10n.rs`
- Optional create: `crates/ui/src/l10n/zh_hant.rs`
- Optional create: `crates/ui/src/l10n/mod.rs`

### Subagents

- [ ] Subagent A: Review current `crates/ui/src/l10n.rs` and recommend whether to split or keep single-file based on local module style.
- [ ] Subagent B: Review terminology consistency from Milestone 4 against current `zh_hant_text` entries.

### Main Worker Tasks

- [ ] Add or preserve tests for locale recognition, fallback behavior, dynamic `text_or_original`, core terminology, and third-slice entries.
- [ ] If splitting files, keep external calls unchanged: `ui::l10n::text(...)` and `ui::l10n::text_or_original(...)`.
- [ ] Keep translations static and deterministic. Do not introduce runtime JSON loading unless explicitly requested later.
- [ ] Run:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo fmt --check
git diff --check
```

- [ ] Commit with:

```bash
git add crates/ui/src/l10n.rs crates/ui/src/l10n
git commit -m "ui: organize Traditional Chinese localization table"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Existing third-stage behavior still works.
- [ ] `l10n` tests pass.
- [ ] No unrelated UI behavior changes.
- [ ] Commit pushed.

---

## Milestone 6: Core Workspace and Project UI Localization

### Goal Command To Use

```text
按照 Milestone 6 執行：全面繁中化核心工作區、專案面板、檔案操作、搜尋與最近專案等主畫面 UI。完成後測試、build、smoke、commit 並 push。
```

### Objective

Make the everyday workspace experience usable mostly in Traditional Chinese.

### Files To Inspect First

- `crates/workspace/src/pane.rs`
- `crates/workspace/src/welcome.rs`
- `crates/workspace/src/security_modal.rs`
- `crates/project_panel/src/`
- `crates/file_finder/src/`
- `crates/search/src/`
- `crates/recent_projects/src/`
- `crates/sidebar/src/sidebar.rs`
- `crates/ui/src/l10n.rs`

### Subagents

- [ ] Subagent A: Audit Project Panel and file operation menus.
- [ ] Subagent B: Audit Search, File Finder, Picker, Recent Projects, and Workspace modal strings.
- [ ] Subagent C: Review translated output for VS Code-style terminology and UI length risks.

### Main Worker Tasks

- [ ] Add failing l10n assertions for at least 20 high-frequency workspace/project/search strings.
- [ ] Run the focused test and confirm it fails on missing translations.
- [ ] Add translations to `ui::l10n`.
- [ ] Wire visible strings to `l10n::text(...)` or `l10n::text_or_original(...)` in scoped files.
- [ ] Run:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo check -p workspace -p project_panel -p file_finder -p search -p recent_projects -p sidebar -p ui
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
```

- [ ] Smoke with `target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-m6 --system-specs`.
- [ ] Commit with:

```bash
git commit -m "ui: localize core workspace and project surfaces"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Project Panel, File Finder, Search, Recent Projects, and common workspace chrome have improved Traditional Chinese coverage.
- [ ] High-frequency visible English leftovers are documented if intentionally kept.
- [ ] Tests/check/build/smoke pass.
- [ ] Commit pushed.

---

## Milestone 7: Developer Workflow Localization

### Goal Command To Use

```text
按照 Milestone 7 執行：全面繁中化開發者工作流 UI，包含命令、鍵盤快速鍵、工作、終端機、診斷、大綱、符號、偵錯、Git、延伸模組。完成後驗證、commit 並 push。
```

### Objective

Localize coding workflow surfaces while keeping technical terms accurate.

### Files To Inspect First

- `crates/command_palette/src/`
- `crates/keymap_editor/src/`
- `crates/tasks_ui/src/`
- `crates/terminal_view/src/`
- `crates/diagnostics/src/`
- `crates/outline_panel/src/`
- `crates/project_symbols/src/`
- `crates/debugger_ui/src/`
- `crates/git_ui/src/`
- `crates/extensions_ui/src/`
- `crates/ui/src/l10n.rs`

### Subagents

- [ ] Subagent A: Audit command palette, keymap, and task UI strings.
- [ ] Subagent B: Audit terminal, diagnostics, outline, symbols, and debugger UI strings.
- [ ] Subagent C: Audit Git and Extensions UI strings.

### Main Worker Tasks

- [ ] Add l10n tests for developer workflow terminology.
- [ ] Keep brand/provider/tool names in English when they are product names, protocol names, or literal commands.
- [ ] Wire labels, buttons, tooltips, modal titles, picker placeholders, and context menus.
- [ ] Run:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo check -p command_palette -p keymap_editor -p tasks_ui -p terminal_view -p diagnostics -p outline_panel -p project_symbols -p debugger_ui -p git_ui -p extensions_ui
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
```

- [ ] Commit with:

```bash
git commit -m "ui: localize developer workflow surfaces"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Developer workflow UI uses consistent Traditional Chinese terminology.
- [ ] Command/action search still works with dynamic command names.
- [ ] Tests/check/build pass.
- [ ] Commit pushed.

---

## Milestone 8: Agent, AI, Provider, and Copilot Deep Localization

### Goal Command To Use

```text
按照 Milestone 8 執行：深度繁中化 Agent、AI、Provider、MCP、Skills、Copilot 與工具權限流程 UI。完成後驗證、commit 並 push。
```

### Objective

Finish the high-complexity AI workflows with clear, safe Traditional Chinese copy.

### Files To Inspect First

- `crates/agent_ui/src/`
- `crates/agent/src/`
- `crates/agent_settings/src/`
- `crates/ai_onboarding/src/`
- `crates/copilot_ui/src/`
- `crates/language_models/src/`
- `crates/settings_ui/src/`
- `crates/ui/src/l10n.rs`

### Subagents

- [ ] Subagent A: Audit `agent_ui` thread, tool-call, permission, and error surfaces.
- [ ] Subagent B: Audit provider, model selector, MCP, Skills, and API keys surfaces.
- [ ] Subagent C: Audit Copilot onboarding, subscription, authentication, and error flows.
- [ ] Subagent D: Review safety/permission copy for clarity and risk of misleading translation.

### Main Worker Tasks

- [ ] Add l10n tests for Agent, Provider, MCP, Skills, Copilot, permission, and error terminology.
- [ ] Translate permission choices conservatively: allow, deny, approve, reject, ask, trust, restricted mode.
- [ ] Preserve product names: Zed, Copilot, GitHub, MCP, ACP, LLM when appropriate.
- [ ] Wire remaining visible strings to `l10n`.
- [ ] Run:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo check -p agent_ui -p agent -p agent_settings -p ai_onboarding -p copilot_ui -p language_models -p settings_ui
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
```

- [ ] Commit with:

```bash
git commit -m "ui: localize agent and AI workflows"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Agent panel primary workflows are understandable in Traditional Chinese.
- [ ] Permission and safety-related copy remains precise.
- [ ] Tests/check/build pass.
- [ ] Commit pushed.

---

## Milestone 9: Settings UI and Description Localization

### Goal Command To Use

```text
按照 Milestone 9 執行：全面繁中化 Settings UI、設定描述、ActionLink、SubPageLink、JSON 設定說明與修改來源標示。完成後驗證、commit 並 push。
```

### Objective

Make Settings feel native in Traditional Chinese, not merely partially translated.

### Files To Inspect First

- `crates/settings_ui/src/settings_ui.rs`
- `crates/settings_ui/src/components/`
- `crates/settings_content/src/`
- `crates/settings/src/`
- `crates/ui/src/l10n.rs`

### Subagents

- [ ] Subagent A: Audit settings titles and section labels.
- [ ] Subagent B: Audit setting descriptions and action links.
- [ ] Subagent C: Review long Traditional Chinese strings for readability and UI length.

### Main Worker Tasks

- [ ] Add l10n tests for representative settings descriptions.
- [ ] Translate setting descriptions using natural Traditional Chinese and consistent terms from `docs/zh-Hant/terminology.md`.
- [ ] Keep literal setting keys, JSON keys, and code values unchanged.
- [ ] Run:

```bash
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo check -p settings_ui -p settings_content -p settings -p ui
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
```

- [ ] Commit with:

```bash
git commit -m "ui: localize settings descriptions"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Settings titles, descriptions, action links, and placeholders are substantially localized.
- [ ] JSON/code identifiers are not translated.
- [ ] Tests/check/build pass.
- [ ] Commit pushed.

---

## Milestone 10: Global Leak Scan and UI Polish

### Goal Command To Use

```text
按照 Milestone 10 執行：進行全域英文 UI 漏翻掃描、allowlist 分類、繁中文字長度與語氣修整。完成後驗證、commit 並 push。
```

### Objective

Catch remaining visible English and improve quality before adding guardrails.

### Files

- Modify: touched UI crates from leak findings
- Modify: `crates/ui/src/l10n.rs`
- Modify: `docs/zh-Hant/ui-localization-audit.md`
- Optional create: `docs/zh-Hant/allowlist.md`

### Subagents

- [ ] Subagent A: Run grep-based visible-string leak scan and classify findings.
- [ ] Subagent B: Review UI copy quality and terminology consistency.
- [ ] Subagent C: Identify strings that should intentionally stay English and justify them.

### Main Worker Tasks

- [ ] Search for suspicious visible English strings:

```bash
rg -n 'Label::new\("[A-Za-z]|Button::new\([^\\n]*"[A-Za-z]|Tooltip::text\("[A-Za-z]|ContextMenuEntry::new\("[A-Za-z]|\.title\("[A-Za-z]|\.description\("[A-Za-z]' crates
```

- [ ] Exclude tests, logs, schema/protocol names, code identifiers, product names, and backend messages.
- [ ] Add missing translations and connect missed call sites.
- [ ] Document intentional English leftovers.
- [ ] Run focused crate checks for touched crates plus `cargo build -p zed`.
- [ ] Commit with:

```bash
git commit -m "ui: polish Traditional Chinese localization coverage"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Remaining visible English is either fixed or documented.
- [ ] High-frequency screens have been manually reviewed through code and smoke checks.
- [ ] Tests/check/build pass.
- [ ] Commit pushed.

---

## Milestone 11: Localization Guardrails

### Goal Command To Use

```text
按照 Milestone 11 執行：建立繁中 UI 漏翻檢查 guardrails、allowlist 與維護 checklist，避免後續 upstream merge 後大量英文回流。完成後驗證、commit 並 push。
```

### Objective

Add lightweight maintenance tooling so future English UI additions are visible.

### Files

- Optional create: `script/check-zh-hant-ui-strings`
- Create: `docs/zh-Hant/localization-checklist.md`
- Create or modify: `docs/zh-Hant/allowlist.md`
- Optional modify: `package.json`, `Makefile`, or existing repo script docs only if a local convention exists

### Subagents

- [ ] Subagent A: Inspect repo script conventions and recommend a minimal check location.
- [ ] Subagent B: Propose allowlist categories and examples.

### Main Worker Tasks

- [ ] Create a lightweight script or documented command that finds suspicious visible English strings.
- [ ] Keep the check advisory unless the user explicitly wants CI enforcement.
- [ ] Document how to update translations after upstream merge.
- [ ] Run the script/check and confirm it reports expected allowlisted findings only.
- [ ] Run:

```bash
git diff --check
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
```

- [ ] Commit with:

```bash
git commit -m "test: add Traditional Chinese localization guardrails"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Maintainers can run one documented command to find likely untranslated UI strings.
- [ ] Allowlist explains why selected English strings remain.
- [ ] Docs explain update workflow.
- [ ] Commit pushed.

---

## Milestone 12: Final Verification, GUI Smoke, and Release Notes

### Goal Command To Use

```text
按照 Milestone 12 執行：完成 Zed 繁體中文 UI 分支最終驗證、GUI smoke、release notes 與分支整理。完成後 commit、push，並標記 goal complete。
```

### Objective

Finish the branch in a clean, auditable state for ongoing personal-fork use.

### Files

- Create: `docs/zh-Hant/release-notes.md`
- Modify: `docs/zh-Hant/ui-localization-audit.md`
- Modify: `docs/zh-Hant/localization-checklist.md`

### Subagents

- [ ] Subagent A: Review final docs for consistency and missing milestone outcomes.
- [ ] Subagent B: Review final diff summary and identify any suspicious broad/unrelated changes.

### Main Worker Tasks

- [ ] Run:

```bash
git status --short --branch
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo fmt --check
git diff --check
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo test -p ui l10n --lib
env TOOLCHAINS=com.apple.dt.toolchain.Metal.32023.883 /Users/cerry/.codex-accounts/personal-home/.cargo/bin/cargo build -p zed
target/debug/zed --user-data-dir /private/tmp/zed-zh-hant-final --system-specs
```

- [ ] Launch or ask the user to inspect GUI screens if GUI access requires approval. Required manual smoke surfaces:
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
- [ ] Write `docs/zh-Hant/release-notes.md` summarizing completed milestones, verification commands, known intentional English leftovers, and next maintenance steps.
- [ ] Commit with:

```bash
git add docs/zh-Hant/release-notes.md docs/zh-Hant/ui-localization-audit.md docs/zh-Hant/localization-checklist.md
git commit -m "docs: summarize Traditional Chinese UI localization"
git push origin zh-hant-ui-slice
```

### Acceptance Criteria

- [ ] Final build and smoke pass.
- [ ] GUI smoke has been performed or explicitly delegated to the user with exact screens.
- [ ] Branch is clean after push.
- [ ] Release notes exist.
- [ ] Goal can be marked complete.

---

## Recommended User Goal Order

1. `按照 Milestone 4 執行：建立 Zed 剩餘 UI 繁體中文化完整盤點、術語表與 milestone 覆蓋地圖。只做盤點與文件，不接線 production UI。完成後 commit 並 push。`
2. `按照 Milestone 5 執行：整理 Zed 個人 fork 的繁中 l10n 架構與測試，保留既有 API，相容第三階段成果。完成後驗證、commit 並 push。`
3. `按照 Milestone 6 執行：全面繁中化核心工作區、專案面板、檔案操作、搜尋與最近專案等主畫面 UI。完成後測試、build、smoke、commit 並 push。`
4. `按照 Milestone 7 執行：全面繁中化開發者工作流 UI，包含命令、鍵盤快速鍵、工作、終端機、診斷、大綱、符號、偵錯、Git、延伸模組。完成後驗證、commit 並 push。`
5. `按照 Milestone 8 執行：深度繁中化 Agent、AI、Provider、MCP、Skills、Copilot 與工具權限流程 UI。完成後驗證、commit 並 push。`
6. `按照 Milestone 9 執行：全面繁中化 Settings UI、設定描述、ActionLink、SubPageLink、JSON 設定說明與修改來源標示。完成後驗證、commit 並 push。`
7. `按照 Milestone 10 執行：進行全域英文 UI 漏翻掃描、allowlist 分類、繁中文字長度與語氣修整。完成後驗證、commit 並 push。`
8. `按照 Milestone 11 執行：建立繁中 UI 漏翻檢查 guardrails、allowlist 與維護 checklist，避免後續 upstream merge 後大量英文回流。完成後驗證、commit 並 push。`
9. `按照 Milestone 12 執行：完成 Zed 繁體中文 UI 分支最終驗證、GUI smoke、release notes 與分支整理。完成後 commit、push，並標記 goal complete。`
