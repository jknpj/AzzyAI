//! The benchmark front ends: the same runs `homun_bench.py`, `eleanor_bench.py` and
//! `preset_report.py` did, now on top of the same scenario runner as `azzy-harness`.
//!
//! They are subcommands:
//!     azzy-harness bench <homunculus> [config...] [--mobs N] [--spregen N] [--seconds N]
//!                                       [--def N] [--mdef N] [--config PATH]
//!     azzy-harness eleanor [configuration...] [--attack-delay MS]
//!     azzy-harness preset-report [--old-ref REF] [--out DIR]

use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::{find_tool, repo_root, run_scenario_output};

struct Result {
    dps: i64,
    damage: i64,
    sp: i64,
    refused: i64,
    skills: Vec<(String, i64, i64)>,
}

fn number(text: &str) -> i64 {
    text.replace(',', "").parse().unwrap_or(0)
}

fn commas(value: i64) -> String {
    let mut out = String::new();
    let digits = value.abs().to_string();
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if value < 0 {
        format!("-{out}")
    } else {
        out
    }
}

fn pretty(name: &str) -> String {
    name.trim_start_matches("MH_")
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_report(output: &str) -> Option<Result> {
    let damage = Regex::new(r"damage: ([\d,]+) in ([\d.]+) s = ([\d,]+) DPS").unwrap();
    let casts = Regex::new(r"casts: (\d+) accepted, (\d+) refused by the server, (\d+) SP spent").unwrap();
    let skill = Regex::new(r"(?m)^\s+(MH_\w+|Normal attack)\s+x(\d+)\s+([\d,]+)").unwrap();
    let total = damage.captures(output)?;
    let cast = casts.captures(output)?;
    let skills = skill
        .captures_iter(output)
        .filter(|caps| &caps[1] != "Normal attack")
        .map(|caps| {
            (
                caps[1].to_string(),
                caps[2].parse().unwrap_or(0),
                number(&caps[3]),
            )
        })
        .collect();
    Some(Result {
        dps: number(&total[3]),
        damage: number(&total[1]),
        sp: cast[3].parse().unwrap_or(0),
        refused: cast[2].parse().unwrap_or(0),
        skills,
    })
}

fn sandbox_env(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

fn run_homun_once(
    homun: &str,
    overrides: &str,
    mobs: u32,
    spregen: u32,
    seconds: u32,
    config: &Path,
    mob_def: Option<u32>,
    mob_mdef: Option<u32>,
    no_types: bool,
) -> Option<Result> {
    let repo = repo_root().ok()?;
    let lua = find_tool("LUA", &["lua5.1", "lua51", "lua"]);
    let harness = repo.join("tools").join("harness");
    let mut env: Vec<(&str, String)> = vec![
        ("HARNESS_DAMAGE", "1".into()),
        ("HBENCH_SET", overrides.into()),
        ("HBENCH_MOBS", mobs.to_string()),
        ("HBENCH_SECONDS", seconds.to_string()),
        ("HARNESS_CONFIG", config.display().to_string()),
    ];
    if no_types {
        env.push(("HARNESS_NO_TYPES", "1".into()));
    }
    if let Some(value) = mob_def {
        env.push(("HBENCH_DEF", value.to_string()));
    }
    if let Some(value) = mob_mdef {
        env.push(("HBENCH_MDEF", value.to_string()));
    }
    if spregen > 0 {
        env.push(("HBENCH_SPREGEN", spregen.to_string()));
    }
    let envs: Vec<(String, String)> = env.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    let (_, output) = run_scenario_output(&lua, &harness, &repo, &format!("hbench_{homun}"), 300, &envs)?;
    parse_report(&output)
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|arg| arg == flag).and_then(|index| args.get(index + 1).cloned())
}

pub fn run_homun(args: &[String]) -> i32 {
    let homun = match args.first() {
        Some(value) if !value.starts_with('-') => value.clone(),
        _ => {
            eprintln!("usage: azzy-harness bench <eira|bayeri|sera|dieter|eleanor> [configs...] [flags]");
            return 2;
        }
    };
    let mut mobs: u32 = 1;
    let mut spregen: u32 = 0;
    let mut seconds: u32 = 180;
    let mut mob_def: Option<u32> = None;
    let mut mob_mdef: Option<u32> = None;
    let mut config_text: Option<String> = None;
    let mut configs: Vec<String> = Vec::new();
    let mut it = args.iter().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--mobs" => mobs = it.next().and_then(|v| v.parse().ok()).unwrap_or(mobs),
            "--spregen" => spregen = it.next().and_then(|v| v.parse().ok()).unwrap_or(spregen),
            "--seconds" => seconds = it.next().and_then(|v| v.parse().ok()).unwrap_or(seconds),
            "--def" => mob_def = it.next().and_then(|v| v.parse().ok()),
            "--mdef" => mob_mdef = it.next().and_then(|v| v.parse().ok()),
            "--config" => config_text = it.next().cloned(),
            other => configs.push(other.to_string()),
        }
    }
    let repo = repo_root().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    });
    let config = config_text
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.join("USER_AI").join("H_Config.lua"));
    let configs = if configs.is_empty() { vec![String::new()] } else { configs };

    let mut results = Vec::new();
    for config_text in &configs {
        let overrides = if config_text.is_empty() || config_text == "base" { "" } else { config_text.as_str() };
        match run_homun_once(&homun, overrides, mobs, spregen, seconds, &config, mob_def, mob_mdef, false) {
            Some(result) => results.push((config_text.clone(), result)),
            None => {
                eprintln!("no damage report for {config_text:?}");
                return 1;
            }
        }
    }
    let base = results[0].1.dps.max(1);
    println!(
        "{homun}: {mobs} monster(s), {seconds} s, {} SP/s regeneration",
        if spregen == 0 { "default".to_string() } else { spregen.to_string() }
    );
    println!("{:>10} {:>9} {:>7} {:>8}  configuration -> casts", "DPS", "vs first", "dmg/SP", "refused");
    for (config_text, result) in &results {
        let per_sp = if result.sp != 0 {
            (result.damage as f64 / result.sp as f64).round() as i64
        } else {
            0
        };
        let mut skills = result.skills.clone();
        skills.sort_by(|a, b| b.1.cmp(&a.1));
        let casts = skills
            .iter()
            .map(|(name, count, _)| format!("{} {count}", pretty(name)))
            .collect::<Vec<_>>()
            .join(", ");
        let change = 100.0 * (result.dps as f64 / base as f64 - 1.0);
        println!(
            "{:>10} {:+8.0}% {:>7} {:>8}  {} -> {}",
            commas(result.dps),
            change,
            if result.sp != 0 { per_sp.to_string() } else { "n/a".into() },
            result.refused,
            if config_text.is_empty() { "(fixture)" } else { config_text.as_str() },
            casts
        );
    }
    0
}

