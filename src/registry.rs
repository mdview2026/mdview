#[cfg(windows)]
use crate::config::{load_config, save_config, get_md_backup};
use crate::i18n;
use anyhow::Result;
#[cfg(windows)]
use anyhow::Context;

#[cfg(windows)]
use winreg::{enums::*, RegKey};

pub fn backup_current_md_handler() -> Result<()> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let old_value = hkcu
            .open_subkey(r"Software\Classes\.md")
            .and_then(|key| key.get_value::<String, _>(""))
            .unwrap_or_default();
        if old_value != "MdView" && !old_value.is_empty() {
            // Only back up ProgIDs that are still valid (have a corresponding registry key),
            // to avoid backing up leftover ProgIDs from uninstalled tools (e.g. old MdPreview)
            let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
            let is_valid = hkcr.open_subkey(&old_value).is_ok();
            let mut config = load_config();
            if is_valid {
                config.md_backup = old_value;
            } else {
                config.md_backup = String::new();
            }
            save_config(&config)?;
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        Ok(())
    }
}

pub fn is_context_menu_installed() -> bool {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        hkcu.open_subkey(r"Software\Classes\SystemFileAssociations\.md\shell\mdview")
            .is_ok()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

pub fn is_default_md_handler() -> bool {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Windows 10/11 UserChoice has the highest priority, so check it first
        if let Ok(user_choice) = hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.md\UserChoice") {
            if let Ok(progid) = user_choice.get_value::<String, _>("Progid") {
                if progid != "MdView" {
                    return false;
                }
                // UserChoice is already MdView; also confirm Classes matches
            }
        }

        if let Ok(prog) = hkcu.open_subkey(r"Software\Classes\.md") {
            if let Ok(val) = prog.get_value::<String, _>("") {
                if val == "MdView" {
                    return true;
                }
            }
        }
        false
    }
    #[cfg(not(windows))]
    {
        false
    }
}

pub fn set_as_default_handler() -> Result<()> {
    #[cfg(windows)]
    {
        backup_current_md_handler()?;
        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_str().context(i18n::tr("error_invalid_exe_path"))?;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Clear stale Windows FileExts entries (including UserChoice, UserChoiceLatest,
        // OpenWithList, OpenWithProgids, etc.). These override Software\Classes settings
        // and would cause double-click to still open with the previous app (e.g. VSCode).
        let _ = hkcu.delete_subkey_all(r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.md");

        // .md -> MdView
        let (md_key, _) = hkcu.create_subkey(r"Software\Classes\.md")?;
        md_key.set_value("", &"MdView")?;

        // MdView ProgID
        let (prog, _) = hkcu.create_subkey(r"Software\Classes\MdView")?;
        prog.set_value("", &i18n::tr("reg_friendly_name_md"))?;

        let (default_icon, _) = prog.create_subkey("DefaultIcon")?;
        default_icon.set_value("", &format!("{},0", exe_path_str))?;

        let (shell, _) = prog.create_subkey(r"shell")?;
        shell.set_value("", &"open")?;

        let (open, _) = shell.create_subkey("open")?;
        open.set_value("", &i18n::tr("reg_open_with_mdview"))?;

        let (cmd, _) = open.create_subkey("command")?;
        cmd.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))?;

        unsafe {
            use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED};
            SHChangeNotify(
                SHCNE_ASSOCCHANGED as i32,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );
        }

        println!("{}", i18n::tr("reg_set_default_success"));
        Ok(())
    }
    #[cfg(not(windows))]
    {
        anyhow::bail!(i18n::tr("reg_windows_only"))
    }
}

pub fn remove_as_default_handler() -> Result<()> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let backup = get_md_backup();

        // Get the current value
        let current_val = hkcu
            .open_subkey(r"Software\Classes\.md")
            .and_then(|key| key.get_value::<String, _>(""))
            .unwrap_or_default();

        // Only clear if the current value is the MdView we set
        if current_val == "MdView" {
            if backup.is_empty() || backup == "MdView" {
                // Clear the default value rather than deleting the whole .md subkey,
                // since .md may contain other important data (e.g. OpenWithProgids)
                if let Ok((md_key, _)) = hkcu.create_subkey(r"Software\Classes\.md") {
                    let _ = md_key.delete_value("");
                }
            } else {
                // Restore the backed-up value
                if let Ok((md_key, _)) = hkcu.create_subkey(r"Software\Classes\.md") {
                    let _ = md_key.set_value("", &backup);
                }
            }
        }

        // Also clear stale FileExts entries to ensure unbinding takes effect
        let _ = hkcu.delete_subkey_all(r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.md");

        // Delete the MdView ProgID
        let _ = hkcu.delete_subkey_all(r"Software\Classes\MdView");

        // Refresh Explorer
        unsafe {
            use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED};
            SHChangeNotify(
                SHCNE_ASSOCCHANGED as i32,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );
        }

        println!("{}", i18n::tr("reg_remove_default_success"));
        Ok(())
    }
    #[cfg(not(windows))]
    {
        anyhow::bail!(i18n::tr("reg_windows_only"))
    }
}

