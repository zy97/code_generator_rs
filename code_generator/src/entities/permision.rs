use super::{format_code, get_class_name, read_file};
use crate::{
    entities::open_file,
    error::{CodeGeneratorError, RegexNoMatchError},
};
use inflector::Inflector;
use regex::Regex;
use std::{cell::RefCell, io::Read, os::windows::prelude::FileExt, vec};

#[derive(Debug)]
pub struct Permission {
    solution_dir: String,
    groups: Vec<PermissionGroup>,
    permissions_class_name: String,
    changed_files: RefCell<Vec<String>>,
}

impl Permission {
    pub fn new(path: String) -> Result<Self, CodeGeneratorError> {
        let src_dir = path.split('\\').collect::<Vec<&str>>();
        let src_index = src_dir.iter().rposition(|&i| i.contains("src")).unwrap();
        let solution_dir = src_dir[..(src_index)].join("\\");

        let code = read_file(&path)?;

        let class_name = get_class_name(&code)?;
        let permissions_class_name = class_name;

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

        Ok(Permission {
            changed_files: RefCell::new(vec![]),
            groups,
            solution_dir,
            permissions_class_name,
        })
    }

    pub fn add_group(
        &self,
        group_name: String,
        file_path: String,
    ) -> Result<(), CodeGeneratorError> {
        let mut file = open_file(&file_path)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let re = Regex::new(r#"public const string ([A-Za-z=\\ "]+);"#).unwrap();
        let range = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .range();
        let insert_code = format!(r#"public const string {0}GroupName = "{0}";"#, group_name);
        if code.contains(&insert_code) {
            return Ok(());
        }
        code.insert_str(range.end + 2, &insert_code);
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(file_path);
        Ok(())
    }

    pub fn add_permission(
        &self,
        file_path: String,
        group: String,
        permission: String,
    ) -> Result<(), CodeGeneratorError> {
        let mut file = open_file(&file_path)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let re = Regex::new(r#"public const string ([A-Za-z=\\ "]+);"#).unwrap();
        let range = re
            .captures_iter(&code)
            .last()
            .unwrap()
            .get(0)
            .unwrap()
            .range();
        let insert_code = format!(
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
        let mut lines = insert_code.lines();
        lines.next();

        if code.contains(lines.next().unwrap().trim()) {
            return Ok(());
        }
        code.insert_str(range.end + 2, &insert_code);
        file.seek_write(code.as_bytes(), 0)?;
        self.add_file_change_log(file_path);
        Ok(())
    }

    pub fn add_permission_to_provider(
        &self,
        file_path: String,
        group: String,
        permission: String,
    ) -> Result<(), CodeGeneratorError> {
        let mut file = open_file(&file_path)?;
        let mut code = String::new();

        file.read_to_string(&mut code)?;
        let re = Regex::new(
            r#"public override void Define([\s\S]+?)}([\s]+)private static LocalizableString L"#,
        )
        .unwrap();
        let insert_range = re.captures(&code).unwrap().get(1).unwrap().range();

        let insert_index = insert_range.end;
        let group_code = format!(
            "context.GetGroupOrNull({0}.{1});",
            self.permissions_class_name, group
        );
        dbg!(group_code);
        let group_var_name = self
            .groups
            .iter()
            .find(|e| e.group_property_name == group)
            .unwrap()
            .group_property_value
            .clone()
            .to_camel_case()
            + "Group";
        let group = format!(
            "var {2} = context.GetGroupOrNull({0}.{1});",
            self.permissions_class_name, group, group_var_name
        );
        match code.find(&group) {
            Some(group_index) => {
                let default_permission = format!(
                    "var {3} = {1}.GetPermissionOrNull({0}.{2}.Default);",
                    self.permissions_class_name,
                    group_var_name,
                    permission,
                    permission.to_camel_case() + "DefaultPermission"
                );
                println!("{},{}", default_permission, group_index);
            }
            None => {
                code.insert_str(insert_index, &group);
            }
        };

        let insert_code = format!(
            r###"
        var {2} = context.GetGroupOrNull({0}.{1});
        if ({2} == null)
        {{
            {2} = context.AddGroup({0}.{1});
        }}
        var {4} = {2}.GetPermissionOrNull({0}.{3}.Default);
        if ({4} == null)
        {{
            {4} = {2}.AddPermission({0}.{3}.Default);
            {4}.AddChild({0}.{3}.Create);
            {4}.AddChild({0}.{3}.Delete);
            {4}.AddChild({0}.{3}.Update);
        }}"###,
            self.permissions_class_name,
            group,
            group_var_name,
            permission,
            permission.to_camel_case() + "DefaultPermission"
        );
        code.insert_str(insert_index, &insert_code);
        file.seek_write(code.as_bytes(), 0)?;
        Ok(())
    }

    fn add_file_change_log(&self, path: String) {
        let mut changs = self.changed_files.borrow_mut();
        changs.push(path);
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
    #[allow(dead_code)]
    class_name: String,
    #[allow(dead_code)]
    permission_name: Vec<String>,
}
impl Permission {
    pub fn format_all(&self) -> Result<(), CodeGeneratorError> {
        let files = self.changed_files.borrow().to_vec();
        format_code(self.solution_dir.clone(), files)
    }
}
