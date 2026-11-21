#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// azzy-native: the AzzyAI configuration GUI in Rust (egui), mirroring the web GUI.
//
// Scope of this milestone (M1): both settings tabs (Homunculus and Mercenary) driven by the same
// schema the web GUI uses (tools/webgui-data.js -> data/options.json), with the option's English
// description, search, category tabs, "only changed from default" filter, typed editors (int,
// double, bool, enum, server type) and Apply/Revert through azzy-config, which preserves the file
// byte for byte. The look (navbar, nav-tabs, cards, footer, row markers) comes from theme.rs, which
// copies webgui/index.html. Tactics/PVP grids, per-type presets, Extra Options, translations and the
// "Update AI from GitHub" flow are implemented (see README).

mod data;
mod i18n;
mod mobs;
mod skills;
mod tactics_ui;
mod theme;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};

use azzy_config::tactics::Tactics;
use azzy_config::Config;
use data::{OptionDef, Schema};
use eframe::egui;
use theme::Palette;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Unit {
    Homunculus,
    Mercenary,
}

impl Unit {
    fn label(self) -> &'static str {
        match self {
            Unit::Homunculus => "Homunculus",
            Unit::Mercenary => "Mercenary",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Unit::Homunculus => "H",
            Unit::Mercenary => "M",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Settings,
    Tactics(tactics_ui::Tab),
    Extra,
}

const EXTRA_FILES: [&str; 2] = ["H_Extra.lua", "M_Extra.lua"];

/// The official source: the update always comes from the repository's `main` branch.
const UPDATE_URL: &str = "https://github.com/jknpj/AzzyAI/archive/refs/heads/main.zip";

/// What happened to the running executable itself (the archive ships it under `tools/`).
enum SelfUpdate {
    Staged(PathBuf),
    Unchanged,
    NotInArchive,
}

struct UpdateOutcome {
    report: azzy_update::Report,
    self_update: SelfUpdate,
    self_error: Option<String>,
}

enum UpdateEvent {
    Applying,
    Done(UpdateOutcome),
    Failed(String),
}

enum UpdateState {
    Idle,
    Downloading,
    Applying,
    Done(UpdateOutcome),
    Failed(String),
}

/// Homunculus S is not part of this build: only the base table below exists.
const S_TYPES: [(&str, &[&str]); 0] = [];

const BASE_TYPES: [(&str, &[&str]); 4] = [
    ("lif", &["escape"]),
    ("amistr", &["bulwark", "castle", "bloodlust", "provokeowner"]),
    ("filir", &["flit", "accel", "moon", "speed", "sbr44"]),
    ("vanilmirth", &["caprice", "chaotic", "selfdestruct"]),
];

fn classify(
    name: &str,
    table: &'static [(&'static str, &'static [&'static str])],
) -> Option<&'static str> {
    let lower = name.to_lowercase();
    for (key, needles) in table {
        if lower.contains(key) {
            return Some(key);
        }
        for needle in *needles {
            if lower.contains(needle) {
                return Some(*key);
            }
        }
    }
    None
}

/// Per-homunculus overrides in USER_AI/Types/<Name>.lua (webgui/app.js TYPE_NAMES).
const TYPE_NAMES: [&str; 4] = ["Lif", "Amistr", "Filir", "Vanilmirth"];

struct App {
    hom: Schema,
    mer: Schema,
    unit: Unit,
    configs: BTreeMap<&'static str, Config>, // "H" / "M"
    paths: BTreeMap<&'static str, PathBuf>,
    type_configs: BTreeMap<&'static str, Config>,
    type_paths: BTreeMap<&'static str, PathBuf>,
    type_selected: Option<&'static str>,
    skill_filter: Option<u32>,
    dirty_configs: BTreeSet<&'static str>,
    dirty_types: BTreeSet<&'static str>,
    dirty_tactics: BTreeSet<&'static str>,
    dirty_extra: BTreeSet<&'static str>,
    io_open: bool,
    io_export_dir: String,
    io_import_dir: String,
    io_status: String,
    help_open: bool,
    version: Option<String>,
    update_open: bool,
    update_overwrite_presets: bool,
    update_dir: String,
    update_target: PathBuf,
    update_state: UpdateState,
    update_rx: Option<std::sync::mpsc::Receiver<UpdateEvent>>,
    version_rx: Option<std::sync::mpsc::Receiver<(Option<String>, Result<Option<String>, String>)>>,
    new_version: Option<String>,
    write_in_place: bool,
    shown_path: String,
    search: String,
    category: Option<String>,
    only_changed: bool,
    selected: Option<String>,
    status: String,
    frames: u64,
    mode: theme::Mode,
    applied_mode: Option<theme::Mode>,
    focus_search: bool,
    // raw text of a numeric field being typed, so "-" or "1." is not thrown away mid-number
    drafts: BTreeMap<String, String>,
    // options appended in memory because the file did not have them (undo on reset)
    appended: std::collections::BTreeSet<String>,
    mobs: mobs::Mobs,
    skills: skills::Skills,
    skill_texture: Option<egui::TextureHandle>,
    tactics: BTreeMap<&'static str, Tactics>,
    tact_paths: BTreeMap<&'static str, PathBuf>,
    tact_selected: BTreeMap<&'static str, usize>,
    tact_filter: String,
    tact_list: BTreeMap<&'static str, tactics_ui::ListState>,
    suggest: tactics_ui::SuggestState,
    page: Page,
    extra: BTreeMap<&'static str, String>,
    extra_original: BTreeMap<&'static str, String>,
    extra_paths: BTreeMap<&'static str, PathBuf>,
    extra_selected: &'static str,
    logo: Option<egui::TextureHandle>,
    lang: i18n::Lang,
}

/// Everything a grid row needs, copied out of the schema/config so the UI can mutate them.
struct RowData {
    option: OptionDef,
    values: Vec<(String, i64)>,
    current: String,
    default_label: String,
    present: bool,
    uses_global: bool,
    icon: Option<usize>,
    skill_name: String,
    pending: bool,
    modified: bool,
    comment: String,
}

enum Row {
    Group { name: String, count: usize },
    Option(RowData),
}

/// Same value as the web GUI's sameValue(): numbers compare as numbers ("60" == "60.0", as the
/// file and the schema can format them differently), booleans as booleans, tokens as text.
fn same_value(kind: &str, a: &str, b: &str) -> bool {
    match kind {
        "bool" => {
            let on = |s: &str| {
                let s = s.trim();
                !(s.is_empty() || s == "0" || s.eq_ignore_ascii_case("false"))
            };
            on(a) == on(b)
        }
        "int" | "double" | "enum" => match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
            (Ok(x), Ok(y)) => x == y,
            _ => a.trim() == b.trim(),
        },
        _ => a.trim() == b.trim(),
    }
}

/// Milliseconds as the web GUI shows them: "{0} min" for whole minutes, otherwise "{0} s" with
/// the decimal separator of the language.
fn seconds_text(ms: i64) -> String {
    let seconds = ms as f64 / 1000.0;
    if seconds >= 60.0 && (seconds % 60.0).abs() < f64::EPSILON {
        return i18n::tr_args("{0} min", &[&format!("{}", (seconds / 60.0) as i64)]);
    }
    let mut text = format!("{}", (seconds * 10.0).round() / 10.0);
    if i18n::lang() != i18n::Lang::En {
        text = text.replace('.', ",");
    }
    i18n::tr_args("{0} s", &[&text])
}

/// "Max level 10 · SP 25–70 · Cast 2 s (1,5 s fixed) · Cooldown 3 s · Duration 15 s", same as the
/// web GUI's skill card.
fn stats_line(skill: &skills::Skill) -> String {
    let stats = skill.stats;
    let mut parts = vec![i18n::tr_args(
        "Max level {0}",
        &[&skill.maxlv.to_string()],
    )];
    if stats[0] == stats[1] {
        parts.push(format!("SP {}", stats[0]));
    } else {
        parts.push(format!("SP {}\u{2013}{}", stats[0], stats[1]));
    }
    if stats[2] != 0 || stats[3] != 0 {
        let mut cast = i18n::tr_args("Cast {0}", &[&seconds_text(stats[2] + stats[3])]);
        if stats[2] != 0 && stats[3] != 0 {
            cast.push_str(&format!(
                " ({})",
                i18n::tr_args("{0} fixed", &[&seconds_text(stats[2])])
            ));
        } else if stats[2] != 0 {
            cast.push_str(&format!(" ({})", i18n::tr("fixed")));
        }
        parts.push(cast);
    }
    if stats[4] != 0 {
        parts.push(i18n::tr_args("Cooldown {0}", &[&seconds_text(stats[4])]));
    }
    if stats[5] != 0 {
        parts.push(i18n::tr_args("Duration {0}", &[&seconds_text(stats[5])]));
    }
    parts.join(" \u{00b7} ")
}

/// Ragnarok's in-game text: ^RRGGBB switches the colour (^000000 is the normal text colour),
/// like the web GUI's roText().
fn ro_text(ui: &mut egui::Ui, text: &str, base: egui::Color32) {
    fn append(job: &mut egui::text::LayoutJob, chunk: &str, color: egui::Color32) {
        if chunk.is_empty() {
            return;
        }
        job.append(
            chunk,
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::proportional(12.0),
                color,
                ..Default::default()
            },
        );
    }
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = ui.available_width();
    let bytes = text.as_bytes();
    let mut color = base;
    let mut buffer = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'^'
            && i + 7 <= bytes.len()
            && bytes[i + 1..i + 7].iter().all(u8::is_ascii_hexdigit)
        {
            append(&mut job, &buffer, color);
            buffer.clear();
            let hex = std::str::from_utf8(&bytes[i + 1..i + 7]).unwrap_or("000000");
            let value = u32::from_str_radix(hex, 16).unwrap_or(0);
            color = if value == 0 {
                base
            } else {
                egui::Color32::from_rgb(
                    ((value >> 16) & 0xff) as u8,
                    ((value >> 8) & 0xff) as u8,
                    (value & 0xff) as u8,
                )
            };
            i += 7;
        } else {
            let len = text[i..].chars().next().map(char::len_utf8).unwrap_or(1);
            buffer.push_str(&text[i..i + len]);
            i += len;
        }
    }
    append(&mut job, &buffer, color);
    ui.label(job);
}

/// What the UI shows for an enum/serverType default ("Chasing"), not the raw token ("-1").
fn default_label(schema: &Schema, option: &OptionDef) -> String {
    match option.kind.as_str() {
        "bool" => if option.default.as_bool().unwrap_or(false) { "True" } else { "False" }.to_string(),
        "serverType" => if option.default.as_i64().unwrap_or(0) == 1 { "SERVER_PRIVATE" } else { "SERVER_OFFICIAL" }.to_string(),
        "enum" => {
            let value = option.default.as_i64().unwrap_or(0);
            schema
                .enum_values(option)
                .and_then(|values| values.iter().find(|(_, v)| *v == value).map(|(label, _)| label.clone()))
                .unwrap_or_else(|| schema.default_token(option))
        }
        _ => schema.default_token(option),
    }
}

/// The web GUI's "True"/"False" select, but it also accepts "false" written by hand.
fn bool_of(token: &str) -> bool {
    let token = token.trim();
    !(token.is_empty() || token == "0" || token.eq_ignore_ascii_case("false"))
}

