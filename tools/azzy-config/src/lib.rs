//! Format-preserving model of the AzzyAI configuration files.
//!
//! H_Config.lua, M_Config.lua and Types/<Name>.lua are plain `Name = value` lines with comments;
//! players edit them by hand and the two GUIs rewrite them in place. This module is the Rust
//! equivalent of webgui/luaConfig.js: it keeps every byte it does not change (spacing, comments,
//! order, line endings), so a save cannot reformat or reorder a player's file.
//!
//! This is the model a native GUI would sit on; it has no UI dependency.

pub mod tactics;

use std::path::Path;

#[derive(Debug, Clone)]
pub enum Line {
    Blank,
    Comment(String),
    Assignment(Assignment),
    /// Anything the format does not cover (a table, a call): kept verbatim.
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    /// Everything up to and including the `=` and the spaces after it, exactly as written.
    pub prefix: String,
    /// The value token, trimmed.
    pub value: String,
    /// Trailing comment with its leading spaces, e.g. ` -- SERVER_PRIVATE ...`.
    pub comment: String,
    /// The complete original line (without the line ending), for the untouched case.
    pub raw: String,
    /// The original value, to know whether this line was edited.
    pub original: String,
}

impl Assignment {
    pub fn changed(&self) -> bool {
        self.value != self.original
    }

    pub fn render(&self) -> String {
        if !self.changed() {
            return self.raw.clone();
        }
        format!("{}{}{}", self.prefix, self.value, self.comment)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    // line content and the line ending it had ("" for a last line without one)
    lines: Vec<(Line, String)>,
}

impl Config {
    pub fn parse(text: &str) -> Config {
        let mut lines = Vec::new();
        for chunk in text.split_inclusive('\n') {
            let (body, ending) = match chunk.strip_suffix("\r\n") {
                Some(body) => (body, "\r\n"),
                None => match chunk.strip_suffix('\n') {
                    Some(body) => (body, "\n"),
                    None => (chunk, ""),
                },
            };
            let (line, _) = parse_line(body);
            lines.push((line, ending.to_string()));
        }
        Config { lines }
    }

    pub fn load(path: &Path) -> std::io::Result<Config> {
        Ok(Config::parse(&std::fs::read_to_string(path)?))
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.to_text())
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for (line, ending) in &self.lines {
            out.push_str(&match line {
                Line::Blank => String::new(),
                Line::Comment(text) => text.clone(),
                Line::Assignment(a) => a.render(),
                Line::Other(text) => text.clone(),
            });
            out.push_str(ending);
        }
        out
    }

    /// The trailing comment of `name`, as written in the file (" -- ..."), if any.
    pub fn comment_of(&self, name: &str) -> Option<&str> {
        self.assignments().find(|a| a.name == name).map(|a| a.comment.as_str())
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.assignments().find(|a| a.name == name).map(|a| a.value.as_str())
    }

    /// Whether `name` was edited in memory since the file was read (the amber "not applied" state).
    pub fn changed_of(&self, name: &str) -> bool {
        self.assignments().find(|a| a.name == name).map(|a| a.changed()).unwrap_or(false)
    }

    /// Adds `Name = value` at the end of the file, using the file's line ending and the same
    /// alignment as the web GUI (name padded to 25 columns). Used for options the shipped file
    /// does not have yet, but which the schema does.
    pub fn set_or_append(&mut self, name: &str, value: &str) -> bool {
        if self.set(name, value) {
            return true;
        }
        let eol = self
            .lines
            .iter()
            .map(|(_, ending)| ending.as_str())
            .find(|ending| !ending.is_empty())
            .unwrap_or("\n")
            .to_string();
        if let Some((_, ending)) = self.lines.last_mut() {
            if ending.is_empty() {
                *ending = eol.clone();
            }
        }
        let mut padded = name.to_string();
        while padded.len() < 25 {
            padded.push(' ');
        }
        let prefix = format!("{padded}= ");
        self.lines.push((
            Line::Assignment(Assignment {
                name: name.to_string(),
                prefix,
                value: value.to_string(),
                comment: String::new(),
                raw: String::new(),
                // differs from the value on disk (there was no line), so changed() and
                // changed_of() count the appended option as pending
                original: String::new(),
            }),
            eol,
        ));
        true
    }

