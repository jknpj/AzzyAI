// azzy-harness: runs the AzzyAI checks without Python.
//
// Same job as tools/harness/run.py: syntax-check every .lua file, verify the release number and
// that every shipped option is read by the AI, then run the offline scenarios in tools/harness/.
// The scenarios themselves stay in Lua: they load the real USER_AI files in the Lua 5.1 runtime the
// game embeds.
//
// Usage (run from the repository root):
//     cargo run --release -p azzy-harness --                 # all checks and scenarios
//     cargo run --release -p azzy-harness -- amistr_fresh  # only these scenarios
//     cargo run --release -p azzy-harness -- --verbose amistr_fresh
//     cargo run --release -p azzy-harness -- --profile --ticks 3000 amistr_fresh
//     cargo run --release -p azzy-harness -- --skip-syntax -- amistr_fresh
//     cargo run --release -p azzy-harness -- --globals
//
// The Lua interpreter comes from LUA/LUAC, then PATH (lua5.1, lua51, lua), then
// %LOCALAPPDATA%\lua51.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};


struct Args {
    scenarios: Vec<String>,
    ticks: u32,
    verbose: bool,
    trace: bool,
    profile: bool,
    globals: bool,
    skip_syntax: bool,
}

fn usage() -> &'static str {
    "azzy-harness: syntax, version, option and AI scenario checks

Usage: azzy-harness [options] [scenario...]

  --ticks N        AI calls per scenario, 100 ms each (default 300)
  --verbose        print every cast
  --trace          print TraceAI output
  --profile        print time per AI() call and the hottest functions
  --globals        only list globals assigned inside functions of USER_AI
  --skip-syntax    skip the syntax, version and option checks
  -h, --help       this help"
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        scenarios: Vec::new(),
        ticks: 300,
        verbose: false,
        trace: false,
        profile: false,
        globals: false,
        skip_syntax: false,
    };
    let mut it = env::args().skip(1).peekable();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", usage());
                std::process::exit(0);
            }
            "--ticks" => {
                let value = it.next().ok_or("--ticks needs a number")?;
                args.ticks = value.parse().map_err(|_| format!("--ticks: {value} is not a number"))?;
            }
            "--verbose" => args.verbose = true,
            "--trace" => args.trace = true,
            "--profile" => args.profile = true,
            "--globals" => args.globals = true,
            "--skip-syntax" => args.skip_syntax = true,
            other if other.starts_with('-') => return Err(format!("unknown option {other}\n\n{}", usage())),
            other => args.scenarios.push(other.to_string()),
        }
    }
    Ok(args)
}

/// Repository root: walk up from the current directory until USER_AI/Const_.lua is found.
pub(crate) fn repo_root() -> Result<PathBuf, String> {
    let mut dir = env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if dir.join("USER_AI").join("Const_.lua").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err("run it from the repository root (USER_AI/Const_.lua not found)".into());
        }
    }
}

pub(crate) fn find_tool(env_name: &str, names: &[&str]) -> String {
    if let Some(value) = env::var_os(env_name) {
        if !value.is_empty() {
            return value.to_string_lossy().to_string();
        }
    }
    for name in names {
        if let Some(path) = which(name) {
            return path;
        }
    }
    if let Some(local) = env::var_os("LOCALAPPDATA") {
        for name in names {
            let path = Path::new(&local).join("lua51").join(format!("{name}.exe"));
            if path.is_file() {
                return path.display().to_string();
            }
        }
    }
    eprintln!("Lua 5.1 not found: set {env_name} or put one of {names:?} on PATH");
    std::process::exit(2);
}

fn which(name: &str) -> Option<String> {
    let paths = env::var_os("PATH")?;
    for dir in env::split_paths(&paths) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate.display().to_string());
        }
        #[cfg(windows)]
        {
            let exe = dir.join(format!("{name}.exe"));
            if exe.is_file() {
                return Some(exe.display().to_string());
            }
        }
    }
    None
}

fn lua_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if name != ".git" && name != "node_modules" && name != "target" {
                lua_files(&path, out);
            }
        } else if name.ends_with(".lua") {
            out.push(path);
        }
    }
}

