use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};
use std::net::TcpListener;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{sse::Event, IntoResponse, Response, Sse},
    routing::get,
    Router,
};
use tokio::sync::broadcast::Sender;
use tokio::time::Duration;
use tower_http::cors::CorsLayer;
use crate::config::{load_config, save_config};
use crate::i18n;
use crate::registry::{is_context_menu_installed, is_default_md_handler, set_as_default_handler, remove_as_default_handler, install_context_menu, uninstall_context_menu};
use crate::editor::{editor_available, open_editor_at_line};
use crate::renderer::{build_html, strip_markdown, STYLE_CSS, LOGO_PNG};

/// Global state
pub struct AppState {
    pub html: RwLock<String>,
    pub md_file: PathBuf,
    pub html_file: Option<PathBuf>,
    pub tx: Sender<String>,
}

/// Find an available port
pub fn get_available_port(start: u16) -> Option<u16> {
    (start..=65535).find(|&port| TcpListener::bind(("127.0.0.1", port)).is_ok())
}

pub fn is_temp_or_download_dir() -> bool {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return true,
    };
    let path_str = exe_path.to_string_lossy().to_lowercase();
    let patterns = [
        "\\temp\\",
        "\\tmp\\",
        "\\appdata\\local\\temp\\",
        "\\downloads\\",
    ];
    patterns.iter().any(|p| path_str.contains(p))
}

async fn style_css_handler() -> impl IntoResponse {
    ([("content-type", "text/css; charset=utf-8")], STYLE_CSS.to_vec()).into_response()
}

async fn logo_handler() -> impl IntoResponse {
    ([("content-type", "image/png")], LOGO_PNG.to_vec()).into_response()
}

pub fn static_router() -> Router {
    Router::new()
        .route("/_static/style.css", get(style_css_handler))
        .route("/_static/logo.png", get(logo_handler))
}

/// SSE endpoint - real-time refresh
pub async fn events_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.tx.subscribe();

    let stream = async_stream::try_stream! {
        yield Event::default().data("connected");

        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    yield Event::default().data(msg);
                }
                Err(_) => {
                    break;
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

/// Home page - returns HTML (with UTF-8 encoding)
pub async fn index_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let html = state.html.read().unwrap().clone();
    let mut wrapped = build_html(&html, true);
    
    let config = load_config();
    let is_default = is_default_md_handler();

    // Inject whether this is the default .md handler state into the frontend
    wrapped = wrapped.replace(
        "</body>",
        &format!(
            "<script>window.__mdIsDefaultHandler={};</script></body>",
            if is_default { "true" } else { "false" },
        ),
    );

    // Inject the user-selected body font (override --md-font only when config is non-empty)
    if !config.md_font.is_empty() {
        let escaped = config.md_font.replace('\\', "\\\\").replace('"', "\\\"");
        let font_style = format!(
            "<style>:root{{--md-font:\"{}\",sans-serif;}}</style>",
            escaped
        );
        if let Some(pos) = wrapped.find("</head>") {
            wrapped.insert_str(pos, &font_style);
        }
    }

    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(wrapped)
        .unwrap()
}

/// Handle image requests
pub async fn image_handler(
    State(state): State<Arc<AppState>>,
    path: axum::extract::Path<String>,
) -> impl IntoResponse {
    let path = path.as_str();
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let image_exts = ["png", "jpg", "jpeg", "gif", "svg", "webp", "bmp", "ico"];
    if !image_exts.contains(&ext.as_str()) {
        return (StatusCode::NOT_FOUND, Vec::new()).into_response();
    }

    let img_path = state.md_file.parent().unwrap().join(path);

    match tokio::fs::read(&img_path).await {
        Ok(data) => {
            let content_type = match ext.as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                "webp" => "image/webp",
                "bmp" => "image/bmp",
                "ico" => "image/x-icon",
                _ => "application/octet-stream",
            };
            ([("content-type", content_type)], data).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, Vec::new()).into_response(),
    }
}

/// Open source editor request
#[derive(serde::Deserialize)]
pub struct OpenSourceRequest {
    text: String,
}

