// azzy-gui: the web GUI, served from the local install, without the internet.
//
// Why: webgui/ is a static site, but opening index.html from file:// does not work - the File
// System Access API needs a secure context, so "Open USER_AI folder" fails there. A server on
// 127.0.0.1 is a secure context (like the GitHub Pages site), so the page behaves exactly as it
// does online, with no dependency on the network or on a WebView runtime (works on old Windows).
//
// No dependencies: the HTTP subset a static page needs (GET, HEAD, MIME, 404, no traversal) is
// small enough to keep it that way.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};


fn main() -> ExitCode {
    let mut port: u16 = 0;
    let mut root: Option<PathBuf> = None;
    let mut user_ai: Option<PathBuf> = None;
    let mut update_url = DEFAULT_UPDATE_URL.to_string();
    let mut idle_minutes: u64 = 15;
    let mut overwrite_presets = false;
    let mut open = true;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--port" => match it.next().and_then(|v| v.parse().ok()) {
                Some(p) => port = p,
                None => return usage("--port needs a number"),
            },
            "--root" => match it.next() {
                Some(p) => root = Some(PathBuf::from(p)),
                None => return usage("--root needs a folder"),
            },
            "--user-ai" => match it.next() {
                Some(p) => user_ai = Some(PathBuf::from(p)),
                None => return usage("--user-ai needs a folder"),
            },
            "--update-url" => match it.next() {
                Some(u) => update_url = u,
                None => return usage("--update-url needs an URL"),
            },
            "--idle" => match it.next().and_then(|v| v.parse().ok()) {
                Some(minutes) => idle_minutes = minutes,
                None => return usage("--idle needs minutes (0 disables the exit)"),
            },
            "--overwrite-presets" => overwrite_presets = true,
            "--no-browser" => open = false,
            "-h" | "--help" => {
                print!("{}", include_str!("help.txt"));
                return ExitCode::SUCCESS;
            }
            other => return usage(&format!("unknown argument {other:?}")),
        }
    }
    let root = match root.or_else(find_webgui) {
        Some(root) => root,
        None => {
            eprintln!("azzy-gui: webgui/ not found next to this program or in the repository; pass --root");
            return ExitCode::FAILURE;
        }
    };
    let user_ai = user_ai.or_else(|| find_user_ai(&root));
    match serve(&root, user_ai, &update_url, idle_minutes, overwrite_presets, port, open) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("azzy-gui: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn usage(msg: &str) -> ExitCode {
    eprintln!("azzy-gui: {msg} (see --help)");
    ExitCode::FAILURE
}

// The exe may sit in webgui/ (packaged build) or in tools/azzy-gui/target/... (cargo run).
fn find_webgui() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(exe) = std::env::current_exe().and_then(|p| p.canonicalize()) {
        candidates.push(exe);
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("x"));
    }
    for start in candidates {
        let mut dir = start;
        for _ in 0..6 {
            if !dir.pop() {
                break;
            }
            let webgui = dir.join("webgui");
            if webgui.join("index.html").is_file() {
                return Some(webgui);
            }
        }
    }
    None
}

const DEFAULT_UPDATE_URL: &str = "https://github.com/jknpj/AzzyAI/archive/refs/heads/main.zip";

fn serve(root: &Path, user_ai: Option<PathBuf>, update_url: &str, idle_minutes: u64, overwrite_presets: bool, port: u16, open: bool) -> Result<(), String> {
    let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|e| format!("bind: {e}"))?;
    let addr = listener.local_addr().map_err(|e| format!("local_addr: {e}"))?;
    let url = format!("http://127.0.0.1:{}/", addr.port());
    let root = Arc::new(root.to_path_buf());
    let state = Arc::new(State {
        user_ai: Arc::new(user_ai),
        update_url: update_url.to_string(),
        overwrite_presets,
        token: make_token(),
        port: addr.port(),
        last_request: AtomicU64::new(now_ms()),
    });
    println!("serving {} at {url}", root.display());
    match state.user_ai.as_ref().as_ref() {
        Some(dir) => println!("update target: {}", dir.display()),
        None => println!("no USER_AI folder found; /api/update will refuse (use --user-ai)"),
    }
    println!("Ctrl+C stops it; the page keeps its theme and settings in the browser");
    if idle_minutes > 0 {
        let state = Arc::clone(&state);
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(30));
            let idle = now_ms().saturating_sub(state.last_request.load(Ordering::Relaxed));
            if idle > idle_minutes * 60_000 {
                println!("no request for {idle_minutes} min, stopping");
                std::process::exit(0);
            }
        });
    }
    if open {
        if let Err(e) = open_browser(&url) {
            println!("  could not open the browser ({e}); open the URL above by hand");
        }
    }
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = Arc::clone(&root);
                let state = Arc::clone(&state);
                std::thread::spawn(move || {
                    state.last_request.store(now_ms(), Ordering::Relaxed);
                    if let Err(e) = handle(stream, &root, &state) {
                        eprintln!("  {e}");
                    }
                });
            }
            Err(e) => eprintln!("  accept: {e}"),
        }
    }
    Ok(())
}

