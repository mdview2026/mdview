use anyhow::{Context, Result};
use std::path::Path;

use crate::i18n;
use crate::md4x;

pub const STYLE_CSS: &[u8] = include_bytes!("../static/style.css");
pub const LOGO_PNG: &[u8] = include_bytes!("../static/logo_big.png");

const APP_VERSION: &str = include_str!("../VERSION");

const STYLE: &str = r#"
<style>
    :root {
        --md-bg: #ffffff;
        --md-text: #333333;
        --md-pre-bg: #f5f5f5;
        --md-code-bg: #f0f0f0;
        --md-bq-border: #dddddd;
        --md-bq-text: #666666;
        --md-tbl-border: #dddddd;
        --md-th-bg: #f5f5f5;
        --md-link: #2563eb;
        --md-link-hover: #1d4ed8;
        --toc-bg: rgba(255,255,255,0.96);
        --toc-border: #e0e0e0;
        --toc-text: #333;
        --toc-hover: #f0f0f0;
        --toc-sponsor: #aaa;
        --sponsor-card-bg: rgba(255,255,255,0.92);
        --sponsor-card-border: rgba(0,0,0,0.08);
        --sponsor-text: #333333;
        --sponsor-subtext: #666666;
        --sponsor-btn-bg: #f1f1f1;
        --sponsor-btn-text: #333333;
        --sponsor-qr-bg: rgba(240,240,240,0.5);
        --sponsor-qr-border: rgba(0,0,0,0.08);
        --sponsor-loading: #999999;
        --sponsor-overlay: rgba(255,255,255,0.96);
        --sponsor-amount-border: #e2e8f0;
        --sponsor-amount-active-bg: #eff6ff;
        --sponsor-amount-active-text: #2563EB;
        --sponsor-amount-active-unit: #60a5fa;
        --sponsor-divider: #f1f5f9;
    }
    [data-theme="dark"] {
        --md-bg: #1e1e1e;
        --md-text: #d4d4d4;
        --md-pre-bg: #2d2d2d;
        --md-code-bg: #3a3a3a;
        --md-bq-border: #555555;
        --md-bq-text: #aaaaaa;
        --md-tbl-border: #555555;
        --md-th-bg: #2d2d2d;
        --md-link: #60a5fa;
        --md-link-hover: #93c5fd;
        --toc-bg: rgba(30,30,30,0.96);
        --toc-border: #444;
        --toc-text: #ddd;
        --toc-hover: #444;
        --toc-sponsor: #777;
        --sponsor-card-bg: rgba(45,45,45,0.92);
        --sponsor-card-border: rgba(255,255,255,0.1);
        --sponsor-text: #dddddd;
        --sponsor-subtext: #aaaaaa;
        --sponsor-btn-bg: rgba(255,255,255,0.12);
        --sponsor-btn-text: #dddddd;
        --sponsor-qr-bg: rgba(255,255,255,0.06);
        --sponsor-qr-border: rgba(255,255,255,0.08);
        --sponsor-loading: #888888;
        --sponsor-overlay: rgba(30,30,30,0.96);
        --sponsor-amount-border: rgba(255,255,255,0.15);
        --sponsor-amount-active-bg: rgba(59,130,246,0.15);
        --sponsor-amount-active-text: #60a5fa;
        --sponsor-amount-active-unit: #93c5fd;
        --sponsor-divider: rgba(255,255,255,0.1);
    }
    body {
        max-width: var(--md-mw, 900px);
        margin: 0 auto;
        padding: 40px 20px 60vh 20px;
        font-family: var(--md-font, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif);
        font-size: var(--md-fs, 16px);
        line-height: 1.6;
        color: var(--md-text);
        background: var(--md-bg);
    }
    img { max-width: 100%; height: auto; }
    pre {
        background: var(--md-pre-bg);
        padding: 20px;
        font-family: "SimSun", "NSimSun", monospace;
        font-size: calc(var(--md-fs, 16px) * 0.875);
        line-height: 1.4;
        white-space: pre;
        overflow-x: auto;
        letter-spacing: 0px;
    }
    code { background: var(--md-code-bg); padding: 2px 6px 2px 0; border-radius: 3px; }
    blockquote { border-left: 4px solid var(--md-bq-border); margin: 0; padding-left: 16px; color: var(--md-bq-text); }
    table { border-collapse: collapse; width: 100%; }
    th, td { border: 1px solid var(--md-tbl-border); padding: 8px; text-align: left; }
    th { background: var(--md-th-bg); }
    /* Anchor jump spacing: leave a gap when a heading is scrolled to the top of the viewport to avoid it touching the edge */
    h1, h2, h3, h4, h5, h6 { scroll-margin-top: 24px; }
    /* Body links: use a brand blue matching the logo background instead of the browser's default harsh blue; remove underline */
    a { color: var(--md-link); text-decoration: none; }
    a:hover { color: var(--md-link-hover); }
    pre:has(> code.hljs) { background: transparent; }
    .__hljs_lang {
        position: absolute; top: 4px; right: 10px;
        font-size: 11px; color: var(--md-bq-text); opacity: .6;
        user-select: none; pointer-events: none;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        text-transform: lowercase;
    }
    @media print {
        #__md_edit_btn, #__md_ctx_menu { display: none !important; }
    }
</style>
"#;

const LIVE_RELOAD_SCRIPT: &str = r#"
<script>
    (function() {
        const evtSource = new EventSource('/_events');
        let reconnectTimer;
        evtSource.onmessage = (e) => {
            if (e.data === 'reload') {
                location.reload();
            }
        };
        evtSource.onopen = () => {
            clearTimeout(reconnectTimer);
        };
        evtSource.onerror = () => {
            reconnectTimer = setTimeout(() => location.reload(), 2000);
        };
    })();
</script>
"#;

const LIVE_RELOAD_SCRIPT_DEFERRED: &str = r#"
<script>
    document.addEventListener('DOMContentLoaded', function() {
        const evtSource = new EventSource('/_events');
        let reconnectTimer;
        evtSource.onmessage = (e) => {
            if (e.data === 'reload') {
                location.reload();
            }
        };
        evtSource.onopen = () => {
            clearTimeout(reconnectTimer);
        };
        evtSource.onerror = () => {
            reconnectTimer = setTimeout(() => location.reload(), 2000);
        };
    });
</script>
"#;

const THEME_SCRIPT: &str = r#"
<script>
(function() {
    var STORAGE_KEY = 'mdview-theme';
    function getSystemDark() {
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    function applyTheme(t) {
        document.documentElement.setAttribute('data-theme', t);
    }
    function getCurrent() {
        return document.documentElement.getAttribute('data-theme') || 'light';
    }
    function initTheme() {
        var saved = localStorage.getItem(STORAGE_KEY);
        applyTheme(saved || getSystemDark());
    }
    window.__toggleTheme = function() {
        var next = getCurrent() === 'dark' ? 'light' : 'dark';
        applyTheme(next);
        localStorage.setItem(STORAGE_KEY, next);
    };
    window.__isThemeDark = function() { return getCurrent() === 'dark'; };
    initTheme();
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {
        if (!localStorage.getItem(STORAGE_KEY)) {
            applyTheme(e.matches ? 'dark' : 'light');
        }
    });
})();
</script>
"#;

