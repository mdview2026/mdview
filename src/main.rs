#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod editor;
mod fonts;
mod i18n;
mod md4x;
mod registry;
mod renderer;
mod server;
mod watcher;
mod webview2;

use tao::{
    event::{Event, Event as TaoEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebContext};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use notify::Watcher;
use crate::server::{get_available_port, static_router, welcome_page_handler, welcome_action_handler, settings_page_handler, settings_status_handler, settings_action_handler, AppState, build_preview_router};
use crate::registry::{install_context_menu, uninstall_context_menu, remove_as_default_handler, set_as_default_handler};
use crate::watcher::{FileChange, watch_handler, update};
use crate::config::{load_config, save_config, add_recent_file, get_webview_data_directory};

fn load_icon() -> tao::window::Icon {
    let icon_bytes = include_bytes!("../static/icon.ico");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon memory")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    tao::window::Icon::from_rgba(rgba, width, height)
        .expect("Failed to open icon")
}

/// Hide the console window (Windows only)
#[cfg(windows)]
use wry::WebViewBuilderExtWindows;

#[cfg(windows)]
fn hide_console_window() {
    unsafe {
        use windows_sys::Win32::System::Console::GetConsoleWindow;
        use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};
        let window = GetConsoleWindow();
        if window != 0 {
            ShowWindow(window, SW_HIDE);
        }
    }
}

#[cfg(not(windows))]
fn hide_console_window() {}

/// Show a file picker dialog for the user to select an .exe file (Windows only)
#[cfg(windows)]
fn pick_exe_file() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use windows_sys::Win32::UI::Controls::Dialogs::{GetOpenFileNameW, OPENFILENAMEW, OFN_EXPLORER, OFN_FILEMUSTEXIST};

    let filter: Vec<u16> = OsString::from(format!("{}\0*.exe\0", i18n::tr("dialog_filter_exe"))).encode_wide().collect();
    let title: Vec<u16> = OsString::from(format!("{}\0", i18n::tr("dialog_title_editor"))).encode_wide().collect();

    let mut buf = vec![0u16; 520];
    let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
    ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.lpstrFilter = filter.as_ptr();
    ofn.lpstrFile = buf.as_mut_ptr();
    ofn.nMaxFile = buf.len() as u32;
    ofn.lpstrTitle = title.as_ptr();
    ofn.Flags = OFN_EXPLORER | OFN_FILEMUSTEXIST;

    unsafe {
        if GetOpenFileNameW(&mut ofn) != 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            let path = OsString::from_wide(&buf[..len]);
            let s = path.to_string_lossy().to_string();
            if s.is_empty() { None } else { Some(s) }
        } else {
            None
        }
    }
}

#[cfg(not(windows))]
fn pick_exe_file() -> Option<String> {
    None
}

/// Show help information
fn show_help() {
    let app = i18n::tr("app_name");
    let usage = i18n::tr("help_usage");
    let md_file = i18n::tr("help_arg_md_file");
    let desc_preview = i18n::tr("help_desc_preview");
    let desc_install = i18n::tr("help_desc_install");
    let desc_uninstall = i18n::tr("help_desc_uninstall");
    let desc_settings = i18n::tr("help_desc_settings");
    let desc_unbind = i18n::tr("help_desc_unbind");
    let desc_help = i18n::tr("help_desc_help");
    let env = i18n::tr("help_env");
    let port_label = i18n::tr("help_env_port_label");
    let env_port = i18n::tr("help_env_port");
    let env_md_html = i18n::tr("help_env_md_html");
    let output_label = i18n::tr("help_env_md_html_output_label");
    let env_md_html_output = i18n::tr("help_env_md_html_output");
    let example = i18n::tr("help_example");

    println!(
        "{app}\n\n{usage}\n    mdview <{md_file}>     {desc_preview}\n    mdview --install          {desc_install}\n    mdview --uninstall        {desc_uninstall}\n    mdview --settings         {desc_settings}\n    mdview --unbind           {desc_unbind}\n    mdview --help             {desc_help}\n\n{env}\n    PORT=<{port_label}>                  {env_port}\n    MD_HTML=1                      {env_md_html}\n    MD_HTML_OUTPUT=<{output_label}>          {env_md_html_output}\n\n{example}\n    mdview demo.md\n    set PORT=8080 && mdview demo.md\n"
    );
}

