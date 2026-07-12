//! mdview internationalization (i18n) module.
//!
//! Lightweight custom implementation:
//! - All UI strings are defined centrally in the `define_translations!` macro
//! - Looked up at runtime via `tr!(key)` / `i18n::tr(key)`
//! - The full table is injected into the frontend as `window.__mdI18n`
//!
//! English only.

use serde::Serialize;
use std::sync::RwLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
}

impl Lang {
    pub fn code(&self) -> &'static str {
        "en-US"
    }

    pub fn webview2_language(&self) -> &'static str {
        "en-US"
    }

    pub fn translations(&self) -> &'static Translations {
        &EN
    }
}

/// Active language (single variant; kept for a stable public API).
static CURRENT_LANG: RwLock<Lang> = RwLock::new(Lang::En);

/// Initialize the module. Called once at startup.
pub fn init() {
    set_lang(Lang::En);
}

pub fn current_lang() -> Lang {
    *CURRENT_LANG.read().unwrap()
}

pub fn set_lang(lang: Lang) {
    *CURRENT_LANG.write().unwrap() = lang;
}

/// Return the translation for `key`.
/// Falls back to the key itself when missing (useful while debugging).
pub fn tr(key: &str) -> &str {
    translate(current_lang(), key)
}

/// Translation with positional placeholders (`{0}`, `{1}`, …, then `{}`).
pub fn trf(key: &str, args: &[&str]) -> String {
    let mut s = tr(key).to_string();
    // Numbered placeholders first
    for (i, arg) in args.iter().enumerate() {
        s = s.replace(&format!("{{{}}}", i), arg);
    }
    // Then sequential {}
    let mut idx = 0;
    while let Some(pos) = s.find("{}") {
        if idx >= args.len() {
            break;
        }
        s.replace_range(pos..pos + 2, args[idx]);
        idx += 1;
    }
    s
}

/// Convenience macro: `trf!("key", &arg1, &arg2)`.
#[macro_export]
macro_rules! trf {
    ($key:expr $(, $arg:expr)*) => {{
        let args: &[&str] = &[$($arg),*];
        $crate::i18n::trf($key, args)
    }};
}

/// Serialize all frontend strings to JSON, injected as `window.__mdI18n`.
pub fn frontend_json() -> String {
    serde_json::json!({
        "lang": current_lang().code(),
        "strings": current_lang().translations(),
    })
    .to_string()
}

/// Template substitution: replace every `{{i18n.key}}` in `html` with its string.
pub fn render_template(html: &str) -> String {
    let mut out = html.to_string();
    let t = current_lang().translations();
    // Serialize to a field-name -> value map, then replace each placeholder.
    if let serde_json::Value::Object(map) = serde_json::to_value(t).unwrap_or_default() {
        for (k, v) in map {
            if let Some(v) = v.as_str() {
                out = out.replace(&format!("{{{{i18n.{}}}}}", k), v);
            }
        }
    }
    out
}

// ═══════════════════════════════════════════════════════════════
//  Macro: central definition of all translation strings
// ═══════════════════════════════════════════════════════════════

macro_rules! define_translations {
    (
        $($key:ident: $en:expr),* $(,)?
    ) => {
        #[derive(Serialize)]
        pub struct Translations {
            $(pub $key: &'static str,)*
        }

        const EN: Translations = Translations {
            $($key: $en,)*
        };

        fn translate<'a>(lang: Lang, key: &'a str) -> &'a str {
            let t = lang.translations();
            match key {
                $(stringify!($key) => t.$key,)*
                _ => key,
            }
        }
    };
}