struct State {
    user_ai: Arc<Option<PathBuf>>,
    update_url: String,
    overwrite_presets: bool,
    token: String,
    port: u16,
    last_request: AtomicU64,
}

// A token the served page carries in a meta tag: another site in the same browser cannot read it,
// and the Origin check covers the rest. Not a secret against local processes - it stops CSRF.
fn make_token() -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0);
    let mut x = nanos ^ ((std::process::id() as u64) << 32);
    let mut out = String::with_capacity(32);
    for _ in 0..4 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.push_str(&format!("{x:016x}"));
    }
    out
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

// USER_AI sits next to the web GUI in the packaged zip, or under the repository root.
fn find_user_ai(webgui: &Path) -> Option<PathBuf> {
    let parent = webgui.parent()?;
    for candidate in [parent.join("USER_AI"), parent.join("AI").join("USER_AI")] {
        if candidate.join("H_Config.lua").is_file() {
            return Some(candidate);
        }
    }
    None
}

struct Request {
    method: String,
    target: String,
    headers: std::collections::BTreeMap<String, String>,
    body: Vec<u8>,
}

fn read_request(reader: &mut BufReader<TcpStream>) -> Result<Request, String> {
    let mut line = String::new();
    reader.read_line(&mut line).map_err(|e| e.to_string())?;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("/").to_string();
    let mut headers = std::collections::BTreeMap::new();
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).map_err(|e| e.to_string())? == 0
            || header == "\r\n" || header == "\n" { break; }
        if let Some((name, value)) = header.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }
    let mut body = Vec::new();
    if let Some(len) = headers.get("content-length").and_then(|v| v.parse::<usize>().ok()) {
        body.resize(len, 0);
        std::io::Read::read_exact(reader, &mut body).map_err(|e| e.to_string())?;
    }
    Ok(Request { method, target, headers, body })
}

