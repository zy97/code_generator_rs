use askama::Template;
use std::{fs::create_dir_all, path::Path};
extern crate inflector;
use super::open_file;
use crate::{args::PropertyInfo, ClassInfo, CodeGeneratorError};

mod filters {
    use inflector::Inflector;

    pub fn pluralize<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.to_plural())
    }
    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.to_camel_case())
    }
}
#[derive(Template)]
#[template(path = "WES/Dto/Dto.cs", escape = "none")]
struct DtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)]
#[template(path = "WES/Dto/CreateDto.cs", escape = "none")]
struct CreateDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)]
#[template(path = "WES/Dto/UpdateDto.cs", escape = "none")]
struct UpdateDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)]
#[template(path = "WES/Dto/QueryDto.cs", escape = "none")]
struct QueryDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)]
#[template(path = "WES/IService.cs", escape = "none")]
struct IServiceTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)]
#[template(path = "WES/Service.cs", escape = "none")]
struct ServiceTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)]
#[template(path = "WES/IRepository.cs", escape = "none")]
struct IRepositoryTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)]
#[template(path = "WES/Repository.cs", escape = "none")]
struct RepositoryTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)]
#[template(path = "WES/Controller.cs", escape = "none")]
struct ControllerTemplate {
    namespace: String,
    class_name: String,
}
fn render<T: Template>(template: T, output: Option<String>) -> Result<(), CodeGeneratorError> {
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
                None => (),
            }
            let mut file = open_file(&file)?;
            let res = template.write_into(&mut file)?;
            Ok(())
        }
        None => {
            let res = template.render()?;
            println!("{}", res);
            Ok(())
        }
    }
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
    render(dto, output)
}
pub fn create_create_dto(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let create_dto = CreateDtoTemplate {
        namespace,
        class_name: class.class_name,
        properties: class.property_infos,
    };
    render(create_dto, output)
}
pub fn create_query_dto(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let create_dto = QueryDtoTemplate {
        namespace,
        class_name: class.class_name,
        properties: class.property_infos,
    };
    render(create_dto, output)
}
pub fn create_update_dto(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let create_dto = UpdateDtoTemplate {
        namespace,
        class_name: class.class_name,
        properties: class.property_infos,
    };
    render(create_dto, output)
}
pub fn create_irepository(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let irepository = IRepositoryTemplate {
        namespace,
        class_name: class.class_name,
    };
    render(irepository, output)
}
pub fn create_repository(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let repository = RepositoryTemplate {
        namespace,
        class_name: class.class_name,
        properties: class.property_infos,
    };
    render(repository, output)
}
pub fn create_iservice(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let iservice = IServiceTemplate {
        namespace,
        class_name: class.class_name,
    };
    render(iservice, output)
}
pub fn create_service(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let service = ServiceTemplate {
        namespace,
        class_name: class.class_name,
    };
    render(service, output)
}
pub fn create_controller(
    class: ClassInfo,
    namespace: String,
    output: Option<String>,
) -> Result<(), CodeGeneratorError> {
    let controller = ControllerTemplate {
        namespace,
        class_name: class.class_name,
    };
    render(controller, output)
}
