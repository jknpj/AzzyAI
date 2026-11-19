// azzy-native: the AzzyAI configuration GUI in Rust (egui), mirroring the web GUI.
//
// Scope of this milestone (M1): both settings tabs (Homunculus and Mercenary) driven by the same
// schema the web GUI uses (tools/webgui-data.js -> data/options.json), with the option's English
// description, search, category filter, "only non-default" filter, typed editors (int, double,
// bool, enum, server type) and Apply/Revert through azzy-config, which preserves the file byte for
// byte. Tactics/PVP grids, per-type presets, Extra Options, the update flow and translations are
// the next milestones (see README).

mod data;

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::PathBuf;

use azzy_config::Config;
use data::{OptionDef, Schema};
use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Unit {
    Homunculus,
    Mercenary,
}

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
}

impl App {
    fn new(paths: &BTreeMap<&'static str, PathBuf>, write_in_place: bool) -> Result<App, String> {
        let (hom, mer) = data::load()?;
        let mut configs = BTreeMap::new();
        let mut shown = Vec::new();
        for (key, path) in paths {
            let config = Config::load(path).map_err(|e| format!("{}: {e}", path.display()))?;
            let canonical = std::fs::canonicalize(path).map(|p| p.display().to_string())
                .unwrap_or_else(|_| path.display().to_string());
            shown.push(canonical.strip_prefix(r"\\?\").unwrap_or(&canonical).to_string());
            configs.insert(*key, config);
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
        })
    }

    fn schema(&self) -> &Schema {
        match self.unit {
            Unit::Homunculus => &self.hom,
            Unit::Mercenary => &self.mer,
        }
    }

    fn unit_key(&self) -> &'static str {
        match self.unit {
            Unit::Homunculus => "H",
            Unit::Mercenary => "M",
        }
    }

    fn config(&self) -> Option<&Config> {
        self.configs.get(self.unit_key())
    }

    fn config_mut(&mut self) -> Option<&mut Config> {
        self.configs.get_mut(self.unit_key())
    }

    fn changed(&self) -> usize {
        self.configs.values().map(|c| c.changed()).sum()
    }

    fn reload(&mut self) {
        let key = self.unit_key();
        let path = self.paths[key].clone();
        match Config::load(&path) {
            Ok(config) => {
                self.configs.insert(key, config);
                self.status = "reloaded from disk".into();
            }
            Err(e) => self.status = format!("reload failed: {e}"),
        }
    }

    fn save(&mut self) {
        let key = self.unit_key();
        let source = self.paths[key].clone();
        let target = if self.write_in_place {
            source.clone()
        } else {
            PathBuf::from(format!("{}.new", source.display()))
        };
        let Some(config) = self.config() else { return };
        let changed = config.changed();
        if let Err(e) = config.save(&target) {
            self.status = format!("save failed: {e}");
            return;
        }
        // reload so the "changed" counter starts from what is now on disk
        if let Ok(fresh) = Config::load(&target) {
            self.configs.insert(key, fresh);
        }
        self.status = format!("wrote {} ({changed} changed){}", target.display(),
            if self.write_in_place { "" } else { " - pass --write to edit the file itself" });
    }