fn handle(mut stream: TcpStream, root: &Path, state: &State) -> Result<(), String> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).ok();
    let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
    let req = read_request(&mut reader)?;

    if req.target == "/api/version" {
        let user_ai = state.user_ai.is_some();
        let body = format!(
            "{{\"name\":\"azzy-gui\",\"version\":\"{}\",\"update\":{},\"files\":{},\"user_ai\":{}}}",
            env!("CARGO_PKG_VERSION"), user_ai, user_ai, user_ai
        );
        return respond(&mut stream, 200, "application/json", body.as_bytes());
    }

    if req.target == "/api/files" || req.target == "/api/save" || req.target == "/api/update" {
        if let Err(status) = authorize(&req, state) {
            return respond(&mut stream, status, "application/json", b"{\"error\":\"forbidden\"}");
        }
        let Some(dir) = state.user_ai.as_ref().as_ref() else {
            return respond(&mut stream, 409, "application/json", b"{\"error\":\"no USER_AI folder (use --user-ai)\"}");
        };
        if req.target == "/api/files" {
            if req.method != "GET" && req.method != "HEAD" {
                return respond(&mut stream, 405, "text/plain", b"GET only");
            }
            let files = match read_files(dir) { Ok(f) => f, Err(e) => return json_error(&mut stream, 500, &e) };
            return respond(&mut stream, 200, "application/json", files.as_bytes());
        }
        if req.method != "POST" {
            return respond(&mut stream, 405, "text/plain", b"POST only");
        }
        if req.target == "/api/save" {
            let (path, content) = match parse_save(&req.body) {
                Ok(v) => v,
                Err(e) => return json_error(&mut stream, 400, &e),
            };
            if !allowed_path(&path) {
                return json_error(&mut stream, 400, &format!("{path} is not a configuration file"));
            }
            let target = dir.join(path.replace('/', std::path::MAIN_SEPARATOR_STR));
            if let Some(parent) = target.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    return json_error(&mut stream, 500, &format!("{}: {e}", parent.display()));
                }
            }
            // write beside the file and rename, so a half-written config is never left behind
            let temporary = target.with_extension("lua.tmp");
            if let Err(e) = std::fs::write(&temporary, content.as_bytes()) {
                return json_error(&mut stream, 500, &format!("{}: {e}", temporary.display()));
            }
            if let Err(e) = std::fs::rename(&temporary, &target) {
                let _ = std::fs::remove_file(&temporary);
                return json_error(&mut stream, 500, &format!("{}: {e}", target.display()));
            }
            return respond(&mut stream, 200, "application/json", b"{\"ok\":true}");
        }
        let body = match azzy_update::download(&state.update_url) {
            Ok(bytes) => match azzy_update::apply_zip(&bytes, dir, state.overwrite_presets) {
                Ok(report) => report.to_json(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e.replace('"', "'")),
            },
            Err(e) => format!("{{\"error\":\"{}\"}}", e.replace('"', "'")),
        };
        let status = if body.contains("\"error\"") { 502 } else { 200 };
        return respond(&mut stream, status, "application/json", body.as_bytes());
    }

    if req.method != "GET" && req.method != "HEAD" {
        return respond(&mut stream, 405, "text/plain", b"method not allowed");
    }
    let path = req.target.split(['?', '#']).next().unwrap_or("/");
    let path = percent_decode(path);
    let relative = path.trim_start_matches('/');
    let relative = if relative.is_empty() { "index.html" } else { relative };
    let Some(file) = safe_join(root, relative) else {
        return respond(&mut stream, 403, "text/plain", b"forbidden");
    };
    let file = if file.is_dir() { file.join("index.html") } else { file };
    match std::fs::read(&file) {
        Ok(mut body) => {
            // the page learns the host token from a meta tag; it never travels in the URL
            if file.file_name().and_then(|n| n.to_str()) == Some("index.html") {
                if let Ok(text) = String::from_utf8(body.clone()) {
                    let meta = format!("<meta name=\"azzy-host-token\" content=\"{}\">", state.token);
                    let injected = match text.find("<head>") {
                        Some(i) => format!("{}{}{}", &text[..i + 6], meta, &text[i + 6..]),
                        None => format!("{meta}{text}"),
                    };
                    body = injected.into_bytes();
                }
            }
            let mime = mime_of(&file);
            if req.method == "HEAD" {
                respond_head(&mut stream, 200, mime, body.len())
            } else {
                respond(&mut stream, 200, mime, &body)
            }
        }
        Err(_) => {
            let body = format!("404 - not found: {relative}");
            respond(&mut stream, 404, "text/plain; charset=utf-8", body.as_bytes())
        }
    }
}

// The API is reachable from any page the browser has open, so it needs two locks: the token only
// the served page knows, and an Origin that matches this server when the browser sends one.
fn authorize(req: &Request, state: &State) -> Result<(), u16> {
    let token = req.headers.get("x-azzy-token").map(|s| s.as_str()).unwrap_or("");
    if token != state.token {
        return Err(403);
    }
    if let Some(origin) = req.headers.get("origin") {
        let expected = format!("http://127.0.0.1:{}", state.port);
        let alias = format!("http://localhost:{}", state.port);
        if origin != &expected && origin != &alias {
            return Err(403);
        }
    }
    Ok(())
}

// Only the files the page edits, plus Types/*.lua: no path from the request can reach anything else.
const PAGE_FILES: [&str; 9] = [
    "H_Config.lua", "M_Config.lua", "H_Tactics.lua", "M_Tactics.lua", "H_PVP_Tact.lua",
    "M_PVP_Tact.lua", "H_Extra.lua", "M_Extra.lua", "A_Friends.lua",
];

fn allowed_path(path: &str) -> bool {
    if PAGE_FILES.contains(&path) {
        return true;
    }
    if let Some(name) = path.strip_prefix("Types/") {
        return name.ends_with(".lua")
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-')
            && !name.starts_with('.');
    }
    false
}

