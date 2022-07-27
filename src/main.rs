mod app;
mod entities;
use app::App;
use entities::Entity;
use entities::WebEntity;
use inflector::Inflector;
use std::collections::HashMap;
use std::io::stdin;
use tera::to_value;
use tera::try_get_value;
use tera::Tera;
use tera::Value;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec![".ts", ".tsx"]);
        tera.register_filter("snake", snake);
        tera.register_filter("plural", plural);
        println!("Tera initialized:{:?}", tera);
        tera
    };
}
pub fn snake(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("snake", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}
pub fn plural(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("plural", "value", String, value);
    Ok(to_value(&s.to_plural()).unwrap())
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(App::default())),
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
        let entity = Entity::new(String::from(entity_path));
        println!("entity:{:#?}", entity);
        let custom = true;
        entity.create_dto();
        entity.create_createorupdatedto();
        entity.create_pagedandsortedandfilterresultdto();
        entity.create_iservice(custom);
        entity.create_service(custom);
        entity.insert_mapper();
    }
}
