//! WebView2 runtime dependency detection and installation guide.
//!
//! mdview renders content on Windows via wry (the WebView2 backend), which
//! requires the system to have the Microsoft Edge WebView2 Runtime installed.
//! This module checks at startup whether the runtime is ready; if missing, it
//! shows a prompt and offers a one-click download of the online installer
//! (`MicrosoftEdgeWebview2Setup.exe`).
//!
//! Modern systems (Win10/11 updated within the last couple of years, or those
//! with a recent Edge) usually already have this runtime built in, so the vast
//! majority of users will never see this prompt.

/// Download URL for the WebView2 Runtime online installer (the official
/// Microsoft Evergreen bootstrapper link).
///
/// It always points to the latest "Microsoft Edge WebView2 Runtime" online
/// installer (bootstrapper) and never expires over time; running it fetches
/// and installs the runtime matching the current system automatically.
use crate::i18n;

pub const DOWNLOAD_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

#[cfg(windows)]
use winreg::{enums::*, HKEY, RegKey};

/// Registry path of the WebView2 Runtime's EdgeUpdate client (product GUID is fixed).
#[cfg(windows)]
const CLIENT_KEY: &str = r"Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";

/// The same path under the 32-bit view (WOW6432Node) — typically written here for per-machine installs.
#[cfg(windows)]
const CLIENT_KEY_WOW64: &str = r"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";

/// Whether a usable WebView2 Runtime is installed on the system.
///
/// Determined via the EdgeUpdate client's `pv` (version) value: present,
/// non-empty, and not equal to `0.0.0.0` (the leftover value from an
/// uninstall/corruption) counts as installed. It checks the current user,
/// the per-machine 64-bit view, and the per-machine 32-bit view (WOW6432Node)
/// in turn, covering both per-user and per-machine installs.
pub fn runtime_available() -> bool {
    #[cfg(windows)]
    {
        pv_present(HKEY_CURRENT_USER, CLIENT_KEY)
            || pv_present(HKEY_LOCAL_MACHINE, CLIENT_KEY)
            || pv_present(HKEY_LOCAL_MACHINE, CLIENT_KEY_WOW64)
    }
    #[cfg(not(windows))]
    {
        true
    }
}

#[cfg(windows)]
fn pv_present(root: HKEY, path: &str) -> bool {
    RegKey::predef(root)
        .open_subkey(path)
        .and_then(|k| k.get_value::<String, _>("pv"))
        .map(|v| !v.is_empty() && v != "0.0.0.0")
        .unwrap_or(false)
}

/// Startup pre-check: returns `true` when the runtime is ready and the caller
/// can proceed; if missing it shows a prompt and returns `false`, and the
/// caller should exit immediately (to avoid continuing when rendering is impossible).
pub fn check_or_prompt() -> bool {
    if runtime_available() {
        return true;
    }
    prompt_to_install();
    false
}

/// Shows a native dialog box prompting that WebView2 is missing and asking
/// whether to download and install it now.
pub fn prompt_to_install() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            MessageBoxW, MB_ICONWARNING, MB_SETFOREGROUND, MB_TOPMOST, MB_YESNO, IDYES,
        };

        let title = i18n::tr("webview2_title");
        let msg = i18n::tr("webview2_message");

        let rc = unsafe {
            MessageBoxW(
                0,
                to_wide(msg).as_ptr(),
                to_wide(title).as_ptr(),
                MB_YESNO | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST,
            )
        };
        if rc == IDYES {
            open_url(DOWNLOAD_URL);
        }
    }
    #[cfg(not(windows))]
    {
        eprintln!(
            "{}",
            i18n::trf("webview2_missing_cli", &[DOWNLOAD_URL])
        );
    }
}

/// Converts an &str into a null-terminated UTF-16 sequence for Win32 APIs
/// that take wide-character pointers.
#[cfg(windows)]
fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Opens a URL using the default program (browser).
#[cfg(windows)]
fn open_url(url: &str) {
    use windows_sys::Win32::UI::Shell::ShellExecuteW;
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    let verb = to_wide("open");
    let wide_url = to_wide(url);
    unsafe {
        ShellExecuteW(
            0,
            verb.as_ptr(),
            wide_url.as_ptr(),
            std::ptr::null::<u16>(),
            std::ptr::null::<u16>(),
            SW_SHOWNORMAL,
        );
    }
}

