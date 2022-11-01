use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::{create_dir, File},
    io::{ErrorKind, Read},
    os::windows::prelude::FileExt,
    path::Path,
};
extern crate inflector;
use inflector::Inflector;
use log::info;

use serde::Serialize;
use tera::Context;

use crate::{
    entities::{get_class_name, get_generic_type, get_namespace, get_properties, read_file},
    error::CodeGeneratorError,
    TEMPLATES,
};

use super::{find, format_code, format_single_file, generate_template, open_file};

#[derive(Debug)]
pub struct Entity {
    namespace: String,
    id_type: String,
    name: String,
    src_dir: String,
    solution_dir: String,
    plural_name: String, //复数名字
    properties: HashMap<String, String>,
    changed_files: RefCell<Vec<String>>,
}
//创建实体文件
impl Entity {
    pub fn create_entity(
        namespace: String,
        id_type: String,
        name: String,
        dir: String,
    ) -> Result<(), Box<dyn Error>> {
        let name = name.to_title_case();
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(namespace));
        kv.insert("entityName", Box::new(&name));
        kv.insert("type", Box::new(id_type));
        let file_full_path =
            vec![dir.trim_end_matches('\\'), format!("{}.cs", name).as_str()].join("\\");
        let file_full_path = generate_template(kv, "Domain/Entity.cs", &file_full_path)?;
        format_single_file(file_full_path)?;
        Ok(())
    }
}

impl Entity {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let code = read_file(&path)?;

        let entity_name = get_class_name(&code)?;
        let entity_names = entity_name.to_plural();

        let id_type = get_generic_type(&code)?;

        let namespace = get_namespace(&code)?;
        let namespace = namespace.replace((String::from(".") + &entity_names).as_str(), "");

