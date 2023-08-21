use askama::Template;
use std::{cell::RefCell, collections::HashMap, fs::create_dir_all, path::Path};
extern crate inflector;
use super::open_file;
use crate::{args::PropertyInfo, entities::read_file, ClassInfo, CodeGeneratorError};

mod filters {
    use inflector::Inflector;

    // This filter does not have extra arguments
    pub fn pluralize<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.to_plural())
    }
    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.to_camel_case())
    }
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Dto/Dto.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct DtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Dto/CreateDto.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct CreateDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Dto/UpdateDto.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct UpdateDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Dto/QueryDto.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct QueryDtoTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/IService.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct IServiceTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Service.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct ServiceTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/IRepository.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct IRepositoryTemplate {
    namespace: String,
    class_name: String,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Repository.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
struct RepositoryTemplate {
    namespace: String,
    class_name: String,
    properties: Vec<PropertyInfo>,
}
#[derive(Template)] // this will generate the code...
#[template(path = "WES/Controller.cs", escape = "none")] // using the template in this path, relative // to the `templates` dir in the crate root
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

#[cfg(test)]
mod tests {
    use askama::Template;

    use crate::entities::wes::IServiceTemplate;

    #[test]
    fn generate() {
        // entity
        //     .create_dto(String::from(r"D:\code\WES\WES.Entity\Model"))
        //     .unwrap();

        // // entity
        // //     .create_exception(String::from(r"D:\code\WES\WES.Entity\Exceptions"))
        // //     .unwrap();

        let iservice = IServiceTemplate {
            namespace: "xyz".to_owned(),
            class_name: "aaaa".to_owned(),
        };
        println!("{}", iservice.render().unwrap());
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
