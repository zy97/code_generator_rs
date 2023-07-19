mod entity;
mod permision;
mod web_entity;
mod wes;
pub use crate::entities::entity::Entity;
use encoding_rs::UTF_8;
use lazy_static::lazy_static;
pub use permision::Permission;
use regex::Regex;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::windows::process::ExitStatusExt,
    path::{self, Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    vec,
};
use tera::{Context, Tera};

use walkdir::DirEntry;
pub use web_entity::WebEntity;

use crate::{
    dynamic_dic,
    error::{CodeGeneratorError, RegexNoMatchError},
    TEMPLATES,
};
fn read_file(file: &str) -> Result<String, CodeGeneratorError> {
    let mut file = File::open(file)?;
    let mut code = vec![];
    file.read_to_end(&mut code)?;
    let (code, ..) = UTF_8.decode(&code);
    Ok(code.to_string())
}
fn open_file(file: &str) -> Result<File, CodeGeneratorError> {
    let mut options = OpenOptions::new();
    let file = options.write(true).read(true).open(&file)?;
    Ok(file)
}
// 创建文件夹
pub fn create_dir(dir: &str) -> Result<(), CodeGeneratorError> {
    if !Path::new(dir).exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn find(src_dir: &str, contain_name: &str, is_file: bool) -> Option<DirEntry> {
    walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            if is_file {
                e.file_type().is_file() && e.file_name().to_str().unwrap().contains(contain_name)
            } else {
                e.file_type().is_dir() && e.file_name().to_str().unwrap().contains(contain_name)
            }
        })
        .nth(0)
}
fn find_current_dir(src_dir: &str, contain_name: &str) -> Option<DirEntry> {
    walkdir::WalkDir::new(src_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() && e.file_name().to_str().unwrap().contains(contain_name)
        })
        .nth(0)
}
fn find_index(start_dir: &str, end_dir: &str) -> String {
    let mut dir = start_dir;
    let mut index_file_path = String::new();
    while end_dir != dir {
        let index_file = find_current_dir(dir, "index.ts");
        match index_file {
            Some(dir) => {
                index_file_path = dir.path().display().to_string();
                break;
            }
            None => dir = &dir[0..dir.rfind('\\').unwrap()],
        }
    }
    if index_file_path.len() == 0 {
        panic!("整个路径未找到index.ts文件");
    }
    index_file_path
}
fn get_class_name(content: &str) -> Result<String, CodeGeneratorError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"class ([a-zA-Z]+)").unwrap();
    }
    let entity_name = RE
        .captures(content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(entity_name)
}
fn get_generic_type(content: &str) -> Result<String, CodeGeneratorError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<([a-zA-Z]+)>").unwrap();
    }
    let t = RE
        .captures(&content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(t)
}
fn get_namespace(content: &str) -> Result<String, CodeGeneratorError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"namespace ([a-zA-Z.]+)").unwrap();
    }
    let namespace = RE
        .captures(&content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(namespace)
}
fn get_properties(content: &str) -> Result<HashMap<String, String>, CodeGeneratorError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"public ([a-zA-Z\\ ]+) \{").unwrap();
    }
    let mut kv = HashMap::new();
    for property in RE.captures_iter(&content) {
        let mut sdf = property
            .get(1)
            .ok_or(RegexNoMatchError)?
            .as_str()
            .split(' ');
        let t = sdf.next().unwrap();
        let name = sdf.next().unwrap();
        kv.insert(name.to_owned(), t.to_owned());
    }
    Ok(kv)
}
fn format_code(work_dir: String, files: Vec<String>) -> Result<(), CodeGeneratorError> {
    if files.len() == 0 {
        return Ok(());
    }
    let output = Command::new("cmd")
        .arg("/c")
        .current_dir(work_dir)
        .arg(format!(r"dprint fmt {}", files.join(" ")))
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    if &output.status == &ExitStatus::from_raw(1) {
        let (output, ..) = UTF_8.decode(&output.stderr);
        return Err(CodeGeneratorError::DprintError(output.to_string()));
    }
    println!("{:#?} format successful!", files);
    Ok(())
}
fn format_single_file(file: String) -> Result<(), CodeGeneratorError> {
    let work_dir = Path::new(&file).parent().unwrap().display().to_string();
    let output = Command::new("cmd")
        .arg("/c")
        .current_dir(work_dir)
        .arg(format!(r"dprint fmt {}", file))
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    if &output.status == &ExitStatus::from_raw(1) {
        let (output, ..) = UTF_8.decode(&output.stderr);
        return Err(CodeGeneratorError::DprintError(output.to_string()));
    }
    eprintln!("{} format successful!", file);
    Ok(())
}
pub fn get_expressions_in_template(
    content: impl AsRef<str>,
) -> Result<Vec<String>, CodeGeneratorError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{\{([a-zA-Z]+)\}\}").unwrap();
    }
    let mut res = RE
        .captures_iter(content.as_ref())
        .into_iter()
        .map(|m| m.get(1).unwrap().as_str().to_string())
        .collect::<Vec<String>>();
    let set: HashSet<_> = res.drain(..).collect();
    res.extend(set.into_iter());
    println!("{:?}", res);
    Ok(res)
}
pub async fn process_template<T>(id: i32, expressions: T) -> Result<String, CodeGeneratorError>
where
    T: Serialize,
{
    // let template = crate::services::templates_svc::find(id).await?;
    // match template {
    //     Some(template) => {
    //         let template_name = format!("{}/{}", template.project_id, template.id);
    //         let result =
    //             TEMPLATES.render(&template_name, &Context::from_serialize(&expressions)?)?;
    //         Ok(result)
    //     }
    //     None => return Err(CodeGeneratorError::NotFindEntity("未找到实体".to_owned())),
    // }
    todo!()
}
pub async fn process_template_to_file<T>(
    id: i32,
    expressions: T,
    file: PathBuf,
) -> Result<(), CodeGeneratorError>
where
    T: Serialize,
{
    // let template = crate::services::templates_svc::find(id).await?;
    // match template {
    //     Some(template) => {
    //         let template_name = format!("{}/{}", template.project_id, template.id);
    //         let file = File::create(file)?;
    //         Ok(TEMPLATES.render_to(
    //             &template_name,
    //             &Context::from_serialize(&expressions)?,
    //             file,
    //         )?)
    //     }
    //     None => return Err(CodeGeneratorError::NotFindEntity("未找到实体".to_owned())),
    // }
    todo!()
}
fn generate_template<T>(
    kv: HashMap<&str, Box<T>>,
    template_name: &str,
    file_full_name: &str,
) -> Result<String, CodeGeneratorError>
where
    T: Serialize + ?Sized,
{
    let mut context = Context::new();
    for entity in kv {
        context.insert(entity.0, &entity.1);
    }

    let file = File::create(file_full_name)?;
    TEMPLATES.render_to(template_name, &context, file)?;
    Ok(file_full_name.to_string())
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use encoding_rs::UTF_8;

    use crate::entities::get_expressions_in_template;

    #[test]
    fn get_expressions_in_template_test() {
        let mut file = File::open(r"..\templates\Domain\Entity.cs").unwrap();
        let mut code = vec![];
        file.read_to_end(&mut code).unwrap();
        let (code, ..) = UTF_8.decode(&code);
        let code = code.to_string();
        // println!("{}", code);
        let res = get_expressions_in_template(&code).unwrap();
        println!("{:#?}", res);
    }
}
