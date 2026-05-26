use std::borrow::Cow;

pub const PERSONAL_FORK_LOCALE: &str = "zh-Hant";

pub fn text(english: &'static str) -> &'static str {
    text_for_locale(PERSONAL_FORK_LOCALE, english)
}

pub fn text_or_original(english: &str) -> Cow<'_, str> {
    if is_traditional_chinese_locale(PERSONAL_FORK_LOCALE) {
        zh_hant_text(english).map_or(Cow::Borrowed(english), Cow::Borrowed)
    } else {
        Cow::Borrowed(english)
    }
}

pub fn text_for_locale(locale: &str, english: &'static str) -> &'static str {
    if is_traditional_chinese_locale(locale) {
        zh_hant_text(english).unwrap_or(english)
    } else {
        english
    }
}

pub fn is_traditional_chinese_locale(locale: &str) -> bool {
    let normalized = locale.replace('_', "-").to_ascii_lowercase();
    normalized == "zh-hant"
        || normalized.starts_with("zh-hant-")
        || normalized == "zh-tw"
        || normalized.starts_with("zh-tw-")
}

fn zh_hant_text(english: &str) -> Option<&'static str> {
    Some(match english {
        "Command Palette" => "命令選擇區",
        "Execute a command..." => "執行命令...",
        "workspace: open" => "工作區：開啟",
        "workspace: save" => "工作區：儲存",
        "workspace: save all" => "工作區：全部儲存",
        "workspace: save as" => "工作區：另存新檔",
        "workspace: close window" => "工作區：關閉視窗",
        "workspace: new window" => "工作區：新增視窗",
        "workspace: reload" => "工作區：重新載入",
        "file finder: toggle" => "檔案尋找器：切換",
        "command palette: toggle" => "命令選擇區：切換",
        "terminal panel: toggle" => "終端機面板：切換",
        "project panel: toggle focus" => "專案面板：切換焦點",
        "outline: toggle" => "大綱：切換",
        "search: focus search" => "搜尋：聚焦搜尋",
        "search: toggle replace" => "搜尋：切換取代",
        "search: select next match" => "搜尋：選取下一個相符項目",
        "search: select previous match" => "搜尋：選取上一個相符項目",
        "editor: go to definition" => "編輯器：移至定義",
        "editor: go to type definition" => "編輯器：移至型別定義",
        "editor: find all references" => "編輯器：尋找所有參考",
        "editor: format" => "編輯器：格式化",
        "editor: rename" => "編輯器：重新命名",
        "editor: backspace" => "編輯器：退格",
        "zed: extensions" => "Zed：延伸模組",
        "zed: install dev extension" => "Zed：安裝開發用延伸模組",
        "Settings" => "設定",
        "Settings Editor" => "設定編輯器",
        "Settings File" => "設定檔",
        "Project" => "專案",
        "Project Panel" => "專案面板",
        "Workspace" => "工作區",
        "Selection" => "選取範圍",
        "Edit" => "編輯",
        "View" => "檢視",
        "Go" => "移至",
        "Help" => "說明",
        "File" => "檔案",
        "Folder" => "資料夾",
        "Terminal" => "終端機",
        "Extensions" => "延伸模組",
        "Keybindings" => "鍵盤快速鍵",
        "Keyboard Shortcuts" => "鍵盤快速鍵",
        "Theme" => "佈景主題",
        "Appearance" => "外觀",
        "Editor" => "編輯器",
        "Panel" => "面板",
        "Dock" => "停駐區",
        "Tab" => "索引標籤",
        "Pane" => "窗格",
        "Window" => "視窗",
        "Search" => "搜尋",
        "Search Project" => "搜尋專案",
        "Search Symbols" => "搜尋符號",
        "Search Inside" => "在此搜尋",
        "Replace" => "取代",
        "Search…" => "搜尋…",
        "Replace with…" => "取代為…",
        "Search all files…" => "搜尋所有檔案…",
        "Replace in project…" => "在專案中取代…",
        "Search project files..." => "搜尋專案檔案...",
        "Search recent projects…" => "搜尋最近專案…",
        "Project Scan in Progress…" => "正在掃描專案…",
        "Filter Options" => "篩選選項",
        "Include Ignored Files" => "包含已忽略檔案",
        "Include: crates/**/*.toml" => "包含：crates/**/*.toml",
        "Exclude: vendor/*, *.lock" => "排除：vendor/*, *.lock",
        "Find in Results" => "在結果中尋找",
        "Project Search" => "專案搜尋",
        "Search All Files" => "搜尋所有檔案",
        "No Results" => "沒有結果",
        "No results found in this project for the provided query" => "此專案中找不到符合查詢的結果",
        "Loading project…" => "正在載入專案…",
        "Searching…" => "正在搜尋…",
        "Match Whole Words" => "符合完整單字",
        "Match Case Sensitivity" => "符合大小寫",
        "Also search files ignored by configuration" => "也搜尋設定中忽略的檔案",
        "Use Regular Expressions" => "使用規則運算式",
        "One Match Per Line" => "每行一個相符項目",
        "Search Backwards" => "向後搜尋",
        "No more matches" => "沒有更多相符項目",
        "Toggle Replace" => "切換取代",
        "Toggle Search Selection" => "切換搜尋選取範圍",
        "Select Previous Match" => "選取上一個相符項目",
        "Select Next Match" => "選取下一個相符項目",
        "Select All Matches" => "選取所有相符項目",
        "Close Search Bar" => "關閉搜尋列",
        "Replace Next Match" => "取代下一個相符項目",
        "Replace All Matches" => "取代所有相符項目",
        "Toggle Filters" => "切換篩選器",
        "Expand All Files" => "展開所有檔案",
        "Collapse All Files" => "摺疊所有檔案",
        "Expand All Search Results" => "展開所有搜尋結果",
        "Collapse All Search Results" => "摺疊所有搜尋結果",
        "Only Search Open Files" => "只搜尋已開啟的檔案",
        "Search Limits Reached\nTry narrowing your search" => "已達搜尋限制\n請嘗試縮小搜尋範圍",
        "Hit enter to search. For more options:" => "按 Enter 搜尋。更多選項：",
        "Include/exclude specific paths" => "包含/排除特定路徑",
        "Find and replace" => "尋找與取代",
        "Match with regex" => "使用規則運算式比對",
        "Match case" => "符合大小寫",
        "Match whole words" => "符合完整單字",
        "Unified" => "統一檢視",
        "Split" => "分割檢視",
        "Split when wider than" => "寬度超過時分割",
        "columns" => "欄",
        "click to change min width" => "按一下以變更最小寬度",
        "Open" => "開啟",
        "Open…" => "開啟…",
        "Open File" => "開啟檔案",
        "Open File..." => "開啟檔案...",
        "Open Folder" => "開啟資料夾",
        "Open Folder..." => "開啟資料夾...",
        "Open Recent..." => "開啟最近使用...",
        "Open Remote..." => "開啟遠端...",
        "Open Settings" => "開啟設定",
        "Open Settings File" => "開啟設定檔",
        "Open Project Settings" => "開啟專案設定",
        "Open Project Settings File" => "開啟專案設定檔",
        "Open Default Settings" => "開啟預設設定",
        "Open Keymap" => "開啟按鍵對應",
        "Open Keymap File" => "開啟按鍵對應檔",
        "Open Default Key Bindings" => "開啟預設鍵盤快速鍵",
        "Open in Terminal" => "在終端機中開啟",
        "Open in Default App" => "在預設 App 中開啟",
        "Open in New Window" => "在新視窗開啟",
        "Open Project in New Window" => "在新視窗開啟專案",
        "Reveal in Finder" => "在 Finder 中顯示",
        "Reveal in File Explorer" => "在 File Explorer 中顯示",
        "Reveal in File Manager" => "在檔案管理員中顯示",
        "Reveal In Project Panel" => "在專案面板中顯示",
        "Close" => "關閉",
        "Close Editor" => "關閉編輯器",
        "Close Project" => "關閉專案",
        "Close Window" => "關閉視窗",
        "Close Terminal" => "關閉終端機",
        "Close Tab" => "關閉索引標籤",
        "Close Others" => "關閉其他",
        "Close Multibuffers" => "關閉 Multibuffer",
        "Close Left" => "關閉左側",
        "Close Right" => "關閉右側",
        "Close Clean" => "關閉未變更項目",
        "Close All" => "全部關閉",
        "Hide Button" => "隱藏按鈕",
        "Save" => "儲存",
        "Save As…" => "另存新檔…",
        "Save All" => "全部儲存",
        "Save all" => "全部儲存",
        "New" => "新增",
        "New File" => "新增檔案",
        "New Folder" => "新增資料夾",
        "New Window" => "新增視窗",
        "Delete" => "刪除",
        "Remove" => "移除",
        "Rename" => "重新命名",
        "Copy" => "複製",
        "Paste" => "貼上",
        "Cut" => "剪下",
        "Undo" => "復原",
        "Redo" => "重做",
        "Retry" => "重試",
        "Dismiss" => "關閉",
        "Restore" => "還原",
        "Overwrite" => "覆蓋",
        "Discard" => "捨棄",
        "Discard all" => "全部捨棄",
        "Don't Save" => "不要儲存",
        "Close Without Saving" => "不儲存並關閉",
        "Read-Only File" => "唯讀檔案",
        "Pin Tab" => "釘選索引標籤",
        "Unpin Tab" => "取消釘選索引標籤",
        "Make File Read-Only" => "將檔案設為唯讀",
        "Make File Editable" => "允許編輯檔案",
        "Yes" => "是",
        "No" => "否",
        "Don't ask me again" => "不要再詢問",
        "Don't Show Again" => "不要再顯示",
        "Configure" => "設定",
        "Install" => "安裝",
        "Install CLI" => "安裝 CLI",
        "Uninstall" => "解除安裝",
        "Update" => "更新",
        "Reload" => "重新載入",
        "Customize" => "自訂",
        "Recent" => "最近使用",
        "Favorites" => "我的最愛",
        "All" => "全部",
        "Installed" => "已安裝",
        "Not Installed" => "未安裝",
        "Outline" => "大綱",
        "Symbols" => "符號",
        "Diagnostics" => "診斷",
        "Debugger" => "偵錯工具",
        "Breakpoint" => "中斷點",
        "Breakpoints" => "中斷點",
        "No Breakpoints Set" => "尚未設定中斷點",
        "Start Debug Session" => "開始偵錯工作階段",
        "New Session" => "新增工作階段",
        "Edit debug.json" => "編輯 debug.json",
        "Edit in debug.json" => "在 debug.json 中編輯",
        "Debugger Docs" => "偵錯工具文件",
        "Debugger Extensions" => "偵錯工具延伸模組",
        "Open Debug Adapter Logs" => "開啟偵錯配接器記錄",
        "Close Panel" => "關閉面板",
        "Launch a new process with a debugger" => "使用偵錯工具啟動新程序",
        "Run predefined task" => "執行預先定義的工作",
        "Start a predefined debug scenario" => "開始預先定義的偵錯情境",
        "Attach the debugger to a running process" => "將偵錯工具附加到執行中的程序",
        "Find a debug task, or debug a command" => "尋找偵錯工作，或偵錯命令",
        "Select Debugger" => "選取偵錯工具",
        "No matches" => "沒有相符項目",
        "Start" => "開始",
        "Debug" => "偵錯",
        "Attach" => "附加",
        "Launch" => "啟動",
        "Pause Program" => "暫停程式",
        "Continue Program" => "繼續程式",
        "Step In" => "逐步進入",
        "Rerun Session" => "重新執行工作階段",
        "Terminate Thread" => "終止執行緒",
        "Terminate All Threads" => "終止所有執行緒",
        "Detach" => "中斷連結",
        "Debugger:" => "偵錯工具：",
        "Stop on Entry" => "進入時停止",
        "Launch Custom" => "啟動自訂設定",
        "Evaluate an expression" => "評估運算式",
        "Evaluate" => "評估",
        "Write to Selected Memory Range" => "寫入選取的記憶體範圍",
        "Go to Memory Address / Expression" => "前往記憶體位址 / 運算式",
        "Restart Stack Frame" => "重新啟動堆疊框架",
        "Task" => "工作",
        "Agent" => "代理",
        "Assistant" => "助理",
        "Model" => "模型",
        "Thread" => "對話串",
        "Image" => "影像",
        "Apply" => "套用",
        "Ok" | "OK" => "確定",
        "Cancel" => "取消",
        "Restart" => "重新啟動",
        "About Zed" => "關於 Zed",
        "Check for Updates" => "檢查更新",
        "Services" => "服務",
        "Hide Zed" => "隱藏 Zed",
        "Hide Others" => "隱藏其他",
        "Show All" => "全部顯示",
        "Quit Zed" => "結束 Zed",
        "Zoom In" => "放大",
        "Zoom Out" => "縮小",
        "Reset Zoom" => "重設縮放",
        "Reset All Zoom" => "重設所有縮放",
        "Toggle Left Dock" => "切換左側停駐區",
        "Toggle Right Dock" => "切換右側停駐區",
        "Toggle Bottom Dock" => "切換底部停駐區",
        "Toggle All Docks" => "切換所有停駐區",
        "Editor Layout" => "編輯器版面配置",
        "Split Up" => "向上分割",
        "Split Down" => "向下分割",
        "Split Left" => "向左分割",
        "Split Right" => "向右分割",
        "Split…" => "分割…",
        "Outline Panel" => "大綱面板",
        "Collab Panel" => "協作面板",
        "Terminal Panel" => "終端機面板",
        "Debugger Panel" => "偵錯工具面板",
        "Toggle GPUI Inspector" => "切換 GPUI 檢查器",
        "Select Theme..." => "選取佈景主題...",
        "Select Icon Theme..." => "選取圖示佈景主題...",
        "Add Folder to Project…" => "新增資料夾至專案…",
        "Add Folders to Project…" => "新增資料夾至專案…",
        "Remove from Project" => "從專案移除",
        "Copy and Trim" => "複製並修剪",
        "Find" => "尋找",
        "Find in Project" => "在專案中尋找",
        "Toggle Line Comment" => "切換行註解",
        "Select All" => "全選",
        "Expand Selection" => "展開選取範圍",
        "Shrink Selection" => "縮小選取範圍",
        "Select Next Sibling" => "選取下一個同層項目",
        "Select Previous Sibling" => "選取上一個同層項目",
        "Add Cursor Above" => "在上方新增游標",
        "Add Cursor Below" => "在下方新增游標",
        "Select Next Occurrence" => "選取下一個相符項目",
        "Select Previous Occurrence" => "選取上一個相符項目",
        "Select All Occurrences" => "選取所有相符項目",
        "Move Line Up" => "將行向上移動",
        "Move Line Down" => "將行向下移動",
        "Duplicate Selection" => "複製選取範圍",
        "Back" => "返回",
        "Forward" => "前進",
        "Command Palette..." => "命令選擇區...",
        "Go to File..." => "移至檔案...",
        "Go to Symbol in Editor..." => "移至編輯器中的符號...",
        "Go to Line/Column..." => "移至行/欄...",
        "Go to Definition" => "移至定義",
        "Go to Declaration" => "移至宣告",
        "Go to Type Definition" => "移至型別定義",
        "Find All References" => "尋找所有參考",
        "Next Problem" => "下一個問題",
        "Previous Problem" => "上一個問題",
        "Spawn Task" => "產生工作",
        "Start Debugger" => "啟動偵錯工具",
        "Edit tasks.json..." => "編輯 tasks.json...",
        "Edit debug.json..." => "編輯 debug.json...",
        "Continue" => "繼續",
        "Step Over" => "逐步跳過",
        "Step Into" => "逐步進入",
        "Step Out" => "逐步跳出",
        "Toggle Breakpoint" => "切換中斷點",
        "Edit Breakpoint" => "編輯中斷點",
        "Clear All Breakpoints" => "清除所有中斷點",
        "Minimize" => "最小化",
        "View Release Notes Locally" => "在本機檢視版本資訊",
        "View Telemetry" => "檢視遙測",
        "View Dependency Licenses" => "檢視相依套件授權",
        "Show Welcome" => "顯示歡迎頁",
        "File Bug Report..." => "回報 Bug...",
        "Request Feature..." => "要求功能...",
        "Email Us..." => "傳送電子郵件給我們...",
        "Documentation" => "文件",
        "Zed Repository" => "Zed 儲存庫",
        "Zed Twitter" => "Zed Twitter",
        "Join the Team" => "加入團隊",
        "Change Keybinding…" => "變更鍵盤快速鍵…",
        "Add Keybinding…" => "新增鍵盤快速鍵…",
        "Run" => "執行",
        "Open Agent Panel" => "開啟代理面板",
        "Title generation failed. Retry" => "標題產生失敗。請重試",
        "Collaborate with Agents" => "與代理協作",
        "Run multiple threads at once, mix and match any ACP-compatible agent, and keep work conflict-free with worktrees." => {
            "一次執行多個對話串，混用任何 ACP 相容代理，並透過工作樹避免工作衝突。"
        }
        "New Thread" => "新增對話串",
        "Start New Thread" => "開始新對話串",
        "New Chat" => "新增對話",
        "Chat" => "對話",
        "Prompt" => "提示",
        "Add a prompt…" => "新增提示…",
        "Subagents Awaiting Permission:" => "等待權限的子代理：",
        "Scroll to Subagent" => "捲動至子代理",
        "Awaiting Confirmation" => "等待確認",
        "Scroll" => "捲動",
        "Clear All" => "全部清除",
        "Current:" => "目前：",
        "All Done" => "全部完成",
        "Plan" => "計畫",
        "Clear Plan" => "清除計畫",
        "Completed Plan" => "已完成計畫",
        "Edits" => "編輯",
        "Review Changes" => "檢閱變更",
        "Queue and Send" => "加入佇列並傳送",
        "Send Immediately" => "立即傳送",
        "Add Context" => "新增內容",
        "Context" => "內容",
        "Cost" => "費用",
        "Rules" => "規則",
        "Files & Directories" => "檔案與目錄",
        "Threads" => "對話串",
        "Branch Diff" => "分支差異",
        "Open Thread as Markdown" => "以 Markdown 開啟對話串",
        "Scroll To Most Recent User Prompt" => "捲動至最近的使用者提示",
        "Scroll To Top" => "捲動至頂端",
        "Sync with source thread" => "與來源對話串同步",
        "Share Thread" => "分享對話串",
        "Start New Agent Thread" => "開始新的代理對話串",
        "Focus Project" => "聚焦專案",
        "Focus Last Project" => "聚焦上一個專案",
        "Open Worktrees" => "開啟工作樹",
        "Move Up" => "上移",
        "Move Down" => "下移",
        "Add Project" => "新增專案",
        "Open Threads Sidebar" => "開啟對話串側邊欄",
        "No threads match your search." => "沒有符合搜尋的對話串。",
        "Show Thread History" => "顯示對話串歷程",
        "Hide Thread History" => "隱藏對話串歷程",
        "Looking for threads from external agents?" => "在找外部代理的對話串嗎？",
        "Import threads from agents like Claude Agent, Codex, and more, whether started in Zed or another client." => {
            "匯入 Claude Agent、Codex 等代理的對話串，無論是在 Zed 或其他用戶端開始。"
        }
        "Import Threads from External Agents" => "從外部代理匯入對話串",
        "Threads found from other channels" => "找到其他頻道的對話串",
        "Import threads from {channels} to continue where you left off." => {
            "從 {channels} 匯入對話串，從上次離開的地方繼續。"
        }
        "Import Threads from Other Channels" => "從其他頻道匯入對話串",
        "Loading Added Context…" => "正在載入新增內容…",
        "Type to Send" => "輸入即可傳送",
        "Send Message" => "傳送訊息",
        "Restore Checkpoint" => "還原檢查點",
        "Stop Subagent" => "停止子代理",
        "Minimize Subagent" => "最小化子代理",
        "Make Subagent Full Screen" => "以全螢幕顯示子代理",
        "Subagent Output" => "子代理輸出",
        "Run Command" => "執行命令",
        "Retry Generation" => "重試產生",
        "Request Refused" => "要求遭拒",
        "Authentication Required" => "需要驗證",
        "Free Usage Exceeded" => "免費用量已超出",
        "Context Too Large" => "內容太大",
        "Copy Error Message" => "複製錯誤訊息",
        "Resumed Session" => "已繼續工作階段",
        "Codex on Windows" => "Windows 上的 Codex",
        "Open in WSL" => "在 WSL 中開啟",
        "Dismiss Warning" => "關閉警告",
        "Authenticate" => "驗證",
        "Provider" => "提供者",
        "Add Provider" => "新增提供者",
        "LLM Providers" => "LLM 提供者",
        "Add at least one provider to use AI-powered features with Zed's native agent." => {
            "至少新增一個提供者，才能在 Zed 原生代理中使用 AI 功能。"
        }
        "Add Server" => "新增伺服器",
        "No MCP servers added yet." => "尚未新增 MCP 伺服器。",
        "No configuration view for {provider_name}" => "沒有 {provider_name} 的設定檢視",
        "Remove Provider" => "移除提供者",
        "Compatible APIs" => "相容 API",
        "Add Custom Server" => "新增自訂伺服器",
        "Install from Extensions" => "從延伸模組安裝",
        "Model Context Protocol (MCP) Servers" => "Model Context Protocol (MCP) 伺服器",
        "All MCP servers connected directly or via a Zed extension." => {
            "所有直接連線或透過 Zed 延伸模組連線的 MCP 伺服器。"
        }
        "Configure MCP Server" => "設定 MCP 伺服器",
        "Configure Server" => "設定伺服器",
        "View Tools" => "檢視工具",
        "Log Out" => "登出",
        "Authenticate to connect this server" => "驗證以連線此伺服器",
        "Enter a client secret to connect this server" => "輸入用戶端密鑰以連線此伺服器",
        "Enter Client Secret" => "輸入用戶端密鑰",
        "Authenticating…" => "驗證中…",
        "1 tool" => "1 個工具",
        "{count} tools" => "{count} 個工具",
        "Add Agent" => "新增代理",
        "Install from Registry" => "從登錄檔安裝",
        "Add Custom Agent" => "新增自訂代理",
        "ACP Docs" => "ACP 文件",
        "External Agents" => "外部代理",
        "All agents connected through the Agent Client Protocol." => {
            "所有透過 Agent Client Protocol 連線的代理。"
        }
        "Restart Agent Connection" => "重新啟動代理連線",
        "Remove Registry Agent" => "移除登錄檔代理",
        "Remove Custom Agent" => "移除自訂代理",
        "The {id} extension provides more than just the MCP server. Proceed to uninstall anyway?" => {
            "{id} 延伸模組不只提供 MCP 伺服器。仍要解除安裝嗎？"
        }
        "Provider Name" => "提供者名稱",
        "API URL" => "API URL",
        "API Key" => "API 金鑰",
        "Model Name" => "模型名稱",
        "Max Completion Tokens" => "最大完成 token 數",
        "Max Output Tokens" => "最大輸出 token 數",
        "Max Tokens" => "最大 token 數",
        "Models" => "模型",
        "Add Model" => "新增模型",
        "Remove Model" => "移除模型",
        "Supports tools" => "支援工具",
        "Supports images" => "支援圖片",
        "Supports parallel_tool_calls" => "支援 parallel_tool_calls",
        "Supports prompt_cache_key" => "支援 prompt_cache_key",
        "Supports /chat/completions" => "支援 /chat/completions",
        "Add LLM Provider" => "新增 LLM 提供者",
        "This provider will use an OpenAI compatible API." => "此提供者會使用 OpenAI 相容 API。",
        "Save Provider" => "儲存提供者",
        "Model Name cannot be empty" => "模型名稱不可空白",
        "Max Completion Tokens must be a number" => "最大完成 token 數必須是數字",
        "Max Output Tokens must be a number" => "最大輸出 token 數必須是數字",
        "Max Tokens must be a number" => "最大 token 數必須是數字",
        "Provider Name cannot be empty" => "提供者名稱不可空白",
        "Provider Name is already taken by another provider" => "提供者名稱已被其他提供者使用",
        "API URL cannot be empty" => "API URL 不可空白",
        "API Key cannot be empty" => "API 金鑰不可空白",
        "Model Names must be unique" => "模型名稱必須唯一",
        "Failed to write API key to keychain" => "無法將 API 金鑰寫入鑰匙圈",
        "Profile name" => "設定檔名稱",
        "Agent Profiles" => "代理設定檔",
        "Custom Profiles" => "自訂設定檔",
        "Add New Profile" => "新增設定檔",
        "New Profile" => "新增設定檔",
        "Fork Profile" => "分支設定檔",
        "Fork {base_profile}" => "分支 {base_profile}",
        "Configure Default Model" => "設定預設模型",
        "Configure Built-in Tools" => "設定內建工具",
        "Configure MCP Tools" => "設定 MCP 工具",
        "Delete Profile" => "刪除設定檔",
        "Go Back" => "返回",
        "{profile_name} — Configure Built-in Tools" => "{profile_name} — 設定內建工具",
        "{profile_name} — Configure Default Model" => "{profile_name} — 設定預設模型",
        "{profile_name} — Configure MCP Tools" => "{profile_name} — 設定 MCP 工具",
        "Search built-in tools…" => "搜尋內建工具…",
        "Search MCP tools…" => "搜尋 MCP 工具…",
        "Tools from {id}" => "{id} 的工具",
        "Add MCP Server" => "新增 MCP 伺服器",
        "Check the server docs for required arguments and environment variables." => {
            "請查看伺服器文件，確認必要引數與環境變數。"
        }
        "Local" => "本機",
        "Remote" => "遠端",
        "Open Repository" => "開啟儲存庫",
        "Enter client secret (leave empty for public clients)" => {
            "輸入用戶端密鑰（公開用戶端可留空）"
        }
        "Enter your OAuth client secret, or leave empty for public clients" => {
            "輸入 OAuth 用戶端密鑰，公開用戶端可留空"
        }
        "Submit" => "送出",
        "Connecting Server…" => "連線伺服器中…",
        "Select a Model" => "選取模型",
        "Select a model…" => "選取模型…",
        "Favorite" => "我的最愛",
        "Latest" => "最新",
        "Cost Multiplier: {cost}" => "成本倍率：{cost}",
        "Cost per Million Tokens: {cost}" => "每百萬 token 成本：{cost}",
        "Cost: {cost}" => "成本：{cost}",
        "Favorite Model" => "加入喜愛模型",
        "Unfavorite Model" => "從喜愛模型移除",
        "Refresh Models" => "重新整理模型",
        "Configure a Provider" => "設定提供者",
        "Sign In Or Configure a Provider" => "登入或設定提供者",
        "This Buffer" => "此緩衝區",
        "All Files" => "所有檔案",
        "Display Modes" => "顯示模式",
        "Eager" => "積極",
        "Subtle" => "低干擾",
        "Training Data Collection" => "訓練資料收集",
        "Data Collection" => "資料收集",
        "Set up different edit prediction providers in complement to Zed's built-in Zeta model." => {
            "設定不同的編輯預測提供者，以搭配 Zed 內建的 Zeta 模型。"
        }
        "Controls whether Zed may collect training data when using Zed's Edit Predictions. Data is only collected for files in projects detected as open source. The default value uses the preference previously set via the status-bar toggle, or false if no preference has been stored." => {
            "控制使用 Zed 編輯預測時，Zed 是否可收集訓練資料。只有偵測為開源專案的檔案才會收集資料。預設值會使用先前透過狀態列切換儲存的偏好；若沒有儲存偏好則為 false。"
        }
        "Controls whether edit predictions are shown immediately or manually." => {
            "控制編輯預測要立即顯示或手動顯示。"
        }
        "Disable in Language Scopes" => "在語言範圍中停用",
        "Controls whether edit predictions are shown in the given language scopes." => {
            "控制是否在指定語言 scope 中顯示編輯預測。"
        }
        "View Docs" => "檢視文件",
        "No provider set" => "尚未設定提供者",
        "Select which provider to use for edit predictions." => "選擇要用於編輯預測的提供者。",
        "Edit Prediction" => "編輯預測",
        "Enable to Use" => "啟用後使用",
        "Sign In to Copilot" => "登入 Copilot",
        "Disable Copilot" => "停用 Copilot",
        "Edit predictions cannot be toggled for this buffer because they are disabled for {language}." => {
            "此緩衝區無法切換編輯預測，因為已對 {language} 停用。"
        }
        "Display predictions inline when there are no language server completions available." => {
            "沒有可用的語言伺服器完成項目時，內嵌顯示預測。"
        }
        "Display predictions inline only when holding a modifier key (alt by default)." => {
            "只有按住修飾鍵時才內嵌顯示預測（預設為 alt）。"
        }
        "Tool Permissions" => "工具權限",
        "Edit File" => "編輯檔案",
        "Write File" => "寫入檔案",
        "Delete Path" => "刪除路徑",
        "Move Path" => "移動路徑",
        "Create Directory" => "建立目錄",
        "Fetch" => "擷取",
        "Web Search" => "網頁搜尋",
        "Skill" => "技能",
        "Commands executed in the terminal" => "在終端機中執行的命令",
        "File editing operations" => "檔案編輯操作",
        "File creation and overwrite operations" => "檔案建立與覆寫操作",
        "File and directory deletion" => "檔案與目錄刪除",
        "File and directory copying" => "檔案與目錄複製",
        "File and directory moves/renames" => "檔案與目錄移動/重新命名",
        "Directory creation" => "目錄建立",
        "HTTP requests to URLs" => "對 URL 發出的 HTTP 要求",
        "Web search queries" => "網頁搜尋查詢",
        "Loading agent skill instructions" => "載入代理技能指示",
        "Patterns are matched against each command in the input. Commands chained with &&, ||, ;, or pipes are split and checked individually." => {
            "Pattern 會對輸入中的每個命令進行比對。以 &&、||、; 或 pipe 串接的命令會被拆開並逐一檢查。"
        }
        "Patterns are matched against the file path being edited." => {
            "Pattern 會對正在編輯的檔案路徑進行比對。"
        }
        "Patterns are matched against the file path being written." => {
            "Pattern 會對正在寫入的檔案路徑進行比對。"
        }
        "Patterns are matched against the path being deleted." => {
            "Pattern 會對正在刪除的路徑進行比對。"
        }
        "Patterns are matched independently against the source path and the destination path. Enter either path below to test." => {
            "Pattern 會分別對來源路徑與目的地路徑進行比對。可在下方輸入任一路徑測試。"
        }
        "Patterns are matched against the directory path being created." => {
            "Pattern 會對正在建立的目錄路徑進行比對。"
        }
        "Patterns are matched against the URL being fetched." => {
            "Pattern 會對正在擷取的 URL 進行比對。"
        }
        "Patterns are matched against the search query." => "會使用模式比對搜尋查詢。",
        "Patterns are matched against the absolute path to the skill's SKILL.md file." => {
            "Pattern 會對技能 SKILL.md 檔案的絕對路徑進行比對。"
        }
        "`rm -rf` commands are always blocked when run on `$HOME`, `~`, `.`, `..`, or `/`" => {
            "在 `$HOME`、`~`、`.`、`..` 或 `/` 執行 `rm -rf` 命令時一律封鎖"
        }
        "Note: custom tool permissions only apply to the Zed native agent and don’t extend to external agents connected through the Agent Client Protocol (ACP)." => {
            "注意：自訂工具權限只會套用到 Zed 原生代理，不會延伸到透過 Agent Client Protocol (ACP) 連線的外部代理。"
        }
        "{tool} Tool" => "{tool} 工具",
        "1 rule" => "1 條規則",
        "{count} rules" => "{count} 條規則",
        "{count} invalid" => "{count} 個無效",
        "Enter a tool input to test your rules…" => "輸入工具內容以測試你的規則…",
        "Always Deny" => "一律拒絕",
        "Always Allow" => "一律允許",
        "Always Confirm" => "一律確認",
        "Confirm" => "確認",
        "If any of these regexes match, the tool action will be denied." => {
            "如果其中任一 regex 相符，工具動作將被拒絕。"
        }
        "If any of these regexes match, the action will be approved—unless an Always Confirm or Always Deny matches." => {
            "如果其中任一 regex 相符，動作將被允許，除非同時符合「一律確認」或「一律拒絕」。"
        }
        "If any of these regexes match, a confirmation will be shown unless an Always Deny regex matches." => {
            "如果其中任一 regex 相符，將顯示確認提示，除非同時符合「一律拒絕」regex。"
        }
        "Controls the default behavior for all tool actions. Per-tool rules and patterns can override this." => {
            "控制所有工具動作的預設行為。各工具規則與 pattern 可覆寫此設定。"
        }
        "Action to take when no patterns match." => "沒有模式相符時要採取的動作。",
        "Test Your Rules" => "測試你的規則",
        "No regex matches, using the default action." => "沒有符合的規則運算式，使用預設動作。",
        "Denied: {reason}" => "已拒絕：{reason}",
        "Pattern preview differs from engine — showing authoritative result." => {
            "Pattern 預覽與引擎不同，正在顯示權威結果。"
        }
        "Reason: {reason}" => "原因：{reason}",
        "Result:" => "結果：",
        "Invalid Patterns" => "無效模式",
        "These patterns failed to compile as regular expressions. The tool will be blocked until they are fixed or removed." => {
            "這些 pattern 無法編譯為 regular expression。修正或移除前，此工具會被封鎖。"
        }
        "Delete Invalid Pattern" => "刪除無效模式",
        "Error: {error}" => "錯誤：{error}",
        "No skills available for this context." => "此情境沒有可用技能。",
        "No patterns configured" => "尚未設定模式",
        "New Skill" => "新增技能",
        "Name" => "名稱",
        "Description" => "描述",
        "Scope" => "範圍",
        "Disable model invocation" => "停用模型叫用",
        "Hide this skill from the model's catalog. It can still be invoked via slash command." => {
            "從模型目錄隱藏此技能。仍可透過 slash command 叫用。"
        }
        "Front-matter" => "Front-matter",
        "Skill Content" => "技能內容",
        "Add skill content…" => "新增技能內容…",
        "Save Skill" => "儲存技能",
        "Saving…" => "正在儲存…",
        "Skill Creator" => "技能建立器",
        "Rules Library" => "規則庫",
        "No rules found matching your search." => "找不到符合搜尋的規則。",
        "Built-in Rules" => "內建規則",
        "Built-in rules are those included out of the box with Zed." => {
            "內建規則是 Zed 預設包含的規則。"
        }
        "Default Rules" => "預設規則",
        "Default Rules are attached by default with every new thread." => {
            "每個新對話串預設都會附加預設規則。"
        }
        "Untitled" => "未命名",
        "Remove from Default Rules" => "從預設規則移除",
        "Add to Default Rules" => "加入預設規則",
        "Always included in every thread." => "每個對話串都會一律包含。",
        "Delete Rule" => "刪除規則",
        "Duplicate Rule" => "複製規則",
        "Restore to Default Content" => "還原為預設內容",
        "New Rule" => "新增規則",
        "Agent Changes Rejected" => "已拒絕代理變更",
        "Always for selected commands" => "一律套用於選取的命令",
        "Only this time" => "僅限這一次",
        "Always for {tool}" => "一律套用於 {tool}",
        "Always for `{command}` commands" => "一律套用於 `{command}` 命令",
        "Select Options…" => "選擇選項…",
        "New From Summary" => "從摘要新增",
        "Zed Agent" => "Zed 代理",
        "Add More Agents" => "新增更多代理",
        "Tools Unsupported" => "不支援工具",
        "This model does not support tools." => "此模型不支援工具。",
        "Change Profile" => "變更設定檔",
        "Cycle Through Profiles" => "循環切換設定檔",
        "MCP Servers" => "MCP 伺服器",
        "Add Custom Server…" => "新增自訂伺服器…",
        "Install New Servers…" => "安裝新伺服器…",
        "Skills" => "技能",
        "Create Skill…" => "建立技能…",
        "Manage Skills…" => "管理技能…",
        "Open Global AGENTS.md" => "開啟全域 AGENTS.md",
        "Open Project AGENTS.md" => "開啟專案 AGENTS.md",
        "Search agents..." => "搜尋代理...",
        "Unavailable" => "無法使用",
        "ACP Registry" => "ACP 登錄檔",
        "Learn More" => "了解更多",
        "Configure Providers" => "設定提供者",
        "API Keys" => "API 金鑰",
        "Add your own keys to use AI without signing in." => {
            "新增你自己的金鑰，不登入也能使用 AI。"
        }
        "Welcome to Zed AI" => "歡迎使用 Zed AI",
        "Sign in to try Zed Pro free for 14 days." => "登入即可免費試用 Zed Pro 14 天。",
        "Try Zed Pro for Free" => "免費試用 Zed Pro",
        "(Current Plan)" => "（目前方案）",
        "Pro Trial" => "Zed Pro 試用",
        "Start Free Trial" => "開始免費試用",
        "Welcome to the Zed Pro Trial" => "歡迎使用 Zed Pro 試用",
        "Here's what you get for the next 14 days:" => "接下來 14 天你可以使用：",
        "Welcome to Zed Pro" => "歡迎使用 Zed Pro",
        "Welcome to Zed Business" => "歡迎使用 Zed Business",
        "Welcome to Zed Student" => "歡迎使用 Zed Student",
        "Here's what you get:" => "你將獲得：",
        "You have access to Zed's hosted models through your Pro subscription." => {
            "你可以透過 Pro 訂閱使用 Zed 託管模型。"
        }
        "You have access to Zed's hosted models through your Pro trial." => {
            "你可以透過 Pro 試用使用 Zed 託管模型。"
        }
        "You have access to Zed's hosted models through your Student subscription." => {
            "你可以透過 Student 訂閱使用 Zed 託管模型。"
        }
        "You have access to Zed's hosted models through your organization." => {
            "你可以透過組織使用 Zed 託管模型。"
        }
        "Zed's hosted models are disabled by your organization's configuration." => {
            "你的組織設定已停用 Zed 託管模型。"
        }
        "Subscribe for access to Zed's hosted models. Start with a 14 day free trial." => {
            "訂閱即可使用 Zed 託管模型。可先開始 14 天免費試用。"
        }
        "Subscribe for access to Zed's hosted models." => "訂閱即可使用 Zed 託管模型。",
        "Manage Subscription" => "管理訂閱",
        "Start 14-day Free Pro Trial" => "開始 14 天免費 Pro 試用",
        "Upgrade to Pro" => "升級至 Pro",
        "Sign in to have access to Zed's complete agentic experience with hosted models." => {
            "登入即可使用 Zed 搭配託管模型的完整代理體驗。"
        }
        "Sign In to use Zed AI" => "登入以使用 Zed AI",
        "API key set in {env_var} environment variable" => "API 金鑰已由 {env_var} 環境變數設定",
        "API key configured" => "API 金鑰已設定",
        "API key configured for {api_url}" => "已為 {api_url} 設定 API 金鑰",
        "dashboard" => "儀表板",
        "Visit the" => "造訪",
        "to generate an API key." => "以產生 API 金鑰。",
        "API Key Set in Environment Variable" => "API 金鑰已由環境變數設定",
        "API Key Configured" => "API 金鑰已設定",
        "Reset Key" => "重設金鑰",
        "Or set the {env_var} env var and restart Zed." => {
            "或設定 {env_var} 環境變數並重新啟動 Zed。"
        }
        "To use Zed's agent with OpenAI, you need to add an API key. Follow these steps:" => {
            "若要搭配 OpenAI 使用 Zed 的代理，你需要新增 API 金鑰。請依照下列步驟："
        }
        "Create one by visiting" => "前往此處建立",
        "Ensure your OpenAI account has credits" => "確認你的 OpenAI 帳戶有可用額度",
        "Paste your API key below and hit enter to start using the agent" => {
            "在下方貼上你的 API 金鑰，然後按 Enter 即可開始使用代理"
        }
        "You can also set the {env_var} environment variable and restart Zed." => {
            "你也可以設定 {env_var} 環境變數並重新啟動 Zed。"
        }
        "Note that having a subscription for another service like GitHub Copilot won't work." => {
            "請注意，GitHub Copilot 等其他服務的訂閱無法在此使用。"
        }
        "To reset your API key, unset the {env_var} environment variable." => {
            "若要重設 API 金鑰，請取消設定 {env_var} 環境變數。"
        }
        "Zed also supports OpenAI-compatible models." => "Zed 也支援 OpenAI-compatible 模型。",
        "Loading credentials…" => "正在載入憑證…",
        "2,000 accepted edit predictions" => "2,000 次已接受的編輯預測",
        "Unlimited prompts with your AI API keys" => "使用你的 AI API 金鑰取得不限次數的提示",
        "Unlimited use of external agents" => "不限次數使用外部代理",
        "Unlimited edit predictions" => "不限次數的編輯預測",
        "$20 of tokens in Zed agent" => "Zed 代理中價值 $20 的 tokens",
        "$10 of tokens in Zed agent" => "Zed 代理中價值 $10 的 tokens",
        "$5 of tokens in Zed agent" => "Zed 代理中價值 $5 的 tokens",
        "No credit card required" => "不需要信用卡",
        "Try it out for 14 days, no credit card required" => "試用 14 天，不需要信用卡",
        "Usage-based billing beyond $5" => "超過 $5 後按用量計費",
        "Usage-based billing" => "按用量計費",
        "Optional credit packs for additional usage" => "可選購額外用量的 credit packs",
        "Start now using API keys from your environment for the following providers:" => {
            "立即使用環境中的 API 金鑰開始，適用於下列提供者："
        }
        "Alternatively, you can continue to use GitHub Copilot as that's already set up." => {
            "或者，你也可以繼續使用已設定好的 GitHub Copilot。"
        }
        "Alternatively, you can use GitHub Copilot as your edit prediction provider." => {
            "或者，你也可以使用 GitHub Copilot 作為編輯預測提供者。"
        }
        "Use Copilot" => "使用 Copilot",
        "Configure Copilot" => "設定 Copilot",
        "Reinstall Copilot and Sign In" => "重新安裝 Copilot 並登入",
        "Reinstall Copilot and Sign in" => "重新安裝 Copilot 並登入",
        "Reinstall and Sign in" => "重新安裝並登入",
        "Copilot Enabled!" => "Copilot 已啟用！",
        "You're all set to use GitHub Copilot." => "你已可開始使用 GitHub Copilot。",
        "Done" => "完成",
        "You must have an active GitHub Copilot subscription." => {
            "你必須有有效的 GitHub Copilot 訂閱。"
        }
        "Enable Copilot by connecting your existing license once you have subscribed or renewed your subscription." => {
            "訂閱或續訂後，連接既有授權即可啟用 Copilot。"
        }
        "Subscribe on GitHub" => "在 GitHub 上訂閱",
        "An Error Happened" => "發生錯誤",
        "Copilot had issues starting. You can try reinstalling it and signing in again." => {
            "Copilot 啟動時發生問題。你可以嘗試重新安裝並再次登入。"
        }
        "Signing out of Copilot…" => "正在登出 Copilot…",
        "Signed out of Copilot" => "已登出 Copilot",
        "Copilot is reinstalling…" => "正在重新安裝 Copilot…",
        "Copilot is starting…" => "Copilot 正在啟動…",
        "Copilot has started." => "Copilot 已啟動。",
        "Copied!" => "已複製！",
        "Waiting for connection…" => "正在等待連線…",
        "Connect to GitHub" => "連線至 GitHub",
        "Use GitHub Copilot in Zed" => "在 Zed 中使用 GitHub Copilot",
        "Using Copilot requires an active subscription on GitHub." => {
            "使用 Copilot 需要有效的 GitHub 訂閱。"
        }
        "Paste this code into GitHub after clicking the button below." => {
            "點選下方按鈕後，將此代碼貼到 GitHub。"
        }
        "Starting Copilot…" => "正在啟動 Copilot…",
        "Signing into Copilot…" => "正在登入 Copilot…",
        "Sign in to GitHub" => "登入 GitHub",
        "Sign in to use GitHub Copilot" => "登入以使用 GitHub Copilot",
        "Authenticate To Use" => "驗證後使用",
        "To use Copilot for edit predictions, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot subscription." => {
            "若要使用 Copilot 進行編輯預測，你需要登入 GitHub。請注意，你的 GitHub 帳戶必須有有效的 Copilot 訂閱。"
        }
        "Copilot requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different edit predictions provider." => {
            "Copilot 需要有效的 GitHub Copilot 訂閱。請確認 Copilot 已設定並再試一次，或使用其他編輯預測提供者。"
        }
        "To use Zed's agent with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription." => {
            "若要搭配 GitHub Copilot 使用 Zed 的代理，你需要登入 GitHub。請注意，你的 GitHub 帳戶必須有有效的 Copilot Chat 訂閱。"
        }
        "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different LLM provider." => {
            "Copilot Chat 需要有效的 GitHub Copilot 訂閱。請確認 Copilot 已設定並再試一次，或使用其他 LLM 提供者。"
        }
        "Copilot can't be started: {error}" => "Copilot 無法啟動：{error}",
        "Reinstall Copilot" => "重新安裝 Copilot",
        "Powered by Codestral" => "由 Codestral 提供支援",
        "Missing API key for Codestral" => "缺少 Codestral API 金鑰",
        "Powered by Ollama ({model})" => "由 Ollama 提供支援（{model}）",
        "Ollama model not configured — configure a model before use" => {
            "尚未設定 Ollama 模型，請先設定模型再使用"
        }
        "Missing API key for Mercury" => "缺少 Mercury API 金鑰",
        "Mercury free tier limit reached" => "已達 Mercury 免費層級限制",
        "Powered by Mercury" => "由 Mercury 提供支援",
        "Powered by Zeta" => "由 Zeta 提供支援",
        "Choose a Plan" => "選擇方案",
        "Edit Predictions" => "編輯預測",
        "Disabled For This File" => "已對此檔案停用",
        "Providers" => "提供者",
        "Edit predictions are disabled for this organization." => "此組織已停用編輯預測。",
        "Show Edit Predictions For" => "顯示編輯預測的範圍",
        "Project identified as open source, and you're sharing data." => {
            "專案已識別為開源，且你正在分享資料。"
        }
        "Project identified as open source, but you're not sharing data." => {
            "專案已識別為開源，但你未分享資料。"
        }
        "Project not identified as open source. No data captured." => {
            "專案未識別為開源，不會擷取資料。"
        }
        "Project not identified as open source, and setting turned off." => {
            "專案未識別為開源，且設定已關閉。"
        }
        "Help us improve our open dataset model by sharing data from open source repositories. Zed must detect a license file in your repo for this setting to take effect. Files with sensitive data and secrets are excluded by default." => {
            "分享開源 repository 的資料，協助我們改善開放資料集模型。Zed 必須在你的 repository 中偵測到 license 檔，此設定才會生效。含有敏感資料與 secret 的檔案預設會排除。"
        }
        "No data captured." => "未擷取資料。",
        "Configure Excluded Files" => "設定排除檔案",
        "Open your settings to add sensitive paths for which Zed will never predict edits." => {
            "開啟設定以新增敏感路徑，Zed 絕不會針對這些路徑預測編輯。"
        }
        "This file is excluded." => "此檔案已排除。",
        "Predict Edit at Cursor" => "在游標處預測編輯",
        "Rate Predictions" => "評分預測",
        "Copilot: Next Edit Suggestions" => "Copilot：下一個編輯建議",
        "Go to Copilot Settings" => "前往 Copilot 設定",
        "You get 2,000 accepted suggestions at every keystroke for free, powered by Zeta, our open-source, open-data model" => {
            "你可以免費取得 2,000 次每次按鍵即時提供、且已接受的建議，由我們的開源開放資料模型 Zeta 提供支援"
        }
        "Sign In & Start Using" => "登入並開始使用",
        "Free tier limit reached" => "已達免費層級限制",
        "Upgrade to a paid plan to continue using the service" => "升級至付費方案以繼續使用此服務",
        "Usage" => "用量",
        "Subscribe to increase your limit" => "訂閱以提高你的額度",
        "Your GitHub account is less than 30 days old." => "你的 GitHub 帳戶建立未滿 30 天。",
        "Upgrade to Zed Pro or contact us." => "升級至 Zed Pro 或聯絡我們。",
        "You have an outstanding invoice" => "你有尚未付款的發票",
        "Check your payment status or contact us at billing-support@zed.dev to continue using this feature." => {
            "請檢查付款狀態，或透過 billing-support@zed.dev 聯絡我們以繼續使用此功能。"
        }
        "Experiment" => "實驗",
        "Sign Out" => "登出",
        "Authorized" => "已授權",
        "Sign In" => "登入",
        "Signing in…" => "正在登入…",
        "Approve" => "核准",
        "Reject" => "拒絕",
        "Reject All" => "全部拒絕",
        "Keep All" => "全部保留",
        "Continue Iterating" => "繼續迭代",
        "No changes to review" => "沒有可檢閱的變更",
        "Generating Changes…" => "正在產生變更…",
        "Allow" => "允許",
        "Deny" => "拒絕",
        "Ask" => "詢問",
        "View Diff" => "檢視差異",
        "View Branch Diff" => "檢視分支差異",
        "Stage" => "暫存",
        "Unstage" => "取消暫存",
        "Stage All" => "全部暫存",
        "Unstage All" => "全部取消暫存",
        "Toggle Staged" => "切換暫存狀態",
        "Stage and go to next hunk" => "暫存並移至下一個變更區塊",
        "Unstage and go to next hunk" => "取消暫存並移至下一個變更區塊",
        "Stage all changes" => "暫存所有變更",
        "Unstage all changes" => "取消暫存所有變更",
        "Go to previous hunk" => "移至上一個變更區塊",
        "Go to next hunk" => "移至下一個變更區塊",
        "No changes to commit" => "沒有可提交的變更",
        "No Changes to Commit" => "沒有可提交的變更",
        "Commit Tracked" => "提交已追蹤項目",
        "Commit in progress" => "提交進行中",
        "No commit message" => "沒有提交訊息",
        "Generate Commit Message" => "產生提交訊息",
        "Generating Commit…" => "正在產生提交訊息…",
        "Cancel Commit Message Generation" => "取消產生提交訊息",
        "Configure an LLM provider to generate commit messages" => "設定 LLM 提供者以產生提交訊息",
        "Open Commit Modal" => "開啟提交視窗",
        "Collapse Commit Editor" => "摺疊提交編輯器",
        "Expand Commit Editor" => "展開提交編輯器",
        "Amend" => "修訂提交",
        "Amend Tracked" => "修訂已追蹤項目",
        "Signoff" => "簽署",
        "You must resolve conflicts before committing" => "你必須先解決衝突才能提交",
        "You do not have write access to this project" => "你沒有此專案的寫入權限",
        "New…" => "新增…",
        "New Terminal" => "新增終端機",
        "New Center Terminal" => "新增中央終端機",
        "Split Pane" => "分割窗格",
        "Close Terminal Tab" => "關閉終端機索引標籤",
        "Paste Text" => "貼上文字",
        "Clear" => "清除",
        "Enter to Confirm" => "按 Enter 確認",
        "Inline Assist" => "行內輔助",
        "Add to Agent Thread" => "新增至代理對話串",
        "Failed to spawn terminal" => "無法產生終端機",
        "Edit Settings" => "編輯設定",
        "Edit settings.json" => "編輯 settings.json",
        "Delete from Recent Tasks" => "從最近工作中刪除",
        "Rerun Last Task" => "重新執行上一個工作",
        "Spawn Oneshot" => "產生一次性工作",
        "Spawn Oneshot Without History" => "產生一次性工作但不保留歷程",
        "Rerun Without History" => "重新執行但不保留歷程",
        "Spawn Without History" => "產生但不保留歷程",
        "Rerun" => "重新執行",
        "Spawn" => "產生",
        "Find a task, or run a command" => "尋找工作，或執行命令",
        "Find a task, or run a command in the central pane" => "尋找工作，或在中央窗格執行命令",
        "View Documentation" => "檢視文件",
        "Enable Vim mode" => "啟用 Vim 模式",
        "Install Dev Extension" => "安裝開發用延伸模組",
        "Rebuild" => "重新建置",
        "Upgrade" => "升級",
        "Incompatible" => "不相容",
        "Search extensions..." => "搜尋延伸模組...",
        "Search & Files" => "搜尋與檔案",
        "Version Control" => "版本控制",
        "General Settings" => "一般設定",
        "Scoped Settings" => "範圍設定",
        "Search Results" => "搜尋結果",
        "File Finder" => "檔案尋找器",
        "No settings match" => "沒有符合的設定",
        "Project Search Button" => "專案搜尋按鈕",
        "Use Smartcase Find" => "尋找時使用智慧大小寫",
        "Regex Search" => "規則運算式搜尋",
        "Use Smartcase Search" => "搜尋時使用智慧大小寫",
        "Center on Match" => "將相符項目置中",
        "Seed Search Query From Cursor" => "從游標位置帶入搜尋查詢",
        "Include Ignored in Search" => "搜尋時包含忽略的項目",
        "Show Branch Status Icon" => "顯示分支狀態圖示",
        "Show Branch Name" => "顯示分支名稱",
        "Show Commit Summary" => "顯示提交摘要",
        "Show Stage/Restore Buttons" => "顯示暫存/還原按鈕",
        "Reset to Default" => "重設為預設值",
        "Copy Link" => "複製連結",
        "Modified in" => "已修改於",
        "Server" => "伺服器",
        "Project Settings" => "專案設定",
        "Window & Layout" => "視窗與版面配置",
        "Panels" => "面板",
        "Text Rendering Mode" => "文字算繪模式",
        "Unnecessary Code Fade" => "非必要程式碼淡出",
        "Minimum Contrast For Highlights" => "醒目提示最低對比",
        "Show Wrap Guides" => "顯示換行參考線",
        "Wrap Guides" => "換行參考線",
        "Helix Mode" => "Helix 模式",
        "Auto Save" => "自動儲存",
        "Auto Save Mode" => "自動儲存模式",
        "Delay (milliseconds)" => "延遲（毫秒）",
        "Menu Delay" => "選單延遲",
        "Multibuffer" => "Multibuffer",
        "Double Click In Multibuffer" => "在 Multibuffer 中按兩下",
        "Expand Excerpt Lines" => "展開 Excerpt 行",
        "Excerpt Context Lines" => "Excerpt 上下文行",
        "Expand Outlines With Depth" => "依深度展開 Outline",
        "Diff View Style" => "差異檢視樣式",
        "Minimum Split Diff Width" => "分割差異檢視最小寬度",
        "Scrolling" => "捲動",
        "Scroll Beyond Last Line" => "捲動超過最後一行",
        "Vertical Scroll Margin" => "垂直捲動邊界",
        "Horizontal Scroll Margin" => "水平捲動邊界",
        "Scroll Sensitivity" => "捲動靈敏度",
        "Mouse Wheel Zoom" => "滑鼠滾輪縮放",
        "Fast Scroll Sensitivity" => "快速捲動靈敏度",
        "Autoscroll On Clicks" => "點選時自動捲動",
        "Sticky Scroll" => "Sticky Scroll",
        "Signature Help" => "簽章說明",
        "Auto Signature Help" => "自動簽章說明",
        "Show Signature Help After Edits" => "編輯後顯示簽章說明",
        "Snippet Sort Order" => "程式碼片段排序順序",
        "Hover Popover" => "Hover Popover",
        "Hiding Delay" => "隱藏延遲",
        "Drag And Drop Selection" => "拖放選取",
        "Gutter" => "Gutter",
        "Show Line Numbers" => "顯示行號",
        "Relative Line Numbers" => "相對行號",
        "Show Runnables" => "顯示可執行項目",
        "Show Breakpoints" => "顯示中斷點",
        "Show Bookmarks" => "顯示書籤",
        "Show Folds" => "顯示摺疊",
        "Min Line Number Digits" => "最小行號位數",
        "Inline Code Actions" => "行內程式碼動作",
        "Scrollbar" => "捲軸",
        "Minimap" => "縮圖地圖",
        "Toolbar" => "工具列",
        "File Types" => "檔案類型",
        "Inline Diagnostics" => "行內診斷",
        "LSP Pull Diagnostics" => "LSP 拉取診斷",
        "LSP Highlights" => "LSP 醒目提示",
        "Languages" => "語言",
        "File Scan" => "檔案掃描",
        "Status Bar" => "狀態列",
        "Title Bar" => "標題列",
        "Tab Bar" => "索引標籤列",
        "Tab Settings" => "索引標籤設定",
        "Preview Tabs" => "預覽索引標籤",
        "Layout" => "版面配置",
        "Pane Modifiers" => "窗格修飾鍵",
        "Pane Split Direction" => "窗格分割方向",
        "Git Panel" => "Git 面板",
        "Collaboration Panel" => "協作面板",
        "Agent Panel" => "代理面板",
        "Environment" => "環境",
        "Font" => "字型",
        "Display Settings" => "顯示設定",
        "Behavior Settings" => "行為設定",
        "Layout Settings" => "版面配置設定",
        "Advanced Settings" => "進階設定",
        "Git Integration" => "Git 整合",
        "Git Gutter" => "Git Gutter",
        "Inline Git Blame" => "行內 Git Blame",
        "Git Blame View" => "Git Blame 檢視",
        "Branch Picker" => "分支選擇器",
        "Git Hunks" => "Git 變更區塊",
        "Calls" => "通話",
        "Agent Configuration" => "代理設定",
        "Context Servers" => "內容伺服器",
        "Indent Guides" => "縮排參考線",
        "Autoclose" => "自動關閉",
        "Whitespace" => "空白字元",
        "Tasks" => "工作",
        "LSP" => "LSP",
        "LSP Completions" => "LSP 自動完成",
        "Debuggers" => "偵錯工具",
        "Prettier" => "Prettier",
        "Enabled" => "已啟用",
        "Delay" => "延遲",
        "Sticky" => "黏附",
        "Show" => "顯示",
        "Cursors" => "游標",
        "Git Diff" => "Git 差異",
        "Selected Text" => "選取文字",
        "Selected Symbol" => "選取符號",
        "Horizontal Scrollbar" => "水平捲軸",
        "Vertical Scrollbar" => "垂直捲軸",
        "Display In" => "顯示於",
        "Thumb" => "滑塊",
        "Thumb Border" => "滑塊邊框",
        "Max Width Columns" => "最大寬度欄數",
        "Breadcrumbs" => "階層連結",
        "Quick Actions" => "快速動作",
        "Selections Menu" => "選取範圍選單",
        "Agent Review" => "代理檢閱",
        "Code Actions" => "程式碼動作",
        "Default Mode" => "預設模式",
        "Toggle Relative Line Numbers" => "切換相對行號",
        "Use System Clipboard" => "使用系統剪貼簿",
        "Global Substitution Default" => "全域替換預設值",
        "Highlight on Yank Duration" => "Yank 後醒目提示時間",
        "Show Edit Predictions in Normal Mode" => "在 Normal 模式顯示編輯預測",
        "Cursor Shape - Normal Mode" => "游標形狀 - Normal 模式",
        "Cursor Shape - Insert Mode" => "游標形狀 - Insert 模式",
        "Cursor Shape - Replace Mode" => "游標形狀 - Replace 模式",
        "Cursor Shape - Visual Mode" => "游標形狀 - Visual 模式",
        "Custom Digraphs" => "自訂 Digraph",
        "File Type Associations" => "檔案類型關聯",
        "Max Severity" => "最高嚴重性",
        "Include Warnings" => "包含警告",
        "Update Debounce" => "更新 Debounce",
        "Padding" => "間距",
        "Minimum Column" => "最小欄位",
        "Debounce" => "Debounce",
        "Languages & Tools" => "語言與工具",
        "Whole Word" => "全字拼寫須相符",
        "Case Sensitive" => "區分大小寫",
        "Include Ignored" => "包含忽略項目",
        "Regex" => "Regex",
        "Search Wrap" => "搜尋循環",
        "File Icons" => "檔案圖示",
        "Modal Max Width" => "視窗最大寬度",
        "Skip Focus For Active In Search" => "搜尋時略過目前作用中檔案焦點",
        "File Scan Exclusions" => "檔案掃描排除項目",
        "File Scan Inclusions" => "檔案掃描包含項目",
        "Restore File State" => "還原檔案狀態",
        "Close on File Delete" => "刪除檔案時關閉",
        "Project Panel Button" => "專案面板按鈕",
        "Active Language Button" => "作用中語言按鈕",
        "Active Encoding Button" => "作用中編碼按鈕",
        "Cursor Position Button" => "游標位置按鈕",
        "Line Endings Button" => "行尾符號按鈕",
        "Terminal Button" => "終端機按鈕",
        "Diagnostics Button" => "診斷按鈕",
        "Debugger Button" => "偵錯工具按鈕",
        "Active File Name" => "作用中檔案名稱",
        "Show Project Items" => "顯示專案項目",
        "Show Onboarding Banner" => "顯示 onboarding 橫幅",
        "Show Sign In" => "顯示登入",
        "Show User Menu" => "顯示使用者選單",
        "Show User Picture" => "顯示使用者圖片",
        "Show Menus" => "顯示選單",
        "Button Layout" => "按鈕版面配置",
        "Custom Button Layout" => "自訂按鈕版面配置",
        "Show Tab Bar" => "顯示索引標籤列",
        "Show Git Status In Tabs" => "在索引標籤顯示 Git 狀態",
        "Show File Icons In Tabs" => "在索引標籤顯示檔案圖示",
        "Tab Close Position" => "索引標籤關閉按鈕位置",
        "Maximum Tabs" => "最大索引標籤數",
        "Show Navigation History Buttons" => "顯示導覽歷程按鈕",
        "Show Tab Bar Buttons" => "顯示索引標籤列按鈕",
        "Pinned Tabs Layout" => "釘選索引標籤版面配置",
        "Activate On Close" => "關閉後啟用",
        "Tab Show Diagnostics" => "索引標籤顯示診斷",
        "Show Close Button" => "顯示關閉按鈕",
        "Preview Tabs Enabled" => "啟用預覽索引標籤",
        "Enable Preview From Project Panel" => "從專案面板啟用預覽",
        "Enable Preview From File Finder" => "從檔案尋找器啟用預覽",
        "Enable Preview From Multibuffer" => "從 Multibuffer 啟用預覽",
        "Enable Preview Multibuffer From Code Navigation" => "從程式碼導覽啟用 Multibuffer 預覽",
        "Enable Preview File From Code Navigation" => "從程式碼導覽啟用檔案預覽",
        "Enable Keep Preview On Code Navigation" => "程式碼導覽時保留預覽",
        "Bottom Dock Layout" => "底部 Dock 版面配置",
        "Centered Layout Left Padding" => "置中版面左側間距",
        "Centered Layout Right Padding" => "置中版面右側間距",
        "Focus Follows Mouse" => "焦點跟隨滑鼠",
        "Focus Follows Mouse Debounce ms" => "焦點跟隨滑鼠 Debounce 毫秒",
        "Use System Window Tabs" => "使用系統視窗索引標籤",
        "Window Decorations" => "視窗裝飾",
        "Inactive Opacity" => "非作用中透明度",
        "Border Size" => "邊框大小",
        "Zoomed Padding" => "縮放窗格間距",
        "Vertical Split Direction" => "垂直分割方向",
        "Horizontal Split Direction" => "水平分割方向",
        "Project Panel Dock" => "專案面板 Dock",
        "Project Panel Default Width" => "專案面板預設寬度",
        "Hide .gitignore" => "隱藏 .gitignore",
        "Entry Spacing" => "項目間距",
        "Folder Icons" => "資料夾圖示",
        "Git Status" => "Git 狀態",
        "Indent Size" => "縮排大小",
        "Auto Reveal Entries" => "自動顯示項目",
        "Starts Open" => "啟動時開啟",
        "Auto Fold Directories" => "自動摺疊目錄",
        "Bold Folder Labels" => "粗體資料夾標籤",
        "Show Scrollbar" => "顯示捲軸",
        "Horizontal Scroll" => "水平捲動",
        "Show Diagnostics" => "顯示診斷",
        "Diagnostic Badges" => "診斷徽章",
        "Git Status Indicator" => "Git 狀態指示器",
        "Show Indent Guides" => "顯示縮排參考線",
        "Drag and Drop" => "拖放",
        "Hide Root" => "隱藏根目錄",
        "Hide Hidden" => "隱藏隱藏項目",
        "Sort Mode" => "排序模式",
        "Sort Order" => "排序順序",
        "Auto Open Files On Create" => "建立時自動開啟檔案",
        "Auto Open Files On Paste" => "貼上時自動開啟檔案",
        "Auto Open Files On Drop" => "拖放時自動開啟檔案",
        "Hidden Files" => "隱藏檔案",
        "Terminal Dock" => "終端機 Dock",
        "Terminal Panel Flexible Sizing" => "終端機面板彈性尺寸",
        "Show Count Badge" => "顯示數量徽章",
        "Outline Panel Button" => "Outline 面板按鈕",
        "Outline Panel Dock" => "Outline 面板 Dock",
        "Outline Panel Default Width" => "Outline 面板預設寬度",
        "Git Panel Button" => "Git 面板按鈕",
        "Git Panel Dock" => "Git 面板 Dock",
        "Git Panel Default Width" => "Git 面板預設寬度",
        "Git Panel Status Style" => "Git 面板狀態樣式",
        "Fallback Branch Name" => "備用分支名稱",
        "Sort By Path" => "依路徑排序",
        "Collapse Untracked Diff" => "摺疊未追蹤 Diff",
        "Tree View" => "樹狀檢視",
        "Diff Stats" => "Diff 統計",
        "Commit Title Max Length" => "提交標題最大長度",
        "Scroll Bar" => "捲軸",
        "Debugger Panel Dock" => "偵錯工具面板 Dock",
        "Collaboration Panel Button" => "協作面板按鈕",
        "Collaboration Panel Dock" => "協作面板 Dock",
        "Collaboration Panel Default Width" => "協作面板預設寬度",
        "Agent Panel Button" => "代理面板按鈕",
        "Agent Panel Dock" => "代理面板 Dock",
        "Agent Panel Flexible Sizing" => "代理面板彈性尺寸",
        "Agent Panel Default Width" => "代理面板預設寬度",
        "Agent Panel Default Height" => "代理面板預設高度",
        "Limit Content Width" => "限制內容寬度",
        "Max Content Width" => "最大內容寬度",
        "Stepping Granularity" => "單步執行粒度",
        "Save Breakpoints" => "儲存中斷點",
        "Timeout" => "逾時",
        "Log DAP Communications" => "記錄 DAP 通訊",
        "Format DAP Log Messages" => "格式化 DAP 記錄訊息",
        "Shell" => "Shell",
        "Program" => "程式",
        "Title Override" => "標題覆寫",
        "Working Directory" => "工作目錄",
        "Directory" => "目錄",
        "Environment Variables" => "環境變數",
        "Detect Virtual Environment" => "偵測虛擬環境",
        "Cursor Blinking" => "游標閃爍",
        "Alternate Scroll" => "替代捲動",
        "Minimum Contrast" => "最低對比",
        "Option As Meta" => "Option 作為 Meta",
        "Copy On Select" => "選取時複製",
        "Keep Selection On Copy" => "複製時保留選取",
        "Audible Bell" => "音效鈴聲",
        "Default Width" => "預設寬度",
        "Default Height" => "預設高度",
        "Max Scroll History Lines" => "最大捲動歷程行數",
        "Scroll Multiplier" => "捲動倍率",
        "Disable Git Integration" => "停用 Git 整合",
        "Enable Git Status" => "啟用 Git 狀態",
        "Enable Git Diff" => "啟用 Git Diff",
        "Visibility" => "可見性",
        "Show Avatar" => "顯示頭像",
        "Show Author Name" => "顯示作者名稱",
        "Hunk Style" => "Hunk 樣式",
        "Path Style" => "路徑樣式",
        "Mute On Join" => "加入時靜音",
        "Share On Join" => "加入時分享",
        "Test Audio" => "測試音訊",
        "Collaboration" => "協作",
        "Disable AI" => "停用 AI",
        "Threads Sidebar Side" => "對話串側邊欄位置",
        "Single File Review" => "單檔檢閱",
        "Enable Feedback" => "啟用回饋",
        "Notify When Agent Waiting" => "代理等待時通知",
        "Play Sound When Agent Done" => "代理完成時播放音效",
        "Expand Edit Card" => "展開編輯卡片",
        "Expand Terminal Card" => "展開終端機卡片",
        "Thinking Display" => "思考區塊顯示",
        "Cancel Generation On Terminal Stop" => "停止終端機時取消產生",
        "Use Modifier To Send" => "使用修飾鍵送出",
        "Message Editor Min Lines" => "訊息編輯器最小行數",
        "Show Turn Stats" => "顯示回合統計",
        "Show Merge Conflict Indicator" => "顯示合併衝突指示器",
        "Context Server Timeout" => "Context Server 逾時",
        "Display Mode" => "顯示模式",
        "AI" => "AI",
        "Tab Size" => "Tab 大小",
        "Hard Tabs" => "硬 Tab",
        "Auto Indent" => "自動縮排",
        "Auto Indent On Paste" => "貼上時自動縮排",
        "Soft Wrap" => "軟換行",
        "Preferred Line Length" => "偏好行長度",
        "Allow Rewrap" => "允許重新換行",
        "Line Width" => "線寬",
        "Active Line Width" => "作用中線寬",
        "Coloring" => "著色",
        "Background Coloring" => "背景著色",
        "Format On Save" => "儲存時格式化",
        "Remove Trailing Whitespace On Save" => "儲存時移除尾端空白",
        "Ensure Final Newline On Save" => "儲存時確保檔尾換行",
        "Line Ending" => "行尾符號",
        "Formatter" => "格式化工具",
        "Use On Type Format" => "使用輸入時格式化",
        "Code Actions On Format" => "格式化時執行程式碼動作",
        "Use Autoclose" => "使用自動關閉",
        "Use Auto Surround" => "使用自動環繞",
        "Always Treat Brackets As Autoclosed" => "一律將括號視為自動關閉",
        "JSX Tag Auto Close" => "JSX 標籤自動關閉",
        "Show Whitespaces" => "顯示空白字元",
        "Space Whitespace Indicator" => "空格空白字元指示器",
        "Tab Whitespace Indicator" => "Tab 空白字元指示器",
        "Show Completions On Input" => "輸入時顯示自動完成",
        "Show Completion Documentation" => "顯示自動完成文件",
        "Words" => "單字",
        "Words Min Length" => "單字最小長度",
        "Completion Menu Scrollbar" => "自動完成選單捲軸",
        "Completion Detail Alignment" => "自動完成詳細資訊對齊",
        "Completion Menu Item Kind" => "自動完成選單項目種類",
        "Show Value Hints" => "顯示值提示",
        "Show Type Hints" => "顯示型別提示",
        "Show Parameter Hints" => "顯示參數提示",
        "Show Other Hints" => "顯示其他提示",
        "Show Background" => "顯示背景",
        "Edit Debounce Ms" => "編輯 Debounce 毫秒",
        "Scroll Debounce Ms" => "捲動 Debounce 毫秒",
        "Toggle On Modifiers Press" => "按下修飾鍵時切換",
        "Variables" => "變數",
        "Prefer LSP" => "偏好 LSP",
        "Word Diff Enabled" => "啟用單字 Diff",
        "Middle Click Paste" => "中鍵貼上",
        "Extend Comment On Newline" => "換行時延伸註解",
        "Colorize Brackets" => "括號著色",
        "Vim/Emacs Modeline Support" => "Vim/Emacs Modeline 支援",
        "Image Viewer" => "圖片檢視器",
        "Auto Replace Emoji Shortcode" => "自動取代 Emoji Shortcode",
        "Drop Size Target" => "拖放目標大小",
        "Code Lens" => "Code Lens",
        "LSP Document Colors" => "LSP 文件色彩",
        "Enable Language Server" => "啟用語言伺服器",
        "Language Servers" => "語言伺服器",
        "Linked Edits" => "連結編輯",
        "Go To Definition Fallback" => "前往定義備援",
        "Go To Definition Scroll Strategy" => "前往定義捲動策略",
        "Semantic Tokens" => "語意 Token",
        "LSP Folding Ranges" => "LSP 摺疊範圍",
        "LSP Document Symbols" => "LSP 文件符號",
        "Fetch Timeout (milliseconds)" => "擷取逾時（毫秒）",
        "Insert Mode" => "插入模式",
        "Allowed" => "允許",
        "Parser" => "Parser",
        "Plugins" => "外掛",
        "Options" => "選項",
        "Show Edit Predictions" => "顯示編輯預測",
        "Audio Settings" => "音訊設定",
        "Output Audio Device" => "輸出音訊裝置",
        "Input Audio Device" => "輸入音訊裝置",
        "Select output audio device" => "選取輸出音訊裝置",
        "Select input audio device" => "選取輸入音訊裝置",
        "Proxy" => "代理伺服器",
        "The proxy to use for network requests." => "網路要求要使用的代理伺服器。",
        "Server URL" => "伺服器 URL",
        "The URL of the Zed server to connect to." => "要連線的 Zed 伺服器 URL。",
        "Settings Profiles" => "設定檔組態",
        "Any number of settings profiles that are temporarily applied on top of your existing user settings." => {
            "可在既有使用者設定之上暫時套用任意數量的設定檔組態。"
        }
        "Preview Channel" => "預覽版頻道",
        "Which settings should be activated only in Preview build of Zed." => {
            "哪些設定只應在 Zed Preview build 中啟用。"
        }
        "When opening Zed, avoid Restricted Mode by auto-trusting all projects, enabling use of all features without having to give permission to each new project." => {
            "開啟 Zed 時自動信任所有專案以避免受限模式，無需對每個新專案授權即可使用所有功能。"
        }
        "What to do when using the 'close active item' action with no tabs." => {
            "沒有索引標籤時，使用「關閉作用中項目」動作要執行的行為。"
        }
        "What to do when the last window is closed." => "最後一個視窗關閉時要執行的行為。",
        "Use native OS dialogs for 'Open' and 'Save As'." => {
            "針對「開啟」與「另存新檔」使用作業系統原生對話框。"
        }
        "Use native OS dialogs for confirmations." => "確認時使用作業系統原生對話框。",
        "Hide the values of variables in private files." => "隱藏私人檔案中的變數值。",
        "Globs to match against file paths to determine if a file is private." => {
            "用來比對檔案路徑、判斷檔案是否為私人檔案的 glob。"
        }
        "How `zed <path>` opens directories when no flag is specified." => {
            "未指定 flag 時，`zed <path>` 如何開啟目錄。"
        }
        "Whether or not to restore unsaved buffers on restart." => {
            "重新啟動時是否還原未儲存的緩衝區。"
        }
        "What to restore from the previous session when opening Zed." => {
            "開啟 Zed 時要從上一個工作階段還原的內容。"
        }
        "Send debug information like crash reports." => "傳送偵錯資訊，例如當機報告。",
        "Send anonymized usage data like what languages you're using Zed with." => {
            "傳送匿名使用資料，例如你在 Zed 中使用哪些語言。"
        }
        "Whether or not to automatically check for updates." => "是否自動檢查更新。",
        "Choose a static, fixed theme or dynamically select themes based on appearance and light/dark modes." => {
            "選擇固定佈景主題，或依外觀與明暗模式動態選取佈景主題。"
        }
        "The name of your selected theme." => "所選佈景主題的名稱。",
        "Choose whether to use the selected light or dark theme or to follow your OS appearance configuration." => {
            "選擇使用所選淺色/深色佈景主題，或跟隨作業系統外觀設定。"
        }
        "The theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式設為淺色，或模式設為系統且目前為淺色模式時使用的佈景主題。"
        }
        "The theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式設為深色，或模式設為系統且目前為深色模式時使用的佈景主題。"
        }
        "The custom set of icons Zed will associate with files and directories." => {
            "Zed 會與檔案和目錄關聯的自訂圖示集。"
        }
        "The name of your selected icon theme." => "所選圖示主題的名稱。",
        "Choose whether to use the selected light or dark icon theme or to follow your OS appearance configuration." => {
            "選擇使用所選淺色/深色圖示主題，或跟隨作業系統外觀設定。"
        }
        "The icon theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式設為淺色，或模式設為系統且目前為淺色模式時使用的圖示主題。"
        }
        "The icon theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式設為深色，或模式設為系統且目前為深色模式時使用的圖示主題。"
        }
        "Default Permission" => "預設權限",
        "Default Action" => "預設動作",
        "Thinking Effort" => "推理強度",
        "Profile" => "代理設定檔",
        "Font family for editor text." => "編輯器文字的字型系列。",
        "Font size for editor text." => "編輯器文字的字型大小。",
        "Font weight for editor text (100-900)." => "編輯器文字的字重 (100-900)。",
        "Font family for UI elements." => "UI 元素的字型系列。",
        "Font size for UI elements." => "UI 元素的字型大小。",
        "Font weight for UI elements (100-900)." => "UI 元素的字重 (100-900)。",
        "Line height for editor text." => "編輯器文字的行高。",
        "Font size for agent response text in the agent panel. Falls back to the regular UI font size." => {
            "代理面板中代理回覆文字的字型大小。未設定時會使用一般 UI 字型大小。"
        }
        "Font size for user messages text in the agent panel." => {
            "代理面板中使用者訊息文字的字型大小。"
        }
        "The text rendering mode to use." => "要使用的文字算繪模式。",
        "Modifier key for adding multiple cursors." => "新增多重游標使用的修飾鍵。",
        "Whether the cursor blinks in the editor." => "游標是否在編輯器中閃爍。",
        "Cursor shape for the editor." => "編輯器的游標形狀。",
        "When to hide the mouse cursor." => "隱藏滑鼠游標的時機。",
        "How much to fade out unused code (0.0 - 0.9)." => "未使用程式碼淡出的程度 (0.0 - 0.9)。",
        "How to highlight the current line." => "目前行的醒目提示方式。",
        "Highlight all occurrences of selected text." => "醒目提示所選文字的所有出現位置。",
        "Whether the text selection should have rounded corners." => "文字選取範圍是否使用圓角。",
        "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds." => {
            "在醒目提示背景上算繪文字時要維持的最低 APCA 感知對比。"
        }
        "Show wrap guides (vertical rulers)." => "顯示換行參考線（垂直尺規）。",
        "Character counts at which to show wrap guides." => "顯示換行參考線的字元數位置。",
        "Enable Helix mode and key bindings." => "啟用 Helix 模式與按鍵繫結。",
        "When to auto save buffer changes." => "自動儲存緩衝區變更的時機。",
        "Save after inactivity period (in milliseconds)." => "閒置一段時間後儲存（毫秒）。",
        "Display the which-key menu with matching bindings while a multi-stroke binding is pending." => {
            "多按鍵繫結等待輸入時，顯示包含相符繫結的 which-key 選單。"
        }
        "Delay in milliseconds before the which-key menu appears." => {
            "which-key 選單出現前的延遲時間（毫秒）。"
        }
        "What to do when multibuffer is double-clicked in some of its excerpts." => {
            "在 multibuffer 的某個 excerpt 上按兩下時要執行的動作。"
        }
        "How many lines to expand the multibuffer excerpts by default." => {
            "multibuffer excerpt 預設展開的行數。"
        }
        "How many lines of context to provide in multibuffer excerpts by default." => {
            "multibuffer excerpt 預設提供的上下文行數。"
        }
        "Default depth to expand outline items in the current file." => {
            "目前檔案中 outline 項目預設展開的深度。"
        }
        "How to display diffs in the editor." => "在編輯器中顯示 diff 的方式。",
        "The minimum width (in columns) at which the split diff view is used. When the editor is narrower, the diff view automatically switches to unified mode. Set to 0 to disable." => {
            "使用分割 diff 檢視所需的最小寬度（欄）。當編輯器較窄時，diff 檢視會自動切換為 unified 模式。設為 0 可停用。"
        }
        "Whether the editor will scroll beyond the last line." => "編輯器是否可捲動超過最後一行。",
        "The number of lines to keep above/below the cursor when auto-scrolling." => {
            "自動捲動時，在游標上方/下方保留的行數。"
        }
        "The number of characters to keep on either side when scrolling with the mouse." => {
            "使用滑鼠捲動時，在兩側保留的字元數。"
        }
        "Scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平與垂直捲動的靈敏度倍率。"
        }
        "Whether to zoom the editor font size with the mouse wheel while holding the primary modifier key." => {
            "按住主要修飾鍵時，是否使用滑鼠滾輪縮放編輯器字型大小。"
        }
        "Fast scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平與垂直快速捲動的靈敏度倍率。"
        }
        "Whether to scroll when clicking near the edge of the visible text area." => {
            "點選可見文字區域邊緣附近時是否捲動。"
        }
        "Whether to stick scopes to the top of the editor" => "是否將 scope 固定在編輯器頂端",
        "Custom line height value (must be at least 1.0)." => "自訂行高值（必須至少為 1.0）。",
        "The OpenType features to enable for rendering in text buffers." => {
            "算繪文字緩衝區時要啟用的 OpenType 功能。"
        }
        "The font fallbacks to use for rendering in text buffers." => {
            "算繪文字緩衝區時要使用的字型後援。"
        }
        "The OpenType features to enable for rendering in UI elements." => {
            "算繪 UI 元素時要啟用的 OpenType 功能。"
        }
        "The font fallbacks to use for rendering in the UI." => "算繪 UI 時要使用的字型後援。",
        "Automatically show a signature help pop-up." => "自動顯示簽章說明彈出視窗。",
        "Show the signature help pop-up after completions or bracket pairs are inserted." => {
            "插入自動完成或成對括號後顯示簽章說明彈出視窗。"
        }
        "Determines how snippets are sorted relative to other completion items." => {
            "決定程式碼片段相對於其他自動完成項目的排序方式。"
        }
        "Show the informational hover box when moving the mouse over symbols in the editor." => {
            "滑鼠移到編輯器中的符號上方時顯示資訊 hover 方塊。"
        }
        "Time to wait in milliseconds before showing the informational hover box." => {
            "顯示資訊 hover 方塊前等待的毫秒數。"
        }
        "Whether the hover popover sticks when the mouse moves toward it, allowing interaction with its contents." => {
            "滑鼠移向 hover popover 時是否讓它保持顯示，以便與內容互動。"
        }
        "Time to wait in milliseconds before hiding the hover popover after the mouse moves away." => {
            "滑鼠移開後隱藏 hover popover 前等待的毫秒數。"
        }
        "Enable drag and drop selection." => "啟用拖放選取。",
        "Delay in milliseconds before drag and drop selection starts." => {
            "拖放選取開始前的延遲毫秒數。"
        }
        "Show line numbers in the gutter." => "在 gutter 中顯示行號。",
        "Controls line number display in the editor's gutter. \"disabled\" shows absolute line numbers, \"enabled\" shows relative line numbers for each absolute line, and \"wrapped\" shows relative line numbers for every line, absolute or wrapped." => {
            "控制編輯器 gutter 中的行號顯示。\"disabled\" 顯示絕對行號，\"enabled\" 為每個絕對行顯示相對行號，\"wrapped\" 則為每一行（絕對行或自動換行）顯示相對行號。"
        }
        "Show runnable buttons in the gutter." => "在 gutter 中顯示可執行按鈕。",
        "Show breakpoints in the gutter." => "在 gutter 中顯示中斷點。",
        "Show bookmarks in the gutter." => "在 gutter 中顯示書籤。",
        "Show code folding controls in the gutter." => "在 gutter 中顯示程式碼摺疊控制項。",
        "Minimum number of characters to reserve space for in the gutter." => {
            "gutter 中要保留空間的最少字元數。"
        }
        "Show code action button at start of buffer line." => "在緩衝區行首顯示程式碼動作按鈕。",
        "When to show the scrollbar in the editor." => "在編輯器中顯示捲軸的時機。",
        "Show cursor positions in the scrollbar." => "在捲軸中顯示游標位置。",
        "Show Git diff indicators in the scrollbar." => "在捲軸中顯示 Git diff 指示器。",
        "Show buffer search result indicators in the scrollbar." => {
            "在捲軸中顯示緩衝區搜尋結果指示器。"
        }
        "Show selected text occurrences in the scrollbar." => "在捲軸中顯示選取文字出現位置。",
        "Show selected symbol occurrences in the scrollbar." => "在捲軸中顯示選取符號出現位置。",
        "Which diagnostic indicators to show in the scrollbar." => "要在捲軸中顯示哪些診斷指示器。",
        "When false, forcefully disables the horizontal scrollbar." => {
            "為 false 時，強制停用水平捲軸。"
        }
        "When false, forcefully disables the vertical scrollbar." => {
            "為 false 時，強制停用垂直捲軸。"
        }
        "When to show the minimap in the editor." => "在編輯器中顯示縮圖地圖的時機。",
        "Where to show the minimap in the editor." => "在編輯器中顯示縮圖地圖的位置。",
        "When to show the minimap thumb." => "顯示縮圖地圖 thumb 的時機。",
        "Border style for the minimap's scrollbar thumb." => "縮圖地圖捲軸 thumb 的邊框樣式。",
        "How to highlight the current line in the minimap." => "在縮圖地圖中醒目提示目前行的方式。",
        "Maximum number of columns to display in the minimap." => "縮圖地圖中要顯示的最大欄數。",
        "Show breadcrumbs." => "顯示階層連結。",
        "Show quick action buttons (e.g., search, selection, editor controls, etc.)." => {
            "顯示快速動作按鈕（例如搜尋、選取、編輯器控制項等）。"
        }
        "Show the selections menu in the editor toolbar." => "在編輯器工具列顯示選取範圍選單。",
        "Show agent review buttons in the editor toolbar." => "在編輯器工具列顯示代理檢閱按鈕。",
        "Show code action buttons in the editor toolbar." => "在編輯器工具列顯示程式碼動作按鈕。",
        "The default mode when Vim starts." => "Vim 啟動時的預設模式。",
        "Toggle relative line numbers in Vim mode." => "在 Vim 模式中切換相對行號。",
        "Controls when to use system clipboard in Vim mode." => {
            "控制在 Vim 模式中何時使用系統剪貼簿。"
        }
        "Enable smartcase searching in Vim mode." => "在 Vim 模式中啟用智慧大小寫搜尋。",
        "When enabled, the :substitute command replaces all matches in a line by default. The 'g' flag then toggles this behavior." => {
            "啟用時，:substitute 命令預設會取代一行中的所有相符項目。'g' flag 會切換此行為。"
        }
        "Duration in milliseconds to highlight yanked text in Vim mode." => {
            "在 Vim 模式中醒目提示 yank 文字的持續毫秒數。"
        }
        "Use regex search by default in Vim search." => "Vim 搜尋預設使用 regex。",
        "Whether edit predictions are shown in normal mode. By default, edit predictions are only shown in insert and replace modes." => {
            "是否在 normal 模式顯示編輯預測。預設只會在 insert 與 replace 模式顯示編輯預測。"
        }
        "Cursor shape for normal mode." => "normal 模式的游標形狀。",
        "Cursor shape for insert mode. Inherit uses the editor's cursor shape." => {
            "insert 模式的游標形狀。Inherit 會使用編輯器的游標形狀。"
        }
        "Cursor shape for replace mode." => "replace 模式的游標形狀。",
        "Cursor shape for visual mode." => "visual 模式的游標形狀。",
        "Custom digraph mappings for Vim mode." => "Vim 模式的自訂 digraph 對應。",
        "A mapping from languages to files and file extensions that should be treated as that language." => {
            "將語言對應到應視為該語言的檔案與副檔名。"
        }
        "Which level to use to filter out diagnostics displayed in the editor." => {
            "用來篩除編輯器中所顯示診斷的層級。"
        }
        "Whether to show warnings or not by default." => "預設是否顯示警告。",
        "Whether to show diagnostics inline or not." => "是否顯示行內診斷。",
        "The delay in milliseconds to show inline diagnostics after the last diagnostic update." => {
            "最後一次診斷更新後，顯示行內診斷前的延遲毫秒數。"
        }
        "The amount of padding between the end of the source line and the start of the inline diagnostic." => {
            "來源行尾與行內診斷起點之間的間距。"
        }
        "The minimum column at which to display inline diagnostics." => "顯示行內診斷的最小欄位。",
        "Whether to pull for language server-powered diagnostics or not." => {
            "是否拉取由語言伺服器提供的診斷。"
        }
        "Minimum time to wait before pulling diagnostics from the language server(s)." => {
            "從語言伺服器拉取診斷前等待的最短時間。"
        }
        "The debounce delay before querying highlights from the language." => {
            "向語言查詢醒目提示前的 debounce 延遲。"
        }
        "Search for whole words by default." => "預設搜尋完整單字。",
        "Search case-sensitively by default." => "預設區分大小寫搜尋。",
        "Whether to automatically enable case-sensitive search based on the search query." => {
            "是否根據搜尋查詢自動啟用區分大小寫搜尋。"
        }
        "Include ignored files in search results by default." => "搜尋結果預設包含忽略的檔案。",
        "Use regex search by default." => "預設使用 regex 搜尋。",
        "Whether the editor search results will loop." => "編輯器搜尋結果是否循環。",
        "Whether to center the current match in the editor" => "是否將目前相符項目置於編輯器中央",
        "When to populate a new search's query based on the text under the cursor." => {
            "何時根據游標下的文字填入新搜尋查詢。"
        }
        "Use gitignored files when searching." => "搜尋時使用 gitignore 忽略的檔案。",
        "Show file icons in the file finder." => "在檔案尋找器中顯示檔案圖示。",
        "Determines how much space the file finder can take up in relation to the available window width." => {
            "決定檔案尋找器相對於可用視窗寬度可占用的空間。"
        }
        "Whether the file finder should skip focus for the active file in search results." => {
            "檔案尋找器是否在搜尋結果中略過作用中檔案的焦點。"
        }
        "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"" => {
            "Zed 會完全排除的檔案或 glob。這些項目會在檔案掃描與檔案搜尋時略過，且不會顯示在專案檔案樹中。優先於 \"File Scan Inclusions\"。"
        }
        "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions" => {
            "Zed 會包含的檔案或 glob，即使它們被 git 忽略也一樣。這對未由 git 追蹤但仍對專案重要的檔案很有用。請注意，過於寬泛的 glob 可能拖慢 Zed 的檔案掃描。\"File Scan Exclusions\" 優先於這些包含項目。"
        }
        "Restore previous file state when reopening." => "重新開啟時還原先前的檔案狀態。",
        "Automatically close files that have been deleted." => "自動關閉已刪除的檔案。",
        "Show the project panel button in the status bar." => "在狀態列顯示專案面板按鈕。",
        "Show the active language button in the status bar." => "在狀態列顯示作用中語言按鈕。",
        "Control when to show the active encoding in the status bar." => {
            "控制何時在狀態列顯示作用中編碼。"
        }
        "Show the cursor position button in the status bar." => "在狀態列顯示游標位置按鈕。",
        "Show the active line endings button in the status bar." => {
            "在狀態列顯示作用中行尾符號按鈕。"
        }
        "Show the terminal button in the status bar." => "在狀態列顯示終端機按鈕。",
        "Show the project diagnostics button in the status bar." => "在狀態列顯示專案診斷按鈕。",
        "Show the project search button in the status bar." => "在狀態列顯示專案搜尋按鈕。",
        "Show the debugger button in the status bar." => "在狀態列顯示偵錯工具按鈕。",
        "Show the name of the active file in the status bar." => "在狀態列顯示作用中檔案名稱。",
        "Show git status indicators on the branch icon in the titlebar." => {
            "在標題列的分支圖示上顯示 Git 狀態指示器。"
        }
        "Show the branch name button in the titlebar." => "在標題列顯示分支名稱按鈕。",
        "Show the project host and name in the titlebar." => "在標題列顯示專案主機與名稱。",
        "Show banners announcing new features in the titlebar." => "在標題列顯示新功能公告橫幅。",
        "Show the sign in button in the titlebar." => "在標題列顯示登入按鈕。",
        "Show the user menu button in the titlebar." => "在標題列顯示使用者選單按鈕。",
        "Show user picture in the titlebar." => "在標題列顯示使用者圖片。",
        "Show the menus in the titlebar." => "在標題列顯示選單。",
        "(Linux only) choose how window control buttons are laid out in the titlebar." => {
            "（僅 Linux）選擇視窗控制按鈕在標題列中的版面配置。"
        }
        "GNOME-style layout string such as \"close:minimize,maximize\"." => {
            "GNOME 風格的版面配置字串，例如 \"close:minimize,maximize\"。"
        }
        "Show the tab bar in the editor." => "在編輯器中顯示索引標籤列。",
        "Show the Git file status on a tab item." => "在索引標籤項目上顯示 Git 檔案狀態。",
        "Show the file icon for a tab." => "顯示索引標籤的檔案圖示。",
        "Position of the close button in a tab." => "索引標籤中關閉按鈕的位置。",
        "Maximum open tabs in a pane. Will not close an unsaved tab." => {
            "一個窗格中的最大開啟索引標籤數。不會關閉未儲存的索引標籤。"
        }
        "Show the navigation history buttons in the tab bar." => "在索引標籤列顯示導覽歷程按鈕。",
        "Show the tab bar buttons (New, Split Pane, Zoom)." => {
            "顯示索引標籤列按鈕（新增、分割窗格、縮放）。"
        }
        "Show pinned tabs in a separate row above unpinned tabs." => {
            "將釘選索引標籤顯示在未釘選索引標籤上方的獨立列。"
        }
        "What to do after closing the current tab." => "關閉目前索引標籤後要執行的動作。",
        "Which files containing diagnostic errors/warnings to mark in the tabs." => {
            "哪些包含診斷錯誤/警告的檔案要在索引標籤中標記。"
        }
        "Controls the appearance behavior of the tab's close button." => {
            "控制索引標籤關閉按鈕的外觀行為。"
        }
        "Show opened editors as preview tabs." => "將開啟的編輯器顯示為預覽索引標籤。",
        "Whether to open tabs in preview mode when opened from the project panel with a single click." => {
            "從專案面板按一下開啟時，是否以預覽模式開啟索引標籤。"
        }
        "Whether to open tabs in preview mode when selected from the file finder." => {
            "從檔案尋找器選取時，是否以預覽模式開啟索引標籤。"
        }
        "Whether to open tabs in preview mode when opened from a multibuffer." => {
            "從 multibuffer 開啟時，是否以預覽模式開啟索引標籤。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a multibuffer." => {
            "使用程式碼導覽開啟 multibuffer 時，是否以預覽模式開啟索引標籤。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a single file." => {
            "使用程式碼導覽開啟單一檔案時，是否以預覽模式開啟索引標籤。"
        }
        "Whether to keep tabs in preview mode when code navigation is used to navigate away from them. If `enable_preview_file_from_code_navigation` or `enable_preview_multibuffer_from_code_navigation` is also true, the new tab may replace the existing one." => {
            "使用程式碼導覽離開索引標籤時，是否讓它們保持預覽模式。如果 `enable_preview_file_from_code_navigation` 或 `enable_preview_multibuffer_from_code_navigation` 也為 true，新索引標籤可能會取代既有索引標籤。"
        }
        "Layout mode for the bottom dock." => "底部 Dock 的版面配置模式。",
        "Left padding for centered layout." => "置中版面配置的左側間距。",
        "Right padding for centered layout." => "置中版面配置的右側間距。",
        "Whether to change focus to a pane when the mouse hovers over it." => {
            "滑鼠停留在窗格上時是否將焦點切換到該窗格。"
        }
        "Amount of time to wait before changing focus." => "切換焦點前等待的時間。",
        "(macOS only) whether to allow Windows to tab together." => {
            "（僅 macOS）是否允許視窗合併為系統索引標籤。"
        }
        "(Linux only) whether Zed or your compositor should draw window decorations." => {
            "（僅 Linux）由 Zed 或 compositor 繪製視窗裝飾。"
        }
        "Opacity of inactive panels (0.0 - 1.0)." => "非作用中面板的不透明度 (0.0 - 1.0)。",
        "Size of the border surrounding the active pane." => "作用中窗格周圍邊框的大小。",
        "Show padding for zoomed panes." => "顯示縮放窗格的間距。",
        "Direction to split vertically." => "垂直分割的方向。",
        "Direction to split horizontally." => "水平分割的方向。",
        "Where to dock the project panel." => "專案面板的 Dock 位置。",
        "Default width of the project panel in pixels." => "專案面板的預設寬度（像素）。",
        "Whether to hide the gitignore entries in the project panel." => {
            "是否在專案面板中隱藏 gitignore 項目。"
        }
        "Spacing between worktree entries in the project panel." => {
            "專案面板中 worktree 項目之間的間距。"
        }
        "Show file icons in the project panel." => "在專案面板中顯示檔案圖示。",
        "Whether to show folder icons or chevrons for directories in the project panel." => {
            "在專案面板中是否對目錄顯示資料夾圖示或 chevron。"
        }
        "Show the Git status in the project panel." => "在專案面板中顯示 Git 狀態。",
        "Amount of indentation for nested items." => "巢狀項目的縮排量。",
        "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active." => {
            "對應專案項目變成作用中時，是否在專案面板中自動顯示該項目。"
        }
        "Whether the project panel should open on startup." => "專案面板是否在啟動時開啟。",
        "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside." => {
            "當目錄內只有一個子目錄時，是否自動摺疊目錄並顯示 compact folders。"
        }
        "Whether to show folder names with bold text in the project panel." => {
            "是否在專案面板中以粗體顯示資料夾名稱。"
        }
        "Show the scrollbar in the project panel." => "在專案面板中顯示捲軸。",
        "Whether to allow horizontal scrolling in the project panel. When disabled, the view is always locked to the leftmost position and long file names are clipped." => {
            "是否允許專案面板水平捲動。停用時，檢視會一律鎖定在最左側，較長的檔名會被截斷。"
        }
        "Which files containing diagnostic errors/warnings to mark in the project panel." => {
            "哪些包含診斷錯誤/警告的檔案要在專案面板中標記。"
        }
        "Show error and warning count badges next to file names in the project panel." => {
            "在專案面板的檔名旁顯示錯誤與警告數量徽章。"
        }
        "Show a git status indicator next to file names in the project panel." => {
            "在專案面板的檔名旁顯示 Git 狀態指示器。"
        }
        "Whether to stick parent directories at top of the project panel." => {
            "是否將父目錄固定在專案面板頂端。"
        }
        "Show indent guides in the project panel." => "在專案面板中顯示縮排參考線。",
        "Whether to enable drag-and-drop operations in the project panel." => {
            "是否在專案面板中啟用拖放操作。"
        }
        "Whether to hide the root entry when only one folder is open in the window." => {
            "視窗只開啟一個資料夾時是否隱藏根項目。"
        }
        "Whether to hide the hidden entries in the project panel." => {
            "是否在專案面板中隱藏隱藏項目。"
        }
        "Sort order for entries in the project panel." => "專案面板中項目的排序順序。",
        "Whether to sort file and folder names case-sensitively in the project panel." => {
            "是否在專案面板中以區分大小寫方式排序檔案與資料夾名稱。"
        }
        "Whether to automatically open newly created files in the editor." => {
            "是否在編輯器中自動開啟新建立的檔案。"
        }
        "Whether to automatically open files after pasting or duplicating them." => {
            "貼上或複製檔案後是否自動開啟。"
        }
        "Whether to automatically open files dropped from external sources." => {
            "從外部來源拖放檔案後是否自動開啟。"
        }
        "Globs to match files that will be considered \"hidden\" and can be hidden from the project panel." => {
            "用來比對視為「隱藏」且可從專案面板隱藏之檔案的 glob。"
        }
        "Where to dock the terminal panel." => "終端機面板的 Dock 位置。",
        "Whether the terminal panel should use flexible (proportional) sizing when docked to the left or right." => {
            "終端機面板停靠在左側或右側時，是否使用彈性（比例）尺寸。"
        }
        "Show a badge on the terminal panel icon with the count of open terminals." => {
            "在終端機面板圖示上顯示開啟終端機數量徽章。"
        }
        "Show the outline panel button in the status bar." => "在狀態列顯示 outline 面板按鈕。",
        "Where to dock the outline panel." => "outline 面板的 Dock 位置。",
        "Default width of the outline panel in pixels." => "outline 面板的預設寬度（像素）。",
        "Show file icons in the outline panel." => "在 outline 面板中顯示檔案圖示。",
        "Whether to show folder icons or chevrons for directories in the outline panel." => {
            "在 outline 面板中是否對目錄顯示資料夾圖示或 chevron。"
        }
        "Show the Git status in the outline panel." => "在 outline 面板中顯示 Git 狀態。",
        "Whether to reveal when a corresponding outline entry becomes active." => {
            "對應 outline 項目變成作用中時，是否自動顯示該項目。"
        }
        "Whether to fold directories automatically when a directory contains only one subdirectory." => {
            "目錄只包含一個子目錄時是否自動摺疊目錄。"
        }
        "When to show indent guides in the outline panel." => {
            "在 outline 面板中顯示縮排參考線的時機。"
        }
        "Show the Git panel button in the status bar." => "在狀態列顯示 Git 面板按鈕。",
        "Where to dock the Git panel." => "Git 面板的 Dock 位置。",
        "Default width of the Git panel in pixels." => "Git 面板的預設寬度（像素）。",
        "How entry statuses are displayed." => "項目狀態的顯示方式。",
        "Default branch name will be when init.defaultbranch is not set in Git." => {
            "Git 中未設定 init.defaultbranch 時使用的預設分支名稱。"
        }
        "Enable to sort entries in the panel by path, disable to sort by status." => {
            "啟用後依路徑排序面板項目；停用後依狀態排序。"
        }
        "Whether to collapse untracked files in the diff panel." => {
            "是否在 diff 面板中摺疊未追蹤檔案。"
        }
        "Enable to show entries in tree view list, disable to show in flat view list." => {
            "啟用後以樹狀檢視清單顯示項目；停用後以平面檢視清單顯示。"
        }
        "Show file icons next to the Git status icon." => "在 Git 狀態圖示旁顯示檔案圖示。",
        "Whether to show folder icons or chevrons for directories in the git panel." => {
            "在 Git 面板中是否對目錄顯示資料夾圖示或 chevron。"
        }
        "Whether to show the addition/deletion change count next to each file in the Git panel." => {
            "是否在 Git 面板中每個檔案旁顯示新增/刪除變更數。"
        }
        "Whether to show a badge on the git panel icon with the count of uncommitted changes." => {
            "是否在 Git 面板圖示上顯示未提交變更數量徽章。"
        }
        "Maximum length of the commit message title before a warning is shown. Set to 0 to disable." => {
            "顯示警告前提交訊息標題的最大長度。設為 0 可停用。"
        }
        "How and when the scrollbar should be displayed." => "捲軸應如何以及何時顯示。",
        "The dock position of the debug panel." => "偵錯面板的 Dock 位置。",
        "Show the collaboration panel button in the status bar." => "在狀態列顯示協作面板按鈕。",
        "Where to dock the collaboration panel." => "協作面板的 Dock 位置。",
        "Default width of the collaboration panel in pixels." => "協作面板的預設寬度（像素）。",
        "Whether to show the agent panel button in the status bar." => {
            "是否在狀態列顯示代理面板按鈕。"
        }
        "Where to dock the agent panel." => "代理面板的 Dock 位置。",
        "Whether the agent panel should use flexible (proportional) sizing when docked to the left or right." => {
            "代理面板停靠在左側或右側時，是否使用彈性（比例）尺寸。"
        }
        "Default width when the agent panel is docked to the left or right." => {
            "代理面板停靠在左側或右側時的預設寬度。"
        }
        "Default height when the agent panel is docked to the bottom." => {
            "代理面板停靠在底部時的預設高度。"
        }
        "Whether to constrain the agent panel content to a maximum width, centering it when the panel is wider, for optimal readability." => {
            "是否將代理面板內容限制在最大寬度內，並在面板較寬時置中，以取得最佳可讀性。"
        }
        "Maximum content width in pixels. Content will be centered when the panel is wider than this value." => {
            "最大內容寬度（像素）。面板寬於此值時，內容會置中。"
        }
        "Determines the stepping granularity for debug operations." => {
            "決定偵錯操作的單步執行粒度。"
        }
        "Whether breakpoints should be reused across Zed sessions." => {
            "是否跨 Zed 工作階段重用中斷點。"
        }
        "Time in milliseconds until timeout error when connecting to a TCP debug adapter." => {
            "連線到 TCP 偵錯配接器時，發生逾時錯誤前的毫秒數。"
        }
        "Whether to log messages between active debug adapters and Zed." => {
            "是否記錄作用中偵錯配接器與 Zed 之間的訊息。"
        }
        "Whether to format DAP messages when adding them to debug adapter logger." => {
            "將 DAP 訊息新增至偵錯配接器 logger 時，是否格式化訊息。"
        }
        "What shell to use when opening a terminal." => "開啟終端機時要使用的 shell。",
        "The shell program to use." => "要使用的 shell 程式。",
        "The shell program to run." => "要執行的 shell 程式。",
        "The arguments to pass to the shell program." => "要傳遞給 shell 程式的引數。",
        "An optional string to override the title of the terminal tab." => {
            "用來覆寫終端機索引標籤標題的選用字串。"
        }
        "What working directory to use when launching the terminal." => {
            "啟動終端機時要使用的工作目錄。"
        }
        "The directory path to use (will be shell expanded)." => {
            "要使用的目錄路徑（會經過 shell 展開）。"
        }
        "Key-value pairs to add to the terminal's environment." => {
            "要新增至終端機環境的 key-value 配對。"
        }
        "Activates the Python virtual environment, if one is found, in the terminal's working directory." => {
            "若在終端機工作目錄中找到 Python 虛擬環境，則啟用它。"
        }
        "Font size for terminal text. If not set, defaults to buffer font size." => {
            "終端機文字的字型大小。未設定時預設使用緩衝區字型大小。"
        }
        "Font family for terminal text. If not set, defaults to buffer font family." => {
            "終端機文字的字型系列。未設定時預設使用緩衝區字型系列。"
        }
        "Font fallbacks for terminal text. If not set, defaults to buffer font fallbacks." => {
            "終端機文字的字型後援。未設定時預設使用緩衝區字型後援。"
        }
        "Font weight for terminal text in CSS weight units (100-900)." => {
            "終端機文字的字重，以 CSS 字重單位表示 (100-900)。"
        }
        "Font features for terminal text." => "終端機文字的字型功能。",
        "Line height for terminal text." => "終端機文字的行高。",
        "Default cursor shape for the terminal (bar, block, underline, or hollow)." => {
            "終端機的預設游標形狀（bar、block、underline 或 hollow）。"
        }
        "Sets the cursor blinking behavior in the terminal." => "設定終端機中的游標閃爍行為。",
        "Whether alternate scroll mode is active by default (converts mouse scroll to arrow keys in apps like Vim)." => {
            "替代捲動模式是否預設啟用（在 Vim 等應用程式中將滑鼠捲動轉為方向鍵）。"
        }
        "The minimum APCA perceptual contrast between foreground and background colors (0-106)." => {
            "前景與背景色彩之間的最低 APCA 感知對比 (0-106)。"
        }
        "Whether the option key behaves as the meta key." => "Option 鍵是否作為 Meta 鍵。",
        "Whether selecting text in the terminal automatically copies to the system clipboard." => {
            "在終端機中選取文字時，是否自動複製到系統剪貼簿。"
        }
        "Whether to keep the text selection after copying it to the clipboard." => {
            "複製到剪貼簿後是否保留文字選取範圍。"
        }
        "Whether to play a sound when the BEL character (`\\a`, `0x07`) is printed" => {
            "列印 BEL 字元 (`\\a`, `0x07`) 時是否播放音效"
        }
        "Default width when the terminal is docked to the left or right (in pixels)." => {
            "終端機停靠在左側或右側時的預設寬度（像素）。"
        }
        "Default height when the terminal is docked to the bottom (in pixels)." => {
            "終端機停靠在底部時的預設高度（像素）。"
        }
        "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)." => {
            "捲動歷程中保留的最大行數（最大 100,000；0 會停用捲動）。"
        }
        "The multiplier for scrolling in the terminal with the mouse wheel" => {
            "在終端機中使用滑鼠滾輪捲動的倍率"
        }
        "Display the terminal title in breadcrumbs inside the terminal pane." => {
            "在終端機窗格內的階層連結中顯示終端機標題。"
        }
        "When to show the scrollbar in the terminal." => "在終端機中顯示捲軸的時機。",
        "Disable all Git integration features in Zed." => "停用 Zed 中所有 Git 整合功能。",
        "Show Git status information in the editor." => "在編輯器中顯示 Git 狀態資訊。",
        "Show Git diff information in the editor." => "在編輯器中顯示 Git diff 資訊。",
        "Control whether Git status is shown in the editor's gutter." => {
            "控制是否在編輯器 gutter 中顯示 Git 狀態。"
        }
        "Debounce threshold in milliseconds after which changes are reflected in the Git gutter." => {
            "變更反映到 Git gutter 前的 debounce 閾值（毫秒）。"
        }
        "Whether or not to show Git blame data inline in the currently focused line." => {
            "是否在目前聚焦行中行內顯示 Git blame 資料。"
        }
        "The delay after which the inline blame information is shown." => {
            "顯示行內 blame 資訊前的延遲。"
        }
        "Padding between the end of the source line and the start of the inline blame in columns." => {
            "來源行尾與行內 blame 起點之間的欄位間距。"
        }
        "The minimum column number at which to show the inline blame information." => {
            "顯示行內 blame 資訊的最小欄位數。"
        }
        "Show commit summary as part of the inline blame." => {
            "將提交摘要作為行內 blame 的一部分顯示。"
        }
        "Show the avatar of the author of the commit." => "顯示提交作者的頭像。",
        "Show author name as part of the commit information in branch picker." => {
            "在分支選擇器的提交資訊中顯示作者名稱。"
        }
        "How Git hunks are displayed visually in the editor." => {
            "Git hunks 在編輯器中的視覺顯示方式。"
        }
        "Should the name or path be displayed first in the git view." => {
            "Git 檢視中應優先顯示名稱或路徑。"
        }
        "Whether to show the stage and restore buttons on diff hunks." => {
            "是否在 diff hunks 上顯示暫存與還原按鈕。"
        }
        "Whether the microphone should be muted when joining a channel or a call." => {
            "加入頻道或通話時，麥克風是否應靜音。"
        }
        "Whether your current project should be shared when joining an empty channel." => {
            "加入空頻道時是否分享目前專案。"
        }
        "Whether to disable all AI features in Zed." => "是否停用 Zed 中所有 AI 功能。",
        "Which side of the window the threads sidebar appears on." => {
            "對話串側邊欄出現在視窗的哪一側。"
        }
        "When enabled, agent edits will also be displayed in single-file buffers for review." => {
            "啟用後，代理編輯也會顯示在單檔緩衝區中供檢閱。"
        }
        "Show voting thumbs up/down icon buttons for feedback on agent edits." => {
            "顯示對代理編輯提供回饋的讚/倒讚圖示按鈕。"
        }
        "Where to show notifications when the agent has completed its response or needs confirmation before running a tool action." => {
            "代理完成回覆或執行工具動作前需要確認時，要在哪裡顯示通知。"
        }
        "When to play a sound when the agent has either completed its response, or needs user input." => {
            "代理完成回覆或需要使用者輸入時，何時播放音效。"
        }
        "Whether to have edit cards in the agent panel expanded, showing a Preview of the diff." => {
            "代理面板中的編輯卡片是否展開，以顯示 diff 預覽。"
        }
        "Whether to have terminal cards in the agent panel expanded, showing the whole command output." => {
            "代理面板中的終端機卡片是否展開，以顯示完整命令輸出。"
        }
        "How thinking blocks should be displayed by default. 'Auto' fully expands during streaming, then auto-collapses when done. 'Preview' auto-expands with a height constraint during streaming. 'Always Expanded' shows full content. 'Always Collapsed' keeps them collapsed." => {
            "思考區塊的預設顯示方式。'Auto' 會在串流期間完全展開，完成後自動收合。'Preview' 會在串流期間以高度限制自動展開。'Always Expanded' 顯示完整內容。'Always Collapsed' 保持收合。"
        }
        "Whether clicking the stop button on a running terminal tool should also cancel the agent's generation. Note that this only applies to the stop button, not to ctrl+c inside the terminal." => {
            "點選執行中終端機工具的停止按鈕時，是否也取消代理產生。請注意，這只套用於停止按鈕，不套用於終端機內的 ctrl+c。"
        }
        "Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages." => {
            "是否一律使用 cmd-enter（Linux 或 Windows 上為 ctrl-enter）送出訊息。"
        }
        "Minimum number of lines to display in the agent message editor." => {
            "代理訊息編輯器中要顯示的最小行數。"
        }
        "Whether to show turn statistics like elapsed time during generation and final turn duration." => {
            "是否顯示回合統計，例如產生期間經過時間與最終回合時間。"
        }
        "Whether to show the merge conflict indicator in the status bar that offers to resolve conflicts using the agent." => {
            "是否在狀態列顯示合併衝突指示器，提供使用代理解決衝突。"
        }
        "Default timeout in seconds for context server tool calls. Can be overridden per-server in context_servers configuration." => {
            "context server 工具呼叫的預設逾時秒數。可在 context_servers 設定中依伺服器覆寫。"
        }
        "When to show edit predictions previews in buffer. The eager mode displays them inline, while the subtle mode displays them only when holding a modifier key." => {
            "何時在緩衝區中顯示編輯預測預覽。eager 模式會行內顯示，subtle 模式則只在按住修飾鍵時顯示。"
        }
        "How many columns a tab should occupy." => "Tab 應占用的欄數。",
        "Whether to indent lines using tab characters, as opposed to multiple spaces." => {
            "是否使用 tab 字元縮排行，而不是多個空格。"
        }
        "Controls automatic indentation behavior when typing." => "控制輸入時的自動縮排行為。",
        "Whether indentation of pasted content should be adjusted based on the context." => {
            "是否根據上下文調整貼上內容的縮排。"
        }
        "How to soft-wrap long lines of text." => "長文字行的軟換行方式。",
        "Show wrap guides in the editor." => "在編輯器中顯示換行參考線。",
        "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled." => {
            "對啟用軟換行的緩衝區，在哪個欄位進行軟換行。"
        }
        "Character counts at which to show wrap guides in the editor." => {
            "在編輯器中顯示換行參考線的字元數位置。"
        }
        "Controls where the `editor::rewrap` action is allowed for this language." => {
            "控制此語言中允許使用 `editor::rewrap` 動作的位置。"
        }
        "Display indent guides in the editor." => "在編輯器中顯示縮排參考線。",
        "The width of the indent guides in pixels, between 1 and 10." => {
            "縮排參考線寬度（像素），介於 1 到 10。"
        }
        "The width of the active indent guide in pixels, between 1 and 10." => {
            "作用中縮排參考線寬度（像素），介於 1 到 10。"
        }
        "Determines how indent guides are colored." => "決定縮排參考線的著色方式。",
        "Determines how indent guide backgrounds are colored." => "決定縮排參考線背景的著色方式。",
        "Whether or not to perform a buffer format before saving." => {
            "儲存前是否執行緩衝區格式化。"
        }
        "Whether or not to remove any trailing whitespace from lines of a buffer before saving it." => {
            "儲存緩衝區前是否移除各行尾端空白。"
        }
        "Whether or not to ensure there's a single newline at the end of a buffer when saving it." => {
            "儲存緩衝區時，是否確保檔尾有單一換行。"
        }
        "How line endings should be handled for new files and during format and save operations." => {
            "新檔案以及格式化/儲存操作期間應如何處理行尾符號。"
        }
        "How to perform a buffer format." => "執行緩衝區格式化的方式。",
        "Whether to use additional LSP queries to format (and amend) the code after every \"trigger\" symbol input, defined by LSP server capabilities" => {
            "是否在每次輸入由 LSP server capability 定義的 \"trigger\" 符號後，使用額外 LSP 查詢來格式化（並修正）程式碼"
        }
        "Additional code actions to run when formatting." => "格式化時要執行的額外程式碼動作。",
        "Whether to automatically type closing characters for you. For example, when you type '(', Zed will automatically add a closing ')' at the correct position." => {
            "是否自動輸入關閉字元。例如輸入 '(' 時，Zed 會在正確位置自動加入關閉的 ')'。"
        }
        "Whether to automatically surround text with characters for you. For example, when you select text and type '(', Zed will automatically surround text with ()." => {
            "是否自動用字元環繞文字。例如選取文字並輸入 '(' 時，Zed 會自動用 () 環繞文字。"
        }
        "Controls whether the closing characters are always skipped over and auto-removed no matter how they were inserted." => {
            "控制關閉字元無論如何插入，是否一律可被跳過並自動移除。"
        }
        "Whether to automatically close JSX tags." => "是否自動關閉 JSX 標籤。",
        "Whether to show tabs and spaces in the editor." => "是否在編輯器中顯示 tab 與空格。",
        "Visible character used to render space characters when show_whitespaces is enabled (default: \"•\")" => {
            "啟用 show_whitespaces 時用來算繪空格字元的可見字元（預設：\"•\"）"
        }
        "Visible character used to render tab characters when show_whitespaces is enabled (default: \"→\")" => {
            "啟用 show_whitespaces 時用來算繪 tab 字元的可見字元（預設：\"→\"）"
        }
        "Whether to pop the completions menu while typing in an editor without explicitly requesting it." => {
            "在編輯器中輸入時，是否在未明確要求的情況下彈出自動完成選單。"
        }
        "Whether to display inline and alongside documentation for items in the completions menu." => {
            "是否顯示自動完成選單項目的行內與旁側文件。"
        }
        "Controls how words are completed." => "控制單字完成的方式。",
        "How many characters has to be in the completions query to automatically show the words-based completions." => {
            "自動顯示以單字為基礎的完成項目前，完成查詢中必須有多少字元。"
        }
        "When to show the scrollbar in the completion menu." => "在自動完成選單中顯示捲軸的時機。",
        "Whether to align detail text in code completions context menus left or right." => {
            "程式碼自動完成內容選單中的詳細文字要向左或向右對齊。"
        }
        "How to display the LSP item kind (function, method, variable, etc.) of each entry in the completions menu." => {
            "如何在自動完成選單中顯示每個項目的 LSP item kind（function、method、variable 等）。"
        }
        "Global switch to toggle hints on and off." => "開啟或關閉提示的全域開關。",
        "Global switch to toggle inline values on and off when debugging." => {
            "偵錯時開啟或關閉行內值的全域開關。"
        }
        "Whether type hints should be shown." => "是否顯示型別提示。",
        "Whether parameter hints should be shown." => "是否顯示參數提示。",
        "Whether other hints should be shown." => "是否顯示其他提示。",
        "Show a background for inlay hints." => "為行內提示顯示背景。",
        "Whether or not to debounce inlay hints updates after buffer edits (set to 0 to disable debouncing)." => {
            "緩衝區編輯後，是否對行內提示更新進行 debounce（設為 0 可停用 debounce）。"
        }
        "Whether or not to debounce inlay hints updates after buffer scrolls (set to 0 to disable debouncing)." => {
            "緩衝區捲動後，是否對行內提示更新進行 debounce（設為 0 可停用 debounce）。"
        }
        "Toggles inlay hints (hides or shows) when the user presses the modifiers specified." => {
            "使用者按下指定修飾鍵時切換行內提示（隱藏或顯示）。"
        }
        "Whether tasks are enabled for this language." => "此語言是否啟用工作。",
        "Extra task variables to set for a particular language." => {
            "針對特定語言要設定的額外工作變數。"
        }
        "Use LSP tasks over Zed language extension tasks." => {
            "優先使用 LSP 工作，而不是 Zed 語言延伸模組工作。"
        }
        "Whether to enable word diff highlighting in the editor. When enabled, changed words within modified lines are highlighted to show exactly what changed." => {
            "是否在編輯器中啟用單字 diff 醒目提示。啟用時，已修改行中的變更單字會醒目提示，以顯示確切變更。"
        }
        "Preferred debuggers for this language." => "此語言偏好的偵錯工具。",
        "Enable middle-click paste on Linux." => "在 Linux 上啟用中鍵貼上。",
        "Whether to start a new line with a comment when a previous line is a comment as well." => {
            "上一行也是註解時，是否以註解開始新行。"
        }
        "Whether to colorize brackets in the editor." => "是否在編輯器中為括號著色。",
        "Number of lines to search for modelines (set to 0 to disable)." => {
            "搜尋 modeline 的行數（設為 0 可停用）。"
        }
        "The unit for image file sizes." => "圖片檔案大小的單位。",
        "Whether to automatically replace emoji shortcodes with emoji characters." => {
            "是否自動將 emoji shortcode 取代為 emoji 字元。"
        }
        "Relative size of the drop target in the editor that will open dropped file as a split pane." => {
            "編輯器中拖放目標的相對大小，拖入檔案後會以分割窗格開啟。"
        }
        "Whether and how to display code lenses from language servers." => {
            "是否以及如何顯示來自語言伺服器的 code lens。"
        }
        "How to render LSP color previews in the editor." => "如何在編輯器中算繪 LSP 色彩預覽。",
        "Whether to use language servers to provide code intelligence." => {
            "是否使用語言伺服器提供程式碼智慧功能。"
        }
        "The list of language servers to use (or disable) for this language." => {
            "此語言要使用（或停用）的語言伺服器清單。"
        }
        "Whether to perform linked edits of associated ranges, if the LS supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well." => {
            "如果語言伺服器支援，是否對關聯範圍執行連結編輯。例如編輯開頭 <html> 標籤時，結尾 </html> 標籤的內容也會一併編輯。"
        }
        "Whether to follow-up empty Go to definition responses from the language server." => {
            "是否對語言伺服器回傳的空白「前往定義」回應進行後續處理。"
        }
        "How to scroll the target into view when navigating to a definition or reference." => {
            "導覽至定義或參考時，如何將目標捲動到檢視中。"
        }
        "When enabled, use folding ranges from the language server instead of indent-based folding." => {
            "啟用時，使用語言伺服器提供的摺疊範圍，而不是以縮排為基礎的摺疊。"
        }
        "When enabled, use the language server's document symbols for outlines and breadcrumbs instead of tree-sitter." => {
            "啟用時，outline 與階層連結使用語言伺服器的文件符號，而不是 tree-sitter。"
        }
        "Whether to fetch LSP completions or not." => "是否擷取 LSP 自動完成。",
        "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)." => {
            "擷取 LSP 自動完成時，決定等待特定伺服器回應的時間（設為 0 會無限等待）。"
        }
        "Controls how LSP completions are inserted." => "控制 LSP 自動完成的插入方式。",
        "Enables or disables formatting with Prettier for a given language." => {
            "針對指定語言啟用或停用 Prettier 格式化。"
        }
        "Forces Prettier integration to use a specific parser name when formatting files with the language." => {
            "強制 Prettier 整合在格式化此語言檔案時使用特定 parser 名稱。"
        }
        "Forces Prettier integration to use specific plugins when formatting files with the language." => {
            "強制 Prettier 整合在格式化此語言檔案時使用特定外掛。"
        }
        "Default Prettier options, in the format as in package.json section for Prettier." => {
            "預設 Prettier 選項，格式與 package.json 中的 Prettier 區段相同。"
        }
        "Test your microphone and speaker setup" => "測試你的麥克風與喇叭設定",
        "View and manage agent skills installed globally or in project worktrees." => {
            "檢視並管理全域或專案 worktree 中安裝的代理技能。"
        }
        "Set up regex patterns to auto-allow, auto-deny, or always request confirmation, for specific tool inputs." => {
            "針對特定工具輸入設定 regex pattern，以自動允許、自動拒絕，或一律要求確認。"
        }
        "Restricted Mode" => "受限模式",
        "You're in Restricted Mode" => "你正處於受限模式",
        "Mark this project as trusted and unlock all features" => {
            "將此專案標記為信任並解鎖所有功能"
        }
        "Restricted Mode prevents:" => "受限模式會防止：",
        "Review .zed/settings.json for any extensions or commands configured by this project." => {
            "請檢查 .zed/settings.json 中此專案設定的任何延伸模組或命令。"
        }
        "Unrecognized Project" => "無法識別的專案",
        "Unrecognized Projects" => "無法識別的專案",
        "Untrusted projects are opened in Restricted Mode to protect your system." => {
            "未信任的專案會以受限模式開啟，以保護你的系統。"
        }
        "Trust all single files" => "信任所有單一檔案",
        "Trust all projects in the {folder} folder" => "信任 {folder} 資料夾中的所有專案",
        "Trust all projects in the parent folders" => "信任父資料夾中的所有專案",
        "Project settings from being applied" => "套用專案設定",
        "Language servers from running" => "語言伺服器執行",
        "MCP Server integrations from installing" => "安裝 MCP 伺服器整合",
        "Stay in Restricted Mode" => "保持受限模式",
        "Trust and Continue" => "信任並繼續",
        "Disconnected" => "已中斷連線",
        "Your connection to the remote project has been lost." => "與遠端專案的連線已中斷。",
        "Your connection to {server} has been lost due to the server {reason}." => {
            "與 {server} 的連線已中斷，原因是伺服器{reason}。"
        }
        "Unsaved changes are stored locally." => "未儲存的變更已儲存在本機。",
        "process exiting unexpectedly" => "程序非預期結束",
        "not responding" => "沒有回應",
        "Failed to connect over SSH" => "無法透過 SSH 連線",
        "Failed to connect to WSL" => "無法連線至 WSL",
        "Failed to connect to Dev Container" => "無法連線至 Dev Container",
        "Failed to connect to mock server" => "無法連線至 mock server",
        "Start Dev Container" => "啟動 Dev Container",
        "Open devcontainer.json" => "開啟 devcontainer.json",
        "View Server Options" => "檢視伺服器選項",
        "Delete Remote Project" => "刪除遠端專案",
        "Error Creating Dev Container:" => "建立 Dev Container 時發生錯誤：",
        "Open Zed Log" => "開啟 Zed 日誌",
        "Exit" => "離開",
        "Enter the command you use to SSH into this server." => "輸入你用來 SSH 進此伺服器的命令。",
        "Connect SSH Server" => "連線至 SSH 伺服器",
        "Connect Dev Container" => "連線至 Dev Container",
        "No remote servers registered yet." => "尚未註冊遠端伺服器。",
        "Create Branch" => "建立分支",
        "Stash" => "貯藏",
        "Branch & Stash" => "分支與貯藏",
        "Restart to update Zed" => "重新啟動以更新 Zed",
        "Open Application Menu" => "開啟應用程式選單",
        "New..." => "新增...",
        "Leave Call" => "離開通話",
        "Unmute Microphone" => "取消麥克風靜音",
        "Mute Microphone" => "麥克風靜音",
        "Audio will be unmuted" => "音訊將取消靜音",
        "Share" => "分享",
        "Unshare" => "停止分享",
        "Stop sharing project with call participants" => "停止與通話參與者分享專案",
        "Share project with call participants" => "與通話參與者分享專案",
        "This project may not be shared in a public channel." => "此專案不可在公開頻道中分享。",
        "Search threads…" => "搜尋對話串…",
        "Search all threads…" => "搜尋所有對話串…",
        "Cancel Restore" => "取消還原",
        "Import Threads" => "匯入對話串",
        "Choose from Local Folders" => "從本機資料夾選擇",
        "Select" => "選取",
        "No threads yet" => "尚無對話串",
        "Archive Thread" => "封存對話串",
        "Remote Project" => "遠端專案",
        "Close Worktree" => "關閉工作樹",
        "Stop Generation" => "停止產生",
        "Discard Draft" => "捨棄草稿",
        "Clear Search" => "清除搜尋",
        "Toggle Sidebar" => "切換側邊欄",
        "Focus Sidebar" => "聚焦側邊欄",
        "View Theme Docs" => "檢視佈景主題文件",
        "Install Themes" => "安裝佈景主題",
        "View Icon Theme Docs" => "檢視圖示佈景主題文件",
        "Install Icon Themes" => "安裝圖示佈景主題",
        "Reconnect" => "重新連線",
        "Add Local Folders" => "新增本機資料夾",
        "Add Remote Folder" => "新增遠端資料夾",
        "Open Local Folders" => "開啟本機資料夾",
        "Open Remote Folder" => "開啟遠端資料夾",
        "Actions" => "動作",
        "Activate" => "啟用",
        "Remove Folder" => "移除資料夾",
        "Remove from Window" => "從視窗移除",
        "Delete from Recent Projects" => "從最近專案移除",
        "Remove Folder from Project" => "從專案移除資料夾",
        "Remove Project from Window" => "從視窗移除專案",
        "Add a nickname for this server" => "為此伺服器新增暱稱",
        "As a multi-root folder" => "作為多根資料夾",
        "Add Folder to this Project" => "將資料夾新增至此專案",
        "Get Started" => "開始使用",
        "Open Project" => "開啟專案",
        "Open Recent Project" => "開啟最近專案",
        "Clone Repository" => "複製儲存庫",
        "Open Command Palette" => "開啟命令選擇區",
        "Customize Keymaps" => "自訂按鍵對應",
        "Explore Extensions" => "探索延伸模組",
        "Search projects…" => "搜尋專案…",
        "Recent Projects" => "最近專案",
        "Welcome back to Zed" => "歡迎回到 Zed",
        "Welcome to Zed" => "歡迎使用 Zed",
        "WSL:" => "WSL：",
        "Dev Containers" => "Dev Containers",
        "Creating Dev Container" => "正在建立 Dev Container",
        "Remove Distro" => "移除發行版",
        "Copy Server Address" => "複製伺服器位址",
        "Remove Server" => "移除伺服器",
        "Add WSL Distro" => "新增 WSL 發行版",
        "Custom" => "自訂",
        "The editor for what's next" => "面向下一步的編輯器",
        "Return to Onboarding" => "返回新手導覽",
        "Create a Skill" => "建立技能",
        "Delete Skill" => "刪除技能",
        "Clear Filter" => "清除篩選",
        "Pin Active Outline" => "釘選作用中的大綱",
        "Unpin Outline" => "取消釘選大綱",
        "Unpin Active Outline" => "取消釘選作用中的大綱",
        "Find in Folder…" => "在資料夾中尋找…",
        "Unfold Directory" => "展開目錄",
        "Fold Directory" => "摺疊目錄",
        "Compare Marked Files" => "比較標記的檔案",
        "Duplicate" => "製作複本",
        "Download..." => "下載...",
        "Copy Path" => "複製路徑",
        "Copy Relative Path" => "複製相對路徑",
        "Restore File" => "還原檔案",
        "Add to .gitignore" => "新增至 .gitignore",
        "View History" => "檢視歷程記錄",
        "Trash" => "丟到垃圾桶",
        "Collapse All" => "全部摺疊",
        "Change Mode" => "變更模式",
        "Cycle Through Modes" => "循環切換模式",
        "Change Model" => "變更模型",
        "Cycle Favorited Models" => "循環切換我的最愛模型",
        "Active Provider" => "作用中的提供者",
        "No global skills installed." => "尚未安裝全域技能。",
        "No project skills found." => "找不到專案技能。",
        "Upgrade to Zed Pro" => "升級至 Zed Pro",
        "Current Plan" => "目前方案",
        "Free" => "免費",
        "Pro" => "Pro",
        "Commit" => "提交",
        "Version" => "版本",
        "Channel Notes" => "頻道筆記",
        "Create file: {path}" => "建立檔案：{path}",
        "Discard changes to {file}?" => "要捨棄 {file} 的變更嗎？",
        "Failed to restore {file}: {error}" => "無法還原 {file}：{error}",
        "Failed to add to .gitignore: {error}" => "無法新增至 .gitignore：{error}",
        "Do you want to trash {path}?" => "要將 {path} 丟到垃圾桶嗎？",
        "Are you sure you want to permanently delete {path}?" => "確定要永久刪除 {path} 嗎？",
        "It has unsaved changes, which will be lost." => "它有未儲存的變更，這些變更將會遺失。",
        ".. 1 file not shown" => ".. 尚有 1 個檔案未顯示",
        ".. {count} files not shown" => ".. 尚有 {count} 個檔案未顯示",
        "1 of these has unsaved changes, which will be lost." => {
            "其中 1 個項目有未儲存的變更，這些變更將會遺失。"
        }
        "{count} of these have unsaved changes, which will be lost." => {
            "其中 {count} 個項目有未儲存的變更，這些變更將會遺失。"
        }
        "Do you want to trash the following {count} files?" => {
            "要將以下 {count} 個檔案丟到垃圾桶嗎？"
        }
        "Are you sure you want to permanently delete the following {count} files?" => {
            "確定要永久刪除以下 {count} 個檔案嗎？"
        }
        "This cannot be undone." => "此動作無法復原。",
        "Downloading {current}/{total} files..." => "正在下載 {current}/{total} 個檔案...",
        "Downloaded {total} files" => "已下載 {total} 個檔案",
        "A file or folder with name {name} already exists in the destination folder. Do you want to replace it?" => {
            "目的地資料夾中已有名為 {name} 的檔案或資料夾。要取代它嗎？"
        }
        "{count} entries" => "{count} 個項目",
        "Do you want to save changes to the following files?" => "要儲存以下檔案的變更嗎？",
        "Unable to save file: {error}" => "無法儲存檔案：{error}",
        "This buffer" => "此緩衝區",
        "This file has changed on disk since you started editing it. Do you want to overwrite it?" => {
            "此檔案在你開始編輯後已於磁碟上變更。要覆蓋它嗎？"
        }
        "This file has been deleted on disk since you started editing it. Do you want to recreate it?" => {
            "此檔案在你開始編輯後已從磁碟刪除。要重新建立它嗎？"
        }
        "{path} contains unsaved edits. Do you want to save it?" => {
            "{path} 包含未儲存的編輯。要儲存嗎？"
        }
        "Project search buffer contains unsaved edits. Do you want to save it?" => {
            "專案搜尋緩衝區包含未儲存的編輯。要儲存嗎？"
        }
        "{worktree} contains a Dev Container configuration file. Would you like to re-open it in a container?" => {
            "{worktree} 包含 Dev Container 設定檔。要在容器中重新開啟嗎？"
        }
        "Yes, Open in Container" => "是，在容器中開啟",
        "Please sign in to continue." => "請登入以繼續。",
        "You are running an unsupported version of Zed. Please update to continue." => {
            "你正在執行不支援的 Zed 版本。請更新以繼續。"
        }
        "No matching channel was found. Please check the link and try again." => {
            "找不到相符的頻道。請檢查連結後再試一次。"
        }
        "This channel is private, and you do not have access. Please ask someone to add you and try again." => {
            "此頻道是私人頻道，你沒有存取權。請其他人加入你後再試一次。"
        }
        "Please check your internet connection and try again." => "請檢查網際網路連線後再試一次。",
        "Please try again." => "請再試一次。",
        "Failed to join channel" => "無法加入頻道",
        "Are you sure you want to restart?" => "確定要重新啟動嗎？",
        "Moving Zed to Applications" => "正在將 Zed 移至「應用程式」",
        "Zed will reopen when installation is complete." => "安裝完成後 Zed 將重新開啟。",
        "Installing Zed…" => "正在安裝 Zed…",
        "Move Zed to Applications?" => "要將 Zed 移至「應用程式」嗎？",
        "Zed is running from a temporary location. Move it to Applications to finish installing it." => {
            "Zed 正從暫存位置執行。請將它移至「應用程式」以完成安裝。"
        }
        "Zoom" => "縮放",
        "Keymap" => "按鍵對應",
        "Keymap Editor" => "鍵盤快速鍵編輯器",
        "Edit Keybindings" => "編輯鍵盤快速鍵",
        "Customize keybindings in the keymap editor." => "在鍵盤快速鍵編輯器中自訂鍵盤快速鍵。",
        "Base Keymap" => "基底按鍵對應",
        "The name of a base set of key bindings to use." => "要使用的基底按鍵對應組名稱。",
        "Modal Editing" => "模式編輯",
        "Vim Mode" => "Vim 模式",
        "Enable Vim mode and key bindings." => "啟用 Vim 模式與鍵盤快速鍵。",
        "Coming from Neovim? Use our first-class implementation of Vim Mode" => {
            "從 Neovim 轉來？使用我們第一流的 Vim 模式實作"
        }
        "Which-key Menu" => "Which-key 選單",
        "Show Which-key Menu" => "顯示 Which-key 選單",
        "Search settings…" => "搜尋設定…",
        "Zed — Settings" => "Zed — 設定",
        "View Other Projects" => "檢視其他專案",
        "Edit in settings.json" => "在 settings.json 中編輯",
        "Focus Content" => "聚焦內容",
        "Focus Navbar" => "聚焦導覽列",
        "Filter action names…" => "篩選動作名稱…",
        "Create" => "建立",
        "Copy Action" => "複製動作",
        "Copy Context" => "複製內容條件",
        "Show Matching Keybindings" => "顯示相符的鍵盤快速鍵",
        "This action is unbound" => "此動作尚未繫結",
        "View conflicts" => "檢視衝突",
        "Use alt+click to show all conflicts" => "使用 Alt+按一下顯示所有衝突",
        "Edit this binding" => "編輯此繫結",
        "This binding is overridden by other bindings." => "此繫結已被其他繫結覆寫。",
        "Show matching keybinds" => "顯示相符的鍵盤快速鍵",
        "This binding is overridden by other bindings.\nUse alt+click to edit this binding" => {
            "此繫結已被其他繫結覆寫。\n使用 Alt+按一下編輯此繫結"
        }
        "No conflicting keybinds found that match the provided query" => {
            "找不到符合查詢的衝突鍵盤快速鍵"
        }
        "No conflicting keybinds found" => "找不到衝突的鍵盤快速鍵",
        "No keybinds found matching the entered keystrokes" => "找不到符合輸入按鍵的鍵盤快速鍵",
        "No matches found for the provided query" => "找不到符合查詢的相符項目",
        "Filters" => "篩選器",
        "Conflicts" => "衝突",
        "No Action" => "無動作",
        "Categories" => "類別",
        "User" => "使用者",
        "Default" => "預設",
        "Vim" => "Vim",
        "Toggle Exact Match Mode" => "切換完全相符模式",
        "Search by Keystrokes" => "依按鍵搜尋",
        "Edit in JSON" => "以 JSON 編輯",
        "Create Keybinding" => "建立鍵盤快速鍵",
        "Edit Keybinding" => "編輯鍵盤快速鍵",
        "Edit Keystroke" => "編輯按鍵",
        "Edit Arguments" => "編輯引數",
        "Action" => "動作",
        "Arguments" => "引數",
        "Keystrokes" => "按鍵",
        "Source" => "來源",
        "Type an action name" => "輸入動作名稱",
        "Keybinding Context" => "鍵盤快速鍵內容條件",
        "Edit Context" => "編輯內容條件",
        "Action Arguments" => "動作引數",
        "Action name is required" => "必須輸入動作名稱",
        "Action '{action}' not found" => "找不到動作「{action}」",
        "Failed to parse action arguments as JSON" => "無法將動作引數剖析為 JSON",
        "Failed to validate action arguments" => "無法驗證動作引數",
        "Keystrokes cannot be empty" => "按鍵不可為空",
        "Failed to parse key context" => "無法剖析按鍵內容條件",
        "Your keybind would conflict with the \"{action}\" action and {count} other bindings" => {
            "你的鍵盤快速鍵會與「{action}」動作及其他 {count} 個繫結衝突"
        }
        "Your keybind would conflict with the \"{action}\" action" => {
            "你的鍵盤快速鍵會與「{action}」動作衝突"
        }
        "Your keybind would conflict with other actions" => "你的鍵盤快速鍵會與其他動作衝突",
        "Saved edits to the {action} action." => "已儲存「{action}」動作的編輯。",
        "your keymap" => "你的按鍵對應",
        "the vim keymap" => "Vim 按鍵對應",
        "your base keymap" => "你的基底按鍵對應",
        "This keybinding is overridden by the '{action}' binding from {source}." => {
            "此鍵盤快速鍵已被 {source} 中的「{action}」繫結覆寫。"
        }
        "This binding is overridden." => "此繫結已被覆寫。",
        "binding has the same keystrokes." => "個繫結使用相同按鍵。",
        "bindings have the same keystrokes." => "個繫結使用相同按鍵。",
        "Start Searching" => "開始搜尋",
        "Stop Searching" => "停止搜尋",
        "Start Recording" => "開始錄製",
        "Stop Recording" => "停止錄製",
        "Clear Keystrokes" => "清除按鍵",
        "Hit it three times to execute" => "連按三次即可執行",
        "REC" => "錄製",
        "SEARCH" => "搜尋",
        "Close Other Tabs" => "關閉其他索引標籤",
        "Move Tab to New Window" => "將索引標籤移至新視窗",
        "Show All Tabs" => "顯示所有索引標籤",
        "Keyboard Context" => "鍵盤情境",
        "This view lets you determine the current context stack for creating custom key bindings in Zed. When a keyboard shortcut is triggered, it also shows all the possible contexts it could have triggered in, and which one matched." => {
            "此檢視可協助判斷目前的內容條件堆疊，以便在 Zed 中建立自訂鍵盤快速鍵。觸發鍵盤快速鍵時，也會顯示所有可能觸發的內容條件，以及實際相符的項目。"
        }
        "Open Documentation" => "開啟文件",
        "View Default Keymap" => "檢視預設按鍵對應",
        "Edit Keymap File" => "編輯按鍵對應檔",
        "Current Context Stack" => "目前情境堆疊",
        "Last Keystroke" => "上一個按鍵",
        "Waiting for more input: {keys}" => "等待更多輸入：{keys}",
        "Typed: {keys}" => "已輸入：{keys}",
        "(match)" => "(相符)",
        "(low precedence)" => "(低優先順序)",
        "(no match)" => "(不相符)",
        "Key Equivalents" => "按鍵等效項",
        "Shortcuts defined using some characters have been remapped so that shortcuts can be typed without holding option." => {
            "使用部分字元定義的快速鍵已重新對應，因此不必按住 Option 也能輸入快速鍵。"
        }
        "View Message" => "檢視訊息",
        "View Logs" => "檢視記錄",
        "Restart Server" => "重新啟動伺服器",
        "Stop Server" => "停止伺服器",
        "Stage Hunk" => "暫存變更區塊",
        "Unstage Hunk" => "取消暫存變更區塊",
        "Restore Hunk" => "還原變更區塊",
        "Next Hunk" => "下一個變更區塊",
        "Previous Hunk" => "上一個變更區塊",
        "Learn more" => "了解更多",
        "You may need to configure git for Github." => "你可能需要設定 GitHub 的 Git 認證。",
        "Switch Branch" => "切換分支",
        "Commit message title exceeds {max_title_length}-character limit." => {
            "提交訊息標題超過 {max_title_length} 個字元限制。"
        }
        "Output Limit Reached" => "已達輸出限制",
        "The model stopped because it reached its maximum output length. You can ask it to continue where it left off." => {
            "模型因達到最大輸出長度而停止。你可以要求它從中斷處繼續。"
        }
        "No Model Selected" => "尚未選取模型",
        "Select a model from the model picker below to get started." => {
            "請從下方模型選擇器選取模型以開始。"
        }
        "API Error" => "API 錯誤",
        "Rate Limit Reached" => "已達速率限制",
        "{provider}'s rate limit was reached. Zed will retry automatically. You can also wait a moment and try again." => {
            "{provider} 已達速率限制。Zed 會自動重試。你也可以稍候再試一次。"
        }
        "Provider Unavailable" => "提供者無法使用",
        "{provider}'s servers are temporarily unavailable. Zed will retry automatically. If the problem persists, check the provider's status page." => {
            "{provider} 的伺服器暫時無法使用。Zed 會自動重試。若問題持續發生，請查看提供者的狀態頁。"
        }
        "API Key Missing" => "缺少 API 金鑰",
        "No API key is configured for {provider}. Add your key via the Agent Panel settings to continue." => {
            "尚未設定 {provider} 的 API 金鑰。請透過代理面板設定新增金鑰以繼續。"
        }
        "Connection Interrupted" => "連線已中斷",
        "The connection to {provider}'s API was interrupted. Zed will retry automatically. If the problem persists, check your network connection." => {
            "與 {provider} API 的連線已中斷。Zed 會自動重試。若問題持續發生，請檢查網路連線。"
        }
        "Invalid API Key" => "API 金鑰無效",
        "The API key for {provider} is invalid or has expired. Update your key via the Agent Panel settings to continue." => {
            "{provider} 的 API 金鑰無效或已過期。請透過代理面板設定更新金鑰以繼續。"
        }
        "Permission Denied" => "權限遭拒",
        "{provider}'s API rejected the request due to insufficient permissions. Check that your API key has access to this model." => {
            "{provider} 的 API 因權限不足而拒絕此要求。請確認你的 API 金鑰可存取此模型。"
        }
        "Request Failed" => "要求失敗",
        "The request could not be completed after multiple attempts. Try again in a moment." => {
            "多次嘗試後仍無法完成要求。請稍候再試。"
        }
        "{provider}'s API returned an unexpected error. If the problem persists, try switching models or restarting Zed." => {
            "{provider} 的 API 傳回未預期的錯誤。若問題持續發生，請嘗試切換模型或重新啟動 Zed。"
        }
        "{model} refused to respond to this prompt. This can happen when a model believes the prompt violates its content policy or safety guidelines, so rephrasing it can sometimes address the issue." => {
            "{model} 拒絕回應此提示。當模型判斷提示可能違反內容政策或安全準則時，可能會發生這種情況；改寫提示有時可以解決。"
        }
        "You reached your free usage limit. Upgrade to Zed Pro for more prompts." => {
            "你已達免費使用額度上限。升級至 Zed Pro 可取得更多提示額度。"
        }
        "This conversation is too long for the model's context window. Start a new thread or remove some attached files to continue." => {
            "這個對話串已超過模型的內容視窗。請開啟新的對話串，或移除部分附加檔案後再繼續。"
        }
        "This agent does not support viewing previous messages. However, your session will still continue from where you last left off." => {
            "此代理不支援檢視先前訊息。不過，你的工作階段仍會從上次離開的地方繼續。"
        }
        "For best performance, run Codex in Windows Subsystem for Linux (WSL2)" => {
            "為了獲得最佳效能，請在 Windows Subsystem for Linux (WSL2) 中執行 Codex"
        }
        "Skill failed to load" => "技能載入失敗",
        "Review before sending" => "送出前請先檢查",
        "This prompt was pre-filled by an external link. Read it carefully before you send it." => {
            "這段提示由外部連結預先填入。送出前請仔細檢查。"
        }
        "This agent only operates on \"{folder}\". Other folders in this workspace are not accessible to it." => {
            "此代理只會在「{folder}」中運作，無法存取此工作區中的其他資料夾。"
        }
        "External Agents currently don't support multi-root workspaces" => {
            "外部代理目前不支援多根目錄工作區"
        }
        "New version available" => "有新版本可用",
        "Agent update available" => "代理更新可用",
        "Update to v{}" => "更新至 v{}",
        "Thread reaching the token limit soon" => "對話串即將達到 token 限制",
        "Thread reached the token limit" => "對話串已達 token 限制",
        "To continue, start a new thread from a summary." => "若要繼續，請從摘要開始新的對話串。",
        "Invalid URL: {error}" => "URL 無效：{error}",
        "Paste a URL to open." => "貼上要開啟的 URL。",
        "Developer" => "開發人員",
        "Feature Flags" => "功能旗標",
        "Instrumentation" => "檢測",
        "Performance Profiler" => "效能分析器",
        "Collect timing data for foreground and background executor tasks so they can be inspected via `zed: open performance profiler`. May lead to increased memory usage." => {
            "收集前景與背景執行器工作的時間資料，以便透過 `zed: open performance profiler` 檢查。可能會增加記憶體使用量。"
        }
        "General" => "一般",
        "When Closing With No Tabs" => "沒有索引標籤時關閉",
        "On Last Window Closed" => "最後一個視窗關閉時",
        "Use System Path Prompts" => "使用系統路徑提示",
        "Use System Prompts" => "使用系統提示",
        "Redact Private Values" => "遮蔽私人值",
        "Private Files" => "私人檔案",
        "CLI Default Open Behavior" => "CLI 預設開啟行為",
        "Security" => "安全性",
        "Trust All Projects By Default" => "預設信任所有專案",
        "Automatically mark all new projects as trusted to unlock all Zed's features" => {
            "自動將所有新專案標記為受信任，以解鎖 Zed 的所有功能"
        }
        "Zed can only allow services like language servers, project settings, and MCP servers to run after you mark a new project as trusted." => {
            "只有在你將新專案標記為受信任後，Zed 才能允許語言伺服器、專案設定與 MCP 伺服器等服務執行。"
        }
        "Failed to load your settings. Some values may be incorrect and changes may be lost." => {
            "無法載入你的設定。部分值可能不正確，變更也可能遺失。"
        }
        "Fix in settings.json" => "在 settings.json 中修正",
        "Your settings are out of date, and need to be updated." => "你的設定已過期，需要更新。",
        "They can be automatically migrated to the latest version." => {
            "它們可以自動遷移至最新版本。"
        }
        "They must be manually migrated to the latest version." => "它們必須手動遷移至最新版本。",
        "Your settings file is out of date, automatic migration failed" => {
            "你的設定檔已過期，自動遷移失敗"
        }
        "This project is in restricted mode. Some project settings may not apply." => {
            "此專案處於受限模式。部分專案設定可能不會套用。"
        }
        "Manage Trust" => "管理信任",
        "Workspace Restoration" => "工作區還原",
        "Restore Unsaved Buffers" => "還原未儲存的緩衝區",
        "Restore On Startup" => "啟動時還原",
        "Privacy" => "隱私權",
        "Telemetry Diagnostics" => "遙測診斷",
        "Telemetry Metrics" => "遙測指標",
        "Auto Update" => "自動更新",
        "Theme Mode" => "佈景主題模式",
        "Theme Name" => "佈景主題名稱",
        "Mode" => "模式",
        "Light Theme" => "淺色佈景主題",
        "Dark Theme" => "深色佈景主題",
        "Icon Theme" => "圖示佈景主題",
        "Icon Theme Name" => "圖示佈景主題名稱",
        "Light Icon Theme" => "淺色圖示佈景主題",
        "Dark Icon Theme" => "深色圖示佈景主題",
        "Buffer Font" => "緩衝區字型",
        "UI Font" => "UI 字型",
        "Agent Panel Font" => "代理面板字型",
        "Text Rendering" => "文字轉譯",
        "Font Family" => "字型系列",
        "Font Size" => "字型大小",
        "Font Weight" => "字重",
        "Line Height" => "行高",
        "Custom Line Height" => "自訂行高",
        "Font Features" => "字型功能",
        "Font Fallbacks" => "字型後援",
        "UI Font Size" => "UI 字型大小",
        "Buffer Font Size" => "緩衝區字型大小",
        "Cursor" => "游標",
        "Multi Cursor Modifier" => "多重游標輔助鍵",
        "Cursor Blink" => "游標閃爍",
        "Cursor Shape" => "游標形狀",
        "Hide Mouse" => "隱藏滑鼠",
        "Highlighting" => "醒目提示",
        "Current Line Highlight" => "目前行醒目提示",
        "Selection Highlight" => "選取範圍醒目提示",
        "Rounded Selection" => "圓角選取範圍",
        "Guides" => "輔助線",
        "Indentation" => "縮排",
        "Wrapping" => "自動換行",
        "Formatting" => "格式化",
        "Completions" => "自動完成",
        "Inlay Hints" => "行內提示",
        "Buffer Search" => "緩衝區搜尋",
        "No Code Actions Available" => "沒有可用的程式碼動作",
        "Selection Controls" => "選取範圍控制項",
        "Go to Symbol" => "移至符號",
        "Go to Line/Column" => "移至行/欄",
        "Editor Controls" => "編輯器控制項",
        "Inline Values" => "行內值",
        "Semantic Highlights" => "語意醒目提示",
        "You can't toggle edit predictions for this file as it is within the excluded files list." => {
            "此檔案在排除檔案清單中，無法切換編輯預測。"
        }
        "Inline diagnostics are not available until regular diagnostics are enabled." => {
            "必須先啟用一般診斷，才能使用行內診斷。"
        }
        "Line Numbers" => "行號",
        "Selection Menu" => "選取範圍選單",
        "Column Git Blame" => "欄位 Git Blame",
        "Backup and Update" => "備份並更新",
        "No telemetry events recorded yet" => "尚未記錄遙測事件",
        "No events match the current filter" => "沒有符合目前篩選條件的事件",
        "Filter events..." => "篩選事件...",
        "Clear Events" => "清除事件",
        "Open Raw Log File" => "開啟原始記錄檔",
        "Interrupt" => "中斷",
        "Clear Outputs" => "清除輸出",
        "Shut Down Kernel" => "關閉 Kernel",
        "Restart Kernel" => "重新啟動 Kernel",
        "View Sessions" => "檢視工作階段",
        "REPL Menu" => "REPL 選單",
        "Select Kernel" => "選取 Kernel",
        "Not attached to an editor" => "未附加至編輯器",
        "No highlights found" => "找不到醒目提示",
        "Focus an editor to show highlights" => "聚焦編輯器以顯示醒目提示",
        "Highlights Settings" => "醒目提示設定",
        "Project is in Restricted Mode" => "專案處於受限模式",
        "Language Servers can't run until you trust this project." => {
            "信任此專案後，語言伺服器才能執行。"
        }
        "Search channels…" => "搜尋頻道…",
        "Follow {user}" => "跟隨 {user}",
        "Calling" => "通話中",
        "Guest" => "訪客",
        "Mic only" => "僅麥克風",
        "Click to Follow" => "按一下以跟隨",
        "Open {project}" => "開啟 {project}",
        "Screen" => "螢幕",
        "Open Shared Screen" => "開啟分享畫面",
        "notes" => "筆記",
        "Open Channel Notes" => "開啟頻道筆記",
        "Grant Mic Access" => "授予麥克風權限",
        "Grant Write Access" => "授予寫入權限",
        "Mute" => "靜音",
        "Revoke Access" => "撤銷權限",
        "Expand Subchannels" => "展開子頻道",
        "Collapse Subchannels" => "摺疊子頻道",
        "Open Notes" => "開啟筆記",
        "Copy Channel Link" => "複製頻道連結",
        "Copy Channel Notes Link" => "複製頻道筆記連結",
        "Remove from Favorites" => "從我的最愛移除",
        "Add to Favorites" => "加入我的最愛",
        "New Subchannel" => "新增子頻道",
        "Move '#{channel}' here" => "將「#{channel}」移到這裡",
        "Manage Members" => "管理成員",
        "Move this channel" => "移動此頻道",
        "Make Channel Private" => "設為私人頻道",
        "Make Channel Public" => "設為公開頻道",
        "Leave Channel" => "離開頻道",
        "Invite {user} to join" => "邀請 {user} 加入",
        "Call {user}" => "呼叫 {user}",
        "Remove Contact" => "移除聯絡人",
        "Are you sure you want to leave \"#{channel}\"?" => "確定要離開「#{channel}」嗎？",
        "Leave" => "離開",
        "Are you sure you want to remove the channel \"{channel}\"?" => {
            "確定要移除頻道「{channel}」嗎？"
        }
        "Are you sure you want to remove \"{user}\" from your contacts?" => {
            "確定要從聯絡人中移除「{user}」嗎？"
        }
        "Connecting…" => "正在連線…",
        "Connect" => "連線",
        "Sign In with GitHub" => "使用 GitHub 登入",
        "Copy public channel link." => "複製公開頻道連結。",
        "Copy private channel link." => "複製私人頻道連結。",
        "Current Call" => "目前通話",
        "Requests" => "邀請",
        "Contacts" => "聯絡人",
        "Channels" => "頻道",
        "Invites" => "邀請",
        "Online" => "線上",
        "Offline" => "離線",
        "Stop Auto Watching Screens" => "停止自動觀看螢幕",
        "Auto Watch Screens" => "自動觀看螢幕",
        "Auto Watch Screens (paused while sharing)" => "自動觀看螢幕（分享時暫停）",
        "Search for new contact" => "搜尋新聯絡人",
        "Show All Channels" => "顯示所有頻道",
        "Show Occupied Channels" => "顯示有人使用的頻道",
        "Create Channel" => "建立頻道",
        "{user} is offline" => "{user} 離線",
        "{user} is on a call" => "{user} 通話中",
        "Invite {user} to join call" => "邀請 {user} 加入通話",
        "Decline invite" => "拒絕邀請",
        "Accept invite" => "接受邀請",
        "Cancel invite" => "取消邀請",
        "Add a Contact" => "新增聯絡人",
        "Join Channel" => "加入頻道",
        "Accept" => "接受",
        "Decline" => "拒絕",
        "Public" => "公開",
        "Invite Members" => "邀請成員",
        "Invited" => "已邀請",
        "Admin" => "管理員",
        "You" => "你",
        "Member" => "成員",
        "Invite new contacts" => "邀請新聯絡人",
        "is sharing a project in Zed" => "正在 Zed 中分享專案",
        "{user} is sharing a project with you{punctuation}" => {
            "{user} 正在與你分享專案{punctuation}"
        }
        "Call Diagnostics" => "通話診斷",
        "Not in a call" => "不在通話中",
        "Network" => "網路",
        "Excellent" => "極佳",
        "Good" => "良好",
        "Poor" => "不佳",
        "Lost" => "已中斷",
        "Normal" => "正常",
        "High" => "偏高",
        "Latency" => "延遲",
        "Time for data to travel to the server" => "資料傳送到伺服器所需時間",
        "Jitter" => "抖動",
        "Variance or fluctuation in latency" => "延遲的變異或波動",
        "Packet loss" => "封包遺失",
        "Amount of data lost during transfer" => "傳輸期間遺失的資料量",
        "Input lag" => "輸入延遲",
        "Delay from audio capture to WebRTC" => "從音訊擷取到 WebRTC 的延遲",
        "Close output area" => "關閉輸出區域",
        "Starting" => "正在啟動",
        "Error:" => "錯誤：",
        "Shutting Down" => "正在關閉",
        "Shutdown" => "已關閉",
        "Restarting" => "正在重新啟動",
        "No Jupyter Kernels Available" => "沒有可用的 Jupyter Kernel",
        "To start interactively running code in your editor, you need to install and configure Jupyter kernels." => {
            "若要在編輯器中互動執行程式碼，你需要安裝並設定 Jupyter Kernel。"
        }
        "Install Kernels" => "安裝 Kernel",
        "Copy Output" => "複製輸出",
        "Open in Buffer" => "在緩衝區中開啟",
        "Open Full Error in Buffer" => "在緩衝區中開啟完整錯誤",
        "Type here and press Enter" => "在這裡輸入並按 Enter",
        "Connecting to kernel..." => "正在連線至 Kernel...",
        "Executing..." => "正在執行...",
        "Unknown status" => "未知狀態",
        "Kernel shutting down..." => "Kernel 正在關閉...",
        "Kernel restarting..." => "Kernel 正在重新啟動...",
        "Kernel shutdown" => "Kernel 已關閉",
        "Queued..." => "已排入佇列...",
        "Recommended" => "建議",
        "ipykernel not installed" => "尚未安裝 ipykernel",
        "Kernel Docs" => "Kernel 文件",
        "More options" => "更多選項",
        "Select Feature" => "選取功能",
        "Confirm Selections" => "確認選取項目",
        "Create Dev Container" => "建立 Dev Container",
        "Search for Dev Container Templates" => "搜尋 Dev Container 範本",
        "Template Option: " => "範本選項：",
        "Overwrite Existing Configuration?" => "要覆蓋現有設定嗎？",
        "Querying template registry..." => "正在查詢範本登錄檔...",
        "Querying features..." => "正在查詢功能...",
        "Download" => "下載",
        "Select Toolchain Path" => "選取工具鏈路徑",
        "Finish Setup" => "完成設定",
        "Agent Setup" => "代理設定",
        "Import Settings" => "匯入設定",
        "Automatically pull your settings from other editors" => "自動從其他編輯器匯入你的設定",
        "Help improve Zed by sending anonymous usage data" => "傳送匿名使用資料以協助改進 Zed",
        "Help fix Zed by sending crash reports so we can fix critical issues fast" => {
            "傳送當機報告以協助 Zed 快速修正重大問題"
        }
        "Signing In…" => "正在登入…",
        "Install your favorite agents and start your first thread." => {
            "安裝你偏好的代理，並開始第一個對話串。"
        }
        "Edit and save files directly in the results multibuffer!" => {
            "直接在結果 Multibuffer 中編輯並儲存檔案！"
        }
        "Dismiss Hint" => "關閉提示",
        "Miscellaneous" => "其他",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_traditional_chinese_locale_tags() {
        assert!(is_traditional_chinese_locale("zh-Hant"));
        assert!(is_traditional_chinese_locale("zh-TW"));
        assert!(is_traditional_chinese_locale("zh_Hant_TW"));
        assert!(!is_traditional_chinese_locale("zh-CN"));
        assert!(!is_traditional_chinese_locale("en-US"));
    }

    #[test]
    fn default_personal_fork_locale_returns_traditional_chinese_text() {
        assert_eq!(text("Command Palette"), "命令選擇區");
        assert_eq!(text("Settings"), "設定");
        assert_eq!(text("Project Panel"), "專案面板");
    }

    #[test]
    fn untranslated_phrases_fall_back_to_english() {
        assert_eq!(text("Not translated yet"), "Not translated yet");
        assert_eq!(
            text_for_locale("zh-Hant", "Not translated yet"),
            "Not translated yet"
        );
    }

    #[test]
    fn unsupported_locale_returns_english_text() {
        assert_eq!(
            text_for_locale("en-US", "Command Palette"),
            "Command Palette"
        );
        assert_eq!(text_for_locale("zh-CN", "Settings"), "Settings");
    }

    #[test]
    fn first_slice_translates_visible_action_labels() {
        assert_eq!(text("Create a Skill"), "建立技能");
        assert_eq!(text("Delete Skill"), "刪除技能");
        assert_eq!(text("Open in Terminal"), "在終端機中開啟");
        assert_eq!(text("Collapse All"), "全部摺疊");
        assert_eq!(text("Open Settings"), "開啟設定");
        assert_eq!(text("Open Recent..."), "開啟最近使用...");
        assert_eq!(text("Command Palette..."), "命令選擇區...");
        assert_eq!(text("Yes"), "是");
        assert_eq!(text("No"), "否");
        assert_eq!(text("Don't ask me again"), "不要再詢問");
    }

    #[test]
    fn second_slice_translates_high_frequency_developer_surfaces() {
        assert_eq!(text("Search…"), "搜尋…");
        assert_eq!(text("Replace with…"), "取代為…");
        assert_eq!(text("Search all files…"), "搜尋所有檔案…");
        assert_eq!(text("Find in Results"), "在結果中尋找");
        assert_eq!(text("Project Search"), "專案搜尋");
        assert_eq!(text("Search All Files"), "搜尋所有檔案");
        assert_eq!(text("No Results"), "沒有結果");
        assert_eq!(text("Match Case Sensitivity"), "符合大小寫");
        assert_eq!(text("Use Regular Expressions"), "使用規則運算式");
        assert_eq!(text("Toggle Filters"), "切換篩選器");
        assert_eq!(text("Stage All"), "全部暫存");
        assert_eq!(text("Unstage All"), "全部取消暫存");
        assert_eq!(text("View Diff"), "檢視差異");
        assert_eq!(text("No changes to commit"), "沒有可提交的變更");
        assert_eq!(text("Generate Commit Message"), "產生提交訊息");
        assert_eq!(text("New Terminal"), "新增終端機");
        assert_eq!(text("Spawn Task"), "產生工作");
        assert_eq!(text("Rerun Last Task"), "重新執行上一個工作");
        assert_eq!(text("Install Dev Extension"), "安裝開發用延伸模組");
        assert_eq!(text("Not Installed"), "未安裝");
        assert_eq!(text("Search & Files"), "搜尋與檔案");
        assert_eq!(text("Version Control"), "版本控制");
    }

    #[test]
    fn third_slice_translates_command_settings_agent_and_chrome_surfaces() {
        assert_eq!(text("Execute a command..."), "執行命令...");
        assert_eq!(text("workspace: open"), "工作區：開啟");
        assert_eq!(text("editor: go to definition"), "編輯器：移至定義");
        assert_eq!(text("Open Agent Panel"), "開啟代理面板");
        assert_eq!(text("New Thread"), "新增對話串");
        assert_eq!(text("Awaiting Confirmation"), "等待確認");
        assert_eq!(text("Queue and Send"), "加入佇列並傳送");
        assert_eq!(text("Open Thread as Markdown"), "以 Markdown 開啟對話串");
        assert_eq!(text("Configure Providers"), "設定提供者");
        assert_eq!(text("Restricted Mode"), "受限模式");
        assert_eq!(text("Create Branch"), "建立分支");
        assert_eq!(text("Sign In"), "登入");
        assert_eq!(text("Search threads…"), "搜尋對話串…");
        assert_eq!(text("No threads yet"), "尚無對話串");
        assert_eq!(text("View Theme Docs"), "檢視佈景主題文件");
        assert_eq!(text("Open Local Folders"), "開啟本機資料夾");
        assert_eq!(text("Welcome to Zed"), "歡迎使用 Zed");
        assert_eq!(
            text("What to do when using the 'close active item' action with no tabs."),
            "沒有索引標籤時，使用「關閉作用中項目」動作要執行的行為。"
        );
        assert_eq!(
            text("Use native OS dialogs for 'Open' and 'Save As'."),
            "針對「開啟」與「另存新檔」使用作業系統原生對話框。"
        );
        assert_eq!(
            text("Font family for editor text."),
            "編輯器文字的字型系列。"
        );
        assert_eq!(text("Font size for UI elements."), "UI 元素的字型大小。");
    }

    #[test]
    fn milestone_four_glossary_terms_are_guarded() {
        assert_eq!(text("Stash"), "貯藏");
        assert_eq!(text("Default Permission"), "預設權限");
        assert_eq!(text("Default Action"), "預設動作");
        assert_eq!(text("Thinking Effort"), "推理強度");
        assert_eq!(text("Profile"), "代理設定檔");
        assert_eq!(
            text_or_original("workspace: save all").as_ref(),
            "工作區：全部儲存"
        );
        assert_eq!(
            text_or_original("untranslated dynamic action").as_ref(),
            "untranslated dynamic action"
        );
    }

    #[test]
    fn milestone_six_core_workspace_static_terms_are_guarded() {
        assert_eq!(text("Search Inside"), "在此搜尋");
        assert_eq!(text("New Folder"), "新增資料夾");
        assert_eq!(text("Reveal in Finder"), "在 Finder 中顯示");
        assert_eq!(text("Reveal in File Explorer"), "在 File Explorer 中顯示");
        assert_eq!(text("Reveal in File Manager"), "在檔案管理員中顯示");
        assert_eq!(text("Search project files..."), "搜尋專案檔案...");
        assert_eq!(text("Project Scan in Progress…"), "正在掃描專案…");
        assert_eq!(text("Filter Options"), "篩選選項");
        assert_eq!(text("Include Ignored Files"), "包含已忽略檔案");
        assert_eq!(text("Channel Notes"), "頻道筆記");
        assert_eq!(text("Search recent projects…"), "搜尋最近專案…");
        assert_eq!(text("Search projects…"), "搜尋專案…");
        assert_eq!(text("Open Recent Project"), "開啟最近專案");
        assert_eq!(text("Start New Agent Thread"), "開始新的代理對話串");
        assert_eq!(text("Open Project in New Window"), "在新視窗開啟專案");
        assert_eq!(text("Focus Project"), "聚焦專案");
        assert_eq!(text("Open Worktrees"), "開啟工作樹");
        assert_eq!(text("Move Up"), "上移");
        assert_eq!(text("Move Down"), "下移");
        assert_eq!(text("Add Project"), "新增專案");
        assert_eq!(
            text("No threads match your search."),
            "沒有符合搜尋的對話串。"
        );
        assert_eq!(text("Show Thread History"), "顯示對話串歷程");
        assert_eq!(text("Hide Thread History"), "隱藏對話串歷程");
        assert_eq!(text("Unrecognized Project"), "無法識別的專案");
        assert_eq!(text("Add a nickname for this server"), "為此伺服器新增暱稱");
        assert_eq!(text("Creating Dev Container"), "正在建立 Dev Container");
        assert_eq!(text("Copy Server Address"), "複製伺服器位址");
        assert_eq!(text("Add WSL Distro"), "新增 WSL 發行版");
        assert_eq!(text("Hide Button"), "隱藏按鈕");
        assert_eq!(text("Read-Only File"), "唯讀檔案");
        assert_eq!(text("Pin Tab"), "釘選索引標籤");
        assert_eq!(text("Unpin Tab"), "取消釘選索引標籤");
        assert_eq!(text("Reveal In Project Panel"), "在專案面板中顯示");
    }

    #[test]
    fn milestone_six_core_workspace_prompt_templates_are_guarded() {
        assert_eq!(text("Create file: {path}"), "建立檔案：{path}");
        assert_eq!(
            text("Discard changes to {file}?"),
            "要捨棄 {file} 的變更嗎？"
        );
        assert_eq!(
            text("Are you sure you want to permanently delete {path}?"),
            "確定要永久刪除 {path} 嗎？"
        );
        assert_eq!(
            text("Do you want to trash the following {count} files?"),
            "要將以下 {count} 個檔案丟到垃圾桶嗎？"
        );
        assert_eq!(text("This cannot be undone."), "此動作無法復原。");
        assert_eq!(
            text(
                "A file or folder with name {name} already exists in the destination folder. Do you want to replace it?"
            ),
            "目的地資料夾中已有名為 {name} 的檔案或資料夾。要取代它嗎？"
        );
        assert_eq!(
            text("Project search buffer contains unsaved edits. Do you want to save it?"),
            "專案搜尋緩衝區包含未儲存的編輯。要儲存嗎？"
        );
        assert_eq!(
            text(
                "{worktree} contains a Dev Container configuration file. Would you like to re-open it in a container?"
            ),
            "{worktree} 包含 Dev Container 設定檔。要在容器中重新開啟嗎？"
        );
        assert_eq!(
            text("Your connection to {server} has been lost due to the server {reason}."),
            "與 {server} 的連線已中斷，原因是伺服器{reason}。"
        );
    }

    #[test]
    fn milestone_seven_menu_keymap_and_command_terms_are_guarded() {
        assert_eq!(text("Zoom"), "縮放");
        assert_eq!(text("Keymap Editor"), "鍵盤快速鍵編輯器");
        assert_eq!(text("Filter action names…"), "篩選動作名稱…");
        assert_eq!(text("Edit in JSON"), "以 JSON 編輯");
        assert_eq!(text("Create Keybinding"), "建立鍵盤快速鍵");
        assert_eq!(text("Edit Keybinding"), "編輯鍵盤快速鍵");
        assert_eq!(text("Edit Keystroke"), "編輯按鍵");
        assert_eq!(text("Action"), "動作");
        assert_eq!(text("Arguments"), "引數");
        assert_eq!(text("Keystrokes"), "按鍵");
        assert_eq!(text("Context"), "內容");
        assert_eq!(text("Source"), "來源");
        assert_eq!(text("Filters"), "篩選器");
        assert_eq!(text("Conflicts"), "衝突");
        assert_eq!(text("No Action"), "無動作");
        assert_eq!(text("Search by Keystrokes"), "依按鍵搜尋");
        assert_eq!(text("Toggle Exact Match Mode"), "切換完全相符模式");
        assert_eq!(text("Last Keystroke"), "上一個按鍵");
        assert_eq!(text("Key Equivalents"), "按鍵等效項");
        assert_eq!(text("(match)"), "(相符)");
    }

    #[test]
    fn milestone_seven_settings_terms_are_guarded() {
        assert_eq!(text("Developer"), "開發人員");
        assert_eq!(text("Feature Flags"), "功能旗標");
        assert_eq!(text("Performance Profiler"), "效能分析器");
        assert_eq!(text("General"), "一般");
        assert_eq!(text("When Closing With No Tabs"), "沒有索引標籤時關閉");
        assert_eq!(text("Use System Prompts"), "使用系統提示");
        assert_eq!(text("Workspace Restoration"), "工作區還原");
        assert_eq!(text("Privacy"), "隱私權");
        assert_eq!(text("Auto Update"), "自動更新");
        assert_eq!(text("Buffer Font"), "緩衝區字型");
        assert_eq!(text("UI Font"), "UI 字型");
        assert_eq!(text("Font Family"), "字型系列");
        assert_eq!(text("Font Size"), "字型大小");
        assert_eq!(text("Font Weight"), "字重");
        assert_eq!(text("Fix in settings.json"), "在 settings.json 中修正");
        assert_eq!(text("Manage Trust"), "管理信任");
        assert_eq!(
            text("Your settings are out of date, and need to be updated."),
            "你的設定已過期，需要更新。"
        );
        assert_eq!(
            text("This project is in restricted mode. Some project settings may not apply."),
            "此專案處於受限模式。部分專案設定可能不會套用。"
        );
    }

    #[test]
    fn milestone_seven_notifications_dialogs_and_error_templates_are_guarded() {
        assert_eq!(text("Learn more"), "了解更多");
        assert_eq!(
            text("You may need to configure git for Github."),
            "你可能需要設定 GitHub 的 Git 認證。"
        );
        assert_eq!(text("Switch Branch"), "切換分支");
        assert_eq!(
            text("Commit message title exceeds {max_title_length}-character limit."),
            "提交訊息標題超過 {max_title_length} 個字元限制。"
        );
        assert_eq!(text("Action name is required"), "必須輸入動作名稱");
        assert_eq!(
            text("Action '{action}' not found"),
            "找不到動作「{action}」"
        );
        assert_eq!(
            text("Failed to parse action arguments as JSON"),
            "無法將動作引數剖析為 JSON"
        );
        assert_eq!(text("Keystrokes cannot be empty"), "按鍵不可為空");
        assert_eq!(
            text("Saved edits to the {action} action."),
            "已儲存「{action}」動作的編輯。"
        );
        assert_eq!(text("Request Failed"), "要求失敗");
        assert_eq!(
            text(
                "The request could not be completed after multiple attempts. Try again in a moment."
            ),
            "多次嘗試後仍無法完成要求。請稍候再試。"
        );
        assert_eq!(text("Rate Limit Reached"), "已達速率限制");
        assert_eq!(text("Invalid API Key"), "API 金鑰無效");
    }

    #[test]
    fn milestone_eight_ai_onboarding_and_copilot_terms_are_guarded() {
        assert_eq!(text("Welcome to Zed AI"), "歡迎使用 Zed AI");
        assert_eq!(text("Try Zed Pro for Free"), "免費試用 Zed Pro");
        assert_eq!(text("Start Free Trial"), "開始免費試用");
        assert_eq!(text("Here's what you get:"), "你將獲得：");
        assert_eq!(
            text("Start now using API keys from your environment for the following providers:"),
            "立即使用環境中的 API 金鑰開始，適用於下列提供者："
        );
        assert_eq!(
            text("Use GitHub Copilot in Zed"),
            "在 Zed 中使用 GitHub Copilot"
        );
        assert_eq!(text("Starting Copilot…"), "正在啟動 Copilot…");
        assert_eq!(
            text("Sign in to use GitHub Copilot"),
            "登入以使用 GitHub Copilot"
        );
    }

    #[test]
    fn milestone_eight_agent_mcp_provider_and_profile_terms_are_guarded() {
        assert_eq!(text("Add Custom Server"), "新增自訂伺服器");
        assert_eq!(text("Install from Extensions"), "從延伸模組安裝");
        assert_eq!(text("Configure MCP Server"), "設定 MCP 伺服器");
        assert_eq!(
            text("Model Context Protocol (MCP) Servers"),
            "Model Context Protocol (MCP) 伺服器"
        );
        assert_eq!(text("View Tools"), "檢視工具");
        assert_eq!(text("External Agents"), "外部代理");
        assert_eq!(text("Add Agent"), "新增代理");
        assert_eq!(text("Install from Registry"), "從登錄檔安裝");
        assert_eq!(text("Agent Profiles"), "代理設定檔");
        assert_eq!(text("Custom Profiles"), "自訂設定檔");
        assert_eq!(text("Change Profile"), "變更設定檔");
        assert_eq!(text("Tools Unsupported"), "不支援工具");
        assert_eq!(text("Zed Agent"), "Zed 代理");
        assert_eq!(text("New From Summary"), "從摘要新增");
        assert_eq!(text("Configure Default Model"), "設定預設模型");
        assert_eq!(text("Configure MCP Tools"), "設定 MCP 工具");
        assert_eq!(text("Add LLM Provider"), "新增 LLM 提供者");
        assert_eq!(text("Save Provider"), "儲存提供者");
    }

    #[test]
    fn milestone_eight_provider_edit_prediction_and_tool_permission_terms_are_guarded() {
        assert_eq!(text("Select a Model"), "選取模型");
        assert_eq!(text("Favorite Model"), "加入喜愛模型");
        assert_eq!(text("Unfavorite Model"), "從喜愛模型移除");
        assert_eq!(text("Refresh Models"), "重新整理模型");
        assert_eq!(text("Configure a Provider"), "設定提供者");
        assert_eq!(text("Sign In Or Configure a Provider"), "登入或設定提供者");
        assert_eq!(text("Reset Key"), "重設金鑰");
        assert_eq!(text("Choose a Plan"), "選擇方案");
        assert_eq!(text("Edit Predictions"), "編輯預測");
        assert_eq!(text("Disabled For This File"), "已對此檔案停用");
        assert_eq!(text("Providers"), "提供者");
        assert_eq!(text("Show Edit Predictions For"), "顯示編輯預測的範圍");
        assert_eq!(text("This Buffer"), "此緩衝區");
        assert_eq!(text("All Files"), "所有檔案");
        assert_eq!(text("Training Data Collection"), "訓練資料收集");
        assert_eq!(text("Configure Excluded Files"), "設定排除檔案");
        assert_eq!(text("Predict Edit at Cursor"), "在游標處預測編輯");
        assert_eq!(text("Go to Copilot Settings"), "前往 Copilot 設定");
        assert_eq!(text("Sign In & Start Using"), "登入並開始使用");
        assert_eq!(text("Usage"), "用量");
        assert_eq!(text("Tool Permissions"), "工具權限");
        assert_eq!(text("Always Deny"), "一律拒絕");
        assert_eq!(text("Always Allow"), "一律允許");
        assert_eq!(text("Always Confirm"), "一律確認");
        assert_eq!(text("Test Your Rules"), "測試你的規則");
    }

    #[test]
    fn milestone_eight_skills_rules_and_safety_terms_are_guarded() {
        assert_eq!(text("New Skill"), "新增技能");
        assert_eq!(text("Scope"), "範圍");
        assert_eq!(text("Skill Content"), "技能內容");
        assert_eq!(text("Save Skill"), "儲存技能");
        assert_eq!(text("Saving…"), "正在儲存…");
        assert_eq!(text("Rules Library"), "規則庫");
        assert_eq!(text("Remove from Default Rules"), "從預設規則移除");
        assert_eq!(text("Delete Rule"), "刪除規則");
        assert_eq!(text("New Rule"), "新增規則");
        assert_eq!(text("Agent Changes Rejected"), "已拒絕代理變更");
        assert_eq!(text("Undo"), "復原");
        assert_eq!(
            text(
                "Note: custom tool permissions only apply to the Zed native agent and don’t extend to external agents connected through the Agent Client Protocol (ACP)."
            ),
            "注意：自訂工具權限只會套用到 Zed 原生代理，不會延伸到透過 Agent Client Protocol (ACP) 連線的外部代理。"
        );
    }

    #[test]
    fn milestone_nine_settings_ui_descriptions_links_and_sources_are_guarded() {
        assert_eq!(text("Window & Layout"), "視窗與版面配置");
        assert_eq!(text("Panels"), "面板");
        assert_eq!(text("Audio Settings"), "音訊設定");
        assert_eq!(text("Input Audio Device"), "輸入音訊裝置");
        assert_eq!(text("Proxy"), "代理伺服器");
        assert_eq!(text("Settings Profiles"), "設定檔組態");
        assert_eq!(text("Edit Keybindings"), "編輯鍵盤快速鍵");
        assert_eq!(
            text("Customize keybindings in the keymap editor."),
            "在鍵盤快速鍵編輯器中自訂鍵盤快速鍵。"
        );
        assert_eq!(text("Open Keymap"), "開啟按鍵對應");
        assert_eq!(
            text(
                "Set up different edit prediction providers in complement to Zed's built-in Zeta model."
            ),
            "設定不同的編輯預測提供者，以搭配 Zed 內建的 Zeta 模型。"
        );
        assert_eq!(text("Data Collection"), "資料收集");
        assert_eq!(
            text("Controls whether edit predictions are shown in the given language scopes."),
            "控制是否在指定語言 scope 中顯示編輯預測。"
        );
        assert_eq!(text("Modified in"), "已修改於");
        assert_eq!(text("User"), "使用者");
        assert_eq!(text("Project"), "專案");
        assert_eq!(text("Server"), "伺服器");
        assert_eq!(text("Edit in settings.json"), "在 settings.json 中編輯");
    }

    #[test]
    fn milestone_ten_global_leak_scan_and_polish_terms_are_guarded() {
        assert_eq!(text("Keyboard Context"), "鍵盤情境");
        assert_eq!(text("Current Context Stack"), "目前情境堆疊");
        assert_eq!(text("Open Documentation"), "開啟文件");
        assert_eq!(text("Backup and Update"), "備份並更新");
        assert_eq!(text("Clear Events"), "清除事件");
        assert_eq!(text("Open Raw Log File"), "開啟原始記錄檔");
        assert_eq!(text("Selection Controls"), "選取範圍控制項");
        assert_eq!(text("Editor Controls"), "編輯器控制項");
        assert_eq!(text("Interrupt"), "中斷");
        assert_eq!(text("Clear Outputs"), "清除輸出");
        assert_eq!(text("Select Kernel"), "選取 Kernel");
        assert_eq!(text("Finish Setup"), "完成設定");
        assert_eq!(text("Agent Setup"), "代理設定");
        assert_eq!(
            text("Coming from Neovim? Use our first-class implementation of Vim Mode"),
            "從 Neovim 轉來？使用我們第一流的 Vim 模式實作"
        );
        assert_eq!(
            text("Automatically mark all new projects as trusted to unlock all Zed's features"),
            "自動將所有新專案標記為受信任，以解鎖 Zed 的所有功能"
        );
        assert_eq!(
            text("Help improve Zed by sending anonymous usage data"),
            "傳送匿名使用資料以協助改進 Zed"
        );
        assert_eq!(
            text("Help fix Zed by sending crash reports so we can fix critical issues fast"),
            "傳送當機報告以協助 Zed 快速修正重大問題"
        );
        assert_eq!(text("Select Feature"), "選取功能");
        assert_eq!(text("Confirm Selections"), "確認選取項目");
        assert_eq!(text("Select Toolchain Path"), "選取工具鏈路徑");
        assert_eq!(text("Calling"), "通話中");
        assert_eq!(text("Open Shared Screen"), "開啟分享畫面");
        assert_eq!(text("Create Channel"), "建立頻道");
        assert_eq!(text("Accept invite"), "接受邀請");
        assert_eq!(text("Step Over"), "逐步跳過");
        assert_eq!(text("New Chat"), "新增對話");
        assert_eq!(text("Type to Send"), "輸入即可傳送");
        assert_eq!(text("Open Repository"), "開啟儲存庫");
        assert_eq!(text("Diff View Style"), "差異檢視樣式");
        assert_eq!(text("Git Hunks"), "Git 變更區塊");
        assert_eq!(text("Context Servers"), "內容伺服器");
        assert_eq!(text("Project settings from being applied"), "套用專案設定");
        assert_eq!(
            text("MCP Server integrations from installing"),
            "安裝 MCP 伺服器整合"
        );
    }

    #[test]
    fn milestone_twelve_debugger_surface_terms_are_guarded() {
        assert_eq!(text("Breakpoints"), "中斷點");
        assert_eq!(text("No Breakpoints Set"), "尚未設定中斷點");
        assert_eq!(text("Start Debug Session"), "開始偵錯工作階段");
        assert_eq!(text("New Session"), "新增工作階段");
        assert_eq!(text("Edit debug.json"), "編輯 debug.json");
        assert_eq!(text("Edit in debug.json"), "在 debug.json 中編輯");
        assert_eq!(text("Debugger Docs"), "偵錯工具文件");
        assert_eq!(text("Debugger Extensions"), "偵錯工具延伸模組");
        assert_eq!(text("Open Debug Adapter Logs"), "開啟偵錯配接器記錄");
        assert_eq!(
            text("Launch a new process with a debugger"),
            "使用偵錯工具啟動新程序"
        );
        assert_eq!(text("Run predefined task"), "執行預先定義的工作");
        assert_eq!(
            text("Start a predefined debug scenario"),
            "開始預先定義的偵錯情境"
        );
        assert_eq!(
            text("Attach the debugger to a running process"),
            "將偵錯工具附加到執行中的程序"
        );
        assert_eq!(
            text("Find a debug task, or debug a command"),
            "尋找偵錯工作，或偵錯命令"
        );
        assert_eq!(text("Select Debugger"), "選取偵錯工具");
        assert_eq!(text("No matches"), "沒有相符項目");
        assert_eq!(text("Debug"), "偵錯");
        assert_eq!(text("Attach"), "附加");
        assert_eq!(text("Launch"), "啟動");
        assert_eq!(text("Close Panel"), "關閉面板");
        assert_eq!(text("Pause Program"), "暫停程式");
        assert_eq!(text("Continue Program"), "繼續程式");
        assert_eq!(text("Step In"), "逐步進入");
        assert_eq!(text("Rerun Session"), "重新執行工作階段");
        assert_eq!(text("Terminate Thread"), "終止執行緒");
        assert_eq!(text("Terminate All Threads"), "終止所有執行緒");
        assert_eq!(text("Detach"), "中斷連結");
        assert_eq!(text("Debugger:"), "偵錯工具：");
        assert_eq!(text("Stop on Entry"), "進入時停止");
        assert_eq!(text("Launch Custom"), "啟動自訂設定");
        assert_eq!(text("Evaluate an expression"), "評估運算式");
        assert_eq!(text("Evaluate"), "評估");
        assert_eq!(
            text("Write to Selected Memory Range"),
            "寫入選取的記憶體範圍"
        );
        assert_eq!(
            text("Go to Memory Address / Expression"),
            "前往記憶體位址 / 運算式"
        );
        assert_eq!(text("Restart Stack Frame"), "重新啟動堆疊框架");
    }
}