define_translations! {
    // App / general
    app_name: "mdview",

    // Window titles
    title_main: "mdview",
    title_settings: "Settings",
    title_welcome: "mdview",
    title_security_warning: "Security Notice",

    // CLI help
    help_usage: "Usage:",
    help_desc_preview: "Start preview server and open window",
    help_desc_install: "Install context menu",
    help_desc_uninstall: "Uninstall context menu",
    help_desc_settings: "Open settings window",
    help_desc_unbind: "Remove default .md association",
    help_desc_help: "Show help",
    help_env: "Environment variables:",
    help_env_port: "Specify server port (default: random available port or 3456)",
    help_env_md_html: "Output .html file next to the markdown file",
    help_env_md_html_output: "Output .html to specified path",
    help_example: "Examples:",
    help_arg_md_file: "markdown-file",
    help_env_port_label: "port",
    help_env_md_html_output_label: "path",

    // Errors
    error_file_not_found: "Error: file '{0}' does not exist",
    error_windows_only: "Error: this feature is only available on Windows",
    error_context_menu_windows_only: "Error: context menu is only available on Windows",
    error_invalid_exe_path: "Invalid executable path",
    error_cannot_start_editor: "Unable to start configured editor",
    error_cannot_start_md_editor: "Unable to start MD_EDITOR",
    error_render: "Render failed",
    error_render_detail: "Render error",
    error_md4x_render: "md4x rendering failed",
    error_md4x_not_utf8: "md4x output is not UTF-8: {0}",
    error_file_read: "Unable to read file: {0}",

    // Status / logs
    status_bound_md: ".md files have been set as the default handler",
    status_auto_bind_failed: "Auto-bind failed",
    status_server_error: "Server error: {0}",
    status_welcome_server_error: "Welcome page server error: {0}",
    status_settings_server_error: "Settings server error: {0}",
    status_watching: "[watch] watching directory: {0}, target file: {1}",
    status_md_changed: "[watch] Markdown file changed, triggering update",
    status_image_changed: "[watch] image file changed, triggering refresh",
    status_watcher_started: "[watch] file watcher started",
    status_watcher_stopped: "[watch] file watcher stopped",
    status_rendering: "[{0}] render: {1}",
    status_html_output: "[{0}] HTML output: {1}",
    status_started: "📝 mdview started\n   File: {0}\n   URL: {1}\n\nClose the preview window to exit...",
    status_loading: "Loading...",

    // File dialogs
    dialog_filter_exe: "Executable files",
    dialog_title_editor: "Choose editor",

    // Windows registry / context menu
    reg_friendly_name_md: "Markdown file",
    reg_open_with_mdview: "Open with mdview",
    reg_ctx_menu_name: "mdview",
    reg_set_default_success: "✅ Set as default program for .md files",
    reg_remove_default_success: "Removed default program for .md files",
    reg_install_ctx_success: "✅ Context menu installed successfully (no admin required)",
    reg_install_ctx_hint: "   Right-click a .md file to see the \"mdview\" option",
    reg_install_ctx_restart_explorer: "   If it doesn't appear immediately, try restarting File Explorer.",
    reg_uninstall_ctx_found: "✅ Context menu uninstalled",
    reg_uninstall_ctx_not_found: "ℹ️ No installed context menu found",
    reg_uninstall_ctx_from_prog: "✅ Removed context menu from {0}",
    reg_windows_only: "Only available on Windows",

    // WebView2 prompts
    webview2_title: "WebView2 Runtime Required - mdview",
    webview2_message: "mdview requires the Microsoft Edge WebView2 Runtime to display content, but it was not detected on this system.\n\nIf your Windows 10 has a newer version of Microsoft Edge installed, or has received regular updates in the last two years, WebView2 Runtime is usually already present and does not need to be installed again.\n\nTo verify: open Control Panel → Programs and Features, and look for \"Microsoft Edge WebView2 Runtime\".\n\nWould you like to open your browser to download the WebView2 online installer?\n(Run the installer, then reopen mdview.)",
    webview2_missing_cli: "mdview requires WebView2 Runtime. Please install manually: {0}",

    // Editors
    editor_notepad: "Notepad",
    editor_notepad_plus: "Notepad++",
    editor_vscode: "VS Code",
    editor_sublime: "Sublime Text",
    editor_emeditor: "EmEditor",

    // Frontend: common
    btn_ok: "OK",
    btn_cancel: "Cancel",
    btn_close: "Close",
    btn_save: "Save",
    btn_browse: "Browse...",
    btn_confirm: "Done",
    btn_retry: "Retry",

    // Frontend: TOC
    toc_title: "Contents",
    toc_pos_viewport_left: "Far left",
    toc_pos_article_left: "Article left",
    toc_pos_article_right: "Article right",
    toc_pos_viewport_right: "Far right",
    toc_pos_title: "Current position: {0} (click to switch)",
    toc_hide: "Hide contents",
    toc_show: "Show contents",
    toc_visit_website: "Visit website",

    // Frontend: context menu
    menu_copy: "Copy",
    menu_select_all: "Select all",
    menu_export_pdf: "Export PDF",
    menu_theme_light: "Switch to light theme",
    menu_theme_dark: "Switch to dark theme",
    menu_font_settings: "Font settings",
    menu_edit_source: "Edit source",
    menu_editor_settings: "Editor settings",
    menu_about: "About",

    // Frontend: edit button
    edit_btn_tooltip: "Edit source (Ctrl+E)",
    bind_btn_tooltip: "File association settings",
    bind_card_title: ".md file association bound",
    bind_card_desc: "Double-click a .md file to open it with this tool",
    bind_unbind: "Unbind",
    bind_processing: "Processing...",
    bind_unbind_success_title: "Unbound",
    bind_unbind_success_desc: ".md files will use the previously saved handler",
    bind_unbind_failed: "Failed to unbind",
    bind_unknown_error: "Unknown error",

    // Frontend: about modal
    about_title: "mdview",
    about_subtitle: "Lightweight Markdown preview tool",
    about_credits: "Markdown rendering by md4x · MIT License",
    about_learn_more: "Learn more",

    // Frontend: editor settings modal
    editor_settings_title: "Editor Settings",
    editor_auto_detect: "Auto detect",
    editor_custom_prefix: "Custom",
    editor_not_installed: "(not installed)",
    editor_save_failed: "Save failed",
    editor_load_failed: "Failed to load editor config",
    edit_open_failed: "Failed to open source",

    // Frontend: font settings modal
    font_settings_title: "Font Settings",
    font_search_placeholder: "Search fonts...",
    font_preview_text: "Font preview AaBb Hello World 123",
    font_default: "Default (system font)",
    font_load_failed: "Failed to load font list",
    font_save_failed: "Save failed",

    // Frontend: zoom indicator
    zoom_font_size: "Font size",
    zoom_column_width: "Column width",

    // Settings page
    settings_app_name: "mdview",
    settings_app_desc: "Lightweight Markdown live previewer",
    settings_ctx_menu: "Context Menu",
    settings_ctx_menu_desc: "Right-click a .md file to see the \"mdview\" option.",
    settings_add_ctx_menu: "Add context menu",
    settings_remove_ctx_menu: "Remove context menu",
    settings_default_association: "Default Association",
    settings_default_association_desc: "Double-click a .md file to open it directly with this tool.",
    settings_set_default: "Set as default",
    settings_usage_title: "Usage:",
    settings_usage_ctx_menu: "Right-click any .md file → select \"mdview\"",
    settings_usage_cli: "Or run in terminal: mdview your-file.md",
    settings_language: "Language",
    settings_repair_ctx_menu: "Repair context menu",
    status_installed: "Installed",
    status_not_installed: "Not installed",
    status_set: "Set",
    status_not_set: "Not set",
    status_operation_failed: "Operation failed",
    status_checking: "Checking...",
    lang_attr: "en-US",
    lang_auto: "Auto detect",
    lang_zh_cn: "Chinese (Simplified)",
    lang_en_us: "English",

    // Welcome page
    welcome_move_tool_title: "Recommended: move the executable",
    welcome_move_tool_desc: "The tool is currently in a \"Downloads\" or \"Temp\" folder. To prevent .md files from becoming unopenable if the tool is accidentally deleted, please move this program to a permanent folder before running it.",
    welcome_current_path: "Current path",
    welcome_got_it: "Got it, exit",
    welcome_bound_title: "Set as default handler for .md files",
    welcome_bound_desc: "Double-click any .md file to preview it",
    welcome_unbind: "Unbind",
    welcome_not_bound_title: ".md files not associated",
    welcome_not_bound_desc: "Set as the default handler, then double-click any file for instant preview.",
    welcome_set_default: "Set as default handler",
    welcome_learn_more: "Learn more",
    welcome_operation_failed: "Operation failed",
    welcome_unknown_error: "Unknown error",

    // Server messages
    msg_ctx_menu_added: "Context menu added",
    msg_ctx_menu_removed: "Context menu removed",
    msg_set_default_success: "Set as default program",
    msg_unbound: "Unbound",
    msg_failed: "Failed",
    msg_save_config_failed: "Failed to save config: {0}",
    msg_save_config_success: "Config saved",
    msg_open_editor_failed: "Failed to open editor: {0}",
    msg_read_file_failed: "Failed to read file: {0}",
}