fn check_syntax(luac: &str, repo: &Path) -> bool {
    let mut files = Vec::new();
    lua_files(repo, &mut files);
    files.sort();
    let mut failed = 0;
    for path in &files {
        let output = Command::new(luac).arg("-p").arg(path).output();
        match output {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                failed += 1;
                let text = String::from_utf8_lossy(&output.stderr).trim().to_string();
                let text = if text.is_empty() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    text
                };
                println!("SYNTAX ERROR {text}");
            }
            Err(error) => {
                failed += 1;
                println!("SYNTAX ERROR {}: {error}", path.display());
            }
        }
    }
    println!("syntax: {} Lua files, {failed} with errors", files.len());
    failed == 0
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// The value of a `Key="..."` line, ignoring comments.
fn assignment(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("--") {
            continue;
        }
        if let Some(rest) = line.strip_prefix(key) {
            let rest = rest.trim_start();
            if !rest.starts_with('=') {
                continue;
            }
            let rest = rest[1..].trim();
            let rest = rest.strip_prefix('"')?;
            let end = rest.find('"')?;
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn check_version(user_ai: &Path, repo: &Path) -> bool {
    let Some(code) = assignment(&read(&user_ai.join("Const_.lua")), "CVersion") else {
        println!("VERSION ERROR: CVersion=\"MAJOR.MINOR.PATCH\" not found in USER_AI/Const_.lua");
        return false;
    };
    let parts: Vec<&str> = code.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|p| p.parse::<u32>().is_err()) {
        println!("VERSION ERROR: CVersion {code} is not MAJOR.MINOR.PATCH");
        return false;
    }
    let expected = format!("{}.{}", parts[0], parts[1]);
    let mut ok = true;
    let changelog = read(&repo.join("CHANGELOG.md"));
    let release = changelog.lines().find_map(|line| {
        let lower = line.to_lowercase();
        let at = lower.find("current release:")?;
        let rest = line[at + "current release:".len()..].trim_start_matches(['*', ' ']);
        let end = rest.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(rest.len());
        Some(rest[..end].to_string())
    });
    match release {
        Some(release) if release == code => {}
        Some(release) => {
            println!("VERSION ERROR: Const_.lua says {code}, CHANGELOG.md says {release}");
            ok = false;
        }
        None => {
            println!("VERSION ERROR: Const_.lua says {code}, CHANGELOG.md has no \"Current release\" line");
            ok = false;
        }
    }
    for (name, var) in [
        ("AI.lua", "Version"),
        ("AI_M.lua", "Version"),
        ("AI_main.lua", "MainVersion"),
        ("AzzyUtil.lua", "AUVersion"),
    ] {
        let value = assignment(&read(&user_ai.join(name)), var);
        if value.as_deref() != Some(expected.as_str()) {
            println!(
                "VERSION ERROR: {name} {var} is {}, release {code} needs \"{expected}\"",
                value.unwrap_or_else(|| "missing".into())
            );
            ok = false;
        }
    }
    if ok {
        println!("version: {code} (file check {expected})");
    }
    ok
}

/// True when `name` appears as a whole word (so `Foo` does not match `FooBar` or `MyFoo`).
fn contains_word(haystack: &str, name: &str) -> bool {
    let bytes = haystack.as_bytes();
    let needle = name.as_bytes();
    let mut start = 0;
    while let Some(at) = haystack[start..].find(name) {
        let at = start + at;
        let before_ok = at == 0 || !is_word(bytes[at - 1]);
        let after = at + needle.len();
        let after_ok = after >= bytes.len() || !is_word(bytes[after]);
        if before_ok && after_ok {
            return true;
        }
        start = at + name.len().max(1);
    }
    false
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn check_options(user_ai: &Path) -> bool {
    const CONFIG_FILES: [&str; 3] = ["Defaults.lua", "H_Config.lua", "M_Config.lua"];
    let mut defined: Vec<(String, Vec<&str>)> = Vec::new();
    for file in CONFIG_FILES {
        for line in read(&user_ai.join(file)).lines() {
            let line = line.trim_start();
            if line.starts_with("--") {
                continue;
            }
            let Some(eq) = line.find('=') else { continue };
            let name = line[..eq].trim();
            if name.is_empty() || !name.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
                continue;
            }
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                continue;
            }
            match defined.iter_mut().find(|(existing, _)| existing == name) {
                Some((_, files)) => files.push(file),
                None => defined.push((name.to_string(), vec![file])),
            }
        }
    }
    let mut blob = String::new();
    if let Ok(entries) = fs::read_dir(user_ai) {
        let mut names: Vec<String> = entries
            .flatten()
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "lua"))
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        for name in names {
            if CONFIG_FILES.contains(&name.as_str()) || name == "Const_.lua" {
                continue;
            }
            blob.push_str(&read(&user_ai.join(&name)));
            blob.push('\n');
        }
    }
    let mut unused = 0;
    defined.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, files) in &defined {
        if !contains_word(&blob, name) {
            unused += 1;
            println!(
                "OPTION ERROR: {name} is in {} but nothing in USER_AI reads it",
                files.join("/")
            );
        }
    }
    println!(
        "options: {} in {}, {unused} unread",
        defined.len(),
        CONFIG_FILES.join("/")
    );
    unused == 0
}

