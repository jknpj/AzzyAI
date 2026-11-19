//! The Tactics tab: a list of `MyTact[...]` entries on the left, the selected entry on the right,
//! mirroring `renderTact()` in webgui/app.js.
//!
//! Field labels and option tokens come from webgui/tactics.js, so an unknown token is shown as
//! "not supported by the GUI" instead of being silently changed. Edits go straight into the
//! format-preserving `azzy_config::tactics` model; nothing is written until Apply Settings.

use azzy_config::tactics::{
    Kind, Tactics, BASIC, CAST, CHASE, CLASS, DEBUFF, KITE, KS, PUSH, RESCUE, SKILL, SNIPE, SP,
    WEIGHT,
};
use eframe::egui;

use crate::mobs::Mobs;
use crate::theme::{self, Palette};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Hom,
    Mer,
    HomPvp,
    MerPvp,
}

impl Tab {
    pub const ALL: [Tab; 4] = [Tab::Hom, Tab::Mer, Tab::HomPvp, Tab::MerPvp];

    pub fn key(self) -> &'static str {
        match self {
            Tab::Hom => "homtact",
            Tab::Mer => "mertact",
            Tab::HomPvp => "hompvp",
            Tab::MerPvp => "merpvp",
        }
    }

    pub fn layout(self) -> &'static Layout {
        match self {
            Tab::Hom => &HOM,
            Tab::Mer => &MER,
            Tab::HomPvp => &HOM_PVP,
            Tab::MerPvp => &MER_PVP,
        }
    }
}

