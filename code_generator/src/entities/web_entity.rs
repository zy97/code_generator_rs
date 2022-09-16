use inflector::Inflector;
use regex::Regex;
use serde::Serialize;
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::{create_dir, File},
    io::{self, ErrorKind, Read, Write},
    os::windows::prelude::FileExt,
    path::Path,
    vec,
};
use tera::Context;

use crate::{error::CodeGeneratorError, TEMPLATES};

use super::{find, format_code, open_file, read_file};

#[derive(Debug)]
pub struct WebEntity {
    entity_name: String,
    url_prefix: String,
    src_dir: String,
    properties: Vec<(String, String)>,
    queries: Vec<(String, String)>,
    solution_dir: String,
    changed_files: RefCell<Vec<String>>,
}

impl WebEntity {
    pub fn new(path: String, url_prefix: String) -> Result<Self, CodeGeneratorError> {
        let file = Path::new(&path);
        let entity_name = file.file_stem().unwrap().to_str().unwrap().to_string();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let solution_dir = src_dir[..(src_index)].join("\\");
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        let mut properties = vec![];
        let code = read_file(&path)?;
        let re = Regex::new(r"([a-zA-Z]+): ([a-zA-Z]+);")?;
        for caps in re.captures_iter(code.as_str()) {
            properties.push((
                caps.get(1).unwrap().as_str().to_owned(),
                caps.get(2).unwrap().as_str().to_owned(),
            ));
        }
        let mut queries = vec![];
        let search = format!(
            r"export interface Search{}Dto extends PageRequest \{{([\s\S]+?)}}",
            entity_name
        );
        let re = Regex::new(&search)?;
        let captures = re.captures(code.as_str());
        if let Some(captures) = captures {
            let search = captures.get(1).unwrap().as_str();
            let sdf = search.lines().filter(|s| !s.is_empty()).filter_map(|f| {
                if f.contains(':') {
                    let mut items = f.split(':');
                    let x: &[_] = &[',', ' '];
                    return Some((
                        items.next().unwrap().trim().to_string(),
                        items.next().unwrap().trim_matches(x).to_string(),
                    ));
                }
                return None;
            });
            queries.append(sdf.collect::<Vec<(String, String)>>().as_mut());
        }

        println!("search_code:{:?}", queries);

        Ok(WebEntity {
            entity_name,
            url_prefix,
            src_dir,
            properties,
            changed_files: RefCell::new(vec![]),
            solution_dir,
            queries,
        })
    }

    fn create_dir(&self, dir: &str) -> io::Result<()> {
        match create_dir(dir) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn create_api(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));
        kv.insert("url_prefix", Box::new(&self.url_prefix));
        kv.insert("queries", Box::new(&self.queries));

        let api_dir = find(&self.src_dir, "apis", false)
            .path()
            .display()
            .to_string();

        let path = self.generate_template(
            kv,
            "Web/api.ts",
            &api_dir,
            format!("{}.ts", self.entity_name),
        )?;
        self.add_file_change_log(path);
        self.export_api(&api_dir)?;
        Ok(())
    }

    fn export_api(&self, api_dir: &str) -> Result<(), CodeGeneratorError> {
        let file_path = api_dir.to_string() + "\\index.ts";

        let mut file = open_file(&file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let insert_code = format!(
            "export * as {}Api from './{}';",
            self.entity_name.to_snake_case(),
            self.entity_name
        );
        if !code.contains(&insert_code) {
            file.write(insert_code.as_bytes())?;
            self.add_file_change_log(file_path);
        }
        Ok(())
    }
    pub fn create_store(&self) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));

        let stores_dir = find(&self.src_dir, "stores", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&stores_dir)?;
        let path = self.generate_template(
            kv,
            "Web/store.ts",
            &stores_dir,
            format!("{}.ts", self.entity_name),
        )?;
        self.add_file_change_log(path);
        self.export_store(&stores_dir)?;
        Ok(())
    }

    fn export_store(&self, api_dir: &str) -> Result<(), CodeGeneratorError> {
        let file_path = api_dir.to_string() + "\\index.ts";

        let mut file = open_file(&file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let import_code = format!(
            "import {}Store from './{}';",
            self.entity_name.to_snake_case(),
            self.entity_name
        );
        if !code.contains(&import_code) {
            let index = code.rfind("export const").unwrap();
            code.insert_str(index - 1, &import_code);
            let index = code.rfind("});").unwrap();
            code.insert_str(
                index - 1,
                format!(",{}Store", self.entity_name.to_snake_case()).as_str(),
            );
            file.seek_write(code.as_bytes(), 0)?;
            self.add_file_change_log(file_path);
        }
        Ok(())
    }
    pub fn create_page(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));
        kv.insert("properties", Box::new(&self.properties));
        kv.insert("queries", Box::new(&self.queries));

        let pages = find(&self.src_dir, "Pages", false)
            .path()
            .display()
            .to_string();

        let page = format!("{}\\{}", pages, self.entity_name);
        self.create_dir(&page)?;
        let path = self.generate_template(kv, "Web/page.tsx", &page, String::from("index.tsx"))?;
        self.add_file_change_log(path);
        Ok(())
    }
    fn add_file_change_log(&self, path: String) {
        let mut changs = self.changed_files.borrow_mut();
        changs.push(path);
    }
}

impl WebEntity {
    fn generate_template<T>(
        &self,
        kv: HashMap<&str, Box<T>>,
        template_name: &str,
        dir: &str,
        file_name: String,
    ) -> Result<String, CodeGeneratorError>
    where
        T: Serialize + ?Sized,
    {
        let file_path = vec![dir, file_name.as_str()].join("\\");

        let mut context = Context::new();
        for entity in kv {
            context.insert(entity.0, &entity.1);
        }

        let file = File::create(&file_path)?;
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
        Ok(file_path)
    }
}

impl WebEntity {
    pub fn format_all(&self) {
        let files = self.changed_files.borrow().to_vec();
        format_code(self.solution_dir.clone(), files)
    }
}
