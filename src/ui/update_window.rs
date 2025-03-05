use eframe::egui::{Align2, Context, Vec2, Window};

use crate::app::App;

pub fn update_window(app: &mut App, ctx: &Context) {
    Window::new("New version available!")
        .id("update_window".into())
        .open(&mut app.show_update_window)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .auto_sized()
        .show(ctx, |ui| {
            let latest_release_url = env!("CARGO_PKG_REPOSITORY").to_owned() + "/releases/latest";
            ui.hyperlink_to(&latest_release_url, &latest_release_url);
        });
}
