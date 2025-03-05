use eframe::egui::{
    Align, Button, Checkbox, Context, Direction, DragValue, Layout, ProgressBar, RichText,
    ScrollArea, SidePanel, TopBottomPanel, menu::menu_custom_button,
};
use egui_extras::{Column, TableBuilder};
use std::thread;

use crate::{
    app::{App, Msg, RegionFilter},
    utils::ui_with_space_before_and_after,
};

pub fn side_panel(app: &mut App, ctx: &Context) {
    SidePanel::right("side_panel")
        .resizable(false)
        .max_width(270.)
        .show(ctx, |ui| {
            if app.steam.read().unwrap().access_token.info.is_err() {
                ui.disable();
            }
            TopBottomPanel::top("friends_list_reload_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui_with_space_before_and_after(ui, |ui| {
                        ui.heading("Friends");
                        ui.vertical_centered_justified(|ui| {
                            ui.add_enabled_ui(!app.friends.is_loading, |ui| {
                                if ui
                                    .button(RichText::new("\u{2B07} Reload").heading())
                                    .on_hover_text_at_pointer(
                                        "Redownloades friends list.\nResets region filters.\nResets friends search.\nResets has store item filters.",
                                    )
                                    .clicked()
                                {
                                    app.friends.update(app.steam.clone(), app.sender.clone());
                                }
                            });
                            if app.friends.is_loading {
                                ui.add(
                                    ProgressBar::new(app.friends.loading_progress)
                                        .desired_height(6.)
                                        .corner_radius(1)
                                        .fill(ui.style().visuals.strong_text_color()),
                                );
                            }
                        });
                    });
                });
            const ROW_HEIGHT: f32 = 18.;
            TopBottomPanel::top("winners_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui_with_space_before_and_after(ui, |ui| {
                        ui.heading("Winners");
                        ui.vertical_centered_justified(|ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 2.0;
                                if ui
                                    .add_enabled(
                                        app.winners.next_number > 1,
                                        Button::new(RichText::new("\u{2796}").monospace()),
                                    )
                                    .clicked()
                                {
                                    app.winners.next_number -= 1;
                                }
                                let max_winners = app.friends.filtered.len().max(1);
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui
                                        .add_enabled(
                                            app.winners.next_number < max_winners,
                                            Button::new(RichText::new("\u{2795}").monospace()),
                                        )
                                        .clicked()
                                    {
                                        app.winners.next_number += 1;
                                    }
                                    ui.add_sized(
                                        [ui.available_width(), ROW_HEIGHT],
                                        DragValue::new(&mut app.winners.next_number)
                                            .prefix("Count: ")
                                            .range(1..=max_winners),
                                    )
                                    .on_hover_text_at_pointer("Number of next winners");
                                });
                            });

                            if ui.button("\u{1F3B2} Random new").clicked() {
                                app.winners.update_current(&app.friends);
                                app.show_winners_window = true;
                            }
                            if ui.button("Show").clicked() {
                                app.show_winners_window = true;
                            }
                            ui.horizontal(|ui| {
                                ui.scope(|ui| {
                                    ui.set_width(ui.available_width() / 2.);
                                    ui.vertical_centered_justified(|ui| {
                                        if ui.add_enabled(
                                            !app.winners.auto_save_current && !app.winners.saved,
                                            Button::new(if app.winners.saved { "Saved" } else { "Save current" })
                                        )
                                        .on_hover_text("Saves winners and how many times they won.\n You can clear them in settings.")
                                        .clicked()
                                        {
                                            app.winners.save_current();
                                        }
                                    });
                                });
                                ui.vertical_centered_justified(|ui| {
                                    ui.checkbox(&mut app.winners.auto_save_current, "Auto save");
                                    if app.winners.auto_save_current && !app.winners.saved {
                                        app.winners.save_current();
                                    }
                                });
                            });
                        });
                    });
                });

            TopBottomPanel::top("store_item_for_giveaway_panel")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui_with_space_before_and_after(ui, |ui| {
                        ui.heading("Store item for giveaway");
                        ui.vertical_centered_justified(|ui| {
                            let r = app.search_select.show(
                                ui,
                                app.steam.clone(),
                                &mut app.store_item_for_giveaway,
                                app.steam
                                    .read()
                                    .unwrap()
                                    .current_user
                                    .as_ref()
                                    .map(|u| u.country_code.clone())
                                    .unwrap_or_default(),
                                app.preferences.store_items_capsules,
                            );
                            if r.changed {
                                app.app_for_giveaway_user_details_is_loading = true;
                                let steam = app.steam.clone();
                                let sender = app.sender.clone();
                                let app_id = app.store_item_for_giveaway.as_ref().unwrap().id;
                                thread::spawn(move || {
                                    let app_user_details = steam
                                        .read()
                                        .unwrap()
                                        .app_user_details(&[app_id])
                                        .unwrap()
                                        .into_values()
                                        .next()
                                        .unwrap()
                                        .unwrap();
                                    let _ = sender.send(Msg::UpdateUserDetailsOfAppForGiveaway(
                                        app_user_details,
                                    ));
                                });
                            }
                        });
                    });
                });

            TopBottomPanel::top("filters_panel")
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui_with_space_before_and_after(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Filters");
                            if ui.button("Reset").clicked() {
                                app.filters.reset(&app.friends);
                            }
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.add_enabled(
                                app.store_item_for_giveaway
                                    .as_ref()
                                    .map(|app| app.user_details.is_some())
                                    .unwrap_or_default(),
                                Checkbox::new(
                                    &mut app.filters.include_who_has_app_in_wishlist,
                                    "Only who has app in wishlist",
                                ),
                            )
                            .on_disabled_hover_text("User details is not loaded.");
                            if app.app_for_giveaway_user_details_is_loading {
                                ui.spinner()
                                    .on_hover_text_at_pointer("Downloading user details...");
                            }
                        });
                        ui.checkbox(
                            &mut app.filters.exclude_who_won_before,
                            "Exclude who won earlier",
                        );
                    });
                });

            TopBottomPanel::top("region_filters_panel")
                .resizable(false)
                .exact_height(ui.available_height() / 2. - 8.)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui_with_space_before_and_after(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Regions");
                            ui.style_mut().spacing.button_padding = [4.0, 2.0].into();
                            menu_custom_button(ui, Button::new("\u{2795}").small(), |ui| {
                                ScrollArea::vertical().min_scrolled_height(160.).show(ui, |ui| {
                                    if matches!(
                                        app.filters.regions_and_countries.unknown,
                                        RegionFilter::Available
                                    ) {
                                        ui.colored_label(
                                            ui.style().visuals.weak_text_color(),
                                            "Other",
                                        );
                                        ui.menu_button("Unknown", |ui| {
                                            if ui.button("Include").clicked() {
                                                app.filters.regions_and_countries.unknown =
                                                    RegionFilter::Include;
                                            }
                                            if ui.button("Exclude").clicked() {
                                                app.filters.regions_and_countries.unknown =
                                                    RegionFilter::Exclude;
                                            }
                                        });
                                        ui.separator();
                                    }
                                    ui.colored_label(
                                        ui.style().visuals.weak_text_color(),
                                        "Regions",
                                    );
                                    let cis_available = matches!(
                                        app.filters.regions_and_countries.cis,
                                        RegionFilter::Available
                                    );
                                    if cis_available {
                                        ui.menu_button("CIS", |ui| {
                                            if ui.button("Include").clicked() {
                                                app.filters.regions_and_countries.cis =
                                                    RegionFilter::Include;
                                            }
                                            if ui.button("Exclude").clicked() {
                                                app.filters.regions_and_countries.cis =
                                                    RegionFilter::Exclude;
                                            }
                                        });
                                    }
                                    let eu_available = matches!(
                                        app.filters.regions_and_countries.eu,
                                        RegionFilter::Available
                                    );
                                    if eu_available {
                                        ui.menu_button("EU", |ui| {
                                            if ui.button("Include").clicked() {
                                                app.filters.regions_and_countries.eu =
                                                    RegionFilter::Include;
                                            }
                                            if ui.button("Exclude").clicked() {
                                                app.filters.regions_and_countries.eu =
                                                    RegionFilter::Exclude;
                                            }
                                        });
                                    }
                                    ui.separator();
                                    ui.colored_label(
                                        ui.style().visuals.weak_text_color(),
                                        "Countries",
                                    );
                                    app.filters.regions_and_countries.available_countries.sort();
                                    app.filters
                                        .regions_and_countries
                                        .available_countries
                                        .retain_mut(|available_region| {
                                            let mut retain = true;
                                            ui.menu_button(&*available_region, |ui| {
                                                if ui.button("Include").clicked() {
                                                    app.filters
                                                        .regions_and_countries
                                                        .include_countries
                                                        .push(available_region.clone());
                                                    retain = false
                                                }
                                                if ui.button("Exclude").clicked() {
                                                    app.filters
                                                        .regions_and_countries
                                                        .exclude_countries
                                                        .push(available_region.clone());
                                                    retain = false;
                                                }
                                            });
                                            retain
                                        });
                                });
                            });
                            if ui.small_button("Reset").clicked() {
                                app.filters.reset_regions_and_countries(&app.friends);
                            }
                        });
                        ui.horizontal_top(|ui| {
                            const HEADER_HEIGHT: f32 = 20.;
                            let table_height = ui.available_height() - 8.;
                            ui.vertical(|ui| {
                                ui.set_width(ui.available_width() / 2.);
                                TableBuilder::new(ui)
                                    .id_salt("include_table")
                                    .striped(true)
                                    .auto_shrink(true)
                                    .stick_to_bottom(true)
                                    .min_scrolled_height(table_height)
                                    .max_scroll_height(table_height)
                                    .cell_layout(Layout::left_to_right(Align::Center))
                                    .column(Column::remainder().at_least(50.))
                                    .column(Column::auto().at_least(20.))
                                    .header(HEADER_HEIGHT, |mut row| {
                                        row.col(|ui| {
                                            ui.label("Include");
                                        });
                                    })
                                    .body(|mut body| {
                                        if matches!(
                                            app.filters.regions_and_countries.unknown,
                                            RegionFilter::Include
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("Unknown");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.unknown =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        if matches!(
                                            app.filters.regions_and_countries.cis,
                                            RegionFilter::Include
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("CIS");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.cis =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        if matches!(
                                            app.filters.regions_and_countries.eu,
                                            RegionFilter::Include
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("EU");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.eu =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        app.filters.regions_and_countries.include_countries.retain(
                                            |region_or_country| {
                                                let mut retain = true;
                                                body.row(ROW_HEIGHT, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(region_or_country.to_string());
                                                    });
                                                    row.col(|ui| {
                                                        ui.style_mut().visuals.button_frame = false;
                                                        if ui.small_button("\u{2796}").clicked() {
                                                            app.filters
                                                                .regions_and_countries
                                                                .available_countries
                                                                .push(region_or_country.clone());
                                                            retain = false;
                                                        }
                                                    });
                                                });
                                                retain
                                            },
                                        );
                                    });
                            });
                            ui.vertical(|ui| {
                                ui.set_width(ui.available_width());
                                 TableBuilder::new(ui)
                                    .id_salt("exclude_table")
                                    .striped(true)
                                    .stick_to_bottom(true)
                                    .min_scrolled_height(table_height)
                                    .max_scroll_height(table_height)
                                    .cell_layout(Layout::left_to_right(Align::Center))
                                    .column(Column::remainder().at_least(50.))
                                    .column(Column::auto().at_least(20.))
                                    .header(HEADER_HEIGHT, |mut row| {
                                        row.col(|ui| {
                                            ui.label("Exclude");
                                        });
                                    })
                                    .body(|mut body| {
                                        if matches!(
                                            app.filters.regions_and_countries.unknown,
                                            RegionFilter::Exclude
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("Unknown");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.unknown =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        if matches!(
                                            app.filters.regions_and_countries.cis,
                                            RegionFilter::Exclude
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("CIS");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.cis =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        if matches!(
                                            app.filters.regions_and_countries.eu,
                                            RegionFilter::Exclude
                                        ) {
                                            body.row(ROW_HEIGHT, |mut row| {
                                                row.col(|ui| {
                                                    ui.label("EU");
                                                });
                                                row.col(|ui| {
                                                    ui.style_mut().visuals.button_frame = false;
                                                    if ui.small_button("\u{2796}").clicked() {
                                                        app.filters.regions_and_countries.eu =
                                                            RegionFilter::Available;
                                                    }
                                                });
                                            });
                                        }
                                        app.filters.regions_and_countries.exclude_countries.retain(
                                            |region_or_country| {
                                                let mut retain = true;
                                                body.row(ROW_HEIGHT, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(region_or_country.to_string());
                                                    });
                                                    row.col(|ui| {
                                                        ui.style_mut().visuals.button_frame = false;
                                                        if ui.small_button("\u{2796}").clicked() {
                                                            app.filters
                                                                .regions_and_countries
                                                                .available_countries
                                                                .push(region_or_country.clone());
                                                            retain = false;
                                                        }
                                                    });
                                                });
                                                retain
                                            },
                                        );
                                    });
                            });
                        });
                    });
                });
                TopBottomPanel::top("has_store_item_filter_panel")
                    .exact_height(ui.available_height())
                    .resizable(false)
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Has store items");
                                if ui.small_button("\u{2795}").clicked() {
                                    app.filters.has_store_items.push(Default::default());
                                }
                                if ui.small_button("Reset").clicked() {
                                    app.filters.has_store_items.clear();
                                }
                            });
                            let table_height = ui.available_height() - 25.;
                            TableBuilder::new(ui)
                                .striped(true)
                                .auto_shrink(true)
                                .stick_to_bottom(true)
                                .min_scrolled_height(table_height)
                                .max_scroll_height(table_height)
                                .column(Column::exact(50.))
                                .column(Column::exact(80.))
                                .column(Column::exact(65.))
                                .column(Column::exact(20.))
                                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                                .header(22., |mut row| {
                                    row.col(|ui| {
                                        ui.label("Item");
                                    });
                                    row.col(|ui| {
                                        ui.label("\u{23F3} Total").on_hover_text("Total playtime for all-time");
                                    });
                                    row.col(|ui| {
                                        ui.label("\u{23F3} 2 weeks").on_hover_text("Playtime for last 2 weeks");
                                    });
                                })
                                .body(|mut body| {
                                    app.filters.has_store_items.dedup();
                                    app.filters.has_store_items.retain_mut(|filter| {
                                        let mut retain = true;
                                        body.row(18., |mut row| {
                                            row.col(|ui| {
                                                // ui.set_max_height(ui.available_height() - 2.);
                                                if filter.is_loading {
                                                    ui.spinner().on_hover_text_at_pointer("Downloading user details...");
                                                    return;
                                                }
                                                let select_app_response = app.search_select.show(
                                                    ui,
                                                    app.steam.clone(),
                                                    &mut filter.app,
                                                    app.steam
                                                        .read()
                                                        .unwrap()
                                                        .current_user
                                                        .as_ref()
                                                        .map(|u| u.country_code.clone())
                                                        .unwrap_or_default(),
                                                    app.preferences.store_items_capsules,
                                                );
                                                if select_app_response.changed {
                                                    filter.is_loading = true;
                                                    let steam = app.steam.clone();
                                                    let sender = app.sender.clone();
                                                    let app_id = filter.app.as_ref().unwrap().id;
                                                    thread::spawn(move || {
                                                        if let Ok(mut apps_user_details) = steam.read().unwrap().app_user_details(&[app_id]) {
                                                            if let Some(app_user_details) = apps_user_details.get_mut(&app_id).map(|opt| opt.take()).unwrap_or_default() {
                                                                let _ = sender.send(Msg::UpdateUserDetailsOfHasAppFilter(app_id, app_user_details));
                                                            }
                                                        }
                                                    });
                                                }
                                            });
                                            row.col(|ui| {
                                                ui.add(DragValue::new(&mut filter.playtime_total).range(0..=999_999).prefix(">= ").suffix(" h."));
                                            });
                                            row.col(|ui| {
                                                ui.add(DragValue::new(&mut filter.playtime_twoweeks).range(0..=336).prefix(">= ").suffix(" h."));
                                            });
                                            row.col(|ui| {
                                                ui.style_mut().visuals.button_frame = false;
                                                if ui.small_button("\u{2796}").clicked() {
                                                    retain = false;
                                                }
                                                ui.add_space(5.);
                                            });
                                        });
                                        retain
                                    });
                                })
                    });
        });
}
