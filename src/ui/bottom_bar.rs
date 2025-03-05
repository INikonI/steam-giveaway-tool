use eframe::egui::{Align, Context, DragValue, Layout, TopBottomPanel};

use crate::{app::App, utils::pluralize};

use super::FRIENDS_PER_PAGE;

pub fn bottom_bar(app: &mut App, ctx: &Context) {
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.with_layout(
            Layout::left_to_right(Align::Center).with_main_align(Align::Center),
            |ui| {
                if app.friends_search_name.is_empty() {
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.;
                        if !app.friends.all.is_empty() {
                            let total_pages =
                                app.friends.filtered.len().div_ceil(FRIENDS_PER_PAGE).max(1);
                            ui.add_enabled_ui(app.main_current_page > 1, |ui| {
                                if ui.small_button("<").clicked() {
                                    app.main_current_page -= 1;
                                }
                            });
                            ui.add(
                                DragValue::new(&mut app.main_current_page)
                                    .range(1..=total_pages.max(1))
                                    .prefix("Page: ")
                                    .suffix(format!("/{}", total_pages)),
                            );
                            ui.add_enabled_ui(app.main_current_page < total_pages, |ui| {
                                if ui.small_button(">").clicked() {
                                    app.main_current_page += 1;
                                }
                            });
                        }
                    });
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let friends_all_count = app.friends.all.len();
                    ui.label(if friends_all_count > 0 {
                        let mut txt =
                            format!("\u{1F465} {}", pluralize("friend", friends_all_count));
                        let friends_filtered_count = app.friends.filtered.len();
                        if friends_all_count != friends_filtered_count {
                            txt += &format!(" ({} filtered)", friends_filtered_count);
                        }
                        txt
                    } else {
                        "\u{1F465} friends not loaded".to_string()
                    });
                });
            },
        );
    });
}