impl App {
    fn new(paths: &BTreeMap<&'static str, PathBuf>, write_in_place: bool) -> Result<App, String> {
        let (hom, mer) = data::load()?;
        let mobs = mobs::load()?;
        let lang = i18n::load_pref();
        i18n::set_lang(lang);
        let skills = skills::load()?;
        let mut configs = BTreeMap::new();
        let mut shown = Vec::new();
        for (key, path) in paths {
            let config = Config::load(path).map_err(|e| format!("{}: {e}", path.display()))?;
            let canonical = std::fs::canonicalize(path).map(|p| p.display().to_string())
                .unwrap_or_else(|_| path.display().to_string());
            shown.push(canonical.strip_prefix(r"\\?\").unwrap_or(&canonical).to_string());
            configs.insert(*key, config);
        }
        // The tactics files live next to the two config files. A missing one is an empty model
        // that Apply Settings can create; it must not stop the settings tabs from opening.
        let base = paths["H"].parent().map(|dir| dir.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
        let mut tactics = BTreeMap::new();
        let mut tact_paths = BTreeMap::new();
        let mut tact_selected = BTreeMap::new();
        for tab in tactics_ui::Tab::ALL {
            let path = base.join(tab.layout().file);
            let model = if path.is_file() {
                Tactics::load(&path, tab.layout().kind)
                    .map_err(|e| format!("{}: {e}", path.display()))?
            } else {
                Tactics::parse("", tab.layout().kind)
            };
            tact_paths.insert(tab.key(), path);
            tact_selected.insert(tab.key(), 0);
            tactics.insert(tab.key(), model);
        }
        // Startup check for a newer release tag. It runs in the background and is skipped in
        // tests, so the harness never touches the network.
        let version_rx = if cfg!(test) {
            None
        } else {
            let installed = installed_version(&base);
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let latest = azzy_update::latest_release_tag("jknpj/AzzyAI");
                match (&installed, &latest) {
                    (Some(installed), Ok(latest)) => crate::log(&format!(
                        "version check: installed {installed}, latest release {latest:?}"
                    )),
                    (installed, latest) => crate::log(&format!(
                        "version check: installed {installed:?}, latest {latest:?}"
                    )),
                }
                let _ = tx.send((installed, latest));
            });
            Some(rx)
        };
        // H_Extra.lua / M_Extra.lua are free-form files: the editor shows the whole text.
        let mut extra = BTreeMap::new();
        let mut extra_original = BTreeMap::new();
        let mut extra_paths = BTreeMap::new();
        for file in EXTRA_FILES {
            let path = base.join(file);
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            extra_paths.insert(file, path);
            extra.insert(file, text.clone());
            extra_original.insert(file, text);
        }
        // Per-type overrides. Missing files are empty models; a save creates them only if the
        // player actually changes an override.
        let mut type_configs = BTreeMap::new();
        let mut type_paths = BTreeMap::new();
        for name in TYPE_NAMES {
            let path = base.join("Types").join(format!("{name}.lua"));
            let config = if path.is_file() {
                Config::load(&path).map_err(|e| format!("{}: {e}", path.display()))?
            } else {
                Config::parse("")
            };
            type_paths.insert(name, path);
            type_configs.insert(name, config);
        }
        Ok(App {
            hom,
            mer,
            unit: Unit::Homunculus,
            configs,
            paths: paths.clone(),
            type_configs,
            type_paths,
            type_selected: None,
            skill_filter: None,
            dirty_configs: BTreeSet::new(),
            dirty_types: BTreeSet::new(),
            dirty_tactics: BTreeSet::new(),
            dirty_extra: BTreeSet::new(),
            io_open: false,
            io_export_dir: String::new(),
            io_import_dir: String::new(),
            io_status: String::new(),
            help_open: false,
            version: installed_version(&base),
            update_open: false,
            update_overwrite_presets: false,
            update_dir: exe_dir()
                .filter(|dir| dir.join("H_Config.lua").is_file())
                .unwrap_or_else(|| base.clone())
                .display()
                .to_string(),
            update_target: PathBuf::new(),
            update_state: UpdateState::Idle,
            update_rx: None,
            version_rx,
            new_version: None,
            write_in_place,
            shown_path: shown.join("  +  "),
            search: String::new(),
            category: None,
            only_changed: false,
            selected: None,
            status: String::new(),
            frames: 0,
            mode: theme::Mode::Dark,
            applied_mode: None,
            focus_search: false,
            drafts: BTreeMap::new(),
            appended: std::collections::BTreeSet::new(),
            mobs,
            skills,
            skill_texture: None,
            tactics,
            tact_paths,
            tact_selected,
            tact_filter: String::new(),
            tact_list: tactics_ui::Tab::ALL
                .iter()
                .map(|tab| (tab.key(), tactics_ui::ListState::default()))
                .collect(),
            suggest: tactics_ui::SuggestState::default(),
            page: Page::Settings,
            extra,
            extra_original,
            extra_paths,
            extra_selected: "H_Extra.lua",
            logo: None,
            lang,
        })
    }

    /// Description shown for an option: the web GUI's translation when there is one.
    fn description(&self, option: &OptionDef) -> String {
        let scope = match self.unit {
            Unit::Homunculus => "hom",
            Unit::Mercenary => "mer",
        };
        i18n::desc(scope, &option.name, &option.description)
    }

    /// The skill header the web GUI shows above a setting's description: icon, official name, the
    /// numbers at max level and the in-game description (which lists the per-level values).
    fn skill_card(&self, ui: &mut egui::Ui, p: &Palette, skill: &skills::Skill) {
        egui::Frame::none()
            .fill(p.grid)
            .stroke(egui::Stroke::new(1.0_f32, p.border))
            .rounding(egui::Rounding::same(4.0))
            .inner_margin(egui::Margin::same(8.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    if let Some(texture) = self.skill_texture.as_ref() {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(26.0, 26.0), egui::Sense::hover());
                        ui.painter().image(
                            texture.id(),
                            rect,
                            self.skills.icon_uv(skill.icon),
                            egui::Color32::WHITE,
                        );
                    }
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(skill.name_for(self.lang))
                                .color(p.text)
                                .strong()
                                .size(13.5),
                        );
                        ui.label(
                            egui::RichText::new(stats_line(skill))
                                .color(p.muted)
                                .size(10.5),
                        );
                        ui.label(
                            egui::RichText::new(i18n::tr(
                                "Official LATAM values at max level.",
                            ))
                            .color(p.muted)
                            .italics()
                            .size(10.0),
                        );
                    });
                });
                let description = skill.description_for(self.lang);
                if description.is_empty() {
                    return;
                }
                ui.add_space(2.0);
                egui::CollapsingHeader::new(i18n::tr("In-game description"))
                    // One state for every skill card, like the web GUI's skillDescOpen.
                    .id_source("skill-description")
                    .default_open(false)
                    .show(ui, |ui| {
                        ro_text(ui, description, p.text);
                        ui.hyperlink_to(
                            format!(
                                "{} Divine Pride",
                                i18n::tr("Source:"),
                            ),
                            format!("https://www.divine-pride.net/database/skill/{}", skill.id),
                        );
                    });
            });
    }

    fn schema(&self) -> &Schema {
        match self.unit {
            Unit::Homunculus => &self.hom,
            Unit::Mercenary => &self.mer,
        }
    }

    fn unit_key(&self) -> &'static str {
        self.unit.key()
    }

    fn config(&self) -> Option<&Config> {
        self.configs.get(self.unit_key())
    }

    fn config_mut(&mut self) -> Option<&mut Config> {
        self.configs.get_mut(self.unit_key())
    }

    fn is_type_mode(&self) -> bool {
        self.unit == Unit::Homunculus && self.type_selected.is_some()
    }

    fn type_config(&self) -> Option<&Config> {
        if !self.is_type_mode() {
            return None;
        }
        self.type_selected.and_then(|name| self.type_configs.get(name))
    }

    /// The value shown in the grid: the type override when one exists, otherwise the global
    /// value, otherwise the schema default (handled by the caller).
    fn current_value(&self, name: &str) -> Option<&str> {
        if let Some(value) = self.type_config().and_then(|config| config.get(name)) {
            return Some(value);
        }
        self.config().and_then(|config| config.get(name))
    }

    fn comment_for(&self, name: &str) -> Option<&str> {
        if let Some(comment) = self.type_config().and_then(|config| config.comment_of(name)) {
            return Some(comment);
        }
        self.config().and_then(|config| config.comment_of(name))
    }

    /// Edits not written to disk yet (the amber marker).
    fn changed(&self) -> usize {
        let configs = self
            .configs
            .values()
            .map(|config| if config.is_modified() { config.changed().max(1) } else { 0 })
            .sum::<usize>();
        let types = self
            .type_configs
            .values()
            .map(|config| if config.is_modified() { config.changed().max(1) } else { 0 })
            .sum::<usize>();
        configs
            + types
            + self.dirty_configs.len()
            + self.dirty_types.len()
            + self.dirty_tactics.len()
            + self.dirty_extra.len()
            + self.tactics.values().map(|t| t.changed()).sum::<usize>()
            + EXTRA_FILES
                .iter()
                .filter(|file| self.extra.get(*file) != self.extra_original.get(*file))
                .count()
    }

    fn target_for(&self, source: &PathBuf) -> PathBuf {
        if self.write_in_place {
            source.clone()
        } else {
            PathBuf::from(format!("{}.new", source.display()))
        }
    }

    fn active_path(&self) -> String {
        let path = match self.page {
            Page::Settings => {
                if let Some(name) = self.type_selected.filter(|_| self.is_type_mode()) {
                    self.type_paths[name].display().to_string()
                } else {
                    return self.shown_path.clone();
                }
            }
            Page::Tactics(tab) => self.tact_paths[tab.key()].display().to_string(),
            Page::Extra => self.extra_paths[self.extra_selected].display().to_string(),
        };
        path.strip_prefix(r"\\?\").unwrap_or(&path).to_string()
    }

    fn reload(&mut self) {
        let mut errors = Vec::new();
        for (key, path) in &self.paths {
            match Config::load(path) {
                Ok(config) => {
                    self.configs.insert(*key, config);
                }
                Err(e) => errors.push(format!("{}: {e}", path.display())),
            }
        }
        for tab in tactics_ui::Tab::ALL {
            let key = tab.key();
            let Some(path) = self.tact_paths.get(key).cloned() else { continue };
            let loaded = if path.is_file() {
                Tactics::load(&path, tab.layout().kind)
                    .map_err(|e| format!("{}: {e}", path.display()))
            } else {
                Ok(Tactics::parse("", tab.layout().kind))
            };
            match loaded {
                Ok(model) => {
                    self.tactics.insert(key, model);
                    self.tact_selected.insert(key, 0);
                }
                Err(e) => errors.push(e),
            }
        }
        for file in EXTRA_FILES {
            let path = self.extra_paths[file].clone();
            match std::fs::read_to_string(&path) {
                Ok(text) => {
                    self.extra.insert(file, text.clone());
                    self.extra_original.insert(file, text);
                }
                Err(_e) if !path.exists() => {
                    self.extra.insert(file, String::new());
                    self.extra_original.insert(file, String::new());
                }
                Err(e) => errors.push(format!("{}: {e}", path.display())),
            }
        }
        for name in TYPE_NAMES {
            let path = self.type_paths[name].clone();
            let config = if path.is_file() {
                Config::load(&path).map_err(|e| format!("{}: {e}", path.display()))
            } else {
                Ok(Config::parse(""))
            };
            match config {
                Ok(config) => {
                    self.type_configs.insert(name, config);
                }
                Err(e) => errors.push(e),
            }
        }
        self.drafts.clear();
        self.appended.clear();
        self.dirty_configs.clear();
        self.dirty_types.clear();
        self.dirty_tactics.clear();
        self.dirty_extra.clear();
        self.status = if errors.is_empty() {
            i18n::tr("reloaded from disk")
        } else {
            i18n::tr_args("reload failed: {0}", &[&errors.join("; ")])
        };
    }

    fn save(&mut self) {
        let mut written = Vec::new();
        let mut errors = Vec::new();
        let mut total = 0;
        let config_keys: Vec<&'static str> = self
            .configs
            .iter()
            .filter(|(key, config)| config.is_modified() || self.dirty_configs.contains(*key))
            .map(|(key, _)| *key)
            .collect();
        for key in config_keys {
            let source = self.paths[key].clone();
            let target = self.target_for(&source);
            let changed = self.configs[key].changed().max(1);
            match self.configs[key].save(&target) {
                Ok(()) => {
                    total += changed;
                    written.push(target.display().to_string());
                    if let Ok(fresh) = Config::load(&target) {
                        self.configs.insert(key, fresh);
                    }
                }
                Err(e) => errors.push(format!("{}: {e}", target.display())),
            }
        }
        let type_names: Vec<&'static str> = TYPE_NAMES
            .iter()
            .copied()
            .filter(|name| {
                self.dirty_types.contains(*name)
                    || self.type_configs.get(name).map(|config| config.is_modified()).unwrap_or(false)
            })
            .collect();
        for name in type_names {
            let source = self.type_paths[name].clone();
            let target = self.target_for(&source);
            if let Some(parent) = target.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    errors.push(format!("{}: {e}", parent.display()));
                    continue;
                }
            }
            let changed = self.type_configs[name].changed().max(1);
            match self.type_configs[name].save(&target) {
                Ok(()) => {
                    total += changed;
                    written.push(target.display().to_string());
                    if let Ok(fresh) = Config::load(&target) {
                        self.type_configs.insert(name, fresh);
                    }
                }
                Err(e) => errors.push(format!("{}: {e}", target.display())),
            }
        }
        let tactic_tabs: Vec<tactics_ui::Tab> = tactics_ui::Tab::ALL
            .iter()
            .copied()
            .filter(|tab| {
                self.dirty_tactics.contains(tab.key())
                    || self.tactics.get(tab.key()).map(|t| t.changed()).unwrap_or(0) > 0
            })
            .collect();
        for tab in tactic_tabs {
            let key = tab.key();
            let source = self.tact_paths[key].clone();
            let target = self.target_for(&source);
            let changed = self.tactics[key].changed();
            match self.tactics[key].save(&target) {
                Ok(()) => {
                    total += changed;
                    written.push(target.display().to_string());
                    if let Ok(fresh) = Tactics::load(&target, tab.layout().kind) {
                        self.tactics.insert(key, fresh);
                    }
                }
                Err(e) => errors.push(format!("{}: {e}", target.display())),
            }
        }
        for file in EXTRA_FILES {
            let changed = self.extra.get(file) != self.extra_original.get(file);
            if !changed && !self.dirty_extra.contains(file) {
                continue;
            }
            let target = self.target_for(&self.extra_paths[file]);
            let text = self.extra.get(file).cloned().unwrap_or_default();
            match std::fs::write(&target, &text) {
                Ok(()) => {
                    total += 1;
                    written.push(target.display().to_string());
                    self.extra_original.insert(file, text);
                }
                Err(e) => errors.push(format!("{}: {e}", target.display())),
            }
        }
        self.drafts.clear();
        self.appended.clear();
        self.dirty_configs.clear();
        self.dirty_types.clear();
        self.dirty_tactics.clear();
        self.dirty_extra.clear();
        if !errors.is_empty() {
            self.status = i18n::tr_args("save failed: {0}", &[&errors.join("; ")]);
        } else if written.is_empty() {
            self.status = i18n::tr("nothing to apply");
        } else {
            let mut message = i18n::tr_args(
                "applied {0} change(s) to {1}",
                &[&total.to_string(), &written.join(", ")],
            );
            if !self.write_in_place {
                message.push(' ');
                message.push_str(&i18n::tr("(pass --write to edit the files themselves)"));
            }
            self.status = message;
        }
    }

    /// Whether the selected "Settings for" type should list this setting (webgui `homVisible`).
    fn type_visible(&self, name: &str) -> bool {
        let Some(scope) = self.type_selected else {
            return true;
        };
        let scope = scope.to_lowercase();
        let s_type = classify(name, &S_TYPES);
        let b_type = if s_type.is_some() { None } else { classify(name, &BASE_TYPES) };
        if S_TYPES.iter().any(|(key, _)| *key == scope.as_str()) {
            // an S keeps every base homunculus's settings visible
            !matches!(s_type, Some(kind) if kind != scope)
        } else {
            if s_type.is_some() {
                return false;
            }
            !matches!(b_type, Some(kind) if kind != scope)
        }
    }

    // The category is its own filter: the tab counts must ignore it, or every other category
    // reads "0" as soon as one is selected.
    fn matches(&self, option: &OptionDef, category: Option<&str>) -> bool {
        if !self.type_visible(&option.name) {
            return false;
        }
        if self.unit == Unit::Homunculus {
            if let Some(id) = self.skill_filter {
                // A skill without settings of its own leaves the grid alone.
                let names = Self::skill_settings(id);
                if !names.is_empty() {
                    if !names.contains(&option.name.as_str()) {
                        return false;
                    }
                } else if self.skill_has_settings(id)
                    && self.skills.for_setting(&option.name).map(|skill| skill.id) != Some(id)
                {
                    return false;
                }
            }
        }
        if let Some(category) = category {
            if option.category != *category {
                return false;
            }
        }
        if !self.search.trim().is_empty() {
            let needle = self.search.to_lowercase();
            let haystack = format!(
                "{} {} {}",
                option.name,
                option.description,
                self.description(option)
            )
            .to_lowercase();
            if !haystack.contains(&needle) {
                return false;
            }
        }
        if self.only_changed {
            let default = self.schema().default_token(option);
            let current = self.current_value(&option.name).unwrap_or("");
            if same_value(&option.kind, current, &default) {
                return false;
            }
        }
        true
    }

    fn set_token(&mut self, name: &str, token: String) {
        let (kind, default) = {
            let schema = self.schema();
            match schema.get(name) {
                Some(option) => (option.kind.clone(), schema.default_token(option)),
                None => return,
            }
        };
        if self.is_type_mode() {
            let same_as_global = same_value(
                &kind,
                &token,
                &self
                    .config()
                    .and_then(|config| config.get(name))
                    .map(str::to_string)
                    .unwrap_or_else(|| default.clone()),
            );
            let Some(type_name) = self.type_selected else { return };
            if let Some(config) = self.type_configs.get_mut(type_name) {
                if same_as_global {
                    // no override: the type falls back to H_Config.lua
                    config.remove(name);
                } else if !config.set(name, &token) {
                    config.set_or_append(name, &token);
                }
            }
            return;
        }
        let differs = !same_value(&kind, &token, &default);
        let existed = self.config().map(|c| c.get(name).is_some()).unwrap_or(false);
        let was_appended = self.appended.contains(name);
        if let Some(config) = self.config_mut() {
            if existed {
                config.set(name, &token);
                // an option that only exists because it was edited: going back to the default
                // removes the line, exactly like the web GUI never appending it
                if !differs && was_appended {
                    config.remove(name);
                    self.appended.remove(name);
                }
            } else if differs {
                config.set_or_append(name, &token);
                self.appended.insert(name.to_string());
            }
        }
    }

    fn reset_to_default(&mut self, name: &str) {
        let token = {
            let schema = self.schema();
            match schema.get(name) {
                Some(option) => schema.default_token(option),
                None => return,
            }
        };
        if self.is_type_mode() {
            self.set_token(name, token);
        } else if self.appended.contains(name) {
            if let Some(config) = self.config_mut() {
                config.remove(name);
            }
            self.appended.remove(name);
        } else {
            self.set_token(name, token);
        }
        self.status = i18n::tr_args("{0} reset to its default", &[name]);
    }

    fn is_searching(&self) -> bool {
        !self.search.trim().is_empty()
            || self.only_changed
            || (self.unit == Unit::Homunculus && self.skill_filter.is_some())
    }

    /// True when at least one setting drives this skill. The panel lists every skill of the
    /// type, but only the ones with settings can filter the grid.
    /// Settings that drive this skill, including options shared by several skills
    /// (Healing Hands and Chaotic Blessing both come from UseAutoHeal, for example).
    fn skill_settings(id: u32) -> &'static [&'static str] {
        match id {
            8001 | 8014 => &["UseAutoHeal"],
            8004 => &["UseOffensiveBuff"],
            8009 => &["UseFilirMoon"],
            8016 => &["UseVanilSelfDestruct"],
            _ => &[],
        }
    }

    fn skill_has_settings(&self, id: u32) -> bool {
        if !Self::skill_settings(id).is_empty() {
            return true;
        }
        self.schema()
            .options
            .iter()
            .any(|option| self.skills.for_setting(&option.name).map(|skill| skill.id) == Some(id))
    }

    /// Builds the rows for one frame. If `grouped`, every category is shown under its header
    /// (search, "only changed" and the "All" tab); otherwise just the selected category.
    fn build_rows(&self, grouped: bool) -> Vec<Row> {
        let schema = self.schema();
        let config = self.config();
        let type_mode = self.is_type_mode();
        let active = if type_mode { self.type_config() } else { config };
        let make = |option: &OptionDef| -> RowData {
            let present = active.map(|c| c.get(&option.name).is_some()).unwrap_or(false);
            let uses_global = type_mode && !present;
            let default_token = schema.default_token(option);
            // missing options read as their default, like the web GUI; otherwise the row would
            // look edited and the editor would be empty. In type mode a missing override reads
            // the global value.
            let current = self
                .current_value(&option.name)
                .map(str::to_string)
                .unwrap_or_else(|| default_token.clone());
            let modified = !same_value(&option.kind, &current, &default_token);
            let pending = active.map(|c| c.changed_of(&option.name)).unwrap_or(false);
            let skill = if self.unit == Unit::Homunculus {
                self.skills.for_setting(&option.name)
            } else {
                None
            };
            let values = if option.kind == "serverType" {
                vec![
                    ("SERVER_OFFICIAL".to_string(), 0),
                    ("SERVER_PRIVATE".to_string(), 1),
                ]
            } else {
                schema.enum_values(option).cloned().unwrap_or_default()
            };
            RowData {
                option: option.clone(),
                values,
                default_label: default_label(schema, option),
                current,
                present,
                uses_global,
                icon: skill.map(|skill| skill.icon),
                skill_name: skill.map(|skill| skill.name.clone()).unwrap_or_default(),
                pending,
                modified,
                comment: self.comment_for(&option.name).unwrap_or("").trim().to_string(),
            }
        };
        let mut rows = Vec::new();
        if grouped {
            for category in &schema.categories {
                let mut list: Vec<&OptionDef> = schema
                    .options
                    .iter()
                    .filter(|o| self.matches(o, Some(category)))
                    .collect();
                list.sort_by_key(|o| o.name.to_lowercase());
                if list.is_empty() {
                    continue;
                }
                rows.push(Row::Group { name: category.clone(), count: list.len() });
                for option in list {
                    rows.push(Row::Option(make(option)));
                }
            }
        } else {
            let mut list: Vec<&OptionDef> = schema
                .options
                .iter()
                .filter(|o| self.matches(o, self.category.as_deref()))
                .collect();
            list.sort_by_key(|o| o.name.to_lowercase());
            for option in list {
                rows.push(Row::Option(make(option)));
            }
        }
        rows
    }

    fn modified_count(&self) -> usize {
        let schema = self.schema();
        schema
            .options
            .iter()
            .filter(|o| {
                let current = self.current_value(&o.name).unwrap_or("");
                !same_value(&o.kind, current, &schema.default_token(o))
            })
            .count()
    }

    // ---------- drawing ----------

    fn navbar(&mut self, ctx: &egui::Context, p: &Palette) {
        egui::TopBottomPanel::top("navbar")
            .exact_height(46.0)
            .frame(egui::Frame::none().fill(p.title1))
            .show(ctx, |ui| {
                theme::band(ui, 46.0, p.title1, p.title2, |ui| {
                    let (logo, _) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::hover());
                    if let Some(texture) = &self.logo {
                        ui.painter().image(
                            texture.id(),
                            logo,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                    } else {
                        ui.painter().rect_filled(logo, egui::Rounding::same(4.0), p.primary);
                        ui.painter().text(
                            logo.center(),
                            egui::Align2::CENTER_CENTER,
                            "A",
                            egui::FontId::proportional(14.0),
                            p.title_text,
                        );
                    }
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(i18n::tr("AzzyAI Configuration"))
                            .color(p.title_text)
                            .size(18.0)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let label = match self.mode {
                            theme::Mode::Dark => i18n::tr("Light theme"),
                            theme::Mode::Light => i18n::tr("Dark theme"),
                        };
                        // Language first: it is the rightmost control in the bar.
                        egui::ComboBox::from_id_source("lang")
                            .selected_text(self.lang.label())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for lang in i18n::Lang::ALL {
                                    if ui
                                        .selectable_label(self.lang == lang, lang.label())
                                        .clicked()
                                        && self.lang != lang
                                    {
                                        self.lang = lang;
                                        i18n::set_lang(lang);
                                        i18n::save_pref(lang);
                                    }
                                }
                            });
                        ui.add_space(6.0);
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(label).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            self.mode = self.mode.toggled();
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Reload")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .on_hover_text(i18n::tr("Discard the edits and read the files again (Ctrl+R)"))
                            .clicked()
                        {
                            self.reload();
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Files")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .on_hover_text(i18n::tr("Import, export and diagnostics"))
                            .clicked()
                        {
                            self.io_open = true;
                            if self.io_export_dir.is_empty() {
                                self.io_export_dir = self.default_export_dir();
                            }
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Update AI from GitHub...")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            self.update_open = true;
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Help")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            self.help_open = true;
                        }
                    });
                });
            });
    }

    fn footer(&mut self, ctx: &egui::Context, p: &Palette) {
        egui::TopBottomPanel::bottom("footer")
            .exact_height(44.0)
            .frame(egui::Frame::none().fill(p.bg1))
            .show(ctx, |ui| {
                theme::band(ui, 44.0, p.tab2, p.bg1, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Quit")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        let changed = self.changed();
                        let apply = egui::Button::new(
                            egui::RichText::new(i18n::tr("Apply Settings"))
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(p.primary)
                        .stroke(egui::Stroke::new(1.0_f32, p.title_edge))
                        .min_size(egui::vec2(0.0, 24.0));
                        if ui
                            .add_enabled(changed > 0, apply)
                            .on_hover_text(if changed > 0 {
                                i18n::tr("Write the edited files (Ctrl+S)")
                            } else {
                                i18n::tr("No changes to apply")
                            })
                            .clicked()
                        {
                            self.save();
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new(i18n::tr("Reload")).color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            self.reload();
                        }
                        if changed > 0 {
                            theme::badge(
                                ui,
                                &i18n::tr_args("{0} not applied", &[&changed.to_string()]),
                                p.pending,
                                egui::Color32::BLACK,
                            );
                        }
                        // whatever is left of the row shows the file being edited
                        let status = if self.status.is_empty() {
                            self.active_path()
                        } else {
                            format!("{}   ·   {}", self.active_path(), self.status)
                        };
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(status).color(p.muted).size(11.0),
                                )
                                .truncate(),
                            );
                        });
                    });
                });
            });
    }

    fn body(&mut self, ctx: &egui::Context, p: &Palette) {
        let page = egui::Frame::none().fill(p.bg1).inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 8.0,
            bottom: 8.0,
        });
        egui::CentralPanel::default().frame(page).show(ctx, |ui| {
            theme::gradient(ui.painter(), ui.max_rect(), p.bg1, p.bg2);
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut active_tab = None;
            ui.horizontal(|ui| {
                for unit in [Unit::Homunculus, Unit::Mercenary] {
                    let active = self.page == Page::Settings && self.unit == unit;
                    let response = theme::tab(ui, p, &i18n::tr(unit.label()), active);
                    if active {
                        active_tab = Some(response.rect);
                    }
                    if response.clicked() {
                        let switching = self.unit != unit || self.page != Page::Settings;
                        self.page = Page::Settings;
                        self.suggest.open = false;
                        if switching {
                            self.unit = unit;
                            self.category = None;
                            self.selected = None;
                            self.drafts.clear();
                        }
                    }
                }
                for tab in tactics_ui::Tab::ALL {
                    let active = self.page == Page::Tactics(tab);
                    let response = theme::tab(ui, p, &i18n::tr(tab.layout().title), active);
                    if active {
                        active_tab = Some(response.rect);
                    }
                    if response.clicked() {
                        if self.page != Page::Tactics(tab) {
                            self.tact_filter.clear();
                        }
                        self.page = Page::Tactics(tab);
                        self.suggest.open = false;
                        self.drafts.clear();
                    }
                }
                let active = self.page == Page::Extra;
                let response = theme::tab(ui, p, &i18n::tr("Extra Options"), active);
                if active {
                    active_tab = Some(response.rect);
                }
                if response.clicked() {
                    self.page = Page::Extra;
                    self.suggest.open = false;
                    self.drafts.clear();
                }
            });
            let card_top = ui.cursor().min.y;
            let card = egui::Frame::none()
                .fill(p.panel)
                .stroke(egui::Stroke::new(1.0_f32, p.border))
                .rounding(egui::Rounding { nw: 0.0, ne: 7.0, sw: 7.0, se: 7.0 })
                .inner_margin(egui::Margin::same(10.0));
            card.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                ui.set_min_width(ui.available_width());
                ui.set_min_height(ui.available_height());
                match self.page {
                    Page::Settings => self.workspace(ui, p),
                    Page::Tactics(tab) => self.tactics_workspace(ui, p, tab),
                    Page::Extra => self.extra_workspace(ui, p),
                }
            });
            // the active tab opens into the panel: erase the card's top border under it
            if let Some(tab) = active_tab {
                let cover = egui::Rect::from_min_max(
                    egui::pos2(tab.left() + 1.0, card_top - 1.0),
                    egui::pos2(tab.right() - 1.0, card_top + 1.5),
                );
                ui.painter().rect_filled(cover, 0.0, p.panel);
            }
        });
    }

    fn tactics_workspace(&mut self, ui: &mut egui::Ui, p: &Palette, tab: tactics_ui::Tab) {
        let key = tab.key();
        let mut selected = self.tact_selected.get(key).copied().unwrap_or(0);
        let mobs = &self.mobs;
        let list_state = self.tact_list.entry(key).or_default();
        if let Some(tactics) = self.tactics.get_mut(key) {
            tactics_ui::draw(
                ui,
                p,
                tab,
                tactics,
                &mut selected,
                &mut self.tact_filter,
                &mut self.suggest,
                list_state,
                mobs,
            );
        }
        self.tact_selected.insert(key, selected);
    }

    fn extra_workspace(&mut self, ui: &mut egui::Ui, p: &Palette) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::tr("Extra options file:")).color(p.text));
            for (file, label) in [("H_Extra.lua", "Homunculus"), ("M_Extra.lua", "Mercenary")] {
                let label = i18n::tr(label);
                let active = self.extra_selected == file;
                if theme::tab(ui, p, &label, active).clicked() {
                    self.extra_selected = file;
                }
            }
            ui.add_space(8.0);
            let path = self.extra_paths[self.extra_selected].display().to_string();
            ui.add(
                egui::Label::new(egui::RichText::new(path).color(p.muted).size(11.0)).truncate(),
            );
            let changed = self.extra.get(self.extra_selected)
                != self.extra_original.get(self.extra_selected);
            if changed {
                ui.add_space(6.0);
                theme::badge(ui, &i18n::tr("Not applied"), p.pending, egui::Color32::BLACK);
            }
        });
        ui.label(
            egui::RichText::new(
                i18n::tr("Here you can modify configurations that are not supported by the GUI."),
            )
            .color(p.muted)
            .size(11.0),
        );
        ui.add_space(4.0);
        let file = self.extra_selected;
        let mut text = self.extra.get(file).cloned().unwrap_or_default();
        let height = (ui.available_height() - 10.0).max(200.0);
        let mut edited = false;
        egui::Frame::none()
            .fill(p.grid)
            .stroke(egui::Stroke::new(1.0_f32, p.border))
            .rounding(egui::Rounding::same(4.0))
            .inner_margin(egui::Margin::same(4.0))
            .show(ui, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        let response = ui.add_sized(
                            [ui.available_width().max(300.0), height - 12.0],
                            egui::TextEdit::multiline(&mut text)
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .desired_rows(30)
                                .id_source(format!("extra:{file}")),
                        );
                        if response.changed() {
                            edited = true;
                        }
                    });
            });
        if edited {
            self.extra.insert(file, text);
        }
    }

    fn workspace(&mut self, ui: &mut egui::Ui, p: &Palette) {
        self.toolbar(ui, p);
        ui.add_space(6.0);
        if self.unit == Unit::Homunculus {
            self.type_selector(ui, p);
            ui.add_space(6.0);
            self.skills_card(ui, p);
            ui.add_space(6.0);
        }
        self.category_tabs(ui, p);
        ui.add_space(6.0);

        let searching = self.is_searching();
        let grouped = searching || self.category.is_none();
        let legend_h = 24.0;
        let grid_h = (ui.available_height() - legend_h).max(200.0);
        let gap = ui.spacing().item_spacing.x;
        let total_w = ui.available_width();
        let desc_w = (total_w * 0.30).clamp(250.0, 380.0);
        let grid_w = (total_w - desc_w - gap).max(200.0);

        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(grid_w, grid_h),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    self.grid_card(ui, p, grouped, grid_h);
                },
            );
            // Bound the card to the row height: its scroll area is auto_shrink=false and would
            // otherwise grow to the whole panel, pushing the legend under the footer.
            ui.allocate_ui_with_layout(
                egui::vec2(desc_w, grid_h),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.description_card(ui, p, grid_h),
            );
        });
        ui.add_space(4.0);
        self.legend(ui, p, grouped);
    }

    // ---------- import / export ----------

    fn user_ai_dir(&self) -> PathBuf {
        self.paths["H"]
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn default_export_dir(&self) -> String {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        self.user_ai_dir()
            .join(format!("azzy-export-{stamp}"))
            .display()
            .to_string()
    }

    /// Writes every configuration file from the current editor state (including changes not
    /// applied yet), like the web GUI's "Download zip".
    fn export_all(&self, dir: &Path) -> Result<usize, String> {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let mut count = 0;
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            if let Some(config) = self.configs.get(key) {
                std::fs::write(dir.join(file), config.to_text())
                    .map_err(|e| format!("{}: {e}", dir.join(file).display()))?;
                count += 1;
            }
        }
        for tab in tactics_ui::Tab::ALL {
            if let Some(model) = self.tactics.get(tab.key()) {
                let path = dir.join(tab.layout().file);
                std::fs::write(&path, model.to_text())
                    .map_err(|e| format!("{}: {e}", path.display()))?;
                count += 1;
            }
        }
        for file in EXTRA_FILES {
            let text = self.extra.get(file).cloned().unwrap_or_default();
            std::fs::write(dir.join(file), text)
                .map_err(|e| format!("{}: {e}", dir.join(file).display()))?;
            count += 1;
        }
        let types_dir = dir.join("Types");
        std::fs::create_dir_all(&types_dir)
            .map_err(|e| format!("{}: {e}", types_dir.display()))?;
        for name in TYPE_NAMES {
            if let Some(config) = self.type_configs.get(name) {
                let text = config.to_text();
                if text.trim().is_empty() {
                    continue;
                }
                let path = types_dir.join(format!("{name}.lua"));
                std::fs::write(&path, text)
                    .map_err(|e| format!("{}: {e}", path.display()))?;
                count += 1;
            }
        }
        Ok(count)
    }

    fn export_diagnostics(&self) -> Result<String, String> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let dir = self.user_ai_dir().join(format!("azzy-diagnostics-{stamp}"));
        let count = self.export_all(&dir)?;
        let mut text = format!(
            "azzy-native {}\nunix time: {stamp}\nos: {} {}\nuser_ai: {}\nexported files: {count}\nH_Config: {}\nM_Config: {}\nH_Tactics: {}\nM_Tactics: {}\nH_PVP: {}\nM_PVP: {}\n",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
            self.user_ai_dir().display(),
            self.paths["H"].display(),
            self.paths["M"].display(),
            self.tact_paths["homtact"].display(),
            self.tact_paths["mertact"].display(),
            self.tact_paths["hompvp"].display(),
            self.tact_paths["merpvp"].display(),
        );
        text.push_str(&format!(
            "settings changed: {}\ntactics entries: {}\ntype selected: {}\n",
            self.changed(),
            self.tactics.values().map(|tactics| tactics.entries.len()).sum::<usize>(),
            self.type_selected.unwrap_or("Global"),
        ));
        if let Some(log) = log_tail() {
            text.push_str("\n--- azzy-native.log (tail) ---\n");
            text.push_str(&log);
        }
        let path = dir.join("diagnostics.txt");
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(dir.display().to_string())
    }

    /// Reads the known files from `dir` into the editor. Nothing is written to the real files
    /// until Apply Settings; the imported text is marked as dirty so Apply writes it verbatim.
    fn import_folder(&mut self, dir: &Path) -> Result<Vec<String>, String> {
        if !dir.is_dir() {
            return Err(format!("not a folder: {}", dir.display()));
        }
        let mut imported = Vec::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            if let Ok(text) = std::fs::read_to_string(dir.join(file)) {
                self.configs.insert(key, Config::parse(&text));
                self.dirty_configs.insert(key);
                imported.push(file.to_string());
            }
        }
        for tab in tactics_ui::Tab::ALL {
            if let Ok(text) = std::fs::read_to_string(dir.join(tab.layout().file)) {
                self.tactics.insert(
                    tab.key(),
                    Tactics::parse(&text, tab.layout().kind),
                );
                self.dirty_tactics.insert(tab.key());
                imported.push(tab.layout().file.to_string());
            }
        }
        for file in EXTRA_FILES {
            if let Ok(text) = std::fs::read_to_string(dir.join(file)) {
                self.extra.insert(file, text);
                self.dirty_extra.insert(file);
                imported.push(file.to_string());
            }
        }
        let types_dir = dir.join("Types");
        if types_dir.is_dir() {
            for name in TYPE_NAMES {
                if let Ok(text) = std::fs::read_to_string(types_dir.join(format!("{name}.lua"))) {
                    self.type_configs.insert(name, Config::parse(&text));
                    self.dirty_types.insert(name);
                    imported.push(format!("Types/{name}.lua"));
                }
            }
        }
        self.drafts.clear();
        self.appended.clear();
        if imported.is_empty() {
            Err(format!("no known configuration files in {}", dir.display()))
        } else {
            Ok(imported)
        }
    }

    /// Collects the startup version check (the latest GitHub tag is compared with CVersion).
    fn poll_version(&mut self) {
        let Some(rx) = self.version_rx.take() else { return };
        match rx.try_recv() {
            Ok((Some(installed), Ok(Some(latest)))) => {
                if azzy_update::newer_version(&latest, &installed) {
                    crate::log(&format!("version check: {latest} is newer than {installed}"));
                    self.new_version = Some(latest);
                } else {
                    crate::log(&format!("version check: {installed} is up to date"));
                }
            }
            Ok(_) => {}
            Err(std::sync::mpsc::TryRecvError::Empty) => self.version_rx = Some(rx),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {}
        }
    }

    /// A slim bar under the navbar when GitHub has a newer release tag.
    fn version_banner(&mut self, ctx: &egui::Context, p: &Palette) {
        let Some(latest) = self.new_version.clone() else { return };
        let mut update = false;
        let mut dismiss = false;
        egui::TopBottomPanel::top("version-banner")
            .frame(egui::Frame::none().fill(p.pending.gamma_multiply(0.15)))
            .show(ctx, |ui| {
                theme::band(
                    ui,
                    32.0,
                    p.pending.gamma_multiply(0.18),
                    p.pending.gamma_multiply(0.08),
                    |ui| {
                        ui.label(
                            egui::RichText::new(i18n::tr_args(
                                "New version {0} is available.",
                                &[&latest],
                            ))
                            .color(p.text)
                            .strong()
                            .size(12.0),
                        );
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.small_button(i18n::tr("Dismiss")).clicked() {
                                    dismiss = true;
                                }
                                if ui
                                    .small_button(i18n::tr("Update"))
                                    .clicked()
                                {
                                    update = true;
                                }
                            },
                        );
                    },
                );
            });
        if dismiss {
            self.new_version = None;
        }
        if update {
            self.update_open = true;
        }
    }

    /// Collects the background update thread's messages (called once per frame).
    fn poll_update(&mut self) {
        let Some(rx) = self.update_rx.take() else { return };
        let mut keep = true;
        loop {
            match rx.try_recv() {
                Ok(UpdateEvent::Applying) => self.update_state = UpdateState::Applying,
                Ok(UpdateEvent::Done(outcome)) => {
                    let staged = !outcome.report.staged.is_empty();
                    self.update_state = UpdateState::Done(outcome);
                    keep = false;
                    if self.update_target == self.user_ai_dir() && self.changed() == 0 {
                        self.reload();
                        self.status = i18n::tr("updated from GitHub");
                    } else {
                        self.status = i18n::tr(
                            "update applied; reload to read the new files (unsaved edits kept)",
                        );
                    }
                    if staged {
                        self.status.push_str(" · ");
                        self.status.push_str(&i18n::tr("a new executable was staged as .exe.new"));
                    }
                    break;
                }
                Ok(UpdateEvent::Failed(error)) => {
                    self.update_state = UpdateState::Failed(error);
                    keep = false;
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    if !matches!(&self.update_state, UpdateState::Done(_) | UpdateState::Failed(_)) {
                        self.update_state = UpdateState::Failed("the update stopped".into());
                    }
                    keep = false;
                    break;
                }
            }
        }
        if keep {
            self.update_rx = Some(rx);
        }
    }

    /// Downloads the archive and applies it on a worker thread, so the window keeps drawing.
    fn start_update(&mut self, ctx: &egui::Context) {
        if self.update_rx.is_some() {
            return;
        }
        let dir = PathBuf::from(self.update_dir.trim());
        if !dir.is_dir() {
            self.update_state = UpdateState::Failed(i18n::tr_args(
                "Folder not found: {0}",
                &[&dir.display().to_string()],
            ));
            return;
        }
        self.update_target = dir.clone();
        let overwrite = self.update_overwrite_presets;
        let url = UPDATE_URL.to_string();
        let (tx, rx) = std::sync::mpsc::channel();
        self.update_rx = Some(rx);
        self.update_state = UpdateState::Downloading;
        let repaint = ctx.clone();
        std::thread::spawn(move || {
            let result = azzy_update::download(&url).and_then(|bytes| {
                let _ = tx.send(UpdateEvent::Applying);
                repaint.request_repaint();
                let report = azzy_update::apply_zip(&bytes, &dir, overwrite)?;
                // The native window itself comes from the archive's tools/ folder; a running exe
                // cannot be replaced, so it is staged as <exe>.new.
                let mut self_error = None;
                let self_update = match std::env::current_exe() {
                    Ok(exe) => {
                        // Find our own build by file name, wherever the archive keeps it: the dev
                        // tree today, USER_AI once this window replaces the C# GUI. No path is
                        // baked in, so moving the binary needs no code change.
                        let name = exe
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("azzy-native.exe");
                        match azzy_update::find_binary(&bytes, name) {
                            Ok(Some(binary)) => match azzy_update::stage_binary(&binary, &exe) {
                                Ok(Some(path)) => SelfUpdate::Staged(path),
                                Ok(None) => SelfUpdate::Unchanged,
                                Err(error) => {
                                    self_error = Some(error);
                                    SelfUpdate::Unchanged
                                }
                            },
                            Ok(None) => SelfUpdate::NotInArchive,
                            Err(error) => {
                                self_error = Some(error);
                                SelfUpdate::NotInArchive
                            }
                        }
                    }
                    Err(error) => {
                        self_error = Some(error.to_string());
                        SelfUpdate::NotInArchive
                    }
                };
                Ok(UpdateOutcome { report, self_update, self_error })
            });
            let event = match result {
                Ok(outcome) => UpdateEvent::Done(outcome),
                Err(error) => UpdateEvent::Failed(error),
            };
            let _ = tx.send(event);
            repaint.request_repaint();
        });
    }

    fn help_window(&mut self, ctx: &egui::Context, p: &Palette) {
        if !self.help_open {
            return;
        }
        let mut open = self.help_open;
        egui::Window::new(i18n::tr("About"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(420.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("AzzyAI")
                        .color(p.text)
                        .strong()
                        .size(20.0),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{}: {}",
                        i18n::tr("AzzyAI version"),
                        self.version.as_deref().unwrap_or("?")
                    ))
                    .color(p.muted)
                    .size(12.0),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(i18n::tr(
                        "Native desktop configuration for AzzyAI, written in Rust; it replaces the C# application.",
                    ))
                    .color(p.text)
                    .size(12.0),
                );
                ui.add_space(8.0);
                let mut url = None;
                ui.horizontal(|ui| {
                    if ui.button(i18n::tr("Documentation")).clicked() {
                        url = Some("https://github.com/jknpj/AzzyAI");
                    }
                    if ui.button(i18n::tr("Web GUI")).clicked() {
                        url = Some("https://jknpj.github.io/AzzyAI-webgui/");
                    }
                    if ui.button(i18n::tr("Report an issue")).clicked() {
                        url = Some("https://github.com/jknpj/AzzyAI/issues/new");
                    }
                });
                if let Some(url) = url {
                    if let Err(error) = open_url(url) {
                        self.status = error;
                    }
                }
                ui.add_space(6.0);
                ui.separator();
                if ui.button(i18n::tr("Close")).clicked() {
                    self.help_open = false;
                }
            });
        self.help_open = open;
    }

    fn update_window(&mut self, ctx: &egui::Context, p: &Palette) {
        if !self.update_open {
            return;
        }
        if self.update_dir.trim().is_empty() {
            self.update_dir = self.user_ai_dir().display().to_string();
        }
        let mut open = self.update_open;
        egui::Window::new(i18n::tr("Update AI from GitHub"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(560.0)
            .min_width(560.0)
            .max_width(560.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                let busy = matches!(
                    &self.update_state,
                    UpdateState::Downloading | UpdateState::Applying
                );
                ui.label(
                    egui::RichText::new(i18n::tr(
                        "Download the latest AI from GitHub and apply it here? Files you edited (H_Config.lua, tactics, extra, A_Friends) are kept and the replaced ones are copied to backup/.",
                    ))
                    .color(p.text)
                    .size(12.0),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::tr("Folder to update:")).color(p.text));
                    ui.add_enabled(
                        !busy,
                        egui::TextEdit::singleline(&mut self.update_dir)
                            .desired_width(400.0)
                            .id_source("update-dir"),
                    );
                });
                ui.add_space(4.0);
                ui.checkbox(
                    &mut self.update_overwrite_presets,
                    i18n::tr("Replace Types/ presets too"),
                );
                ui.add_space(6.0);
                let mut restart = false;
                match &self.update_state {
                    UpdateState::Idle => {}
                    UpdateState::Downloading => {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(i18n::tr("Downloading the update..."));
                        });
                    }
                    UpdateState::Applying => {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(i18n::tr("Applying the update..."));
                        });
                    }
                    UpdateState::Done(outcome) => {
                        let report = &outcome.report;
                        ui.label(
                            egui::RichText::new(i18n::tr_args(
                                "Updated {0}, added {1}, kept {2}.",
                                &[
                                    &report.updated.to_string(),
                                    &report.added.to_string(),
                                    &report.kept.to_string(),
                                ],
                            ))
                            .color(p.text)
                            .size(12.0),
                        );
                        if let Some(backup) = &report.backup {
                            ui.label(
                                egui::RichText::new(i18n::tr_args("Backup: {0}", &[backup]))
                                    .color(p.muted)
                                    .size(11.0),
                            );
                        }
                        if !report.staged.is_empty() {
                            ui.label(
                                egui::RichText::new(i18n::tr_args(
                                    "New executable staged as {0}; the next launch uses it.",
                                    &[&report.staged.join(", ")],
                                ))
                                .color(p.muted)
                                .size(11.0),
                            );
                        }
                        match &outcome.self_update {
                            SelfUpdate::Staged(path) => {
                                ui.label(
                                    egui::RichText::new(i18n::tr_args(
                                        "New azzy-native.exe staged as {0}; restart to use it.",
                                        &[&path.display().to_string()],
                                    ))
                                    .color(p.text)
                                    .size(11.5),
                                );
                                if ui
                                    .button(egui::RichText::new(i18n::tr("Restart now")).strong())
                                    .clicked()
                                {
                                    restart = true;
                                }
                            }
                            SelfUpdate::Unchanged => {
                                ui.label(
                                    egui::RichText::new(i18n::tr(
                                        "azzy-native.exe is already up to date.",
                                    ))
                                    .color(p.muted)
                                    .size(11.0),
                                );
                            }
                            SelfUpdate::NotInArchive => {
                                ui.label(
                                    egui::RichText::new(i18n::tr(
                                        "The archive has no azzy-native.exe build.",
                                    ))
                                    .color(p.muted)
                                    .size(11.0),
                                );
                            }
                        }
                        if let Some(error) = &outcome.self_error {
                            ui.label(
                                egui::RichText::new(i18n::tr_args(
                                    "Could not stage azzy-native.exe: {0}",
                                    &[error],
                                ))
                                .color(p.pending)
                                .size(11.0),
                            );
                        }
                        if self.changed() > 0 {
                            ui.label(
                                egui::RichText::new(i18n::tr(
                                    "Reload to read the new files (unsaved edits will be discarded).",
                                ))
                                .color(p.pending)
                                .size(11.0),
                            );
                        }
                    }
                    UpdateState::Failed(error) => {
                        ui.label(
                            egui::RichText::new(i18n::tr_args("Update failed: {0}", &[error]))
                                .color(p.pending)
                                .size(11.5)
                                .strong(),
                        );
                    }
                }
                let staged_path = match &self.update_state {
                    UpdateState::Done(outcome) => match &outcome.self_update {
                        SelfUpdate::Staged(path) => Some(path.clone()),
                        _ => None,
                    },
                    _ => None,
                };
                if restart {
                    if let Some(path) = staged_path {
                        match restart_with_staged_exe(&path) {
                            Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                            Err(error) => {
                                self.update_state = UpdateState::Failed(i18n::tr_args(
                                    "Could not restart: {0}",
                                    &[&error],
                                ));
                            }
                        }
                    }
                }
                ui.add_space(4.0);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(
                            !busy,
                            egui::Button::new(egui::RichText::new(i18n::tr("Update")).strong()),
                        )
                        .clicked()
                    {
                        self.start_update(ctx);
                    }
                    if ui.button(i18n::tr("Close")).clicked() {
                        self.update_open = false;
                    }
                });
            });
        self.update_open = open;
    }

    fn files_window(&mut self, ctx: &egui::Context, p: &Palette) {
        if !self.io_open {
            return;
        }
        let mut open = self.io_open;
        egui::Window::new(i18n::tr("Import / Export"))
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_width(640.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(
                        i18n::tr("All exports use the current editor state, including changes not applied yet."),
                    )
                    .color(p.muted)
                    .size(11.0),
                );
                ui.add_space(4.0);
                ui.label(egui::RichText::new(i18n::tr("Export to folder")).strong().color(p.text));
                ui.add_sized(
                    [ui.available_width(), 20.0],
                    egui::TextEdit::singleline(&mut self.io_export_dir)
                        .id_source("io-export-dir"),
                );
                ui.horizontal(|ui| {
                    if ui.button(i18n::tr("Export all files")).clicked() {
                        let dir = PathBuf::from(self.io_export_dir.trim());
                        self.io_status = match self.export_all(&dir) {
                            Ok(count) => format!("exported {count} file(s) to {}", dir.display()),
                            Err(e) => format!("export failed: {e}"),
                        };
                    }
                    if ui
                        .button(i18n::tr("Export diagnostics"))
                        .on_hover_text("Configurations, paths and the tail of azzy-native.log")
                        .clicked()
                    {
                        self.io_status = match self.export_diagnostics() {
                            Ok(dir) => format!("diagnostics written to {dir}"),
                            Err(e) => format!("diagnostics failed: {e}"),
                        };
                    }
                });
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(4.0);
                ui.label(egui::RichText::new(i18n::tr("Import from folder")).strong().color(p.text));
                ui.label(
                    egui::RichText::new(
                        i18n::tr("Folder with H_Config.lua, M_Config.lua, the tactics files and/or Types/."),
                    )
                    .color(p.muted)
                    .size(11.0),
                );
                ui.add_sized(
                    [ui.available_width(), 20.0],
                    egui::TextEdit::singleline(&mut self.io_import_dir)
                        .hint_text(i18n::tr("path to a USER_AI folder or an exported folder"))
                        .id_source("io-import-dir"),
                );
                if ui.button(i18n::tr("Import into the editor")).clicked() {
                    let dir = PathBuf::from(self.io_import_dir.trim());
                    self.io_status = match self.import_folder(&dir) {
                        Ok(files) => format!(
                            "imported {} file(s): {}. Nothing was written yet; Apply Settings writes them.",
                            files.len(),
                            files.join(", ")
                        ),
                        Err(e) => format!("import failed: {e}"),
                    };
                }
                if !self.io_status.is_empty() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(&self.io_status)
                            .color(p.pending)
                            .size(11.5)
                            .strong(),
                    );
                }
            });
        self.io_open = open;
    }

    fn skills_card(&mut self, ui: &mut egui::Ui, p: &Palette) {
        let mut clicked: Option<Option<u32>> = None;
        theme::card(p).show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            theme::title_bar(p, ui, &i18n::tr("Homunculus skills"));
            ui.horizontal(|ui| {
                if let Some(id) = self.skill_filter {
                    if let Some(skill) = self.skills.get(id) {
                        if ui
                            .small_button(format!("{}  ×", skill.name))
                            .on_hover_text(i18n::tr("Show all settings"))
                            .clicked()
                        {
                            clicked = Some(None);
                        }
                        ui.label(
                            egui::RichText::new(i18n::tr("Showing only this skill's settings."))
                                .color(p.muted)
                                .size(11.0),
                        );
                    }
                } else {
                    ui.label(
                        egui::RichText::new(i18n::tr("Click a skill to see only its settings."))
                            .color(p.muted)
                            .size(11.0),
                    );
                }
            });
            ui.add_space(4.0);
            if let Some(texture) = self.skill_texture.as_ref() {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
                    for (key, label) in skills::HOMUNCULI {
                        let list: Vec<&skills::Skill> = self
                            .skills
                            .skills
                            .iter()
                            .filter(|skill| {
                                skill.homun == key 
                            })
                            .collect();
                        if list.is_empty() {
                            continue;
                        }
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
                            ui.label(egui::RichText::new(label).color(p.muted).size(10.0));
                        for skill in list {
                            let active = self.skill_filter == Some(skill.id);
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(28.0, 28.0),
                                egui::Sense::click(),
                            );
                            // Like the web GUI's .skill-btn: a 24 px icon with 2 px padding, no
                            // frame at rest, a subtle border on hover and a primary ring when active.
                            let fill = if active {
                                p.primary.gamma_multiply(0.15)
                            } else if response.hovered() {
                                p.panel2
                            } else {
                                egui::Color32::TRANSPARENT
                            };
                            ui.painter().rect_filled(rect, egui::Rounding::same(4.0), fill);
                            if active {
                                ui.painter().rect_stroke(
                                    rect,
                                    egui::Rounding::same(4.0),
                                    egui::Stroke::new(1.0_f32, p.primary),
                                );
                            } else if response.hovered() {
                                ui.painter().rect_stroke(
                                    rect,
                                    egui::Rounding::same(4.0),
                                    egui::Stroke::new(1.0_f32, p.btn_border),
                                );
                            }
                            let icon = egui::Rect::from_center_size(
                                rect.center(),
                                egui::vec2(24.0, 24.0),
                            );
                            ui.painter().image(
                                texture.id(),
                                icon,
                                self.skills.icon_uv(skill.icon),
                                egui::Color32::WHITE,
                            );
                            if response.on_hover_text(&skill.name).clicked() {
                                clicked = Some(if active { None } else { Some(skill.id) });
                            }
                        }
                        });
                        ui.add_space(10.0);
                    }
                });
            }
        });
        if let Some(selection) = clicked {
            self.skill_filter = selection;
            self.category = None;
            if let Some(id) = selection {
                if let Some(selected) = self.selected.clone() {
                    let belongs = self
                        .schema()
                        .get(&selected)
                        .and_then(|option| self.skills.for_setting(&option.name))
                        .map(|skill| skill.id)
                        == Some(id);
                    if !belongs {
                        self.selected = None;
                    }
                }
            }
        }
    }

    fn type_selector(&mut self, ui: &mut egui::Ui, p: &Palette) {
        let mut selection: Option<Option<&'static str>> = None;
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::tr("Settings for:")).color(p.text));
            egui::ComboBox::from_id_source("settings-for")
                .width(240.0)
                .selected_text(match self.type_selected {
                    Some(name) => name.to_string(),
                    None => i18n::tr("Global (all homunculi)"),
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(self.type_selected.is_none(), i18n::tr("Global (all homunculi)"))
                        .clicked()
                    {
                        selection = Some(None);
                    }
                    for name in TYPE_NAMES {
                        if ui
                            .selectable_label(self.type_selected == Some(name), name)
                            .clicked()
                        {
                            selection = Some(Some(name));
                        }
                    }
                });
            if let Some(name) = self.type_selected {
                if ui.button(i18n::tr("Use global values")).clicked() {
                    self.clear_type_overrides();
                }
                ui.label(
                    egui::RichText::new(i18n::tr_args(
                        "Only values that differ from the global settings are saved, in Types/{0}.lua.",
                        &[name],
                    ))
                    .color(p.muted)
                    .size(11.0),
                );
            }
        });
        if let Some(selection) = selection {
            if selection != self.type_selected {
                self.type_selected = selection;
                self.drafts.clear();
                self.appended.clear();
                self.status = match selection {
                    Some(name) => {
                        i18n::tr_args("editing the {0} overrides (Types/{0}.lua)", &[name])
                    }
                    None => i18n::tr("editing the global settings"),
                };
            }
        }
    }

    fn clear_type_overrides(&mut self) {
        let Some(name) = self.type_selected else { return };
        if let Some(config) = self.type_configs.get_mut(name) {
            let names: Vec<String> = config.assignments().map(|a| a.name.clone()).collect();
            for name in names {
                config.remove(&name);
            }
        }
        self.drafts.clear();
        self.status =
            i18n::tr_args("{0}: using the global values (saved on Apply)", &[name]);
    }

    fn toolbar(&mut self, ui: &mut egui::Ui, p: &Palette) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::tr("Search settings:")).color(p.text));
            let reserve = 350.0;
            let width = (ui.available_width() - reserve).max(160.0);
            let response = ui.add_sized(
                [width, 22.0],
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text(i18n::tr("Name or description (Ctrl+F)"))
                    .id_source("search"),
            );
            if std::mem::take(&mut self.focus_search) {
                response.request_focus();
            }
            ui.checkbox(&mut self.only_changed, i18n::tr("Only changed from default"));
            if ui.button(i18n::tr("Clear filters")).clicked() {
                self.search.clear();
                self.only_changed = false;
                self.category = None;
            }
        });
    }

    fn category_tabs(&mut self, ui: &mut egui::Ui, p: &Palette) {
        // counts ignore the selected category (they are per category already) but follow the search
        let (total, categories) = {
            let schema = self.schema();
            let total = schema.options.iter().filter(|o| self.matches(o, None)).count();
            let categories: Vec<(String, usize)> = schema
                .categories
                .iter()
                .map(|category| {
                    let count = schema
                        .options
                        .iter()
                        .filter(|o| self.matches(o, Some(category)))
                        .count();
                    (category.clone(), count)
                })
                .collect();
            (total, categories)
        };
        let searching = self.is_searching();
        let mut selection: Option<Option<String>> = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
            if theme::tab(ui, p, &i18n::tr_args("All ({0})", &[&total.to_string()]), !searching && self.category.is_none())
                .clicked()
            {
                selection = Some(None);
            }
            for (category, count) in categories {
                let active = !searching && self.category.as_deref() == Some(category.as_str());
                if theme::tab(ui, p, &format!("{} ({count})", i18n::tr(&category)), active).clicked() {
                    selection = Some(Some(category));
                }
            }
        });
        if let Some(selection) = selection {
            // clicking a category while a search/skill filter is active means "show me that category"
            if searching {
                self.search.clear();
                self.only_changed = false;
                self.skill_filter = None;
            }
            self.category = selection;
        }
    }

    fn grid_card(&mut self, ui: &mut egui::Ui, p: &Palette, grouped: bool, height: f32) {
        egui::Frame::none()
            .fill(p.grid)
            .stroke(egui::Stroke::new(1.0_f32, p.border))
            .rounding(egui::Rounding::same(4.0))
            .inner_margin(egui::Margin::same(3.0))
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(ui.available_width(), (height - 8.0).max(100.0)));
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        self.grid_rows(ui, p, grouped);
                    });
            });
    }

    fn grid_rows(&mut self, ui: &mut egui::Ui, p: &Palette, grouped: bool) {
        let rows = self.build_rows(grouped);
        let selected = self.selected.clone();
        let mut edits: Vec<(String, String)> = Vec::new();
        let mut reset: Option<String> = None;
        let mut clicked: Option<String> = None;
        for row in rows {
            match row {
                Row::Group { name, count } => group_header(ui, p, &i18n::tr(&name), count),
                Row::Option(data) => {
                    let is_selected = selected.as_deref() == Some(data.option.name.as_str());
                    let (select, edit, do_reset) = option_row(
                        ui,
                        p,
                        &data,
                        is_selected,
                        &mut self.drafts,
                        self.skill_texture.as_ref(),
                        &self.skills,
                    );
                    if select {
                        clicked = Some(data.option.name.clone());
                    }
                    if let Some(token) = edit {
                        edits.push((data.option.name.clone(), token));
                    }
                    if do_reset {
                        reset = Some(data.option.name.clone());
                    }
                }
            }
        }
        for (name, token) in edits {
            self.set_token(&name, token);
        }
        if let Some(name) = reset {
            self.reset_to_default(&name);
        }
        if let Some(name) = clicked {
            self.selected = Some(name);
        }
    }

    fn description_card(&self, ui: &mut egui::Ui, p: &Palette, height: f32) {
        theme::card(p).show(ui, |ui| {
            // The card is drawn next to the grid, inside a horizontal row, and Frame::show
            // inherits that layout: without this the name, facts and text would be laid out
            // side by side and the description would never get any width.
            ui.vertical(|ui| {
                ui.set_min_size(egui::vec2(ui.available_width(), (height - 12.0).max(100.0)));
                // The web GUI puts the selected setting's name in the card header, and
                // "Description" while nothing is selected.
                let option = self.selected.as_deref().and_then(|name| self.schema().get(name));
                let title = match option {
                    Some(option) => option.name.clone(),
                    None => i18n::tr("Description"),
                };
                theme::title_bar(p, ui, &title);
                ui.add_space(6.0);
                let Some(option) = option else {
                    ui.label(
                        egui::RichText::new(i18n::tr(
                            "Select a setting to see its description here.",
                        ))
                        .color(p.muted)
                        .size(12.0),
                    );
                    return;
                };
                let mut facts = format!("{}  ·  {}", i18n::tr(&option.category), option.kind);
                if let (Some(min), Some(max)) = (option.min, option.max) {
                    facts.push_str(&format!("  ·  {min}..{max}"));
                }
                facts.push_str(&format!(
                    "  ·  {}",
                    i18n::tr_args("default {0}", &[&default_label(self.schema(), option)])
                ));
                if let Some(skill) = self.skills.for_setting(&option.name) {
                    facts.push_str(&format!("  ·  {}", i18n::tr_args("Skill: {0}", &[&skill.name])));
                }
                ui.label(egui::RichText::new(facts).color(p.muted).size(11.0));
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);
                egui::ScrollArea::vertical()
                    .id_source("description-scroll")
                    .auto_shrink([false, false])
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        if self.unit == Unit::Homunculus {
                            if let Some(skill) = self.skills.for_setting(&option.name) {
                                self.skill_card(ui, p, skill);
                                ui.add_space(8.0);
                                ui.separator();
                                ui.add_space(4.0);
                            }
                        }
                        let description = self.description(option);
                        if description.trim().is_empty() {
                            ui.label(
                                egui::RichText::new(i18n::tr(
                                    "No description available for this property.",
                                ))
                                .color(p.muted)
                                .size(12.0),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(description).color(p.text).size(12.5),
                            );
                        }
                        if let Some(comment) = self.config().and_then(|c| c.comment_of(&option.name)) {
                            let comment = comment.trim();
                            if !comment.is_empty() {
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new(i18n::tr("In the file:"))
                                        .color(p.muted)
                                        .size(11.0)
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(comment).color(p.muted).italics().size(11.5),
                                );
                            }
                        }
                    });
            });
        });
    }

    fn legend(&self, ui: &mut egui::Ui, p: &Palette, grouped: bool) {
        let schema = self.schema();
        let shown = schema
            .options
            .iter()
            .filter(|o| {
                if grouped {
                    self.matches(o, None)
                } else {
                    self.matches(o, self.category.as_deref())
                }
            })
            .count();
        let modified = self.modified_count();
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(i18n::tr_args(
                    "{0} settings  ·  {1} changed from default",
                    &[&shown.to_string(), &modified.to_string()],
                ))
                .color(p.muted)
                .size(11.0),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(i18n::tr("Not applied yet")).color(p.muted).size(11.0));
                theme::swatch(ui, p.pending);
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(i18n::tr("Changed from default"))
                        .color(p.muted)
                        .size(11.0),
                );
                theme::swatch(ui, p.modified);
            });
        });
    }
}

