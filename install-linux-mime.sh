#!/usr/bin/env bash
# Register mdview as the default application for .md files on Linux.
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Prefer the mdview executable on PATH; otherwise fall back to the local build.
MDVIEW_EXE="$(command -v mdview || true)"
if [ -z "$MDVIEW_EXE" ]; then
    MDVIEW_EXE="$SCRIPT_DIR/target/release/mdview"
fi

if [ ! -x "$MDVIEW_EXE" ]; then
    echo "Error: mdview executable not found." >&2
    echo "Expected: $MDVIEW_EXE" >&2
    echo "Build it first with: PATH=\"/path/to/zig:\$PATH\" cargo build --release" >&2
    exit 1
fi

# Resolve to an absolute path.
MDVIEW_EXE="$(cd "$(dirname "$MDVIEW_EXE")" && pwd)/$(basename "$MDVIEW_EXE")"

ICON_FILE="$SCRIPT_DIR/static/logo_big.png"

DESKTOP_DIR="$HOME/.local/share/applications"
mkdir -p "$DESKTOP_DIR"

DESKTOP_FILE="$DESKTOP_DIR/mdview.desktop"

cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Name=mdview
Comment=Markdown live preview
Exec=$MDVIEW_EXE %f
Icon=$ICON_FILE
Type=Application
Terminal=false
MimeType=text/markdown;text/x-markdown;
Categories=Utility;TextEditor;
EOF

# Make sure the .desktop file is executable.
chmod +x "$DESKTOP_FILE"

# Register mdview as the default handler for Markdown MIME types.
xdg-mime default mdview.desktop text/markdown || true
xdg-mime default mdview.desktop text/x-markdown || true

# Refresh the desktop database so file managers pick up the change.
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR"
fi

echo "mdview registered as the default app for .md files:"
echo "  executable: $MDVIEW_EXE"
echo "  desktop:    $DESKTOP_FILE"
