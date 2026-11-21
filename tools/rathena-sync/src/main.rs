// rathena-sync: regenerates the rAthena-derived data files of AzzyAI.
//
// Same flags and outputs as the tool it replaced; the generation date is the only difference.
// The YAML parsing of mob_db.yml (some 40k monsters) and the walk over the spawn scripts are pure
// CPU work, which is why this runs in Rust.
//
// Outputs:
//   tools/harness/fixtures/rathena.lua  "server truth" for the harness (SERVER_PRIVATE scenarios,
//                                       skill trees, damage model)
//   USER_AI/MobInfo.lua                 monster facts the AI itself loads
//   webgui/mobdata.js                   monsters and spawn maps for the web GUI (submodule present)
//
// Usage:
//   rathena-sync                    # latest rathena/rathena master
//   rathena-sync --ref <sha>        # a specific commit/branch/tag
//   rathena-sync --fixture-only     # only rewrite fixtures/rathena.lua

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::process::ExitCode;

mod fetch;
mod yaml;
use yaml::Value;

const REPO: &str = "rathena/rathena";
const SKILL_SOURCE_DIR: &str = "src/map/skills/homunculus";

const ELEMENTS: [&str; 10] = ["Neutral", "Water", "Earth", "Fire", "Wind", "Poison", "Holy", "Dark", "Ghost", "Undead"];
const RACES: [&str; 12] = ["Formless", "Undead", "Brute", "Plant", "Insect", "Fish", "Demon", "Demihuman", "Angel", "Dragon",
    "Player_Human", "Player_Doram"];
const SIZES: [&str; 3] = ["Small", "Medium", "Large"];
const FLAG_BOSS: i64 = 1;
const FLAG_MVP: i64 = 2;
const FLAG_PLANT: i64 = 4;
const FLAG_IMMOBILE: i64 = 8;
const PLANT_MODES: [&str; 4] = ["IgnoreMelee", "IgnoreMagic", "IgnoreRanged", "IgnoreMisc"];
const WG_BOSS: i64 = 1;
const WG_MVP: i64 = 2;
const WG_PLANT: i64 = 4;
const WG_AGGRESSIVE: i64 = 8;
const WG_IMMOBILE: i64 = 16;
const AGGRESSIVE_AI: [&str; 13] = ["04", "05", "08", "09", "10", "11", "12", "13", "19", "20", "21", "26", "27"];
const IMMOBILE_AI: [&str; 3] = ["06", "10", "11"];

const HOMUN_CLASSES: [(&str, &str); 9] = [
    ("LIF", "Lif"), ("AMISTR", "Amistr"), ("FILIR", "Filir"), ("VANILMIRTH", "Vanilmirth"),
    ("EIRA", "Eira"), ("BAYERI", "Bayeri"), ("SERA", "Sera"), ("DIETER", "Dieter"), ("ELEANOR", "Eleanor"),
];

const MOB_IDS: [i64; 21] = [
    1002, 1007, 1015, 1031, 1039, 1068, 1076, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085,
    1101, 1147, 1368, 1672, 1673, 1678,
];

struct Args {
    reference: String,
    fixture_only: bool,
    /// "re" (Renewal) or "pre" (Pre-Renewal): picks db/re or db/pre-re and npc/re or npc/pre-re.
    era: String,
}

fn main() -> ExitCode {
    let mut args = Args { reference: "master".into(), fixture_only: false, era: "re".into() };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--ref" => match it.next() {
                Some(v) => args.reference = v,
                None => return usage("--ref needs a value"),
            },
            "--fixture-only" => args.fixture_only = true,
            "--era" => match it.next() {
                Some(v) if v == "re" || v == "pre" => args.era = v,
                Some(v) => return usage(&format!("--era takes re or pre, not {v:?}")),
                None => return usage("--era needs a value"),
            },
            "-h" | "--help" => {
                print!("{}", include_str!("help.txt"));
                return ExitCode::SUCCESS;
            }
            other => return usage(&format!("unknown argument {other:?}")),
        }
    }
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("rathena-sync: {e}");
            ExitCode::FAILURE
        }
    }
}

fn usage(msg: &str) -> ExitCode {
    eprintln!("rathena-sync: {msg}\nusage: rathena-sync [--ref <sha>] [--era re|pre] [--fixture-only]");
    ExitCode::FAILURE
}

