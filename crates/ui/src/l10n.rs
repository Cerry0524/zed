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
        "Close" => "關閉",
        "Save" => "儲存",
        "New" => "新增",
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
        "Configure" => "設定",
        "Install" => "安裝",
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
        "Change Keybinding…" => "變更鍵盤快速鍵…",
        "Add Keybinding…" => "新增鍵盤快速鍵…",
        "Run" => "執行",
        "Clear Filter" => "清除篩選",
        "Change Mode" => "變更模式",
        "Cycle Through Modes" => "循環切換模式",
        "Change Model" => "變更模型",
        "Cycle Favorited Models" => "循環切換我的最愛模型",
        "Active Provider" => "作用中的提供者",
        "No global skills installed." => "尚未安裝全域技能。",
        "No project skills found." => "找不到專案技能。",
        "View" => "檢視",
        "Upgrade to Zed Pro" => "升級至 Zed Pro",
        "Current Plan" => "目前方案",
        "Free" => "免費",
        "Pro" => "Pro",
        "Commit" => "提交",
        "Version" => "版本",
        "Moving Zed to Applications" => "正在將 Zed 移至「應用程式」",
        "Zed will reopen when installation is complete." => "安裝完成後 Zed 將重新開啟。",
        "Installing Zed…" => "正在安裝 Zed…",
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
}