const ELEANOR_CONFIGS: [(&str, &str); 10] = [
    ("attack_only", "no skills: normal attacks only"),
    ("sonic_only", "fighting: Sonic Claw only"),
    ("fighting_none", "fighting: combo (Sonic>Silvervein>Midnight)"),
    ("fighting_blazing", "fighting: combo + Blazing And Furious"),
    ("fighting_fighter", "fighting: combo + The One Fighter Rises"),
    ("fighting_both", "fighting: combo + both"),
    ("grappling_none", "grappling: combo (Tinder>CBC>EQC)"),
    ("grappling_blazing", "grappling: combo + Blazing And Furious"),
    ("grappling_fighter", "grappling: combo + The One Fighter Rises"),
    ("grappling_both", "grappling: combo + both"),
];

fn run_eleanor_once(name: &str, attack_delay: Option<u32>) -> Option<Result> {
    let repo = repo_root().ok()?;
    let lua = find_tool("LUA", &["lua5.1", "lua51", "lua"]);
    let harness = repo.join("tools").join("harness");
    let mut envs = sandbox_env(&[("HARNESS_DAMAGE", "1")]);
    if let Some(delay) = attack_delay {
        envs.push(("ELEANOR_ATTACK_DELAY".into(), delay.to_string()));
    }
    let (_, output) = run_scenario_output(&lua, &harness, &repo, &format!("eleanor_bench_{name}"), 300, &envs)?;
    parse_report(&output)
}

