#[cfg(test)]
mod tests {
    use code_generator::WebEntity;

    #[test]
    fn init_web_entity() {
        let web_entity_path =
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\AuditLog.ts".to_owned();
        let web_entity = WebEntity::new(web_entity_path, "/api/app/audit-log".to_string()).unwrap();
        println!("web entity:{:#?}", web_entity);
    }
}
