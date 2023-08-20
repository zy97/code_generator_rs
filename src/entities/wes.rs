use askama::Template;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    os::windows::prelude::FileExt,
    path::{self, Path},
};
extern crate inflector;
use super::{create_dir, find, open_file};
use crate::{args::PropertyInfo, entities::read_file, ClassInfo, CodeGeneratorError};
use inflector::Inflector;
use log::info;
use std::fmt::Write as _;
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Dto/Dto.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct DtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
pub fn create_dto(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let dto = DtoTemplate {
        namespace,
        class_name: class.class_name,
        properties: class.property_infos,
    };
    match output {
        Some(file) => {
            let path: &Path = Path::new(&file);
            if path.is_dir() {
                println!("输出地址应该是个文件，不应该是个文件夹路径");
                return Ok(());
            }
            match path.parent() {
                Some(dir) => {
                    create_dir_all(dir)?;
                }
                None => {}
            }
            let mut file = open_file(&file)?;
            let res = dto.write_into(&mut file)?;
        }
        None => {
            let res = dto.render()?;
            println!("{}", res);
        }
    }

    // let kv = dynamicDic![
    //     ("namespace", "WES.Entity.Dto"),
    //     ("folder", &self.plural_name),
    //     ("entity", &self.name),
    //     ("properties", &self.properties)
    // ];
    // let kv1 = dynamicDic![
    //     ("namespace", "WES.Entity.Dto"),
    //     ("folder", &self.plural_name),
    //     ("entity", "Create".to_owned() + &self.name),
    //     ("properties", &self.properties)
    // ];
    // let kv2 = dynamicDic![
    //     ("namespace", "WES.Entity.Dto"),
    //     ("folder", &self.plural_name),
    //     ("entity", "Update".to_owned() + &self.name),
    //     ("properties", &self.properties)
    // ];
    // let kv3 = dynamicDic![
    //     ("namespace", "WES.Entity.Dto"),
    //     ("folder", &self.plural_name),
    //     ("entity", "Query".to_owned() + &self.name),
    //     ("properties", &self.properties)
    // ];
    // let dto_dir = format!("{}\\{}", dir.trim_end_matches('\\'), self.plural_name,);
    // create_dir(&dto_dir);
    // let dto_path = format!("{}\\{}Dto.cs", dto_dir, self.name);
    // let create_dto_path = format!("{}\\Create{}Dto.cs", dto_dir, self.name);
    // let update_dto_path = format!("{}\\Update{}Dto.cs", dto_dir, self.name);
    // let query_dto_path = format!("{}\\Query{}Dto.cs", dto_dir, self.name);
    // generate_template(kv, "WES/Dto/Dto.cs", &dto_path)?;
    // generate_template(kv1, "WES/Dto/CreateDto.cs", &create_dto_path)?;
    // generate_template(kv2, "WES/Dto/UpdateDto.cs", &update_dto_path)?;
    // generate_template(kv3, "WES/Dto/QueryDto.cs", &query_dto_path)?;
    Ok(())
}
#[derive(Debug, Default)]
pub struct Entity {
    namespace: String,
    name: String,
    src_dir: String,
    solution_dir: String,
    plural_name: String, //复数名字
    properties: HashMap<String, String>,
    changed_files: RefCell<Vec<String>>,
}

impl Entity {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let code = read_file(&path)?;

        // let entity_name = get_class_name(&code)?;
        // let entity_names = entity_name.to_plural();

        // let namespace = get_namespace(&code)?;
        // let namespace = namespace.replace((String::from(".") + &entity_names).as_str(), "");

        // let properties = get_properties(&code).unwrap();