/// Open source editor endpoint
pub async fn open_source_handler(
    State(state): State<Arc<AppState>>,
    axum::Json(req): axum::Json<OpenSourceRequest>,
) -> impl IntoResponse {
    // No selected text, open the file at line 1 directly
    if req.text.is_empty() {
        eprintln!("[editor] open {}:1 (no selected text)", state.md_file.display());
        return match open_editor_at_line(&state.md_file, 1) {
            Ok(()) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"ok": true, "line": 1})),
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(
                    serde_json::json!({"ok": false, "error": i18n::trf("msg_open_editor_failed", &[&e.to_string()])}),
                ),
            )
                .into_response(),
        };
    }

    let content = match tokio::fs::read_to_string(&state.md_file).await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(
                    serde_json::json!({"ok": false, "error": i18n::trf("msg_read_file_failed", &[&e.to_string()])}),
                ),
            )
                .into_response();
        }
    };

    let search = strip_markdown(&req.text);

    // Search line by line for a match
    let mut found_line = None;
    for (i, line) in content.lines().enumerate() {
        let stripped = strip_markdown(line);
        if stripped.contains(&search) || (search.contains(&stripped) && !stripped.is_empty()) {
            found_line = Some(i + 1); // 1-indexed
            break;
        }
    }

    let line = found_line.unwrap_or(1);
    eprintln!(
        "[editor] open {}:{} (search: {:?})",
        state.md_file.display(),
        line,
        &req.text[..req.text.len().min(50)]
    );

    match open_editor_at_line(&state.md_file, line) {
        Ok(()) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({"ok": true, "line": line})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({"ok": false, "error": i18n::trf("msg_open_editor_failed", &[&e.to_string()])})),
        )
            .into_response(),
    }
}


const SETTINGS_HTML: &str = r##"<!DOCTYPE html><html class="light" lang="{{i18n.lang_attr}}"><head>
<meta charset="utf-8">
<meta content="width=device-width, initial-scale=1.0" name="viewport">
<title>{{i18n.title_settings}} - {{i18n.app_name}}</title>
<link rel="stylesheet" href="/_static/style.css">
<style>
    body {
        font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif;
        background-color: #ffffff;
    }
    .editorial-shadow {
        box-shadow: 0 24px 48px -12px rgba(50, 50, 53, 0.04);
    }