impl App {
    fn draw(&mut self, ctx: &egui::Context) {
        if self.applied_mode != Some(self.mode) {
            theme::apply(ctx, self.mode);
            self.applied_mode = Some(self.mode);
        }
        let p = theme::palette(self.mode);
        if self.logo.is_none() {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [48, 48],
                include_bytes!("../data/azzyai-48.rgba"),
            );
            self.logo = Some(ctx.load_texture("azzyai", image, egui::TextureOptions::LINEAR));
        }
        if self.skill_texture.is_none() {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [self.skills.width as usize, self.skills.height as usize],
                self.skills.atlas,
            );
            self.skill_texture = Some(ctx.load_texture(
                "skill-icons",
                image,
                egui::TextureOptions::NEAREST,
            ));
        }

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) && self.changed() > 0 {
            self.save();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::F)) {
            self.focus_search = true;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::R)) {
            self.reload();
        }

        self.poll_update();
        self.poll_version();
        self.navbar(ctx, &p);
        self.version_banner(ctx, &p);
        self.footer(ctx, &p);
        self.body(ctx, &p);
        self.files_window(ctx, &p);
        self.update_window(ctx, &p);
        self.help_window(ctx, &p);
        if matches!(
            &self.update_state,
            UpdateState::Downloading | UpdateState::Applying
        ) {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        if self.version_rx.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frames += 1;
        if self.frames == 1 || self.frames == 5 {
            log(&format!("frame {}", self.frames));
        }
        self.draw(ctx);
    }
}

