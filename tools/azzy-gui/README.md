# azzy-gui

Serves the AzzyAI web GUI from the local folder and opens it in the default browser.

**Why:** `webgui/` is a static site, but opening `index.html` from `file://` breaks "Open USER_AI
folder" - the File System Access API needs a secure context. `http://127.0.0.1` **is** one, so the
page behaves exactly as it does on GitHub Pages, offline, with no WebView runtime (works on old
Windows, no WebView2).

This is the "one UI, native host" path: the interface stays the existing web GUI, and the native
part is a host that can later own the updater, diagnostics and file dialogs.

```
cargo run --release --manifest-path tools/Cargo.toml -p azzy-gui -- [--port 8080] [--root <dir>] [--no-browser]
```

- `--port` fixed port (default: any free one), `--root` another copy of the web GUI,
  `--no-browser` only serves.
- No dependencies: the HTTP subset a static page needs (GET/HEAD, MIME, 404/403/405, no path
  traversal, percent-decoding) is in ~250 lines of plain Rust.
- Verified by hand against the real `webgui/`: `/` and `/index.html` 200 `text/html`,
  `/app.js` 200 `text/javascript`, `?query` ignored, `HEAD` returns headers only, `POST` 405,
  `/../Cargo.toml` and `/%2e%2e%2fCargo.toml` 403.

Not done yet: closing the server when the tab closes, a tray icon, native file dialogs (the page
uses the browser's own folder picker), the updater and the diagnostics export - those already exist
in the Windows app and would move here.
