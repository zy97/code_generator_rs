#[cfg(test)]
mod tests {
    use code_generator::Entity;
    use code_generator::Permission;
    use code_generator::WebEntity;

    #[test]
    fn init_web_entity() {
        let dto_dir = String::from(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models");
        WebEntity::create_dto("testAppledto".to_owned(), dto_dir).unwrap();
        // let web_entity_path =
        //     r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\Test.ts".to_owned();
        // let web_entity = WebEntity::new(web_entity_path).unwrap();
        // web_entity
        //     .crteate_api("/api/app/audit-log".to_string())
        //     .unwrap();
        // web_entity.create_store().unwrap();
        // web_entity.create_page().unwrap();
        // web_entity.format_all();
        // println!("web entity:{:#?}", web_entity);
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
    fn csharp_entity_test() {
        //调试测试是运行在工作空间根目录，运行测试则是在code_generator目录下,所有运行测试会找不到模板
        let entity_dir = String::from(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Domain\Tests");
        let entity_dto_dir =
            String::from(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application.Contracts\Tests");
        let service_dir = String::from(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application\Tests");
        let mapper_file_path = String::from(
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application\BlogApplicationAutoMapperProfile.cs",
        );
        let ef_core_repository_file_path =
            String::from(r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.EntityFrameworkCore\Tests");

        let db_context_file_path = String::from(
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.EntityFrameworkCore\EntityFrameworkCore\BlogDbContext.cs",
        );

        let entity_path = Entity::create_entity(
            "Bom.Blog.Tests".to_string(),
            "Guid".to_string(),
            "Test".to_string(),
            entity_dir.clone(),
        )
        .unwrap();
        let entity = Entity::new(entity_path).unwrap();
        println!("entity:{:#?}", entity);
        entity.create_dto(entity_dto_dir.clone()).unwrap();
        entity
            .create_createorupdatedto(entity_dto_dir.clone())
            .unwrap();
        entity
            .create_pagedandsortedandfilterresultdto(entity_dto_dir.clone())
            .unwrap();
        let custom = false;
        entity
            .create_iservice(custom, entity_dto_dir.clone())
            .unwrap();
        entity.create_service(custom, service_dir).unwrap();
        entity.insert_mapper(mapper_file_path).unwrap();
        entity
            .create_repository_interface(entity_dir.clone())
            .unwrap();
        entity.create_manager(entity_dir.clone()).unwrap();
        entity
            .create_exception("Some".to_owned(), entity_dir.clone())
            .unwrap();
        entity
            .create_ef_repository(ef_core_repository_file_path)
            .unwrap();
        entity
            .insert_efcore_entity_config(db_context_file_path)
            .unwrap();
        entity.format_all();
    }
}
