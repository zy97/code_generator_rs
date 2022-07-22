use std::{
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
    let mut entity_path =
        String::from(r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs");
    // stdin().read_line(&mut entity_path).unwrap();
    //如果从控制台接受输入，如果没有这句，会提示路径不对等信息，可能是有其他特殊字符
    let mut entity_path = entity_path.trim().to_string();
    let entity_path = entity_path.to_string();

    let mut file = File::open(&entity_path).unwrap();
    let mut code = vec![];
    file.read_to_end(&mut code).unwrap();
    let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();
    println!("code:{}", code);
    // let entity_name = code.
    let re = Regex::new(r"class ([a-zA-Z]+) :").unwrap();
    let entity_name = re.captures(&code).unwrap().get(1).unwrap().as_str();
    println!("entity_name:{}", entity_name);

    let re = Regex::new(
        r###">
    {
        ([a-zA-Z]+) 
    }
}"###,
    )
    .unwrap();
    let properties = re.captures(&code).unwrap().get(1).unwrap().as_str();
    println!("properties:{}", properties);

    let src_dir = entity_path.split('\\').collect::<Vec<&str>>();
    let src = src_dir.iter().position(|&i| i.contains("src")).unwrap();
    let application_contracts_dir = src_dir[..(src + 1)].join("\\");
    println!("path:{:?}", application_contracts_dir);
    let application_contracts_dir = walkdir::WalkDir::new(application_contracts_dir)
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
    println!("path:{:?}", application_contracts_dir);
    let application_contracts_dir = vec![
        application_contracts_dir.path().to_str().unwrap(),
        entity_name.to_plural().as_str(),
    ]
    .join("\\");
    create_dir(&application_contracts_dir);
    let application_contracts_dir =
        vec![application_contracts_dir, format!("{}Dto.cs", entity_name)].join("\\");
    println!("{}", application_contracts_dir);
    let mut context = Context::new();

    context.insert("username", &"Bob");
    context.insert("numbers", &vec![1, 2, 3]);
    context.insert("show_all", &false);
    context.insert("bio", &"<script>alert('pwnd');</script>");
    context.insert("properties", "");

    // A one off template
    Tera::one_off("hello", &Context::new(), true).unwrap();

    let mut file = File::create(application_contracts_dir).expect("create failed");
    match TEMPLATES.render_to("Application.Contracts/Dto.cs", &context, file) {
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

    // // let result = TEMPLATES.render("hello.html", &context).unwrap();
    // // println!("{}", result);
}
