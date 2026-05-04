use eframe::egui::{self, Color32, FontId};
use crate::calculator::Calculator;
use crate::ui::{
    buttons::{basic_row, sci_row, BtnKind},
    display::render_display,
};

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        for ch in text.chars() {
                            match ch {
                                '0'..='9' | '.' => self.push_digit(ch),
                                '+' => self.apply_op('+'),
                                '-' => self.apply_op('-'),
                                '*' => self.apply_op('*'),
                                '/' => self.apply_op('/'),
                                '=' => self.equals(),
                                '(' => self.open_paren(),
                                ')' => self.close_paren(),
                                '%' => self.percent(),
                                _ => {}
                            }
                        }
                    }
                    egui::Event::Key { key, pressed: true, repeat: false, .. } => {
                        match key {
                            egui::Key::Enter => self.equals(),
                            egui::Key::Backspace => self.backspace(),
                            egui::Key::Escape => self.clear(),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(28, 28, 30)))
            .show(ctx, |ui| {
                let avail_w = ui.available_width();
                let sci_rows = if self.show_scientific { 5 } else { 0 };
                let total_rows = (5 + sci_rows) as f32;
                let avail_h = ui.available_height();
                let display_h = 90.0f32.max(avail_h * 0.17);

                render_display(ui, self, display_h);

                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    let toggle_label = if self.show_scientific { "▲ Basic" } else { "▼ Scientific" };
                    let toggle_btn = egui::Button::new(
                        egui::RichText::new(toggle_label)
                            .font(FontId::proportional(13.0))
                            .color(Color32::from_rgb(10, 132, 255)),
                    )
                    .fill(Color32::TRANSPARENT)
                    .frame(false);
                    if ui.add(toggle_btn).clicked() {
                        self.show_scientific = !self.show_scientific;
                    }

                    if self.show_scientific {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            let mode_label = if self.deg_mode { "DEG" } else { "RAD" };
                            let mode_btn = egui::Button::new(
                                egui::RichText::new(mode_label)
                                    .font(FontId::proportional(13.0))
                                    .color(Color32::from_rgb(255, 149, 0)),
                            )
                            .fill(Color32::TRANSPARENT)
                            .frame(false);
                            if ui.add(mode_btn).clicked() {
                                self.deg_mode = !self.deg_mode;
                            }
                        });
                    }
                });

                let button_area_h = ui.available_height();
                let row_gap = ui.spacing().item_spacing.y;
                let btn_h = (button_area_h - row_gap * (total_rows - 1.0)) / total_rows;
                let col_gap = 4.0_f32;
                let btn_w = (avail_w - col_gap * 3.0) / 4.0;

                if self.show_scientific {
                    sci_row(ui, btn_w, btn_h, col_gap, &[
                        ("sin", BtnKind::Sci), ("cos", BtnKind::Sci), ("tan", BtnKind::Sci), ("π", BtnKind::Sci),
                    ], self);
                    sci_row(ui, btn_w, btn_h, col_gap, &[
                        ("asin", BtnKind::Sci), ("acos", BtnKind::Sci), ("atan", BtnKind::Sci), ("e", BtnKind::Sci),
                    ], self);
                    sci_row(ui, btn_w, btn_h, col_gap, &[
                        ("x²", BtnKind::Sci), ("x³", BtnKind::Sci), ("√x", BtnKind::Sci), ("∛x", BtnKind::Sci),
                    ], self);
                    sci_row(ui, btn_w, btn_h, col_gap, &[
                        ("ln", BtnKind::Sci), ("log", BtnKind::Sci), ("1/x", BtnKind::Sci), ("x!", BtnKind::Sci),
                    ], self);
                    sci_row(ui, btn_w, btn_h, col_gap, &[
                        ("xʸ", BtnKind::Sci), ("ʸ√x", BtnKind::Sci), ("10ˣ", BtnKind::Sci), ("|x|", BtnKind::Sci),
                    ], self);
                }

                let paren_label = if self.paren_stack.is_empty() { "(" } else { ")" };
                basic_row(ui, btn_w, btn_h, col_gap, &[
                    ("AC", BtnKind::Action), (paren_label, BtnKind::Action), ("⌫", BtnKind::Action), ("÷", BtnKind::Op),
                ], self);
                basic_row(ui, btn_w, btn_h, col_gap, &[
                    ("7", BtnKind::Digit), ("8", BtnKind::Digit), ("9", BtnKind::Digit), ("×", BtnKind::Op),
                ], self);
                basic_row(ui, btn_w, btn_h, col_gap, &[
                    ("4", BtnKind::Digit), ("5", BtnKind::Digit), ("6", BtnKind::Digit), ("−", BtnKind::Op),
                ], self);
                basic_row(ui, btn_w, btn_h, col_gap, &[
                    ("1", BtnKind::Digit), ("2", BtnKind::Digit), ("3", BtnKind::Digit), ("+", BtnKind::Op),
                ], self);
                basic_row(ui, btn_w, btn_h, col_gap, &[
                    ("+/−", BtnKind::Action), ("0", BtnKind::Digit), (".", BtnKind::Digit), ("=", BtnKind::Equals),
                ], self);
            });
    }
}