/// A category header inside the grid, like the web GUI's `tr.group-row th`.
fn group_header(ui: &mut egui::Ui, p: &Palette, name: &str, count: usize) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 25.0),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, egui::Rounding::same(3.0), p.panel2);
    ui.painter().text(
        rect.left_center() + egui::vec2(8.0, 0.0),
        egui::Align2::LEFT_CENTER,
        name,
        egui::FontId::proportional(12.5),
        p.tab_text,
    );
    // the count pill on the right, like the badge in the web group row
    let text = ui.painter().layout_no_wrap(
        count.to_string(),
        egui::FontId::proportional(11.0),
        egui::Color32::WHITE,
    );
    let size = text.size() + egui::vec2(12.0, 3.0);
    let pill = egui::Rect::from_min_size(
        egui::pos2(rect.right() - size.x - 8.0, rect.center().y - size.y / 2.0),
        size,
    );
    ui.painter().rect_filled(pill, egui::Rounding::same(8.0), p.btn_border);
    ui.painter().galley(pill.center() - text.size() / 2.0, text, egui::Color32::WHITE);
}

/// One settings row: marker, name, editor and (when changed from default) the reset button.
fn option_row(
    ui: &mut egui::Ui,
    p: &Palette,
    row: &RowData,
    selected: bool,
    drafts: &mut BTreeMap<String, String>,
    skill_texture: Option<&egui::TextureHandle>,
    skills: &skills::Skills,
) -> (bool, Option<String>, bool) {
    let full_w = ui.available_width().max(120.0);
    let height = 28.0;
    // reserve the full row first: allocate_ui_at_rect would advance only by the content height
    // and the next row would overlap this one
    let (rect, _) = ui.allocate_exact_size(egui::vec2(full_w, height), egui::Sense::hover());
    let hovered = ui.rect_contains_pointer(rect);
    let background = if row.pending {
        p.pending.gamma_multiply(0.10)
    } else if selected {
        p.tab2
    } else if hovered {
        p.panel2
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, egui::Rounding::same(3.0), background);
    let marker = if row.pending {
        Some(p.pending)
    } else if row.modified {
        Some(p.modified)
    } else {
        None
    };
    if let Some(color) = marker {
        let bar = egui::Rect::from_min_size(
            rect.left_top() + egui::vec2(0.0, 3.0),
            egui::vec2(4.0, height - 6.0),
        );
        ui.painter().rect_filled(bar, egui::Rounding::same(2.0), color);
    }

    let mut clicked = false;
    let mut token_out: Option<String> = None;
    let mut reset = false;
    ui.allocate_ui_at_rect(rect.shrink2(egui::vec2(10.0, 1.0)), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            let show_icon = row.icon.is_some() && skill_texture.is_some();
            let icon_w = if show_icon { 26.0 } else { 0.0 };
            let name_w = ((full_w * 0.45) - icon_w).min((full_w - 300.0).max(140.0));
            if let (true, Some(texture), Some(icon)) = (show_icon, skill_texture, row.icon) {
                let (icon_rect, _) =
                    ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
                ui.painter().image(
                    texture.id(),
                    icon_rect,
                    skills.icon_uv(icon),
                    egui::Color32::WHITE,
                );
            }
            let mut name = egui::RichText::new(&row.option.name)
                .color(if row.pending { p.pending } else { p.text })
                .size(12.5);
            if row.modified {
                name = name.strong();
            }
            let mut tooltip = format!("{}  ·  {}", i18n::tr(&row.option.category), row.option.kind);
            if row.pending {
                tooltip.push_str(&format!("  ·  {}", i18n::tr("not applied yet")));
            } else if row.modified {
                tooltip.push_str(&format!("  ·  {}", i18n::tr("changed from default")));
            }
            if row.uses_global {
                tooltip.push_str(&format!("  ·  {}", i18n::tr("uses the global value")));
            } else if !row.present {
                tooltip.push_str(&format!("  ·  {}", i18n::tr("not in the file yet")));
            }
            if !row.skill_name.is_empty() {
                tooltip.push_str(&format!("\n{}", i18n::tr_args("Skill: {0}", &[&row.skill_name])));
            }
            if !row.comment.is_empty() {
                tooltip.push_str(&format!("\n{}", row.comment));
            }
            // A child UI for the label keeps the text left-aligned (add_sized centers it), and
            // an explicit interaction over the whole name cell makes the empty part clickable.
            let name_rect = egui::Rect::from_min_size(
                ui.cursor().min,
                egui::vec2(name_w, 20.0),
            );
            let mut name_ui = ui.child_ui(
                name_rect,
                egui::Layout::left_to_right(egui::Align::Center),
                None,
            );
            name_ui.add(egui::Label::new(name).truncate());
            ui.advance_cursor_after_rect(name_rect);
            let response = ui.interact(
                name_rect,
                egui::Id::new(("option-name", &row.option.name)),
                egui::Sense::click(),
            );
            if response.on_hover_text(tooltip).clicked() {
                clicked = true;
            }

            ui.allocate_ui_with_layout(
                egui::vec2(190.0, 20.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    match row.option.kind.as_str() {
                "bool" => {
                    let mut on = bool_of(&row.current);
                    let before = on;
                    egui::ComboBox::from_id_source(format!("bool:{}", row.option.name))
                        .width(82.0)
                        .selected_text(if on { i18n::tr("True") } else { i18n::tr("False") })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut on, true, i18n::tr("True"));
                            ui.selectable_value(&mut on, false, i18n::tr("False"));
                        });
                    if on != before {
                        token_out = Some(if on { "1" } else { "0" }.to_string());
                    }
                }
                "serverType" => {
                    let mut chosen = if row.current.trim() == "SERVER_PRIVATE" { 1 } else { 0 };
                    let before = chosen;
                    egui::ComboBox::from_id_source(format!("server:{}", row.option.name))
                        .width(160.0)
                        .selected_text(if chosen == 1 { "SERVER_PRIVATE" } else { "SERVER_OFFICIAL" })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut chosen, 0, "SERVER_OFFICIAL");
                            ui.selectable_value(&mut chosen, 1, "SERVER_PRIVATE");
                        });
                    if chosen != before {
                        token_out = Some(
                            if chosen == 1 { "SERVER_PRIVATE" } else { "SERVER_OFFICIAL" }.to_string(),
                        );
                    }
                }
                "enum" => {
                    let selected_label = row
                        .current
                        .trim()
                        .parse::<i64>()
                        .ok()
                        .and_then(|n| row.values.iter().find(|(_, v)| *v == n).map(|(l, _)| l.clone()))
                        .unwrap_or_else(|| row.current.clone());
                    let mut chosen = selected_label.clone();
                    egui::ComboBox::from_id_source(format!("enum:{}", row.option.name))
                        .width(170.0)
                        .selected_text(selected_label)
                        .show_ui(ui, |ui| {
                            for (label, value) in &row.values {
                                if ui.selectable_value(&mut chosen, label.clone(), label).clicked() {
                                    token_out = Some(value.to_string());
                                }
                            }
                        });
                }
                _ => {
                    let width = if row.option.kind == "double" { 86.0 } else { 76.0 };
                    let mut text = drafts
                        .get(&row.option.name)
                        .cloned()
                        .unwrap_or_else(|| row.current.clone());
                    let response = ui.add_sized(
                        [width, 20.0],
                        egui::TextEdit::singleline(&mut text)
                            .id_source(format!("num:{}", row.option.name)),
                    );
                    if response.changed() {
                        // keep the raw text even while it is not a number yet ("-", "1."), or the
                        // next frame would restore the old value and negative numbers would be
                        // impossible to type
                        drafts.insert(row.option.name.clone(), text.clone());
                        if text.trim().parse::<f64>().is_ok() {
                            token_out = Some(text.clone());
                        }
                    }
                    if response.lost_focus() {
                        drafts.remove(&row.option.name);
                        // the web's number input clamps on change; do it when the field is done,
                        // not on every keystroke
                        match text.trim().parse::<f64>() {
                            Ok(value) => {
                                let clamped = value
                                    .max(row.option.min.unwrap_or(f64::MIN))
                                    .min(row.option.max.unwrap_or(f64::MAX));
                                token_out = Some(if row.option.kind == "double" {
                                    format!("{clamped}")
                                } else {
                                    format!("{}", clamped as i64)
                                });
                            }
                            Err(_) => token_out = Some(row.current.clone()),
                        }
                    }
                }
                    }
                },
            );

            if row.modified {
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("reset").size(11.0))
                            .frame(false)
                            .fill(egui::Color32::TRANSPARENT),
                    )
                    .on_hover_text(format!("Reset to default ({})", row.default_label))
                    .clicked()
                {
                    reset = true;
                }
            }
        });
    });
    (clicked, token_out, reset)
}

