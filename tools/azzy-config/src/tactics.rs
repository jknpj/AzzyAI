//! `H_Tactics.lua`, `M_Tactics.lua`, `H_PVP_Tact.lua` and `M_PVP_Tact.lua`.
//!
//! These files are a table of entries, not `Name = value` lines:
//!
//! ```lua
//! MyTact[1005]={TACT_ATTACK_L,SKILL_NEVER,KITE_NEVER,...,CHASE_NORMAL} --Familiar
//! ```
//!
//! The web GUI regenerates the whole file on save; here every line that was not edited is kept
//! byte for byte (comments, blank lines, spacing, line endings), and only edited entries are
//! rewritten. New entries are appended. The field indexes are the ones in `webgui/tactics.js`.

use std::path::Path;

/// `F` in webgui/tactics.js.
pub const BASIC: usize = 0;
pub const SKILL: usize = 1;
pub const KITE: usize = 2;
pub const CAST: usize = 3;
pub const PUSH: usize = 4;
pub const DEBUFF: usize = 5;
pub const CLASS: usize = 6;
pub const RESCUE: usize = 7;
pub const SP: usize = 8;
pub const SNIPE: usize = 9;
pub const KS: usize = 10;
pub const WEIGHT: usize = 11;
pub const CHASE: usize = 12;

pub const TACT_DEFAULTS: [&str; 13] = [
    "TACT_IGNORE",
    "SKILL_ALWAYS",
    "KITE_NEVER",
    "CAST_REACT",
    "PUSH_NEVER",
    "DEBUFF_NEVER",
    "CLASS_BOTH",
    "RESCUE_NEVER",
    "-1",
    "SNIPE_OK",
    "KS_NEVER",
    "1",
    "CHASE_NORMAL",
];

pub const PVP_DEFAULTS: [&str; 8] = [
    "TACT_REACT_L",
    "SKILL_ALWAYS",
    "KITE_NEVER",
    "CAST_REACT",
    "PUSH_NEVER",
    "DEBUFF_NEVER",
    "CLASS_S",
    "RESCUE_NEVER",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Tactics,
    Pvp,
}