fn all_scenarios(harness: &Path) -> Vec<String> {
    let text = read(&harness.join("scenarios.lua"));
    let Some(start) = text.find("SCENARIOS = {") else { return Vec::new() };
    let body = &text[start + "SCENARIOS = {".len()..];
    let mut names = Vec::new();
    for line in body.lines() {
        let Some(rest) = line.strip_prefix('\t') else { continue };
        // scenario entries sit at one tab; fields inside them are at two
        if rest.starts_with(['\t', ' ']) {
            continue;
        }
        let Some(eq) = rest.find('=') else { continue };
        let name = rest[..eq].trim();
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            continue;
        }
        let value = rest[eq + 1..].trim_start();
        if value.starts_with('{') || value.contains('(') {
            names.push(name.to_string());
        }
    }
    names
}

/// Copies USER_AI into the sandbox, skipping the files the harness never reads.
fn copy_tree(src: &Path, dst: &Path, skip_types: bool) {
    let Ok(entries) = fs::read_dir(src) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "data" || name == "backup" || name.ends_with(".exe") || name.ends_with(".pdb")
            || name.ends_with(".pdf") || (skip_types && name == "Types")
        {
            continue;
        }
        let target = dst.join(&name);
        if path.is_dir() {
            let _ = fs::create_dir_all(&target);
            copy_tree(&path, &target, false);
        } else if fs::copy(&path, &target).is_err() {
            eprintln!("warning: could not copy {}", path.display());
        }
    }
}