// A window that flashes and closes leaves nothing behind; the log does.
fn log(message: &str) {
    let dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let path = dir.join("azzy-native.log");
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(file, "{stamp} {message}");
    }
    eprintln!("azzy-native: {message}");
}

fn log_tail() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    let path = exe.parent()?.join("azzy-native.log");
    let text = std::fs::read_to_string(path).ok()?;
    let bytes = text.as_bytes();
    let start = bytes.len().saturating_sub(20_000);
    Some(String::from_utf8_lossy(&bytes[start..]).to_string())
}

fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let location = panic.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
        let message = if let Some(text) = panic.payload().downcast_ref::<&str>() {
            (*text).to_string()
        } else if let Some(text) = panic.payload().downcast_ref::<String>() {
            text.clone()
        } else {
            "unknown panic".into()
        };
        let backtrace = std::backtrace::Backtrace::force_capture();
        log(&format!("PANIC at {location}: {message}\n{backtrace}"));
        previous(panic);
    }));
}

// The web GUI uses Tahoma/Verdana/Segoe UI; on Windows use the real Segoe UI so the app does not
// look like a Linux demo. Missing file (or another OS) just keeps egui's default font.
fn install_fonts(ctx: &egui::Context) {
    let candidates = [r"C:\Windows\Fonts\segoeui.ttf", r"C:\Windows\Fonts\segoeuib.ttf"];
    let mut fonts = egui::FontDefinitions::default();
    let mut loaded = false;
    for path in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let name = format!("segoe:{}", fonts.font_data.len());
            fonts
                .font_data
                .insert(name.clone(), egui::FontData::from_owned(bytes));
            // at the front: egui takes glyphs from the first family entry that has them
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, name);
            loaded = true;
        }
    }
    if loaded {
        ctx.set_fonts(fonts);
    }
}