pub fn install_context_menu() -> Result<()> {
    #[cfg(windows)]
    {
        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_str().context(i18n::tr("error_invalid_exe_path"))?;

        // Write to HKCU (no admin rights needed); SystemFileAssociations gives better compatibility
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (md_shell, _) = hkcu.create_subkey(r"Software\Classes\SystemFileAssociations\.md\shell")?;

        // Create the context menu entry
        let (md_preview, _) = md_shell.create_subkey("mdview")?;
        md_preview.set_value("", &i18n::tr("reg_ctx_menu_name"))?;
        md_preview.set_value("Icon", &format!("{},0", exe_path_str))?;

        // The associated command
        let (md_cmd, _) = md_preview.create_subkey("command")?;
        md_cmd.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))?;

        // Refresh Explorer
        unsafe {
            use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED};
            SHChangeNotify(
                SHCNE_ASSOCCHANGED as i32,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );
        }

        println!("{}", i18n::tr("reg_install_ctx_success"));
        println!("{}", i18n::tr("reg_install_ctx_hint"));
        println!("{}", i18n::tr("reg_install_ctx_restart_explorer"));
        Ok(())
    }
    #[cfg(not(windows))]
    {
        anyhow::bail!(i18n::tr("reg_windows_only"))
    }
}

pub fn uninstall_context_menu() -> Result<()> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let mut found = false;

        // Remove the menu entry from SystemFileAssociations
        if let Ok(shell) = hkcu.open_subkey(r"Software\Classes\SystemFileAssociations\.md\shell") {
            if shell.open_subkey("mdview").is_ok() {
                let _ = shell.delete_subkey_all("mdview");
                println!("{}", i18n::tr("reg_uninstall_ctx_found"));
                found = true;
            }
        }

        // Backward compat: also clean up old versions written under HKEY_CLASSES_ROOT (if removable)
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

        if let Ok((md_ext, _)) = hkcr.create_subkey(".md") {
            if let Ok((shell, _)) = md_ext.create_subkey("shell") {
                if shell.open_subkey("mdview").is_ok() {
                    let _ = shell.delete_subkey_all("mdview");
                    found = true;
                }
            }
            if let Ok(prog_id) = md_ext.get_value::<String, _>("") {
                if let Ok((prog_key, _)) = hkcr.create_subkey(&prog_id) {
                    if let Ok((shell, _)) = prog_key.create_subkey("shell") {
                        if shell.open_subkey("mdview").is_ok() {
                            let _ = shell.delete_subkey_all("mdview");
                            found = true;
                        }
                    }
                }
            }
        }

        // Try to remove the menu from common ProgIDs
        let common_prog_ids = [
            "mdfile",
            "MarkdownFile",
            "Markdown",
            "Typora.md",
            "vscode.md",
        ];
        for prog_id in &common_prog_ids {
            if let Ok(prog_key) = hkcr.open_subkey(prog_id) {
                if let Ok(shell) = prog_key.open_subkey("shell") {
                    if shell.open_subkey("mdview").is_ok() {
                        let _ = shell.delete_subkey_all("mdview");
                        println!("{}", i18n::trf("reg_uninstall_ctx_from_prog", &[prog_id]));
                        found = true;
                    }
                }
            }
        }

        // Refresh Explorer
        unsafe {
            use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED};
            SHChangeNotify(
                SHCNE_ASSOCCHANGED as i32,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );
        }

        if found {
            println!("{}", i18n::tr("reg_uninstall_ctx_found"));
        } else {
            println!("{}", i18n::tr("reg_uninstall_ctx_not_found"));
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        anyhow::bail!(i18n::tr("reg_windows_only"))
    }
}
