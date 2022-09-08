use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::{create_dir, File, OpenOptions},
    io::{ErrorKind, Read},
    os::windows::prelude::FileExt,
};
extern crate inflector;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
use inflector::Inflector;
use log::info;

use serde::Serialize;
use tera::Context;

use crate::{
    entities::{get_class_name, get_generic_type, get_namespace, get_properties},
    error::CodeGeneratorError,
    TEMPLATES,
};

use super::{find, format_csharp_code};

#[derive(Debug)]
pub struct Entity {
    namespace: String,
    id_type: String,
    name: String,
    src_dir: String,
    entity_dir: String,
    plural_name: String, //复数名字
    properties: HashMap<String, String>,
    changed_files: RefCell<Vec<String>>,
}

impl Entity {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let mut file = File::open(&path)?;
        let mut code = vec![];
        file.read_to_end(&mut code)?;
        let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();

        let entity_name = get_class_name(&code)?;
        let entity_names = entity_name.to_plural();

        let id_type = get_generic_type(&code)?;

        let namespace = get_namespace(&code)?;
        let namespace = namespace.replace((String::from(".") + &entity_names).as_str(), "");

        let properties = get_properties(&code).unwrap();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().position(|&i| i.contains("src")).unwrap();
        let entity_dir = src_dir[..(src_dir.len() - 1)].join("\\");
        let src_dir = src_dir[..(src_index + 1)].join("\\");

        info!("初始化完成");
        Ok(Entity {
            id_type,
            name: entity_name,
            namespace,
            plural_name: entity_names,
            src_dir,
            entity_dir,
            properties,
            changed_files: RefCell::new(vec![]),
        })
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
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("properties", Box::new(&self.properties));

