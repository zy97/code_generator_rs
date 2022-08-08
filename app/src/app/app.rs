use std::sync::mpsc::{channel, Receiver};

use env_logger::{Builder, Env, Target};

use super::{
    components::{AppComponent, DemoPage, ServicePage, TabEnum, WebPage},
    font::setup_custom_fonts,
    Logger,
};
#[derive()]
pub struct App {
    pub can_exit: bool,
    pub is_exiting: bool,
    pub selected_tab: TabEnum,
    pub log_text: String,
    pub logger: Receiver<u8>,

    pub demo_page: DemoPage,
    pub service_page: ServicePage,
    pub web_page: WebPage,
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
        let (_, rx) = channel();
        String::from_utf8(self.logger.try_iter().collect::<Vec<u8>>())
            .unwrap()
            .split('\n')
            .for_each(|msg| {
                if !msg.is_empty() {
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
                    // self.service_ui(ui, ctx);
                    ServicePage::add(self, ui);
                    // self.service_page.add(&mut self, ui);
                }
                TabEnum::Web => {
                    WebPage::add(self, ui);
                }
                _ => {
                    DemoPage::add(self, ui);
                    // self.demo_page.add(&mut self, ui);
                }
            }
        });

        if !ctx.input().raw.dropped_files.is_empty() {
            match self.selected_tab {
                TabEnum::Web => {
                    let files = ctx.input().raw.dropped_files.clone();
                    let file = &files[0];
                    self.web_page.entity_path =
                        file.path.clone().unwrap().display().to_string();
                }
                TabEnum::Service => {
                    let files = ctx.input().raw.dropped_files.clone();
                    let file = &files[0];
                    self.service_page.entity_path =
                        file.path.clone().unwrap().display().to_string();
                }
                TabEnum::Other => {
                    self.demo_page.dropped_files = ctx.input().raw.dropped_files.clone();
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
            selected_tab: TabEnum::Service,
            logger: rx,
            log_text: String::new(),
            demo_page: DemoPage::default(),
            service_page: ServicePage::default(),
            web_page: WebPage::default(),
        }
    }
}
