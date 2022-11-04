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

use super::{
    find, find_current_dir, find_index, format_code, format_single_file, generate_template,
    open_file, read_file,
};

#[derive(Debug)]
pub struct WebEntity {
    entity_name: String,
    src_dir: String,
    dto: Vec<(String, String)>,
    queries: Vec<(String, String)>,
    create_or_update_base: Vec<(String, String)>,
    create: Vec<(String, String)>,
    update: Vec<(String, String)>,
    solution_dir: String,
    changed_files: RefCell<Vec<String>>,
}
impl WebEntity {
    pub fn create_dto(dto_name: String, dir: String) -> Result<(), CodeGeneratorError> {
        let name = dto_name.to_pascal_case();
        let name = name.trim_end_matches("Dto").trim_end_matches("dto");
        let file_name = format!("{}.ts", name.to_camel_case());
        let file_path = format!("{}/{}", dir.trim_end_matches('\\'), file_name);
        let mut kv = HashMap::new();
        kv.insert("dto_name", Box::new(name));
        let path = generate_template(kv, "Web/dto.ts", &file_path)?;
        format_single_file(path)?;
        Ok(())
    }
}

impl WebEntity {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let file = Path::new(&path);
        let entity_name = file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_pascal_case();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let solution_dir = src_dir[..(src_index)].join("\\");
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        let code = read_file(&path)?;

        let dto = format!(
            r"export interface {}Dto extends ExtensibleEntityDto<string>",
            entity_name
        );
        let dto = get_range_propperties(code.clone(), dto)?;

        let queries = format!(
            r"export interface Get{}Input extends PagedAndSortedResultRequestDto",
            entity_name
        );
        let queries = get_range_propperties(code.clone(), queries)?;

        let create = format!(
            r"export interface {}CreateDto extends TestAppleCreateOrUpdateDtoBase",
            entity_name
        );
        let create = get_range_propperties(code.clone(), create)?;

        let update = format!(
            r"export interface {}UpdateDto extends TestAppleCreateOrUpdateDtoBase",
            entity_name
        );
        let update = get_range_propperties(code.clone(), update)?;

        let create_or_update_base = format!(
            r"export interface {}CreateOrUpdateDtoBase extends ExtensibleObject",
            entity_name
        );
        let create_or_update_base = get_range_propperties(code.clone(), create_or_update_base)?;

        Ok(WebEntity {
            entity_name,
            create,
            update,
            create_or_update_base,
            src_dir,
            dto,
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

    pub fn create_api(&self, url_prefix: String, dir: String) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));
        kv.insert("url_prefix", Box::new(&url_prefix));

        let api_path = format!(
            "{}/{}.ts",
            dir.trim_end_matches('\\'),
            self.entity_name.to_camel_case()
        );

        let path = generate_template(kv, "Web/api.ts", &api_path)?;
        self.add_file_change_log(path);
        self.export_api(&dir)?;
        Ok(())
    }

    fn export_api(&self, api_dir: &str) -> Result<(), CodeGeneratorError> {
        let index_file_path = find_index(api_dir, &self.src_dir);
        let mut file = open_file(&index_file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let insert_code = format!(
            r#"export * as {0}Api from "./{0}";"#,
            self.entity_name.to_camel_case(),
        );
        if !code.contains(&insert_code) {
            file.write(insert_code.as_bytes())?;
            self.add_file_change_log(index_file_path);
        }
        Ok(())
    }
    pub fn create_store(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("entity", Box::new(&self.entity_name));

        let store_file_path = format!(
            "{}/{}.ts",
            dir.trim_end_matches('\\'),
            self.entity_name.to_camel_case()
        );
        let path = generate_template(kv, "Web/store.ts", &store_file_path)?;
        self.add_file_change_log(path);
        // self.export_store(&store_file_path)?;
        Ok(())
    }

    fn export_store(&self, api_dir: &str) -> Result<(), CodeGeneratorError> {
        let file_path = api_dir.to_string() + "\\index.ts";

        let mut file = open_file(&file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let import_code = format!(
            r#"import {}Store from "./{}";"#,
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
        let mut kv: HashMap<String, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("entity".to_string(), Box::new(&self.entity_name));
        kv.insert("properties".to_string(), Box::new(&self.dto));
        kv.insert("queries".to_string(), Box::new(&self.queries));

        let pages = find(&self.src_dir, "Pages", false)
            .unwrap()
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
        kv: HashMap<String, Box<T>>,
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
    pub fn format_all(&self) -> Result<(), CodeGeneratorError> {
        let files = self.changed_files.borrow().to_vec();
        format_code(self.solution_dir.clone(), files)
    }
}

fn get_range_propperties(
    code: String,
    prefix_code: String,
) -> Result<Vec<(String, String)>, CodeGeneratorError> {
    let mut queries = vec![];
    let search = format!(r"{} \{{([\s\S]+?)}}", prefix_code);
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
    Ok(queries)
}