        // let src_dir = path.split('\\').collect::<Vec<&str>>();
        // let src_index = src_dir.iter().position(|&i| i.eq("WES")).unwrap();
        // let solution_dir = src_dir[..(src_index)].join("\\");
        // let src_dir = src_dir[..(src_index + 1)].join("\\");
        // info!("初始化完成");
        // Ok(Entity {
        //     solution_dir,
        //     name: entity_name,
        //     namespace,
        //     plural_name: entity_names,
        //     src_dir,
        //     properties,
        //     changed_files: RefCell::new(vec![]),
        // })
        Ok(Entity::default())
    }

    pub fn create_dto(&self, dir: String) -> Result<(), CodeGeneratorError> {
        // let kv = dynamicDic![
        //     ("namespace", "WES.Entity.Dto"),
        //     ("folder", &self.plural_name),
        //     ("entity", &self.name),
        //     ("properties", &self.properties)
        // ];
        // let kv1 = dynamicDic![
        //     ("namespace", "WES.Entity.Dto"),
        //     ("folder", &self.plural_name),
        //     ("entity", "Create".to_owned() + &self.name),
        //     ("properties", &self.properties)
        // ];
        // let kv2 = dynamicDic![
        //     ("namespace", "WES.Entity.Dto"),
        //     ("folder", &self.plural_name),
        //     ("entity", "Update".to_owned() + &self.name),
        //     ("properties", &self.properties)
        // ];
        // let kv3 = dynamicDic![
        //     ("namespace", "WES.Entity.Dto"),
        //     ("folder", &self.plural_name),
        //     ("entity", "Query".to_owned() + &self.name),
        //     ("properties", &self.properties)
        // ];
        // let dto_dir = format!("{}\\{}", dir.trim_end_matches('\\'), self.plural_name,);
        // create_dir(&dto_dir);
        // let dto_path = format!("{}\\{}Dto.cs", dto_dir, self.name);
        // let create_dto_path = format!("{}\\Create{}Dto.cs", dto_dir, self.name);
        // let update_dto_path = format!("{}\\Update{}Dto.cs", dto_dir, self.name);
        // let query_dto_path = format!("{}\\Query{}Dto.cs", dto_dir, self.name);
        // generate_template(kv, "WES/Dto/Dto.cs", &dto_path)?;
        // generate_template(kv1, "WES/Dto/CreateDto.cs", &create_dto_path)?;
        // generate_template(kv2, "WES/Dto/UpdateDto.cs", &update_dto_path)?;
        // generate_template(kv3, "WES/Dto/QueryDto.cs", &query_dto_path)?;
        Ok(())
    }

    // pub fn create_repository_interface(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let kv = dynamicDic![
    //         ("namespace", "WES.Repository.IRepository"),
    //         ("entities", &self.plural_name),
    //         ("entity", &self.name)
    //     ];
    //     let irepository_dir = format!(
    //         "{}\\I{}Repository.cs",
    //         dir.trim_end_matches('\\'),
    //         self.name
    //     );
    //     let path = generate_template(kv, "WES/IRepository.cs", &irepository_dir)?;
    //     Ok(())
    // }
    // pub fn create_repository(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let kv = dynamicDic![
    //         ("namespace", "WES.Repository.Repository"),
    //         ("entities", &self.plural_name),
    //         ("entity", &self.name),
    //         ("properties", &self.properties)
    //     ];
    //     let irepository_dir = format!("{}\\{}Repository.cs", dir.trim_end_matches('\\'), self.name);
    //     let path = generate_template(kv, "WES/Repository.cs", &irepository_dir)?;
    //     Ok(())
    // }

    // pub fn create_exception(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let already_exists_status = "AlreadyExists";
    //     let not_found_status = "NotFound";
    //     let kv = dynamicDic![
    //         ("namespace", &self.namespace),
    //         ("entities", &self.plural_name),
    //         ("entity", &self.name),
    //         ("status", already_exists_status),
    //         ("project_name", self.namespace.split('.').last().unwrap())
    //     ];
    //     let kv1 = dynamicDic![
    //         ("namespace", &self.namespace),
    //         ("entities", &self.plural_name),
    //         ("entity", &self.name),
    //         ("status", not_found_status),
    //         ("project_name", self.namespace.split('.').last().unwrap())
    //     ];
    //     let exception_dir = format!("{}\\{}", dir.trim_end_matches('\\'), self.name);
    //     create_dir(&exception_dir)?;
    //     let already_exists_exception_path = format!(
    //         "{}\\{}{}Exception.cs",
    //         exception_dir, &self.name, already_exists_status
    //     );
    //     let not_found_exception_path = format!(
    //         "{}\\{}{}Exception.cs",
    //         exception_dir, &self.name, not_found_status
    //     );

    //     generate_template(kv, "WES/Exception.cs", &already_exists_exception_path)?;
    //     generate_template(kv1, "WES/Exception.cs", &not_found_exception_path)?;
    //     Ok(())
    // }

    // pub fn create_iservice(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let kv = dynamicDic![
    //         ("namespace", "WES.Services.IServices"),
    //         ("entity", &self.name),
    //         ("entities", &self.plural_name)
    //     ];
    //     let iservice_dir = format!("{}\\I{}Service.cs", dir.trim_end_matches('\\'), self.name);
    //     let path = generate_template(kv, "WES/IService.cs", &iservice_dir)?;
    //     Ok(())
    // }

    // pub fn create_service(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let kv = dynamicDic![
    //         ("namespace", "WES.Services.Services"),
    //         ("entity", &self.name),
    //         ("entities", &self.plural_name),
    //         ("properties", &self.properties)
    //     ];
    //     let service_dir = format!("{}\\{}Service.cs", dir.trim_end_matches('\\'), self.name);
    //     let path = generate_template(kv, "WES/Service.cs", &service_dir)?;
    //     Ok(())
    // }
    // pub fn create_controller(&self, dir: String) -> Result<(), CodeGeneratorError> {
    //     let kv = dynamicDic![
    //         ("namespace", "WES.API.Controllers"),
    //         ("entity", &self.name),
    //         ("entities", &self.plural_name),
    //         ("properties", &self.properties)
    //     ];
    //     let service_dir = format!("{}\\{}Controller.cs", dir.trim_end_matches('\\'), self.name);
    //     generate_template(kv, "WES/Controller.cs", &service_dir)?;
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::Entity;

    #[test]
    fn generate() {
        let entity_dir = r"D:\code\WES\WES.Entity\Entity\OutboundLock.cs";
        let entity = Entity::new(entity_dir.to_owned()).unwrap();
        println!("{:?}", entity);

        // entity
        //     .create_dto(String::from(r"D:\code\WES\WES.Entity\Model"))
        //     .unwrap();

        // // entity
        // //     .create_exception(String::from(r"D:\code\WES\WES.Entity\Exceptions"))
        // //     .unwrap();

        // entity
        //     .create_repository_interface(String::from(r"D:\code\WES\WES.Repository\IRepository"))
        //     .unwrap();

        // entity
        //     .create_repository(String::from(r"D:\code\WES\WES.Repository\Repository"))
        //     .unwrap();

        // entity
        //     .create_iservice(String::from(r"D:\code\WES\WES.Services\IServices"))
        //     .unwrap();

        // entity
        //     .create_service(String::from(r"D:\code\WES\WES.Services\Services"))
        //     .unwrap();

        // entity
        //     .create_controller(String::from(r"D:\code\WES\WES.API\Controllers"))
        //     .unwrap();
    }
}
