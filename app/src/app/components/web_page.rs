use anyhow::Ok;
use code_generator::WebEntity;

use crate::app::{preview_file_being_dropped, App};

use super::AppComponent;

pub struct WebPage {
    pub entity_path: String,
    url_prefix: String,
    entity: Option<WebEntity>,
    service: Service,
}
#[derive(Default)]
struct Service {
    api: bool,
    store: bool,
    page: bool,
}
impl Default for WebPage {
    fn default() -> Self {
        Self {
            entity_path: Default::default(),
            entity: Default::default(),
            service: Default::default(),
            url_prefix: Default::default(),
        }
    }
}

impl AppComponent for WebPage {
    type AppData = App;
    fn add(app: &mut Self::AppData, ui: &mut egui::Ui) {
        preview_file_being_dropped(&ui.ctx());

        let data = &mut app.web_page;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("拖放abp domain entity文件到此处:");
                ui.text_edit_singleline(&mut data.entity_path);
                if ui.button("Open file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        data.entity_path = path.display().to_string();
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("请输入api地址前缀:");
                ui.centered_and_justified(|ui| {
                    ui.text_edit_singleline(&mut data.url_prefix);
                });
            });

            egui::SidePanel::left("left_panel").show_inside(ui, |ui| {
                ui.checkbox(&mut data.service.api, "生成api文件");
                ui.checkbox(&mut data.service.store, "生成store文件");
                ui.checkbox(&mut data.service.page, "生成page文件");
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                    if ui.button("生成").clicked() {
                        if data.entity_path == String::default() {
                            warn!("请选择abp entity文件！");
                        } else {
                            let entity =
                                WebEntity::new(data.entity_path.clone(), data.url_prefix.clone());
                            match entity {
                                core::result::Result::Ok(entity) => {
                                    data.entity = Some(entity);
                                }
                                Err(err) => {
                                    warn!("{}", err);
                                }
                            }
                            match &data.entity {
                                Some(entity) => {
                                    let data = &data.service;
                                    debug!("开始执行生成操作：");
                                    if data.api {
                                        entity.create_api();
                                    }
                                    if data.store {
                                        entity.create_store();
                                    }
                                    if data.page {
                                        entity.create_page();
                                    }
                                    debug!("执行生成操作完成！");
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
                            egui::TextEdit::multiline(&mut app.log_text),
                        );
                    });
                    ui.allocate_space(ui.available_size());
                });
            });
        });
    }
}