fn show_welcome_window() -> Result<()> {
    let rt = Box::leak(Box::new(tokio::runtime::Runtime::new()?));
    let port = get_available_port(3456).unwrap_or(3456);

    let in_bad_dir = crate::server::is_temp_or_download_dir();
    let bound = if !in_bad_dir {
        let result = set_as_default_handler();
        if result.is_ok() {
            println!("{}", i18n::tr("status_bound_md"));
            true
        } else {
            eprintln!("{}: {:?}", i18n::tr("status_auto_bind_failed"), result.err());
            false
        }
    } else {
        false
    };

    let app = Router::new()
        .route("/", get(welcome_page_handler))
        .route("/_welcome/action", get(welcome_action_handler))
        .merge(static_router())
        .layer(CorsLayer::permissive());

    let std_listener = std::net::TcpListener::bind(("127.0.0.1", port))?;
    let actual_port = std_listener.local_addr()?.port();
    std_listener.set_nonblocking(true)?;

    let _guard = rt.enter();
    let tokio_listener = tokio::net::TcpListener::from_std(std_listener)?;

    rt.spawn(async move {
        if let Err(e) = axum::serve(tokio_listener, app).await {
            eprintln!("{}", i18n::trf("status_welcome_server_error", &[&e.to_string()]));
        }
    });

    let url = format!(
        "http://127.0.0.1:{}/?bad_dir={}&bound={}",
        actual_port,
        if in_bad_dir { "1" } else { "0" },
        if bound { "1" } else { "0" }
    );

    hide_console_window();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_window_icon(Some(load_icon()))
        .with_title(i18n::tr("title_main"))
        .with_inner_size(tao::dpi::LogicalSize::new(560, 550))
        .with_resizable(false)
        .with_visible(false)
        .build(&event_loop)?;

    // Center the window
    if let Some(monitor) = window.current_monitor() {
        let monitor_size = monitor.size();
        let window_size = window.outer_size();
        let x = (monitor_size.width.saturating_sub(window_size.width)) / 2;
        let y = (monitor_size.height.saturating_sub(window_size.height)) / 2;
        window.set_outer_position(tao::dpi::PhysicalPosition::new(x as i32, y as i32));
    }
    window.set_visible(true);

    let mut web_context = WebContext::new(Some(get_webview_data_directory()));
    let mut webview_builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(&url)
        .with_devtools(true)
        .with_ipc_handler(move |msg| {
            let body = msg.into_body();
            if body.starts_with("open:") {
                let path = body[5..].to_string();
                let exe = std::env::current_exe().unwrap_or_else(|_| std::env::args().next().unwrap().into());
                let _ = std::process::Command::new(&exe).arg(&path).spawn();
                std::process::exit(0);
            }
            if body.starts_with("external:") {
                let url = body[9..].to_string();
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    let _ = std::process::Command::new("cmd")
                        .args(["/c", "start", "", &url])
                        .creation_flags(CREATE_NO_WINDOW)
                        .spawn();
                }
                #[cfg(not(windows))]
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                return;
            }
            if body.contains("settings") {
                std::process::exit(77);
            }
            std::process::exit(0);
        });
    #[cfg(windows)]
    {
        webview_builder = webview_builder.with_additional_browser_args(format!("--lang={}", i18n::current_lang().webview2_language()));
    }
    let _webview = webview_builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let TaoEvent::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            std::process::exit(0);
        }
    });
}