</style>
</head>
<body class="bg-white text-on-surface selection:bg-tertiary-container/30 overflow-hidden h-screen flex flex-col">
<header class="pt-12 pb-6 px-8 text-center relative">
<div class="z-10">
<h1 class="text-3xl font-headline font-extrabold tracking-tight text-on-surface mb-2">{{i18n.settings_app_name}}</h1>
<p class="text-base text-on-surface-variant font-body">{{i18n.settings_app_desc}}</p>
</div>
</header>
<main class="flex-grow overflow-y-auto px-6 pb-10">
<div class="max-w-md mx-auto space-y-4">
<div class="bg-surface-container-lowest rounded-xl p-6 editorial-shadow">
<div class="flex items-center gap-3 mb-3">
<span class="text-primary text-2xl">&#128295;</span>
<h2 class="text-lg font-headline font-bold text-on-surface">{{i18n.settings_ctx_menu}}</h2>
<span id="badge-ctx" class="ml-auto text-xs font-label px-3 py-1 rounded-full">{{i18n.status_checking}}</span>
</div>
<p class="text-sm text-on-surface-variant font-body mb-4 leading-relaxed">{{i18n.settings_ctx_menu_desc}}</p>
<div class="flex gap-3">
<button id="btn-ctx-install" class="flex-1 py-2.5 bg-primary text-on-primary rounded-full font-label font-medium text-sm hover:opacity-90 transition-opacity cursor-pointer" onclick="doAction('install_ctx')">{{i18n.settings_add_ctx_menu}}</button>
<button id="btn-ctx-uninstall" class="flex-1 py-2.5 bg-surface-container-high text-error rounded-full font-label font-medium text-sm hover:bg-error-container/30 transition-colors cursor-pointer" onclick="doAction('uninstall_ctx')">{{i18n.settings_remove_ctx_menu}}</button>
</div>
<button id="btn-ctx-repair" class="w-full mt-3 py-2 bg-surface-container-high text-on-surface-variant rounded-full font-label font-medium text-sm hover:bg-surface-container transition-colors cursor-pointer" onclick="doAction('repair_ctx')">{{i18n.settings_repair_ctx_menu}}</button>
</div>
<div class="bg-surface-container-lowest rounded-xl p-6 editorial-shadow">
<div class="flex items-center gap-3 mb-3">
<span class="text-primary text-2xl">&#128193;</span>
<h2 class="text-lg font-headline font-bold text-on-surface">{{i18n.settings_default_association}}</h2>
<span id="badge-default" class="ml-auto text-xs font-label px-3 py-1 rounded-full">{{i18n.status_checking}}</span>
</div>
<p class="text-sm text-on-surface-variant font-body mb-4 leading-relaxed">{{i18n.settings_default_association_desc}}</p>
<button id="btn-default-set" class="w-full py-2.5 bg-tertiary-container text-white rounded-full font-label font-medium text-sm hover:opacity-90 transition-opacity cursor-pointer" onclick="doAction('set_default')">{{i18n.settings_set_default}}</button>
</div>
<div class="text-center mt-6 space-y-2">
<p class="text-xs text-on-surface-variant/60 font-body"><b class="text-on-surface-variant/80">{{i18n.settings_usage_title}}</b>{{i18n.settings_usage_ctx_menu}}</p>
<p class="text-xs text-on-surface-variant/60 font-body">{{i18n.settings_usage_cli}}</p>
</div>
</div>
</main>
<footer class="pb-6 pt-4 flex flex-col items-center gap-2">
<p class="text-sm font-label tracking-widest text-outline-variant opacity-60">github.com/mdview2026/mdview</p>
</footer>
<div id="toast" class="fixed top-5 right-5 px-5 py-3 rounded-xl text-sm font-label text-white z-[9999] opacity-0 transition-opacity duration-300 max-w-xs"></div>
<script>
const i18n = window.__mdI18n ? window.__mdI18n.strings : {};
function t(key, fallback) { return i18n[key] || fallback || key; }
function showToast(msg, ok) {
    const el = document.getElementById('toast');
    el.textContent = msg;
    el.style.background = ok ? '#43a047' : '#e53935';
    el.style.opacity = '1';
    setTimeout(() => { el.style.opacity = '0'; }, 2500);
}

function updateStatus(data) {
    const bc = document.getElementById('badge-ctx');
    bc.textContent = data.ctx_installed ? t('status_installed', 'Installed') : t('status_not_installed', 'Not installed');
    bc.className = 'ml-auto text-xs font-label px-3 py-1 rounded-full ' + (data.ctx_installed ? 'bg-primary-container/30 text-primary' : 'bg-surface-container-high text-on-surface-variant');

    const bd = document.getElementById('badge-default');
    bd.textContent = data.is_default ? t('status_set', 'Set') : t('status_not_set', 'Not set');
    bd.className = 'ml-auto text-xs font-label px-3 py-1 rounded-full ' + (data.is_default ? 'bg-primary-container/30 text-primary' : 'bg-surface-container-high text-on-surface-variant');

    const btnDefaultSet = document.getElementById('btn-default-set');

    btnCtxInstall.style.opacity = data.ctx_installed ? '0.4' : '1';
    btnCtxInstall.style.pointerEvents = data.ctx_installed ? 'none' : 'auto';
    btnCtxUninstall.style.opacity = data.ctx_installed ? '1' : '0.4';
    btnCtxUninstall.style.pointerEvents = data.ctx_installed ? 'auto' : 'none';
    btnDefaultSet.style.opacity = data.is_default ? '0.4' : '1';
    btnDefaultSet.style.pointerEvents = data.is_default ? 'none' : 'auto';
}

