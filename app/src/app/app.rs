use std::sync::mpsc::{channel, Receiver};

use code_generator::Entity;
use egui::Vec2;
use egui_extras::RetainedImage;
use env_logger::{Builder, Env, Target};

use poll_promise::Promise;

use super::{
    components::{AppComponent, DemoPage},
    file_drop::preview_file_being_dropped,
    font::setup_custom_fonts,
    toggle_switch::toggle,
    Logger,
};
#[derive()]
pub struct App {
    pub can_exit: bool,
    pub is_exiting: bool,
    pub promise: Option<Promise<ehttp::Result<RetainedImage>>>,
    pub dropped_files: Vec<egui::DroppedFile>,
    pub picked_path: Option<String>,
    pub image: RetainedImage,
    pub toggled: bool,
    pub selected_tab: TabEnum,
    pub service: Service,
    pub log_text: String,
    pub logger: Receiver<u8>,
    pub entity: Option<Entity>,
    pub entity_path: String,
}

#[derive(PartialEq)]
pub enum TabEnum {
    Web,
    Service,
    Other,
}
#[derive(Default)]
pub struct Service {
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
                    DemoPage::add(self, ui);
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
