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
use std::sync::Arc;

fn main() -> ExitCode {
    let mut port: u16 = 0;
    let mut root: Option<PathBuf> = None;
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
    match serve(&root, port, open) {
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

fn serve(root: &Path, port: u16, open: bool) -> Result<(), String> {
    let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|e| format!("bind: {e}"))?;
    let addr = listener.local_addr().map_err(|e| format!("local_addr: {e}"))?;
    let url = format!("http://127.0.0.1:{}/", addr.port());
    let root = Arc::new(root.to_path_buf());
    println!("serving {} at {url}", root.display());
    println!("Ctrl+C stops it; the page keeps its theme and settings in the browser");
    if open {
        if let Err(e) = open_browser(&url) {
            println!("  could not open the browser ({e}); open the URL above by hand");
        }
    }
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = Arc::clone(&root);
                std::thread::spawn(move || {
                    if let Err(e) = handle(stream, &root) {
                        eprintln!("  {e}");
                    }
                });
            }
            Err(e) => eprintln!("  accept: {e}"),
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream, root: &Path) -> Result<(), String> {
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
