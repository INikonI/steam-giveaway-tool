use eframe::egui::Context;

use crate::app::App;

use super::{bottom_bar, central_panel, side_panel, top_bar};

pub fn main_window(app: &mut App, ctx: &Context) {
    top_bar(app, ctx);
    side_panel(app, ctx);
    bottom_bar(app, ctx);
    central_panel(app, ctx);
}
