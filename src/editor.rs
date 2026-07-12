use crate::i18n;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[cfg(windows)]
fn no_window(cmd: &mut Command) -> &mut Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    cmd.creation_flags(CREATE_NO_WINDOW)
}

#[cfg(not(windows))]
fn no_window(cmd: &mut Command) -> &mut Command {
    cmd
}

/// Check whether an editor command is available (via PATH)
fn cmd_in_path(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}

/// Check whether the path exists and is an executable file
fn exe_exists(path: &str) -> bool {
    let p = Path::new(path);
    p.exists() && p.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("exe"))
}

/// Find the Notepad++ installation path
fn find_notepadpp() -> Option<String> {
    // 1. Search in PATH
    if cmd_in_path("notepad++") {
        return Some("notepad++".to_string());
    }

    // 2. Common install paths
    let drive_letters = ['C', 'D', 'E', 'F'];
    let sub_dirs = [
        "Notepad++",
        "Program Files\\Notepad++",
        "Program Files (x86)\\Notepad++",
    ];
    for drive in &drive_letters {
        for sub in &sub_dirs {
            let path = format!("{}:\\{}\\notepad++.exe", drive, sub);
            if exe_exists(&path) {
                return Some(path);
            }
        }
    }

    // 3. Check Windows registry App Paths
    for key_path in &[
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\notepad++.exe",
    ] {
        if let Ok(output) = Command::new("reg")
            .args(["query", &format!("HKLM\\{}", key_path), "/ve", "/t", "REG_SZ"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Registry output format: "    (Default)    REG_SZ    C:\...\notepad++.exe"
            for line in stdout.lines() {
                if let Some(path) = line.trim().rsplit("REG_SZ").next() {
                    let path = path.trim();
                    if exe_exists(path) {
                        return Some(path.to_string());
                    }
                }
            }
        }
        // Also check HKCU
        if let Ok(output) = Command::new("reg")
            .args(["query", &format!("HKCU\\{}", key_path), "/ve", "/t", "REG_SZ"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(path) = line.trim().rsplit("REG_SZ").next() {
                    let path = path.trim();
                    if exe_exists(path) {
                        return Some(path.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Find the VS Code installation path
fn find_vscode() -> Option<String> {
    // 1. Search in PATH
    if cmd_in_path("code") {
        return Some("code".to_string());
    }

    // 2. Common install paths
    let drive_letters = ['C', 'D', 'E', 'F'];
    for drive in &drive_letters {
        let paths = [
            format!("{}:\\Users\\{}\\AppData\\Local\\Programs\\Microsoft VS Code\\bin\\code.cmd", drive, "Harry"),
            format!("{}:\\Users\\{}\\AppData\\Local\\Programs\\Microsoft VS Code\\bin\\code.cmd", drive, "Administrator"),
            format!("{}:\\Users\\{}\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe", drive, "Harry"),
            format!("{}:\\Users\\{}\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe", drive, "Administrator"),
            format!("{}:\\Program Files\\Microsoft VS Code\\bin\\code.cmd", drive),
            format!("{}:\\Program Files (x86)\\Microsoft VS Code\\bin\\code.cmd", drive),
        ];
        for path in &paths {
            if std::path::Path::new(path).exists() {
                // Return the .cmd path if it's a .cmd script,
                // or the .exe path if it's an .exe
                return Some(path.clone());
            }
        }
    }

    // 3. Check Windows registry App Paths
    for key_path in &[
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\code.exe",
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\VisualStudioCode.exe",
    ] {
        for hive in &["HKLM", "HKCU"] {
            if let Ok(output) = Command::new("reg")
                .args(["query", &format!("{}\\{}", hive, key_path), "/ve", "/t", "REG_SZ"])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some(path) = line.trim().rsplit("REG_SZ").next() {
                        let path = path.trim();
                        if !path.is_empty() && (exe_exists(path) || std::path::Path::new(path).exists()) {
                            return Some(path.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

/// Find the Sublime Text installation path
fn find_sublime() -> Option<String> {
    // 1. Search in PATH
    if cmd_in_path("subl") {
        return Some("subl".to_string());
    }

    // 2. Common install paths
    let drive_letters = ['C', 'D', 'E', 'F'];
    for drive in &drive_letters {
        let paths = [
            format!("{}:\\SublimeText3\\sublime_text.exe", drive),
            format!("{}:\\SublimeText4\\sublime_text.exe", drive),
            format!("{}:\\Program Files\\Sublime Text 3\\sublime_text.exe", drive),
            format!("{}:\\Program Files\\Sublime Text 4\\sublime_text.exe", drive),
            format!("{}:\\Program Files (x86)\\Sublime Text 3\\sublime_text.exe", drive),
        ];
        for path in &paths {
            if exe_exists(path) {
                return Some(path.clone());
            }
        }
    }

    // 3. Check Windows registry
    for key_path in &[
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\sublime_text.exe",
    ] {
        for hive in &["HKLM", "HKCU"] {
            if let Ok(output) = Command::new("reg")
                .args(["query", &format!("{}\\{}", hive, key_path), "/ve", "/t", "REG_SZ"])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some(path) = line.trim().rsplit("REG_SZ").next() {
                        let path = path.trim();
                        if exe_exists(path) {
                            return Some(path.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

fn find_emeditor() -> Option<String> {
    if cmd_in_path("emeditor") {
        return Some("emeditor".to_string());
    }
    let drive_letters = ['C', 'D', 'E', 'F', 'Z'];
    let sub_dirs = [
        "EmEditor",
        "Program Files\\EmEditor",
        "Program Files (x86)\\EmEditor",
        "emed64",
        "EmEditor64",
    ];
    for drive in &drive_letters {
        for sub in &sub_dirs {
            let path = format!("{}:\\{}\\EmEditor.exe", drive, sub);
            if exe_exists(&path) {
                return Some(path);
            }
        }
    }
    for key_path in &[
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\EmEditor.exe",
    ] {
        for hive in &["HKLM", "HKCU"] {
            if let Ok(output) = Command::new("reg")
                .args(["query", &format!("{}\\{}", hive, key_path), "/ve", "/t", "REG_SZ"])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some(path) = line.trim().rsplit("REG_SZ").next() {
                        let path = path.trim();
                        if exe_exists(path) {
                            return Some(path.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Cached editor paths to avoid repeated searches
/// key: "notepad++" / "code" / "subl" / "emeditor"
/// value: None means not found, Some(path) is the found path
static EDITOR_CACHE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, Option<String>>>> =
    std::sync::OnceLock::new();

fn get_editor_cache() -> &'static std::sync::Mutex<std::collections::HashMap<String, Option<String>>> {
    EDITOR_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// Check whether an editor is available (multi-strategy detection + caching)
pub fn editor_available(name: &str) -> bool {
    let cache = get_editor_cache();
    {
        let cached = cache.lock().unwrap();
        if let Some(result) = cached.get(name) {
            return result.is_some();
        }
    }

    let found = match name {
        "notepad++" => find_notepadpp(),
        "code" => find_vscode(),
        "subl" => find_sublime(),
        "emeditor" => find_emeditor(),
        _ => {
            if cmd_in_path(name) {
                Some(name.to_string())
            } else {
                None
            }
        }
    };

    let available = found.is_some();
    cache.lock().unwrap().insert(name.to_string(), found);
    available
}

/// Get the actual executable path for an editor
pub fn editor_path(name: &str) -> Option<String> {
    let cache = get_editor_cache();
    {
        let cached = cache.lock().unwrap();
        if let Some(result) = cached.get(name) {
            return result.clone();
        }
    }

    let found = match name {
        "notepad++" => find_notepadpp(),
        "code" => find_vscode(),
        "subl" => find_sublime(),
        "emeditor" => find_emeditor(),
        _ => {
            if cmd_in_path(name) {
                Some(name.to_string())
            } else {
                None
            }
        }
    };

    let path = found.clone();
    cache.lock().unwrap().insert(name.to_string(), found);
    path
}

/// Open an editor and jump to the specified line
pub fn open_editor_at_line(file: &Path, line: usize) -> Result<()> {
    let file_str = file.to_str().unwrap();

    // 1. Environment variable MD_EDITOR
    if let Ok(editor) = std::env::var("MD_EDITOR") {
        let parts: Vec<&str> = editor.split_whitespace().collect();
        if !parts.is_empty() {
            let mut cmd = Command::new(parts[0]);
            cmd.args(&parts[1..])
                .args([file_str, &line.to_string()]);
            no_window(&mut cmd)
                .spawn()
                .context(i18n::tr("error_cannot_start_md_editor"))?;
            return Ok(());
        }
    }

    // 2. Editor setting from the config file
    let config_editor = crate::config::load_config().editor;
    if !config_editor.is_empty() {
        match config_editor.as_str() {
            "subl" => {
                let exe = editor_path("subl").unwrap_or_else(|| "subl".to_string());
                let mut cmd = Command::new(&exe);
                cmd.arg(format!("{}:{}", file_str, line));
                no_window(&mut cmd).spawn()?;
                return Ok(());
            }
            "code" => {
                let exe = editor_path("code").unwrap_or_else(|| "code".to_string());
                let mut cmd = Command::new(&exe);
                cmd.args(["--goto", &format!("{}:{}", file_str, line)]);
                no_window(&mut cmd).spawn()?;
                return Ok(());
            }
            "notepad++" => {
                let exe = editor_path("notepad++").unwrap_or_else(|| "notepad++".to_string());
                let mut cmd = Command::new(&exe);
                cmd.args([file_str, "-n", &line.to_string()]);
                no_window(&mut cmd).spawn()?;
                return Ok(());
            }
            "emeditor" => {
                let exe = editor_path("emeditor").unwrap_or_else(|| "emeditor".to_string());
                let mut cmd = Command::new(&exe);
                cmd.args(["/l", &line.to_string(), file_str]);
                no_window(&mut cmd).spawn()?;
                return Ok(());
            }
            "notepad" => {
                let mut cmd = Command::new("notepad");
                cmd.arg(file_str);
                no_window(&mut cmd).spawn()?;
                return Ok(());
            }
            _ => {
                // Try to parse as a custom command
                // If the value contains a path separator or ends with .exe, treat it as a full executable path
                if config_editor.contains('\\')
                    || config_editor.contains('/')
                    || config_editor.to_lowercase().ends_with(".exe")
                {
                    let lower = config_editor.to_lowercase();
                    let mut cmd = Command::new(&config_editor);
                    if lower.contains("emeditor") {
                        cmd.args(["/l", &line.to_string(), file_str]);
                    } else {
                        cmd.args([file_str, &line.to_string()]);
                    }
                    no_window(&mut cmd)
                        .spawn()
                        .context(i18n::tr("error_cannot_start_editor"))?;
                    return Ok(());
                }
                let parts: Vec<&str> = config_editor.split_whitespace().collect();
                if !parts.is_empty() {
                    let mut cmd = Command::new(parts[0]);
                    cmd.args(&parts[1..])
                        .args([file_str, &line.to_string()]);
                    no_window(&mut cmd)
                        .spawn()
                        .context(i18n::tr("error_cannot_start_editor"))?;
                    return Ok(());
                }
            }
        }
    }

    // 3. Sublime Text
    if editor_available("subl") {
        let exe = editor_path("subl").unwrap_or_else(|| "subl".to_string());
        let mut cmd = Command::new(&exe);
        cmd.arg(format!("{}:{}", file_str, line));
        no_window(&mut cmd).spawn()?;
        return Ok(());
    }

    // 4. VS Code
    if editor_available("code") {
        let exe = editor_path("code").unwrap_or_else(|| "code".to_string());
        let mut cmd = Command::new(&exe);
        cmd.args(["--goto", &format!("{}:{}", file_str, line)]);
        no_window(&mut cmd).spawn()?;
        return Ok(());
    }

    // 5. Notepad++
    if editor_available("notepad++") {
        let exe = editor_path("notepad++").unwrap_or_else(|| "notepad++".to_string());
        let mut cmd = Command::new(&exe);
        cmd.args([file_str, "-n", &line.to_string()]);
        no_window(&mut cmd).spawn()?;
        return Ok(());
    }

    // 6. Notepad (does not support line numbers)
    let mut cmd = Command::new("notepad");
    cmd.arg(file_str);
    no_window(&mut cmd).spawn()?;
    Ok(())
}
