use egui::Vec2;
use egui_extras::RetainedImage;
use poll_promise::Promise;

use crate::app::{preview_file_being_dropped, toggle_switch::toggle, App};

use super::AppComponent;

pub struct DemoPage {
    promise: Option<Promise<ehttp::Result<RetainedImage>>>,
    pub dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    image: RetainedImage,
    toggled: bool,
}

impl AppComponent for DemoPage {
    type AppData = App;

    fn add(app: &mut Self::AppData, ui: &mut egui::Ui) {
        preview_file_being_dropped(&ui.ctx());

        let data = &mut app.demo_page;
        let ctx = ui.ctx().clone();
        let promise = data.promise.get_or_insert_with(|| {
            let ctx = ui.ctx().clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get("https://picsum.photos/seed/1.759706314/1024");
            ehttp::fetch(request, move |response| {
                let image = response.and_then(parse_response);
                sender.send(image);
                ctx.request_repaint();
            });
            promise
        });
        ui.add(toggle(&mut data.toggled));
        ui.horizontal(|ui| {
            let mut image_rect = egui::Rect::everything_above(1.0);
            ui.vertical(|ui| {
                ui.heading("This is an image:");
                image_rect = data.image.show_scaled(ui, 0.1).rect;
            });
            let image_size = Vec2::new(
                data.image.size_vec2().x / 10.0,
                data.image.size_vec2().y / 10.0,
            );
            ui.vertical(|ui| {
                ui.heading("This is a rotated image:");
                let sdf = egui::Image::new(data.image.texture_id(&ctx), image_size)
                    .rotate(45.0f32.to_radians(), egui::Vec2::splat(0.5));
                ui.add(sdf);
            });
            ui.vertical(|ui| {
                ui.heading("This is an image you can click:");
                ui.add(egui::ImageButton::new(
                    data.image.texture_id(&ctx),
                    image_size,
                ));
            });
        });

        ui.label("Drag-and-drop files onto the window!");
        if ui.button("Open file...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                data.picked_path = Some(path.display().to_string());
            }
        }
        if let Some(picked_path) = &data.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
            });
        }
        if !data.dropped_files.is_empty() {
            ui.group(|ui| {
                ui.label("Dropped files:");
                for file in &data.dropped_files {
                    let mut info = if let Some(path) = &file.path {
                        path.display().to_string()
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        "???".to_owned()
                    };
                    if let Some(bytes) = &file.bytes {
                        use std::fmt::Write as _;
                        write!(info, " ({} bytes)", bytes.len()).ok();
                    }
                    println!("info:{}", info);
                    ui.label(info);
                }
            });
        }

        ui.label("This is just the content of the window");
        ui.horizontal(|ui| {
            ui.label("egui theme:");
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
        match promise.ready() {
            None => {
                ui.spinner();
            }
            Some(Err(e)) => {
                ui.colored_label(egui::Color32::RED, e);
            }
            Some(Ok(image)) => {
                image.show_max_size(ui, ui.available_size());
            }
        };
    }
}
impl Default for DemoPage {
    fn default() -> Self {
        Self {
            promise: Default::default(),
            dropped_files: Default::default(),
            picked_path: Default::default(),
            toggled: Default::default(),
            image: RetainedImage::from_image_bytes(
                "rust-logo",
                include_bytes!("..\\..\\..\\assets\\rust-logo.png"),
            )
            .unwrap(),
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn parse_response(response: ehttp::Response) -> ehttp::Result<RetainedImage> {
    let content_type = response.content_type().unwrap_or_default();
    if content_type.starts_with("image/") {
        RetainedImage::from_image_bytes(&response.url, &response.bytes)
    } else {
        Err(format!(
            "Expected image, found content-type {:?}",
            content_type
        ))
    }
}
// fn other_ui(self: &mut App, ui: &mut egui::Ui, ctx: &egui::Context) {}
