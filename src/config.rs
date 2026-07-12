use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub md_backup: String,
    #[serde(default)]
    pub recent_files: Vec<String>,
    #[serde(default)]
    pub editor: String,
    #[serde(default)]
    pub window_width: u32,
    #[serde(default)]
    pub window_height: u32,
    #[serde(default)]
    pub window_x: i32,
    #[serde(default)]
    pub window_y: i32,
    #[serde(default)]
    pub window_maximized: bool,
    /// Markdown body font; empty string uses the built-in default stack.
    #[serde(default)]
    pub md_font: String,
    /// UI language; empty string means auto-detect.
    #[serde(default)]
    pub lang: String,
}

pub const MAX_RECENT_FILES: usize = 5;

/// Add a file to the recent files list (dedup, move to top, enforce limit)
pub fn add_recent_file(config: &mut AppConfig, path: &str) {
    let path = path.to_string();
    config.recent_files.retain(|p| p != &path);
    config.recent_files.insert(0, path);
    if config.recent_files.len() > MAX_RECENT_FILES {
        config.recent_files.truncate(MAX_RECENT_FILES);
    }
}

pub fn get_config_path() -> PathBuf {
    #[cfg(windows)]
    {
        let app_data = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("USERPROFILE")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| PathBuf::from("."))
            });
        app_data.join("mdview").join("config.json")
    }
    #[cfg(not(windows))]
    {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        home.join(".config").join("mdview").join("config.json")
    }
}

/// WebView2 user data directory.
///
/// WebView2 creates its data directory next to the exe by default (`<exe>.exe.WebView2`).
/// When the app is installed in a read-only location like `Program Files`, WebView2
/// cannot read/write that directory and reports "cannot read or write its data directory".
/// Therefore we explicitly point it to a user-writable directory.
pub fn get_webview_data_directory() -> PathBuf {
    #[cfg(windows)]
    {
        let base = std::env::var("LOCALAPPDATA")
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .or_else(|| std::env::var("APPDATA").ok().map(PathBuf::from))
            .unwrap_or_else(|| {
                std::env::var("USERPROFILE")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| PathBuf::from("."))
            });
        base.join("mdview").join("WebView2")
    }
    #[cfg(not(windows))]
    {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        home.join(".cache").join("mdview")
    }
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let config_str = serde_json::to_string_pretty(config)?;
    std::fs::write(path, config_str)?;
    Ok(())
}

pub fn get_md_backup() -> String {
    load_config().md_backup
}
