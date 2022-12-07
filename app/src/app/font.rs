use std::borrow::Cow;

use egui::{FontId, TextStyle};
// https://github.com/emilk/egui/issues/64
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "yahei".to_owned(),
        egui::FontData::from_static(include_bytes!(r"C:\Windows\Fonts\FZYTK.TTF")), // Cow::Borrowed(include_bytes!(r"C:\Windows\Fonts\FZYTK.TTF")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "yahei".to_owned());

    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("yahei".to_owned());
    ctx.set_fonts(fonts);
    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (
    //         TextStyle::Heading,
    //         FontId::new(30.0, egui::FontFamily::Proportional),
    //     ),
    //     (
    //         TextStyle::Body,
    //         FontId::new(18.0, egui::FontFamily::Proportional),
    //     ),
    //     (
    //         TextStyle::Monospace,
    //         FontId::new(18.0, egui::FontFamily::Proportional),
    //     ),
    //     (
    //         TextStyle::Button,
    //         FontId::new(18.0, egui::FontFamily::Proportional),
    //     ),
    //     (
    //         TextStyle::Small,
    //         FontId::new(10.0, egui::FontFamily::Proportional),
    //     ),
    // ]
    // .into();
    // ctx.set_style(style);
}