pub fn run_eleanor(args: &[String]) -> i32 {
    let delay: Option<u32> = flag_value(args, "--attack-delay").and_then(|v| v.parse().ok());
    let mut names: Vec<String> = Vec::new();
    let mut skip_next = false;
    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--attack-delay" {
            skip_next = true;
            continue;
        }
        if arg.starts_with('-') {
            continue;
        }
        names.push(arg.clone());
    }
    if names.is_empty() {
        names = ELEANOR_CONFIGS.iter().map(|(name, _)| name.to_string()).collect();
    }
    let mut results = Vec::new();
    for name in &names {
        let Some(result) = run_eleanor_once(name, delay) else {
            eprintln!("{name}: no damage report");
            return 1;
        };
        results.push((name.clone(), result));
    }
    let base = results
        .iter()
        .find(|(name, _)| name == "sonic_only")
        .map(|(_, result)| result.dps as f64);
    println!(
        "{:44} {:>9} {:>9} {:>7} {:>8}  skills (casts)",
        "configuration", "DPS", "vs Sonic", "dmg/SP", "refused"
    );
    for (name, result) in &results {
        let per_sp = if result.sp != 0 {
            (result.damage as f64 / result.sp as f64).round() as i64
        } else {
            0
        };
        let mut skills = result.skills.clone();
        skills.sort_by(|a, b| b.2.cmp(&a.2));
        let casts = skills
            .iter()
            .filter(|(skill, _, _)| skill != "Normal attack")
            .map(|(skill, count, _)| {
                format!("{} {count}", pretty(&skill.replace("CRAW", "CLAW")))
            })
            .collect::<Vec<_>>()
            .join(", ");
        let label = ELEANOR_CONFIGS
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, label)| *label)
            .unwrap_or(name.as_str());
        let vs = match base {
            Some(base) if name != "sonic_only" => {
                format!("{:>9}", format!("{:+.0}%", 100.0 * (result.dps as f64 / base - 1.0)))
            }
            _ => " ".repeat(9),
        };
        println!(
            "{:44} {:>9} {vs} {:>7} {:>8}  {}",
            label,
            commas(result.dps),
            if result.sp != 0 { per_sp.to_string() } else { "n/a".into() },
            result.refused,
            casts
        );
    }
    println!("expected values at the pinned rAthena commit; the model ignores status effects and buffs");
    0
}

// ---------- preset-report ----------

const PACKS: [u32; 4] = [1, 2, 3, 5];
const HOMUNCULI: [&str; 5] = ["eira", "bayeri", "sera", "dieter", "eleanor"];