/// Automatically generates anchor ids for body headings and intercepts clicks on
/// in-document <a href="#..."> links so that Markdown [links](#heading) jump correctly.
///
/// The md4x C renderer outputs <h1>~<h6> without id attributes, so the browser cannot
/// locate href="#foo" to the corresponding heading. This script runs before the TOC
/// script, adds GitHub-style slug ids based on text content to headings that lack one,
/// and listens for anchor clicks to perform smooth scrolling.
const ANCHOR_SCRIPT: &str = r#"
<script>
(function() {
    function slugify(text) {
        return (text || '')
            .replace(/<[^>]+>/g, '')
            .trim()
            .toLowerCase()
            .replace(/[\s.]+/g, '-')
            .replace(/[^\p{L}\w-]+/gu, '')
            .replace(/-+/g, '-')
            .replace(/^-|-$/g, '');
    }

    function ensureHeadingIds() {
        var headings = document.querySelectorAll('h1,h2,h3,h4,h5,h6');
        var used = {};
        for (var i = 0; i < headings.length; i++) {
            var h = headings[i];
            if (h.id) { used[h.id] = true; continue; }
            var base = slugify(h.textContent);
            if (!base) continue;
            var id = base, n = 2;
            while (used[id]) { id = base + '-' + n; n++; }
            used[id] = true;
            h.id = id;
        }
    }

    function init() {
        ensureHeadingIds();
        document.addEventListener('click', function(e) {
            var a = e.target.closest && e.target.closest('a');
            if (!a) return;
            var href = a.getAttribute('href');
            if (!href || href.charAt(0) !== '#' || href.length < 2) return;
            if (a.closest('#__md_toc_panel')) return;
            var id;
            try { id = decodeURIComponent(href.slice(1)); } catch (err) { id = href.slice(1); }
            var target = document.getElementById(id);
            if (target) {
                e.preventDefault();
                target.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
</script>
"#;

const TOC_SCRIPT: &str = r#"
<script>
(function() {
    // Hide the TOC panel during printing and restore body width
    var printStyle = document.createElement('style');
    printStyle.textContent = '@media print { #__md_toc_panel { display: none !important; } body { padding-left: 0 !important; padding-right: 0 !important; } }';
    document.head.appendChild(printStyle);

    var POS_KEY = 'mdview-toc-pos';
    var POS_MODES = ['viewport-left', 'article-left', 'article-right', 'viewport-right'];
    var POS_ICON = '&#8644;';
    var POS_TITLES = {'viewport-left': t('toc_pos_viewport_left', 'Far left'), 'article-left': t('toc_pos_article_left', 'Article left'), 'article-right': t('toc_pos_article_right', 'Article right'), 'viewport-right': t('toc_pos_viewport_right', 'Far right')};

    function getSavedPos() {
        var saved = localStorage.getItem(POS_KEY);
        return POS_MODES.indexOf(saved) >= 0 ? saved : 'viewport-left';
    }

    function initToc() {
        var allHeadings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
        var headings = [];
        allHeadings.forEach(function(h) {
            headings.push(h);
        });
        if (headings.length < 2) return;

        var panelBg = 'var(--toc-bg)';
        var panelBorder = 'var(--toc-border)';
        var textColor = 'var(--toc-text)';
        var linkHover = 'var(--toc-hover)';
        var activeColor = '#2196F3';

        headings.forEach(function(h, i) {
            if (!h.id) h.id = '__toc_h_' + i;
        });

        var mode = getSavedPos();

        var panel = document.createElement('div');
        panel.id = '__md_toc_panel';
        var panelStyle = {
            position: 'fixed',
            background: panelBg,
            zIndex: '999997',
            fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
            fontSize: '13px',
            transition: 'transform 0.25s ease, width 0.25s ease'
        };
        // All four positions use a full-height sidebar (top to bottom, square corners); left/right border switches based on whether the bar is on the left/right side of the content
        Object.assign(panelStyle, {
            top: '0',
            bottom: '0',
            width: (parseInt(localStorage.getItem('mdview-toc-width')) || 260) + 'px',
            borderRadius: '0',
            padding: '0',
            boxShadow: 'none'
        });
        Object.assign(panel.style, panelStyle);
        panel.style.display = 'flex';
        panel.style.flexDirection = 'column';
        applyBorderStyle();

        // ---- Panel header (TOC title + position switch) ----
        var header = document.createElement('div');
        Object.assign(header.style, {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            padding: '8px 12px',
            borderBottom: '1px solid ' + panelBorder,
            marginBottom: '4px',
            opacity: '0.8',
            flexShrink: '0'
        });
        var titleSpan = document.createElement('span');
        titleSpan.textContent = t('toc_title', 'Contents');
        Object.assign(titleSpan.style, { fontWeight: '600', fontSize: '13px', color: textColor });
        var posBtn = document.createElement('span');
        posBtn.innerHTML = POS_ICON;
        posBtn.title = t('toc_pos_title', 'Current position: {0} (click to switch)').replace('{0}', POS_TITLES[mode]);
        Object.assign(posBtn.style, { cursor: 'pointer', fontSize: '12px', userSelect: 'none' });
        posBtn.addEventListener('click', function(e) {
            e.stopPropagation();
            var idx = POS_MODES.indexOf(mode);
            mode = POS_MODES[(idx + 1) % POS_MODES.length];
            localStorage.setItem(POS_KEY, mode);
            posBtn.innerHTML = POS_ICON;
            posBtn.title = t('toc_pos_title', 'Current position: {0} (click to switch)').replace('{0}', POS_TITLES[mode]);
            applyBorderStyle();
            recalcPosition();
            if (!expanded) {
                panel.style.transform = getHideTransform();
            }
        });
        header.appendChild(titleSpan);
        header.appendChild(posBtn);
        panel.appendChild(header);
        // ---- end header ----

        var inner = document.createElement('div');
        Object.assign(inner.style, {
            flex: '1',
            minHeight: '0',
            overflowY: 'auto',
            padding: '8px 0'
        });
        // Wheel scrolls the TOC itself first without bubbling to the main page (fixes overflow:auto inside fixed panel not scrolling under WebView2)
        inner.addEventListener('wheel', function (e) {
            if (e.ctrlKey) return;                                              // Let page zoom handle it when Ctrl is held
            if (inner.scrollHeight - inner.clientHeight <= 0) return;           // If TOC doesn't overflow, let the page scroll
            e.preventDefault();
            inner.scrollTop += e.deltaY;
        }, { passive: false });

        panel.appendChild(inner);

        var ul = document.createElement('ul');
        ul.style.cssText = 'list-style:none;margin:0;padding:0;';
        var levelStyles = [
            { size: '14px', weight: 'bold', pad: 10 },   // h1
            { size: '13px', weight: '600', pad: 10 },    // h2
            { size: '12px', weight: 'normal', pad: 22 }, // h3
            { size: '11px', weight: 'normal', pad: 34 }, // h4
            { size: '11px', weight: 'normal', pad: 44 }, // h5
            { size: '11px', weight: 'normal', pad: 44 }  // h6
        ];
        headings.forEach(function(h) {
            var li = document.createElement('li');
            var a = document.createElement('a');
            a.href = '#' + h.id;
            a.textContent = h.textContent;
            var level = parseInt(h.tagName[1]);
            var st = levelStyles[level - 1] || levelStyles[5];
            Object.assign(a.style, {
                display: 'block',
                padding: '3px 8px 3px ' + st.pad + 'px',
                color: textColor,
                textDecoration: 'none',
                whiteSpace: 'nowrap',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                borderLeft: '2px solid transparent',
                transition: 'all 0.15s',
                fontSize: st.size,
                fontWeight: st.weight,
                lineHeight: '1.5'
            });
            a.addEventListener('mouseenter', function() { a.style.background = linkHover; });
            a.addEventListener('mouseleave', function() { a.style.background = 'transparent'; });
            a.addEventListener('click', function(e) {
                e.preventDefault();
                h.scrollIntoView({ behavior: 'smooth', block: 'start' });
            });
            li.appendChild(a);
            ul.appendChild(li);
        });
        inner.appendChild(ul);

        // —— Bottom fixed area: website entry pinned to the very bottom of the panel ——
        var footer = document.createElement('div');
        Object.assign(footer.style, {
            flexShrink: '0',
            borderTop: '1px solid ' + panelBorder,
            paddingTop: '6px',
            paddingBottom: '6px'
        });

        // Website entry (community edition), shown greyed out
        var websiteLi = document.createElement('div');
        var websiteA = document.createElement('a');
        websiteA.textContent = t('toc_visit_website', 'Visit website');
        Object.assign(websiteA.style, {
            display: 'block',
            padding: '4px 8px 4px 14px',
            color: 'var(--toc-sponsor)',
            textDecoration: 'none',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            fontSize: '11px',
            fontWeight: 'normal',
            lineHeight: '1.5',
            cursor: 'pointer'
        });
        websiteA.addEventListener('mouseenter', function() { websiteA.style.background = linkHover; });
        websiteA.addEventListener('mouseleave', function() { websiteA.style.background = 'transparent'; });
        websiteA.addEventListener('click', function(e) {
            e.preventDefault();
            window.ipc.postMessage('external:https://github.com/mdview2026/mdview');
        });
        websiteLi.appendChild(websiteA);
        footer.appendChild(websiteLi);

        // Star on GitHub entry — opens the repo in the browser
        var starLi = document.createElement('div');
        var starA = document.createElement('a');
        starA.textContent = t('toc_star_on_github', 'Star on GitHub');
        Object.assign(starA.style, {
            display: 'block',
            padding: '4px 8px 4px 14px',
            color: 'var(--toc-sponsor)',
            textDecoration: 'none',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            fontSize: '11px',
            fontWeight: 'normal',
            lineHeight: '1.5',
            cursor: 'pointer'
        });
        starA.addEventListener('mouseenter', function() { starA.style.background = linkHover; });
        starA.addEventListener('mouseleave', function() { starA.style.background = 'transparent'; });
        starA.addEventListener('click', function(e) {
            e.preventDefault();
            window.ipc.postMessage('external:https://github.com/mdview2026/mdview');
        });
        starLi.appendChild(starA);
        footer.appendChild(starLi);

        panel.appendChild(footer);

        var expanded = localStorage.getItem('mdview-toc-visible') !== '0';
        // When the bar is on the right side of content, draw the border on the left; otherwise on the right (refreshed synchronously when switching position)
        function applyBorderStyle() {
            var r = mode === 'viewport-right' || mode === 'article-right';
            panel.style.borderRight = r ? 'none' : ('1px solid ' + panelBorder);
            panel.style.borderLeft = r ? ('1px solid ' + panelBorder) : 'none';
        }

        function getHideTransform() {
            var rect = panel.getBoundingClientRect();
            if (mode === 'viewport-left') {
                return 'translateX(-' + (panel.offsetWidth + 20) + 'px)';
            }
            if (mode === 'viewport-right') {
                return 'translateX(' + (panel.offsetWidth + 20) + 'px)';
            }
            if (mode === 'article-right') {
                return 'translateX(' + (window.innerWidth - rect.left + 20) + 'px)';
            }
            return 'translateX(-' + (rect.right + 20) + 'px)'; // article-left: push off-screen to the left
        }

        document.body.appendChild(panel);

        // ---- Position calculation ----
        function recalcPosition() {
            var tocW = panel.offsetWidth;
            var gap = 4;
            var winW = window.innerWidth;
            var bodyRect = document.body.getBoundingClientRect();
            // First clear body side padding, then reset per current mode
            document.body.style.paddingLeft = '';
            document.body.style.paddingRight = '';

            if (mode === 'viewport-left') {
                panel.style.left = '0';
                panel.style.right = 'auto';
                if (expanded) document.body.style.paddingLeft = tocW + 'px';
            } else if (mode === 'viewport-right') {
                panel.style.right = '0';
                panel.style.left = 'auto';
                if (expanded) document.body.style.paddingRight = tocW + 'px';
            } else if (mode === 'article-left') {
                var left = bodyRect.left - tocW - gap;
                if (left < gap) left = gap;
                panel.style.left = left + 'px';
                panel.style.right = 'auto';
            } else if (mode === 'article-right') {
                var left = bodyRect.right + gap;
                if (left + tocW > winW - gap) {
                    panel.style.left = 'auto';
                    panel.style.right = gap + 'px';
                } else {
                    panel.style.left = left + 'px';
                    panel.style.right = 'auto';
                }
            }
        }
        window.addEventListener('resize', recalcPosition);
        // ---- end position calculation ----

        // ---- Drag to resize TOC width ----
        var resizerL = document.createElement('div');
        var resizerR = document.createElement('div');
        var resizerStyle = {
            position: 'absolute',
            top: '12px',
            width: '6px',
            height: 'calc(100% - 24px)',
            cursor: 'col-resize',
            zIndex: '1'
        };
        Object.assign(resizerL.style, Object.assign({}, resizerStyle, { left: '-3px' }));
        Object.assign(resizerR.style, Object.assign({}, resizerStyle, { right: '-3px' }));
        panel.appendChild(resizerL);
        panel.appendChild(resizerR);

        var isResizing = false;
        var resizeSide = 'right';
        var startX = 0;
        var startWidth = 0;
        var MIN_W = 120;
        var MAX_W = 500;

        function onResizerDown(side, e) {
            isResizing = true;
            resizeSide = side;
            startX = e.clientX;
            startWidth = panel.offsetWidth;
            document.body.style.cursor = 'col-resize';
            document.body.style.userSelect = 'none';
            e.preventDefault();
        }
        resizerL.addEventListener('mousedown', function(e) { onResizerDown('left', e); });
        resizerR.addEventListener('mousedown', function(e) { onResizerDown('right', e); });

        document.addEventListener('mousemove', function(e) {
            if (!isResizing) return;
            var delta = e.clientX - startX;
            var newWidth = resizeSide === 'left' ? startWidth - delta : startWidth + delta;
            if (newWidth < MIN_W) newWidth = MIN_W;
            if (newWidth > MAX_W) newWidth = MAX_W;
            panel.style.width = newWidth + 'px';
            recalcPosition();
        });

        document.addEventListener('mouseup', function() {
            if (isResizing) {
                isResizing = false;
                document.body.style.cursor = '';
                document.body.style.userSelect = '';
                localStorage.setItem('mdview-toc-width', panel.offsetWidth);
            }
        });
        // ---- end drag ----

        function highlight() {
            var current = null;
            for (var i = headings.length - 1; i >= 0; i--) {
                var rect = headings[i].getBoundingClientRect();
                if (rect.top <= 120) {
                    current = headings[i].id;
                    break;
                }
            }
            var links = ul.querySelectorAll('a');
            for (var j = 0; j < links.length; j++) {
                var a = links[j];
                var hrefAttr = a.getAttribute('href');
                if (!hrefAttr) continue;
                var href = hrefAttr.slice(1);
                if (href === current) {
                    a.style.borderLeftColor = activeColor;
                    a.style.color = activeColor;
                } else {
                    a.style.borderLeftColor = 'transparent';
                    a.style.color = textColor;
                }
            }
        }
        window.addEventListener('scroll', highlight);
        highlight();

        // Expose global interface for the right-click menu to call
        window.__mdTocSetPos = function(newMode) {
            if (POS_MODES.indexOf(newMode) < 0) return;
            mode = newMode;
            localStorage.setItem(POS_KEY, mode);
            applyBorderStyle();
            recalcPosition();
        };

        window.__mdTocToggle = function() {
            expanded = !expanded;
            localStorage.setItem('mdview-toc-visible', expanded ? '1' : '0');
            panel.style.transform = expanded ? 'translateX(0)' : getHideTransform();
            recalcPosition();
        };

        // Initialize position and visibility
        recalcPosition();
        if (!expanded) {
            panel.style.transform = getHideTransform();
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initToc);
    } else {
        initToc();
    }
})();
</script>
"#;

const EDIT_SCRIPT: &str = r#"
<script>
(function() {
    // === Core function: edit source ===
    function editSource() {
        const sel = window.getSelection().toString().trim();
        fetch('/_open-source', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ text: sel })
        }).catch(e => console.error(t('edit_open_failed', 'Failed to open source:'), e));
    }

    // === Ctrl+E shortcut ===
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && (e.key === 'e' || e.key === 'E')) {
            e.preventDefault();
            editSource();
        }
    });

    // === Ctrl+W shortcut (close window) ===
    // WebView2 intercepts Ctrl+W as a built-in shortcut; we must catch it at the JS layer and notify Rust to close via IPC
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && (e.key === 'w' || e.key === 'W')) {
            e.preventDefault();
            window.ipc.postMessage('close-window');
        }
    });

    var isDark = window.__isThemeDark ? window.__isThemeDark() : false;

    // === Floating edit button in the bottom-right corner ===
    var btn = document.createElement('div');
    btn.id = '__md_edit_btn';
    btn.innerHTML = '&#9998;';
    btn.title = t('edit_btn_tooltip', 'Edit source (Ctrl+E)');
    var btnBg = isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.08)';
    var btnColor = isDark ? '#aaa' : '#666';
    var btnHoverBg = isDark ? 'rgba(255,255,255,0.2)' : 'rgba(0,0,0,0.15)';
    var btnHoverColor = isDark ? '#ddd' : '#333';
    Object.assign(btn.style, {
        position: 'fixed', bottom: '20px', right: '20px', zIndex: '999998',
        width: '36px', height: '36px', borderRadius: '50%',
        background: btnBg, color: btnColor,
        display: 'flex', alignItems: 'center', justifyContent: 'center',
        fontSize: '16px', cursor: 'pointer', userSelect: 'none',
        transition: 'background 0.2s, color 0.2s',
        boxShadow: '0 1px 4px rgba(0,0,0,0.1)'
    });
    btn.addEventListener('mouseenter', () => {
        btn.style.background = btnHoverBg;
        btn.style.color = btnHoverColor;
    });
    btn.addEventListener('mouseleave', () => {
        btn.style.background = btnBg;
        btn.style.color = btnColor;
    });
    btn.addEventListener('click', (e) => {
        e.stopPropagation();
        editSource();
    });
    document.body.appendChild(btn);

    // === File-association status floating button ===
    (function() {
        if (!window.__mdIsDefaultHandler) return;
        var bindBtn = document.createElement('div');
        bindBtn.id = '__md_bind_btn';
        bindBtn.innerHTML = '&#128279;';
        bindBtn.title = t('bind_btn_tooltip', 'File association settings');
        var bindBtnBg = isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.08)';
        var bindBtnColor = isDark ? '#aaa' : '#666';
        var bindBtnHoverBg = isDark ? 'rgba(255,255,255,0.2)' : 'rgba(0,0,0,0.15)';
        var bindBtnHoverColor = isDark ? '#ddd' : '#333';
        Object.assign(bindBtn.style, {
            position: 'fixed', bottom: '20px', right: '64px', zIndex: '999998',
            width: '36px', height: '36px', borderRadius: '50%',
            background: bindBtnBg, color: bindBtnColor,
            display: 'flex', alignItems: 'center', justifyContent: 'center',
            fontSize: '16px', cursor: 'pointer', userSelect: 'none',
            transition: 'background 0.2s, color 0.2s',
            boxShadow: '0 1px 4px rgba(0,0,0,0.1)'
        });
        bindBtn.addEventListener('mouseenter', () => {
            bindBtn.style.background = bindBtnHoverBg;
            bindBtn.style.color = bindBtnHoverColor;
        });
        bindBtn.addEventListener('mouseleave', () => {
            bindBtn.style.background = bindBtnBg;
            bindBtn.style.color = bindBtnColor;
        });

        var card = document.createElement('div');
        card.id = '__md_bind_card';
        var cardBg = isDark ? '#2d2d2d' : '#fff';
        var cardBorder = isDark ? '#555' : '#ccc';
        var cardText = isDark ? '#ddd' : '#333';
        var cardSub = isDark ? '#aaa' : '#666';
        Object.assign(card.style, {
            position: 'fixed', bottom: '64px', right: '20px', zIndex: '999998',
            background: cardBg, border: '1px solid ' + cardBorder, borderRadius: '12px',
            padding: '20px', width: '280px', display: 'none',
            boxShadow: '0 4px 20px rgba(0,0,0,0.15)', fontFamily: 'Segoe UI, sans-serif',
            fontSize: '14px', color: cardText
        });
        card.innerHTML = '<div style="margin-bottom:12px;"><div style="font-weight:600;margin-bottom:4px;">&#128279; ' + t('bind_card_title', '.md file association bound') + '</div><div style="font-size:13px;color:' + cardSub + ';">' + t('bind_card_desc', 'Double-click a .md file to open it with this tool') + '</div></div><button id="__md_bind_unbind" style="width:100%;padding:8px 0;background:#e53935;color:#fff;border:none;border-radius:6px;cursor:pointer;font-size:13px;">' + t('bind_unbind', 'Unbind') + '</button>';

        document.body.appendChild(bindBtn);
        document.body.appendChild(card);

        var cardVisible = false;
        bindBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            cardVisible = !cardVisible;
            card.style.display = cardVisible ? 'block' : 'none';
        });

        document.addEventListener('click', (e) => {
            if (!card.contains(e.target) && e.target !== bindBtn) {
                cardVisible = false;
                card.style.display = 'none';
            }
        });

        document.getElementById('__md_bind_unbind').addEventListener('click', (e) => {
            e.stopPropagation();
            var unbindBtn = document.getElementById('__md_bind_unbind');
            unbindBtn.textContent = t('bind_processing', 'Processing...');
            unbindBtn.disabled = true;
            fetch('/_unbind', {method: 'POST'})
                .then(r => r.json())
                .then(d => {
                    if (d.ok) {
                        card.innerHTML = '<div style="text-align:center;padding:10px 0;"><div style="font-size:28px;margin-bottom:8px;">&#9989;</div><div style="font-weight:600;margin-bottom:4px;">' + t('bind_unbind_success_title', 'Unbound') + '</div><div style="font-size:13px;color:' + cardSub + ';">' + t('bind_unbind_success_desc', '.md files will use the previously saved handler') + '</div></div>';
                    } else {
                        unbindBtn.textContent = t('bind_unbind', 'Unbind');
                        unbindBtn.disabled = false;
                        alert(t('bind_unbind_failed', 'Failed to unbind') + ': ' + (d.error || t('welcome_unknown_error', 'Unknown error')));
                    }
                })
                .catch(err => {
                    unbindBtn.textContent = t('bind_unbind', 'Unbind');
                    unbindBtn.disabled = false;
                    alert(t('bind_unbind_failed', 'Failed to unbind') + ': ' + err.message);
                });
        });
    })();

    // === Custom right-click menu ===
    var menuBg = isDark ? '#2d2d2d' : '#fff';
    var menuBorder = isDark ? '#555' : '#ccc';
    var menuShadow = isDark ? 'rgba(0,0,0,0.4)' : 'rgba(0,0,0,0.15)';
    var menuSep = isDark ? '#444' : '#e0e0e0';
    var menuItemColor = isDark ? '#ddd' : '#333';
    var menuItemHover = isDark ? '#3a3a3a' : '#e8e8e8';

    const menu = document.createElement('div');
    menu.id = '__md_ctx_menu';
    Object.assign(menu.style, {
        position: 'fixed', display: 'none', zIndex: '999999',
        background: menuBg, border: '1px solid ' + menuBorder, borderRadius: '6px',
        boxShadow: '0 4px 12px ' + menuShadow, padding: '4px 0',
        minWidth: '160px', fontFamily: 'Segoe UI, sans-serif', fontSize: '13px',
        userSelect: 'none'
    });

    function addItem(label, action, separator) {
        if (separator) {
            const sep = document.createElement('div');
            Object.assign(sep.style, { height: '1px', background: menuSep, margin: '4px 0' });
            menu.appendChild(sep);
        }
        const item = document.createElement('div');
        item.textContent = label;
        Object.assign(item.style, {
            padding: '6px 24px', cursor: 'pointer', color: menuItemColor, whiteSpace: 'nowrap'
        });
        item.addEventListener('mouseenter', () => { item.style.background = menuItemHover; });
        item.addEventListener('mouseleave', () => { item.style.background = 'transparent'; });
        item.addEventListener('click', (e) => { e.stopPropagation(); action(); menu.style.display = 'none'; });
        menu.appendChild(item);
    }

    addItem(t('menu_copy', 'Copy'), () => { document.execCommand('copy'); });
    addItem(t('menu_select_all', 'Select all'), () => { window.getSelection().selectAllChildren(document.body); });
    addItem(t('menu_export_pdf', 'Export PDF'), () => { window.print(); });
    var themeToggleItem = document.createElement('div');
    themeToggleItem.textContent = (window.__isThemeDark ? window.__isThemeDark() : false) ? t('menu_theme_light', 'Switch to light theme') : t('menu_theme_dark', 'Switch to dark theme');
    Object.assign(themeToggleItem.style, {
        padding: '6px 24px', cursor: 'pointer', color: menuItemColor, whiteSpace: 'nowrap'
    });
    themeToggleItem.addEventListener('mouseenter', () => { themeToggleItem.style.background = menuItemHover; });
    themeToggleItem.addEventListener('mouseleave', () => { themeToggleItem.style.background = 'transparent'; });
    themeToggleItem.addEventListener('click', (e) => {
        e.stopPropagation();
        if (window.__toggleTheme) window.__toggleTheme();
        menu.style.display = 'none';
    });
    menu.appendChild(themeToggleItem);

    addItem(t('menu_font_settings', 'Font settings'), openFontSettings, false);

    // ---- TOC show/hide ----
    var tocToggleItem = document.createElement('div');
    Object.assign(tocToggleItem.style, {
        padding: '6px 24px', cursor: 'pointer', color: menuItemColor, whiteSpace: 'nowrap'
    });
    tocToggleItem.addEventListener('mouseenter', () => { tocToggleItem.style.background = menuItemHover; });
    tocToggleItem.addEventListener('mouseleave', () => { tocToggleItem.style.background = 'transparent'; });
    tocToggleItem.addEventListener('click', (e) => {
        e.stopPropagation();
        if (window.__mdTocToggle) window.__mdTocToggle();
        menu.style.display = 'none';
    });
    menu.appendChild(tocToggleItem);
    // ---- end TOC option ----

    addItem(t('menu_edit_source', 'Edit source'), editSource, true);
    addItem(t('menu_editor_settings', 'Editor settings'), openEditorSettings, false);
    addItem(t('menu_about', 'About'), function() {
        document.getElementById('about-modal').style.display = 'flex';
    }, true);

    document.body.appendChild(menu);

    // === About modal ===
    var aboutModal = document.createElement('div');
    aboutModal.id = 'about-modal';
    Object.assign(aboutModal.style, {
        position: 'fixed', top: '0', left: '0', width: '100%', height: '100%',
        background: 'rgba(0,0,0,0.5)', display: 'none', justifyContent: 'center',
        alignItems: 'center', zIndex: '1000000', fontFamily: 'sans-serif'
    });
    aboutModal.innerHTML = '<div style="background:var(--sponsor-card-bg);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid var(--sponsor-card-border);padding:32px;border-radius:16px;text-align:center;max-width:320px;width:90%;box-shadow:0 12px 40px rgba(0,0,0,0.3);position:relative;">' +
        '<div style="width:48px;height:48px;background:linear-gradient(135deg,#3b82f6,#2563eb);border-radius:10px;display:flex;align-items:center;justify-content:center;margin:0 auto 8px;font-size:28px;font-weight:bold;color:white;font-family:sans-serif;box-shadow:0 4px 8px rgba(0,0,0,0.1);">M</div>' +
        '<div style="margin-top:0;margin-bottom:4px;color:var(--sponsor-text);font-weight:600;font-size:20px;">' + t('about_title', 'mdview') + '</div>' +
        '<p style="color:var(--sponsor-subtext);font-size:13px;margin:0 0 16px;">v{{VERSION}}</p>' +
        '<p style="color:var(--sponsor-subtext);line-height:1.6;margin-bottom:20px;font-size:14px;">' + t('about_subtitle', 'Lightweight Markdown preview tool') + '</p>' +
        '<p style="color:var(--sponsor-subtext);font-size:11px;margin:0 0 20px;opacity:0.7;">' + t('about_credits', 'Markdown rendering by md4x · MIT License') + '</p>' +
        '<button id="about-close-btn" style="padding:8px 24px;background:var(--sponsor-btn-bg);color:var(--sponsor-btn-text);border:none;border-radius:8px;cursor:pointer;font-size:14px;font-weight:500;">' + t('btn_close', 'Close') + '</button>' +
        '<p style="margin:16px 0 0;"><a href="javascript:void(0)" id="about-external-link" style="color:var(--sponsor-subtext);font-size:12px;text-decoration:underline;">' + t('about_learn_more', 'Learn more') + '</a></p>' +
    '</div>';
    document.body.appendChild(aboutModal);
    aboutModal.addEventListener('click', function(e) {
        if (e.target === aboutModal) aboutModal.style.display = 'none';
    });
    document.getElementById('about-close-btn').addEventListener('click', function() {
        aboutModal.style.display = 'none';
    });
    var aboutExternalLink = document.getElementById('about-external-link');
    if (aboutExternalLink) {
        aboutExternalLink.addEventListener('click', function(e) {
            e.preventDefault();
            window.ipc.postMessage('external:https://github.com/mdview2026/mdview');
        });
    }

    // === Editor settings modal ===
    var editorModal = document.createElement('div');
    editorModal.id = 'editor-modal';
    Object.assign(editorModal.style, {
        position: 'fixed', top: '0', left: '0', width: '100%', height: '100%',
        background: 'rgba(0,0,0,0.5)', display: 'none', justifyContent: 'center',
        alignItems: 'center', zIndex: '1000000', fontFamily: 'sans-serif'
    });
    editorModal.innerHTML = '<div style="background:var(--sponsor-card-bg);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid var(--sponsor-card-border);padding:24px;border-radius:16px;text-align:left;max-width:320px;width:90%;box-shadow:0 12px 40px rgba(0,0,0,0.3);position:relative;">' +
        '<div style="color:var(--sponsor-text);font-weight:600;font-size:16px;margin-bottom:16px;text-align:center;">' + t('editor_settings_title', 'Editor Settings') + '</div>' +
        '<div id="editor-options" style="margin-bottom:12px;"></div>' +
        '<div style="text-align:center;margin-bottom:16px;">' +
        '<button id="editor-browse-btn" style="padding:6px 16px;background:transparent;color:var(--sponsor-text);border:1px solid var(--sponsor-card-border);border-radius:8px;cursor:pointer;font-size:13px;">' + t('btn_browse', 'Browse...') + '</button>' +
        '</div>' +
        '<div style="display:flex;gap:8px;justify-content:center;">' +
        '<button id="editor-save-btn" style="padding:6px 20px;background:var(--sponsor-btn-bg);color:var(--sponsor-btn-text);border:none;border-radius:8px;cursor:pointer;font-size:14px;font-weight:500;">' + t('btn_ok', 'OK') + '</button>' +
        '<button id="editor-cancel-btn" style="padding:6px 20px;background:transparent;color:var(--sponsor-text);border:1px solid var(--sponsor-card-border);border-radius:8px;cursor:pointer;font-size:14px;">' + t('btn_cancel', 'Cancel') + '</button>' +
        '</div>' +
    '</div>';
    document.body.appendChild(editorModal);
    editorModal.addEventListener('click', function(e) {
        if (e.target === editorModal) editorModal.style.display = 'none';
    });
    document.getElementById('editor-cancel-btn').addEventListener('click', function() {
        editorModal.style.display = 'none';
    });

    var editorChoices = [
        {key: '', label: t('editor_auto_detect', 'Auto detect')},
        {key: 'code', label: t('editor_vscode', 'VS Code')},
        {key: 'subl', label: t('editor_sublime', 'Sublime Text')},
        {key: 'notepad++', label: t('editor_notepad_plus', 'Notepad++')},
        {key: 'emeditor', label: t('editor_emeditor', 'EmEditor')},
        {key: 'notepad', label: t('editor_notepad', 'Notepad')}
    ];
    var currentEditorChoice = '';
    var customEditorPath = '';
    var lastAvailable = {};

    function renderEditorOptions(available) {
        lastAvailable = available || lastAvailable;
        var container = document.getElementById('editor-options');
        container.innerHTML = '';
        var allChoices = editorChoices.slice();
        if (customEditorPath) {
            var fileName = customEditorPath.replace(/\\/g, '/').split('/').pop();
            allChoices.push({key: customEditorPath, label: t('editor_custom_prefix', 'Custom') + ': ' + fileName});
        }
        allChoices.forEach(function(c) {
            var row = document.createElement('div');
            var isAvail;
            if (c.key === '' || c.key === 'notepad' || c.key === customEditorPath) {
                isAvail = true;
            } else {
                isAvail = lastAvailable[c.key];
            }
            var isSelected = currentEditorChoice === c.key;
            row.style.cssText = 'padding:8px 12px;margin-bottom:6px;border-radius:6px;cursor:pointer;display:flex;align-items:center;gap:8px;' + (isSelected ? 'background:rgba(59,130,246,0.15);' : 'background:transparent;');
            if (!isAvail) row.style.opacity = '0.5';
            row.innerHTML = '<span style="width:16px;height:16px;border-radius:50%;border:2px solid ' + (isSelected ? '#3b82f6' : '#888') + ';display:flex;align-items:center;justify-content:center;"><span style="width:8px;height:8px;border-radius:50%;background:' + (isSelected ? '#3b82f6' : 'transparent') + ';"></span></span>' +
                '<span style="color:var(--sponsor-text);font-size:14px;flex:1;">' + c.label + (isAvail ? '' : t('editor_not_installed', '(not installed)')) + '</span>';
            if (isAvail) {
                row.addEventListener('click', function() {
                    currentEditorChoice = c.key;
                    renderEditorOptions(lastAvailable);
                });
            }
            container.appendChild(row);
        });
    }

    function openEditorSettings() {
        fetch('/_editor').then(function(r) { return r.json(); }).then(function(data) {
            currentEditorChoice = data.current;
            customEditorPath = '';
            var presetKeys = editorChoices.map(function(c) { return c.key; });
            if (data.current && presetKeys.indexOf(data.current) < 0) {
                customEditorPath = data.current;
            }
            renderEditorOptions(data.available);
            editorModal.style.display = 'flex';
        }).catch(function(e) {
            alert(t('editor_load_failed', 'Failed to load editor config') + ': ' + e.message);
        });
    }

    var browseBtn = document.getElementById('editor-browse-btn');
    var saveBtn = document.getElementById('editor-save-btn');
    var cancelBtn = document.getElementById('editor-cancel-btn');
    var btnHoverBg = isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.05)';
    var saveHoverBg = isDark ? 'rgba(255,255,255,0.18)' : 'rgba(0,0,0,0.12)';
    browseBtn.addEventListener('mouseenter', function() { browseBtn.style.background = btnHoverBg; });
    browseBtn.addEventListener('mouseleave', function() { browseBtn.style.background = 'transparent'; });
    saveBtn.addEventListener('mouseenter', function() { saveBtn.style.background = saveHoverBg; });
    saveBtn.addEventListener('mouseleave', function() { saveBtn.style.background = 'var(--sponsor-btn-bg)'; });
    cancelBtn.addEventListener('mouseenter', function() { cancelBtn.style.background = btnHoverBg; });
    cancelBtn.addEventListener('mouseleave', function() { cancelBtn.style.background = 'transparent'; });

    browseBtn.addEventListener('click', function() {
        window.ipc.postMessage('browse:editor');
    });

    window.__onEditorPathSelected = function(path) {
        customEditorPath = path;
        currentEditorChoice = path;
        renderEditorOptions(lastAvailable);
    };

    document.getElementById('editor-save-btn').addEventListener('click', function() {
        fetch('/_editor', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({editor: currentEditorChoice})
        }).then(function(r) { return r.json(); }).then(function(data) {
            if (data.ok) {
                editorModal.style.display = 'none';
            } else {
                alert(t('font_save_failed', 'Save failed') + ': ' + (data.error || t('bind_unknown_error', 'Unknown error')));
            }
        }).catch(function(e) {
            alert(t('font_save_failed', 'Save failed') + ': ' + e.message);
        });
    });

    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
        // Update TOC show/hide state
        var tocVisible = localStorage.getItem('mdview-toc-visible') !== '0';
        tocToggleItem.textContent = tocVisible ? t('toc_hide', 'Hide contents') : t('toc_show', 'Show contents');
        // Update theme toggle menu text
        var currentDark = window.__isThemeDark ? window.__isThemeDark() : false;
        themeToggleItem.textContent = currentDark ? t('menu_theme_light', 'Switch to light theme') : t('menu_theme_dark', 'Switch to dark theme');
        let x = e.clientX, y = e.clientY;
        menu.style.display = 'block';
        const rect = menu.getBoundingClientRect();
        if (x + rect.width > window.innerWidth) x = window.innerWidth - rect.width - 4;
        if (y + rect.height > window.innerHeight) y = window.innerHeight - rect.height - 4;
        menu.style.left = x + 'px';
        menu.style.top = y + 'px';
    });

    document.addEventListener('click', () => { menu.style.display = 'none'; });
    document.addEventListener('scroll', () => { menu.style.display = 'none'; }, true);

    // === Font settings modal ===
    var fontModal = document.createElement('div');
    fontModal.id = 'font-modal';
    Object.assign(fontModal.style, {
        position: 'fixed', top: '0', left: '0', width: '100%', height: '100%',
        background: 'rgba(0,0,0,0.5)', display: 'none', justifyContent: 'center',
        alignItems: 'center', zIndex: '1000000', fontFamily: 'sans-serif'
    });
    fontModal.innerHTML = '<div style="background:var(--sponsor-card-bg);backdrop-filter:blur(20px);-webkit-backdrop-filter:blur(20px);border:1px solid var(--sponsor-card-border);padding:24px;border-radius:16px;text-align:left;width:360px;max-width:90%;box-shadow:0 12px 40px rgba(0,0,0,0.3);position:relative;">' +
        '<div style="color:var(--sponsor-text);font-weight:600;font-size:16px;margin-bottom:12px;text-align:center;">' + t('font_settings_title', 'Font Settings') + '</div>' +
        '<input id="font-search" type="search" placeholder="' + t('font_search_placeholder', 'Search fonts...') + '" style="width:100%;box-sizing:border-box;padding:8px 12px;margin-bottom:8px;border:1px solid var(--sponsor-card-border);border-radius:8px;background:transparent;color:var(--sponsor-text);font-size:13px;outline:none;" />' +
        '<select id="font-list" size="10" style="width:100%;box-sizing:border-box;margin-bottom:12px;padding:4px;border:1px solid var(--sponsor-card-border);border-radius:8px;background:transparent;color:var(--sponsor-text);font-size:14px;outline:none;"></select>' +
        '<div id="font-preview" style="padding:10px 12px;margin-bottom:16px;border:1px dashed var(--sponsor-card-border);border-radius:8px;color:var(--sponsor-text);font-size:15px;text-align:center;">' + t('font_preview_text', 'Font preview AaBb Hello World 123') + '</div>' +
        '<div style="display:flex;gap:8px;justify-content:center;">' +
        '<button id="font-save-btn" style="padding:6px 20px;background:var(--sponsor-btn-bg);color:var(--sponsor-btn-text);border:none;border-radius:8px;cursor:pointer;font-size:14px;font-weight:500;">' + t('btn_ok', 'OK') + '</button>' +
        '<button id="font-cancel-btn" style="padding:6px 20px;background:transparent;color:var(--sponsor-text);border:1px solid var(--sponsor-card-border);border-radius:8px;cursor:pointer;font-size:14px;">' + t('btn_cancel', 'Cancel') + '</button>' +
        '</div>' +
    '</div>';
    document.body.appendChild(fontModal);
    fontModal.addEventListener('click', function(e) {
        if (e.target === fontModal) fontModal.style.display = 'none';
    });

    var fontAllList = [];        // All font names
    var fontCurrentChoice = '';  // Currently selected (empty string = default)

    function renderFontList(filter) {
        var sel = document.getElementById('font-list');
        sel.innerHTML = '';
        var f = (filter || '').trim().toLowerCase();
        // First entry: default
        var def = document.createElement('option');
        def.value = '';
        def.textContent = t('font_default', 'Default (system font)');
        def.style.fontFamily = '';
        sel.appendChild(def);
        fontAllList.forEach(function(name) {
            if (f && name.toLowerCase().indexOf(f) < 0) return;
            var opt = document.createElement('option');
            opt.value = name;
            opt.textContent = name;
            opt.style.fontFamily = '"' + name + '", sans-serif';
            sel.appendChild(opt);
        });
        sel.value = fontCurrentChoice;
        updateFontPreview();
    }

    function updateFontPreview() {
        var prev = document.getElementById('font-preview');
        if (fontCurrentChoice) {
            prev.style.fontFamily = '"' + fontCurrentChoice + '", sans-serif';
        } else {
            prev.style.fontFamily = '';
        }
    }

    document.getElementById('font-list').addEventListener('change', function() {
        fontCurrentChoice = this.value;
        updateFontPreview();
    });

    document.getElementById('font-search').addEventListener('input', function() {
        renderFontList(this.value);
    });

    function openFontSettings() {
        Promise.all([
            fetch('/_fonts').then(function(r) { return r.json(); }),
            fetch('/_font_config').then(function(r) { return r.json(); })
        ]).then(function(results) {
            fontAllList = results[0].fonts || [];
            fontCurrentChoice = results[1].md_font || '';
            document.getElementById('font-search').value = '';
            renderFontList('');
            fontModal.style.display = 'flex';
        }).catch(function(e) {
            alert(t('font_load_failed', 'Failed to load font list') + ': ' + e.message);
        });
    }

    document.getElementById('font-cancel-btn').addEventListener('click', function() {
        fontModal.style.display = 'none';
    });
    document.getElementById('font-save-btn').addEventListener('click', function() {
        fetch('/_font_config', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({md_font: fontCurrentChoice})
        }).then(function(r) { return r.json(); }).then(function(data) {
            if (data.ok) {
                // Apply immediately: change the CSS variable directly without refresh
                if (fontCurrentChoice) {
                    document.documentElement.style.setProperty('--md-font', '"' + fontCurrentChoice + '", sans-serif');
                } else {
                    document.documentElement.style.removeProperty('--md-font');
                }
                fontModal.style.display = 'none';
            } else {
                alert(t('font_save_failed', 'Save failed') + ': ' + (data.error || t('bind_unknown_error', 'Unknown error')));
            }
        }).catch(function(e) {
            alert(t('font_save_failed', 'Save failed') + ': ' + e.message);
        });
    });
    // === end font settings ===
})();

