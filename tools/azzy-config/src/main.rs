// azzy-config: command-line view of a Lua configuration, useful on its own and as the check for
// the model a native GUI would use.
//
//     azzy-config list <file>              name=value per line
//     azzy-config get <file> NAME          one value
//     azzy-config sections <file>          section -> options
//     azzy-config diff <a> <b>             what differs between two files
//     azzy-config set <file> NAME=VALUE    in place, touching only that line
use std::process::ExitCode;

use azzy_config::Config;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = || {
        eprintln!("usage: azzy-config <list|get|sections|diff|set> <file> [name[=value]]");
        ExitCode::FAILURE
    };
    let Some(command) = args.first() else { return usage() };
    match command.as_str() {
        "list" => {
            let Some(path) = args.get(1) else { return usage() };
            match Config::load(std::path::Path::new(path)) {
                Ok(config) => {
                    for a in config.assignments() {
                        println!("{}={}", a.name, a.value);
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => fail(e),
            }
        }
        "get" => {
            let (Some(path), Some(name)) = (args.get(1), args.get(2)) else { return usage() };
            match Config::load(std::path::Path::new(path)) {
                Ok(config) => match config.get(name) {
                    Some(value) => {
                        println!("{value}");
                        ExitCode::SUCCESS
                    }
                    None => {
                        eprintln!("azzy-config: {name} is not in {path}");
                        ExitCode::FAILURE
                    }
                },
                Err(e) => fail(e),
            }
        }
        "sections" => {
            let Some(path) = args.get(1) else { return usage() };
            match Config::load(std::path::Path::new(path)) {
                Ok(config) => {
                    for (section, names) in config.sections() {
                        let label = if section.is_empty() { "(no section)" } else { &section };
                        println!("{label} ({}): {}", names.len(), names.join(", "));
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => fail(e),
            }
        }
        "diff" => {
            let (Some(a), Some(b)) = (args.get(1), args.get(2)) else { return usage() };
            match (Config::load(std::path::Path::new(a)), Config::load(std::path::Path::new(b))) {
                (Ok(left), Ok(right)) => {
                    let diff = left.diff(&right);
                    if diff.is_empty() {
                        println!("identical");
                    } else {
                        for line in diff {
                            println!("{line}");
                        }
                    }
                    ExitCode::SUCCESS
                }
                (Err(e), _) | (_, Err(e)) => fail(e),
            }
        }
        "set" => {
            let (Some(path), Some(pair)) = (args.get(1), args.get(2)) else { return usage() };
            let Some((name, value)) = pair.split_once('=') else {
                eprintln!("azzy-config: expected NAME=VALUE");
                return ExitCode::FAILURE;
            };
            match Config::load(std::path::Path::new(path)) {
                Ok(mut config) => {
                    let before = config.get(name).map(|v| v.to_string());
                    if !config.set(name, value) {
                        eprintln!("azzy-config: {name} is not in {path} (nothing written)");
                        return ExitCode::FAILURE;
                    }
                    if let Err(e) = config.save(std::path::Path::new(path)) {
                        return fail(e);
                    }
                    println!("{name}: {} -> {value}", before.unwrap_or_default());
                    ExitCode::SUCCESS
                }
                Err(e) => fail(e),
            }
        }
        _ => usage(),
    }
}

fn fail(error: std::io::Error) -> ExitCode {
    eprintln!("azzy-config: {error}");
    ExitCode::FAILURE
}
