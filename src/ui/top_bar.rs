use eframe::egui::{
    Align, Color32, Context, CursorIcon, Image, Layout, OpenUrl, RichText, Sense, Stroke,
    TopBottomPanel, ViewportCommand, global_theme_preference_switch, menu,
};

use crate::{app::App, steam::TokenError};

pub fn top_bar(app: &mut App, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.scope(|ui| {
                    if app.steam.read().unwrap().access_token.info.is_err() {
                        let styles = ui.style_mut();
                        let stroke = Stroke {
                            width: 1.,
                            color: styles.visuals.error_fg_color,
                        };
                        styles.visuals.widgets.inactive.bg_stroke = stroke;
                        styles.visuals.widgets.hovered.bg_stroke = stroke;
                    }
                    ui.menu_button("Menu", |ui| {
                        ui.scope(|ui| {
                            if app.steam.read().unwrap().access_token.info.is_err() {
                                let styles = ui.style_mut();
                                let stroke = Stroke {
                                    width: 1.,
                                    color: styles.visuals.error_fg_color,
                                };
                                styles.visuals.widgets.inactive.bg_stroke = stroke;
                                styles.visuals.widgets.hovered.bg_stroke = stroke;
                            }
                            if ui.button("\u{2699} Settings").clicked() {
                                app.show_settings_window = true;
                                ui.close_menu();
                            }
                        });
                        if ui.button("\u{1F419} Github").clicked() {
                            ctx.open_url(OpenUrl::new_tab(env!("CARGO_PKG_REPOSITORY")));
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                });
                ui.separator();
                global_theme_preference_switch(ui);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match &app.steam.read().unwrap().access_token.info {
                        Ok(token_info) => {
                            if let Some(current_user) = &app.steam.read().unwrap().current_user {
                                if ui
                                    .label(RichText::new(&current_user.name).color(
                                        if ui.style().visuals.dark_mode {
                                            Color32::LIGHT_GREEN
                                        } else {
                                            Color32::DARK_GREEN
                                        },
                                    ))
                                    .interact(Sense::click())
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .on_hover_text_at_pointer("Open Steam Profile")
                                    .clicked()
                                {
                                    ctx.open_url(OpenUrl::new_tab(format!(
                                        "steam://url/SteamIDPage/{}",
                                        current_user.id
                                    )));
                                }
                                if app.preferences.avatars {
                                    ui.add(
                                        Image::from_uri(&current_user.avatar_url).corner_radius(32),
                                    );
                                }
                            } else if ui
                                .label(RichText::new(token_info.user_id.to_string()).color(
                                    if ui.style().visuals.dark_mode {
                                        Color32::LIGHT_GREEN
                                    } else {
                                        Color32::DARK_GREEN
                                    },
                                ))
                                .interact(Sense::click())
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .on_hover_text_at_pointer("Open Steam Profile")
                                .clicked()
                            {
                                ctx.open_url(OpenUrl::new_tab(format!(
                                    "steam://url/SteamIDPage/{}",
                                    token_info.user_id
                                )));
                            }
                        }
                        Err(err) => {
                            ui.colored_label(
                                ui.style().visuals.error_fg_color,
                                "\u{26A0} Access token ".to_owned()
                                    + match err {
                                        TokenError::EmptyString => "is not specified",
                                        TokenError::ParseFailed => "is invalid",
                                        TokenError::Expired => "is expired",
                                    },
                            );
                        }
                    }
                });
            });
        });
    });
}