    // The category is its own filter: the sidebar counts must ignore it, or every other category
    // reads "(0)" as soon as one is selected (seen in the screenshot).
    fn matches(&self, option: &OptionDef, category: Option<&str>) -> bool {
        if let Some(category) = category {
            if option.category != *category {
                return false;
            }
        }
        if !self.search.is_empty() {
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

    fn visible(&self, option: &OptionDef) -> bool {
        self.matches(option, self.category.as_deref())
    }

    fn set_token(&mut self, name: &str, token: String) {
        if let Some(config) = self.config_mut() {
            config.set(name, &token);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frames += 1;
        if self.frames == 1 || self.frames == 5 {
            log(&format!("frame {}", self.frames));
        }
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("AzzyAI configuration");
                ui.separator();
                for (label, unit) in [("Homunculus", Unit::Homunculus), ("Mercenary", Unit::Mercenary)] {
                    if ui.selectable_label(self.unit == unit, label).clicked() {
                        self.unit = unit;
                        self.category = None;
                        self.selected = None;
                    }
                }
                ui.separator();
                let changed = self.changed();
                ui.colored_label(
                    if changed > 0 { egui::Color32::from_rgb(200, 120, 0) } else { egui::Color32::GRAY },
                    format!("{changed} changed"),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_enabled(changed > 0, egui::Button::new("Save")).clicked() {
                        self.save();
                    }
                    if ui.button("Reload").clicked() {
                        self.reload();
                    }
                });
            });
            // the path needs its own row: on one line it ran under the search box and the counter
            ui.horizontal(|ui| {
                let path_width = (ui.available_width() - 360.0).max(120.0);
                ui.add_sized(
                    [path_width, 18.0],
                    egui::Label::new(egui::RichText::new(&self.shown_path).weak()).truncate(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("name or description")
                            .desired_width(200.0),
                    );
                    ui.checkbox(&mut self.only_changed, "only non-default");
                });
            });
            if !self.status.is_empty() {
                ui.label(&self.status);
            }
        });

        egui::SidePanel::left("categories").resizable(true).default_width(220.0).show(ctx, |ui| {
            ui.heading("Categories");
            // counts first: assigning self.category while schema borrows self is not allowed
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
            if ui.selectable_label(self.category.is_none(), format!("All matching ({total})")).clicked() {
                self.category = None;
            }
            for (category, count) in categories {
                if ui.selectable_label(self.category.as_deref() == Some(category.as_str()),
                    format!("{category} ({count})")).clicked() {
                    self.category = Some(category);
                }
            }
        });

        egui::TopBottomPanel::bottom("description").resizable(true).min_height(90.0).show(ctx, |ui| {
            match self.selected.as_deref().and_then(|name| self.schema().get(name)) {
                Some(option) => {
                    ui.horizontal(|ui| {
                        ui.strong(&option.name);
                        let mut facts = format!("{}  ·  {}", option.category, option.kind);
                        if let (Some(min), Some(max)) = (option.min, option.max) {
                            facts.push_str(&format!("  ·  {min}..{max}"));
                        }
                        facts.push_str(&format!("  ·  default {}", self.schema().default_token(option)));
                        ui.weak(facts);
                    });
                    egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                        ui.label(&option.description);
                    });
                }
                None => { ui.weak("Select an option to read its description."); }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                let mut edits: Vec<(String, String)> = Vec::new();
                let mut selected = None;
                let has_comment = |name: &str| self.config().and_then(|c| c.comment_of(name));
                egui::Grid::new("options").num_columns(3).spacing([10.0, 3.0]).striped(true).show(ui, |ui| {
                    ui.strong("Option");
                    ui.strong("Value");
                    ui.strong("Notes");
                    ui.end_row();
                    // clone the option list so the config can be edited while drawing
                    let schema = self.schema();
                    let options: Vec<OptionDef> = schema.options.iter().filter(|o| self.visible(o)).cloned().collect();
                    for option in &options {
                        let current = self.config().and_then(|c| c.get(&option.name)).map(|v| v.to_string()).unwrap_or_default();
                        let response = ui.selectable_label(
                            self.selected.as_deref() == Some(option.name.as_str()),
                            &option.name,
                        );
                        if response.clicked() {
                            selected = Some(option.name.clone());
                        }
                        let mut token = current.clone();
                        match option.kind.as_str() {
                            "bool" => {
                                let mut on = token.trim() != "0" && !token.is_empty();
                                if ui.checkbox(&mut on, "").changed() {
                                    token = if on { "1".into() } else { "0".into() };
                                    edits.push((option.name.clone(), token.clone()));
                                }
                            }
                            "enum" | "serverType" => {
                                let values: Vec<(String, i64)> = if option.kind == "serverType" {
                                    vec![("SERVER_OFFICIAL".into(), 0), ("SERVER_PRIVATE".into(), 1)]
                                } else {
                                    schema.enum_values(option).cloned().unwrap_or_default()
                                };
                                let selected_label = match option.kind.as_str() {
                                    "serverType" => if token.trim() == "SERVER_PRIVATE" { "SERVER_PRIVATE" } else { "SERVER_OFFICIAL" }.to_string(),
                                    _ => token.parse::<i64>().ok()
                                        .and_then(|n| values.iter().find(|(_, v)| *v == n).map(|(l, _)| l.clone()))
                                        .unwrap_or_else(|| token.clone()),
                                };
                                let mut chosen = selected_label.clone();
                                egui::ComboBox::from_id_source(format!("{}-{}", option.name, option.kind))
                                    .selected_text(chosen.clone())
                                    .show_ui(ui, |ui| {
                                        for (label, value) in &values {
                                            let text = if option.kind == "serverType" { label.clone() } else { format!("{label} ({value})") };
                                            if ui.selectable_value(&mut chosen, label.clone(), text).clicked() {
                                                token = if option.kind == "serverType" {
                                                    label.clone()
                                                } else {
                                                    value.to_string()
                                                };
                                            }
                                        }
                                    });
                                if token != current {
                                    edits.push((option.name.clone(), token.clone()));
                                }
                            }
                            _ => {
                                let width = if option.kind == "double" { 80.0 } else { 96.0 };
                                if ui.add_sized([width, 20.0], egui::TextEdit::singleline(&mut token)).changed() {
                                    if let Ok(value) = token.trim().parse::<f64>() {
                                        // the web GUI clamps to min/max; do the same here
                                        let clamped = value
                                            .max(option.min.unwrap_or(f64::MIN))
                                            .min(option.max.unwrap_or(f64::MAX));
                                        token = if option.kind == "double" {
                                            format!("{clamped}")
                                        } else {
                                            format!("{}", clamped as i64)
                                        };
                                        edits.push((option.name.clone(), token.clone()));
                                    }
                                }
                            }
                        }
                        match has_comment(&option.name) {
                            Some(comment) => { ui.weak(comment.trim()); }
                            None => { ui.label(""); }
                        }
                        ui.end_row();
                    }
                });
                for (name, token) in edits {
                    self.set_token(&name, token);
                }
                if let Some(name) = selected {
                    self.selected = Some(name);
                }
            });
        });
    }
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
                        "{file} not found: pass the USER_AI folder, e.g. azzy-native.exe C:\\git\\AzzyAI\\USER_AI"
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
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 760.0]),
        ..Default::default()
    };
    let result = eframe::run_native("AzzyAI configuration", options, Box::new(|_cc| Ok(Box::new(app))));
    match &result {
        Ok(()) => log("window closed normally"),
        Err(e) => log(&format!("could not open the window: {e}")),
    }
    result
}