async function refreshStatus() {
    try {
        const r = await fetch('/_settings/status');
        const d = await r.json();
        updateStatus(d);
    } catch(e) { console.error(e); }
}

async function doAction(action) {
    try {
        const r = await fetch('/_settings/action', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({action})
        });
        const d = await r.json();
        showToast(d.message, d.ok);
        refreshStatus();
    } catch(e) { showToast(t('status_operation_failed', 'Operation failed') + ': ' + e.message, false); }
}

refreshStatus();
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        document.body.style.transition = 'opacity 0.15s';
        document.body.style.opacity = '0';
        setTimeout(function() { window.ipc.postMessage('exit'); }, 150);
    }
});
</script>
</body></html>"##;


#[derive(serde::Deserialize)]
pub struct SettingsAction {
    action: String,
}

pub async fn settings_status_handler() -> impl IntoResponse {
    let ctx = is_context_menu_installed();
    let default = is_default_md_handler();
    axum::Json(serde_json::json!({
        "ctx_installed": ctx,
        "is_default": default,
        "lang": i18n::current_lang().code(),
    }))
}

pub async fn settings_action_handler(axum::Json(req): axum::Json<SettingsAction>) -> impl IntoResponse {
    let result = if req.action == "install_ctx" {
        install_context_menu().map(|_| i18n::tr("msg_ctx_menu_added"))
    } else if req.action == "uninstall_ctx" {
        uninstall_context_menu().map(|_| i18n::tr("msg_ctx_menu_removed"))
    } else if req.action == "set_default" {
        set_as_default_handler().map(|_| i18n::tr("msg_set_default_success"))
    } else if req.action == "repair_ctx" {
        uninstall_context_menu().and_then(|_| install_context_menu()).map(|_| i18n::tr("msg_ctx_menu_added"))
    } else {
        Err(anyhow::anyhow!(i18n::tr("welcome_unknown_error")))
    };

    match result {
        Ok(msg) => axum::Json(serde_json::json!({"ok": true, "message": msg})),
        Err(e) => axum::Json(serde_json::json!({"ok": false, "message": format!("{}: {}", i18n::tr("msg_failed"), e)})),
    }
}

