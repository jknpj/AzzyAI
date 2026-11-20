// divinepride-sync: downloads the homunculus skill data AzzyAI uses from the Divine Pride API.
//
// Same flags, output path and byte-identical JSON as the tool it replaced
// (indent=1, sorted keys, UTF-8, like the Python writer).
//
// Usage (needs a free API key from your divine-pride.net profile page):
//     set DIVINEPRIDE_API_KEY=<your key>
//     divinepride-sync               # LATAM
//     divinepride-sync --server kROM # another server
//     divinepride-sync --refresh     # fetch again even if cached

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::process::ExitCode;
use std::time::Duration;

use serde_json::Value;

const API: &str = "https://www.divine-pride.net/api/database/Skill/{id}";

// homunculus skills defined in USER_AI/H_SkillList.lua (passives included, for the descriptions)
const SKILL_IDS: [i64; 53] = [
    8001, 8002, 8003, 8004, // Lif
    8005, 8006, 8007, 8008, // Amistr
    8009, 8010, 8011, 8012, // Filir
    8013, 8014, 8015, 8016, // Vanilmirth
    8018, 8019, 8020, 8021, 8053, 8054, // Sera
    8022, 8023, 8024, 8025, 8026, 8047, 8048, // Eira
    8027, 8028, 8029, 8030, 8036, 8037, 8038, 8050, 8051, // Eleanor
    8031, 8032, 8033, 8034, 8035, 8056, 8057, 8058, // Bayeri
    8039, 8040, 8041, 8042, 8043, 8044, 8045, // Dieter
];

struct Args {
    server: String,
    language: String,
    delay: f64,
    refresh: bool,
}

enum FetchError {
    Status(u16), // HTTP status
    Network(String),
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(Some(args)) => args,
        Ok(None) => return ExitCode::SUCCESS, // --help
        Err(msg) => {
            eprintln!("divinepride-sync: {msg}");
            return ExitCode::FAILURE;
        }
    };
    match run(&args) {
        Ok(code) => ExitCode::from(code),
        Err(msg) => {
            eprintln!("divinepride-sync: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Option<Args>, String> {
    let mut args = Args { server: "LATAM".into(), language: "en".into(), delay: 1.0, refresh: false };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} needs a value"));
        match arg.as_str() {
            "--server" => args.server = value("--server")?,
            "--language" => args.language = value("--language")?,
            "--delay" => args.delay = value("--delay")?.parse().map_err(|_| "--delay must be a number".to_string())?,
            "--refresh" => args.refresh = true,
            "-h" | "--help" => {
                print!("{}", include_str!("help.txt"));
                return Ok(None);
            }
            other => return Err(format!("unknown argument {other:?} (see --help)")),
        }
    }
    Ok(Some(args))
}