fn run(args: &Args) -> Result<(), String> {
    let client = fetch::Client::new()?;
    let sha = client.resolve_ref(&args.reference)?;
    let pre = args.era == "pre";
    let db = if pre { "pre-re" } else { "re" };
    println!("rAthena {sha} ({db})");

    let skill_body = load_db(&client, &sha, &format!("db/{db}/skill_db.yml"))?;
    let name_to_id: BTreeMap<String, i64> = skill_body
        .iter()
        .filter_map(|s| Some((s.get_str("Name")?.to_string(), s.get_int("Id")?)))
        .collect();
    let skills = build_skills(&skill_body);

    let mut mobs = load_db(&client, &sha, &format!("db/{db}/mob_db.yml"))?;
    if !pre {
        // db/mob_db2.yml holds extra Renewal monsters; pre-re has a self-contained mob_db.
        if let Ok(extra) = load_db(&client, &sha, "db/mob_db2.yml") {
            mobs.extend(extra);
        }
    }
    if !args.fixture_only {
        let attr = load_db(&client, &sha, &format!("db/{db}/attr_fix.yml"))?;
        write_mobinfo(&sha, db, &mobs, &skill_body, &attr)?;
        write_webgui_mobdata(&client, &sha, db, &mobs)?;
        if pre {
            // The harness fixture still describes Renewal (SkillImpl ratios, Homunculus S trees):
            // pre-re scenarios use the AI's own data until a pre-re damage model exists.
            println!("  pre-re: harness fixture not written (Renewal SkillImpl ratios and S trees)");
        }
    }
    if pre {
        return Ok(());
    }
    let ratios = build_ratios(&client, &sha, &name_to_id)?;
    let homun = build_homun(&load_db(&client, &sha, "db/re/homunculus_db.yml")?, &name_to_id)?;
    let mobs = build_mobs(&mobs)?;

    let today = fetch::today();
    let mut out = String::new();
    let _ = writeln!(out, "-- Generated by tools/rathena-sync; do not edit by hand.");
    let _ = writeln!(out, "-- Source: https://github.com/{REPO}/tree/{sha} (db/re), generated {today}.");
    let _ = writeln!(out, "-- Per-level lists are indexed by skill level; times in ms.");
    let _ = writeln!(out, "RATHENA_COMMIT = \"{sha}\"");
    let _ = writeln!(out);
    let _ = writeln!(out, "-- RA_SKILL[id]: homunculus skills from db/re/skill_db.yml");
    let _ = writeln!(out, "RA_SKILL = {{");
    for sid in skills.keys() {
        let _ = writeln!(out, "\t[{sid}] = {},", lua_map(&skills[sid]));
    }
    let _ = writeln!(out, "}}");
    let _ = writeln!(out);
    let _ = writeln!(out, "-- RA_HOMUN[homuntype constant][skill id]: skill tree from db/re/homunculus_db.yml");
    let _ = writeln!(out, "RA_HOMUN = {{");
    for (const_name, _) in HOMUN_CLASSES {
        let _ = writeln!(out, "\t{const_name} = {{");
        if let Some(tree) = homun.get(const_name) {
            for sid in tree.keys() {
                let _ = writeln!(out, "\t\t[{sid}] = {},", lua_map(&tree[sid]));
            }
        }
        let _ = writeln!(out, "\t}},");
    }
    let _ = writeln!(out, "}}");
    let _ = writeln!(out);
    let _ = writeln!(out, "-- RA_MOB[id]: monsters from db/re/mob_db.yml");
    let _ = writeln!(out, "RA_MOB = {{");
    for mid in mobs.keys() {
        let _ = writeln!(out, "\t[{mid}] = {},", lua_map(&mobs[mid]));
    }
    let _ = writeln!(out, "}}");
    let _ = writeln!(out);
    let _ = writeln!(out, "-- RA_RATIO[id]: percentage added to a skill's base 100% ratio, from SkillImpl::calculateSkillRatio");
    let _ = writeln!(out, "-- in {SKILL_SOURCE_DIR}/*.cpp. lv = skill level, blv = the caster's base level, st = its");
    let _ = writeln!(out, "-- status (str, agi, vit, int, dex, luk). idiv truncates like C integer division.");
    let _ = writeln!(out, "local function idiv(a, b)");
    let _ = writeln!(out, "\tlocal q = a / b");
    let _ = writeln!(out, "\tif q >= 0 then return math.floor(q) end");
    let _ = writeln!(out, "\treturn -math.floor(-q)");
    let _ = writeln!(out, "end");
    let _ = writeln!(out, "RA_RATIO = {{");
    for sid in ratios.keys() {
        let (c, lua) = &ratios[sid];
        let _ = writeln!(out, "\t[{sid}] = function(lv, blv, st) return {lua} end, -- {c}");
    }
    let _ = writeln!(out, "}}");

    std::fs::write(fetch::fixture_path(), out)
        .map_err(|e| format!("write {}: {e}", fetch::fixture_path().display()))?;
    println!("wrote {}: {} skills, {} homunculi, {} monsters",
        fetch::fixture_path().display(), skills.len(), homun.len(), mobs.len());
    Ok(())
}

