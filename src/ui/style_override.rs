use eframe::egui::Style;

pub fn style_override(style: &mut Style) {
    remove_tooltip_delay(style);
}

#[inline]
fn remove_tooltip_delay(style: &mut Style) {
    style.interaction.tooltip_delay = 0.;
    style.interaction.tooltip_grace_time = 0.;
}