pub async fn unbind_handler() -> impl IntoResponse {
    match remove_as_default_handler() {
        Ok(_) => axum::Json(serde_json::json!({"ok": true, "message": i18n::tr("msg_unbound")})),
        Err(e) => axum::Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

fn render_page_html(html: &str) -> String {
    let html = i18n::render_template(html);
    let html = html.replace(
        "</head>",
        &format!(
            "<script>window.__mdI18n = {};function t(key,fallback){{return window.__mdI18n.strings[key]||fallback||key;}}</script></head>",
            i18n::frontend_json()
        ),
    );
    let version_html = format!(
        r#"<p class="text-xs text-outline-variant opacity-60 font-body">v{}</p>"#,
        env!("APP_VERSION")
    );
    html.replace("<!-- APP_VERSION -->", &version_html)
}

pub async fn settings_page_handler() -> impl IntoResponse {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(render_page_html(SETTINGS_HTML))
        .unwrap()
}

pub async fn welcome_page_handler(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    let in_bad_dir = params.get("bad_dir").map(|v| v == "1").unwrap_or(false);
    // Check binding status in real time rather than relying on URL parameters
    let is_bound = is_default_md_handler();

    let html = if in_bad_dir {
        let exe_path = std::env::current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| i18n::tr("welcome_unknown_error").into());
        format!(
            r##"<!DOCTYPE html><html class="light" lang="{{i18n.lang_attr}}"><head>
<meta charset="utf-8">
<meta content="width=device-width, initial-scale=1.0" name="viewport">
<title>{{i18n.title_security_warning}} - {{i18n.app_name}}</title>
<link rel="stylesheet" href="/_static/style.css">
<style>
    body {{
        font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif;
        background-color: #ffffff;
    }}
    .editorial-shadow {{
        box-shadow: 0 24px 48px -12px rgba(50, 50, 53, 0.04);
    }}
</style>
</head>
<body class="bg-white text-on-surface selection:bg-error-container/30 overflow-hidden h-screen flex flex-col">
<main class="flex-grow flex flex-col items-center justify-center px-8 relative">
<div class="z-10 w-full max-w-md flex flex-col items-center">
<div class="w-16 h-16 rounded-2xl bg-error-container/30 flex items-center justify-center mb-8">
<span class="text-error text-4xl">&#9888;&#65039;</span>
</div>
<h1 class="text-3xl font-headline font-extrabold tracking-tight text-on-surface mb-4 text-center">{{i18n.welcome_move_tool_title}}</h1>
<p class="text-base text-on-surface-variant font-body leading-relaxed mb-6 text-center">
                {{i18n.welcome_move_tool_desc}}
            </p>
<div class="w-full bg-surface-container-low rounded-xl p-4 mb-8 editorial-shadow">
<p class="text-xs font-label text-on-surface-variant mb-1 opacity-60">{{i18n.welcome_current_path}}</p>
<p class="text-sm font-body text-on-surface break-all">{exe_path}</p>
</div>
<button onclick="window.ipc.postMessage('exit')" class="px-8 py-3 bg-primary text-on-primary rounded-full font-label font-medium text-sm hover:opacity-90 transition-opacity cursor-pointer">
                {{i18n.welcome_got_it}}
            </button>
<a href="#" onclick="window.ipc.postMessage('external:https://github.com/mdview2026/mdview');return false;" class="text-sm text-primary font-body hover:underline mt-6">{{i18n.welcome_learn_more}}</a>
</div>
</main>
<footer class="pb-10 pt-4 flex flex-col items-center gap-1">
<!-- APP_VERSION -->
<p class="text-sm font-label tracking-widest text-outline-variant opacity-60">
            github.com/mdview2026/mdview
        </p>
</footer>
<script>
document.addEventListener('keydown', function(e) {{
    if (e.key === 'Escape') {{
        document.body.style.transition = 'opacity 0.15s';
        document.body.style.opacity = '0';
        setTimeout(function() {{ window.ipc.postMessage('exit'); }}, 150);
    }}
}});
</script>
</body></html>"##,
            exe_path = exe_path,
        )
    } else if is_bound {
        r##"<!DOCTYPE html><html class="light" lang="{{i18n.lang_attr}}"><head>
<meta charset="utf-8">
<meta content="width=device-width, initial-scale=1.0" name="viewport">
<title>{{i18n.title_welcome}}</title>
<link rel="stylesheet" href="/_static/style.css">
<style>
    body {
        font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif;
        background-color: #ffffff;
        position: relative;
        height: 100vh;
        overflow: hidden;
        margin: 0;
    }
</style>
</head>
<body class="bg-white text-on-surface">
<!-- Top area -->
<div style="position: absolute; top: 80px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center;">
    <img src="/_static/logo.png" alt="Logo" style="width: 80px; height: 80px; border-radius: 16px; margin-bottom: 24px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1);">
    <h1 class="text-3xl font-headline font-bold text-on-surface">{{i18n.settings_app_name}}</h1>
</div>
<!-- Middle area -->
<div style="position: absolute; top: 240px; left: 0; right: 0; text-align: center;">
    <div style="height: 24px;"></div>
    <div style="display: flex; align-items: center; justify-content: center; gap: 8px; margin-bottom: 24px;">
        <svg style="width: 20px; height: 20px; color: #22c55e;" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
        </svg>
        <span class="text-base text-on-surface font-body">{{i18n.welcome_bound_title}}</span>
    </div>
    <!-- Placeholder: simulates the first-line text height of the unbound state to align the second line -->
    <div style="height: 24px;"></div>
    <p class="text-sm text-on-surface-variant font-body leading-relaxed">{{i18n.welcome_bound_desc}}</p>
</div>
<!-- Bottom buttons -->
<div style="position: absolute; bottom: 80px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center;">
    <button onclick="doAction('unbind')" class="px-8 py-2.5 bg-surface-container-high text-on-surface-variant rounded-full font-body text-sm hover:bg-surface-container transition-colors cursor-pointer border border-outline-variant/30">
        {{i18n.welcome_unbind}}
    </button>
    <a href="#" onclick="window.ipc.postMessage('external:https://github.com/mdview2026/mdview');return false;" class="text-sm text-primary font-body hover:underline" style="margin-top: 24px;">{{i18n.welcome_learn_more}}</a>
</div>
<!-- footer -->
<div style="position: absolute; bottom: 24px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center; gap: 4px;">
    <!-- APP_VERSION -->
    <p class="text-xs font-body tracking-wider text-outline-variant opacity-60">github.com/mdview2026/mdview</p>
</div>
<script>
async function doAction(action) {
    console.log('action:', action);
    try {
        const r = await fetch('/_welcome/action?action=' + encodeURIComponent(action));
        console.log('status:', r.status);
        const d = await r.json();
        console.log('data:', d);
        if (d.ok) {
            // Operation succeeded, refresh the page
            location.reload();
        } else {
            alert(t('welcome_operation_failed', 'Operation failed') + ': ' + (d.error || t('welcome_unknown_error', 'Unknown error')));
        }
    } catch(e) {
        console.error('error:', e);
        alert(t('welcome_operation_failed', 'Operation failed') + ': ' + e.message);
    }
}
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        document.body.style.transition = 'opacity 0.15s';
        document.body.style.opacity = '0';
        setTimeout(function() { window.ipc.postMessage('exit'); }, 150);
    }
});
</script>
</body></html>"##.to_string()
    } else {
        r##"<!DOCTYPE html><html class="light" lang="{{i18n.lang_attr}}"><head>
<meta charset="utf-8">
<meta content="width=device-width, initial-scale=1.0" name="viewport">
<title>{{i18n.title_welcome}}</title>
<link rel="stylesheet" href="/_static/style.css">
<style>
    body {
        font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif;
        background-color: #ffffff;
        position: relative;
        height: 100vh;
        overflow: hidden;
        margin: 0;
    }
</style>
</head>
<body class="bg-white text-on-surface">
<!-- Top area -->
<div style="position: absolute; top: 80px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center;">
    <img src="/_static/logo.png" alt="Logo" style="width: 80px; height: 80px; border-radius: 16px; margin-bottom: 24px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1);">
    <h1 class="text-3xl font-headline font-bold text-on-surface">{{i18n.settings_app_name}}</h1>
</div>
<!-- Middle area -->
<div style="position: absolute; top: 240px; left: 0; right: 0; text-align: center;">
    <div style="height: 24px; margin-bottom: 24px;"></div>
    <p class="text-sm text-on-surface-variant font-body leading-relaxed">
        {{i18n.welcome_not_bound_title}}. {{i18n.welcome_not_bound_desc}}
    </p>
</div>
<!-- Bottom buttons -->
<div style="position: absolute; bottom: 80px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center;">
    <button onclick="doAction('set_default')" class="px-6 py-2.5 bg-tertiary-container text-white rounded-full font-body text-sm hover:opacity-90 transition-opacity cursor-pointer flex items-center gap-2 shadow-md">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
        </svg>
        {{i18n.welcome_set_default}}
    </button>
    <a href="https://github.com/mdview2026/mdview" target="_blank" class="text-sm text-primary font-body hover:underline" style="margin-top: 24px;">{{i18n.welcome_learn_more}}</a>
</div>
<!-- footer -->
<div style="position: absolute; bottom: 24px; left: 0; right: 0; display: flex; flex-direction: column; align-items: center; gap: 4px;">
    <!-- APP_VERSION -->
    <p class="text-xs font-body tracking-wider text-outline-variant opacity-60">github.com/mdview2026/mdview</p>
</div>
<script>
async function doAction(action) {
    console.log('action:', action);
    try {
        const r = await fetch('/_welcome/action?action=' + encodeURIComponent(action));
        console.log('status:', r.status);
        const d = await r.json();
        console.log('data:', d);
        if (d.ok) {
            // Operation succeeded, refresh the page
            location.reload();
        } else {
            alert(t('welcome_operation_failed', 'Operation failed') + ': ' + (d.error || t('welcome_unknown_error', 'Unknown error')));
        }
    } catch(e) {
        console.error('error:', e);
        alert(t('welcome_operation_failed', 'Operation failed') + ': ' + e.message);
    }
}
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        document.body.style.transition = 'opacity 0.15s';
        document.body.style.opacity = '0';
        setTimeout(function() { window.ipc.postMessage('exit'); }, 150);
    }
});
</script>
</body></html>"##.to_string()
    };

    let html = i18n::render_template(&html);
    let html = html.replace(
        "</head>",
        &format!(
            "<script>window.__mdI18n = {};</script></head>",
            i18n::frontend_json()
        ),
    );
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(render_page_html(&html))
        .unwrap()
}

