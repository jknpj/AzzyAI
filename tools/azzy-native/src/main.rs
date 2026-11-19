#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// azzy-native: the AzzyAI configuration GUI in Rust (egui), mirroring the web GUI.
//
// Scope of this milestone (M1): both settings tabs (Homunculus and Mercenary) driven by the same
// schema the web GUI uses (tools/webgui-data.js -> data/options.json), with the option's English
// description, search, category tabs, "only changed from default" filter, typed editors (int,
// double, bool, enum, server type) and Apply/Revert through azzy-config, which preserves the file
// byte for byte. The look (navbar, nav-tabs, cards, footer, row markers) comes from theme.rs, which
// copies webgui/index.html. Tactics/PVP grids, per-type presets, Extra Options, the update flow and
// translations are the next milestones (see README).

mod data;
mod mobs;
mod tactics_ui;
mod theme;

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::PathBuf;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Settings,
    Tactics(tactics_ui::Tab),
    Extra,
}

const EXTRA_FILES: [&str; 2] = ["H_Extra.lua", "M_Extra.lua"];

struct App {
    hom: Schema,
    mer: Schema,
    unit: Unit,
    configs: BTreeMap<&'static str, Config>, // "H" / "M"
    paths: BTreeMap<&'static str, PathBuf>,
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
    tactics: BTreeMap<&'static str, Tactics>,
    tact_paths: BTreeMap<&'static str, PathBuf>,
    tact_selected: BTreeMap<&'static str, usize>,
    tact_filter: String,
    page: Page,
    extra: BTreeMap<&'static str, String>,
    extra_original: BTreeMap<&'static str, String>,
    extra_paths: BTreeMap<&'static str, PathBuf>,
    extra_selected: &'static str,
    logo: Option<egui::TextureHandle>,
}

/// Everything a grid row needs, copied out of the schema/config so the UI can mutate them.
struct RowData {
    option: OptionDef,
    values: Vec<(String, i64)>,
    current: String,
    default_label: String,
    present: bool,
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
        Ok(App {
            hom,
            mer,
            unit: Unit::Homunculus,
            configs,
            paths: paths.clone(),
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
            tactics,
            tact_paths,
            tact_selected,
            tact_filter: String::new(),
            page: Page::Settings,
            extra,
            extra_original,
            extra_paths,
            extra_selected: "H_Extra.lua",
            logo: None,
        })
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

    /// Edits not written to disk yet (the amber marker).
    fn changed(&self) -> usize {
        self.configs.values().map(|c| c.changed()).sum::<usize>()
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
            Page::Settings => return self.shown_path.clone(),
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
        self.drafts.clear();
        self.appended.clear();
        self.status = if errors.is_empty() {
            "reloaded from disk".into()
        } else {
            format!("reload failed: {}", errors.join("; "))
        };
    }

    fn save(&mut self) {
        let mut written = Vec::new();
        let mut errors = Vec::new();
        let mut total = 0;
        let config_keys: Vec<&'static str> = self
            .configs
            .iter()
            .filter(|(_, config)| config.changed() > 0)
            .map(|(key, _)| *key)
            .collect();
        for key in config_keys {
            let source = self.paths[key].clone();
            let target = self.target_for(&source);
            let changed = self.configs[key].changed();
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
        let tactic_tabs: Vec<tactics_ui::Tab> = tactics_ui::Tab::ALL
            .iter()
            .copied()
            .filter(|tab| self.tactics.get(tab.key()).map(|t| t.changed()).unwrap_or(0) > 0)
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
            if !changed {
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
        if !errors.is_empty() {
            self.status = format!("save failed: {}", errors.join("; "));
        } else if written.is_empty() {
            self.status = "nothing to apply".into();
        } else {
            self.status = format!(
                "applied {total} change(s) to {}{}",
                written.join(", "),
                if self.write_in_place { "" } else { " (pass --write to edit the files themselves)" }
            );
        }
    }

    // The category is its own filter: the tab counts must ignore it, or every other category
    // reads "0" as soon as one is selected.
    fn matches(&self, option: &OptionDef, category: Option<&str>) -> bool {
        if let Some(category) = category {
            if option.category != *category {
                return false;
            }
        }
        if !self.search.trim().is_empty() {
            let needle = self.search.to_lowercase();
            let haystack = format!("{} {}", option.name, option.description).to_lowercase();
            if !haystack.contains(&needle) {
                return false;
            }
        }
        if self.only_changed {
            let current = self.config().and_then(|c| c.get(&option.name)).unwrap_or("");
            if current == self.schema().default_token(option) {
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
        if self.appended.contains(name) {
            if let Some(config) = self.config_mut() {
                config.remove(name);
            }
            self.appended.remove(name);
        } else {
            self.set_token(name, token);
        }
        self.status = format!("{name} reset to its default");
    }

    fn is_searching(&self) -> bool {
        !self.search.trim().is_empty() || self.only_changed
    }

    /// Builds the rows for one frame. If `grouped`, every category is shown under its header
    /// (search, "only changed" and the "All" tab); otherwise just the selected category.
    fn build_rows(&self, grouped: bool) -> Vec<Row> {
        let schema = self.schema();
        let config = self.config();
        let make = |option: &OptionDef| -> RowData {
            let present = config.map(|c| c.get(&option.name).is_some()).unwrap_or(false);
            let default_token = schema.default_token(option);
            // missing options read as their default, like the web GUI; otherwise the row would
            // look edited and the editor would be empty
            let current = config
                .and_then(|c| c.get(&option.name))
                .map(|value| value.to_string())
                .unwrap_or_else(|| default_token.clone());
            let modified = !same_value(&option.kind, &current, &default_token);
            let pending = config.map(|c| c.changed_of(&option.name)).unwrap_or(false);
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
                pending,
                modified,
                comment: config
                    .and_then(|c| c.comment_of(&option.name))
                    .unwrap_or("")
                    .trim()
                    .to_string(),
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
        let config = self.config();
        schema
            .options
            .iter()
            .filter(|o| {
                let current = config.and_then(|c| c.get(&o.name)).unwrap_or("");
                !same_value(&o.kind, current, &schema.default_token(o))
            })
            .count()
    }

    // ---------- drawing ----------

    fn navbar(&mut self, ctx: &egui::Context, p: &Palette) {
        egui::TopBottomPanel::top("navbar")
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
                        egui::RichText::new("AzzyAI Configuration")
                            .color(p.title_text)
                            .size(18.0)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let label = match self.mode {
                            theme::Mode::Dark => "Light theme",
                            theme::Mode::Light => "Dark theme",
                        };
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
                                egui::Button::new(egui::RichText::new("Reload").color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .on_hover_text("Discard the edits and read the files again (Ctrl+R)")
                            .clicked()
                        {
                            self.reload();
                        }
                    });
                });
            });
    }

