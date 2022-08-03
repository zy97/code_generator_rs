#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate log;
mod app;
use anyhow::{Ok, Result};
use app::App;
use code_generator::{Entity, WebEntity};
use egui::vec2;
use env_logger::{Builder, Target};
use std::{io::stdin, sync::mpsc::channel};

use crate::app::Logger;

fn main() -> Result<()> {
    let (tx, rx) = channel();
    let logger = Logger { sender: tx.clone() };
    let logger = Box::new(logger);
    Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .target(Target::Pipe(logger))
        .init();
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
            let app = App::new(_cc, rx);
            Box::new(app)
        }),
    );
    println!("web or service");
    let mut web_or_service = String::from("");
    stdin().read_line(&mut web_or_service).unwrap();
    if web_or_service.trim() == "web" {
        println!("web entity path:");
        let mut entity_path = String::from("");
        stdin().read_line(&mut entity_path).unwrap();
        //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
        let entity_path = String::from(
            r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\Test.ts",
        );
        let entity_path = entity_path.trim().to_string();

        println!("url prefix:");
        let mut url = String::from("");
        stdin().read_line(&mut url).unwrap();
        let url = String::from("/api/app/post-admin");
        let url = url.trim().to_string();

        let web_entity = WebEntity::new(entity_path, url);
        println!("web entity:{:#?}", web_entity);
        web_entity.create_api();
        web_entity.create_store();
        web_entity.create_page();
    } else {
        println!("abp entity path:");
        let mut entity_path = String::from("");
        stdin().read_line(&mut entity_path).unwrap();
        //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
        let entity_path = String::from(
            r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs",
        );

        let entity_path = entity_path.trim().to_string();
        let entity = Entity::new(String::from(entity_path))?;
        println!("entity:{:#?}", entity);
        let custom = true;
        entity.create_dto()?;
        entity.create_createorupdatedto()?;
        entity.create_pagedandsortedandfilterresultdto()?;
        entity.create_iservice(custom)?;
        entity.create_service(custom)?;
        entity.insert_mapper()?;
    }
    Ok(())
}
