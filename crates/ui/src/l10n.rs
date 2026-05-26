pub const PERSONAL_FORK_LOCALE: &str = "zh-Hant";

pub fn text(english: &'static str) -> &'static str {
    text_for_locale(PERSONAL_FORK_LOCALE, english)
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
        "Replace" => "取代",
        "Open" => "開啟",
        "Open…" => "開啟…",
        "Open File..." => "開啟檔案...",
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
        "Close" => "關閉",
        "Close Editor" => "關閉編輯器",
        "Close Project" => "關閉專案",
        "Close Window" => "關閉視窗",
        "Save" => "儲存",
        "Save As…" => "另存新檔…",
        "Save All" => "全部儲存",
        "New" => "新增",
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
        "Yes" => "是",
        "No" => "否",
        "Don't ask me again" => "不要再詢問",
        "Configure" => "設定",
        "Install" => "安裝",
        "Install CLI" => "安裝 CLI",
        "Uninstall" => "解除安裝",
        "Update" => "更新",
        "Reload" => "重新載入",
        "Recent" => "最近使用",
        "Favorites" => "我的最愛",
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
        "Ok" | "OK" => "確定",
        "Cancel" => "取消",
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
        "Moving Zed to Applications" => "正在將 Zed 移至「應用程式」",
        "Zed will reopen when installation is complete." => "安裝完成後 Zed 將重新開啟。",
        "Installing Zed…" => "正在安裝 Zed…",
        "Move Zed to Applications?" => "要將 Zed 移至「應用程式」嗎？",
        "Zed is running from a temporary location. Move it to Applications to finish installing it." => {
            "Zed 正從暫存位置執行。請將它移至「應用程式」以完成安裝。"
        }
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
}