pub async fn welcome_action_handler(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let action = params.get("action").cloned().unwrap_or_default();
    eprintln!("[welcome_action] action: {}", action);

    let result = match action.as_str() {
        "set_default" => {
            set_as_default_handler().map(|_| i18n::tr("msg_set_default_success"))
        }
        "unbind" => {
            remove_as_default_handler().map(|_| i18n::tr("msg_unbound"))
        }
        _ => Err(anyhow::anyhow!("{}: {}", i18n::tr("welcome_unknown_error"), action)),
    };

    match result {
        Ok(msg) => {
            eprintln!("[welcome_action] operation succeeded: {}", msg);
            axum::Json(serde_json::json!({"ok": true, "message": msg})).into_response()
        }
        Err(e) => {
            eprintln!("[welcome_action] operation failed: {}", e);
            axum::Json(serde_json::json!({"ok": false, "error": e.to_string()})).into_response()
        }
    }
}


/// Get editor config and list of available editors
pub async fn editor_get_handler() -> impl IntoResponse {
    let config = load_config();
    axum::Json(serde_json::json!({
        "current": config.editor,
        "available": {
            "subl": editor_available("subl"),
            "code": editor_available("code"),
            "notepad++": editor_available("notepad++"),
            "emeditor": editor_available("emeditor"),
            "notepad": true,
        }
    }))
}

