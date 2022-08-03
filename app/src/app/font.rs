pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "yahei".to_owned(),
        egui::FontData::from_static(include_bytes!("..\\..\\assets\\yahei.ttf")),
    );
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
    let mut style: egui::Style = (*ctx.style()).clone();
}
