use std::{
    cell::RefCell, collections::HashMap, error::Error, io::Read, os::windows::prelude::FileExt,
};
extern crate inflector;
use super::{find, format_code, format_single_file, generate_template, open_file};
use crate::{
    dynamicDic,
    entities::{get_class_name, get_generic_type, get_namespace, get_properties, read_file},
    error::CodeGeneratorError,
};
use inflector::Inflector;
use log::info;

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
    ) -> Result<String, Box<dyn Error>> {
        let name = name.to_title_case();
        let kv = dynamicDic![
            ("namespace", &namespace),
            ("entityName", &name),
            ("type", &id_type)
        ];
        let file_full_path =
            vec![dir.trim_end_matches('\\'), format!("{}.cs", name).as_str()].join("\\");
        let file_full_path = generate_template(kv, "Domain/Entity.cs", &file_full_path)?;
        format_single_file(file_full_path.clone())?;
        Ok(file_full_path)
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

    pub fn create_dto(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("folder", &self.plural_name),
            ("entity", &self.name),
            ("id", &self.id_type),
            ("properties", &self.properties)
        ];
        let dto_path = format!("{}\\{}Dto.cs", dir.trim_end_matches('\\'), self.name);
        let path = generate_template(kv, "Application.Contracts/Dto.cs", &dto_path)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_repository_interface(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entities", &self.plural_name),
            ("entity", &self.name),
            ("generic_type", &self.id_type)
        ];
        let irepository_dir = format!(
            "{}\\I{}Repository.cs",
            dir.trim_end_matches('\\'),
            self.name
        );
        let path = generate_template(kv, "Domain/IRepository.cs", &irepository_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_manager(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entities", &self.plural_name),
            ("entity", &self.name)
        ];
        let imanager_dir = format!("{}\\{}Manager.cs", dir.trim_end_matches('\\'), self.name);
        let path = generate_template(kv, "Domain/Manager.cs", &imanager_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_exception(
        &self,
        exception_name: String,
        dir: String,
    ) -> Result<(), CodeGeneratorError> {
        let mut exception_name = exception_name;
        if !exception_name.ends_with("Exception") {
            exception_name.push_str("Exception");
        }
        if !exception_name.starts_with(&self.name) {
            exception_name.insert_str(0, &self.name);
        }
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entities", &self.plural_name),
            ("entity", &self.name),
            ("exception_name", &exception_name),
            ("project_name", self.namespace.split('.').last().unwrap())
        ];

        let exception_dir = format!("{}\\{}.cs", dir.trim_end_matches('\\'), exception_name);

        let path = generate_template(kv, "Domain/Exception.cs", &exception_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }
    #[allow(dead_code)]
    fn _insert_error_code(
        &self,
        exception_name: &str,
        exception_code: String,
    ) -> Result<(), CodeGeneratorError> {
        let error_code_file_path = find(&self.src_dir, "ErrorCodes.cs", true).unwrap();

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
    #[allow(dead_code)]
    fn insert_dispaly_text(
        &self,
        exception_code: String,
        exception_display_text: String,
    ) -> Result<(), CodeGeneratorError> {
        let json_path = find(&self.src_dir, "zh-Hans.json", true).unwrap();

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
    pub fn create_create_and_update_dto(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("folder", &self.plural_name),
            ("entity", &self.name),
            ("properties", &self.properties)
        ];
        let kv1 = dynamicDic![
            ("namespace", &self.namespace),
            ("folder", &self.plural_name),
            ("entity", &self.name),
            ("properties", &self.properties)
        ];
        let create_dto_path = format!("{}\\Create{}Dto.cs", dir.trim_end_matches('\\'), self.name);
        let update_dto_path = format!("{}\\Update{}Dto.cs", dir.trim_end_matches('\\'), self.name);
        let create_dto_path =
            generate_template(kv, "Application.Contracts/CreateDto.cs", &create_dto_path)?;
        let update_dto_path =
            generate_template(kv1, "Application.Contracts/UpdateDto.cs", &update_dto_path)?;
        self.add_file_change_log(create_dto_path);
        self.add_file_change_log(update_dto_path);
        Ok(())
    }

    pub fn create_pagedandsortedandfilterresultdto(
        &self,
        dir: String,
    ) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("folder", &self.plural_name),
            ("properties", &self.properties)
        ];
        let page_request_dto = format!(
            "{}\\PagedAndSortedAndFilteredResultRequestDto.cs",
            dir.trim_end_matches('\\'),
        );
        let path = generate_template(
            kv,
            "Application.Contracts/PagedAndSortedAndFilteredResultRequestDto.cs",
            &page_request_dto,
        )?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_iservice(&self, custom: bool, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entity", &self.name),
            ("folder", &self.plural_name),
            ("id", &self.id_type),
            ("custom", custom)
        ];
        let iservice_dir = format!(
            "{}\\I{}AppService.cs",
            dir.trim_end_matches('\\'),
            self.name
        );
        let path = generate_template(kv, "Application.Contracts/IService.cs", &iservice_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_service(&self, custom: bool, dir: String) -> Result<(), CodeGeneratorError> {
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entity", &self.name),
            ("folder", &self.plural_name),
            ("id", &self.id_type),
            ("properties", &self.properties),
            ("custom", custom)
        ];
        let service_dir = format!("{}\\{}AppService.cs", dir.trim_end_matches('\\'), self.name);
        let path = generate_template(kv, "Application/Service.cs", &service_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn create_ef_repository(&self, dir: String) -> Result<(), CodeGeneratorError> {
        let dbcontext_path = find(&self.src_dir, "DbContext.cs", true)
            .unwrap()
            .path()
            .display()
            .to_string();
        let dbcontext_code = read_file(&dbcontext_path)?;
        let dbconetxt_class_name = get_class_name(&dbcontext_code)?;
        let kv = dynamicDic![
            ("namespace", &self.namespace),
            ("entity", &self.name),
            ("entities", &self.plural_name),
            ("generic_type", &self.id_type),
            ("dbcontext", &dbconetxt_class_name)
        ];

        let ef_core_dir = format!(
            "{}\\EfCore{}Repository.cs",
            dir.trim_end_matches('\\'),
            self.name
        );
        let path = generate_template(kv, "EfCore/EfCoreRepository.cs", &ef_core_dir)?;
        self.add_file_change_log(path);
        Ok(())
    }

    pub fn insert_mapper(&self, mapper_file_path: String) -> Result<(), CodeGeneratorError> {
        let mut file = open_file(&mapper_file_path)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let map_to_dto = format!("CreateMap<{0}, {0}Dto>();", self.name);
        let map_to_entity1 = format!("CreateMap<Create{0}Dto, {0}>();", self.name);
        let map_to_entity2 = format!("CreateMap<Update{0}Dto, {0}>();", self.name);
        if code.contains(map_to_dto.as_str()) {
            return Ok(());
        }
        let index = code.rfind(';').unwrap();
        code.insert_str(
            index + 1,
            format!(
                "\r\n{}\r\n{}\r\n{}",
                map_to_dto, map_to_entity1, map_to_entity2
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

    pub fn insert_efcore_entity_config(
        &self,
        db_context_file: String,
    ) -> Result<(), CodeGeneratorError> {
        let mut file = open_file(&db_context_file)?;
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
        let db_context_consts_class_name = format!(
            "{}Consts",
            &self.namespace.split('.').into_iter().last().unwrap()
        );
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
            self.name, self.plural_name, db_context_consts_class_name
        );
        code.insert_str(config_index + 2, format!("\r\n{}", insert_config).as_str());
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(db_context_file);
        Ok(())
    }

    fn add_file_change_log(&self, path: String) {
        let mut changs = self.changed_files.borrow_mut();
        changs.push(path);
    }
}

// 格式化
impl Entity {
    pub fn format_all(&self) -> Result<(), CodeGeneratorError> {
        let files = self.changed_files.borrow().to_vec();
        format_code(self.solution_dir.clone(), files)
    }
}
