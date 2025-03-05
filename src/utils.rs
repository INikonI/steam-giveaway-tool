use std::io::Cursor;

use eframe::egui::{IconData, Ui};
use image::ImageReader;

#[inline]
pub fn pluralize(word: &str, count: usize) -> String {
    if count % 10 == 1 && count % 100 != 11 {
        format!("{count} {word}")
    } else {
        format!("{count} {word}s")
    }
}

#[inline]
pub fn ui_with_space_before_and_after<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
    ui.add_space(ui.style().spacing.item_spacing.y * 2.);
    ui.scope(add_contents);
    ui.add_space(ui.style().spacing.item_spacing.y * 2.);
}

pub fn icon_from_bytes(bytes: &[u8]) -> Option<IconData> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?
        .into_rgba8();
    let (width, height) = img.dimensions();
    Some(IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}