// Finds USER_AI/<name> without being told, so a double-click works: the working directory, the
// exe's folder, and up to three levels above it (tools/azzy-native -> repository root).
/// `CVersion` of the USER_AI being edited: the release number the startup check compares tags
/// with. Only the uncommented assignment at the top of Const_.lua counts.
fn installed_version(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join("Const_.lua")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("CVersion=") {
            let rest = rest.trim().trim_start_matches('"');
            let end = rest.find('"').unwrap_or(rest.len());
            let version = rest[..end].trim();
            if !version.is_empty() {
                return Some(version.to_string());
            }
        }
    }
    None
}

/// Folder the executable lives in. In the shipped layout this is USER_AI itself (the exe sits next
/// to H_Config.lua, like the old AzzyAIConfig.exe), which makes it the natural update target.
fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
}

/// Opens a link in the default browser.
fn open_url(url: &str) -> Result<(), String> {
    #[cfg(windows)]
    let (program, args) = ("cmd", vec!["/C", "start", "", url]);
    #[cfg(target_os = "macos")]
    let (program, args) = ("open", vec![url]);
    #[cfg(all(unix, not(target_os = "macos")))]
    let (program, args) = ("xdg-open", vec![url]);
    std::process::Command::new(program)
        .args(args)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("{url}: {e}"))
}

