use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir, File, OpenOptions},
    io::{self, ErrorKind, Read, Write},
    path::Path,
};

use inflector::Inflector;
use serde::Serialize;
use tera::{to_value, try_get_value, Context, Tera, Value};
use walkdir::DirEntry;

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
        tera.register_filter("snake", do_nothing_filter);
        println!("Tera initialized:{:?}", tera);
        tera
    };
}
pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}
#[derive(Debug)]
pub struct WebEntity {
    entity_name: String,
    url_prefix: String,
    src_dir: String,
    plural_name: String,
}

impl WebEntity {
    pub fn new(path: String, url_prefix: String) -> Self {
        let file = Path::new(&path);
        let entity_name = file.file_stem().unwrap().to_str().unwrap().to_string();
        // let mut file = File::open(&path).unwrap();
        // let mut code = vec![];
        // file.read_to_end(&mut code).unwrap();
        // let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();

        // let re = Regex::new(r"class ([a-zA-Z]+) :").unwrap();
        // let entity_name = re
        //     .captures(&code)
        //     .unwrap()
        //     .get(1)
        //     .unwrap()
        //     .as_str()
        //     .to_string();
        let entity_names = entity_name.to_plural();

        // let re = Regex::new(r"<([a-zA-Z]+)>").unwrap();
        // let id_type = re
        //     .captures(&code)
        //     .unwrap()
        //     .get(1)
        //     .unwrap()
        //     .as_str()
        //     .to_string();

        // let re = Regex::new(format!(r"namespace ([a-zA-Z.]+).{}", entity_names).as_str()).unwrap();
        // let namespace = re
        //     .captures(&code)
        //     .unwrap()
        //     .get(1)
        //     .unwrap()
        //     .as_str()
        //     .to_string();

        // //正则表达式可以优化
        // let re = Regex::new(r">([\s]*)\{([a-zA-Z\\ \r\n;{}]+)}([\s]*)}").unwrap();
        // let properties = re
        //     .captures(&code)
        //     .unwrap()
        //     .get(2)
        //     .unwrap()
        //     .as_str()
        //     .to_string();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        WebEntity {
            entity_name,
            url_prefix,
            src_dir,
            plural_name: entity_names,
        }
    }
    fn find(&self, contain_name: &str, is_file: bool) -> DirEntry {
        let result = walkdir::WalkDir::new(&self.src_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                if is_file {
                    e.file_type().is_file()
                        && e.file_name().to_str().unwrap().contains(contain_name)
                } else {
                    e.file_type().is_dir() && e.file_name().to_str().unwrap().contains(contain_name)
                }
            })
            .nth(0)
            .unwrap();
        return result;
    }
    fn create_dir(&self, dir: &str) -> io::Result<()> {
        match create_dir(dir) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn create_api(&self) {
        let mut kv = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));
        kv.insert("url_prefix", Box::new(&self.url_prefix));

        let api_dir = self
            .find("apis", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&api_dir).unwrap();
        self.generate_template(
            kv,
            "Web/api.ts",
            &api_dir,
            format!("{}.ts", self.entity_name),
        );
        self.export_api(&api_dir);
    }
    fn export_api(&self, api_dir: &str) {
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(api_dir.to_string() + "//index.ts")
            .unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let insert_code = format!(
            "export * as {}Api from './{}';\r\n",
            self.entity_name.to_snake_case(),
            self.entity_name
        );
        if !code.contains(&insert_code) {
            // code.push_str(&insert_code)
            file.write(insert_code.as_bytes()).unwrap();
        }
    }
}

impl WebEntity {
    fn generate_template<T>(
        &self,
        kv: HashMap<&str, Box<T>>,
        template_name: &str,
        dir: &str,
        file_name: String,
    ) where
        T: Serialize + ?Sized,
    {
        let file_path = vec![dir, file_name.as_str()].join("\\");

        let mut context = Context::new();
        // context.insert("numbers", &vec![1, 2, 3]);
        for entity in kv {
            context.insert(entity.0, &entity.1);
        }

        // A one off template
        Tera::one_off("hello", &Context::new(), true).unwrap();

        let file = File::create(&file_path).expect("create failed");
        match TEMPLATES.render_to(template_name, &context, file) {
            Ok(()) => println!("{} write success", file_path),
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
}