pub enum FieldKind {
    Select(&'static [(&'static str, &'static str)]),
    Skill,
    Debuff(&'static [(&'static str, Option<&'static str>)]),
    Snipe,
    Sp,
    Weight,
}

pub struct Field {
    pub index: usize,
    pub label: &'static str,
    pub kind: FieldKind,
}

pub struct Layout {
    pub title: &'static str,
    pub file: &'static str,
    pub kind: Kind,
    pub id_label: &'static str,
    pub name_label: &'static str,
    pub warning: Option<&'static str>,
    pub once_level: bool,
    pub fields: &'static [Field],
}

// ---------- options, copied from webgui/tactics.js ----------

static BASIC_OPTIONS: &[(&str, &str)] = &[
    ("Tank", "TACT_TANK"),
    ("Ignore", "TACT_IGNORE"),
    ("Attack (low)", "TACT_ATTACK_L"),
    ("Attack (medium)", "TACT_ATTACK_M"),
    ("Attack (high)", "TACT_ATTACK_H"),
    ("React (low)", "TACT_REACT_L"),
    ("React (medium)", "TACT_REACT_M"),
    ("React (high)", "TACT_REACT_H"),
    ("React (self)", "TACT_REACT_SELF"),
    ("Snipe (low)", "TACT_SNIPE_L"),
    ("Snipe (medium)", "TACT_SNIPE_M"),
    ("Snipe (high)", "TACT_SNIPE_H"),
    ("Attack (low) React (medium)", "TACT_ATK_L_REACT_M"),
    ("Attack (last)", "TACT_ATTACK_LAST"),
    ("Attack (top)", "TACT_ATTACK_TOP"),
];
static KITE_OPTIONS: &[(&str, &str)] = &[
    ("Never", "KITE_NEVER"),
    ("React", "KITE_REACT"),
    ("Always", "KITE_ALWAYS"),
];
static KS_OPTIONS: &[(&str, &str)] = &[
    ("Never", "KS_NEVER"),
    ("Always", "KS_ALWAYS"),
    ("Polite", "KS_POLITE"),
];
static CHASE_OPTIONS: &[(&str, &str)] = &[
    ("Normal", "CHASE_NORMAL"),
    ("Always", "CHASE_ALWAYS"),
    ("Never", "CHASE_NEVER"),
    ("Pause for SP", "CHASE_CLEVER"),
];
static PUSH_OPTIONS: &[(&str, &str)] = &[
    ("Never", "PUSH_NEVER"),
    ("Off self", "PUSH_SELF"),
    ("Off self + friend", "PUSH_FRIEND"),
];
static HOM_CAST: &[(&str, &str)] = &[
    ("No", "CAST_PASSIVE"),
    ("React to casts", "CAST_REACT"),
    ("React w/Any", "CAST_REACT_ANY"),
    ("React w/Breeze", "CAST_REACT_BREEZE"),
    ("React w/Old", "CAST_REACT_MAIN"),
    ("React w/S", "CAST_REACT_S"),
    ("React w/Mob", "CAST_REACT_MOB"),
    ("React w/Debuff", "CAST_REACT_DEBUFF"),
    ("React w/Minions", "CAST_REACT_MINION"),
];
static MER_CAST: &[(&str, &str)] = &[
    ("No", "CAST_PASSIVE"),
    ("React to casts", "CAST_REACT"),
    ("React w/Any", "CAST_REACT_ANY"),
    ("React w/LexDiv", "CAST_REACT_LEXDIV"),
    ("React w/Main", "CAST_REACT_MAIN"),
    ("React w/Mob", "CAST_REACT_MOB"),
    ("React w/Provoke", "CAST_REACT_PROVOKE"),
    ("React w/Sandman", "CAST_REACT_SANDMAN"),
    ("React w/Freeze", "CAST_REACT_FREEZE"),
    ("React w/DecAgi", "CAST_REACT_DECAGI"),
    ("React w/Debuff", "CAST_REACT_DEBUFF"),
    ("React w/Crash", "CAST_REACT_CRASH"),
];
static HOM_CLASS: &[(&str, &str)] = &[
    ("Any skill", "CLASS_BOTH"),
    ("Non-Homun S skills", "CLASS_OLD"),
    ("Homun S skills", "CLASS_S"),
    ("AoE attacks", "CLASS_MOB"),
    ("Combo (first skill)", "CLASS_COMBO_1"),
    ("Combo (full combos)", "CLASS_COMBO_2"),
    ("Summon minions", "CLASS_MINION"),
    ("Grapple (Tinder Breaker)", "CLASS_GRAPPLE"),
    ("Grapple (CBC)", "CLASS_GRAPPLE_1"),
    ("Grapple (EQC)", "CLASS_GRAPPLE_2"),
    ("Minions + Pre-S skill", "CLASS_MIN_OLD"),
    ("Minions + Homun S", "CLASS_MIN_S"),
];
static MER_CLASS: &[(&str, &str)] = &[("All skills", "CLASS_BOTH"), ("Mob skills", "CLASS_MOB")];
static HOM_RESCUE: &[(&str, &str)] = &[
    ("Never", "RESCUE_NEVER"),
    ("Friends", "RESCUE_FRIEND"),
    ("Owner's merc", "RESCUE_RETAINER"),
    ("Self (homunculus)", "RESCUE_SELF"),
    ("Owner", "RESCUE_OWNER"),
    ("All of the above", "RESCUE_ALL"),
];
static MER_RESCUE: &[(&str, &str)] = &[
    ("Never", "RESCUE_NEVER"),
    ("Friends", "RESCUE_FRIEND"),
    ("Owner's homun", "RESCUE_RETAINER"),
    ("Self (mercenary)", "RESCUE_SELF"),
    ("Owner", "RESCUE_OWNER"),
    ("All of the above", "RESCUE_ALL"),
];
static HOM_DEBUFF: &[(&str, Option<&str>)] = &[
    ("Never", None),
    ("Any", Some("ANY")),
    ("Silent Breeze", Some("BREEZE")),
    ("Volcanic Ash", Some("ASH")),
];
static MER_DEBUFF: &[(&str, Option<&str>)] = &[
    ("Never", None),
    ("Any", Some("ANY")),
    ("Crash", Some("CRASH")),
    ("Provoke", Some("PROVOKE")),
    ("Sandman", Some("SANDMAN")),
    ("Freeze Trap", Some("FREEZE")),
    ("Decrease Agi", Some("DECAGI")),
    ("Lex Divina", Some("LEXDIV")),
];

// The field order follows the web GUI; the form lays them out in two columns.
static HOM_FIELDS: &[Field] = &[
    Field { index: BASIC, label: "Basic Behavior", kind: FieldKind::Select(BASIC_OPTIONS) },
    Field { index: DEBUFF, label: "Debuff", kind: FieldKind::Debuff(HOM_DEBUFF) },
    Field { index: SKILL, label: "Use Attack Skills", kind: FieldKind::Skill },
    Field { index: KITE, label: "Kiting", kind: FieldKind::Select(KITE_OPTIONS) },
    Field { index: CAST, label: "React", kind: FieldKind::Select(HOM_CAST) },
    Field { index: CLASS, label: "Skill Class", kind: FieldKind::Select(HOM_CLASS) },
    Field { index: SNIPE, label: "Snipe OK", kind: FieldKind::Snipe },
    Field { index: RESCUE, label: "Rescue", kind: FieldKind::Select(HOM_RESCUE) },
    Field { index: SP, label: "SP", kind: FieldKind::Sp },
    Field { index: KS, label: "Kill-steal", kind: FieldKind::Select(KS_OPTIONS) },
    Field { index: WEIGHT, label: "Weight", kind: FieldKind::Weight },
    Field { index: CHASE, label: "Chase", kind: FieldKind::Select(CHASE_OPTIONS) },
];
static MER_FIELDS: &[Field] = &[
    Field { index: BASIC, label: "Basic Behavior", kind: FieldKind::Select(BASIC_OPTIONS) },
    Field { index: DEBUFF, label: "Use Debuffs", kind: FieldKind::Debuff(MER_DEBUFF) },
    Field { index: SKILL, label: "Use Attack Skills", kind: FieldKind::Skill },
    Field { index: KITE, label: "Kiting", kind: FieldKind::Select(KITE_OPTIONS) },
    Field { index: CAST, label: "React", kind: FieldKind::Select(MER_CAST) },
    Field { index: CLASS, label: "Skill Class", kind: FieldKind::Select(MER_CLASS) },
    Field { index: SNIPE, label: "Snipe OK", kind: FieldKind::Snipe },
    Field { index: PUSH, label: "Push", kind: FieldKind::Select(PUSH_OPTIONS) },
    Field { index: RESCUE, label: "Rescue", kind: FieldKind::Select(MER_RESCUE) },
    Field { index: SP, label: "SP", kind: FieldKind::Sp },
    Field { index: KS, label: "Kill Steal", kind: FieldKind::Select(KS_OPTIONS) },
    Field { index: WEIGHT, label: "Weight", kind: FieldKind::Weight },
    Field { index: CHASE, label: "Chase", kind: FieldKind::Select(CHASE_OPTIONS) },
];
static HOM_PVP_FIELDS: &[Field] = &[
    Field { index: BASIC, label: "Basic Behavior", kind: FieldKind::Select(BASIC_OPTIONS) },
    Field { index: CAST, label: "React", kind: FieldKind::Select(HOM_CAST) },
    Field { index: DEBUFF, label: "Use Debuffs", kind: FieldKind::Debuff(HOM_DEBUFF) },
    Field { index: SKILL, label: "Use Attack Skills", kind: FieldKind::Skill },
    Field { index: KITE, label: "Kiting", kind: FieldKind::Select(KITE_OPTIONS) },
    Field { index: CLASS, label: "Skill Class", kind: FieldKind::Select(HOM_CLASS) },
    Field { index: RESCUE, label: "Rescue", kind: FieldKind::Select(HOM_RESCUE) },
];
static MER_PVP_FIELDS: &[Field] = &[
    Field { index: BASIC, label: "Basic Behavior", kind: FieldKind::Select(BASIC_OPTIONS) },
    Field { index: CAST, label: "React", kind: FieldKind::Select(MER_CAST) },
    Field { index: DEBUFF, label: "Use Debuffs", kind: FieldKind::Debuff(MER_DEBUFF) },
    Field { index: SKILL, label: "Use Attack Skills", kind: FieldKind::Skill },
    Field { index: PUSH, label: "Use Pushback", kind: FieldKind::Select(PUSH_OPTIONS) },
    Field { index: KITE, label: "Kiting", kind: FieldKind::Select(KITE_OPTIONS) },
    Field { index: CLASS, label: "Skill Class", kind: FieldKind::Select(MER_CLASS) },
    Field { index: RESCUE, label: "Rescue", kind: FieldKind::Select(MER_RESCUE) },
];

static HOM: Layout = Layout {
    title: "Homunculus Tactics",
    file: "H_Tactics.lua",
    kind: Kind::Tactics,
    id_label: "Monster ID",
    name_label: "Monster Name",
    warning: None,
    once_level: true,
    fields: HOM_FIELDS,
};
static MER: Layout = Layout {
    title: "Mercenary Tactics",
    file: "M_Tactics.lua",
    kind: Kind::Tactics,
    id_label: "Monster ID",
    name_label: "Monster Name",
    warning: Some(
        "WARNING: non-default mercenary tactics require a MobID file. It must be regenerated \
         after each maintenance. If you do not know what this means, you don't have it.",
    ),
    once_level: false,
    fields: MER_FIELDS,
};
static HOM_PVP: Layout = Layout {
    title: "PVP Tactics",
    file: "H_PVP_Tact.lua",
    kind: Kind::Pvp,
    id_label: "Player ID or Friend Class",
    name_label: "Monster Name",
    warning: None,
    once_level: true,
    fields: HOM_PVP_FIELDS,
};
static MER_PVP: Layout = Layout {
    title: "Merc PVP Tactics",
    file: "M_PVP_Tact.lua",
    kind: Kind::Pvp,
    id_label: "Player ID or Friend Class",
    name_label: "Monster Name",
    warning: None,
    once_level: false,
    fields: MER_PVP_FIELDS,
};

// ---------- drawing ----------

/// Draws the whole tab. Returns true when something was added/removed (the caller may want to
/// refresh a summary in the same frame).
pub fn draw(
    ui: &mut egui::Ui,
    p: &Palette,
    tab: Tab,
    tactics: &mut Tactics,
    selected: &mut usize,
    filter: &mut String,
    mobs: &Mobs,
) {
    let layout = tab.layout();
    if let Some(warning) = layout.warning {
        warning_box(ui, p, warning);
        ui.add_space(6.0);
    }
    let height = (ui.available_height() - 22.0).max(320.0);
    let total = ui.available_width();
    let left_w = (total * 0.30).clamp(230.0, 340.0);
    let right_w = (total - left_w - 8.0).max(240.0);
    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(left_w, height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| list_panel(ui, p, tab, tactics, selected, filter, mobs, height),
        );
        ui.add_space(8.0);
        ui.allocate_ui_with_layout(
            egui::vec2(right_w, height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| form_panel(ui, p, layout, tab.key(), tactics, *selected, mobs, height),
        );
    });
    ui.add_space(4.0);
    let visible = tactics.entries.iter().filter(|entry| !entry.is_removed()).count();
    ui.label(
        egui::RichText::new(format!(
            "{visible} entries  ·  {} changed  ·  Apply Settings writes {}",
            tactics.changed(),
            layout.file
        ))
        .color(p.muted)
        .size(11.0),
    );
}

fn warning_box(ui: &mut egui::Ui, p: &Palette, text: &str) {
    egui::Frame::none()
        .fill(p.pending.gamma_multiply(0.12))
        .stroke(egui::Stroke::new(1.0_f32, p.pending.gamma_multiply(0.7)))
        .rounding(egui::Rounding::same(5.0))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(0.0));
            ui.label(egui::RichText::new(text).color(p.text).size(11.5));
        });
}

