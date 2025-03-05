#![cfg_attr(windows, windows_subsystem = "windows")]

mod app;
mod steam;
mod ui;
mod utils;

use app::App;
use utils::icon_from_bytes;

const APP_NAME: &str = "Steam Giveaway Tool";

fn main() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 670.0])
            .with_min_inner_size([800.0, 670.0])
            .with_resizable(true)
            .with_icon(
                icon_from_bytes(include_bytes!("../assets/icons/icon.png"))
                    .expect("Icon should be loaded"),
            ),
        ..Default::default()
    };

    eframe::run_native(APP_NAME, options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
        .expect("Failed to run native app");
}
