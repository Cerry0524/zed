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
        "Step Over" => "逐程序",
        "Step Into" => "逐步執行",
        "Step Out" => "跳出",
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
        "New Chat" => "新增聊天",
        "Chat" => "聊天",
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
        "Type to Send" => "輸入後傳送",
        "Send Message" => "傳送訊息",
        "Restore Checkpoint" => "還原檢查點",
        "Stop Subagent" => "停止子代理",
        "Minimize Subagent" => "最小化子代理",
        "Make Subagent Full Screen" => "讓子代理全螢幕",
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
        "Search agents..." => "搜尋代理...",
        "Unavailable" => "無法使用",
        "ACP Registry" => "ACP 登錄檔",
        "Learn More" => "了解更多",
        "Configure Providers" => "設定提供者",
        "API Keys" => "API 金鑰",
        "Add your own keys to use AI without signing in." => {
            "新增你自己的金鑰，不登入也能使用 AI。"
        }
        "Reinstall Copilot and Sign In" => "重新安裝 Copilot 並登入",
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
        "What to do when using the 'close active item' action with no tabs." => {
            "沒有索引標籤時，使用「關閉作用中項目」動作要執行的行為。"
        }
        "What to do when the last window is closed." => "最後一個視窗關閉時要執行的行為。",
        "Use native OS dialogs for 'Open' and 'Save As'." => {
            "針對「開啟」與「另存新檔」使用作業系統原生對話框。"
        }
        "Use native OS dialogs for confirmations." => "確認時使用作業系統原生對話框。",
        "Hide the values of variables in private files." => "隱藏私人檔案中的變數值。",
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
        "Project settings from being applied" => "專案設定被套用",
        "Language servers from running" => "語言伺服器執行",
        "MCP Server integrations from installing" => "MCP Server 整合安裝",
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
        "As a multi-root folder" => "作為多根資料夾",
        "Add Folder to this Project" => "將資料夾新增至此專案",
        "Get Started" => "開始使用",
        "Open Project" => "開啟專案",
        "Clone Repository" => "複製儲存庫",
        "Open Command Palette" => "開啟命令選擇區",
        "Customize Keymaps" => "自訂按鍵對應",
        "Explore Extensions" => "探索延伸模組",
        "Recent Projects" => "最近專案",
        "Welcome back to Zed" => "歡迎回到 Zed",
        "Welcome to Zed" => "歡迎使用 Zed",
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
        "Keyboard Context" => "鍵盤內容條件",
        "This view lets you determine the current context stack for creating custom key bindings in Zed. When a keyboard shortcut is triggered, it also shows all the possible contexts it could have triggered in, and which one matched." => {
            "此檢視可協助判斷目前的內容條件堆疊，以便在 Zed 中建立自訂鍵盤快速鍵。觸發鍵盤快速鍵時，也會顯示所有可能觸發的內容條件，以及實際相符的項目。"
        }
        "Open Documentation" => "開啟文件",
        "View Default Keymap" => "檢視預設按鍵對應",
        "Edit Keymap File" => "編輯按鍵對應檔",
        "Current Context Stack" => "目前內容條件堆疊",
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
        "You may need to configure git for Github." => "你可能需要設定 GitHub 的 Git。",
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
            "這個對話已超過模型的內容視窗。請開啟新的對話串，或移除部分附加檔案後再繼續。"
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
            "你可能需要設定 GitHub 的 Git。"
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
}
