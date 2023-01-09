#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut win_options = eframe::NativeOptions::default();
    win_options.initial_window_size = Some(egui::vec2(800.0, 600.0));
    eframe::run_native(
        "Register App",
        win_options,
        Box::new(|cc| Box::new(register_app::RegisterApp::new(cc))),
    );
}