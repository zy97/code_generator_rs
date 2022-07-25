use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir, File},
    io::Read,
};
extern crate inflector;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
use inflector::Inflector;
use regex::Regex;
use tera::{Context, Tera};
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
        tera.autoescape_on(vec![".html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        println!("Tera initialized:{:?}", tera);
        tera
    };
}
fn main() {
    println!("abp entity path:");
    let entity_path =
        String::from(r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs");
    // stdin().read_line(&mut entity_path).unwrap();
    //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
    let entity_path = entity_path.trim().to_string();

    let mut file = File::open(&entity_path).unwrap();
    let mut code = vec![];
    file.read_to_end(&mut code).unwrap();
    let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();
    println!("code:{}", code);

    let re = Regex::new(r"class ([a-zA-Z]+) :").unwrap();
    let entity_name = re.captures(&code).unwrap().get(1).unwrap().as_str();
    let entity_names = entity_name.to_plural();
    println!("entity_name:{}, entity_names:{}", entity_name, entity_names);
    let re = Regex::new(r"<([a-zA-Z]+)>").unwrap();
    let id_type = re.captures(&code).unwrap().get(1).unwrap().as_str();
    println!("id_type:{}", id_type);
    let re = Regex::new(format!(r"namespace ([a-zA-Z.]+).{}", entity_names).as_str()).unwrap();
    let namespace = re.captures(&code).unwrap().get(1).unwrap().as_str();
    println!("namespace:{}", namespace);
    //正则表达式可以优化
    let re = Regex::new(r">([\s]*)\{([a-zA-Z\\ \r\n;{}]+)}([\s]*)}").unwrap();
    let properties = re.captures(&code).unwrap().get(2).unwrap().as_str();
    println!("properties:{}", properties.trim());

    let src_dir = entity_path.split('\\').collect::<Vec<&str>>();
    let src_index = src_dir.iter().position(|&i| i.contains("src")).unwrap();
    let src_dir = src_dir[..(src_index + 1)].join("\\");

    create_dto(
        &src_dir,
        namespace,
        id_type,
        properties,
        entity_name,
        &entity_names,
    );
    create_createorupdatedto(&src_dir, namespace, properties, entity_name, &entity_names);
}
fn create_dto(
    src_path: &str,
    namespace: &str,
    id_type: &str,
    properties: &str,
    entity_name: &str,
    folder: &str,
) {
    let mut kv = HashMap::new();
    kv.insert("namespace", namespace);
    kv.insert("folder", folder);
    kv.insert("entity", entity_name);
    kv.insert("id", id_type);
    kv.insert("properties", properties);
    let application_contracts_dir = walkdir::WalkDir::new(src_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_dir()
                && e.file_name()
                    .to_str()
                    .unwrap()
                    .contains(".Application.Contracts")
        })
        .nth(0)
        .unwrap();
    let application_contracts_dir = vec![
        application_contracts_dir.path().to_str().unwrap(),
        entity_name.to_plural().as_str(),
    ]
    .join("\\");
    create_dir(&application_contracts_dir);
    let application_contracts_dir =
        vec![application_contracts_dir, format!("{}Dto.cs", entity_name)].join("\\");
    generate_template(
        kv,
        "Application.Contracts/Dto.cs",
        &application_contracts_dir,
    )
}
fn create_createorupdatedto(
    src_path: &str,
    namespace: &str,
    properties: &str,
    entity_name: &str,
    folder: &str,
) {
    let mut kv = HashMap::new();
    kv.insert("namespace", namespace);
    kv.insert("folder", folder);
    kv.insert("entity", entity_name);
    kv.insert("properties", properties);
    let application_contracts_dir = walkdir::WalkDir::new(src_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_dir()
                && e.file_name()
                    .to_str()
                    .unwrap()
                    .contains(".Application.Contracts")
        })
        .nth(0)
        .unwrap();
    let application_contracts_dir = vec![
        application_contracts_dir.path().to_str().unwrap(),
        entity_name.to_plural().as_str(),
    ]
    .join("\\");
    create_dir(&application_contracts_dir);
    let file_path = vec![
        application_contracts_dir,
        format!("CreateOrUpdate{}Dto.cs", entity_name),
    ]
    .join("\\");
    generate_template(kv, "Application.Contracts/CreateOrUpdateDto.cs", &file_path)
}
fn generate_template(kv: HashMap<&str, &str>, template_name: &str, file_path: &str) {
    let mut context = Context::new();
    // context.insert("numbers", &vec![1, 2, 3]);
    for entity in kv {
        context.insert(entity.0, entity.1);
    }
    // context.insert("namespace", namespace);
    // context.insert("folder", &entity_names);
    // context.insert("entity", entity_name);
    // context.insert("id", id_type);
    // context.insert("properties", properties);

    // A one off template
    Tera::one_off("hello", &Context::new(), true).unwrap();

    let mut file = File::create(file_path).expect("create failed");
    match TEMPLATES.render_to(template_name, &context, file) {
        Ok(()) => println!("write success"),
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
        }
    };
}