        let application_contracts_dir = find(&self.src_dir, ".Application.Contracts", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        let path = self.generate_template(
            kv,
            "Application.Contracts/Dto.cs",
            &application_contracts_dir,
            format!("{}Dto.cs", self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_repository_interface(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entities", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("generic_type", Box::new(&self.id_type));

        let domain_dir = find(&self.src_dir, ".Domain", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&domain_dir)?;
        let path = self.generate_template(
            kv,
            "Domain/IRepository.cs",
            &domain_dir,
            format!("I{}Repository.cs", self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }
    pub fn create_manager(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entities", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));

        let domain_dir = find(&self.src_dir, ".Domain", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&domain_dir)?;
        let path = self.generate_template(
            kv,
            "Domain/Manager.cs",
            &domain_dir,
            format!("{}Manager.cs", self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }
    
    pub fn create_exception(&self, exception_name:Option<String>,exception_code:Option<String>,exception_display_text:Option<String>) -> Result<(), CodeGeneratorError> {
        let exception_name = exception_name.unwrap_or("Template".to_owned());
        let exception_code = exception_code.unwrap_or("TemplateCode".to_owned());
        let exception_display_text = exception_display_text.unwrap_or("TemplateDiaplayText".to_owned());
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entities", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("exception_name", Box::new(&exception_name));

        let domain_dir = find(&self.src_dir, ".Domain", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&domain_dir)?;
        let path = self.generate_template(
            kv,
            "Domain/Exception.cs",
            &domain_dir,
            format!("{}{}Exception.cs", self.name,exception_name),
        )?;
        self.insert_error_code(&exception_name, exception_code.clone())?;
        self.insert_dispaly_text(exception_code.clone(), exception_display_text)?;
        self.add_file_change_log(path);
        Ok(())
    }
    fn insert_error_code(&self,exception_name:&str,exception_code:String)->Result<(), CodeGeneratorError>{
        let error_code_file_path = find(&self.src_dir, "ErrorCodes.cs", true);

        let mapper_file_path = error_code_file_path.path().to_str().unwrap();
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
                "\r\npublic const string {}{} = \"{}\";",
                self.name,exception_name,exception_code
            )
            .as_str(),
        );
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(mapper_file_path.to_owned());
        Ok(())  
    }
    fn insert_dispaly_text(&self,exception_code:String,exception_display_text:String)->Result<(), CodeGeneratorError>{
        let json_path = find(&self.src_dir, "zh-Hans.json", true);

        let mapper_file_path = json_path.path().to_str().unwrap();
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(&mapper_file_path)
            .expect("create failed");
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let index = code.rfind('"').unwrap();
        code.insert_str(
            index + 1,
            format!(
                ",\r\n\"{}\": \"{}\"",
                exception_code,exception_display_text
            )
            .as_str(),
        );
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(mapper_file_path.to_owned());
        Ok(())  
    }
    pub fn create_createorupdatedto(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("properties", Box::new(&self.properties));
        let application_contracts_dir = find(&self.src_dir, ".Application.Contracts", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        let path = self.generate_template(
            kv,
            "Application.Contracts/CreateOrUpdateDto.cs",
            &application_contracts_dir,
            format!("CreateOrUpdate{}Dto.cs", self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_pagedandsortedandfilterresultdto(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("properties", Box::new(&self.properties));
        let application_contracts_dir = find(&self.src_dir, ".Application.Contracts", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&application_contracts_dir)?;
        let path = self.generate_template(
            kv,
            "Application.Contracts/PagedAndSortedAndFilteredResultRequestDto.cs",
            &application_contracts_dir,
            String::from("PagedAndSortedAndFilteredResultRequestDto.cs"),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_iservice(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("custom", Box::new(custom));
        let application_contracts_dir =find(&self.src_dir, ".Application.Contracts", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&application_contracts_dir)?;

        let path = self.generate_template(
            kv,
            "Application.Contracts/IService.cs",
            &application_contracts_dir,
            format!("I{}Service.cs", self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_service(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("folder", Box::new(&self.plural_name));
        kv.insert("id", Box::new(&self.id_type));
        kv.insert("properties", Box::new(&self.properties));
        kv.insert("custom", Box::new(custom));
        let application_dir = find(&self.src_dir, ".Application.Contracts", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&application_dir)?;
        let path = self.generate_template(
            kv,
            "Application/Service.cs",
            &application_dir,
            format!("{}Service.cs", &self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    // pub fn create_exception(&self,exception_name:&str) -> Result<(), CodeGeneratorError> {
    //     let exception_name = exception_name.trim_end_matches("Exception");
    //     let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
    //     kv.insert("namespace", Box::new(&self.namespace));
    //     kv.insert("entities", Box::new(&self.plural_name));
    //     kv.insert("exception_name", Box::new(exception_name));
    //     kv.insert("error_codes", Box::new(&self.id_type));
    //     let error_codes = self
    //         .find("ErrorCodes.cs", true)
    //         .path()
    //         .to_str()
    //         .unwrap()
    //         .to_string();

    //     self.generate_template(
    //         kv,
    //         "Domain/Exception.cs",
    //         &self.,
    //         format!("{}Service.cs", &self.name),
    //     )
    // }

    pub fn insert_mapper(&self) -> Result<(), CodeGeneratorError> {
        let mapper_file_path = find(&self.src_dir, ".Application.Contracts", false);

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
        self.add_file_change_log(mapper_file_path.to_owned());
        Ok(())
    }
    
    fn add_file_change_log(&self, path: String) {
        let mut changs = self.changed_files.borrow_mut();
        changs.push(path);
    }

}

impl Entity {
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
        let file_path = vec![dir, self.plural_name.as_str(), file_name.as_str()].join("\\");

        let mut context = Context::new();
        for entity in kv {
            context.insert(entity.0, &entity.1);
        }

        let file = File::create(&file_path)?;
        match TEMPLATES.render_to(template_name, &context, file) {
            Ok(()) => println!("{} 生成成功！", file_path),
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
impl Entity {
    pub fn format_all(&self) {
        self.format_application_project();
        self.format_application_contracts_project();
        self.format_domain_project();
        self.format_domain_share_project();
    }
    pub fn format_application_project(&self) {
        let project_dir = find(&self.src_dir, ".Application", false);
        let work_dir = project_dir.path().display().to_string();
        let files = self.changed_files.borrow().to_vec();
        let files = files
            .into_iter()
            .filter(|f| f.starts_with(work_dir.as_str()))
            .map(|f| f.trim_start_matches(work_dir.as_str()).to_owned())
            .collect::<Vec<_>>();
        format_csharp_code(work_dir, files)
    }
    pub fn format_application_contracts_project(&self) {
        let project_dir = find(&self.src_dir, "Application.Contracts", false);
        let work_dir = project_dir.path().display().to_string();
        let files = self.changed_files.borrow().to_vec();
        let files = files
            .into_iter()
            .filter(|f| f.starts_with(work_dir.as_str()))
            .map(|f| f.trim_start_matches(work_dir.as_str()).to_owned())
            .collect::<Vec<_>>();
        format_csharp_code(work_dir, files)
    }
    pub fn format_domain_project(&self) {
        let domain_dir = find(&self.src_dir, ".Domain", false);
        let work_dir = domain_dir.path().display().to_string();
        let files = self.changed_files.borrow().to_vec();
        let files = files
            .into_iter()
            .filter(|f| f.starts_with(work_dir.as_str()))
            .map(|f| f.trim_start_matches(work_dir.as_str()).to_owned())
            .collect::<Vec<_>>();
        format_csharp_code(work_dir, files)
    }
    pub fn format_domain_share_project(&self) {
        let project_dir = find(&self.src_dir, "Domain.Shared", false);
        let work_dir = project_dir.path().display().to_string();
        let files = self.changed_files.borrow().to_vec();
        let files = files
            .into_iter()
            .filter(|f| f.starts_with(work_dir.as_str()))
            .map(|f| f.trim_start_matches(work_dir.as_str()).to_owned())
            .collect::<Vec<_>>();
        format_csharp_code(work_dir, files)
    }
}