// (homunculus, monsters, SP/s or 0 for the default, label, overrides applied to the current preset)
const OPTIONS: &[(&str, u32, u32, &str, &str)] = &[
    ("eira", 1, 0, "current preset", ""),
    ("eira", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("eira", 1, 0, "Twister Cutter on", "UseEiraTwisterCutter=1"),
    ("eira", 1, 0, "Twister Cutter on, AutoMobCount 1", "UseEiraTwisterCutter=1,AutoMobCount=1"),
    ("eira", 1, 30, "current preset", ""),
    ("eira", 1, 30, "Twister Cutter on", "UseEiraTwisterCutter=1"),
    ("eira", 3, 0, "current preset", ""),
    ("eira", 3, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 1, 0, "current preset", ""),
    ("bayeri", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("bayeri", 1, 0, "Glanzen Spies off", "UseBayeriGlanzenSpies=0"),
    ("bayeri", 1, 0, "Heilige Pferd off", "UseBayeriHeiligePferd=0"),
    ("bayeri", 2, 80, "current preset", ""),
    ("bayeri", 2, 80, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 2, 80, "Heilige Pferd off", "UseBayeriHeiligePferd=0"),
    ("bayeri", 3, 80, "current preset", ""),
    ("bayeri", 3, 80, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 3, 80, "AutoMobCount 4", "AutoMobCount=4"),
    ("bayeri", 3, 80, "Heilige Pferd off", "UseBayeriHeiligePferd=0"),
    ("sera", 1, 0, "current preset", ""),
    ("sera", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("sera", 1, 0, "Needle Stinger on", "UseSeraNeedleStinger=1"),
    ("sera", 3, 0, "current preset", ""),
    ("sera", 3, 0, "Needle Stinger on", "UseSeraNeedleStinger=1"),
    ("sera", 3, 0, "Toxin of Mandara off", "UseSeraToxinOfMandara=0"),
    ("dieter", 1, 0, "current preset", ""),
    ("dieter", 1, 0, "AutoMobCount 2 (the global value)", "AutoMobCount=2"),
    ("dieter", 1, 0, "Volcanic Ash as an attack on", "UseDieterAshAsAttack=1"),
    ("dieter", 1, 0, "AutoMobCount 1, Blast Forge off (previous preset)", "AutoMobCount=1,UseDieterBlastForge=0"),
    ("dieter", 2, 0, "current preset", ""),
    ("dieter", 2, 0, "Blast Forge off", "UseDieterBlastForge=0"),
    ("dieter", 2, 0, "AutoMobCount 2 (the global value)", "AutoMobCount=2"),
    ("dieter", 2, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("dieter", 2, 0, "Lava Slide off", "UseDieterLavaSlide=0"),
    ("dieter", 3, 0, "current preset", ""),
    ("dieter", 3, 0, "Blast Forge off", "UseDieterBlastForge=0"),
    ("dieter", 3, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("dieter", 3, 0, "Magma Flow off", "UseDieterMagmaFlow=0"),
    ("eleanor", 1, 12, "current preset", ""),
    ("eleanor", 1, 12, "Blazing And Furious off", "UseEleanorBlazingAndFurious=0"),
    ("eleanor", 1, 22, "current preset", ""),
    ("eleanor", 1, 22, "Blazing And Furious off", "UseEleanorBlazingAndFurious=0"),
    ("eleanor", 1, 22, "AutoMobCount 1", "AutoMobCount=1"),
    ("eleanor", 1, 22, "The One Fighter Rises off", "UseEleanorTheOneFighterRises=0"),
    ("eleanor", 1, 22, "AutoComboSpheres 3", "AutoComboSpheres=3"),
    ("eleanor", 1, 22, "AutoComboSpheres 6", "AutoComboSpheres=6"),
    ("eleanor", 3, 22, "current preset", ""),
    ("eleanor", 3, 22, "Blazing And Furious off", "UseEleanorBlazingAndFurious=0"),
    ("eleanor", 3, 22, "The One Fighter Rises off", "UseEleanorTheOneFighterRises=0"),
    ("eleanor", 3, 22, "AutoComboSpheres 3", "AutoComboSpheres=3"),
    ("eleanor", 3, 40, "current preset", ""),
    ("eleanor", 3, 40, "Blazing And Furious off", "UseEleanorBlazingAndFurious=0"),
    ("eleanor", 5, 22, "current preset", ""),
    ("eleanor", 5, 22, "Blazing And Furious off", "UseEleanorBlazingAndFurious=0"),
];

struct Job {
    homun: &'static str,
    mobs: u32,
    spregen: u32,
    overrides: &'static str,
    config: PathBuf,
    no_types: bool,
}

fn spot_spregen(homun: &str) -> u32 {
    if homun == "eleanor" {
        22
    } else {
        60
    }
}

fn casts_text(skills: &[(String, i64, i64)]) -> String {
    let mut sorted = skills.to_vec();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
        .iter()
        .map(|(name, count, _)| format!("{} {count}", pretty(&name.replace("CRAW", "CLAW"))))
        .collect::<Vec<_>>()
        .join("; ")
}

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git").args(args).current_dir(repo).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parallel_run(jobs: &[Job]) -> Option<Vec<Result>> {
    let total = jobs.len();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let slots: std::sync::Mutex<Vec<Option<Result>>> = std::sync::Mutex::new((0..total).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..4.min(total.max(1)) {
            scope.spawn(|| loop {
                let index = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if index >= total {
                    break;
                }
                let job = &jobs[index];
                if let Some(result) = run_homun_once(
                    job.homun,
                    job.overrides,
                    job.mobs,
                    job.spregen,
                    180,
                    &job.config,
                    None,
                    None,
                    job.no_types,
                ) {
                    slots.lock().unwrap()[index] = Some(result);
                }
            });
        }
    });
    let slots = slots.into_inner().unwrap();
    if slots.iter().any(Option::is_none) {
        return None;
    }
    Some(slots.into_iter().flatten().collect())
}

struct AbRow {
    homun: &'static str,
    mobs: u32,
    spregen: u32,
    old_dps: i64,
    new_dps: i64,
    change: String,
    old_per_sp: Option<i64>,
    new_per_sp: Option<i64>,
    old_casts: String,
    new_casts: String,
}

struct OptRow {
    homun: &'static str,
    mobs: u32,
    spregen: u32,
    label: &'static str,
    overrides: &'static str,
    dps: i64,
    change: String,
    per_sp: Option<i64>,
    refused: i64,
    casts: String,
}

fn per_sp(damage: i64, sp: i64) -> Option<i64> {
    if sp != 0 {
        Some((damage as f64 / sp as f64).round() as i64)
    } else {
        None
    }
}

fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn write_csv(path: &Path, header: &[&str], rows: &[Vec<String>]) -> std::io::Result<()> {
    let mut text = String::from("\u{feff}");
    text.push_str(&header.iter().map(|cell| csv_field(cell)).collect::<Vec<_>>().join(","));
    text.push_str("\r\n");
    for row in rows {
        text.push_str(&row.iter().map(|cell| csv_field(cell)).collect::<Vec<_>>().join(","));
        text.push_str("\r\n");
    }
    fs::write(path, text)
}

fn markdown(sha: &str, ab: &[AbRow], opt: &[OptRow]) -> String {
    let mut out: Vec<String> = Vec::new();
    out.push("# Default preset tuning: comparison tables".into());
    out.push(String::new());
    out.push("[Technical reference](../TECHNICAL.md#default-preset-tuning) | spreadsheets: [preset-before-after.csv](preset-before-after.csv), [preset-options.csv](preset-options.csv)".into());
    out.push(String::new());
    out.push("Generated by `cargo run --release -p azzy-harness -- preset-report`; do not edit by hand. Every number is the expected value of an offline simulation (`tools/harness/damage.lua`): a zero-defense Neutral dummy for 180 s, the level 275 homunculi of `tools/harness/fixtures/homun_stats.lua`, rAthena data of the commit pinned in `fixtures/rathena.lua`, no SP refill. It is **not** a measurement on a server: the homunculus stats are hypothetical and the real DPS depends on the server (the RO LATAM homunculi are much stronger than rAthena's), so read the tables as a comparison between configurations, not as expected in-game DPS; only Sonic Claw and Xeno Slasher were checked against measured damage. DPS is expected damage per second; damage/SP is expected damage divided by the SP the casts spent. SP regeneration is 60 SP/s unless stated (Eleanor 22 SP/s, measured on the test server). Monsters are the same dummy, standing together in a pack.".into());
    out.push(String::new());
    out.push(format!("## 1. Previous preset (`{sha}`) against the current settings"));
    out.push(String::new());
    out.push("Previous: that commit's `H_Config.lua` alone. Current: `USER_AI/H_Config.lua` plus each homunculus' own `USER_AI/Types/<Name>.lua`. No overrides on either side.".into());
    out.push(String::new());
    for homun in HOMUNCULI {
        out.push(format!("### {} (before and after)", title(homun)));
        out.push(String::new());
        let rows: Vec<Vec<String>> = ab
            .iter()
            .filter(|row| row.homun == homun)
            .map(|row| {
                vec![
                    row.mobs.to_string(),
                    n(row.old_dps),
                    n(row.new_dps),
                    format!("{}%", row.change),
                    row.old_per_sp.map(n).unwrap_or_default(),
                    row.new_per_sp.map(n).unwrap_or_default(),
                    if row.old_casts.is_empty() { "none".into() } else { row.old_casts.clone() },
                    if row.new_casts.is_empty() { "none".into() } else { row.new_casts.clone() },
                ]
            })
            .collect();
        out.push(table(
            &[
                "monsters",
                "DPS before",
                "DPS after",
                "change",
                "damage/SP before",
                "damage/SP after",
                "casts before (180 s)",
                "casts after (180 s)",
            ],
            &rows,
        ));
        out.push(String::new());
    }
    out.push("## 2. One option changed at a time, against the current preset".into());
    out.push(String::new());
    out.push("`vs preset` compares each row with the `current preset` row of the same group (same monsters and SP regeneration). The overrides are applied on top of the current preset.".into());
    out.push(String::new());
    for homun in HOMUNCULI {
        out.push(format!("### {} (one option at a time)", title(homun)));
        out.push(String::new());
        let rows: Vec<Vec<String>> = opt
            .iter()
            .filter(|row| row.homun == homun)
            .map(|row| {
                vec![
                    row.mobs.to_string(),
                    row.spregen.to_string(),
                    row.label.to_string(),
                    format!("`{}`", if row.overrides.is_empty() { "(none)" } else { row.overrides }),
                    n(row.dps),
                    format!("{}%", row.change),
                    row.per_sp.map(n).unwrap_or_default(),
                    row.refused.to_string(),
                    if row.casts.is_empty() { "none".into() } else { row.casts.clone() },
                ]
            })
            .collect();
        out.push(table(
            &[
                "monsters",
                "SP/s",
                "variant",
                "overrides",
                "DPS",
                "vs preset",
                "damage/SP",
                "refused",
                "casts (180 s)",
            ],
            &rows,
        ));
        out.push(String::new());
    }
    out.push("## Limits".into());
    out.push(String::new());
    out.push("- Validated against measured damage only for Sonic Claw (Eleanor) and Xeno Slasher (Eira). The other skills use the same formulas but were never compared with the server.".into());
    out.push("- Magma Flow (Dieter) is priced as one splash hit per cast; the in-game buff that triggers on attacks is not modelled, so its rows understate or overstate it. Ground units (Lava Slide, Blast Forge) are priced as hitting every monster in the area on every tick for the whole duration.".into());
    out.push("- Status effects (Bleeding, Stun, Tinder Breaker's DEF reduction), buffs, the owner's skills and monster attacks are not modelled. The AI follows its skill-class priority, so these tables do not show what the best skill would be, only what the AI does with each configuration.".into());
    out.push("- No official-server data: skill values differ there, and the preset keeps `ServerType = SERVER_OFFICIAL`.".into());
    out.push(String::new());
    out.join("\n")
}

fn title(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

fn n(value: i64) -> String {
    commas(value)
}

fn table(header: &[&str], rows: &[Vec<String>]) -> String {
    let mut lines = vec![
        format!("| {} |", header.join(" | ")),
        format!("|{}|", header.iter().map(|_| "---").collect::<Vec<_>>().join("|")),
    ];
    for row in rows {
        lines.push(format!("| {} |", row.join(" | ")));
    }
    lines.join("\n")
}

fn type_settings(repo: &Path, homun: &str) -> String {
    let path = repo.join("USER_AI").join("Types").join(format!("{}.lua", title(homun)));
    let Ok(text) = fs::read_to_string(path) else { return "none".into() };
    let re = Regex::new(r"^\s*(\w+)\s*=\s*(-?\d+(?:\.\d+)?)").unwrap();
    let pairs: Vec<String> = text
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|caps| format!("`{}` {}", &caps[1], &caps[2]))
        .collect();
    if pairs.is_empty() {
        "none".into()
    } else {
        pairs.join(", ")
    }
}

fn update_release_notes(repo: &Path, ab: &[AbRow]) {
    const START: &str = "<!-- preset-table:start -->";
    const END: &str = "<!-- preset-table:end -->";
    let path = repo.join("docs").join("TECHNICAL.md");
    let Ok(text) = fs::read_to_string(&path) else { return };
    let Some(start) = text.find(START) else {
        println!("release notes not updated: the preset-table markers were not found");
        return;
    };
    let Some(end) = text.find(END) else {
        println!("release notes not updated: the preset-table markers were not found");
        return;
    };
    let mut rows = Vec::new();
    for homun in HOMUNCULI {
        let mut row = vec![title(homun)];
        for pack in PACKS {
            let cell = ab
                .iter()
                .find(|row| row.homun == homun && row.mobs == pack)
                .map(|row| format!("{} ({}%)", n(row.new_dps), row.change))
                .unwrap_or_default();
            row.push(cell);
        }
        row.push(type_settings(repo, homun));
        rows.push(row);
    }
    let block = table(
        &[
            "Homunculus",
            "1 monster",
            "2",
            "3",
            "5",
            "Individual settings (`Types/<Name>.lua`)",
        ],
        &rows,
    );
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let replacement = format!("{START}{newline}{newline}{}{newline}{newline}", block.replace('\n', newline));
    let updated = format!("{}{}{}", &text[..start], replacement, &text[end..]);
    if fs::write(&path, updated).is_ok() {
        println!("updated the preset table in docs/TECHNICAL.md");
    }
}

pub fn run_preset_report(args: &[String]) -> i32 {
    let repo = match repo_root() {
        Ok(repo) => repo,
        Err(error) => {
            eprintln!("{error}");
            return 2;
        }
    };
    let old_ref = flag_value(args, "--old-ref").unwrap_or_else(|| "HEAD".into());
    let out_dir = flag_value(args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.join("docs").join("preset-tuning"));
    let _ = fs::create_dir_all(&out_dir);
    let Some(data) = git(&repo, &["show", &format!("{old_ref}:USER_AI/H_Config.lua")]) else {
        eprintln!("git show {old_ref}:USER_AI/H_Config.lua failed");
        return 1;
    };
    let Some(sha) = git(&repo, &["rev-parse", "--short", &old_ref]) else {
        eprintln!("git rev-parse --short {old_ref} failed");
        return 1;
    };
    let sha = sha.trim().to_string();
    let tmp = std::env::temp_dir().join(format!("azzyai-preset-{}", std::process::id()));
    let _ = fs::create_dir_all(&tmp);
    let old_config = tmp.join("H_Config.lua");
    if fs::write(&old_config, data).is_err() {
        eprintln!("cannot write {}", old_config.display());
        return 1;
    }
    let new_config = repo.join("USER_AI").join("H_Config.lua");

    let mut jobs: Vec<Job> = Vec::new();
    for homun in HOMUNCULI {
        for mobs in PACKS {
            let spregen = spot_spregen(homun);
            jobs.push(Job { homun, mobs, spregen, overrides: "", config: old_config.clone(), no_types: true });
            jobs.push(Job { homun, mobs, spregen, overrides: "", config: new_config.clone(), no_types: false });
        }
    }
    let ab_count = jobs.len();
    for (homun, mobs, spregen, _label, overrides) in OPTIONS {
        jobs.push(Job { homun, mobs: *mobs, spregen: *spregen, overrides, config: new_config.clone(), no_types: false });
    }
    let Some(results) = parallel_run(&jobs) else {
        eprintln!("a benchmark run failed");
        let _ = fs::remove_dir_all(&tmp);
        return 1;
    };
    let mut ab = Vec::new();
    let mut index = 0;
    for homun in HOMUNCULI {
        for mobs in PACKS {
            let old = &results[index];
            let new = &results[index + 1];
            index += 2;
            ab.push(AbRow {
                homun,
                mobs,
                spregen: spot_spregen(homun),
                old_dps: old.dps,
                new_dps: new.dps,
                change: format!("{:+.0}", 100.0 * (new.dps as f64 / old.dps.max(1) as f64 - 1.0)),
                old_per_sp: per_sp(old.damage, old.sp),
                new_per_sp: per_sp(new.damage, new.sp),
                old_casts: casts_text(&old.skills),
                new_casts: casts_text(&new.skills),
            });
        }
    }
    let mut reference: Vec<((&str, u32, u32), i64)> = Vec::new();
    let mut opt = Vec::new();
    for (slot, (homun, mobs, spregen, label, overrides)) in OPTIONS.iter().enumerate() {
        let result = &results[ab_count + slot];
        let key = (*homun, *mobs, *spregen);
        if *label == "current preset" {
            reference.push((key, result.dps));
        }
        let reference_dps = reference.iter().find(|(existing, _)| *existing == key).map(|(_, dps)| *dps).unwrap_or(result.dps);
        let change = if *label == "current preset" {
            "0".to_string()
        } else {
            format!("{:+.0}", 100.0 * (result.dps as f64 / reference_dps.max(1) as f64 - 1.0))
        };
        opt.push(OptRow {
            homun,
            mobs: *mobs,
            spregen: if *spregen == 0 { spot_spregen(homun) } else { *spregen },
            label,
            overrides,
            dps: result.dps,
            change,
            per_sp: per_sp(result.damage, result.sp),
            refused: result.refused,
            casts: casts_text(&result.skills),
        });
    }

    let ab_header = [
        "homunculus",
        "monsters",
        "sp_regen_per_s",
        &format!("dps_before ({sha})"),
        "dps_after",
        "dps_change_pct",
        "damage_per_sp_before",
        "damage_per_sp_after",
        "casts_before",
        "casts_after",
    ];
    let ab_rows: Vec<Vec<String>> = ab
        .iter()
        .map(|row| {
            vec![
                row.homun.to_string(),
                row.mobs.to_string(),
                row.spregen.to_string(),
                row.old_dps.to_string(),
                row.new_dps.to_string(),
                row.change.clone(),
                row.old_per_sp.map(|v| v.to_string()).unwrap_or_default(),
                row.new_per_sp.map(|v| v.to_string()).unwrap_or_default(),
                row.old_casts.clone(),
                row.new_casts.clone(),
            ]
        })
        .collect();
    let opt_header = [
        "homunculus",
        "monsters",
        "sp_regen_per_s",
        "variant",
        "overrides",
        "dps",
        "vs_current_preset_pct",
        "damage_per_sp",
        "refused_casts",
        "casts",
    ];
    let opt_rows: Vec<Vec<String>> = opt
        .iter()
        .map(|row| {
            vec![
                row.homun.to_string(),
                row.mobs.to_string(),
                row.spregen.to_string(),
                row.label.to_string(),
                if row.overrides.is_empty() { "(none)".into() } else { row.overrides.to_string() },
                row.dps.to_string(),
                row.change.clone(),
                row.per_sp.map(|v| v.to_string()).unwrap_or_default(),
                row.refused.to_string(),
                row.casts.clone(),
            ]
        })
        .collect();

    if write_csv(&out_dir.join("preset-before-after.csv"), &ab_header, &ab_rows).is_err()
        || write_csv(&out_dir.join("preset-options.csv"), &opt_header, &opt_rows).is_err()
        || fs::write(out_dir.join("README.md"), markdown(&sha, &ab, &opt)).is_err()
    {
        eprintln!("cannot write the files in {}", out_dir.display());
        let _ = fs::remove_dir_all(&tmp);
        return 1;
    }
    println!(
        "wrote {} before/after rows and {} option rows to {} (previous preset {sha})",
        ab_rows.len(),
        opt_rows.len(),
        out_dir.display()
    );
    update_release_notes(&repo, &ab);
    let _ = fs::remove_dir_all(&tmp);
    0
}
