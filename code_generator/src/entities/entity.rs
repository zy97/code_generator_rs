use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir, File, OpenOptions},
    io::{self, ErrorKind, Read},
    os::windows::prelude::FileExt,
    result,
};
extern crate inflector;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
use inflector::Inflector;
use regex::Regex;
use serde::Serialize;
use tera::{Context, Tera};
use walkdir::DirEntry;

use crate::{error::CodeGeneratorError, TEMPLATES};

#[derive(Debug)]
pub struct Entity {
    namespace: String,
    id_type: String,
    name: String,
    src_dir: String,
    //复数名字
    plural_name: String,
    properties: String,
}

impl Entity {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let mut file = File::open(&path)?;
        let mut code = vec![];
        file.read_to_end(&mut code)?;
        let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();

        let re = Regex::new(r"class ([a-zA-Z]+) :")?;
        let entity_name = re
            .captures(&code)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();
        let entity_names = entity_name.to_plural();

        let re = Regex::new(r"<([a-zA-Z]+)>")?;
        let id_type = re
            .captures(&code)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let re = Regex::new(format!(r"namespace ([a-zA-Z.]+).{}", entity_names).as_str())?;
        let namespace = re
            .captures(&code)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        //正则表达式可以优化
        let re = Regex::new(r">([\s]*)\{([a-zA-Z\\ \r\n;{}]+)}([\s]*)}")?;
        let properties = re
            .captures(&code)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .to_string();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().position(|&i| i.contains("src")).unwrap();
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        Ok(Entity {
            id_type,
            name: entity_name,
            namespace,
            plural_name: entity_names,
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
    fn create_dir(&self, dir: &str) -> Result<(), CodeGeneratorError> {
        let dir = format!("{}\\{}", dir, &self.plural_name);
        match create_dir(dir) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    pub fn create_dto(&self) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("properties", Box::new(&self.properties));

        let application_contracts_dir = self
            .find(".Application.Contracts", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        self.generate_template(
            kv,
            "Application.Contracts/Dto.cs",
            &application_contracts_dir,
            format!("{}Dto.cs", self.name),
        )
    }

    pub fn create_createorupdatedto(&self) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("properties", Box::new(&self.properties));
        let application_contracts_dir = self
            .find(".Application.Contracts", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        self.generate_template(
            kv,
            "Application.Contracts/CreateOrUpdateDto.cs",
            &application_contracts_dir,
            format!("CreateOrUpdate{}Dto.cs", self.name),
        )
    }

    pub fn create_pagedandsortedandfilterresultdto(&self) -> Result<(), CodeGeneratorError> {
        let mut kv = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("properties", Box::new(&self.properties));
        let application_contracts_dir = self
            .find(".Application.Contracts", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        self.generate_template(
            kv,
            "Application.Contracts/PagedAndSortedAndFilteredResultRequestDto.cs",
            &application_contracts_dir,
            String::from("PagedAndSortedAndFilteredResultRequestDto.cs"),
        )
    }

    pub fn create_iservice(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("custom", Box::new(custom));
        let application_contracts_dir = self
            .find(".Application.Contracts", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&application_contracts_dir)?;

        self.generate_template(
            kv,
            "Application.Contracts/IService.cs",
            &application_contracts_dir,
            format!("I{}Service.cs", self.name),
        )
    }

    pub fn create_service(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        let re = Regex::new(r"public ([a-zA-Z\\ ]+)")?;
        let properties: Vec<(_, _)> = re
            .captures_iter(&self.properties)
            .map(|m| {
                println!("tttt:{:?}", m);
                let mut splited = m.get(1).unwrap().as_str().trim().split(' ');
                let t = splited.next().unwrap();
                let p = splited.next().unwrap();
                (p, t)
            })
            .collect();
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("properties", Box::new(properties));
        kv.insert("custom", Box::new(custom));
        let application_dir = self
            .find(".Application", false)
            .path()
            .to_str()
            .unwrap()
            .to_string();

        self.create_dir(&application_dir)?;
        self.generate_template(
            kv,
            "Application/Service.cs",
            &application_dir,
            format!("{}Service.cs", &self.name),
        )
    }

    pub fn insert_mapper(&self) -> Result<(), CodeGeneratorError> {
        let mapper_file_path = self.find("ApplicationAutoMapperProfile", true);

        let mapper_file_path = mapper_file_path.path().to_str().unwrap();
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(&mapper_file_path)
            .expect("create failed");
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let index = code.rfind(';').unwrap();
        code.insert_str(
            index + 1,
            format!(
                "\r\n\t\tCreateMap<{0}, {0}Dto>();\r\n\t\tCreateMap<CreateOrUpdate{0}Dto, {0}>();",
                self.name
            )
            .as_str(),
        );
        code.insert_str(
            0,
            format!("using {}.{};\r\n", &self.namespace, self.plural_name).as_str(),
        );
        file.seek_write(code.as_bytes(), 0)?;
        Ok(())
    }
}

impl Entity {
    fn generate_template<T>(
        &self,
        kv: HashMap<&str, Box<T>>,
        template_name: &str,
        dir: &str,
        file_name: String,
    ) -> Result<(), CodeGeneratorError>
    where
        T: Serialize + ?Sized,
    {
        let file_path = vec![dir, self.plural_name.as_str(), file_name.as_str()].join("\\");

        let mut context = Context::new();
        // context.insert("numbers", &vec![1, 2, 3]);
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
        Ok(())
    }
}