document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        var aboutModal = document.getElementById('about-modal');
        if (aboutModal && aboutModal.style.display === 'flex') {
            aboutModal.style.display = 'none';
            return;
        }
        document.body.style.transition = 'opacity 0.15s';
        document.body.style.opacity = '0';
        setTimeout(function() { window.ipc.postMessage('exit'); }, 150);
    }
});
</script>
"#;

/// Page zoom (font size and column width are independent):
/// - Ctrl + wheel / Ctrl + Up/Down: zoom font size (--md-fs)
/// - Alt + wheel / Alt + Up/Down: zoom body column width (--md-mw)
/// - Ctrl + 0 / Alt + 0: reset font size / column width respectively
/// Deliberately avoids CSS zoom (which scales the whole page as a bitmap, shrinking
/// the TOC as well without reflowing text).
/// With independent font-size scaling, text truly reflows; independent column-width
/// scaling lets you decide how many characters fit on a line.
/// Both ratios persist to localStorage (preserved across live-reload refreshes); the
/// lower bounds only exist to avoid illegal 0/negative values.
const ZOOM_SCRIPT: &str = r#"
<script>
(function () {
    // === Two independent ratios: font size f and column width w, both default to 1.0 ===
    var BASE_FS = 16, BASE_MW = 900, STEP = 0.1;
    var FS_FLOOR = 0.2, FS_CEIL = 4.0;   // Font size range
    var MW_FLOOR = 0.4;                   // Column width floor (360px)
    // Column width ceiling: dynamically computed to fill the entire viewport (including TOC width compensation)
    function maxW() { return Math.max(3.0, window.innerWidth / BASE_MW + 0.1); }
    var f = parseFloat(localStorage.getItem('__md_zoom_fs'));
    var w = parseFloat(localStorage.getItem('__md_zoom_mw'));
    // Migrate legacy linked zoom (single __md_zoom): reuse old value for font size, compute column width as inverse
    var legacy = localStorage.getItem('__md_zoom');
    if (!(f > 0) && legacy != null) {
        var lf = parseFloat(legacy);
        if (lf > 0) { f = lf; w = 1 / lf; }
        localStorage.removeItem('__md_zoom');
    }
    if (!(f > 0)) f = 1.0;
    if (!(w > 0)) w = 1.0;

    function applyZoom() {
        document.documentElement.style.setProperty('--md-fs', (BASE_FS * f) + 'px');
        document.documentElement.style.setProperty('--md-mw', Math.round(BASE_MW * w) + 'px');
        localStorage.setItem('__md_zoom_fs', String(f));
        localStorage.setItem('__md_zoom_mw', String(w));
    }
    applyZoom();

    // === Zoom percentage indicator (appears while zooming, fades out after) ===
    var isDark = window.__isThemeDark ? window.__isThemeDark() : false;
    var ind = document.createElement('div');
    ind.id = '__md_zoom_ind';
    Object.assign(ind.style, {
        position: 'fixed', bottom: '24px', left: '50%',
        transform: 'translateX(-50%)',
        zIndex: '999999', pointerEvents: 'none',
        padding: '6px 14px', borderRadius: '999px',
        fontSize: '13px', fontWeight: '600', fontVariantNumeric: 'tabular-nums',
        fontFamily: '-apple-system,BlinkMacSystemFont,Segoe UI,system-ui,sans-serif',
        background: isDark ? 'rgba(30,30,30,0.85)' : 'rgba(255,255,255,0.95)',
        color: isDark ? '#ddd' : '#333',
        border: isDark ? '1px solid rgba(255,255,255,0.12)' : '1px solid rgba(0,0,0,0.06)',
        boxShadow: '0 4px 14px rgba(0,0,0,0.18)',
        opacity: '0',
        transition: 'opacity 0.25s ease'
    });
    document.body.appendChild(ind);

    var hideTimer = null;
    function flashIndicator(label, value) {
        ind.textContent = label + ' ' + value;
        ind.style.opacity = '1';
        if (hideTimer) clearTimeout(hideTimer);
        hideTimer = setTimeout(function () { ind.style.opacity = '0'; }, 1200);
    }

    // Ctrl + wheel: zoom font size; Alt + wheel: zoom column width
    window.addEventListener('wheel', function (e) {
        if (e.ctrlKey && !e.altKey) {
            e.preventDefault();
            f += e.deltaY < 0 ? STEP : -STEP;
            if (f < FS_FLOOR) f = FS_FLOOR;
            if (f > FS_CEIL) f = FS_CEIL;
            applyZoom();
            flashIndicator(t('zoom_font_size', 'Font size'), Math.round(f * 100) + '%');
        } else if (e.altKey && !e.ctrlKey) {
            e.preventDefault();
            w += e.deltaY < 0 ? STEP : -STEP;
            if (w < MW_FLOOR) w = MW_FLOOR;
            var mwCeil = maxW();
            if (w > mwCeil) w = mwCeil;
            applyZoom();
            flashIndicator(t('zoom_column_width', 'Column width'), Math.round(BASE_MW * w) + 'px');
        }
    }, { passive: false });

    // Ctrl + 0 resets font size; Alt + 0 resets column width
    document.addEventListener('keydown', function (e) {
        var isZero = e.key === '0' || e.key === 'NumPad0';
        if (!isZero) return;
        if (e.ctrlKey && !e.altKey) {
            e.preventDefault();
            f = 1.0;
            applyZoom();
            flashIndicator(t('zoom_font_size', 'Font size'), '100%');
        } else if (e.altKey && !e.ctrlKey) {
            e.preventDefault();
            w = 1.0;
            applyZoom();
            flashIndicator(t('zoom_column_width', 'Column width'), BASE_MW + 'px');
        }
    });

    // Ctrl + Up/Down arrows: zoom font size (same effect as Ctrl + wheel)
    // Alt + Up/Down arrows: zoom column width (same effect as Alt + wheel)
    document.addEventListener('keydown', function (e) {
        if (e.key !== 'ArrowUp' && e.key !== 'ArrowDown') return;
        if (e.ctrlKey && !e.altKey) {
            e.preventDefault();
            f += e.key === 'ArrowUp' ? STEP : -STEP;
            if (f < FS_FLOOR) f = FS_FLOOR;
            if (f > FS_CEIL) f = FS_CEIL;
            applyZoom();
            flashIndicator(t('zoom_font_size', 'Font size'), Math.round(f * 100) + '%');
        } else if (e.altKey && !e.ctrlKey) {
            e.preventDefault();
            w += e.key === 'ArrowUp' ? STEP : -STEP;
            if (w < MW_FLOOR) w = MW_FLOOR;
            var mwCeil = maxW();
            if (w > mwCeil) w = mwCeil;
            applyZoom();
            flashIndicator(t('zoom_column_width', 'Column width'), Math.round(BASE_MW * w) + 'px');
        }
    });
})();
</script>
"#;



