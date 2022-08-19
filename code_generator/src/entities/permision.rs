use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Read,
    os::windows::{prelude::FileExt, process::CommandExt},
    vec, process::{Command, Stdio},
};

use encoding::{all::UTF_8, DecoderTrap, Encoding};
use inflector::Inflector;
use log::{info, trace};
use regex::Regex;

use crate::error::{CodeGeneratorError, RegexNoMatchError};

use super::find;

#[derive(Debug)]
pub struct Permission {
    // namespace: String,
    // id_type: String,
    // name: String,
    src_dir: String,
    // //复数名字
    // plural_name: String,
    // properties: String,
    groups: Vec<PermissionGroup>,
}

impl Permission {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let src_dir = src_dir[..(src_index + 1)].join("\\");

        let mut file = File::open(&path)?;
        let mut code = vec![];
        file.read_to_end(&mut code)?;
        let code = UTF_8.decode(&code, DecoderTrap::Strict).unwrap();
        eprintln!("{}", code);
        let re = Regex::new(r"public static class ([a-zA-Z]+)")?;

        let class_name = re
            .captures(&code)
            .ok_or(RegexNoMatchError)?
            .get(1)
            .ok_or(RegexNoMatchError)?
            .as_str()
            .to_string();

        let re = Regex::new(r"\{([\s\S]+)}([\s]*)}").unwrap();
        let class_properties = re
            .captures(&code)
            .ok_or(RegexNoMatchError)?
            .get(0)
            .ok_or(RegexNoMatchError)?
            .as_str()
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .map(|f| f.to_string() + "\r\n")
            .collect::<String>();

        let re = Regex::new(r#"public const string ([A-Za-z=\\ "]+);"#).unwrap();
        let mut groups = vec![];
        for caps in re.captures_iter(&class_properties) {
            let mut property = caps.get(1).unwrap().as_str().split('=');
            groups.push(PermissionGroup {
                group_property_name: property.next().unwrap().trim().to_owned(),
                group_property_value: property.next().unwrap().trim().trim_matches('"').to_owned(),
                permissions: vec![],
            });
        }

        let re = Regex::new(r#"public static class([\s\S]+?)}"#).unwrap();
        for caps in re.captures_iter(&class_properties) {
            let permisson_code = caps.get(0).unwrap().as_str();
            eprintln!("------{}", permisson_code);
            let re = Regex::new(r"public static class ([a-zA-Z]+)")?;

            let class_name = re
                .captures(permisson_code)
                .ok_or(RegexNoMatchError)?
                .get(1)
                .ok_or(RegexNoMatchError)?
                .as_str()
                .to_string();
            for group in &mut groups {
                let mut group_name = group.group_property_name.clone();
                group_name.insert(0, ' ');
                if permisson_code.contains(&group_name) {
                    let re = Regex::new(r"public const string ([a-zA-Z]+) ")?;
                    let mut property = vec![];
                    for p in re.captures_iter(permisson_code) {
                        property.push(p.get(1).unwrap().as_str().to_owned())
                    }

                    group.permissions.push(PermissionDetail {
                        class_name: class_name.clone(),
                        permission_name: property,
                    });
                }
            }
        }

        Ok(Permission { groups, src_dir })
    }
    pub fn add_group(&self, group: &str) -> Result<(), CodeGeneratorError> {
        let permission_file_path = find(&self.src_dir, "Permissions.cs", true);

        let permission_file_path = permission_file_path.path().to_str().unwrap();
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(&permission_file_path)
            .expect("create failed");
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let re = Regex::new(r#"public const string ([A-Za-z=\\ "]+);"#).unwrap();
        let sdf = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        eprintln!("#################{}", sdf);
        let range = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .range();
        eprintln!("#################{:?}", range);
        let mut insert_code = format!(r#"public const string {0}GroupName = "{0}";"#, group);
        insert_code.insert(0, '\t');
        insert_code.push('\r');
        insert_code.push('\n');
        code.insert_str(range.end + 2, &insert_code);
        file.seek_write(code.as_bytes(), 0)?;
        Ok(())
    }
    pub fn add_permission(&self, group: &str, permission: &str) -> Result<(), CodeGeneratorError> {
        let permission_file_path = find(&self.src_dir, "Permissions.cs", true);

        let permission_file_path = permission_file_path.path().to_str().unwrap();
        let mut options = OpenOptions::new();
        let mut file = options
            .write(true)
            .read(true)
            .open(&permission_file_path)
            .expect("create failed");
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let re = Regex::new(r#"public const string ([A-Za-z=\\ "]+);"#).unwrap();
        let sdf = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let range = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .range();
        let mut insert_code = format!(
            r#"
        public static class {0}
        {{
            public const string Default = {1} + ".{0}";
            public const string Create = Default + ".Create";
            public const string Delete = Default + ".Delete";
            public const string Update = Default + ".Update";
        }}
        "#,
            permission, group
        );
        // insert_code.insert(0, '\t');
        // insert_code.push('\r');
        // insert_code.push('\n');
        code.insert_str(range.end + 2, &insert_code);
        file.seek_write(code.as_bytes(), 0)?;
        Ok(())
    }
}
#[derive(Debug)]
struct PermissionGroup {
    group_property_name: String,
    group_property_value: String,
    permissions: Vec<PermissionDetail>,
}
#[derive(Debug)]
struct PermissionDetail {
    class_name: String,
    permission_name: Vec<String>,
}