fn entry_label(layout: &Layout, tactics: &Tactics, index: usize, mobs: &Mobs) -> String {
    let entry = &tactics.entries[index];
    if layout.kind.is_pvp() {
        return entry.id.clone();
    }
    if !entry.name.is_empty() {
        return entry.name.clone();
    }
    if let Some(name) = mobs.name(&entry.id) {
        return format!("{name} (#{})", entry.id);
    }
    format!("(unnamed) #{}", entry.id)
}

fn list_panel(
    ui: &mut egui::Ui,
    p: &Palette,
    tab: Tab,
    tactics: &mut Tactics,
    selected: &mut usize,
    filter: &mut String,
    mobs: &Mobs,
    height: f32,
) {
    let layout = tab.layout();
    let mut select_first = false;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Find:").color(p.muted).size(11.0));
        let width = (ui.available_width() - 50.0).max(80.0);
        let response = ui.add_sized(
            [width, 20.0],
            egui::TextEdit::singleline(filter)
                .hint_text("type a name or id, Enter picks the first match")
                .id_source(format!("tact-filter:{}", tab.key())),
        );
        if response.lost_focus() && !response.changed() {
            select_first = true;
        }
        if ui.small_button("Clear").clicked() {
            filter.clear();
        }
    });
    ui.add_space(4.0);
    let needle = filter.trim().to_lowercase();
    let visible: Vec<usize> = tactics
        .entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| !entry.is_removed())
        .filter(|(index, entry)| {
            if needle.is_empty() {
                return true;
            }
            entry_label(layout, tactics, *index, mobs).to_lowercase().contains(&needle)
                || entry.id.to_lowercase().contains(&needle)
                || entry.name.to_lowercase().contains(&needle)
        })
        .map(|(index, _)| index)
        .collect();
    let visible_count = visible.len();
    let mut scroll_to_selected = false;
    if select_first {
        if let Some(first) = visible.first() {
            *selected = *first;
            scroll_to_selected = true;
        }
    } else if !visible.is_empty() && !visible.contains(selected) {
        *selected = visible[0];
        scroll_to_selected = true;
    }
    let list_height = (height - 80.0).max(140.0);
    egui::Frame::none()
        .fill(p.grid)
        .stroke(egui::Stroke::new(1.0_f32, p.border))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::same(3.0))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(ui.available_width().max(0.0), list_height));
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .drag_to_scroll(false)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width().max(0.0));
                    if visible.is_empty() {
                        ui.label(
                            egui::RichText::new("No entries yet. Use Add.")
                                .color(p.muted)
                                .size(12.0),
                        );
                    }
                    for index in visible {
                        let text = entry_label(layout, tactics, index, mobs);
                        let active = index == *selected;
                        // the whole row is the click target: aiming at the text alone was hard
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), 24.0),
                            egui::Sense::click(),
                        );
                        let fill = if active {
                            p.primary
                        } else if response.hovered() {
                            p.panel2
                        } else {
                            egui::Color32::TRANSPARENT
                        };
                        ui.painter().rect_filled(rect, egui::Rounding::same(3.0), fill);
                        ui.painter()
                            .with_clip_rect(rect)
                            .text(
                                rect.left_center() + egui::vec2(6.0, 0.0),
                                egui::Align2::LEFT_CENTER,
                                &text,
                                egui::FontId::proportional(12.0),
                                if active { egui::Color32::WHITE } else { p.text },
                            );
                        if response.clicked() {
                            *selected = index;
                        }
                        response.on_hover_cursor(egui::CursorIcon::PointingHand);
                        if active && scroll_to_selected {
                            ui.scroll_to_rect(rect, Some(egui::Align::Center));
                            scroll_to_selected = false;
                        }
                    }
                });
        });
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if ui
            .add(
                egui::Button::new(egui::RichText::new("Add").color(p.btn_text))
                    .fill(p.btn1)
                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
            )
            .clicked()
        {
            let id = if layout.kind.is_pvp() {
                let mut candidate = "new".to_string();
                let mut suffix = 1;
                while tactics.entries.iter().any(|entry| entry.id == candidate && !entry.is_removed()) {
                    candidate = format!("new{suffix}");
                    suffix += 1;
                }
                candidate
            } else {
                tactics.next_numeric_id()
            };
            *selected = tactics.add(id, "");
        }
        let can_remove = tactics
            .entries
            .get(*selected)
            .map(|entry| !entry.is_zero() && !entry.is_removed())
            .unwrap_or(false);
        if ui
            .add_enabled(
                can_remove,
                egui::Button::new(egui::RichText::new("Remove").color(p.btn_text))
                    .fill(p.btn1)
                    .stroke(egui::Stroke::new(1.0_f32, p.btn_border)),
            )
            .clicked()
        {
            if tactics.remove(*selected) {
                *selected = 0;
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let total = tactics.entries.iter().filter(|entry| !entry.is_removed()).count();
            ui.label(
                egui::RichText::new(format!("{visible_count} of {total}"))
                    .color(p.muted)
                    .size(11.0),
            );
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn form_panel(
    ui: &mut egui::Ui,
    p: &Palette,
    layout: &Layout,
    tab_key: &str,
    tactics: &mut Tactics,
    selected: usize,
    mobs: &Mobs,
    height: f32,
) {
    theme::card(p).show(ui, |ui| {
        ui.set_min_size(egui::vec2(ui.available_width().max(0.0), (height - 12.0).max(200.0)));
        // ids of the other entries, for the duplicate validation below (before the mutable borrow)
        let others: Vec<String> = tactics
            .entries
            .iter()
            .enumerate()
            .filter(|(index, other)| *index != selected && !other.is_removed())
            .map(|(_, other)| other.id.clone())
            .collect();
        let Some(entry) = tactics.entry_mut(selected) else {
            ui.label(egui::RichText::new("Select an entry on the left.").color(p.muted));
            return;
        };
        if entry.is_removed() {
            ui.label(egui::RichText::new("This entry was removed.").color(p.muted));
            return;
        }
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width().max(0.0));
                ui.spacing_mut().item_spacing.y = 6.0;
                ui.horizontal_top(|ui| {
                    if !layout.kind.is_pvp() {
                        let width = (ui.available_width() * 0.62).max(160.0);
                        ui.allocate_ui_with_layout(
                            egui::vec2(width, 44.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(layout.name_label).color(p.muted).size(11.0),
                                );
                                ui.add_enabled_ui(!entry.is_zero(), |ui| {
                                    ui.add_sized(
                                        [ui.available_width().max(60.0), 22.0],
                                        egui::TextEdit::singleline(&mut entry.name)
                                            .id_source(format!("{tab_key}:name")),
                                    );
                                });
                            },
                        );
                    }
                    let id_width = if layout.kind.is_pvp() {
                        ui.available_width()
                    } else {
                        ui.available_width().max(120.0)
                    };
                    ui.allocate_ui_with_layout(
                        egui::vec2(id_width, 44.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.label(egui::RichText::new(layout.id_label).color(p.muted).size(11.0));
                            let zero = entry.is_zero();
                            ui.add_enabled_ui(!zero, |ui| {
                                ui.add_sized(
                                    [ui.available_width().max(60.0), 22.0],
                                    egui::TextEdit::singleline(&mut entry.id)
                                        .id_source(format!("{tab_key}:id")),
                                );
                            });
                        },
                    );
                });
                let duplicate = others.iter().any(|id| id == &entry.id);
                let invalid = if layout.kind.is_pvp() {
                    entry.id.trim().is_empty() || entry.id == "new" || duplicate
                } else {
                    entry.id.trim().parse::<i64>().is_err() || duplicate
                };
                if invalid {
                    let message = if duplicate {
                        "This id is already used by another entry."
                    } else if layout.kind.is_pvp() {
                        "You must enter a name."
                    } else {
                        "You must enter a numeric monster id."
                    };
                    ui.label(egui::RichText::new(message).color(p.pending).size(11.0));
                }
                if !layout.kind.is_pvp() {
                    if let Some(mob) = mobs.get(&entry.id) {
                        let mut line = format!(
                            "#{} {}  ·  Lv {}  ·  {} {}  ·  {}",
                            mob.id,
                            mob.name,
                            mob.level,
                            mobs.element_name(mob),
                            mob.element_level,
                            mobs.race_name(mob)
                        );
                        if mob.flags & 2 != 0 {
                            line.push_str("  ·  MVP");
                        } else if mob.flags & 1 != 0 {
                            line.push_str("  ·  Boss");
                        }
                        ui.label(egui::RichText::new(line).color(p.muted).size(11.0));
                    }
                }
                ui.add_space(2.0);
                egui::Frame::none()
                    .stroke(egui::Stroke::new(1.0_f32, p.border))
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width().max(0.0));
                        ui.label(
                            egui::RichText::new("Overall Tactics").color(p.tab_text).strong(),
                        );
                        ui.add_space(2.0);
                        ui.columns(2, |columns| {
                            for (index, field) in layout.fields.iter().enumerate() {
                                let column = &mut columns[index % 2];
                                field_ui(column, p, layout, tab_key, field, entry);
                            }
                        });
                    });
            });
    });
}