/// Renders a Markdown file to HTML using the embedded md4x C library
pub async fn render_markdown(file: &Path) -> Result<String> {
    let content = std::fs::read_to_string(file)
        .with_context(|| i18n::trf("error_file_read", &[&file.display().to_string()]))?;
    md4x::render_html(&content).map_err(|e| anyhow::anyhow!("{}", e))
}

/// Strips Markdown syntax characters for text matching
pub fn strip_markdown(s: &str) -> String {
    let re = regex_lite::Regex::new(r"[#*`~_\[\]()>!|]").unwrap();
    let s = re.replace_all(s, "");
    // Collapse extra whitespace
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}


pub fn build_html(html: &str, live_reload: bool) -> String {
    let theme = THEME_SCRIPT.to_string();
    let style = STYLE.to_string();
    let i18n_script = format!(
        "<script>window.__mdI18n = {};function t(key,fallback){{return (window.__mdI18n&&window.__mdI18n.strings?window.__mdI18n.strings[key]:null)||fallback||key;}}window.__applyI18n=function(root){{if(!root)return;root.querySelectorAll('[data-i18n]').forEach(function(el){{var k=el.getAttribute('data-i18n');var fb=el.getAttribute('data-i18n-fb')||el.textContent||'';el.textContent=t(k,fb);}});}};</script>",
        i18n::frontend_json()
    );
    let edit_script = EDIT_SCRIPT.replace("{{VERSION}}", APP_VERSION.trim());
    let ctx = edit_script + ANCHOR_SCRIPT + TOC_SCRIPT + ZOOM_SCRIPT;

    if html.contains("</body>") {
        let script = if live_reload { LIVE_RELOAD_SCRIPT } else { "" };
        html.replace("</body>", &format!("{}{}</body>", script, ctx))
            .replace("</head>", &format!("{}{}{}</head>", theme, style, i18n_script))
    } else if html.contains("</head>") {
        let script = if live_reload {
            LIVE_RELOAD_SCRIPT_DEFERRED
        } else {
            ""
        };
        html.replace("</head>", &format!("{}{}{}{}</head>", theme, style, i18n_script, script))
            .replace("</body>", &format!("{}</body>", ctx))
    } else {
        format!(
            r#"<!DOCTYPE html><html><head><meta charset="UTF-8">{}{}{}</head><body>{}{}</body></html>"#,
            theme, style, i18n_script, html, ctx
        )
    }
}
