#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod ui;

use eframe::{ egui::vec2, run_native, Error };
use app::ScratchNotesApp;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    run_native(
        "scratchnotes",
        options,
        Box::new(|cc| Box::new(ScratchNotesApp::new(cc))),
    )
}