    fn footer(&mut self, ctx: &egui::Context, p: &Palette) {
        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame::none().fill(p.bg1))
            .show(ctx, |ui| {
                theme::band(ui, 44.0, p.tab2, p.bg1, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new("Quit").color(p.btn_text))
                                    .fill(p.btn1)
                                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
                            )
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        let changed = self.changed();
                        let apply = egui::Button::new(
                            egui::RichText::new("Apply Settings")
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(p.primary)
                        .stroke(egui::Stroke::new(1.0_f32, p.title_edge))
                        .min_size(egui::vec2(0.0, 24.0));
                        if ui
                            .add_enabled(changed > 0, apply)
                            .on_hover_text(if changed > 0 {
                                "Write the edited files (Ctrl+S)"
                            } else {
                                "No changes to apply"
                            })
                            .clicked()
                        {
                            self.save();
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new("Reload").color(p.btn_text))
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
                                &format!("{changed} not applied"),
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
                    let response = theme::tab(ui, p, unit.label(), active);
                    if active {
                        active_tab = Some(response.rect);
                    }
                    if response.clicked() {
                        let switching = self.unit != unit || self.page != Page::Settings;
                        self.page = Page::Settings;
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
                    let response = theme::tab(ui, p, tab.layout().title, active);
                    if active {
                        active_tab = Some(response.rect);
                    }
                    if response.clicked() {
                        if self.page != Page::Tactics(tab) {
                            self.tact_filter.clear();
                        }
                        self.page = Page::Tactics(tab);
                        self.drafts.clear();
                    }
                }
                let active = self.page == Page::Extra;
                let response = theme::tab(ui, p, "Extra Options", active);
                if active {
                    active_tab = Some(response.rect);
                }
                if response.clicked() {
                    self.page = Page::Extra;
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
        if let Some(tactics) = self.tactics.get_mut(key) {
            tactics_ui::draw(ui, p, tab, tactics, &mut selected, &mut self.tact_filter, mobs);
        }
        self.tact_selected.insert(key, selected);
    }

    fn extra_workspace(&mut self, ui: &mut egui::Ui, p: &Palette) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Extra options file:").color(p.text));
            for (file, label) in [("H_Extra.lua", "Homunculus"), ("M_Extra.lua", "Mercenary")] {
                let active = self.extra_selected == file;
                if theme::tab(ui, p, label, active).clicked() {
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
                theme::badge(ui, "not applied", p.pending, egui::Color32::BLACK);
            }
        });
        ui.label(
            egui::RichText::new(
                "Here you can modify configurations that are not supported by the GUI.",
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
            self.description_card(ui, p, grid_h);
        });
        ui.add_space(4.0);
        self.legend(ui, p, grouped);
    }

