#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate log;
mod app;
use anyhow::{Result};
use app::App;

use egui::vec2;


fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        decorated: true, //如果自定义边框，拖动界面可能会导致操作失效，如自定义界面的关闭按钮
        transparent: true,
        drag_and_drop_support: true,
        min_window_size: Some(vec2(320.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "egui demo",
        options,
        Box::new(|_cc| {
            let app = App::new(_cc);
            Box::new(app)
        }),
    );
}
