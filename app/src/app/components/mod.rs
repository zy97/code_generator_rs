use egui::Ui;

mod demo_page;
mod service_page;
pub use demo_page::DemoPage;
pub use service_page::ServicePage;

#[derive(PartialEq)]
pub enum TabEnum {
    Web,
    Service,
    Other,
}
pub trait AppComponent {
    type AppData;
    #[allow(unused)]
    fn add(app: &mut Self::AppData, ui: &mut Ui) {}
    #[allow(unused)]
    fn with_frame(app: &mut Self::AppData, ui: &mut Ui, frame: &egui::Frame) {}
}
