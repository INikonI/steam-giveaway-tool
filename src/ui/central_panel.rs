use eframe::egui::{Align, CentralPanel, Context, Layout, TextEdit};
use egui_extras::{Column, TableBuilder};

use crate::{
    app::{App, Msg},
    utils::pluralize,
};

pub const FRIENDS_PER_PAGE: usize = 100;

pub fn central_panel(app: &mut App, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.set_min_width(250.);
        ui.add_enabled_ui(!app.friends.all.is_empty(), |ui| {
            ui.horizontal(|ui| {
                ui.label("\u{1F50D}");
                if ui
                    .add(
                        TextEdit::singleline(&mut app.friends_search_name)
                            .char_limit(100)
                            .hint_text("Search friends by name..."),
                    )
                    .changed()
                {
                    let _ = app.sender.send(Msg::UpdateFoundedFriends);
                }
                if !app.friends_search_results.is_empty() {
                    ui.label(pluralize("result", app.friends_search_results.len()));
                }
            });
        })
        .response
        .on_disabled_hover_text("No friends to search.");
        ui.add_space(7.);
        let mut table = TableBuilder::new(ui).striped(true).auto_shrink(false);
        if app.preferences.avatars {
            table = table.column(Column::auto().at_least(40.));
        }
        table
            .column(Column::remainder().at_least(200.))
            .column(Column::auto().at_least(60.))
            .column(Column::auto())
            .column(Column::auto())
            .cell_layout(Layout::left_to_right(Align::Center))
            .header(20.0, |mut header| {
                if app.preferences.avatars {
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(8.);
                            ui.heading("\u{1F5BC}");
                        });
                    });
                }
                header.col(|ui| {
                    ui.heading("\u{1F3F7} Name");
                });
                header.col(|ui| {
                    ui.heading("\u{1F30D}")
                        .on_hover_text_at_pointer("Region in profile");
                });
                header.col(|ui| {
                    ui.add_space(10.);
                    ui.heading("\u{1F4C5} Age")
                        .on_hover_text_at_pointer("How old is the account in years");
                    ui.add_space(10.);
                });
                header.col(|ui| {
                    ui.heading("\u{1F3C6} Won");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 32.;

                if app.friends.all.is_empty() {
                    return;
                }

                if !app.friends_search_name.is_empty() {
                    body.rows(ROW_HEIGHT, app.friends_search_results.len(), |mut row| {
                        let friend = &app.friends_search_results[row.index()];
                        friend.add_to_table_row(&app.preferences, &app.winners, ctx, &mut row);
                    });
                    return;
                }

                app.friends.update_filtered(
                    &app.filters,
                    &app.winners,
                    app.store_item_for_giveaway
                        .as_ref()
                        .map(|app| app.user_details.as_ref())
                        .unwrap_or_default(),
                );
                let total_pages = app.friends.filtered.len().div_ceil(FRIENDS_PER_PAGE).max(1);
                app.main_current_page = app.main_current_page.min(total_pages);

                let start = (app.main_current_page - 1) * FRIENDS_PER_PAGE;
                let end = (start + FRIENDS_PER_PAGE).min(app.friends.filtered.len());

                let filtered_friends_page = &app.friends.filtered[start..end];
                body.rows(ROW_HEIGHT, filtered_friends_page.len(), |mut row| {
                    let friend = &filtered_friends_page[row.index()];
                    friend.add_to_table_row(&app.preferences, &app.winners, ctx, &mut row);
                });
            });
    });
}
