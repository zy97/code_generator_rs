use std::sync::mpsc::channel;

use egui::Vec2;
use egui_extras::RetainedImage;
use poll_promise::Promise;

use super::{
    custom_window::custom_window_frame, file_drop::preview_file_being_dropped,
    font::setup_custom_fonts, toggle_switch::toggle,
};
#[derive()]
pub struct App {
    can_exit: bool,
    is_exiting: bool,
    promise: Option<Promise<ehttp::Result<RetainedImage>>>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    image: RetainedImage,
    toggled: bool,
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |ui| ui.heading("Try to close window"));
        let (tx, rx) = channel();
        let promise = self.promise.get_or_insert_with(|| {
            let ctx = ctx.clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get("https://picsum.photos/seed/1.759706314/1024");
            ehttp::fetch(request, move |response| {
                let image = response.and_then(parse_response);
                sender.send(image);
                ctx.request_repaint();
            });
            promise
        });

        custom_window_frame(tx, ctx, frame, "egui with custom frame", |ui| {
            ui.add(toggle(&mut self.toggled));
            ui.horizontal(|ui| {
                let mut image_rect = egui::Rect::everything_above(1.0);
                ui.vertical(|ui| {
                    ui.heading("This is an image:");
                    image_rect = self.image.show_scaled(ui, 0.1).rect;
                });
                let image_size = Vec2::new(
                    self.image.size_vec2().x / 10.0,
                    self.image.size_vec2().y / 10.0,
                );
                ui.vertical(|ui| {
                    ui.heading("This is a rotated image:");
                    let sdf = egui::Image::new(self.image.texture_id(ctx), image_size)
                        .rotate(45.0f32.to_radians(), egui::Vec2::splat(0.5));
                    ui.add(sdf);
                });
                ui.vertical(|ui| {
                    ui.heading("This is an image you can click:");
                    ui.add(egui::ImageButton::new(
                        self.image.texture_id(ctx),
                        image_size,
                    ));
                });
            });

            ui.label("Drag-and-drop files onto the window!");
            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");
                    for file in &self.dropped_files {
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
        });
        preview_file_being_dropped(ctx);
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
        if let Ok(r) = rx.try_recv() {
            self.is_exiting = r
        }

        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Not yet").clicked() {
                            self.is_exiting = false;
                        }
                        if ui.button("Yes!").clicked() {
                            self.can_exit = true;
                            frame.quit()
                        }
                    })
                });
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            can_exit: false,
            is_exiting: false,
            promise: None,
            dropped_files: Vec::new(),
            toggled: false,
            picked_path: None,
            image: RetainedImage::from_image_bytes(
                "rust-logo",
                include_bytes!("..\\..\\assets\\rust-logo.png"),
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
