use inflector::Inflector;
use log::info;

use std::collections::HashMap;
use tera::{to_value, try_get_value, Tera, Value};
// mod entities;
mod error;
// pub use entities::Entity;
// pub use entities::Permission;
// pub use entities::WebEntity;
pub use error::CodeGeneratorError;
mod services;
// pub use entities::{get_expressions_in_template, process_template, process_template_to_file};
pub use sea_orm::DbErr;
pub use services::*;
const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
// const DB_NAME: &str = "bakeries_db";

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref TEMPLATES: Tera = {
        //如果是相对路径，那么会在可执行文件目录中查找模板，因此，最终需要把模板复制过去
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // TODO: 从数据库中获取模板


        let templates = async_std::task::block_on(async {
            crate::services::templates_svc::get_all().await.unwrap()
        });
        // println!("templates: {:#?}", templates);
        for template in templates {
            let template_name = format!("{}/{}", template.project_id, template.id);
            tera.add_raw_template(&template_name, &template.content)
                .unwrap();
            // println!("template add success! template_name: {}", template_name)
        }
        tera.register_filter("snake", snake);
        tera.register_filter("plural", plural);
        tera.register_filter("camel", camel);
        info!("{:?}",tera);
        tera
    };
}
fn snake(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("snake", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}
fn plural(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("plural", "value", String, value);
    Ok(to_value(&s.to_plural()).unwrap())
}
fn camel(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("camel", "value", String, value);
    Ok(to_value(&s.to_camel_case()).unwrap())
}
#[macro_export]
macro_rules! dynamicDic {
( $( $x:expr ),* ) => {
    {
        let mut kv: std::collections::HashMap<&str, Box<dyn erased_serde::Serialize>> = std::collections::HashMap::new();
        $(
            kv.insert($x.0, Box::new($x.1));
        )*
        kv
    }
};
}
#[macro_export]
macro_rules! dynamic_dic {
( $( $x:expr ),* ) => {
    {
        let mut kv: std::collections::HashMap<String, Box<dyn erased_serde::Serialize>> = std::collections::HashMap::new();
        $(
            kv.insert($x.0, Box::new($x.1));
        )*
        kv
    }
};
}
