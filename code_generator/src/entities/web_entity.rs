use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir, File, OpenOptions},
    hash,
    io::{self, ErrorKind, Read, Write},
    os::windows::prelude::FileExt,
    path::Path,
    vec,
};

use encoding::{all::UTF_8, DecoderTrap, Encoding};
use inflector::Inflector;
use regex::Regex;
use serde::Serialize;
use tera::Context;
use walkdir::DirEntry;

use crate::{
    error::{CodeGeneratorError, RegexNoMatchError},
    TEMPLATES,
};

#[derive(Debug)]
pub struct WebEntity {
    entity_name: String,
    url_prefix: String,
    src_dir: String,
    properties: Vec<(String, String)>,
}

impl WebEntity {
    pub fn new(path: String, url_prefix: String) -> Result<Self, CodeGeneratorError> {
        let file = Path::new(&path);
        let entity_name = file.file_stem().unwrap().to_str().unwrap().to_string();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        let mut properties = vec![];
        let mut file = File::open(&path)?;
        let mut code = vec![];
        file.read_to_end(&mut code)?;
        let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();
        let re = Regex::new(r"([a-zA-Z]+): ([a-zA-Z]+);")?;
        for caps in re.captures_iter(code.as_str()) {
            // properties.push(caps.get(1).unwrap().as_str().to_string());
            properties.push((
                caps.get(1).unwrap().as_str().to_owned(),
                caps.get(2).unwrap().as_str().to_owned(),
            ));
        }

        Ok(WebEntity {
            entity_name,
            url_prefix,
            src_dir,
            properties,
        })
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
    pub fn create_store(&self) {
        let mut kv = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));

        let stores_dir = self
            .find("stores", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&stores_dir).unwrap();
        self.generate_template(
            kv,
            "Web/store.ts",
            &stores_dir,
            format!("{}.ts", self.entity_name),
        );
        self.export_store(&stores_dir);
    }
    fn export_store(&self, api_dir: &str) {
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(api_dir.to_string() + "//index.ts")
            .unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let import_code = format!(
            "import {}Store from './{}';\r\n",
            self.entity_name.to_snake_case(),
            self.entity_name
        );
        if !code.contains(&import_code) {
            let index = code.rfind("export const").unwrap();
            code.insert_str(index - 1, &import_code);
            let index = code.rfind("});").unwrap();
            code.insert_str(
                index - 1,
                format!("\t{}Store", self.entity_name.to_snake_case()).as_str(),
            );
            file.seek_write(code.as_bytes(), 0).unwrap();
        }
    }
    pub fn create_page(&self) {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));
        kv.insert("properties", Box::new(&self.properties));

        let pages = self
            .find("Pages", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        let page = format!("{}\\{}", pages, self.entity_name);
        self.create_dir(&page).unwrap();
        self.generate_template(kv, "Web/page.tsx", &page, String::from("index.tsx"));
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