fn load_db(client: &fetch::Client, sha: &str, path: &str) -> Result<Vec<Value>, String> {
    let text = client.raw(sha, path)?;
    let doc = yaml::parse(&text)?;
    doc.get("Body")
        .and_then(|b| b.as_seq().cloned())
        .ok_or_else(|| format!("{path}: no Body list"))
}

// ---------- skill database ----------

fn per_level(value: Option<&Value>, maxlv: usize, key: &str) -> Vec<i64> {
    let fill = |v: i64| vec![v; maxlv];
    let Some(value) = value else { return fill(0) };
    match value {
        Value::Int(v) => fill(*v),
        Value::Seq(entries) => {
            let mut by_level: Vec<Option<i64>> = vec![None; maxlv];
            for e in entries {
                let (Some(lv), Some(v)) = (e.get_int("Level"), e.get_int(key)) else { continue };
                if (1..=maxlv as i64).contains(&lv) {
                    by_level[(lv - 1) as usize] = Some(v);
                }
            }
            let mut out = Vec::with_capacity(maxlv);
            let mut last = 0;
            for lv in by_level {
                last = lv.unwrap_or(last);
                out.push(last);
            }
            out
        }
        _ => fill(0),
    }
}

fn per_level_str(value: Option<&Value>, maxlv: usize, key: &str) -> Option<Vec<String>> {
    let value = value?;
    match value {
        Value::Str(s) => Some(vec![s.clone(); maxlv]),
        Value::Seq(entries) => {
            let mut by_level: Vec<Option<String>> = vec![None; maxlv];
            for e in entries {
                let (Some(lv), Some(v)) = (e.get_int("Level"), e.get_str(key)) else { continue };
                if (1..=maxlv as i64).contains(&lv) {
                    by_level[(lv - 1) as usize] = Some(v.to_string());
                }
            }
            let mut out = Vec::with_capacity(maxlv);
            let mut last = "Neutral".to_string();
            for lv in by_level {
                if let Some(v) = lv { last = v; }
                out.push(last.clone());
            }
            Some(out)
        }
        _ => None,
    }
}

// Insertion order matters: the generated Lua must stay byte-identical to the Python output,
// which writes dict keys in the order they were added.
type Table = Vec<(String, Value)>;

fn t_insert(t: &mut Table, key: &str, value: Value) {
    if let Some(slot) = t.iter_mut().find(|(k, _)| k == key) {
        slot.1 = value;
    } else {
        t.push((key.to_string(), value));
    }
}

fn build_skills(body: &[Value]) -> BTreeMap<i64, Table> {
    let mut skills: BTreeMap<i64, Table> = BTreeMap::new();
    for s in body {
        let Some(sid) = s.get_int("Id") else { continue };
        if !(8001..8100).contains(&sid) { continue; }
        let maxlv = s.get_int("MaxLevel").unwrap_or(1).max(1) as usize;
        let req = s.get("Requires");
        let unit = s.get("Unit");
        let flags = s.get("DamageFlags");
        let mut t = Table::new();
        t_insert(&mut t, "name", Value::Str(s.get_str("Name").unwrap_or_default().to_string()));
        t_insert(&mut t, "maxlv", Value::Int(maxlv as i64));
        t_insert(&mut t, "target", Value::Str(s.get_str("TargetType").unwrap_or("Passive").to_string()));
        t_insert(&mut t, "range", ints(per_level(s.get("Range"), maxlv, "Size")));
        t_insert(&mut t, "splash", ints(per_level(s.get("SplashArea"), maxlv, "Area")));
        t_insert(&mut t, "layout", ints(per_level(unit.and_then(|u| u.get("Layout")), maxlv, "Size")));
        t_insert(&mut t, "sp", ints(per_level(req.and_then(|r| r.get("SpCost")), maxlv, "Amount")));
        t_insert(&mut t, "hp", ints(per_level(req.and_then(|r| r.get("HpCost")), maxlv, "Amount")));
        t_insert(&mut t, "spheres", ints(per_level(req.and_then(|r| r.get("SpiritSphereCost")), maxlv, "Amount")));
        t_insert(&mut t, "cast", ints(per_level(s.get("CastTime"), maxlv, "Time")));
        t_insert(&mut t, "fixed", ints(per_level(s.get("FixedCastTime"), maxlv, "Time")));
        t_insert(&mut t, "delay", ints(per_level(s.get("AfterCastActDelay"), maxlv, "Time")));
        t_insert(&mut t, "cooldown", ints(per_level(s.get("Cooldown"), maxlv, "Time")));
        t_insert(&mut t, "duration", ints(per_level(s.get("Duration1"), maxlv, "Time")));
        t_insert(&mut t, "type", Value::Str(s.get_str("Type").unwrap_or("None").to_string()));
        if let Some(e) = per_level_str(s.get("Element"), maxlv, "Element") {
            t_insert(&mut t, "element", Value::Seq(e.into_iter().map(Value::Str).collect()));
        }
        t_insert(&mut t, "hits", ints(per_level(s.get("HitCount").or(Some(&Value::Int(1))), maxlv, "Count")));
        t_insert(&mut t, "nodamage", Value::Bool(flags.and_then(|f| f.get_bool("NoDamage")).unwrap_or(false)));
        t_insert(&mut t, "ignoredef", Value::Bool(flags.and_then(|f| f.get_bool("IgnoreDefense")).unwrap_or(false)));
        t_insert(&mut t, "splitsplash", Value::Bool(flags.and_then(|f| f.get_bool("SplashSplit")).unwrap_or(false)));
        t_insert(&mut t, "unit", Value::Bool(unit.is_some()));
        if let Some(u) = unit.and_then(|u| u.get_str("Id")) {
            t_insert(&mut t, "unitid", Value::Str(u.to_string()));
        }
        t_insert(&mut t, "interval", Value::Int(unit.and_then(|u| u.get_int("Interval")).unwrap_or(0)));
        skills.insert(sid, t);
    }
    skills
}

