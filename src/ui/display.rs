use eframe::egui::{self, Color32, FontId, Vec2};
use crate::calculator::Calculator;

pub fn render_display(ui: &mut egui::Ui, calc: &Calculator, display_h: f32) {
    egui::Frame::new()
        .fill(Color32::from_rgb(28, 28, 30))
        .inner_margin(egui::Margin { left: 12, right: 12, top: 8, bottom: 4 })
        .show(ui, |ui| {
            let w = ui.available_width();

            // Formula line — medium, right-aligned, gray (shows full expression)
            if !calc.formula.is_empty() && !calc.error {
                ui.allocate_ui_with_layout(
                    Vec2::new(w, 40.0),
                    egui::Layout::right_to_left(egui::Align::BOTTOM),
                    |ui| {
                        ui.label(
                            egui::RichText::new(&calc.formula)
                                .font(FontId::monospace(16.0))
                                .color(Color32::from_rgb(140, 140, 145)),
                        );
                    },
                );
            }

            // Main number — large, right-aligned
            let main_h = if !calc.formula.is_empty() && !calc.error { display_h - 40.0 } else { display_h };
            ui.allocate_ui_with_layout(
                Vec2::new(w, main_h),
                egui::Layout::right_to_left(egui::Align::BOTTOM),
                |ui| {
                    let display_text = if calc.display.is_empty() { "0" } else { &calc.display };
                    let font_size = if display_text.len() > 12 { 32.0 } else { 52.0 };
                    ui.label(
                        egui::RichText::new(display_text)
                            .font(FontId::monospace(font_size))
                            .color(Color32::WHITE),
                    );
                },
            );
        });
}
