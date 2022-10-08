mod entity;
mod permision;
mod web_entity;
use encoding::{all::UTF_8, DecoderTrap, Encoding};
pub use entity::Entity;
use lazy_static::lazy_static;
pub use permision::Permission;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Read,
    process::{Command, Stdio},
};

use walkdir::DirEntry;
pub use web_entity::WebEntity;

use crate::error::{CodeGeneratorError, RegexNoMatchError};

fn read_file(file: &str) -> Result<String, CodeGeneratorError> {
    let mut file = File::open(file)?;
    let mut code = vec![];
    file.read_to_end(&mut code)?;
    let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();
    Ok(code)
}
fn open_file(file: &str) -> Result<File, CodeGeneratorError> {
    let mut options = OpenOptions::new();
    let file = options.write(true).read(true).open(&file)?;
    Ok(file)
}
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
fn format_code(work_dir: String, files: Vec<String>) {
    if files.len() == 0 {
        return;
    }
    let output = Command::new("cmd")
        .arg("/c")
        .current_dir(work_dir)
        .arg(format!(r"dprint fmt {}", files.join(" ")))
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    println!("{}", &output.status);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
