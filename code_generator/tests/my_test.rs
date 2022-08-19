#[cfg(test)]
mod tests {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use std::process::Stdio;

    use code_generator::Permission;
    use code_generator::WebEntity;

    #[test]
    fn init_web_entity() {
        let web_entity_path =
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\system\User.ts"
                .to_owned();
        let web_entity = WebEntity::new(web_entity_path, "/api/app/audit-log".to_string()).unwrap();
        println!("web entity:{:#?}", web_entity);
    }
    #[test]
    fn read_permission_file() {
        let permission = Permission::new(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application.Contracts\Permissions\BlogPermissions.cs".to_owned());
        println!("{:#?}", permission);
        let permission = permission.unwrap();
        permission.add_group("Hello");
        permission.add_permission("TestGroup", "TestPermission");
    }
    #[test]
    fn cmd() {
        let output = Command::new("cmd")
            // .creation_flags(0x08000000)
            .arg("/c")
            .arg("ping baidu.com")
            .stdout(Stdio::piped())
            .output()
            .expect("cmd exec error!");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
