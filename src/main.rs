#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState, MouseState};
use eframe::egui;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut old_button_pressed = true;
        /*
         * So... first, we cannot do this in the app loop as it won't register clicks outside of
         * the app window. Then two methods were tested, this naive polling method using
         * device_query and a blocking event listening method using rdev. But because the event
         * listening receives event like mouse move, it actually hogs the CPU when the user moves
         * his cursor a little bit too much, so I went with the code bellow:
         */
        loop {
            let mouse: MouseState = device_state.get_mouse();
            if mouse.button_pressed[1] && !old_button_pressed {
                let _ = tx.send(mouse.coords);
            }
            old_button_pressed = mouse.button_pressed[1];
            thread::sleep(Duration::from_millis(10));
        }
    });

    eframe::run_native(
        "RizzBot",
        options,
        Box::new(|cc| Ok(Box::new(RizzBotApp::new(cc, rx)))),
    )
}

struct RizzBotApp {
    calibrated: bool,
    rx: mpsc::Receiver<(i32, i32)>,
    calibration_points: Vec<(i32, i32)>,
}

impl RizzBotApp {
    fn new(_cc: &eframe::CreationContext<'_>, rx: mpsc::Receiver<(i32, i32)>) -> Self {
        Self {
            calibrated: false,
            rx,
            calibration_points: Vec::new(),
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
                // The code bellow isn't great as this blocks rendering until user finishes
                // calibration
                self.calibration_points.clear();
                while let Ok(_) = self.rx.try_recv() {} // Drain the queue

                while self.calibration_points.len() < 4 {
                    if let Ok(coords) = self.rx.recv() {
                        self.calibration_points.push(coords);
                    }
                }
                self.calibrated = true;
            }

            if self.calibrated {
                for point in &self.calibration_points {
                    ui.label(format!("Point: ({}, {})", point.0, point.1));
                }
            }
        });
    }
}