fn read_files(dir: &Path) -> Result<String, String> {
    let mut out = String::from("{\"files\":{");
    let mut first = true;
    let add = |name: &str, text: String, out: &mut String, first: &mut bool| {
        if !*first { out.push(','); }
        *first = false;
        out.push_str(&serde_json::to_string(name).map_err(|e| e.to_string()).unwrap_or_default());
        out.push(':');
        out.push_str(&serde_json::to_string(&text).map_err(|e| e.to_string()).unwrap_or_default());
    };
    for name in PAGE_FILES {
        let path = dir.join(name);
        if let Ok(text) = std::fs::read_to_string(&path) {
            add(name, text, &mut out, &mut first);
        }
    }
    if let Ok(entries) = std::fs::read_dir(dir.join("Types")) {
        let mut names: Vec<String> = entries
            .flatten()
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter(|n| n.ends_with(".lua"))
            .collect();
        names.sort();
        for name in names {
            if let Ok(text) = std::fs::read_to_string(dir.join("Types").join(&name)) {
                add(&format!("Types/{name}"), text, &mut out, &mut first);
            }
        }
    }
    out.push_str("}}");
    Ok(out)
}

fn parse_save(body: &[u8]) -> Result<(String, String), String> {
    let value: serde_json::Value = serde_json::from_slice(body).map_err(|e| format!("invalid JSON: {e}"))?;
    let path = value.get("path").and_then(|v| v.as_str()).ok_or("path is missing")?;
    let content = value.get("content").and_then(|v| v.as_str()).ok_or("content is missing")?;
    Ok((path.to_string(), content.to_string()))
}

fn json_error(stream: &mut TcpStream, status: u16, message: &str) -> Result<(), String> {
    let body = serde_json::json!({ "error": message }).to_string();
    respond(stream, status, "application/json", body.as_bytes())
}

// No "..", no absolute paths: only names below the root.
fn safe_join(root: &Path, relative: &str) -> Option<PathBuf> {
    let mut path = root.to_path_buf();
    for part in Path::new(relative).components() {
        match part {
            Component::Normal(name) => path.push(name),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(path)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
            if let Ok(b) = u8::from_str_radix(hex, 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn mime_of(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "txt" | "md" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn respond(stream: &mut TcpStream, status: u16, mime: &str, body: &[u8]) -> Result<(), String> {
    let head = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        reason(status),
        body.len()
    );
    stream.write_all(head.as_bytes()).and_then(|_| stream.write_all(body)).map_err(|e| e.to_string())
}

fn respond_head(stream: &mut TcpStream, status: u16, mime: &str, len: usize) -> Result<(), String> {
    let head = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: {mime}\r\nContent-Length: {len}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        reason(status)
    );
    stream.write_all(head.as_bytes()).map_err(|e| e.to_string())
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    }
}

fn open_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let (program, args) = ("cmd", vec!["/C", "start", "", url]);
    #[cfg(target_os = "macos")]
    let (program, args) = ("open", vec![url]);
    #[cfg(all(unix, not(target_os = "macos")))]
    let (program, args) = ("xdg-open", vec![url]);
    std::process::Command::new(program)
        .args(args)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_page_files_and_presets_are_writable() {
        assert!(allowed_path("H_Config.lua"));
        assert!(allowed_path("Types/Dieter.lua"));
        for bad in ["", "../H_Config.lua", "Types/../H_Config.lua", "MobInfo.lua",
                    "A_Friends.lua.bak", "Types/.hidden.lua", "/etc/passwd", "data/H_1Skills.lua"] {
            assert!(!allowed_path(bad), "{bad:?} must be refused");
        }
    }

    #[test]
    fn read_files_returns_disk_content_and_presets() {
        let dir = std::env::temp_dir().join(format!("azzy-gui-files-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Types")).unwrap();
        std::fs::write(dir.join("H_Config.lua"), "LavaSlideMode = 0\n").unwrap();
        std::fs::write(dir.join("Types").join("Dieter.lua"), "AutoMobCount = 1\n").unwrap();
        let json = read_files(&dir).unwrap();
        assert!(json.contains("\"H_Config.lua\":\"LavaSlideMode = 0\\n\""), "{json}");
        assert!(json.contains("\"Types/Dieter.lua\""), "{json}");
        assert!(!json.contains("MobInfo"), "{json}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