/// Waits for this process to exit, moves `<exe>.new` over the executable and starts it again,
/// the same way the old Windows app does.
fn restart_with_staged_exe(staged: &Path) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let staged = staged.display().to_string();
    let exe = exe.display().to_string();
    #[cfg(windows)]
    {
        let command = format!(
            "ping 127.0.0.1 -n 3 >nul & move /y \"{staged}\" \"{exe}\" & start \"\" \"{exe}\""
        );
        std::process::Command::new("cmd")
            .args(["/C", &command])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(not(windows))]
    {
        let command = format!("sleep 2; mv -- '{staged}' '{exe}'; exec '{exe}' &");
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn resolve(name: &str) -> Option<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.join("USER_AI"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            // the exe was copied into USER_AI (shipped layout): read the files next to it
            roots.push(dir.to_path_buf());
        }
        let mut dir = exe;
        for _ in 0..4 {
            if !dir.pop() {
                break;
            }
            roots.push(dir.join("USER_AI"));
            roots.push(dir.join("AI").join("USER_AI"));
        }
    }
    roots.into_iter().map(|root| root.join(name)).find(|c| c.is_file())
}

/// Parses the command line into (USER_AI folder, write in place, show help).
///
/// Save writes the files by default: the app ships to players, and a silent sandbox that only
/// writes `<file>.new` looks like the window cannot save. `--new`/`--sandbox` restores it.
fn parse_args<I: Iterator<Item = String>>(args: I) -> (Option<PathBuf>, bool, bool) {
    let mut write_in_place = true;
    let mut dir: Option<PathBuf> = None;
    let mut help = false;
    for arg in args {
        match arg.as_str() {
            "--write" => write_in_place = true,
            "--new" | "--sandbox" => write_in_place = false,
            "-h" | "--help" => help = true,
            other => dir = Some(PathBuf::from(other)),
        }
    }
    (dir, write_in_place, help)
}