fn show_settings_window() -> Result<()> {
    let rt = Box::leak(Box::new(tokio::runtime::Runtime::new()?));
    let port = get_available_port(3456).unwrap_or(3456);

    let app = Router::new()
        .route("/_settings/status", get(settings_status_handler))
        .route(
            "/_settings/action",
            axum::routing::post(settings_action_handler),
        )
        .route("/", get(settings_page_handler))
        .merge(static_router())
        .layer(CorsLayer::permissive());

    let std_listener = std::net::TcpListener::bind(("127.0.0.1", port))?;
    let actual_port = std_listener.local_addr()?.port();
    std_listener.set_nonblocking(true)?;

    let _guard = rt.enter();
    let tokio_listener = tokio::net::TcpListener::from_std(std_listener)?;

    rt.spawn(async move {
        if let Err(e) = axum::serve(tokio_listener, app).await {
            eprintln!("{}", i18n::trf("status_settings_server_error", &[&e.to_string()]));
        }
    });

    let url = format!("http://127.0.0.1:{}", actual_port);

    hide_console_window();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_window_icon(Some(load_icon()))
        .with_title(format!("{} - {}", i18n::tr("app_name"), i18n::tr("title_settings")))
        .with_inner_size(tao::dpi::LogicalSize::new(560, 650))
        .with_resizable(true)
        .build(&event_loop)?;

    let mut web_context = WebContext::new(Some(get_webview_data_directory()));
    let mut webview_builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(&url)
        .with_devtools(true)
        .with_ipc_handler(|_| {
            std::process::exit(0);
        });
    #[cfg(windows)]
    {
        webview_builder = webview_builder.with_additional_browser_args(format!("--lang={}", i18n::current_lang().webview2_language()));
    }
    let _webview = webview_builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let TaoEvent::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            std::process::exit(0);
        }
    });
}

