use std::sync::mpsc::{channel, Receiver};

use code_generator::Entity;
use egui::Vec2;
use egui_extras::RetainedImage;
use env_logger::{Builder, Env, Target};

use poll_promise::Promise;

use super::{
    file_drop::preview_file_being_dropped, font::setup_custom_fonts, toggle_switch::toggle, Logger,
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
    selected_tab: TabEnum,
    service: Service,
    log_text: String,
    logger: Receiver<u8>,
    entity: Option<Entity>,
    entity_path: String,
}

#[derive(PartialEq)]
enum TabEnum {
    Web,
    Service,
    Other,
}
#[derive(Default)]
struct Service {
    create_dto: bool,
    create_createorupdatedto: bool,
    create_pagedandsortedandfilterresultdto: bool,
    create_iservice: bool,
    create_service: bool,
    insert_mapper: bool,
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
        let (_, rx) = channel();
        String::from_utf8(self.logger.try_iter().collect::<Vec<u8>>())
            .unwrap()
            .split('\n')
            .for_each(|msg| {
                if !msg.is_empty() {
                    // println!("from pipe: {}", msg);
                    self.log_text.push_str(format!("{}\r\n", msg).as_str());
                }
            });
        // custom_window_frame(tx, ctx, frame, "egui with custom frame", |ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.selectable_value(&mut self.selected_tab, TabEnum::Service, "Web服务生成选项");
                ui.selectable_value(&mut self.selected_tab, TabEnum::Web, "前端生成选项");
                ui.selectable_value(&mut self.selected_tab, TabEnum::Other, "其他测试");
            });

            match self.selected_tab {
                TabEnum::Service => {
                    self.service_ui(ui, ctx);
                }
                TabEnum::Web => {}
                _ => {
                    self.other_ui(ui, ctx);
                }
            }
        });
        preview_file_being_dropped(ctx);
        if !ctx.input().raw.dropped_files.is_empty() {
            match self.selected_tab {
                TabEnum::Web => {}
                TabEnum::Service => {
                    let files = ctx.input().raw.dropped_files.clone();
                    let file = &files[0];
                    self.entity_path = file.path.clone().unwrap().display().to_string();
                }
                TabEnum::Other => {
                    self.dropped_files = ctx.input().raw.dropped_files.clone();
                }
            }
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
        let env = Env::default()
            .filter_or("MY_LOG_LEVEL", "trace")
            // Normally using a pipe as a target would mean a value of false, but this forces it to be true.
            .write_style_or("MY_LOG_STYLE", "always");
        let (tx, rx) = channel::<u8>();
        Builder::from_env(env)
            .filter_level(log::LevelFilter::max())
            .target(Target::Pipe(Box::new(Logger { sender: tx })))
            // .target(Target::Stdout)
            .init();
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
            selected_tab: TabEnum::Service,
            service: Service::default(),
            logger: rx,
            log_text: String::new(),
            entity: None,
            entity_path: String::new(),
        }
    }
    fn other_ui(self: &mut App, ui: &mut egui::Ui, ctx: &egui::Context) {
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
    }

    fn service_ui(self: &mut App, ui: &mut egui::Ui, _: &egui::Context) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Drag-and-drop files onto the window!");
                ui.text_edit_singleline(&mut self.entity_path);
                if ui.button("Open file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.entity_path = path.display().to_string();
                    }
                }
            });
            egui::SidePanel::left("left_panel").show_inside(ui, |ui| {
                ui.checkbox(&mut self.service.create_dto, "生成DTO文件");
                ui.checkbox(
                    &mut self.service.create_createorupdatedto,
                    "生成CreateOrUpdateDTO文件",
                );
                ui.checkbox(
                    &mut self.service.create_pagedandsortedandfilterresultdto,
                    "生成PagedAndSortedAndFilterResultDTO文件",
                );
                ui.checkbox(&mut self.service.create_iservice, "生成IService文件");
                ui.checkbox(&mut self.service.create_service, "生成Service文件");
                ui.checkbox(&mut self.service.insert_mapper, "插入Mapper配置");
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                    if ui.button("生成").clicked() {
                        if self.entity_path == String::default() {
                            warn!("请选择abp entity文件！");
                        } else {
                            // self.entity = Entity::new(self.entity_path.clone()).ok();
                            let et = Entity::new(self.entity_path.clone());
                            match &self.entity {
                                Some(entity) => {
                                    debug!("开始执行生成操作：");
                                }
                                None => {
                                    warn!("请选择有效的abp entity文件！");
                                }
                            }
                        }
                    }
                });
                egui::TopBottomPanel::top("top").show_inside(ui, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.log_text),
                        );
                    });
                    ui.allocate_space(ui.available_size());
                });
            });
        });
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