fn field_ui(
    ui: &mut egui::Ui,
    p: &Palette,
    layout: &Layout,
    tab_key: &str,
    field: &Field,
    entry: &mut azzy_config::tactics::Entry,
) {
    ui.add_space(3.0);
    ui.label(egui::RichText::new(field.label).color(p.muted).size(11.0));
    let id = format!("tact:{tab_key}:{}", field.index);
    match &field.kind {
        FieldKind::Select(options) => {
            let current = entry.values[field.index].clone();
            let selected = options
                .iter()
                .find(|(_, token)| *token == current)
                .map(|(label, _)| (*label).to_string())
                .unwrap_or_else(|| unsupported(&current));
            let mut chosen = selected.clone();
            egui::ComboBox::from_id_source(&id)
                .width(ui.available_width().max(80.0))
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    for (label, token) in options.iter() {
                        if ui.selectable_value(&mut chosen, (*label).to_string(), *label).clicked() {
                            entry.values[field.index] = (*token).to_string();
                        }
                    }
                });
        }
        FieldKind::Skill => skill_ui(ui, &id, layout.once_level, entry),
        FieldKind::Debuff(options) => debuff_ui(ui, p, &id, options, entry),
        FieldKind::Snipe => {
            let mut on = entry.values[field.index] != "SNIPE_DISABLE";
            if ui.checkbox(&mut on, "Snipe OK").changed() {
                entry.values[field.index] = if on { "SNIPE_OK" } else { "SNIPE_DISABLE" }.to_string();
            }
        }
        FieldKind::Sp => {
            let mut value = entry.values[field.index].parse::<f64>().unwrap_or(-1.0);
            let response = ui.add(
                egui::DragValue::new(&mut value)
                    .range(-1.0..=1000.0)
                    .speed(1.0)
                    .max_decimals(0),
            );
            if response.changed() {
                entry.values[field.index] = format!("{}", value as i64);
            }
        }
        FieldKind::Weight => {
            let mut value = entry.values[field.index].parse::<f64>().unwrap_or(1.0);
            let response = ui.add(
                egui::DragValue::new(&mut value)
                    .range(0.0..=20.0)
                    .speed(0.1)
                    .max_decimals(1),
            );
            if response.changed() {
                entry.values[field.index] = format_weight(value);
            }
        }
    }
}

