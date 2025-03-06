use eframe::egui::{Align, Button, Context, Layout, OpenUrl, Window};
use egui_extras::{Column, TableBuilder};

use crate::app::App;

pub fn winners_window(app: &mut App, ctx: &Context) {
    Window::new(format!(
        "\u{1F3C6} Current winners ({})",
        app.winners.current.len()
    ))
    .open(&mut app.show_winners_window)
    .order(eframe::egui::Order::Middle)
    .show(ctx, |ui| {
        let mut table = TableBuilder::new(ui).striped(true);
        if app.preferences.avatars {
            table = table.column(Column::auto().at_least(40.));
        }
        table
            .column(Column::remainder().at_least(200.))
            .column(Column::auto().at_least(60.))
            .column(Column::auto())
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
                if !app.winners.current.is_empty() {
                    body.rows(32., app.winners.current.len(), |mut row| {
                        let friend = &app.winners.current[row.index()];
                        friend.add_to_table_row(&app.preferences, &app.winners, ctx, &mut row);
                        row.col(|ui| {
                            let app_for_giveaway_is_some = app.store_item_for_giveaway.is_some();
                            if ui
                                .add_enabled(
                                    app_for_giveaway_is_some,
                                    Button::new("\u{1F381} Send gift"),
                                )
                                .on_disabled_hover_text("Need to select app for giveaway first!")
                                .on_hover_text_at_pointer(if app_for_giveaway_is_some {
                                    "Copy nickname and open purchase page in steam"
                                } else {
                                    "App for giveaway is not specified!"
                                })
                                .clicked()
                            {
                                ctx.copy_text(friend.name.clone());
                                ctx.open_url(OpenUrl::new_tab(format!(
                                    "steam://purchase/{}",
                                    app.store_item_for_giveaway.as_ref().unwrap().id
                                )));
                            }
                        });
                    });
                }
            });
    });
}