fn ints(v: Vec<i64>) -> Value {
    Value::Seq(v.into_iter().map(Value::Int).collect())
}

fn build_homun(body: &[Value], name_to_id: &BTreeMap<String, i64>) -> Result<BTreeMap<String, BTreeMap<i64, Table>>, String> {
    let by_class: BTreeMap<&str, &Value> = body
        .iter()
        .filter_map(|h| Some((h.get_str("Class")?, h)))
        .collect();
    let mut trees = BTreeMap::new();
    for (const_name, class) in HOMUN_CLASSES {
        let h = by_class.get(class)
            .ok_or_else(|| format!("homunculus class {class} not found in homunculus_db.yml"))?;
        let mut tree = BTreeMap::new();
        if let Some(list) = h.get("SkillTree").and_then(|v| v.as_seq()) {
            for e in list {
                let Some(skill) = e.get_str("Skill") else { continue };
                let Some(sid) = name_to_id.get(skill) else { continue };
                let mut t = Table::new();
                t_insert(&mut t, "maxlv", Value::Int(e.get_int("MaxLevel").unwrap_or(1)));
                t_insert(&mut t, "level", Value::Int(e.get_int("RequiredLevel").unwrap_or(0)));
                t_insert(&mut t, "intimacy", Value::Int(e.get_int("RequiredIntimacy").unwrap_or(0)));
                tree.insert(*sid, t);
            }
        }
        trees.insert(const_name.to_string(), tree);
    }
    Ok(trees)
}

fn build_mobs(body: &[Value]) -> Result<BTreeMap<i64, Table>, String> {
    let mut mobs = BTreeMap::new();
    for m in body {
        let Some(mid) = m.get_int("Id") else { continue };
        if !MOB_IDS.contains(&mid) { continue; }
        let mut t = Table::new();
        t_insert(&mut t, "name", Value::Str(m.get_str("Name").or(m.get_str("AegisName")).unwrap_or_default().to_string()));
        t_insert(&mut t, "level", Value::Int(m.get_int("Level").unwrap_or(1)));
        t_insert(&mut t, "hp", Value::Int(m.get_int("Hp").unwrap_or(1)));
        t_insert(&mut t, "race", Value::Str(m.get_str("Race").unwrap_or("Formless").to_string()));
        t_insert(&mut t, "element", Value::Str(m.get_str("Element").unwrap_or("Neutral").to_string()));
        t_insert(&mut t, "elv", Value::Int(m.get_int("ElementLevel").unwrap_or(1)));
        t_insert(&mut t, "size", Value::Str(m.get_str("Size").unwrap_or("Small").to_string()));
        t_insert(&mut t, "ai", Value::Str(ai_code(m.get("Ai"))));
        t_insert(&mut t, "class", Value::Str(m.get_str("Class").unwrap_or("Normal").to_string()));
        t_insert(&mut t, "mvp", Value::Bool(m.get("MvpExp").is_some()));
        for (k, field) in [("def", "Defense"), ("mdef", "MagicDefense"), ("res", "Resistance"), ("mres", "MagicResistance")] {
            t_insert(&mut t, k, Value::Int(m.get_int(field).unwrap_or(0)));
        }
        for stat in ["str", "agi", "vit", "int", "dex", "luk"] {
            let field: String = stat[0..1].to_uppercase() + &stat[1..];
            t_insert(&mut t, stat, Value::Int(m.get_int(&field).unwrap_or(1)));
        }
        mobs.insert(mid, t);
    }
    let missing: Vec<i64> = MOB_IDS.iter().copied().filter(|id| !mobs.contains_key(id)).collect();
    if !missing.is_empty() {
        return Err(format!("monsters not found in mob_db.yml: {missing:?}"));
    }
    Ok(mobs)
}

