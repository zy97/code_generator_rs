use egui::Ui;

mod demo_page;
pub use demo_page::DemoPage;

pub trait AppComponent {
    type AppData;
    #[allow(unused)]
    fn add(app: &mut Self::AppData, ui: &mut Ui) {}
    #[allow(unused)]
    fn with_frame(app: &mut Self::AppData, ui: &mut Ui, frame: &egui::Frame) {}
}
