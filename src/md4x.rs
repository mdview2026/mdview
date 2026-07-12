//! md4x C library FFI bindings for Markdown → HTML rendering.
//!
//! Provides `render_html()` which calls md4x's `md_html_ex()` directly
//! via FFI, eliminating the need to spawn an external md4x.exe process.

use crate::i18n;
use std::ffi::c_void;
use std::slice;

// ── md4x parser flags ──

const MD_FLAG_PERMISSIVEEMAILAUTOLINKS: u32 = 0x0008;
const MD_FLAG_PERMISSIVEURLAUTOLINKS: u32 = 0x0004;
const MD_FLAG_PERMISSIVEWWWAUTOLINKS: u32 = 0x0400;
const MD_FLAG_TABLES: u32 = 0x0100;
const MD_FLAG_STRIKETHROUGH: u32 = 0x0200;
const MD_FLAG_TASKLISTS: u32 = 0x0800;
const MD_FLAG_LATEXMATHSPANS: u32 = 0x1000;
const MD_FLAG_WIKILINKS: u32 = 0x2000;
const MD_FLAG_UNDERLINE: u32 = 0x4000;
const MD_FLAG_FRONTMATTER: u32 = 0x10000;
const MD_FLAG_COMPONENTS: u32 = 0x20000;
const MD_FLAG_ATTRIBUTES: u32 = 0x40000;
const MD_FLAG_ALERTS: u32 = 0x80000;

const MD_DIALECT_ALL: u32 = MD_FLAG_PERMISSIVEEMAILAUTOLINKS
    | MD_FLAG_PERMISSIVEURLAUTOLINKS
    | MD_FLAG_PERMISSIVEWWWAUTOLINKS
    | MD_FLAG_TABLES
    | MD_FLAG_STRIKETHROUGH
    | MD_FLAG_TASKLISTS
    | MD_FLAG_LATEXMATHSPANS
    | MD_FLAG_WIKILINKS
    | MD_FLAG_UNDERLINE
    | MD_FLAG_FRONTMATTER
    | MD_FLAG_COMPONENTS
    | MD_FLAG_ATTRIBUTES
    | MD_FLAG_ALERTS;

// ── md4x HTML renderer flags ──

const MD_HTML_FLAG_FULL_HTML: u32 = 0x0008;
const MD_HTML_FLAG_SKIP_UTF8_BOM: u32 = 0x0004;

// ── FFI types ──

#[repr(C)]
pub struct MdHtmlOpts {
    pub title: *const i8,
    pub css_url: *const i8,
}

type MdChar = u8;
type MdSize = u32;

// ── FFI function ──

extern "C" {
    fn md_html_ex(
        input: *const MdChar,
        input_size: MdSize,
        process_output: Option<unsafe extern "C" fn(*const MdChar, MdSize, *mut c_void)>,
        userdata: *mut c_void,
        parser_flags: u32,
        renderer_flags: u32,
        opts: *const MdHtmlOpts,
    ) -> i32;
}

// ── Public API ──

/// Render Markdown text to a full HTML document string.
///
/// Equivalent to `md4x input.md -t html -f` (full HTML with frontmatter support).
pub fn render_html(input: &str) -> Result<String, String> {
    let mut output: Vec<u8> = Vec::new();

    unsafe extern "C" fn process_cb(
        data: *const MdChar,
        size: MdSize,
        userdata: *mut c_void,
    ) {
        let buf = &mut *(userdata as *mut Vec<u8>);
        let s = slice::from_raw_parts(data, size as usize);
        buf.extend_from_slice(s);
    }

    let userdata_ptr = &mut output as *mut Vec<u8> as *mut c_void;

    let ret = unsafe {
        md_html_ex(
            input.as_ptr(),
            input.len() as MdSize,
            Some(process_cb),
            userdata_ptr,
            MD_DIALECT_ALL,
            MD_HTML_FLAG_FULL_HTML | MD_HTML_FLAG_SKIP_UTF8_BOM,
            std::ptr::null(),
        )
    };

    if ret != 0 {
        return Err(i18n::tr("error_md4x_render").into());
    }

    String::from_utf8(output).map_err(|e| i18n::trf("error_md4x_not_utf8", &[&e.to_string()]))
}