fn ai_code(v: Option<&Value>) -> String {
    match v {
        None => "06".into(),
        Some(Value::Int(v)) => format!("{v:02}"),
        Some(Value::Str(s)) => format!("{s:0>2}"),
        _ => "06".into(),
    }
}

// ---------- calculateSkillRatio translation ----------

fn build_ratios(client: &fetch::Client, sha: &str, name_to_id: &BTreeMap<String, i64>) -> Result<BTreeMap<i64, (String, String)>, String> {
    let mut ratios = BTreeMap::new();
    let items = client.contents(sha, SKILL_SOURCE_DIR)?;
    for item in &items {
        let name = item["name"].as_str().unwrap_or_default().to_string();
        if !name.ends_with(".cpp") || name.starts_with("skill_factory") { continue; }
        let path = item["path"].as_str().unwrap_or_default().to_string();
        let text = client.raw(sha, &path)?;
        let Some((sid, c_expr)) = extract_ratio(&text, name_to_id) else { continue };
        let lua = c_to_lua(&c_expr).map_err(|e| format!("{name}: {e}"))?;
        ratios.insert(sid, (c_expr, lua));
    }
    Ok(ratios)
}

fn extract_ratio(text: &str, name_to_id: &BTreeMap<String, i64>) -> Option<(i64, String)> {
    let owner = regex::Regex::new(r"SkillImpl\w*\(\s*(MH_\w+)\s*\)").unwrap()
        .captures(text)?;
    let sid = *name_to_id.get(owner.get(1)?.as_str())?;
    let body = regex::Regex::new(r"(?s)::calculateSkillRatio\([^)]*\)\s*const\s*\{(.*?)\n\}").unwrap()
        .captures(text)?.get(1)?.as_str();
    let adds = regex::Regex::new(r"base_skillratio\s*\+=\s*([^;]+);").unwrap();
    let mut matches = adds.captures_iter(body);
    let first = matches.next()?.get(1)?.as_str().to_string();
    if matches.next().is_some() { return None; }
    // anything besides the status_data declaration and the addition means the formula is not
    // a single expression, and a wrong model is worse than no model
    for line in body.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with("//") || l.starts_with("const status_data") || l.starts_with("base_skillratio") {
            continue;
        }
        return None;
    }
    Some((sid, regex::Regex::new(r"\s+").unwrap().replace_all(&first, " ").trim().to_string()))
}

/// Translates the integer arithmetic of calculateSkillRatio (numbers, + - * / and parentheses,
/// skill_lv, status_get_lv(src) and sstatus->stat) into a Lua expression; / becomes idiv().
fn c_to_lua(expr: &str) -> Result<String, String> {
    let mut e = regex::Regex::new(r"status_get_lv\(\s*src\s*\)").unwrap()
        .replace_all(expr, "blv").to_string();
    e = regex::Regex::new(r"sstatus->(\w+)").unwrap()
        .replace_all(&e, |c: &regex::Captures| format!("st_{}", &c[1].trim_end_matches('_')))
        .to_string();
    e = regex::Regex::new(r"\bskill_lv\b").unwrap().replace_all(&e, "lv").to_string();
    e = e.replace(" ", "");
    let mut pos = 0usize;
    fn peek(e: &str, pos: usize) -> Option<char> { e[pos..].chars().next() }
    fn take(e: &str, pos: &mut usize) -> char {
        let c = e[*pos..].chars().next().unwrap();
        *pos += c.len_utf8();
        c
    }
    fn ident(e: &str, pos: &mut usize) -> String {
        let start = *pos;
        while *pos < e.len() && (e[*pos..].chars().next().unwrap().is_ascii_alphanumeric() || e.as_bytes()[*pos] == b'_') {
            *pos += 1;
        }
        e[start..*pos].to_string()
    }
    fn factor(e: &str, pos: &mut usize) -> Result<String, String> {
        let c = peek(e, *pos).ok_or("unexpected end of expression")?;
        if c.is_ascii_digit() {
            return Ok(ident(e, pos));
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let id = ident(e, pos);
            return match id.as_str() {
                "lv" | "blv" => Ok(id),
                _ if id.starts_with("st_") => Ok(format!("st.{}", &id[3..])),
                _ => Err(format!("unsupported identifier {id:?}")),
            };
        }
        if c == '(' {
            *pos += 1;
            let inner = additive(e, pos)?;
            if take(e, pos) != ')' { return Err("missing )".into()); }
            return Ok(format!("({inner})"));
        }
        if c == '-' {
            *pos += 1;
            return Ok(format!("(-{})", factor(e, pos)?));
        }
        Err(format!("unsupported character {c:?}"))
    }
    fn multiplicative(e: &str, pos: &mut usize) -> Result<String, String> {
        let mut left = factor(e, pos)?;
        while matches!(peek(e, *pos), Some('*') | Some('/')) {
            let op = take(e, pos);
            let right = factor(e, pos)?;
            left = if op == '*' { format!("({left} * {right})") } else { format!("idiv({left}, {right})") };
        }
        Ok(left)
    }
    fn additive(e: &str, pos: &mut usize) -> Result<String, String> {
        let mut left = multiplicative(e, pos)?;
        while matches!(peek(e, *pos), Some('+') | Some('-')) {
            let op = take(e, pos);
            let right = multiplicative(e, pos)?;
            left = format!("({left} {op} {right})");
        }
        Ok(left)
    }
    let out = additive(&e, &mut pos)?;
    if pos != e.len() {
        return Err(format!("trailing tokens at {} in {expr:?}", &e[pos..]));
    }
    Ok(out)
}