impl Kind {
    fn prefix(self) -> &'static str {
        match self {
            Kind::Tactics => "MyTact",
            Kind::Pvp => "MyPVPTact",
        }
    }

    fn defaults(self) -> &'static [&'static str] {
        match self {
            Kind::Tactics => &TACT_DEFAULTS,
            Kind::Pvp => &PVP_DEFAULTS,
        }
    }

    pub fn field_count(self) -> usize {
        self.defaults().len()
    }

    pub fn is_pvp(self) -> bool {
        self == Kind::Pvp
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Origin {
    /// The entry came from this line of the original file.
    Line(usize),
    /// The implicit `[0]` row the web GUI inserts when the file does not have one. It is only
    /// written when the user edits it.
    Synthetic,
    /// Added in the editor.
    Added,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
    origin: Origin,
    original_id: String,
    original_name: String,
    original_values: Vec<String>,
    indent: String,
    has_comment: bool,
    removed: bool,
}

impl Entry {
    /// An edited entry (or one that is new/removed) must be rewritten on save.
    pub fn changed(&self) -> bool {
        if self.removed {
            return self.origin != Origin::Added;
        }
        self.origin == Origin::Added
            || self.id != self.original_id
            || self.name != self.original_name
            || self.values != self.original_values
    }

    pub fn is_zero(&self) -> bool {
        self.id == "0"
    }

    pub fn is_removed(&self) -> bool {
        self.removed
    }
}

#[derive(Debug, Clone)]
pub struct Tactics {
    kind: Kind,
    lines: Vec<(String, String)>, // line body, line ending
    pub entries: Vec<Entry>,
}

struct ParsedLine {
    id: String,
    name: String,
    values: Vec<String>,
    indent: String,
    has_comment: bool,
}

fn split_line(chunk: &str) -> (&str, &str) {
    match chunk.strip_suffix("\r\n") {
        Some(body) => (body, "\r\n"),
        None => match chunk.strip_suffix('\n') {
            Some(body) => (body, "\n"),
            None => (chunk, ""),
        },
    }
}

fn parse_entry_line(body: &str, kind: Kind) -> Option<ParsedLine> {
    let trimmed = body.trim_start();
    if trimmed.starts_with("--") {
        return None;
    }
    let indent_len = body.len() - trimmed.len();
    let indent = body[..indent_len].to_string();
    let rest = trimmed.strip_prefix(kind.prefix())?;
    let rest = rest.trim_start().strip_prefix('[')?;
    let close = rest.find(']')?;
    let id = rest[..close].trim().to_string();
    let rest = rest[close + 1..].trim_start().strip_prefix('=')?;
    let rest = rest.trim_start().strip_prefix('{')?;
    let close = rest.find('}')?;
    let body_values = &rest[..close];
    let after = &rest[close + 1..];
    let (name, has_comment) = match after.find("--") {
        Some(index) => (after[index + 2..].trim().to_string(), true),
        None => (String::new(), false),
    };
    let defaults = kind.defaults();
    let mut values: Vec<String> = defaults.iter().map(|value| value.to_string()).collect();
    for (index, part) in body_values.split(',').take(defaults.len()).enumerate() {
        let part = part.trim();
        if !part.is_empty() {
            values[index] = part.to_string();
        }
    }
    Some(ParsedLine { id, name, values, indent, has_comment })
}

fn render_body(prefix: &str, entry: &Entry) -> String {
    let mut out = String::new();
    out.push_str(&entry.indent);
    out.push_str(prefix);
    out.push('[');
    out.push_str(&entry.id);
    out.push_str("]={");
    out.push_str(&entry.values.join(","));
    out.push('}');
    if entry.has_comment || !entry.name.is_empty() {
        out.push_str(" --");
        out.push_str(&entry.name);
    }
    out
}

impl Tactics {
    pub fn parse(text: &str, kind: Kind) -> Tactics {
        let mut lines = Vec::new();
        let mut entries = Vec::new();
        for chunk in text.split_inclusive('\n') {
            let (body, ending) = split_line(chunk);
            if let Some(parsed) = parse_entry_line(body, kind) {
                let line = lines.len();
                entries.push(Entry {
                    id: parsed.id.clone(),
                    name: parsed.name.clone(),
                    values: parsed.values.clone(),
                    origin: Origin::Line(line),
                    original_id: parsed.id,
                    original_name: parsed.name,
                    original_values: parsed.values,
                    indent: parsed.indent,
                    has_comment: parsed.has_comment,
                    removed: false,
                });
            }
            lines.push((body.to_string(), ending.to_string()));
        }
        if !entries.iter().any(|entry| entry.id == "0") {
            let defaults = kind.defaults().iter().map(|value| value.to_string()).collect::<Vec<_>>();
            // PVP files have no trailing names; only MyTact lines carry `--Monster`.
            let name = if kind.is_pvp() { String::new() } else { "Default".to_string() };
            entries.insert(
                0,
                Entry {
                    id: "0".to_string(),
                    name: name.clone(),
                    values: defaults.clone(),
                    origin: Origin::Synthetic,
                    original_id: "0".to_string(),
                    original_name: name,
                    original_values: defaults,
                    indent: String::new(),
                    has_comment: !kind.is_pvp(),
                    removed: false,
                },
            );
        }
        Tactics { kind, lines, entries }
    }

    pub fn load(path: &Path, kind: Kind) -> std::io::Result<Tactics> {
        Ok(Tactics::parse(&std::fs::read_to_string(path)?, kind))
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// The file's line ending (CRLF by default, like the files the web GUI writes).
    fn eol(&self) -> String {
        self.lines
            .iter()
            .map(|(_, ending)| ending.as_str())
            .find(|ending| !ending.is_empty())
            .unwrap_or("\r\n")
            .to_string()
    }

    pub fn to_text(&self) -> String {
        let mut by_line = std::collections::BTreeMap::new();
        for (index, entry) in self.entries.iter().enumerate() {
            if let Origin::Line(line) = entry.origin {
                by_line.insert(line, index);
            }
        }
        let mut out = String::new();
        for (line, (body, ending)) in self.lines.iter().enumerate() {
            match by_line.get(&line) {
                Some(&entry) if self.entries[entry].removed => {}
                Some(&entry) if self.entries[entry].changed() => {
                    out.push_str(&render_body(self.kind.prefix(), &self.entries[entry]));
                    out.push_str(ending);
                }
                _ => {
                    out.push_str(body);
                    out.push_str(ending);
                }
            }
        }
        let eol = self.eol();
        let header = format!("{}={{}}", self.kind.prefix());
        let has_header = self.lines.iter().any(|(body, _)| body.trim() == header);
        for entry in &self.entries {
            let append = match entry.origin {
                Origin::Line(_) => false,
                Origin::Synthetic => entry.changed(),
                Origin::Added => !entry.removed,
            };
            if !append {
                continue;
            }
            if out.is_empty() && !has_header {
                // a file created from scratch still needs the Lua table constructor
                out.push_str(&header);
                out.push_str(&eol);
            }
            if !out.is_empty() && !out.ends_with('\n') {
                out.push_str(&eol);
            }
            out.push_str(&render_body(self.kind.prefix(), entry));
            out.push_str(&eol);
        }
        out
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.to_text())
    }

    pub fn changed(&self) -> usize {
        self.entries.iter().filter(|entry| entry.changed()).count()
    }

    pub fn entry_mut(&mut self, index: usize) -> Option<&mut Entry> {
        self.entries.get_mut(index)
    }

    pub fn add(&mut self, id: impl Into<String>, name: impl Into<String>) -> usize {
        let index = self.entries.len();
        let values = self.kind.defaults().iter().map(|value| value.to_string()).collect::<Vec<_>>();
        self.entries.push(Entry {
            id: id.into(),
            name: name.into(),
            values: values.clone(),
            origin: Origin::Added,
            original_id: String::new(),
            original_name: String::new(),
            original_values: values,
            indent: String::new(),
            has_comment: false,
            removed: false,
        });
        index
    }

    /// Marks an entry for deletion. The `[0]` row is the fallback and cannot be removed,
    /// like the disabled Remove button in the web GUI.
    pub fn remove(&mut self, index: usize) -> bool {
        match self.entries.get(index) {
            Some(entry) if !entry.is_zero() => {
                if entry.origin == Origin::Added {
                    // never reached the file, so there is nothing to delete there
                    self.entries.remove(index);
                } else {
                    self.entries[index].removed = true;
                }
                true
            }
            _ => false,
        }
    }

    /// The highest numeric id + 1, for the Add button in the tactics editor.
    pub fn next_numeric_id(&self) -> String {
        let max = self
            .entries
            .iter()
            .filter_map(|entry| entry.id.trim().parse::<i64>().ok())
            .max()
            .unwrap_or(0);
        (max + 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn tactics_round_trip_is_byte_identical() {
        for (file, kind) in [
            ("H_Tactics.lua", Kind::Tactics),
            ("M_Tactics.lua", Kind::Tactics),
            ("H_PVP_Tact.lua", Kind::Pvp),
            ("M_PVP_Tact.lua", Kind::Pvp),
        ] {
            let path = repo().join("USER_AI").join(file);
            let text = std::fs::read_to_string(&path).unwrap();
            let tactics = Tactics::parse(&text, kind);
            assert_eq!(tactics.to_text(), text, "{file} did not round-trip");
            assert!(tactics.entries.iter().any(|entry| entry.id == "0"), "{file} lost the default");
        }
    }

    #[test]
    fn editing_one_entry_leaves_the_other_lines_alone() {
        let path = repo().join("USER_AI").join("H_Tactics.lua");
        let text = std::fs::read_to_string(&path).unwrap();
        let mut tactics = Tactics::parse(&text, Kind::Tactics);
        let index = tactics
            .entries
            .iter()
            .position(|entry| entry.id == "1005")
            .expect("Familiar");
        tactics.entry_mut(index).unwrap().values[BASIC] = "TACT_TANK".into();
        assert_eq!(tactics.changed(), 1);
        let edited = tactics.to_text();
        let before: Vec<&str> = text.lines().collect();
        let after: Vec<&str> = edited.lines().collect();
        assert_eq!(before.len(), after.len());
        let differing: Vec<usize> = before
            .iter()
            .zip(&after)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(index, _)| index)
            .collect();
        assert_eq!(differing.len(), 1);
        assert!(after[differing[0]].contains("MyTact[1005]={TACT_TANK,"));
        // and the edit survives a parse of the saved text
        let reparsed = Tactics::parse(&edited, Kind::Tactics);
        assert_eq!(reparsed.entries[index].values[BASIC], "TACT_TANK");
    }

    #[test]
    fn missing_default_is_virtual_until_edited() {
        let mut tactics = Tactics::parse("MyTact[1005]={TACT_IGNORE,SKILL_NEVER}\r\n", Kind::Tactics);
        assert_eq!(tactics.entries[0].id, "0");
        assert_eq!(tactics.changed(), 0);
        assert_eq!(tactics.to_text(), "MyTact[1005]={TACT_IGNORE,SKILL_NEVER}\r\n");
        tactics.entries[0].values[KITE] = "KITE_ALWAYS".into();
        let text = tactics.to_text();
        assert!(text.starts_with("MyTact[0]={TACT_IGNORE,SKILL_ALWAYS,KITE_ALWAYS,") || text.contains("MyTact[0]="));
    }

    #[test]
    fn a_new_file_gets_the_table_header() {
        let mut tactics = Tactics::parse("", Kind::Tactics);
        tactics.entries[0].values[BASIC] = "TACT_TANK".into();
        let text = tactics.to_text();
        assert!(text.starts_with("MyTact={}\r\nMyTact[0]={TACT_TANK,"), "{text:?}");

        let mut pvp = Tactics::parse("", Kind::Pvp);
        pvp.entries[0].values[BASIC] = "TACT_TANK".into();
        let text = pvp.to_text();
        assert!(text.starts_with("MyPVPTact={}\r\nMyPVPTact[0]="), "{text:?}");
        assert!(!text.contains("--Default"), "{text:?}");
    }

    #[test]
    fn add_and_remove_round_trips() {
        let original = "MyTact={}\r\nMyTact[1]={TACT_IGNORE,SKILL_NEVER} --Poring\r\n";
        let mut tactics = Tactics::parse(original, Kind::Tactics);
        let index = tactics.add("42", "New monster");
        assert!(tactics.to_text().contains("MyTact[42]={TACT_IGNORE,SKILL_ALWAYS,KITE_NEVER,"));
        assert!(tactics.remove(index));
        assert_eq!(tactics.to_text(), original);
        assert_eq!(tactics.changed(), 0);
    }

    #[test]
    fn pvp_entries_have_names_too() {
        let original = "MyPVPTact={}\r\nMyPVPTact[Friend]={TACT_REACT_L,SKILL_ALWAYS}\r\n";
        let tactics = Tactics::parse(original, Kind::Pvp);
        let entry = tactics.entries.iter().find(|entry| entry.id == "Friend").unwrap();
        assert_eq!(entry.values.len(), PVP_FIELDS);
        assert_eq!(entry.values[0], "TACT_REACT_L");
        assert_eq!(tactics.to_text(), original);
    }

    const PVP_FIELDS: usize = 8;
}
