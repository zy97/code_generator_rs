mod entity;
mod permision;
mod web_entity;
use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

pub use entity::Entity;
pub use permision::Permission;
use regex::Regex;

use walkdir::DirEntry;
pub use web_entity::WebEntity;

use crate::error::{CodeGeneratorError, RegexNoMatchError};

fn find(src_dir: &str, contain_name: &str, is_file: bool) -> DirEntry {
    let result = walkdir::WalkDir::new(src_dir)
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
        .unwrap();
    return result;
}
fn get_class_name(content: &str) -> Result<String, CodeGeneratorError> {
    let re = Regex::new(r"class ([a-zA-Z]+) :")?;
    let entity_name = re
        .captures(content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(entity_name)
}
fn get_generic_type(content: &str) -> Result<String, CodeGeneratorError> {
    let re = Regex::new(r"<([a-zA-Z]+)>")?;
    let t = re
        .captures(&content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(t)
}
fn get_namespace(content: &str) -> Result<String, CodeGeneratorError> {
    let re = Regex::new(r"namespace ([a-zA-Z.]+)")?;
    let namespace = re
        .captures(&content)
        .ok_or(RegexNoMatchError)?
        .get(1)
        .ok_or(RegexNoMatchError)?
        .as_str()
        .to_string();
    Ok(namespace)
}
fn get_properties(content: &str) -> Result<HashMap<String, String>, CodeGeneratorError> {
    let re = Regex::new(r"public ([a-zA-Z\\ ]+) \{")?;
    let mut kv = HashMap::new();
    for property in re.captures_iter(&content) {
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
fn format_csharp_code(work_dir: String, relative_file_or_dir: Vec<String>) {
    let output = Command::new("cmd")
        // .creation_flags(0x08000000)
        .arg("/c")
        .current_dir(work_dir)
        .arg(format!(
            r"dotnet format --include {}",
            relative_file_or_dir.join(" ")
        ))
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    println!("{}", &output.status);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    #[test]
    fn format_csharp1() {
        {
            let relative_file_path = r"Permissions\BlogPermissionDefinitionProvider.cs";
            let output = Command::new("cmd")
                // .creation_flags(0x08000000)
                .arg("/c")
                .current_dir(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application.Contracts")
                .arg(format!(r"dotnet format --include {}", relative_file_path))
                .stdout(Stdio::piped())
                .output()
                .expect("cmd exec error!");
            println!("{}", &output.status);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        };
    }
}
