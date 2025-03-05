use chrono::Local;
use eframe::egui::{Align, Align2, Color32, Context, Layout, TextEdit, Vec2, Window};

use crate::{
    app::{App, Msg},
    steam::TokenError,
    utils::ui_with_space_before_and_after,
};

pub fn settings_window(app: &mut App, ctx: &Context) {
    Window::new("\u{2699} Settings")
        .open(&mut app.show_settings_window)
        .order(eframe::egui::Order::Foreground)
        .max_width(400.)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_max_width(400.);
            ui.heading("Credentials");
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Access token");
                    match &app.steam.read().unwrap().access_token.info {
                        Ok(info) => {
                            ui.colored_label(
                                if ui.style().visuals.dark_mode {
                                    Color32::LIGHT_GREEN
                                } else {
                                    Color32::DARK_GREEN
                                },
                                format!(
                                    "expires on {}",
                                    info.expires_on.with_timezone(&Local).format("%b %d, %Y, %H:%M")
                                ),
                            );
                        }
                        Err(err) => {
                            ui.colored_label(
                                ui.style().visuals.error_fg_color,
                                match err {
                                    TokenError::EmptyString => "is not specified",
                                    TokenError::ParseFailed => "is invalid",
                                    TokenError::Expired => "is expired",
                                }.to_owned() + " \u{26A0}",
                            );
                        }
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let button = ui.link("\u{2139} How to get it");
                        button.on_hover_ui(|ui| {
                            ui.heading("How to get a store token:");
                            ui.horizontal(|ui| {
                                ui.label("1. Log in on");
                                ui.hyperlink("https://store.steampowered.com/login");
                            });
                            ui.horizontal(|ui| {
                                ui.label("2. Open");
                                ui.hyperlink("https://store.steampowered.com/pointssummary/ajaxgetasyncconfig");
                            });
                            ui.horizontal(|ui| {
                                ui.label("3. Copy the value of");
                                ui.monospace("webapi_token");
                            });
                        });
                    });
                });
                if ui
                    .add(TextEdit::multiline(&mut app.steam_access_token_buffer)
                        .min_size([ui.available_width(), 20.].into())
                        .char_limit(1024)
                        .password(true)
                        .hint_text("Type your token here..."))
                    .changed()
                {
                    app.steam.write().unwrap().set_access_token(&app.steam_access_token_buffer);
                    let _ = app.sender.send(Msg::AccessTokenSetted);
                }

            });

            ui_with_space_before_and_after(ui, |ui| {
                ui.separator();
            });

            ui.heading("Preferences");
            ui.checkbox(&mut app.preferences.avatars, "Avatars");
            ui.checkbox(&mut app.preferences.flags_icons, "Flag icons");
            ui.checkbox(&mut app.preferences.store_items_capsules, "Apps banners");

            ui_with_space_before_and_after(ui, |ui| {
                ui.separator();
            });

            ui.heading("Other");
            if ui.button("Clear cached images").clicked() {
                ctx.forget_all_images();
            }
            if ui.button("Clear winners for all time").clicked() {
                app.winners.all_time = Default::default();
            }
        });
}
