#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

extern crate calculator;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([400.0, 500.0]),
        centered: true,
        ..Default::default()
    };

    if let Err(err) = eframe::run_native("Calculator", options, Box::new(|_cc| Ok(Box::<calculator::MyApp>::default()))) {
        eprintln!("Error running the application: {:?}", err);
        std::process::exit(1);
    }

    Ok(())
}