use std::{
    sync::{
        Arc, RwLock,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
};

use eframe::egui::{Button, CursorIcon, Image, ScrollArea, TextEdit, Ui, menu};

use crate::steam::{SteamApiClient, SteamStoreItem, StoreItemKind};

pub struct SearchSelect {
    term: String,
    results: Vec<SteamStoreItem>,
    is_loading: bool,
    sender: Sender<Vec<SteamStoreItem>>,
    receiver: Receiver<Vec<SteamStoreItem>>,
}

pub struct SearchSelectResponse {
    pub changed: bool,
}

impl SearchSelect {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            term: String::new(),
            results: vec![],
            is_loading: false,
            sender,
            receiver,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        steam: Arc<RwLock<SteamApiClient>>,
        selected: &mut Option<SteamStoreItem>,
        country_code: Option<String>,
        with_capsules: bool,
    ) -> SearchSelectResponse {
        let mut changed = false;

        if let Ok(results) = self.receiver.try_recv() {
            self.is_loading = false;
            self.results = results;
        }

        let selected_item_clone = selected.clone();
        let menu_ui = |ui: &mut Ui| {
            ui.set_width(400.);
            ui.vertical_centered_justified(|ui| {
                ui.heading("\u{1F50D} Steam store search");
                if ui
                    .add(
                        TextEdit::singleline(&mut self.term)
                            .hint_text("Type some name here...")
                            .char_limit(200),
                    )
                    .changed()
                    && !self.term.trim().is_empty()
                {
                    self.is_loading = true;
                    let sender = self.sender.clone();
                    let term = self.term.clone();
                    thread::spawn(move || {
                        let _ = sender.send(
                            steam
                                .read()
                                .unwrap()
                                .store_search(&term, country_code.as_deref())
                                .map(|res| {
                                    res.into_iter()
                                        .filter(|item| matches!(item.kind, StoreItemKind::App))
                                        .collect()
                                })
                                .unwrap_or_default(),
                        );
                    });
                }
                ui.separator();
                if self.is_loading {
                    ui.spinner();
                } else {
                    ui.label(format!("Results ({}):", self.results.len()));
                    ui.add_space(5.);
                    ScrollArea::vertical()
                        .min_scrolled_height(255.)
                        .show(ui, |ui| {
                            let i_last = self.results.len() - 1;
                            for (i, store_item) in self.results.iter().enumerate() {
                                if ui
                                    .add_enabled(store_item.price.is_some(), {
                                        let btn = if with_capsules {
                                            Button::image_and_text(
                                                Image::from_uri(&store_item.capsule_url)
                                                    .maintain_aspect_ratio(true)
                                                    .fit_to_original_size(0.5),
                                                &store_item.name,
                                            )
                                        } else {
                                            Button::new(&store_item.name)
                                        };
                                        btn.shortcut_text(match &store_item.price {
                                            Some(price) => format!(
                                                "{} {}",
                                                &price.currency,
                                                price.value_in_cents as f32 / 100.
                                            ),
                                            None => "Free".to_string(),
                                        })
                                        .selected(
                                            selected_item_clone
                                                .as_ref()
                                                .map(|a| a.id == store_item.id)
                                                .unwrap_or_default(),
                                        )
                                        .min_size(
                                            [46.2, if with_capsules { 47. } else { 24. }].into(),
                                        )
                                    })
                                    .on_disabled_hover_text("Free cannot be given away.")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    *selected = Some(store_item.clone());
                                    changed = true;
                                    ui.close_menu();
                                }
                                if i < i_last {
                                    ui.add_space(5.);
                                }
                            }
                        });
                }
            });
        };

        let select_menu_btn_response = menu::menu_custom_button(
            ui,
            if let Some(ref store_item) = selected_item_clone {
                if with_capsules {
                    Button::image(
                        Image::from_uri(&store_item.capsule_url)
                            .maintain_aspect_ratio(true)
                            .shrink_to_fit(),
                    )
                } else {
                    Button::new(&store_item.name)
                }
            } else {
                Button::new("...")
            }
            .truncate(),
            menu_ui,
        )
        .response;

        if let Some(app) = selected_item_clone {
            select_menu_btn_response.on_hover_ui(|ui| {
                ui.hyperlink_to(app.name, format!("steam://store/{}", app.id));
            });
        } else {
            select_menu_btn_response
                .on_hover_text_at_pointer("Search and select item from steam store.");
        }

        SearchSelectResponse { changed }
    }
}