// ---------- output writers ----------

fn write_mobinfo(sha: &str, db: &str, mob_body: &[Value], skill_body: &[Value], attr_body: &[Value]) -> Result<(), String> {
    let today = fetch::today();
    let era = if db == "pre-re" { "Pre-Renewal" } else { "Renewal" };
    let mut lines = vec![
        format!("-- MobInfo.lua: generated by tools/rathena-sync from rAthena {era}; do not edit by hand."),
        format!("-- Source: https://github.com/{REPO}/tree/{sha} (db/{db}/mob_db.yml, attr_fix.yml, skill_db.yml),"),
        format!("-- generated {today}. Used by GetMobInfo() and friends in AzzyUtil.lua."),
        "-- MobInfo[class] = {race, element, element level, size, flags, attack range, DEF, MDEF}".to_string(),
        format!("--   element: {}", ELEMENTS.iter().enumerate().map(|(i, n)| format!("{i} {n}")).collect::<Vec<_>>().join(", ")),
        format!("--   race: {}", RACES.iter().enumerate().map(|(i, n)| format!("{i} {n}")).collect::<Vec<_>>().join(", ")),
        format!("--   size: {}", SIZES.iter().enumerate().map(|(i, n)| format!("{i} {n}")).collect::<Vec<_>>().join(", ")),
        format!("--   flags (sum): {FLAG_BOSS} boss protocol, {FLAG_MVP} MVP, {FLAG_PLANT} plant (1 damage from everything),"),
        format!("--                {FLAG_IMMOBILE} immobile (never chases: Ai 06, 10 or 11)"),
        "--   attack range: cells (mob_db AttackRange, at least 1)".to_string(),
        "--   DEF, MDEF: mob_db Defense and MagicDefense (the hard part; the soft part comes from VIT and INT)".to_string(),
        "MobInfo = {}".to_string(),
    ];
    let mut sorted: Vec<&Value> = mob_body.iter().collect();
    sorted.sort_by_key(|m| m.get_int("Id").unwrap_or(0));
    let mut count = 0;
    for m in sorted {
        let modes = m.get("Modes");
        let mut flags = 0;
        if m.get_str("Class") == Some("Boss") { flags += FLAG_BOSS; }
        if m.get("MvpExp").is_some() { flags += FLAG_MVP; }
        if PLANT_MODES.iter().all(|k| modes.and_then(|x| x.get_bool(k)).unwrap_or(false)) { flags += FLAG_PLANT; }
        if IMMOBILE_AI.contains(&ai_code(m.get("Ai")).as_str()) { flags += FLAG_IMMOBILE; }
        let race = index_of(&RACES, m.get_str("Race"), "race")?;
        let element = index_of(&ELEMENTS, m.get_str("Element"), "element")?;
        let size = index_of(&SIZES, m.get_str("Size"), "size")?;
        let info = [
            race, element, m.get_int("ElementLevel").unwrap_or(1), size, flags,
            m.get_int("AttackRange").unwrap_or(0).max(1),
            m.get_int("Defense").unwrap_or(0), m.get_int("MagicDefense").unwrap_or(0),
        ];
        lines.push(format!("MobInfo[{}]={{{}}}", m.get_int("Id").unwrap_or(0),
            info.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
        count += 1;
    }

    lines.push("".into());
    lines.push("-- ElementFix[element level][attack element][defense element] = damage %".into());
    lines.push("ElementFix = {}".into());
    for entry in attr_body {
        let lv = entry.get_int("Level").unwrap_or(0);
        lines.push(format!("ElementFix[{lv}] = {{}}"));
        for (i, atk) in ELEMENTS.iter().enumerate() {
            let row = entry.get(atk);
            let values: Vec<i64> = (0..ELEMENTS.len())
                .map(|d| row.and_then(|r| r.get_int(ELEMENTS[d])).unwrap_or(100))
                .collect();
            lines.push(format!("ElementFix[{lv}][{i}] = {{[0]={},{}}}",
                values[0], values[1..].iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")));
        }
    }

    lines.push("".into());
    lines.push("-- SkillElement[skill] = element of the homunculus skills that deal damage".into());
    lines.push("-- (weapon-element skills use the homunculus' neutral attack)".into());
    lines.push("SkillElement = {}".into());
    for s in skill_body {
        let Some(sid) = s.get_int("Id") else { continue };
        let flags = s.get("DamageFlags");
        if !(8001..8100).contains(&sid) { continue; }
        if !matches!(s.get_str("Type"), Some("Weapon") | Some("Magic") | Some("Misc")) { continue; }
        if flags.and_then(|f| f.get_bool("NoDamage")).unwrap_or(false)
            && !flags.and_then(|f| f.get_bool("Splash")).unwrap_or(false) { continue; }
        let mut element = s.get("Element");
        if let Some(Value::Seq(list)) = element {
            element = list.first();
        }
        let idx = match element.and_then(|e| e.as_str()) {
            None => 0,
            Some(name) => ELEMENTS.iter().position(|x| *x == name).unwrap_or(0),
        };
        let label = element.and_then(|e| e.as_str()).unwrap_or("Weapon");
        lines.push(format!("SkillElement[{sid}] = {idx} -- {} ({label})", s.get_str("Name").unwrap_or_default()));
    }

    let path = fetch::mobinfo_path();
    std::fs::write(&path, lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {e}", path.display()))?;
    println!("wrote {}: {} monsters", path.display(), count);
    Ok(())
}

fn write_webgui_mobdata(client: &fetch::Client, sha: &str, db: &str, mob_body: &[Value]) -> Result<(), String> {
    let dir = fetch::webgui_path();
    if !dir.is_dir() {
        println!("  {} not found, mobdata.js skipped", dir.display());
        return Ok(());
    }
    let today = fetch::today();
    let aegis: BTreeMap<String, i64> = mob_body.iter()
        .filter_map(|m| Some((m.get_str("AegisName")?.to_string(), m.get_int("Id")?)))
        .collect();
    let mut mobs = BTreeMap::new();
    for m in mob_body {
        let Some(mid) = m.get_int("Id") else { continue };
        let modes = m.get("Modes");
        let ai = ai_code(m.get("Ai"));
        let mut flags = 0;
        if m.get_str("Class") == Some("Boss") { flags |= WG_BOSS; }
        if m.get("MvpExp").is_some() { flags |= WG_MVP; }
        if PLANT_MODES.iter().all(|k| modes.and_then(|x| x.get_bool(k)).unwrap_or(false)) { flags |= WG_PLANT; }
        if AGGRESSIVE_AI.contains(&ai.as_str()) { flags |= WG_AGGRESSIVE; }
        if IMMOBILE_AI.contains(&ai.as_str()) { flags |= WG_IMMOBILE; }
        let name = m.get_str("Name").or(m.get_str("AegisName")).unwrap_or_default();
        mobs.insert(mid, json_array(vec![
            JsonPart::Str(name.to_string()),
            JsonPart::Int(m.get_int("Level").unwrap_or(1)),
            JsonPart::Int(flags),
            JsonPart::Int(index_of(&ELEMENTS, m.get_str("Element"), "element")?),
            JsonPart::Int(m.get_int("ElementLevel").unwrap_or(1)),
            JsonPart::Int(index_of(&RACES, m.get_str("Race"), "race")?),
        ]));
    }

    // npc/re/mobs/**.txt: "map,x,y[,xs,ys]<TAB>monster|boss_monster|miniboss_monster<TAB>Name<TAB>id,amount,..."
    // (?m): every spawn line is its own match, like Python's re.M
    let spawn = regex::Regex::new(
        r"(?m)^([^,\s/]+),[^\t]*\t(?:boss_|miniboss_)?monster\t[^\t]*\t([A-Za-z0-9_]+)\s*,"
    ).unwrap();
    let mut maps: BTreeMap<String, Vec<i64>> = BTreeMap::new();
    let files = list_spawn_files(client, sha, &format!("npc/{db}/mobs"))?;
    for path in &files {
        let text = client.raw(sha, path)?;
        for c in spawn.captures_iter(&text) {
            let map_name = c[1].to_string();
            let mob = c[2].to_string();
            let mid = mob.parse::<i64>().ok().or_else(|| aegis.get(&mob).copied());
            if let Some(mid) = mid.filter(|id| mobs.contains_key(id)) {
                let ids = maps.entry(map_name).or_default();
                if !ids.contains(&mid) { ids.push(mid); }
            }
        }
    }

    let mut lines = vec![
        "// mobdata.js: generated by tools/harness/rathena_sync.py (AzzyAI repository); do not edit by hand.".to_string(),
        format!("// Source: https://github.com/{REPO}/tree/{sha} (db/re/mob_db.yml, npc/re/mobs), generated {today}."),
        "// mobs[id] = [name, level, flags, element, element level, race]".to_string(),
        format!("//   flags (sum): {WG_BOSS} boss protocol, {WG_MVP} MVP, {WG_PLANT} plant (1 damage from everything),"),
        format!("//   {WG_AGGRESSIVE} aggressive, {WG_IMMOBILE} can't move. Element/race numbers as in USER_AI/MobInfo.lua."),
        "// maps[map] = monster IDs that spawn there (rAthena Renewal spawn scripts)".to_string(),
        "window.AzzyMobData = {".to_string(),
        format!("  elements: {},", json_list(&ELEMENTS)),
        format!("  races: {},", json_list(&RACES)),
        "  mobs: {".to_string(),
    ];
    for (mid, v) in &mobs {
        lines.push(format!("    {mid}: {v},"));
    }
    lines.push("  },".to_string());
    lines.push("  maps: {".to_string());
    for (name, ids) in &maps {
        let arr = json_array(ids.iter().map(|i| JsonPart::Int(*i)).collect());
        lines.push(format!("    {}: {arr},", json_str(name)));
    }
    lines.push("  }".to_string());
    lines.push("};".to_string());
    lines.push("".to_string());

    let out = dir.join("mobdata.js");
    std::fs::write(&out, lines.join("\n"))
        .map_err(|e| format!("write {}: {e}", out.display()))?;
    println!("wrote {}: {} monsters, {} maps from {} spawn files",
        out.display(), mobs.len(), maps.len(), files.len());
    Ok(())
}

fn list_spawn_files(client: &fetch::Client, sha: &str, path: &str) -> Result<Vec<String>, String> {
    let items = client.contents(sha, path)?;
    let mut files = Vec::new();
    for item in &items {
        let name = item["name"].as_str().unwrap_or_default().to_string();
        let item_path = item["path"].as_str().unwrap_or_default().to_string();
        match item["type"].as_str() {
            Some("dir") => files.extend(list_spawn_files(client, sha, &item_path)?),
            _ if name.ends_with(".txt") => files.push(item_path),
            _ => {}
        }
    }
    Ok(files)
}

// ---------- shared helpers ----------

fn index_of(table: &[&str], name: Option<&str>, what: &str) -> Result<i64, String> {
    let Some(name) = name else { return Ok(0) };
    table.iter().position(|x| *x == name).map(|i| i as i64)
        .ok_or_else(|| format!("unknown {what} {name:?}, using {}", table[0]))
}

fn lua_map(map: &Table) -> String {
    let parts: Vec<String> = map.iter()
        .filter(|(_, x)| !matches!(x, Value::Null))
        .map(|(k, x)| format!("{}={}", key_str(k), lua_value(x)))
        .collect();
    format!("{{{}}}", parts.join(", "))
}

fn lua_value(v: &Value) -> String {
    match v {
        Value::Bool(b) => if *b { "true".into() } else { "false".into() },
        Value::Int(i) => i.to_string(),
        Value::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::Seq(list) => format!("{{{}}}", list.iter().map(lua_value).collect::<Vec<_>>().join(",")),
        Value::Map(_) => unreachable!("tables are Vec pairs, not Value::Map"),
        Value::Null => "nil".into(),
    }
}

fn key_str(k: &str) -> String {
    if k.chars().next().is_some_and(|c| (c.is_ascii_alphabetic() || c == '_')
        && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')) {
        k.to_string()
    } else {
        format!("[{k}]")
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => { let _ = write!(out, "\\u{:04x}", c as u32); }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

enum JsonPart {
    Str(String),
    Int(i64),
}

// json.dumps: ", " between items, strings quoted, numbers bare
fn json_array(parts: Vec<JsonPart>) -> String {
    let rendered: Vec<String> = parts.iter().map(|p| match p {
        JsonPart::Str(s) => json_str(s),
        JsonPart::Int(i) => i.to_string(),
    }).collect();
    format!("[{}]", rendered.join(", "))
}

fn json_list(list: &[&str]) -> String {
    // json.dumps default separators: ", " between items
    let parts: Vec<String> = list.iter().map(|s| json_str(s)).collect();
    format!("[{}]", parts.join(", "))
}