    /// Removes every assignment of `name` (used to undo an append that went back to its default).
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.lines.len();
        self.lines.retain(|(line, _)| !matches!(line, Line::Assignment(a) if a.name == name));
        self.lines.len() != before
    }

    /// Replaces the value token of `name`; the rest of the line stays byte for byte.
    pub fn set(&mut self, name: &str, value: &str) -> bool {
        for (line, _) in &mut self.lines {
            if let Line::Assignment(a) = line {
                if a.name == name {
                    a.value = value.to_string();
                    return true;
                }
            }
        }
        false
    }

    pub fn assignments(&self) -> impl Iterator<Item = &Assignment> {
        self.lines.iter().filter_map(|(line, _)| match line {
            Line::Assignment(a) => Some(a),
            _ => None,
        })
    }

    pub fn changed(&self) -> usize {
        self.assignments().filter(|a| a.changed()).count()
    }

    /// Option names grouped under the `-- comment` that precedes them, in file order. A block of
    /// consecutive comments names the section by its FIRST line: the shipped H_Config.lua starts
    /// its Homunculus S block with "-- New Homunculus S Skills" and then explains the choice, and
    /// the header is what a reader recognises, not the last line of the explanation.
    pub fn sections(&self) -> Vec<(String, Vec<String>)> {
        let mut out: Vec<(String, Vec<String>)> = Vec::new();
        let mut current = String::new();
        let mut in_comment_block = false;
        for (line, _) in &self.lines {
            match line {
                Line::Comment(text) => {
                    let t = text.trim_start_matches('-').trim();
                    if !t.is_empty() && !in_comment_block {
                        current = t.to_string();
                    }
                    in_comment_block = true;
                }
                Line::Assignment(a) => {
                    in_comment_block = false;
                    if out.last().map(|(s, _)| s != &current).unwrap_or(true) {
                        out.push((current.clone(), Vec::new()));
                    }
                    if let Some((_, names)) = out.last_mut() {
                        names.push(a.name.clone());
                    }
                }
                _ => in_comment_block = false,
            }
        }
        out
    }

    pub fn option_count(&self) -> usize {
        self.assignments().count()
    }

    /// A unified-style diff against another config, for the "what did I change" view.
    pub fn diff(&self, other: &Config) -> Vec<String> {
        let mut out = Vec::new();
        for a in self.assignments() {
            match other.get(&a.name) {
                Some(v) if v != a.value => out.push(format!("{}: {} -> {}", a.name, a.value, v)),
                None => out.push(format!("{}: only in this file ({})", a.name, a.value)),
                _ => {}
            }
        }
        for a in other.assignments() {
            if self.get(&a.name).is_none() {
                out.push(format!("{}: only in the other file ({})", a.name, a.value));
            }
        }
        out
    }
}

fn parse_line(body: &str) -> (Line, ()) {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return (Line::Blank, ());
    }
    if trimmed.starts_with("--") {
        return (Line::Comment(body.to_string()), ());
    }
    // Name = value [-- comment]; the name is the first identifier, the separator the first '='
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    let name_start = i;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }
    let name = &body[name_start..i];
    if name.is_empty() || !name.as_bytes()[0].is_ascii_alphabetic() {
        return (Line::Other(body.to_string()), ());
    }
    let after_name = i;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'=' {
        return (Line::Other(body.to_string()), ());
    }
    i += 1; // '='
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    let value_start = i;
    // a value ends at a comment (" --" or a tab before "--") or at the end of the line
    let rest = &body[value_start..];
    let (value, comment) = split_value(rest);
    let prefix = body[..value_start].to_string();
    let _ = after_name;
    (
        Line::Assignment(Assignment {
            name: name.to_string(),
            prefix,
            value: value.to_string(),
            comment: comment.to_string(),
            raw: body.to_string(),
            original: value.to_string(),
        }),
        (),
    )
}