fn run(args: &Args) -> Result<u8, String> {
    let key = std::env::var("DIVINEPRIDE_API_KEY").unwrap_or_default().trim().to_string();
    if key.is_empty() {
        return Err("DIVINEPRIDE_API_KEY is not set (get it from your divine-pride.net profile page)".into());
    }
    let out = out_path(&args.server);
    let mut skills: BTreeMap<String, Value> = BTreeMap::new();
    if out.exists() && !args.refresh {
        let text = std::fs::read_to_string(&out).map_err(|e| format!("{}: {e}", out.display()))?;
        let doc: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", out.display()))?;
        if let Some(Value::Object(map)) = doc.get("skills") {
            skills = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        }
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("AzzyAI-harness")
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("HTTP client: {e}"))?;

    let mut missing: Vec<i64> = Vec::new();
    for sid in SKILL_IDS {
        if skills.contains_key(&sid.to_string()) {
            continue;
        }
        let data = match fetch(&client, sid, &key, &args.server, &args.language) {
            Ok(data) => data,
            Err(FetchError::Status(401)) => return Err("HTTP 401: the API key was refused".into()),
            Err(FetchError::Status(404)) => {
                missing.push(sid);
                continue;
            }
            Err(FetchError::Status(code)) => return Err(format!("HTTP {code} for skill {sid}")),
            Err(FetchError::Network(reason)) => return Err(format!("network error: {reason}")),
        };
        let region = data.get("region").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        println!("{sid} {name} ({region})");
        if region != "?" && region != args.server {
            println!("  warning: asked for {}, got {region}", args.server);
        }
        skills.insert(sid.to_string(), data);
        sleep(args.delay);
    }

    let doc = serde_json::json!({
        "server": args.server,
        "date": today(),
        "skills": Value::Object(skills.iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
    });
    // json.dump(indent=1, sort_keys=True, ensure_ascii=False): no trailing newline
    std::fs::write(&out, to_python_json(&doc, 0)).map_err(|e| format!("{}: {e}", out.display()))?;
    let not_found = if missing.is_empty() {
        "none".to_string()
    } else {
        missing.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ")
    };
    println!("wrote {}: {} skills, not found: {not_found}", out.display(), skills.len());
    Ok(0)
}

fn fetch(client: &reqwest::blocking::Client, sid: i64, key: &str, server: &str, language: &str) -> Result<Value, FetchError> {
    let url = format!("{}?apiKey={}", API.replace("{id}", &sid.to_string()), percent_encode(key));
    for _ in 0..5 {
        let resp = client
            .get(&url)
            .header("Accept", "application/json")
            .header("Accept-Language", language)
            .header("x-server", server)
            .send();
        let resp = match resp {
            Ok(r) => r,
            Err(e) => return Err(FetchError::Network(format!("{e}"))),
        };
        let status = resp.status().as_u16();
        if status == 429 {
            let wait = resp
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(30);
            println!("  rate limited, waiting {wait} s");
            sleep(wait as f64);
            continue;
        }
        if !(200..300).contains(&status) {
            return Err(FetchError::Status(status));
        }
        let text = resp.text().map_err(|e| FetchError::Network(format!("{e}")))?;
        return serde_json::from_str(&text).map_err(|e| FetchError::Network(format!("invalid JSON: {e}")));
    }
    Err(FetchError::Network("still rate limited, try again later".into()))
}

fn out_path(server: &str) -> std::path::PathBuf {
    repo_root().join("tools/harness/fixtures").join(format!("divinepride_{}.json", server.to_lowercase()))
}

fn repo_root() -> std::path::PathBuf {
    // the binary usually lives in tools/divinepride-sync/target[/debug|release]; with a custom
    // CARGO_TARGET_DIR it does not, so the working directory is tried as well
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    if let Some(exe) = std::env::current_exe().ok().and_then(|p| p.canonicalize().ok()) {
        candidates.push(exe);
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("placeholder"));
    }
    for start in candidates {
        let mut dir = start;
        for _ in 0..6 {
            if !dir.pop() { break; }
            if dir.join("tools").join("harness").is_dir() || dir.join("USER_AI").is_dir() {
                return dir;
            }
        }
    }
    std::path::PathBuf::from(".")
}

fn sleep(seconds: f64) {
    if seconds > 0.0 {
        std::thread::sleep(Duration::from_millis((seconds * 1000.0) as u64));
    }
}

fn percent_encode(value: &str) -> String {
    let mut out = String::new();
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            let _ = write!(out, "%{b:02X}");
        }
    }
    out
}

/// UTC date, like datetime.date.today().
fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    civil_from_days((secs / 86400) as i64)
}

fn civil_from_days(z: i64) -> String {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

/// Python's json.dump(indent=1, sort_keys=True, ensure_ascii=False), byte for byte: one space per
/// level, `", "`-free separators (each item on its own line), `{}`/`[]` inline when empty, no
/// trailing newline.
fn to_python_json(value: &Value, level: usize) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(true) => "true".into(),
        Value::Bool(false) => "false".into(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => quote(s),
        Value::Array(items) => {
            if items.is_empty() {
                return "[]".into();
            }
            let pad = " ".repeat(level + 1);
            let body: Vec<String> = items
                .iter()
                .map(|v| format!("{pad}{}", to_python_json(v, level + 1)))
                .collect();
            format!("[\n{}\n{}]", body.join(",\n"), " ".repeat(level))
        }
        Value::Object(map) => {
            if map.is_empty() {
                return "{}".into();
            }
            // serde_json's default map is already key-sorted; be explicit anyway
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let pad = " ".repeat(level + 1);
            let body: Vec<String> = keys
                .iter()
                .map(|k| format!("{pad}{}: {}", quote(k), to_python_json(&map[*k], level + 1)))
                .collect();
            format!("{{\n{}\n{}}}", body.join(",\n"), " ".repeat(level))
        }
    }
}

/// Python escapes only the quote, the backslash and the control characters, and leaves UTF-8 raw.
fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