    fn toolbar(&mut self, ui: &mut egui::Ui, p: &Palette) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Search settings:").color(p.text));
            let reserve = 350.0;
            let width = (ui.available_width() - reserve).max(160.0);
            let response = ui.add_sized(
                [width, 22.0],
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Name or description (Ctrl+F)")
                    .id_source("search"),
            );
            if std::mem::take(&mut self.focus_search) {
                response.request_focus();
            }
            ui.checkbox(&mut self.only_changed, "Only changed from default");
            if ui.button("Clear filters").clicked() {
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
            if theme::tab(ui, p, &format!("All ({total})"), !searching && self.category.is_none())
                .clicked()
            {
                selection = Some(None);
            }
            for (category, count) in categories {
                let active = !searching && self.category.as_deref() == Some(category.as_str());
                if theme::tab(ui, p, &format!("{category} ({count})"), active).clicked() {
                    selection = Some(Some(category));
                }
            }
        });
        if let Some(selection) = selection {
            // clicking a category while a search is active means "show me that category"
            if searching {
                self.search.clear();
                self.only_changed = false;
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
                Row::Group { name, count } => group_header(ui, p, &name, count),
                Row::Option(data) => {
                    let is_selected = selected.as_deref() == Some(data.option.name.as_str());
                    let (select, edit, do_reset) =
                        option_row(ui, p, &data, is_selected, &mut self.drafts);
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
                ui.set_min_size(egui::vec2(ui.available_width(), (height - 12.0).max(100.0)));
                theme::title_bar(p, ui, "Description");
                ui.add_space(6.0);
                let Some(option) = self.selected.as_deref().and_then(|name| self.schema().get(name)) else {
                    ui.label(
                        egui::RichText::new("Click a setting to read its description.")
                            .color(p.muted)
                            .size(12.0),
                    );
                    return;
                };
                ui.label(egui::RichText::new(&option.name).color(p.text).strong().size(15.0));
                let mut facts = format!("{}  ·  {}", option.category, option.kind);
                if let (Some(min), Some(max)) = (option.min, option.max) {
                    facts.push_str(&format!("  ·  {min}..{max}"));
                }
                facts.push_str(&format!(
                    "  ·  default {}",
                    default_label(self.schema(), option)
                ));
                ui.label(egui::RichText::new(facts).color(p.muted).size(11.0));
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&option.description)
                                .color(p.text)
                                .size(12.5),
                        );
                        if let Some(comment) = self.config().and_then(|c| c.comment_of(&option.name)) {
                            let comment = comment.trim();
                            if !comment.is_empty() {
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new("In the file:")
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
                egui::RichText::new(format!(
                    "{shown} settings  ·  {modified} changed from default"
                ))
                .color(p.muted)
                .size(11.0),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("Not applied yet").color(p.muted).size(11.0));
                theme::swatch(ui, p.pending);
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Changed from default")
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

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) && self.changed() > 0 {
            self.save();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::F)) {
            self.focus_search = true;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::R)) {
            self.reload();
        }

        self.navbar(ctx, &p);
        self.footer(ctx, &p);
        self.body(ctx, &p);
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
            let name_w = (full_w * 0.45).min((full_w - 300.0).max(140.0));
            let mut name = egui::RichText::new(&row.option.name)
                .color(if row.pending { p.pending } else { p.text })
                .size(12.5);
            if row.modified {
                name = name.strong();
            }
            let mut tooltip = format!("{}  ·  {}", row.option.category, row.option.kind);
            if row.pending {
                tooltip.push_str("  ·  not applied yet");
            } else if row.modified {
                tooltip.push_str("  ·  changed from default");
            }
            if !row.present {
                tooltip.push_str("  ·  not in the file yet");
            }
            if !row.comment.is_empty() {
                tooltip.push_str(&format!("\n{}", row.comment));
            }
            let response = ui.add_sized(
                [name_w, 20.0],
                egui::Label::new(name).truncate().sense(egui::Sense::click()),
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
                        .selected_text(if on { "True" } else { "False" })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut on, true, "True");
                            ui.selectable_value(&mut on, false, "False");
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
fn resolve(name: &str) -> Option<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.join("USER_AI"));
    }
    if let Ok(exe) = std::env::current_exe() {
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

fn main() -> eframe::Result<()> {
    install_panic_hook();
    let mut write_in_place = false;
    let mut dir: Option<PathBuf> = None;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--write" => write_in_place = true,
            "-h" | "--help" => {
                print!("{}", include_str!("help.txt"));
                return Ok(());
            }
            other => dir = Some(PathBuf::from(other)),
        }
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
        for tab in tactics_ui::Tab::ALL {
            app.page = Page::Tactics(tab);
            let _ = ctx.run(input(), |ctx| app.draw(ctx));
        }
        app.page = Page::Extra;
        let _ = ctx.run(input(), |ctx| app.draw(ctx));
        assert_eq!(app.changed(), 0, "drawing must not edit the configuration");
    }
}
