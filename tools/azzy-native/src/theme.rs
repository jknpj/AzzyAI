// theme.rs: the web GUI's look, in egui.
//
// The palette and the shapes come from webgui/index.html (the "Ragnarok look" variables): the
// blue gradient title bar, framed panels with a highlight, nav-tabs joined to the panel below,
// compact rows with a coloured left bar (blue = changed from default, amber = not applied yet)
// and the footer with the primary Apply button.

use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    pub fn toggled(self) -> Mode {
        match self {
            Mode::Dark => Mode::Light,
            Mode::Light => Mode::Dark,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg1: egui::Color32,
    pub bg2: egui::Color32,
    pub panel: egui::Color32,
    pub panel2: egui::Color32,
    pub grid: egui::Color32,
    pub border: egui::Color32,
    pub title1: egui::Color32,
    pub title2: egui::Color32,
    pub title_edge: egui::Color32,
    pub title_text: egui::Color32,
    pub tab1: egui::Color32,
    pub tab2: egui::Color32,
    pub tab_text: egui::Color32,
    pub tab_active_text: egui::Color32,
    pub btn1: egui::Color32,
    pub btn2: egui::Color32,
    pub btn_border: egui::Color32,
    pub btn_text: egui::Color32,
    pub primary: egui::Color32,
    pub text: egui::Color32,
    pub muted: egui::Color32,
    pub modified: egui::Color32,
    pub pending: egui::Color32,
}

pub fn palette(mode: Mode) -> Palette {
    match mode {
        Mode::Dark => Palette {
            bg1: egui::Color32::from_rgb(0x0e, 0x15, 0x25),
            bg2: egui::Color32::from_rgb(0x1b, 0x27, 0x42),
            panel: egui::Color32::from_rgb(0x18, 0x22, 0x37),
            panel2: egui::Color32::from_rgb(0x22, 0x30, 0x4c),
            grid: egui::Color32::from_rgb(0x12, 0x1a, 0x2b),
            border: egui::Color32::from_rgb(0x3d, 0x58, 0x88),
            title1: egui::Color32::from_rgb(0x4a, 0x6e, 0xae),
            title2: egui::Color32::from_rgb(0x23, 0x3c, 0x6d),
            title_edge: egui::Color32::from_rgb(0x11, 0x20, 0x3d),
            title_text: egui::Color32::from_rgb(0xf2, 0xf6, 0xff),
            tab1: egui::Color32::from_rgb(0x2a, 0x3b, 0x5d),
            tab2: egui::Color32::from_rgb(0x1d, 0x2a, 0x44),
            tab_text: egui::Color32::from_rgb(0xb9, 0xcb, 0xec),
            tab_active_text: egui::Color32::WHITE,
            btn1: egui::Color32::from_rgb(0x33, 0x47, 0x6e),
            btn2: egui::Color32::from_rgb(0x23, 0x33, 0x53),
            btn_border: egui::Color32::from_rgb(0x4b, 0x65, 0x97),
            btn_text: egui::Color32::from_rgb(0xdc, 0xe6, 0xfa),
            primary: egui::Color32::from_rgb(0x5f, 0x86, 0xc9),
            text: egui::Color32::from_rgb(0xdb, 0xe4, 0xf3),
            muted: egui::Color32::from_rgb(0x9a, 0xae, 0xd2),
            modified: egui::Color32::from_rgb(0x6d, 0x93, 0xd6),
            pending: egui::Color32::from_rgb(0xe0, 0xb0, 0x4a),
        },
        Mode::Light => Palette {
            bg1: egui::Color32::from_rgb(0xdb, 0xe6, 0xf5),
            bg2: egui::Color32::from_rgb(0xb9, 0xcb, 0xe7),
            panel: egui::Color32::from_rgb(0xf8, 0xfa, 0xfe),
            panel2: egui::Color32::from_rgb(0xe6, 0xee, 0xf9),
            grid: egui::Color32::from_rgb(0xff, 0xff, 0xff),
            border: egui::Color32::from_rgb(0x7f, 0x9d, 0xd0),
            title1: egui::Color32::from_rgb(0x7f, 0xa2, 0xd9),
            title2: egui::Color32::from_rgb(0x44, 0x67, 0xa8),
            title_edge: egui::Color32::from_rgb(0x2d, 0x4b, 0x82),
            title_text: egui::Color32::WHITE,
            tab1: egui::Color32::from_rgb(0xf2, 0xf6, 0xfc),
            tab2: egui::Color32::from_rgb(0xd6, 0xe2, 0xf4),
            tab_text: egui::Color32::from_rgb(0x2b, 0x4a, 0x80),
            tab_active_text: egui::Color32::from_rgb(0x10, 0x21, 0x3f),
            btn1: egui::Color32::from_rgb(0xff, 0xff, 0xff),
            btn2: egui::Color32::from_rgb(0xdb, 0xe4, 0xf2),
            btn_border: egui::Color32::from_rgb(0x8a, 0xa6, 0xd6),
            btn_text: egui::Color32::from_rgb(0x23, 0x40, 0x6f),
            primary: egui::Color32::from_rgb(0x3d, 0x62, 0xa5),
            text: egui::Color32::from_rgb(0x22, 0x2a, 0x36),
            muted: egui::Color32::from_rgb(0x5c, 0x6b, 0x82),
            modified: egui::Color32::from_rgb(0x2d, 0x56, 0xa0),
            pending: egui::Color32::from_rgb(0xb0, 0x7a, 0x10),
        },
    }
}

pub fn apply(ctx: &egui::Context, mode: Mode) {
    let p = palette(mode);
    let mut style = (*ctx.style()).clone();
    let visuals = &mut style.visuals;
    visuals.dark_mode = mode == Mode::Dark;
    visuals.override_text_color = Some(p.text);
    visuals.panel_fill = p.panel;
    visuals.window_fill = p.panel;
    visuals.extreme_bg_color = p.grid;
    visuals.faint_bg_color = p.panel2;
    visuals.window_stroke = egui::Stroke::new(1.0_f32, p.border);
    visuals.hyperlink_color = p.primary;
    visuals.selection.bg_fill = p.primary;
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, p.tab_active_text);
    visuals.window_rounding = egui::Rounding::same(6.0);
    let widgets = &mut visuals.widgets;
    for (widget, fill) in [
        (&mut widgets.noninteractive, p.panel2),
        (&mut widgets.inactive, p.btn1),
        (&mut widgets.hovered, p.tab1),
        (&mut widgets.active, p.btn2),
        (&mut widgets.open, p.tab2),
    ] {
        widget.bg_fill = fill;
        widget.weak_bg_fill = fill;
        widget.bg_stroke = egui::Stroke::new(1.0_f32, p.btn_border);
        widget.fg_stroke = egui::Stroke::new(1.0_f32, p.text);
        widget.rounding = egui::Rounding::same(4.0);
    }
    widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, p.btn_text);
    widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, p.tab_active_text);
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 4.0);
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(17.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(13.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Button, egui::FontId::new(13.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(11.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(12.0, egui::FontFamily::Monospace)),
    ]
    .into();
    ctx.set_style(style);
}

/// Paints a vertical gradient, top to bottom, in `rect` (the web GUI's title bars and page).
pub fn gradient(painter: &egui::Painter, rect: egui::Rect, top: egui::Color32, bottom: egui::Color32) {
    if !rect.is_positive() {
        return;
    }
    let vertex = |pos: egui::Pos2, color: egui::Color32| egui::epaint::Vertex {
        pos,
        uv: egui::Pos2::ZERO,
        color,
    };
    let mut mesh = egui::Mesh::default();
    mesh.vertices.push(vertex(rect.left_top(), top));
    mesh.vertices.push(vertex(rect.right_top(), top));
    mesh.vertices.push(vertex(rect.right_bottom(), bottom));
    mesh.vertices.push(vertex(rect.left_bottom(), bottom));
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(egui::Shape::mesh(mesh));
}

/// A full-width band with a gradient background and vertically centred contents, e.g. the navbar.
pub fn band<R>(
    ui: &mut egui::Ui,
    height: f32,
    top: egui::Color32,
    bottom: egui::Color32,
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    gradient(ui.painter(), rect, top, bottom);
    ui.allocate_ui_at_rect(rect.shrink2(egui::vec2(12.0, 0.0)), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), contents).inner
    })
    .inner
}