fn split_value(rest: &str) -> (&str, &str) {
    let bytes = rest.as_bytes();
    let mut i = 0;
    let mut quoted = false;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => quoted = !quoted,
            b'-' if !quoted && i + 1 < bytes.len() && bytes[i + 1] == b'-' && i > 0 && (bytes[i - 1] == b' ' || bytes[i - 1] == b'\t') => {
                let value = rest[..i].trim_end();
                return (value, &rest[value.len()..]);
            }
            _ => {}
        }
        i += 1;
    }
    (rest.trim_end(), "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn round_trip_is_byte_identical() {
        for name in ["H_Config.lua", "M_Config.lua", "Types/Dieter.lua"] {
            let path = repo().join("USER_AI").join(name);
            let text = std::fs::read_to_string(&path).unwrap();
            let config = Config::parse(&text);
            assert_eq!(config.to_text(), text, "{name} did not round-trip");
            assert!(config.option_count() > 0, "{name} has no options");
        }
    }

    #[test]
    fn set_changes_only_the_value() {
        let text = "ServerType = SERVER_OFFICIAL -- private?\nLavaSlideMode = 0\n";
        let mut config = Config::parse(text);
        assert_eq!(config.get("LavaSlideMode"), Some("0"));
        assert!(config.set("LavaSlideMode", "1"));
        assert_eq!(config.changed(), 1);
        assert_eq!(
            config.to_text(),
            "ServerType = SERVER_OFFICIAL -- private?\nLavaSlideMode = 1\n"
        );
    }

    #[test]
    fn unknown_option_is_not_added() {
        let mut config = Config::parse("A = 1\n");
        assert!(!config.set("B", "2"));
    }

    #[test]
    fn comments_and_sections_are_kept() {
        let text = "-- Attack skill options\nA = 1\nB = 2\n\n-- Buffs\nC = 3\n";
        let config = Config::parse(text);
        let sections = config.sections();
        assert_eq!(sections, vec![
            ("Attack skill options".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("Buffs".to_string(), vec!["C".to_string()]),
        ]);
        assert_eq!(config.to_text(), text);
    }

    #[test]
    fn sections_of_the_shipped_file_use_the_header_line() {
        let path = repo().join("USER_AI/H_Config.lua");
        let config = Config::load(&path).unwrap();
        let sections = config.sections();
        let homun = sections.iter().find(|(name, _)| name == "New Homunculus S Skills")
            .unwrap_or_else(|| panic!("no New Homunculus S Skills section: {:?}",
                sections.iter().map(|(n, _)| n).collect::<Vec<_>>()));
        assert_eq!(homun.1.len(), 20, "{:?}", homun.1);
        assert!(sections.iter().all(|(name, _)| !name.starts_with("Blazing And Furious (Eleanor)")));
    }

    #[test]
    fn trailing_comments_are_readable() {
        let config = Config::parse("ServerType = SERVER_OFFICIAL -- private servers\nAggroHP = 0\n");
        assert_eq!(config.comment_of("ServerType"), Some(" -- private servers"));
        assert_eq!(config.comment_of("AggroHP"), Some(""));
        assert_eq!(config.comment_of("Nope"), None);
    }

    #[test]
    fn missing_options_are_appended_like_the_web_gui() {
        let mut config = Config::parse("AggroHP = 60\n");
        assert!(config.set_or_append("BossAggro", "2"));
        assert_eq!(config.get("BossAggro"), Some("2"));
        assert!(config.changed_of("BossAggro"));
        assert_eq!(config.changed(), 1);
        let text = config.to_text();
        assert!(text.contains("BossAggro                = 2\n"), "{text:?}");
        // the next edit replaces the line instead of appending a second one
        assert!(config.set_or_append("BossAggro", "3"));
        assert_eq!(config.to_text().matches("BossAggro").count(), 1);
        assert!(config.remove("BossAggro"));
        assert_eq!(config.get("BossAggro"), None);
    }

    #[test]
    fn changed_of_tracks_edits_per_option() {
        let mut config = Config::parse("AggroHP = 60\nUseAutoHeal = 1\n");
        assert!(!config.changed_of("AggroHP"));
        config.set("AggroHP", "80");
        assert!(config.changed_of("AggroHP"));
        assert!(!config.changed_of("UseAutoHeal"));
        // setting it back to the value on disk clears the flag
        config.set("AggroHP", "60");
        assert!(!config.changed_of("AggroHP"));
        assert!(!config.changed_of("Unknown"));
    }

    #[test]
    fn quoted_values_with_dashes_are_not_comments() {
        let config = Config::parse("LastSavedDate = \"Default Configuration LATAM\"\n");
        assert_eq!(config.get("LastSavedDate"), Some("\"Default Configuration LATAM\""));
    }
}
