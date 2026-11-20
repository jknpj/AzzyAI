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
- The static server itself has no dependencies: GET/HEAD, MIME, 404/403/405, no path traversal and
  percent-decoding are plain std Rust. The updater adds `reqwest` and `zip` (already in the
  workspace for the sync tools).
- **`GET /api/files` and `POST /api/save`** carry the host mode: served by `azzy-gui` the page
  reads the configuration from disk and writes the edited files straight back, so there is no
  "Open USER_AI folder", no File System Access API (Firefox/Safari work too) and no copy in
  `localStorage` to drift from the files. Only the nine page files plus `Types/*.lua` are read or
  written; writes go to a `.lua.tmp` and are renamed, so a half-written config never exists.
  Both require the token the host injects into `index.html` as a meta tag plus a matching `Origin`,
  so another page in the same browser cannot drive the API.
- **`POST /api/update`** is the Windows app's "Update AI from git" without git: downloads
  `https://github.com/jknpj/AzzyAI/archive/refs/heads/main.zip`, keeps `H_Config.lua`, `M_Config.lua`,
  the tactics/extra files, `A_Friends.lua` and `Types/*.lua` (the last unless `--overwrite-presets`),
  backs up every replaced file under `USER_AI/backup/update-<timestamp>/`, and stages a newer
  executable as `.exe.new` (a running exe cannot be replaced on Windows). `--update-url` points it
  at another archive, which is how the tests run it against a local zip.
- `--idle <minutes>` stops the server after N minutes without requests (default 15, 0 disables),
  so closing the tab does not leave a process behind. `--user-ai <folder>` overrides the update
  target.
- Verified by hand against the real `webgui/`: `/` and `/index.html` 200 `text/html`,
  `/app.js` 200 `text/javascript`, `?query` ignored, `HEAD` returns headers only, `POST` 405,
  `/../Cargo.toml` and `/%2e%2e%2fCargo.toml` 403.
- `cargo test -p azzy-gui` covers the updater: user files kept, shipped files replaced with a
  backup, `Types/` kept by default and replaced with `--overwrite-presets`, `.exe` staged as
  `.exe.new`, an archive without `USER_AI` refused. End to end it was also run against a zip served
  by `python3 -m http.server`: `{"updated":2,"added":1,"kept":1,...}` and the user's
  `H_Config.lua` untouched.

Not done yet: closing the server when the tab closes, a tray icon, native file dialogs (the page
uses the browser's own folder picker), the updater and the diagnostics export - those already exist
in the Windows app and would move here.