/// Runs one scenario in a sandbox and returns (passed, combined stdout+stderr).
pub(crate) fn run_scenario_output(
    lua: &str,
    harness: &Path,
    repo: &Path,
    name: &str,
    ticks: u32,
    envs: &[(String, String)],
) -> Option<(bool, String)> {
    let user_ai = repo.join("USER_AI");
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let slot = NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let root = env::temp_dir().join(format!("azzyai-{}-{slot}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let dst = root.join("AI").join("USER_AI");
    if fs::create_dir_all(dst.join("data")).is_err() {
        println!("cannot create the sandbox for {name}");
        return None;
    }
    // Per-run options come from the caller's env list, not the process environment: the benchmark
    // runs several configurations in parallel and each one has its own config/Types choice.
    let option = |key: &str| {
        envs.iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
    };
    copy_tree(&user_ai, &dst, option("HARNESS_NO_TYPES").is_some());
    let config = option("HARNESS_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|| harness.join("fixtures").join("H_Config.lua"));
    let _ = fs::copy(&config, dst.join("H_Config.lua"));
    let merc = option("HARNESS_MCONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|| harness.join("fixtures").join("M_Config.lua"));
    if merc.is_file() {
        let _ = fs::copy(&merc, dst.join("M_Config.lua"));
    }
    // Branch-specific presets: a file under fixtures/Types overrides the checked-in Types preset for
    // the run, so the scenarios always exercise the generic AI. run.py did this before the Rust port;
    // without it a branch whose Types/<homunculus>.lua is tuned (the hero branch's Eleanor) fails
    // scenarios that assume the default preset.
    if option("HARNESS_NO_TYPES").is_none() {
        let fixtures_types = harness.join("fixtures").join("Types");
        if let Ok(entries) = fs::read_dir(&fixtures_types) {
            let _ = fs::create_dir_all(dst.join("Types"));
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    let _ = fs::copy(entry.path(), dst.join("Types").join(entry.file_name()));
                }
            }
        }
    }
    let mut command = Command::new(lua);
    command
        .arg(harness.join("harness.lua"))
        .arg(name)
        .arg(ticks.to_string())
        .current_dir(&root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in envs {
        command.env(key, value);
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            println!("cannot run {name}: {error}");
            let _ = fs::remove_dir_all(&root);
            return None;
        }
    };
    // Drain both pipes while waiting: a verbose run can print more than the pipe buffer.
    let mut stdout = child.stdout.take();
    let mut stderr = child.stderr.take();
    let out_reader = std::thread::spawn(move || {
        let mut text = Vec::new();
        if let Some(pipe) = stdout.as_mut() {
            let _ = std::io::Read::read_to_end(pipe, &mut text);
        }
        text
    });
    let err_reader = std::thread::spawn(move || {
        let mut text = Vec::new();
        if let Some(pipe) = stderr.as_mut() {
            let _ = std::io::Read::read_to_end(pipe, &mut text);
        }
        text
    });
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() < Duration::from_secs(300) => {
                std::thread::sleep(Duration::from_millis(50));
            }
            _ => {
                let _ = child.kill();
                break None;
            }
        }
    };
    let mut text = String::from_utf8_lossy(&out_reader.join().unwrap_or_default()).to_string();
    text.push_str(&String::from_utf8_lossy(&err_reader.join().unwrap_or_default()));
    let _ = fs::remove_dir_all(&root);
    Some((status.is_some_and(|status| status.success()), text))
}

/// Checks that webgui/templates.js embeds the shipped H_Config.lua and M_Config.lua (the web page
/// starts a new configuration from those copies).
fn check_templates() -> i32 {
    let repo = match repo_root() {
        Ok(repo) => repo,
        Err(error) => {
            eprintln!("{error}");
            return 2;
        }
    };
    let templates = repo.join("webgui").join("templates.js");
    if !templates.is_file() {
        println!("webgui/templates.js not found (submodule not checked out), skipping");
        return 0;
    }
    let Ok(text) = fs::read_to_string(&templates) else {
        eprintln!("cannot read {}", templates.display());
        return 2;
    };
    let mut failed = false;
    for name in ["H_Config.lua", "M_Config.lua"] {
        let marker = format!("'{name}': String.raw`");
        let Some(start) = text.find(&marker) else {
            println!("ERROR: webgui/templates.js has no '{name}' template");
            failed = true;
            continue;
        };
        let body_start = start + marker.len();
        let Some(length) = text[body_start..].find('`') else {
            println!("ERROR: webgui/templates.js has no end of the '{name}' template");
            failed = true;
            continue;
        };
        let embedded = normalize(&text[body_start..body_start + length]);
        let shipped = normalize(&read(&repo.join("USER_AI").join(name)));
        if embedded == shipped {
            println!("{name}: matches USER_AI/{name}");
            continue;
        }
        failed = true;
        println!("ERROR: the web GUI copy of {name} differs from USER_AI/{name}");
        let shipped_lines: Vec<&str> = shipped.split('\n').collect();
        let embedded_lines: Vec<&str> = embedded.split('\n').collect();
        for index in 0..shipped_lines.len().max(embedded_lines.len()) {
            let a = shipped_lines.get(index).copied().unwrap_or("<missing>");
            let b = embedded_lines.get(index).copied().unwrap_or("<missing>");
            if a != b {
                println!("  line {}: USER_AI: {a}", index + 1);
                println!("  line {}: webgui : {b}", index + 1);
                break;
            }
        }
        println!("  copy USER_AI/{name} into webgui/templates.js (String.raw body) to fix it");
    }
    if failed {
        1
    } else {
        0
    }
}

