#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::process::Stdio;

    use code_generator::Entity;
    use code_generator::Permission;
    use code_generator::WebEntity;

    #[test]
    fn init_web_entity() {
        let web_entity_path =
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\Test.ts".to_owned();
        let web_entity = WebEntity::new(web_entity_path, "/api/app/audit-log".to_string()).unwrap();
        web_entity.create_api().unwrap();
        web_entity.create_store().unwrap();
        web_entity.create_page().unwrap();
        web_entity.format_all();
        println!("web entity:{:#?}", web_entity);
    }

    #[test]
    fn read_permission_file() {
        let permission = Permission::new(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application.Contracts\Permissions\BlogPermissions.cs".to_owned()).unwrap();
        println!("{:#?}", permission);
        permission.add_group("Hello").unwrap();
        permission
            .add_permission(format!("{}GroupName", "Hello").as_str(), "TestPermission")
            .unwrap();
        permission
            .add_permission_to_provider("GroupName", "Admin")
            .unwrap();
        permission.format_all();

        // permission
        //     .add_permission_to_service(
        //         r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application\Tags\TagService.cs",
        //         "group",
        //         "permission",
        //     )
        //     .unwrap();
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

    #[test]
    fn csharp_entity_test() {
        //不能运行测试，只能运行调试，不然找不到模板
        let entity_path = r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs".to_owned();
        let entity = Entity::new(entity_path).unwrap();
        println!("entity:{:#?}", entity);
        let custom = true;
        entity.create_dto().unwrap();
        // entity.create_createorupdatedto().unwrap();
        // entity.create_pagedandsortedandfilterresultdto().unwrap();
        // entity.create_iservice(custom).unwrap();
        // entity.create_service(custom).unwrap();
        // entity.insert_mapper().unwrap();
        // entity.create_repository_interface().unwrap();
        // entity.create_manager().unwrap();
        // entity.create_exception(Some("AlreadyExist".to_owned()),Some("xxxxxx".to_owned()),Some("tttttttttt".to_owned())).unwrap();
        // entity.create_ef_repository().unwrap();
        // entity.insert_efcore_entity_config().unwrap();
        // entity.format_all();
    }
}
