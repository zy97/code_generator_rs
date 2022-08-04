use std::{borrow::Cow, fs::File, io::Read};

use egui::{FontId, TextStyle};

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut font = File::open(r"C:\Windows\Fonts\msyhbd.ttc").unwrap();
    let mut buffer = vec![];
    let font = font.read_to_end(&mut buffer);
    // let sd: Cow<'static, [u8]> = std::borrow::Cow::Owned(buffer);
    fonts
        .font_data
        .insert("yahei".to_owned(), egui::FontData::from_owned(buffer));
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "yahei".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("yahei".to_owned());
    ctx.set_fonts(fonts);
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(30.0, egui::FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(18.0, egui::FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(18.0, egui::FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(18.0, egui::FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(10.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}