/// A framed panel like `.card` in the web GUI.
pub fn card(p: &Palette) -> egui::Frame {
    egui::Frame::none()
        .fill(p.panel)
        .stroke(egui::Stroke::new(1.0_f32, p.border))
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(6.0))
}

/// The blue title bar of a card (`.card-header`).
pub fn title_bar(p: &Palette, ui: &mut egui::Ui, text: &str) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 24.0),
        egui::Sense::hover(),
    );
    gradient(ui.painter(), rect, p.title1, p.title2);
    ui.painter().hline(rect.x_range(), rect.bottom(), egui::Stroke::new(1.0_f32, p.title_edge));
    ui.allocate_ui_at_rect(rect.shrink2(egui::vec2(8.0, 0.0)), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(text).color(p.title_text).strong());
        });
    });
}

/// A nav-tab: rounded top, the active one merges with the panel below.
pub fn tab(ui: &mut egui::Ui, p: &Palette, text: &str, active: bool) -> egui::Response {
    let (fill, color) = if active {
        (p.panel, p.tab_active_text)
    } else {
        (p.tab2, p.tab_text)
    };
    let mut label = egui::RichText::new(text).color(color).size(12.5);
    if active {
        label = label.strong();
    }
    ui.add(
        egui::Button::new(label)
            .fill(fill)
            .stroke(egui::Stroke::new(1.0_f32, p.border))
            .rounding(egui::Rounding { nw: 5.0, ne: 5.0, sw: 0.0, se: 0.0 })
            .min_size(egui::vec2(0.0, 25.0)),
    )
}

/// A pill like Bootstrap's `.badge`.
pub fn badge(ui: &mut egui::Ui, text: &str, fill: egui::Color32, text_color: egui::Color32) {
    egui::Frame::none()
        .fill(fill)
        .rounding(egui::Rounding::same(9.0))
        .inner_margin(egui::Margin::symmetric(8.0, 2.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(text_color).size(11.0).strong());
        });
}

/// The legend swatch: a light tint with a solid bar on the left.
pub fn swatch(ui: &mut egui::Ui, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, egui::Rounding::same(2.0), color.gamma_multiply(0.22));
    let bar = egui::Rect::from_min_size(rect.min, egui::vec2(3.0, rect.height()));
    ui.painter().rect_filled(bar, egui::Rounding::same(1.0), color);
}