/// Save editor config request
#[derive(serde::Deserialize)]
pub struct EditorSetRequest {
    editor: String,
}

/// Save editor config endpoint
pub async fn editor_set_handler(
    axum::Json(req): axum::Json<EditorSetRequest>,
) -> impl IntoResponse {
    let mut config = load_config();
    config.editor = req.editor;
    match save_config(&config) {
        Ok(()) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({"ok": true})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({"ok": false, "error": i18n::trf("msg_save_config_failed", &[&e.to_string()])})),
        ),
    }
}

/// Return list of system-installed fonts
pub async fn fonts_list_handler() -> impl IntoResponse {
    let fonts = crate::fonts::enumerate_system_fonts();
    axum::Json(serde_json::json!({
        "fonts": fonts,
    }))
}

/// Get current font config
pub async fn font_config_get_handler() -> impl IntoResponse {
    let config = load_config();
    axum::Json(serde_json::json!({
        "md_font": config.md_font,
    }))
}

/// Save font config request
#[derive(serde::Deserialize)]
pub struct FontConfigRequest {
    md_font: String,
}

/// Save font config endpoint
pub async fn font_config_set_handler(
    axum::Json(req): axum::Json<FontConfigRequest>,
) -> impl IntoResponse {
    let mut config = load_config();
    config.md_font = req.md_font;
    match save_config(&config) {
        Ok(()) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({"ok": true})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({"ok": false, "error": i18n::trf("msg_save_config_failed", &[&e.to_string()])})),
        ),
    }
}

pub fn build_preview_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/_events", get(events_handler))
        .route("/_open-source", axum::routing::post(open_source_handler))
        .route("/_editor", get(editor_get_handler).post(editor_set_handler))
        .route("/_fonts", get(fonts_list_handler))
        .route("/_font_config", get(font_config_get_handler).post(font_config_set_handler))
        .route("/_unbind", axum::routing::post(unbind_handler))
        .route("/*path", get(image_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
