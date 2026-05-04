use eframe::egui::{self, Color32, FontId, Vec2};
use crate::calculator::{Calculator, ops};

#[derive(Clone, Copy, PartialEq)]
pub enum BtnKind {
    Digit,
    Op,
    Action,
    Equals,
    Sci,
}

pub fn btn_colors(kind: BtnKind) -> (Color32, Color32) {
    match kind {
        BtnKind::Digit  => (Color32::from_rgb(58, 58, 60),  Color32::WHITE),
        BtnKind::Op     => (Color32::from_rgb(255, 149, 0), Color32::WHITE),
        BtnKind::Action => (Color32::from_rgb(72, 72, 74),  Color32::WHITE),
        BtnKind::Equals => (Color32::from_rgb(255, 149, 0), Color32::WHITE),
        BtnKind::Sci    => (Color32::from_rgb(44, 44, 46),  Color32::from_rgb(10, 132, 255)),
    }
}

pub fn calc_button(ui: &mut egui::Ui, label: &str, kind: BtnKind, w: f32, h: f32) -> bool {
    let (bg, fg) = btn_colors(kind);
    let font_size = if label.len() > 3 { 14.0 } else { 22.0 };
    let btn = egui::Button::new(
        egui::RichText::new(label)
            .font(FontId::proportional(font_size))
            .color(fg),
    )
    .fill(bg)
    .corner_radius((h / 2.0) as u8)
    .min_size(Vec2::new(w, h));
    ui.add(btn).clicked()
}

pub fn basic_row(
    ui: &mut egui::Ui,
    btn_w: f32,
    btn_h: f32,
    gap: f32,
    btns: &[(&str, BtnKind)],
    calc: &mut Calculator,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        for &(label, kind) in btns {
            if calc_button(ui, label, kind, btn_w, btn_h) {
                ops::handle_basic(calc, label);
            }
        }
    });
}

pub fn sci_row(
    ui: &mut egui::Ui,
    btn_w: f32,
    btn_h: f32,
    gap: f32,
    btns: &[(&str, BtnKind)],
    calc: &mut Calculator,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        for &(label, kind) in btns {
            if calc_button(ui, label, kind, btn_w, btn_h) {
                ops::handle_sci(calc, label);
            }
        }
    });
}
