use eframe::egui::{Context, CursorIcon, Image, OpenUrl, Sense};
use egui_extras::TableRow;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use std::{fmt::Display, ops::Deref};

use crate::{
    app::{Preferences, Winners},
    utils::pluralize,
};

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct SteamId(#[serde(deserialize_with = "deserialize_u64")] pub u64);

impl Display for SteamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SteamId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for SteamId
where
    <SteamId as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<SteamId> for u64 {
    fn from(val: SteamId) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamUser {
    #[serde(rename = "steamid")]
    pub id: SteamId,
    #[serde(rename = "personaname")]
    pub name: String,

    #[serde(rename = "avatarmedium")]
    pub avatar_url: String,

    /// ISO3166-2 code. For current user - real, for others - profile
    #[serde(rename = "loccountrycode")]
    pub country_code: Option<String>,
}

impl SteamUser {
    pub fn add_to_table_row(
        &self,
        preferences: &Preferences,
        winners: &Winners,
        ctx: &Context,
        row: &mut TableRow,
    ) {
        if preferences.avatars {
            row.col(|ui| {
                let avatar_ui = ui
                    .image(&self.avatar_url)
                    .interact(Sense::click())
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .on_hover_text_at_pointer("Open Steam profile");
                if avatar_ui.clicked() {
                    ctx.open_url(OpenUrl::new_tab(format!(
                        "steam://url/SteamIDPage/{}",
                        self.id
                    )));
                }
            });
        }
        row.col(|ui| {
            ui.horizontal_centered(|ui| {
                let name_ui = ui
                    .label(&self.name)
                    .on_hover_cursor(CursorIcon::Copy)
                    .on_hover_text_at_pointer("Copy name to clipboard");
                if name_ui.clicked() {
                    ctx.copy_text(self.name.clone());
                }
            });
        });
        row.col(|ui| {
            if let Some(ref country_code) = self.country_code {
                if preferences.flags_icons {
                    ui.add(
                        Image::new(flagcdn::flag_url(
                            flagcdn::size::FixedHeight::S,
                            country_code,
                            flagcdn::Format::JPEG,
                        ))
                        .maintain_aspect_ratio(false)
                        .max_size([18., 13.].into()),
                    )
                    .on_hover_text_at_pointer(country_code);
                } else {
                    ui.label(country_code);
                }
            } else {
                ui.add_space(2.);
                ui.label("\u{2753}");
            }
        });
        row.col(|ui| {
            ui.label(if let Some(times) = winners.all_time.get(&self.id) {
                pluralize("time", *times)
            } else {
                "Never".to_owned()
            });
        });
    }
}

/// deserialize number or string to u64
fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Number(num) => num.as_u64().ok_or_else(|| {
            de::Error::invalid_type(
                de::Unexpected::Other("large or negative number"),
                &"a u64 or a stringifed u64",
            )
        }),
        Value::String(s) => s.parse::<u64>().map_err(de::Error::custom),
        _ => Err(de::Error::invalid_type(
            de::Unexpected::Other("unsupported type"),
            &"a u64 or a stringifed u64",
        )),
    }
}