        let properties = get_properties(&code).unwrap();

        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().position(|&i| i.contains("src")).unwrap();
        // let entity_dir = src_dir[..(src_dir.len() - 1)].join("\\");
        let solution_dir = src_dir[..(src_index)].join("\\");
        let src_dir = src_dir[..(src_index + 1)].join("\\");
        info!("初始化完成");
        Ok(Entity {
            solution_dir,
            id_type,
            name: entity_name,
            namespace,
            plural_name: entity_names,
            src_dir,
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

    pub fn create_exception(
        &self,
        exception_name: Option<String>,
        exception_code: Option<String>,
        exception_display_text: Option<String>,
    ) -> Result<(), CodeGeneratorError> {
        let exception_name = exception_name.unwrap_or("Template".to_owned());
        let exception_code = exception_code.unwrap_or("TemplateCode".to_owned());
        let exception_display_text =
            exception_display_text.unwrap_or("TemplateDiaplayText".to_owned());
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entities", Box::new(&self.plural_name));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("exception_name", Box::new(&exception_name));
        let error_code_file = find(&self.src_dir, "DomainErrorCodes.cs", true)
            .path()
            .display()
            .to_string();
        let code = read_file(&error_code_file)?;
        let class_name = get_class_name(&code)?;
        kv.insert("error_codes", Box::new(class_name));

        let domain_dir = find(&self.src_dir, ".Domain", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&domain_dir)?;
        let path = self.generate_template(
            kv,
            "Domain/Exception.cs",
            &domain_dir,
            format!("{}{}Exception.cs", self.name, exception_name),
        )?;
        self.insert_error_code(&exception_name, exception_code.clone())?;
        self.insert_dispaly_text(exception_code.clone(), exception_display_text)?;
        self.add_file_change_log(path);
        Ok(())
    }
    fn insert_error_code(
        &self,
        exception_name: &str,
        exception_code: String,
    ) -> Result<(), CodeGeneratorError> {
        let error_code_file_path = find(&self.src_dir, "ErrorCodes.cs", true);

        let mapper_file_path = error_code_file_path.path().to_str().unwrap();
        let mut file = open_file(mapper_file_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let insert_code = format!(
            "public const string {}{} = \"{}\";",
            self.name, exception_name, exception_code
        );
        if code.contains(&insert_code) {
            return Ok(());
        }
        let index = code.rfind(';').unwrap();
        code.insert_str(index + 1, format!("\r\n{}", insert_code).as_str());
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(mapper_file_path.to_owned());
        Ok(())
    }
    fn insert_dispaly_text(
        &self,
        exception_code: String,
        exception_display_text: String,
    ) -> Result<(), CodeGeneratorError> {
        let json_path = find(&self.src_dir, "zh-Hans.json", true);

        let json_file = json_path.path().to_str().unwrap();

        let mut file = open_file(json_file)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let insert_code = format!("\"{}\": \"{}\"", exception_code, exception_display_text);
        if code.contains(&insert_code) {
            return Ok(());
        }
        let index = code.rfind('"').unwrap();
        code.insert_str(index + 1, format!(",{}", insert_code).as_str());
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(json_file.to_owned());
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
        let application_contracts_dir = find(&self.src_dir, ".Application.Contracts", false)
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
        let application_dir = find(&self.src_dir, ".Application", false)
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

    pub fn create_ef_repository(&self) -> Result<(), CodeGeneratorError> {
        let mut kv: HashMap<&str, Box<dyn erased_serde::Serialize>> = HashMap::new();
        kv.insert("namespace", Box::new(&self.namespace));
        kv.insert("entity", Box::new(&self.name));
        kv.insert("entities", Box::new(&self.plural_name));
        kv.insert("generic_type", Box::new(&self.id_type));
        let dbcontext_path = find(&self.src_dir, "DbContext.cs", true)
            .path()
            .display()
            .to_string();
        let dbcontext_code = read_file(&dbcontext_path)?;
        let dbconetxt_class_name = get_class_name(&dbcontext_code)?;
        kv.insert("dbcontext", Box::new(dbconetxt_class_name));
        let ef_core_dir = find(&self.src_dir, ".EntityFrameworkCore", false)
            .path()
            .display()
            .to_string();

        self.create_dir(&ef_core_dir)?;
        let path = self.generate_template(
            kv,
            "EfCore/EfCoreRepository.cs",
            &ef_core_dir,
            format!("EfCore{}Repository.cs", &self.name),
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn insert_mapper(&self) -> Result<(), CodeGeneratorError> {
        let mapper_file_path = find(&self.src_dir, "ApplicationAutoMapperProfile.cs", true);

        let mapper_file_path = mapper_file_path.path().to_str().unwrap();

        let mut file = open_file(mapper_file_path)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let map_to_dto = format!("CreateMap<{0}, {0}Dto>();", self.name);
        let map_to_entity = format!("CreateMap<CreateOrUpdate{0}Dto, {0}>();", self.name);
        if code.contains(map_to_dto.as_str()) {
            return Ok(());
        }
        let index = code.rfind(';').unwrap();
        code.insert_str(
            index + 1,
            format!("\r\n{}\r\n{}", map_to_dto, map_to_entity).as_str(),
        );
        code.insert_str(
            0,
            format!("using {}.{};\r\n", &self.namespace, self.plural_name).as_str(),
        );
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(mapper_file_path.to_owned());
        Ok(())
    }

    pub fn insert_efcore_entity_config(&self) -> Result<(), CodeGeneratorError> {
        let dbcontext_path = find(&self.src_dir, "DbContext.cs", true)
            .path()
            .display()
            .to_string();
        let mut file = open_file(&dbcontext_path)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        let insert_dbset_property = format!(
            "public DbSet<{}> {} {{ get; set; }}",
            &self.name, &self.plural_name
        );
        if code.contains(&insert_dbset_property) {
            return Ok(());
        }
        let insert_namespace = format!("using {}.{};\r\n", self.namespace, self.plural_name);
        code.insert_str(0, &insert_namespace);
        let dbconetxt_class_name = get_class_name(&code)?;
        let constructor = format!("public {}", dbconetxt_class_name);
        let constructor_index = code.find(&constructor).unwrap();

        code.insert_str(
            constructor_index - 1,
            format!("{}\r\n", insert_dbset_property).as_str(),
        );
        let config_index = code.rfind(';').unwrap();
        let domain_dir = find(&self.src_dir, ".Domain", false)
            .path()
            .display()
            .to_string();
        let consts_path = find(&domain_dir, "Consts.cs", true)
            .path()
            .display()
            .to_string();
        let consts_code = read_file(&consts_path)?;
        let consts_class_name = get_class_name(&consts_code)?;
        let insert_config = format!(
            r###"
            builder.Entity<{}>(b =>
            {{
                        b.ToTable({2}.DbTablePrefix + "{}", {2}.DbSchema);
                        b.ConfigureByConvention(); //auto configure for the base class props
                        b.HasKey(i => i.Id);
                        //根据自己的情况配置
                        //b.Property(i => i.Name).IsRequired().HasMaxLength(FriendLinkConst.MaxNameLength);
                        //b.Property(i => i.Url).IsRequired().HasMaxLength(FriendLinkConst.MaxUrlLength);
            }});
                 "###,
            self.name, self.plural_name, consts_class_name
        );
        code.insert_str(config_index + 2, format!("\r\n{}", insert_config).as_str());
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(dbcontext_path);
        Ok(())
    }

    fn add_file_change_log(&self, path: String) {
        let mut changs = self.changed_files.borrow_mut();
        changs.push(path);
    }
}

//生成模板
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
            Ok(()) => eprintln!("{} create successful!", file_path),
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
// 格式化
impl Entity {
    pub fn format_all(&self) {
        let files = self.changed_files.borrow().to_vec();
        format_code(self.solution_dir.clone(), files)
    }
}
