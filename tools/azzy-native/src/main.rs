// azzy-native: prototype of a native configuration GUI for AzzyAI, in Rust with egui.
//
// This is the "full rewrite" path (spike): it proves that the model and the editing loop fit, not
// that the whole 24k-line WinForms app is replaced. Scope here:
//   - loads H_Config.lua (or any file passed on the command line) through azzy-config,
//   - lists the options grouped by their comment sections, with a search box,
//   - edits values in place and shows how many changed,
//   - Save writes the file (or <file>.new unless --write is given),
//   - Reload discards the edits.
// Missing for parity with the C# app: tactics/PVP grids, per-type presets, the updater, the
// diagnostics dialog (which already exists as Rust logic in the Windows app) and translations.

use std::io::Write as _;
use std::path::PathBuf;

use azzy_config::Config;
use eframe::egui;

struct App {
    path: PathBuf,
    shown_path: String,
    write_in_place: bool,
    config: Config,
    sections: Vec<(String, Vec<String>)>,
    section: Option<usize>,
    search: String,
    status: String,
}

impl App {
    fn new(path: PathBuf, write_in_place: bool) -> Result<App, String> {
        let config = Config::load(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let sections = config.sections();
        let shown_path = std::fs::canonicalize(&path)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.display().to_string());
        let shown_path = shown_path.strip_prefix(r"\\?\").unwrap_or(&shown_path).to_string();
        Ok(App {
            path,
            shown_path,
            write_in_place,
            config,
            sections,
            section: None,
            search: String::new(),
            status: String::new(),
        })
    }

    fn reload(&mut self) {
        match Config::load(&self.path) {
            Ok(config) => {
                self.sections = config.sections();
                self.config = config;
                self.status = "reloaded".into();
            }
            Err(e) => self.status = format!("reload failed: {e}"),
        }
    }

    fn save(&mut self) {
        let target = if self.write_in_place {
            self.path.clone()
        } else {
            let mut name = self.path.clone().into_os_string();
            name.push(".new");
            PathBuf::from(name)
        };
        match self.config.save(&target) {
            Ok(()) => {
                self.status = format!("wrote {} ({} changed)", target.display(), self.config.changed());
                self.config = Config::load(&target).unwrap_or_else(|_| self.config.clone());
                if !self.write_in_place {
                    self.status.push_str(" - pass --write to edit the file itself");
                }
            }
            Err(e) => self.status = format!("save failed: {e}"),
        }
    }

    fn visible(&self, name: &str) -> bool {
        if let Some(index) = self.section {
            if !self.sections.get(index).map(|(_, names)| names.iter().any(|n| n == name)).unwrap_or(false) {
                return false;
            }
        }
        if self.search.is_empty() {
            return true;
        }
        let needle = self.search.to_lowercase();
        name.to_lowercase().contains(&needle)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("AzzyAI configuration");
                ui.separator();
                ui.label(&self.shown_path);
                ui.separator();
                let changed = self.config.changed();
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
                    ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("search"));
                });
            });
            if !self.status.is_empty() {
                ui.label(&self.status);
            }
        });

        egui::SidePanel::left("sections").resizable(true).show(ctx, |ui| {
            ui.heading("Sections");
            if ui.selectable_label(self.section.is_none(), format!("All ({})", self.config.option_count())).clicked() {
                self.section = None;
            }
            for (index, (name, names)) in self.sections.iter().enumerate() {
                let label = if name.is_empty() { "(no section)" } else { name.as_str() };
                if ui.selectable_label(self.section == Some(index), format!("{label} ({})", names.len())).clicked() {
                    self.section = Some(index);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                // edits are collected first: the config cannot be borrowed while it is drawn
                let mut edits: Vec<(String, String)> = Vec::new();
                for assignment in self.config.assignments() {
                    if !self.visible(&assignment.name) {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        let width = ui.available_width();
                        let name_w = (width * 0.32).max(120.0);
                        let value_w = (width * 0.22).max(80.0);
                        // a Label inside add_sized is centred; allocate with a left layout instead
                        ui.allocate_ui_with_layout(
                            egui::vec2(name_w, 20.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| { ui.label(&assignment.name); },
                        );
                        let mut value = assignment.value.clone();
                        let response = ui.add_sized([value_w, 20.0], egui::TextEdit::singleline(&mut value));
                        if response.changed() {
                            edits.push((assignment.name.clone(), value));
                        }
                        if assignment.changed() {
                            ui.colored_label(egui::Color32::from_rgb(200, 120, 0), "changed");
                        } else if !assignment.comment.is_empty() {
                            let rest = (width - name_w - value_w - 60.0).max(60.0);
                            ui.allocate_ui_with_layout(
                                egui::vec2(rest, 20.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.add(egui::Label::new(
                                        egui::RichText::new(assignment.comment.trim()).weak()
                                    ).truncate());
                                },
                            );
                        }
                    });
                }
                for (name, value) in edits {
                    self.config.set(&name, &value);
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_and_save_writes_only_the_changed_value() {
        let dir = std::env::temp_dir().join(format!("azzy-native-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("H_Config.lua");
        let original = "LavaSlideMode = 0 -- lava at the target\nAutoMobCount = 2";
        std::fs::write(&path, original).unwrap();

        let mut app = App::new(path.clone(), false).unwrap();
        assert_eq!(app.sections.len(), 1);
        assert!(app.config.set("LavaSlideMode", "1"));
        app.save();

        let written = std::fs::read_to_string(dir.join("H_Config.lua.new")).unwrap();
        assert_eq!(written, "LavaSlideMode = 1 -- lava at the target\nAutoMobCount = 2");
        // the original file is untouched without --write
        assert_eq!(std::fs::read_to_string(&path).unwrap(), original);
        let _ = std::fs::remove_dir_all(&dir);
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

// Launched by double-click the working directory is the exe's folder, so try the layouts the
// repository and the packaged zip use before giving up.
fn resolve(path: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(path) = path {
        return Some(path);
    }
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()))?;
    let candidates = [
        PathBuf::from("USER_AI/H_Config.lua"),
        exe_dir.join("../../USER_AI/H_Config.lua"),
        exe_dir.join("../USER_AI/H_Config.lua"),
        exe_dir.join("USER_AI/H_Config.lua"),
        PathBuf::from("AI/USER_AI/H_Config.lua"),
    ];
    candidates.into_iter().find(|c| c.is_file())
}

fn main() -> eframe::Result<()> {
    let mut path: Option<PathBuf> = None;
    let mut write_in_place = false;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--write" => write_in_place = true,
            "-h" | "--help" => {
                print!("{}", include_str!("help.txt"));
                return Ok(());
            }
            other => path = Some(PathBuf::from(other)),
        }
    }
    let Some(path) = resolve(path) else {
        log("no configuration found: pass the file, e.g. azzy-native.exe C:\\git\\AzzyAI\\USER_AI\\H_Config.lua");
        std::process::exit(2);
    };
    log(&format!("opening {}", path.display()));
    let app = match App::new(path.clone(), write_in_place) {
        Ok(app) => app,
        Err(e) => {
            log(&format!("cannot load {}: {e}", path.display()));
            std::process::exit(1);
        }
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([980.0, 720.0]),
        ..Default::default()
    };
    let result = eframe::run_native("AzzyAI configuration (Rust prototype)", options, Box::new(|_cc| Ok(Box::new(app))));
    match &result {
        Ok(()) => log("window closed normally"),
        Err(e) => log(&format!("could not open the window: {e}")),
    }
    result
}