fn unsupported(current: &str) -> String {
    format!("{current} (not supported by the GUI)")
}

fn format_weight(value: f64) -> String {
    let rounded = (value * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SkillMode {
    Always,
    Never,
    Times,
    Once,
    Raw,
}

fn skill_ui(
    ui: &mut egui::Ui,
    id: &str,
    once_level: bool,
    entry: &mut azzy_config::tactics::Entry,
) {
    let current = entry.values[SKILL].clone();
    let (mut mode, mut number) = match current.as_str() {
        "SKILL_ALWAYS" => (SkillMode::Always, 100.0),
        "SKILL_NEVER" => (SkillMode::Never, 0.0),
        _ => match current.trim().parse::<i64>() {
            Ok(value) if value < 0 && once_level => (SkillMode::Once, (-value).min(5) as f64),
            Ok(value) => (SkillMode::Times, value.clamp(0, 100) as f64),
            Err(_) => (SkillMode::Raw, 0.0),
        },
    };
    let before_mode = mode;
    let selected = match mode {
        SkillMode::Always => "Always",
        SkillMode::Never => "Never",
        SkillMode::Times => "This many times",
        SkillMode::Once => "Once; level",
        SkillMode::Raw => "Custom",
    };
    let mut chosen = selected.to_string();
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(format!("{id}:mode"))
            .width((ui.available_width() - 70.0).max(90.0))
            .selected_text(if mode == SkillMode::Raw {
                unsupported(&current)
            } else {
                selected.to_string()
            })
            .show_ui(ui, |ui| {
                if ui.selectable_value(&mut chosen, "Always".to_string(), "Always").clicked() {
                    mode = SkillMode::Always;
                }
                if ui.selectable_value(&mut chosen, "Never".to_string(), "Never").clicked() {
                    mode = SkillMode::Never;
                }
                if ui.selectable_value(&mut chosen, "This many times".to_string(), "This many times").clicked() {
                    mode = SkillMode::Times;
                }
                if once_level
                    && ui.selectable_value(&mut chosen, "Once; level".to_string(), "Once; level").clicked()
                {
                    mode = SkillMode::Once;
                }
                if mode == SkillMode::Raw {
                    ui.selectable_value(&mut chosen, "Custom".to_string(), unsupported(&current));
                }
            });
        ui.add_enabled_ui(mode == SkillMode::Times || mode == SkillMode::Once, |ui| {
            let response = ui.add(
                egui::DragValue::new(&mut number)
                    .range(if mode == SkillMode::Once { 1.0..=5.0 } else { 0.0..=100.0 })
                    .speed(1.0)
                    .max_decimals(0),
            );
            if response.changed() {
                number = number.clamp(
                    if mode == SkillMode::Once { 1.0 } else { 0.0 },
                    if mode == SkillMode::Once { 5.0 } else { 100.0 },
                );
                entry.values[SKILL] = match mode {
                    SkillMode::Once => format!("-{}", number as i64),
                    _ => format!("{}", number as i64),
                };
            }
        });
        if mode != before_mode {
            entry.values[SKILL] = match mode {
                SkillMode::Always => "SKILL_ALWAYS".to_string(),
                SkillMode::Never => "SKILL_NEVER".to_string(),
                SkillMode::Times => format!("{}", number as i64),
                SkillMode::Once => format!("-{}", number.max(1.0) as i64),
                SkillMode::Raw => current.clone(),
            };
        }
    });
}