fn main() -> eframe::Result<()> {
    install_panic_hook();
    let (dir, write_in_place, help) = parse_args(std::env::args().skip(1));
    if help {
        print!("{}", include_str!("help.txt"));
        return Ok(());
    }
    let base = dir.clone();
    let mut paths: BTreeMap<&'static str, PathBuf> = BTreeMap::new();
    for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
        let candidate = match &base {
            Some(base) => base.join(file),
            None => match resolve(file) {
                Some(path) => path,
                None => {
                    log(&format!(
                        "{file} not found: pass the USER_AI folder, e.g. azzy-native.exe C:\\\\git\\\\AzzyAI\\\\USER_AI"
                    ));
                    std::process::exit(2);
                }
            },
        };
        if !candidate.is_file() {
            log(&format!("{} not found (pass the USER_AI folder)", candidate.display()));
            std::process::exit(2);
        }
        paths.insert(key, candidate);
    }
    log(&format!("opening {} and {}", paths["H"].display(), paths["M"].display()));
    let app = match App::new(&paths, write_in_place) {
        Ok(app) => app,
        Err(e) => {
            log(&format!("cannot start: {e}"));
            std::process::exit(1);
        }
    };
    let icon = egui::viewport::IconData {
        rgba: include_bytes!("../data/azzyai-48.rgba").to_vec(),
        width: 48,
        height: 48,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 800.0])
            .with_min_inner_size([920.0, 600.0])
            .with_title("AzzyAI Configuration")
            .with_icon(std::sync::Arc::new(icon)),
        ..Default::default()
    };
    let result = eframe::run_native(
        "AzzyAI Configuration",
        options,
        Box::new(move |cc| {
            install_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    match &result {
        Ok(()) => log("window closed normally"),
        Err(e) => log(&format!("could not open the window: {e}")),
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_description_card_lays_its_text_out_vertically() {
        // The card sits next to the grid, inside a horizontal row; a Frame inherits that layout.
        // Without the explicit vertical layout the name/facts/text were drawn side by side and the
        // description never got any width (it was clipped out of the card).
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        i18n::set_lang(i18n::Lang::En);
        let mut app = App::new(&paths, false).unwrap();
        app.selected = Some("AggroHP".into());
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| app.draw(ctx));
        let mut facts_y = None;
        let mut description = None;
        for clipped in &output.shapes {
            if let egui::Shape::Text(text) = &clipped.shape {
                let content = text.galley.text();
                if content.contains("0..100") {
                    facts_y = Some(text.pos.y);
                }
                if content.contains("seek out and attack monsters") {
                    description = Some((text.pos, text.galley.size()));
                }
            }
        }
        let facts_y = facts_y.expect("facts line");
        let (pos, size) = description.expect("description text was not drawn");
        assert!(pos.y > facts_y, "description overlaps the facts line");
        assert!(size.x <= 380.0, "description is wider than the card: {size:?}");
        assert!(size.y > 20.0, "description was not wrapped into lines: {size:?}");
    }

    #[test]
    fn the_description_card_shows_the_skill_card() {
        // Like the web GUI: a setting that belongs to a skill gets the official name, the numbers
        // at max level and the in-game description (which lists the per-level values).
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        i18n::set_lang(i18n::Lang::En);
        let mut app = App::new(&paths, false).unwrap();
        app.selected = Some("AmiBulwarkLevel".into());
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| app.draw(ctx));
        let mut texts = Vec::new();
        for clipped in &output.shapes {
            if let egui::Shape::Text(text) = &clipped.shape {
                texts.push((text.pos, text.galley.text().to_string()));
            }
        }
        let find = |needle: &str| texts.iter().find(|(_, text)| text.contains(needle));
        let title = find("Amistr Bulwark").expect("skill title");
        let stats = find("Max level 5").expect("skill stats");
        assert!(stats.0.y > title.0.y, "stats must be under the skill name");
        // Like the web GUI's <details>, the in-game text starts collapsed...
        let summary = find("In-game description").expect("in-game summary");
        assert!(summary.0.y > stats.0.y, "summary must be under the stats");
        assert!(find("Max Lv").is_none(), "in-game text must start collapsed");
        // ...and the setting's own description still comes after the skill card.
        let setting = find("Set this value to the level of Amistr Bulwark").expect("setting description");
        assert!(setting.0.y > summary.0.y, "setting description must follow the skill card");
        i18n::set_lang(i18n::Lang::En);
    }

    #[test]
    fn the_legend_stays_above_the_footer() {
        // The footer band is 44 px tall; if the panels do not reserve that (or a card grows past the
        // row height), the legend is painted under the footer and reads clipped.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        i18n::set_lang(i18n::Lang::En);
        let mut app = App::new(&paths, false).unwrap();
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| app.draw(ctx));
        let mut legend = None;
        let mut quit = None;
        for clipped in &output.shapes {
            if let egui::Shape::Text(text) = &clipped.shape {
                let content = text.galley.text();
                if content.contains("changed from default") {
                    legend = Some(text.pos.y + text.galley.size().y);
                }
                if content == "Quit" {
                    quit = Some(text.pos.y);
                }
            }
        }
        let legend_bottom = legend.expect("legend line");
        let quit_top = quit.expect("footer button");
        assert!(
            legend_bottom < quit_top,
            "the legend ({legend_bottom}) overlaps the footer ({quit_top})"
        );
    }

    #[test]
    fn the_update_window_draws_in_every_state() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.update_open = true;
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            ..Default::default()
        };
        let report = azzy_update::Report {
            updated: 2,
            added: 1,
            kept: 3,
            staged: vec!["AzzyAIConfig.exe".into()],
            backup: Some("C:\\temp\\backup".into()),
        };
        let staged = UpdateOutcome {
            report,
            self_update: SelfUpdate::Staged(PathBuf::from("C:\\git\\AzzyAI\\tools\\azzy-native\\azzy-native.exe.new")),
            self_error: None,
        };
        let unchanged = UpdateOutcome {
            report: azzy_update::Report {
                updated: 0,
                added: 0,
                kept: 0,
                staged: Vec::new(),
                backup: None,
            },
            self_update: SelfUpdate::NotInArchive,
            self_error: None,
        };
        for state in [
            UpdateState::Idle,
            UpdateState::Downloading,
            UpdateState::Applying,
            UpdateState::Done(staged),
            UpdateState::Done(unchanged),
            UpdateState::Failed("HTTP 404".into()),
        ] {
            app.update_state = state;
            let _ = ctx.run(input.clone(), |ctx| app.draw(ctx));
        }
    }

    #[test]
    fn a_newer_tag_sets_the_banner() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send((Some("2.0.1".to_string()), Ok(Some("v2.0.3".to_string()))))
            .unwrap();
        app.version_rx = Some(rx);
        app.poll_version();
        assert_eq!(app.new_version.as_deref(), Some("v2.0.3"));

        app.new_version = None;
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send((Some("2.0.3".to_string()), Ok(Some("v2.0.3".to_string()))))
            .unwrap();
        app.version_rx = Some(rx);
        app.poll_version();
        assert!(app.new_version.is_none(), "same version must not show the banner");
    }

    #[test]
    fn reads_the_release_version_from_const() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        if !root.join("Const_.lua").is_file() {
            return;
        }
        let version = installed_version(&root).expect("CVersion");
        assert_eq!(version.split('.').count(), 3, "{version}");
    }

    #[test]
    fn the_settings_for_type_filters_the_grid() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        // a Homunculus S shows its own settings plus every base homunculus's
        app.type_selected = Some("Amistr");
        assert!(app.type_visible("AmiBulwarkLevel"));
        assert!(!app.type_visible("UseLifEscape"));
        assert!(app.type_visible("AggroHP"));
        // a base homunculus shows only its own and hides every S
        app.type_selected = Some("Lif");
        assert!(app.type_visible("UseLifEscape"));
        assert!(app.type_visible("AggroHP"));
        // Global shows everything
        app.type_selected = None;
        assert!(app.type_visible("AmiBulwarkLevel"));
    }

    #[test]
    fn values_compare_like_the_web_gui() {
        assert!(same_value("int", "60", "60.0"));
        assert!(same_value("double", "0.5", "0.50"));
        assert!(!same_value("int", "60", "61"));
        assert!(same_value("bool", "1", "true"));
        assert!(same_value("bool", "0", "false"));
        assert!(!same_value("bool", "1", "0"));
        assert!(same_value("serverType", "SERVER_OFFICIAL", "SERVER_OFFICIAL"));
        assert!(!same_value("serverType", "SERVER_OFFICIAL", "SERVER_PRIVATE"));
    }

    #[test]
    fn a_selected_skill_filters_the_settings() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = repo.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.skill_filter = Some(8006); // Amistr Bulwark
        assert!(app.is_searching());
        let bulwark = app.hom.get("AmiBulwarkLevel").expect("Amistr Bulwark setting");
        assert!(app.matches(bulwark, None));
        let aggro = app.hom.get("AggroHP").expect("AggroHP");
        assert!(!app.matches(aggro, None), "AggroHP does not belong to Amistr Bulwark");
        app.skill_filter = None;
        assert!(app.matches(aggro, None));
    }

    #[test]
    fn clicking_a_tactic_row_selects_it() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = repo.join(file);
            if !path.is_file() {
                return; // the submodule is not checked out
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.page = Page::Tactics(tactics_ui::Tab::Hom);
        let ctx = egui::Context::default();
        let input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        // layout twice, then find the row's real rectangle (not a guessed coordinate)
        let _ = ctx.run(input(Vec::new()), |ctx| app.draw(ctx));
        let _ = ctx.run(input(Vec::new()), |ctx| app.draw(ctx));
        let index = app.tactics["homtact"]
            .entries
            .iter()
            .position(|entry| entry.id == "1005")
            .expect("Familiar");
        let id = egui::Id::new(("tact-row", "homtact", index));
        for i in 0..4 {
            let r = ctx.data(|data| data.get_temp::<egui::Rect>(egui::Id::new(("tact-row", "homtact", i))));
            eprintln!("row {i}: {r:?}");
        }
        let rect = ctx
            .data(|data| data.get_temp::<egui::Rect>(id))
            .expect("the tactics list must expose the row rect");
        let pos = rect.center();
        // two frames: egui computes hover from the pointer position settled at the end of the
        // previous frame, so the first frame with PointerMoved still reports hovered=false
        let _ = ctx.run(input(vec![egui::Event::PointerMoved(pos)]), |ctx| app.draw(ctx));
        let _ = ctx.run(input(vec![egui::Event::PointerMoved(pos)]), |ctx| app.draw(ctx));
        let _ = ctx.run(
            input(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: Default::default(),
                },
            ]),
            |ctx| app.draw(ctx),
        );

        let _ = ctx.run(
            input(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: Default::default(),
                },
            ]),
            |ctx| app.draw(ctx),
        );

        assert_eq!(
            app.tact_selected["homtact"], index,
            "clicking the center of a row must select it"
        );
        let scroll = ctx.data(|data| {
            data.get_temp::<(f32, f32, f32)>(egui::Id::new(("tact-scroll", "homtact")))
        });
        assert!(scroll.map(|(content, viewport, _)| content > viewport).unwrap_or(false));
    }

    #[test]
    fn the_tactic_list_scrolls_with_the_wheel() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = repo.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.page = Page::Tactics(tactics_ui::Tab::Hom);
        let ctx = egui::Context::default();
        let input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        for _ in 0..2 {
            let _ = ctx.run(input(Vec::new()), |ctx| app.draw(ctx));
        }
        let pos = egui::pos2(190.0, 300.0);
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: egui::vec2(0.0, -5.0),
            modifiers: Default::default(),
        };
        for _ in 0..3 {
            let _ = ctx.run(
                input(vec![egui::Event::PointerMoved(pos), wheel.clone()]),
                |ctx| app.draw(ctx),
            );
        }
        let scroll = ctx.data(|data| {
            data.get_temp::<(f32, f32, f32)>(egui::Id::new(("tact-scroll", "homtact")))
        });
        assert!(
            scroll.map(|(_, _, offset)| offset > 0.0).unwrap_or(false),
            "the wheel must scroll the tactics list"
        );
    }

    #[test]
    fn arrow_keys_navigate_the_tactic_list() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = repo.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.page = Page::Tactics(tactics_ui::Tab::Hom);
        let ctx = egui::Context::default();
        let input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        for _ in 0..2 {
            let _ = ctx.run(input(Vec::new()), |ctx| app.draw(ctx));
        }
        let press = |key| {
            input(vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            }])
        };
        let _ = ctx.run(press(egui::Key::ArrowDown), |ctx| app.draw(ctx));
        assert_eq!(app.tact_selected["homtact"], 1, "down moves one row");
        let _ = ctx.run(press(egui::Key::ArrowUp), |ctx| app.draw(ctx));
        assert_eq!(app.tact_selected["homtact"], 0, "up moves back");
        let _ = ctx.run(press(egui::Key::End), |ctx| app.draw(ctx));
        let last = app.tactics["homtact"].entries.len() - 1;
        assert_eq!(app.tact_selected["homtact"], last, "End goes to the last row");
        let _ = ctx.run(press(egui::Key::Home), |ctx| app.draw(ctx));
        assert_eq!(app.tact_selected["homtact"], 0, "Home goes to the first row");
    }

    #[test]
    fn adding_suggestions_does_not_break_the_list_scroll() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = repo.join(file);
            if !path.is_file() {
                return;
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        app.page = Page::Tactics(tactics_ui::Tab::Hom);
        {
            let tactics = app.tactics.get_mut("homtact").unwrap();
            for id in 1000..1030 {
                tactics.add_with_values(
                    id.to_string(),
                    format!("Monster {id}"),
                    azzy_config::tactics::TACT_DEFAULTS
                        .iter()
                        .map(|value| value.to_string())
                        .collect(),
                );
            }
            app.tact_selected.insert("homtact", tactics.entries.len() - 1);
        }
        app.suggest.open = true;
        let ctx = egui::Context::default();
        let input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        for _ in 0..2 {
            let _ = ctx.run(input(Vec::new()), |ctx| app.draw(ctx));
        }
        let pos = egui::pos2(190.0, 300.0);
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: egui::vec2(0.0, -5.0),
            modifiers: Default::default(),
        };
        for _ in 0..3 {
            let _ = ctx.run(
                input(vec![egui::Event::PointerMoved(pos), wheel.clone()]),
                |ctx| app.draw(ctx),
            );
        }
        let scroll = ctx.data(|data| {
            data.get_temp::<(f32, f32, f32)>(egui::Id::new(("tact-scroll", "homtact")))
        });
        assert!(
            scroll.map(|(_, _, offset)| offset > 0.0).unwrap_or(false),
            "the wheel must scroll the list after suggestions were added"
        );
    }

    #[test]
    fn type_presets_override_only_their_file() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        if !repo.join("Types").join("Amistr.lua").is_file() {
            return;
        }
        let dir = std::env::temp_dir().join(format!("azzy-native-types-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Types")).unwrap();
        for file in ["H_Config.lua", "M_Config.lua"] {
            std::fs::copy(repo.join(file), dir.join(file)).unwrap();
        }
        for entry in std::fs::read_dir(repo.join("Types")).unwrap() {
            let entry = entry.unwrap();
            std::fs::copy(entry.path(), dir.join("Types").join(entry.file_name())).unwrap();
        }
        let mut paths = BTreeMap::new();
        paths.insert("H", dir.join("H_Config.lua"));
        paths.insert("M", dir.join("M_Config.lua"));
        let before_h = std::fs::read_to_string(dir.join("H_Config.lua")).unwrap();
        let mut app = App::new(&paths, true).unwrap();
        app.unit = Unit::Homunculus;
        app.type_selected = Some("Amistr");
        let global_aggro = app.config().and_then(|config| config.get("AggroHP")).unwrap_or("").to_string();

        app.set_token("AggroHP", "77".to_string());
        assert_eq!(app.type_configs["Amistr"].get("AggroHP"), Some("77"));
        assert_eq!(app.current_value("AggroHP"), Some("77"));

        // back to the global value -> the override line disappears
        app.set_token("AggroHP", global_aggro);
        assert_eq!(app.type_configs["Amistr"].get("AggroHP"), None);

        // an existing override is changed in place
        app.set_token("AutoMobCount", "3".to_string());
        assert_eq!(app.type_configs["Amistr"].get("AutoMobCount"), Some("3"));
        assert!(app.type_configs["Amistr"].is_modified());
        app.save();
        assert_eq!(std::fs::read_to_string(dir.join("H_Config.lua")).unwrap(), before_h);
        let amistr = std::fs::read_to_string(dir.join("Types").join("Amistr.lua")).unwrap();
        assert!(amistr.contains("AutoMobCount             = 3"), "{amistr}");
        assert!(!amistr.contains("AutoMobCount             = 1"), "{amistr}");
        assert_eq!(app.changed(), 0, "a save must clear the type edits");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn export_and_import_round_trip() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        if !repo.join("H_Config.lua").is_file() {
            return;
        }
        let dir = std::env::temp_dir().join(format!("azzy-native-io-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Types")).unwrap();
        for file in ["H_Config.lua", "M_Config.lua", "H_Tactics.lua", "M_Tactics.lua"] {
            if repo.join(file).is_file() {
                std::fs::copy(repo.join(file), dir.join(file)).unwrap();
            }
        }
        if repo.join("Types").is_dir() {
            for entry in std::fs::read_dir(repo.join("Types")).unwrap() {
                let entry = entry.unwrap();
                std::fs::copy(entry.path(), dir.join("Types").join(entry.file_name())).unwrap();
            }
        }
        let mut paths = BTreeMap::new();
        paths.insert("H", dir.join("H_Config.lua"));
        paths.insert("M", dir.join("M_Config.lua"));
        let mut app = App::new(&paths, true).unwrap();
        let export = dir.join("export");
        let count = app.export_all(&export).unwrap();
        assert!(count > 5, "exported {count} files");
        assert!(export.join("H_Config.lua").is_file());
        assert!(export.join("H_Tactics.lua").is_file());
        assert!(export.join("Types").join("Vanilmirth.lua").is_file());

        // change one value in the exported file, then import the folder
        let text = std::fs::read_to_string(export.join("H_Config.lua")).unwrap();
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim_start().starts_with("AggroHP") && line.contains('=') {
                *line = "AggroHP                 = 123".to_string();
                found = true;
            }
        }
        assert!(found, "AggroHP must be in H_Config.lua");
        std::fs::write(
            export.join("H_Config.lua"),
            lines.join("\n") + "\n",
        )
        .unwrap();
        let imported = app.import_folder(&export).unwrap();
        assert!(imported.iter().any(|file| file == "H_Config.lua"));
        assert!(app.dirty_configs.contains("H"));
        assert_eq!(app.configs["H"].get("AggroHP"), Some("123"));
        app.save();
        let after = std::fs::read_to_string(dir.join("H_Config.lua")).unwrap();
        assert!(after.contains("AggroHP                 = 123"), "{after}");
        assert_eq!(app.changed(), 0, "a save must clear imported files");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn applying_tactics_writes_only_that_file() {
        use azzy_config::tactics::BASIC;
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let source = repo.join("H_Tactics.lua");
        if !source.is_file() {
            return; // the submodule is not checked out
        }
        let dir = std::env::temp_dir().join(format!("azzy-native-tactics-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for file in ["H_Config.lua", "M_Config.lua", "H_Tactics.lua"] {
            std::fs::copy(repo.join(file), dir.join(file)).unwrap();
        }
        let mut paths = BTreeMap::new();
        paths.insert("H", dir.join("H_Config.lua"));
        paths.insert("M", dir.join("M_Config.lua"));
        let before_h = std::fs::read_to_string(dir.join("H_Config.lua")).unwrap();
        let before_m = std::fs::read_to_string(dir.join("M_Config.lua")).unwrap();
        let mut app = App::new(&paths, true).unwrap();
        let tactics = app.tactics.get_mut("homtact").unwrap();
        let index = tactics.entries.iter().position(|entry| entry.id == "1005").unwrap();
        tactics.entry_mut(index).unwrap().values[BASIC] = "TACT_TANK".into();
        app.save();
        assert_eq!(std::fs::read_to_string(dir.join("H_Config.lua")).unwrap(), before_h);
        assert_eq!(std::fs::read_to_string(dir.join("M_Config.lua")).unwrap(), before_m);
        let after = std::fs::read_to_string(dir.join("H_Tactics.lua")).unwrap();
        assert!(after.contains("MyTact[1005]={TACT_TANK,"));
        let before = std::fs::read_to_string(source).unwrap();
        assert_eq!(before.lines().count(), after.lines().count());
        assert_eq!(app.changed(), 0, "a save must clear the pending changes");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn draws_without_panicking() {
        // egui lays out for real in a headless context: this catches invalid sizes, duplicated
        // widget ids and the borrow mistakes that only show up while drawing.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../USER_AI");
        let mut paths = BTreeMap::new();
        for (key, file) in [("H", "H_Config.lua"), ("M", "M_Config.lua")] {
            let path = root.join(file);
            if !path.is_file() {
                return; // the submodule is not checked out
            }
            paths.insert(key, path);
        }
        let mut app = App::new(&paths, false).unwrap();
        let ctx = egui::Context::default();
        // a real window size: the default RawInput has no screen rect and egui then lays out
        // into a degenerate one, which is not what the app ever sees
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1180.0, 800.0),
            )),
            ..Default::default()
        };
        for _ in 0..3 {
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
        }
        app.search = "heal".into();
        app.only_changed = true;
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.unit = Unit::Mercenary;
        app.search.clear();
        app.only_changed = false;
        app.category = None;
        app.selected = Some("AutoMobMode".into());
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.type_selected = Some("Amistr");
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.type_selected = None;
        app.skill_filter = Some(8041);
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.skill_filter = None;
        for tab in tactics_ui::Tab::ALL {
            app.page = Page::Tactics(tab);
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
        }
        app.page = Page::Extra;
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.page = Page::Tactics(tactics_ui::Tab::Hom);
        app.io_open = true;
        app.io_export_dir = "C:\\temp\\azzy-export".into();
        app.io_import_dir = "C:\\temp\\azzy-import".into();
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.io_open = false;
        app.suggest.open = true;
        app.suggest.map = "prt_fild08".into();
        app.suggest.mob = "poring".into();
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.suggest.open = false;
        app.new_version = Some("v9.9.9".into());
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.new_version = None;
        app.help_open = true;
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        app.help_open = false;
        assert_eq!(app.changed(), 0, "drawing must not edit the configuration");

        // Every language draws the same screens: a missing translation must not break layout.
        for lang in i18n::Lang::ALL {
            app.lang = lang;
            i18n::set_lang(lang);
            app.page = Page::Settings;
            app.unit = Unit::Homunculus;
            app.selected = Some("AggroHP".into());
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
            app.page = Page::Tactics(tactics_ui::Tab::Hom);
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
            app.page = Page::Extra;
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
        }
        i18n::set_lang(i18n::Lang::En);
    }
}

#[cfg(test)]
mod arg_tests {
    use super::parse_args;
    use std::path::PathBuf;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn save_writes_in_place_by_default() {
        let (dir, write, help) = parse_args(args(&[]).into_iter());
        assert!(dir.is_none());
        assert!(write, "Save must write the files unless --new is passed");
        assert!(!help);
    }

    #[test]
    fn new_and_sandbox_enable_the_sandbox() {
        for flag in ["--new", "--sandbox"] {
            let (_, write, _) = parse_args(args(&[flag]).into_iter());
            assert!(!write, "{flag} must enable the sandbox");
        }
    }

    #[test]
    fn folder_and_help_are_parsed() {
        let (dir, write, help) = parse_args(args(&["C:/x/USER_AI", "--help"]).into_iter());
        assert_eq!(dir, Some(PathBuf::from("C:/x/USER_AI")));
        assert!(write);
        assert!(help);
    }
}
