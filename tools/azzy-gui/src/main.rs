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

mod update;

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
    last_request: AtomicU64,
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

fn handle(mut stream: TcpStream, root: &Path, state: &State) -> Result<(), String> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).ok();
    let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
    let mut request = String::new();
    reader.read_line(&mut request).map_err(|e| e.to_string())?;
    // drain the headers
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    if target == "/api/version" {
        let user_ai = state.user_ai.is_some();
        let body = format!(
            "{{\"name\":\"azzy-gui\",\"version\":\"{}\",\"update\":{},\"user_ai\":{}}}",
            env!("CARGO_PKG_VERSION"),
            user_ai,
            user_ai
        );
        return respond(&mut stream, 200, "application/json", body.as_bytes());
    }
    if target == "/api/update" {
        if method != "POST" {
            return respond(&mut stream, 405, "text/plain", b"POST only");
        }
        let Some(dir) = state.user_ai.as_ref().as_ref() else {
            return respond(&mut stream, 409, "application/json", b"{\"error\":\"no USER_AI folder (use --user-ai)\"}");
        };
        let body = match download(&state.update_url) {
            Ok(bytes) => match update::apply_zip(&bytes, dir, state.overwrite_presets) {
                Ok(report) => report.to_json(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e.replace('"', "'")),
            },
            Err(e) => format!("{{\"error\":\"{}\"}}", e.replace('"', "'")),
        };
        let status = if body.contains("\"error\"") { 502 } else { 200 };
        return respond(&mut stream, status, "application/json", body.as_bytes());
    }
    if method != "GET" && method != "HEAD" {
        return respond(&mut stream, 405, "text/plain", b"method not allowed");
    }
    let path = target.split(['?', '#']).next().unwrap_or("/");
    let path = percent_decode(path);
    let relative = path.trim_start_matches('/');
    let relative = if relative.is_empty() { "index.html" } else { relative };
    let Some(file) = safe_join(root, relative) else {
        return respond(&mut stream, 403, "text/plain", b"forbidden");
    };
    let file = if file.is_dir() { file.join("index.html") } else { file };
    match std::fs::read(&file) {
        Ok(body) => {
            let mime = mime_of(&file);
            if method == "HEAD" {
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

fn download(url: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("azzy-gui")
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    let response = client.get(url).send().map_err(|e| format!("{url}: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("{url}: HTTP {}", response.status().as_u16()));
    }
    response.bytes().map(|b| b.to_vec()).map_err(|e| format!("{url}: {e}"))
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