fn debuff_ui(
    ui: &mut egui::Ui,
    p: &Palette,
    id: &str,
    options: &'static [(&'static str, Option<&'static str>)],
    entry: &mut azzy_config::tactics::Entry,
) {
    let current = entry.values[DEBUFF].clone();
    let mut kind: Option<&str> = None;
    let mut phase = "C";
    let mut known = current == "DEBUFF_NEVER";
    if let Some(rest) = current.strip_prefix("DEBUFF_") {
        if let Some((name, ending)) = rest.rsplit_once('_') {
            if ending == "C" || ending == "A" {
                if options.iter().any(|(_, token)| *token == Some(name)) {
                    kind = Some(name);
                    phase = ending;
                    known = true;
                }
            }
        }
    }
    let mut touched = false;
    ui.horizontal(|ui| {
        let selected = if known {
            options
                .iter()
                .find(|(_, token)| *token == kind)
                .map(|(label, _)| (*label).to_string())
                .unwrap_or_else(|| "Never".to_string())
        } else {
            unsupported(&current)
        };
        let mut chosen = selected.clone();
        egui::ComboBox::from_id_source(format!("{id}:kind"))
            .width((ui.available_width() * 0.52).max(100.0))
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (label, token) in options {
                    if ui.selectable_value(&mut chosen, (*label).to_string(), *label).clicked() {
                        kind = *token;
                        known = true;
                        touched = true;
                    }
                }
            });
        ui.label(egui::RichText::new("While").color(p.muted).size(11.0));
        let mut chasing = phase == "C";
        ui.add_enabled_ui(kind.is_some(), |ui| {
            egui::ComboBox::from_id_source(format!("{id}:phase"))
                .width(86.0)
                .selected_text(if chasing { "Chasing" } else { "Attacking" })
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut chasing, true, "Chasing").clicked() {
                        phase = "C";
                        touched = true;
                    }
                    if ui.selectable_value(&mut chasing, false, "Attacking").clicked() {
                        phase = "A";
                        touched = true;
                    }
                });
        });
    });
    // Only write when the user picked something: an unknown token must survive a redraw.
    if touched {
        entry.values[DEBUFF] = match kind {
            None => "DEBUFF_NEVER".to_string(),
            Some(kind) => format!("DEBUFF_{kind}_{phase}"),
        };
    }
}