fn main() -> Result<()> {
    // Initialize i18n (must happen before any user-visible output)
    i18n::init();

    let args: Vec<String> = std::env::args().collect();

    // No arguments: double-click launch, show welcome page (auto-bind or prompt)
    if args.len() < 2 {
        // The welcome page itself relies on WebView2; ensure the runtime is ready first
        #[cfg(windows)]
        if !crate::webview2::check_or_prompt() {
            return Ok(());
        }
        return show_welcome_window();
    }

    // Handle command-line options
    if args[1].starts_with('-') {
        match args[1].as_str() {
            "--install" | "-i" => {
                #[cfg(windows)]
                {
                    install_context_menu()?;
                    return Ok(());
                }
                #[cfg(not(windows))]
                {
                    eprintln!("{}", i18n::tr("error_context_menu_windows_only"));
                    std::process::exit(1);
                }
            }
            "--uninstall" | "-u" => {
                #[cfg(windows)]
                {
                    uninstall_context_menu()?;
                    return Ok(());
                }
                #[cfg(not(windows))]
                {
                    eprintln!("{}", i18n::tr("error_context_menu_windows_only"));
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                show_help();
                return Ok(());
            }
            "--settings" => {
                // The settings window relies on WebView2; ensure the runtime is ready first
                #[cfg(windows)]
                if !crate::webview2::check_or_prompt() {
                    return Ok(());
                }
                return show_settings_window();
            }
            "--unbind" => {
                #[cfg(windows)]
                {
                    remove_as_default_handler()?;
                    return Ok(());
                }
                #[cfg(not(windows))]
                {
                    eprintln!("{}", i18n::tr("error_windows_only"));
                    std::process::exit(1);
                }
            }
            _ => {
                show_help();
                std::process::exit(1);
            }
        }
    }

    let md_file = PathBuf::from(&args[1]);
    if !md_file.exists() {
        eprintln!("{}", i18n::trf("error_file_not_found", &[&md_file.display().to_string()]));
        std::process::exit(1);
    }

    // The preview window relies on WebView2; prompt and exit if missing.
    #[cfg(windows)]
    if !crate::webview2::check_or_prompt() {
        return Ok(());
    }

    // Update recent files
    let mut config = load_config();
    add_recent_file(&mut config, &md_file.to_string_lossy());
    let _ = save_config(&config);

    // Canonicalize to an absolute path, using dunce to avoid Windows UNC path issues
    let md_file = dunce::canonicalize(&md_file).unwrap_or(md_file);
    let html_file = std::env::var("MD_HTML_OUTPUT")
        .ok()
        .filter(|v| !v.is_empty())
        .map(|v| PathBuf::from(v))
        .or_else(|| {
            if std::env::var("MD_HTML").as_deref() == Ok("1") {
                Some(md_file.with_extension("html"))
            } else {
                None
            }
        });

    // Hide the DOS console window
    #[cfg(windows)]
    hide_console_window();

    // Get the port
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .or_else(|| get_available_port(3456))
        .unwrap_or(3456);

    // Create the tokio runtime
    let rt = tokio::runtime::Runtime::new()?;

    // Create the broadcast channel
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(16);

    // Create the shared state
    let state = Arc::new(AppState {
        html: std::sync::RwLock::new(format!("<p>{}</p>", i18n::tr("status_loading"))),
        md_file: md_file.clone(),
        html_file,
        tx: tx.clone(),
    });

    // Initial render
    rt.block_on(async {
        update(&state).await;
    });

    // Create the file-change channel
    let (change_tx, change_rx) = tokio::sync::mpsc::unbounded_channel::<FileChange>();

    // Start the file-change handler task
    let state_for_watch = Arc::clone(&state);
    rt.spawn(async move {
        watch_handler(change_rx, state_for_watch).await;
    });

    // Start the file watcher - bridge via std channel
    let (std_tx, std_rx) = std::sync::mpsc::channel::<FileChange>();
    let change_tx_clone = change_tx.clone();

    // Forward messages from the std channel to the tokio channel
    rt.spawn_blocking(move || {
        while let Ok(change) = std_rx.recv() {
            if change_tx_clone.send(change).is_err() {
                break;
            }
        }
    });

    // Watch the directory (atomic saves on Windows can break single-file watches)
    let watch_dir = md_file.parent().unwrap().to_path_buf();
    let md_file_name = md_file.file_name().unwrap().to_str().unwrap().to_string();

    eprintln!(
        "{}",
        i18n::trf(
            "status_watching",
            &[&format!("{:?}", watch_dir), &md_file_name]
        )
    );

    // Unified file watcher
    let std_tx_file = std_tx.clone();
    let md_name_clone = md_file_name.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            // Check whether the target Markdown file is involved
            let is_md_file = event.paths.iter().any(|p| {
                p.file_name()
                    .map(|n| n.to_str().unwrap_or(""))
                    .unwrap_or("")
                    == md_name_clone
            });

            // Check whether an image file is involved
            let is_image_file = event.paths.iter().any(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| {
                        matches!(
                            e.to_lowercase().as_str(),
                            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp"
                        )
                    })
                    .unwrap_or(false)
            });

            let should_reload = matches!(
                event.kind,
                notify::EventKind::Modify(_) | notify::EventKind::Create(_) | notify::EventKind::Remove(_)
            );

            if should_reload {
                if is_md_file {
                    eprintln!("{}", i18n::tr("status_md_changed"));
                    let _ = std_tx_file.send(FileChange::Markdown);
                } else if is_image_file {
                    eprintln!("{}", i18n::tr("status_image_changed"));
                    let _ = std_tx_file.send(FileChange::Image);
                }
            }
        }
    })?;

    // Use a single watcher for the directory
    watcher.watch(&watch_dir, notify::RecursiveMode::NonRecursive)?;

    // Build the router
    let app = build_preview_router(state);

    // Start the server
    let std_listener = std::net::TcpListener::bind(("127.0.0.1", port))?;
    let actual_port = std_listener.local_addr()?.port();
    std_listener.set_nonblocking(true)?;

    let _guard = rt.enter();
    let tokio_listener = tokio::net::TcpListener::from_std(std_listener)?;

    rt.spawn(async move {
        if let Err(e) = axum::serve(tokio_listener, app).await {
            eprintln!("{}", i18n::trf("status_server_error", &[&e.to_string()]));
        }
    });

    let url = format!("http://127.0.0.1:{}", actual_port);
    println!(
        "{}",
        i18n::trf(
            "status_started",
            &[&md_file.display().to_string(), &url]
        )
    );

    // Launch the WebView window
    let event_loop = EventLoopBuilder::<String>::with_user_event().build();

    let mut window_builder = WindowBuilder::new()
        .with_window_icon(Some(load_icon()))
        .with_title(format!("{} - {}", md_file_name, i18n::tr("app_name")));

    if config.window_maximized {
        window_builder = window_builder.with_maximized(true);
    } else if config.window_width > 0 && config.window_height > 0 {
        window_builder = window_builder
            .with_inner_size(tao::dpi::PhysicalSize::new(config.window_width, config.window_height));
    } else {
        window_builder = window_builder
            .with_inner_size(tao::dpi::LogicalSize::new(1200, 800));
    }

    let window = window_builder.build(&event_loop)?;

    // Restore window position (only effective when not maximized)
    if !config.window_maximized {
        // Validate coordinates (exclude bogus values like -32000 from minimized state)
        let valid_pos = config.window_x > 0 && config.window_y > 0 
            && config.window_x < 10000 && config.window_y < 10000;
        
        let target_pos = if valid_pos {
            tao::dpi::PhysicalPosition::new(config.window_x, config.window_y)
        } else {
            // Coordinates invalid, force centering
            let monitor = window.current_monitor();
            let monitor_size = monitor.map(|m| m.size()).unwrap_or(tao::dpi::PhysicalSize::new(1920, 1080));
            let window_size = window.outer_size();
            let x = (monitor_size.width.saturating_sub(window_size.width)) / 2;
            let y = (monitor_size.height.saturating_sub(window_size.height)) / 2;
            tao::dpi::PhysicalPosition::new(x as i32, y as i32)
        };
        window.set_outer_position(target_pos);
    }

    let proxy = event_loop.create_proxy();

    let mut web_context = WebContext::new(Some(get_webview_data_directory()));
    let mut webview_builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(&url)
        .with_default_context_menus(false)
        .with_ipc_handler(move |msg| {
            let body = msg.into_body();
            if body.starts_with("open:") {
                let path = body[5..].to_string();
                let exe = std::env::current_exe().unwrap_or_else(|_| std::env::args().next().unwrap().into());
                let _ = std::process::Command::new(&exe).arg(&path).spawn();
                std::process::exit(0);
            }
            if body.starts_with("external:") {
                let url = body[9..].to_string();
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    let _ = std::process::Command::new("cmd")
                        .args(["/c", "start", "", &url])
                        .creation_flags(CREATE_NO_WINDOW)
                        .spawn();
                }
                #[cfg(not(windows))]
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                return;
            }
            if body == "browse:editor" {
                #[cfg(windows)]
                {
                    let p = proxy.clone();
                    std::thread::spawn(move || {
                        if let Some(path) = pick_exe_file() {
                            let _ = p.send_event(path);
                        }
                    });
                }
                return;
            }
            if body == "close-window" {
                // Ctrl+W: exit directly
                std::process::exit(0);
            }
            std::process::exit(0);
        });
    #[cfg(windows)]
    {
        webview_builder = webview_builder.with_additional_browser_args(format!("--lang={}", i18n::current_lang().webview2_language()));
    }
    let _webview = webview_builder.build(&window)?;

    // Run the UI event loop (blocks the main thread until the window closes)
    event_loop.run(move |event, _, control_flow| {
        let _ = &watcher; // keep watcher alive
        let _ = &rt; // keep runtime alive

        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(path) => {
                let js = format!(
                    "if(window.__onEditorPathSelected)window.__onEditorPathSelected('{}')",
                    path.replace('\\', "\\\\").replace('\'', "\\'")
                );
                let _ = _webview.evaluate_script(&js);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let mut config_to_save = load_config();
                let is_maximized = window.is_maximized();
                if !is_maximized {
                    let inner_size = window.inner_size();
                    let outer_pos = window.outer_position().unwrap_or(tao::dpi::PhysicalPosition::new(0, 0));
                    // Skip bogus coordinates from minimized state (-32000)
                    if outer_pos.x > 0 && outer_pos.y > 0 && outer_pos.x < 10000 && outer_pos.y < 10000 {
                        config_to_save.window_width = inner_size.width;
                        config_to_save.window_height = inner_size.height;
                        config_to_save.window_x = outer_pos.x;
                        config_to_save.window_y = outer_pos.y;
                    }
                }
                config_to_save.window_maximized = is_maximized;
                let _ = save_config(&config_to_save);
                std::process::exit(0);
            }
            _ => {}
        }
    });
}
