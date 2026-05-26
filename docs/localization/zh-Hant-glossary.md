# Zed Traditional Chinese Glossary

This glossary defines the first-pass Traditional Chinese terminology for the
personal Zed fork. It favors familiar editor wording used by Traditional Chinese
developer tools, with VS Code Traditional Chinese phrasing as the main reference.

## Principles

- Use Traditional Chinese for Taiwan (`zh-Hant` / `zh-TW`) in user-facing UI.
- Prefer concise labels that fit menus, buttons, tabs, and tooltips.
- Keep product names, programming language names, commands, paths, and code
  identifiers in English.
- Translate editor concepts consistently across menus, the command palette,
  settings, and panels.
- Use full-width Chinese punctuation in prose, but keep ASCII punctuation in
  code-like snippets, settings keys, and file names.

## Core Terms

| English | Traditional Chinese | Notes |
| --- | --- | --- |
| Command Palette | 命令選擇區 | Common VS Code Traditional Chinese wording. |
| Settings | 設定 | Use for UI settings and settings pages. |
| Settings Editor | 設定編輯器 | Use when distinguishing from `settings.json`. |
| Settings File | 設定檔 | Use for JSON file actions. |
| Project | 專案 | Use for Zed project model. |
| Project Panel | 專案面板 | File tree side panel. |
| Workspace | 工作區 | Use when referring to the broader editor workspace. |
| File | 檔案 | Menu and generic file noun. |
| Folder | 資料夾 | Directories in UI. |
| Terminal | 終端機 | Built-in terminal. |
| Extensions | 延伸模組 | Keep consistent with VS Code zh-Hant. |
| Keybinding | 鍵盤快速鍵 | Prefer over hotkey. |
| Keyboard Shortcut | 鍵盤快速鍵 | Same as keybinding in visible UI. |
| Theme | 佈景主題 | UI theme. |
| Appearance | 外觀 | Settings category. |
| Editor | 編輯器 | Code editor surface. |
| Panel | 面板 | Docked side/bottom surfaces. |
| Dock | 停駐區 | Use when the UI means the dock container itself. |
| Tab | 索引標籤 | Editor tabs. |
| Pane | 窗格 | Split editor panes. |
| Window | 視窗 | App windows. |
| Search | 搜尋 | Search UI. |
| Replace | 取代 | Search/replace UI. |
| Open | 開啟 | Commands and menu items. |
| Close | 關閉 | Commands and menu items. |
| Save | 儲存 | Commands and menu items. |
| New | 新增 | Commands and menu items. |
| Delete | 刪除 | Files and actions. |
| Remove | 移除 | Removing from a list or configuration. |
| Rename | 重新命名 | Files and symbols. |
| Copy | 複製 | Clipboard. |
| Paste | 貼上 | Clipboard. |
| Cut | 剪下 | Clipboard. |
| Undo | 復原 | Editing action. |
| Redo | 重做 | Editing action. |
| Retry | 重試 | Error recovery action. |
| Dismiss | 關閉 | Lightweight dismiss action unless meaning requires `略過`. |
| Configure | 設定 | Verb in buttons. |
| Install | 安裝 | Extensions/tools. |
| Uninstall | 解除安裝 | Extensions/tools. |
| Update | 更新 | Software or extension update. |
| Reload | 重新載入 | Reloading window/project/settings. |
| Recent | 最近使用 | Recent files/projects. |
| Favorites | 我的最愛 | Favorited items. |
| Outline | 大綱 | Symbols/document outline. |
| Symbols | 符號 | Code symbols. |
| Diagnostics | 診斷 | LSP diagnostics. |
| Debugger | 偵錯工具 | Debugger UI. |
| Breakpoint | 中斷點 | Debugging. |
| Task | 工作 | Zed tasks. |
| Agent | 代理 | AI agent surfaces. |
| Assistant | 助理 | Assistant UI surfaces. |
| Model | 模型 | AI model selector. |
| Thread | 對話串 | AI conversation thread. |

## Style Decisions

- Use `命令選擇區` for `Command Palette` in visible UI and documentation.
- Use `延伸模組` for `Extensions`, not `擴充功能`, to stay closer to VS Code
  Traditional Chinese wording.
- Use `鍵盤快速鍵` for both `Keybindings` and `Keyboard Shortcuts` unless a
  lower-level config key must remain literal.
- Use `專案面板` for `Project Panel` and `工作區` for `Workspace` to preserve the
  distinction between Zed's file tree and the whole editing context.
- Keep `Zed` untranslated.