fn normalize(text: &str) -> String {
    text.replace("\r\n", "\n").trim_matches('\n').to_string()
}

fn check_globals(luac: &str, user_ai: &Path) {
    let mut found: Vec<(String, Vec<String>)> = Vec::new();
    let mut names: Vec<String> = fs::read_dir(user_ai)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .filter(|name| name.ends_with(".lua"))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    for name in names {
        let output = Command::new(luac)
            .arg("-l")
            .arg("-p")
            .arg(user_ai.join(&name))
            .output();
        let Ok(output) = output else { continue };
        let text = String::from_utf8_lossy(&output.stdout);
        let mut scope: Option<u32> = None;
        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("main <") {
                scope = None;
                let _ = rest;
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("function <") {
                scope = rest
                    .split(':')
                    .nth(1)
                    .and_then(|rest| rest.split(',').next())
                    .and_then(|line| line.parse().ok());
                continue;
            }
            if let (Some(scope), Some(at)) = (scope, trimmed.find("SETGLOBAL")) {
                let rest = &trimmed[at..];
                if let Some(semi) = rest.find(';') {
                    let global = rest[semi + 1..].trim().to_string();
                    if !global.is_empty() {
                        let entry = format!("{name}:{scope}");
                        match found.iter_mut().find(|(name, _)| *name == global) {
                            Some((_, where_)) => where_.push(entry),
                            None => found.push((global, vec![entry])),
                        }
                    }
                }
            }
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    for (global, where_) in &found {
        println!("  {global:28} {}", where_.join(", "));
    }
    println!("globals assigned inside functions: {}", found.len());
}

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    match argv.first().map(String::as_str) {
        Some("templates-check") => std::process::exit(check_templates()),
        Some("globals") => {
            let repo = match repo_root() {
                Ok(repo) => repo,
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(2);
                }
            };
            let luac = find_tool("LUAC", &["luac5.1", "luac51", "luac"]);
            check_globals(&luac, &repo.join("USER_AI"));
            std::process::exit(0);
        }
        _ => {}
    }
    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };
    let repo = match repo_root() {
        Ok(repo) => repo,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };
    let harness = repo.join("tools").join("harness");
    let user_ai = repo.join("USER_AI");
    let lua = find_tool("LUA", &["lua5.1", "lua51", "lua"]);
    let luac = find_tool("LUAC", &["luac5.1", "luac51", "luac"]);

    if args.globals {
        check_globals(&luac, &user_ai);
        return;
    }

    let mut ok = true;
    if !args.skip_syntax {
        ok = check_syntax(&luac, &repo);
        ok = check_version(&user_ai, &repo) && ok;
        ok = check_options(&user_ai) && ok;
    }

    let envs = vec![
        (
            "HARNESS_VERBOSE".to_string(),
            if args.verbose { "1" } else { "0" }.to_string(),
        ),
        (
            "HARNESS_TRACE".to_string(),
            if args.trace { "1" } else { "0" }.to_string(),
        ),
        (
            "HARNESS_PROFILE".to_string(),
            if args.profile { "1" } else { "0" }.to_string(),
        ),
    ];
    let names = if args.scenarios.is_empty() {
        all_scenarios(&harness)
    } else {
        args.scenarios.clone()
    };
    let mut passed = 0;
    for name in &names {
        match run_scenario_output(&lua, &harness, &repo, name, args.ticks, &envs) {
            Some((ok, output)) => {
                print!("{output}");
                if ok {
                    passed += 1;
                }
            }
            None => {}
        }
    }
    println!("scenarios: {passed}/{} passed", names.len());
    ok = ok && passed == names.len();
    std::process::exit(if ok { 0 } else { 1 });
}
