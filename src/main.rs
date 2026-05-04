mod app;
mod calculator;
mod ui;

use calculator::Calculator;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Scientific Calculator")
            .with_inner_size([420.0, 620.0])
            .with_min_inner_size([360.0, 520.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Scientific Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(Calculator::default()))),
    )
}
