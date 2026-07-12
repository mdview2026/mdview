use std::sync::Arc;
use tokio::time::Duration;
use crate::i18n;
use crate::server::AppState;
use crate::renderer::{render_markdown, build_html};

#[derive(Clone, Copy, Debug)]
pub enum FileChange {
    Markdown,
    Image,
}

/// Re-render and broadcast a refresh
pub async fn update(state: &Arc<AppState>) {
    let now = chrono::Local::now().format("%H:%M:%S");
    println!("{}", i18n::trf("status_rendering", &[&now.to_string(), &state.md_file.display().to_string()]));

    match render_markdown(&state.md_file).await {
        Ok(html) => {
            *state.html.write().unwrap() = html;
            if let Some(ref path) = state.html_file {
                let static_html = build_html(&state.html.read().unwrap(), false);
                let _ = std::fs::write(path, &static_html);
                eprintln!("{}", i18n::trf("status_html_output", &[&now.to_string(), &path.display().to_string()]));
            }
            let _ = state.tx.send("reload".to_string());
        }
        Err(e) => {
            eprintln!("{}: {}", i18n::tr("error_render"), e);
            let error_html = format!(
                r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"></head>
<body style="color:red; padding:40px;">
  <h1>{}</h1>
  <pre>{}</pre>
</body></html>"#,
                i18n::tr("error_render_detail"),
                e
            );
            *state.html.write().unwrap() = error_html;
            let _ = state.tx.send("reload".to_string());
        }
    }
}

/// Handle file changes in the background
pub async fn watch_handler(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<FileChange>,
    state: Arc<AppState>,
) {
    eprintln!("{}", i18n::tr("status_watcher_started"));
    while let Some(change) = rx.recv().await {
        // Debug log kept as-is
        eprintln!("[watch] received change: {:?}", change);
        tokio::time::sleep(Duration::from_millis(100)).await;
        match change {
            FileChange::Markdown => {
                // Debug log kept as-is
                eprintln!("[watch] triggering Markdown update");
                update(&state).await;
            }
            FileChange::Image => {
                // Debug log kept as-is
                eprintln!("[watch] triggering image refresh");
                let _ = state.tx.send("reload".to_string());
            }
        }
    }
    eprintln!("{}", i18n::tr("status_watcher_stopped"));
}
