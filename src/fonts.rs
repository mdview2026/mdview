use std::sync::OnceLock;
use std::collections::HashSet;

#[cfg(windows)]
use winreg::{enums::*, RegKey};

/// Font registry path (checked under both HKLM and HKCU)
const FONT_REG_PATH: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts";

/// Common CJK fonts to prioritize (shown first when sorting)
const CJK_PRIORITY: &[&str] = &[
    "Microsoft YaHei",
    "SimSun",
    "SimHei",
    "FangSong",
    "KaiTi",
    "NSimSun",
    "DengXian",
    "Microsoft JhengHei",
    "MingLiU",
];

fn parse_font_names(raw_name: &str) -> Vec<String> {
    // Strip suffixes like (TrueType) / (OpenType) / (All Res)
    let name = raw_name.trim();
    let name = if let Some(pos) = name.rfind(" (") {
        name[..pos].trim().to_string()
    } else {
        name.to_string()
    };
    // Split multi-font entries like "SimSun & NSimSun"
    name.split('&')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn sort_fonts(fonts: &mut Vec<String>) {
    // Group by priority: CJK first, then the rest alphabetically
    let priority_set: HashSet<&str> = CJK_PRIORITY.iter().copied().collect();

    fonts.sort_by(|a, b| {
        let a_prio = priority_set.contains(a.as_str());
        let b_prio = priority_set.contains(b.as_str());
        match (a_prio, b_prio) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.to_lowercase().cmp(&b.to_lowercase()),
        }
    });

    // Within the CJK priority group, follow CJK_PRIORITY array order
    fonts.sort_by(|a, b| {
        let a_idx = CJK_PRIORITY.iter().position(|&x| x == a);
        let b_idx = CJK_PRIORITY.iter().position(|&x| x == b);
        match (a_idx, b_idx) {
            (Some(ai), Some(bi)) => ai.cmp(&bi),
            _ => std::cmp::Ordering::Equal,
        }
    });
}

/// Enumerate fonts under a single registry subkey
#[cfg(windows)]
fn read_fonts_from_key(key: &RegKey) -> Vec<String> {
    let mut names = HashSet::new();
    if let Ok(fonts_key) = key.open_subkey(FONT_REG_PATH) {
        for (raw_name, _) in fonts_key.enum_values().flatten() {
            for parsed in parse_font_names(&raw_name) {
                names.insert(parsed);
            }
        }
    }
    names.into_iter().collect()
}

/// Get the list of system-installed fonts (process-level cache, enumerated once)
pub fn enumerate_system_fonts() -> Vec<String> {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| {
        #[cfg(windows)]
        {
            let mut fonts = Vec::new();
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            fonts.extend(read_fonts_from_key(&hkcu));
            fonts.extend(read_fonts_from_key(&hklm));
            // Deduplicate
            let mut unique: Vec<String> = fonts.into_iter().collect();
            unique.sort();
            unique.dedup();
            sort_fonts(&mut unique);
            unique
        }
        #[cfg(not(windows))]
        {
            Vec::new()
        }
    }).clone()
}
