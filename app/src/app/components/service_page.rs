use code_generator::Entity;

use crate::app::{preview_file_being_dropped, App};

use super::AppComponent;

pub struct ServicePage {
    pub entity_path: String,
    entity: Option<Entity>,
    service: Service,
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
impl Default for ServicePage {
    fn default() -> Self {
        Self {
            entity_path: Default::default(),
            entity: Default::default(),
            service: Default::default(),
        }
    }
}

impl AppComponent for ServicePage {
    type AppData = App;
    fn add(app: &mut Self::AppData, ui: &mut egui::Ui) {
        preview_file_being_dropped(&ui.ctx());

        let data = &mut app.service_page;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Drag-and-drop files onto the window!");
                ui.text_edit_singleline(&mut data.entity_path);
                if ui.button("Open file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        data.entity_path = path.display().to_string();
                    }
                }
            });
            egui::SidePanel::left("left_panel").show_inside(ui, |ui| {
                ui.checkbox(&mut data.service.create_dto, "生成DTO文件");
                ui.checkbox(
                    &mut data.service.create_createorupdatedto,
                    "生成CreateOrUpdateDTO文件",
                );
                ui.checkbox(
                    &mut data.service.create_pagedandsortedandfilterresultdto,
                    "生成PagedAndSortedAndFilterResultDTO文件",
                );
                ui.checkbox(&mut data.service.create_iservice, "生成IService文件");
                ui.checkbox(&mut data.service.create_service, "生成Service文件");
                ui.checkbox(&mut data.service.insert_mapper, "插入Mapper配置");
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                    if ui.button("生成").clicked() {
                        if data.entity_path == String::default() {
                            warn!("请选择abp entity文件！");
                        } else {
                            // app.entity = Entity::new(app.entity_path.clone()).ok();
                            let et = Entity::new(data.entity_path.clone());
                            match &data.entity {
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
                            egui::TextEdit::multiline(&mut app.log_text),
                        );
                    });
                    ui.allocate_space(ui.available_size());
                });
            });
        });
    }
}
