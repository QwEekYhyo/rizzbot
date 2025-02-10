#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RizzBot",
        options,
        Box::new(|cc| Ok(Box::new(RizzBotApp::new(cc)))),
    )
}

struct RizzBotApp {
    calibrated: bool,
}

impl RizzBotApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for RizzBotApp {
    fn default() -> Self {
        Self {
            calibrated: false,
        }
    }
}

impl eframe::App for RizzBotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to RizzBot");
            if self.calibrated {
                ui.label("All good");
            } else {
                ui.label("RizzBot is not calibrated, please calibrate");
            }
            if ui.button("Calibrate").clicked() {
                if !self.calibrated {
                    self.calibrated = true;
                }
            }
        });
    }
}
