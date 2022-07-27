mod entities;
use entities::Entity;
use entities::WebEntity;
use std::io::stdin;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("web or service");
    let mut web_or_service = String::from("");
    stdin().read_line(&mut web_or_service).unwrap();
    if web_or_service.trim() == "web" {
        println!("web entity path:");
        let mut entity_path = String::from("");
        stdin().read_line(&mut entity_path).unwrap();
        //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
        let entity_path = String::from(
            r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\Test.ts",
        );
        let entity_path = entity_path.trim().to_string();

        println!("url prefix:");
        let mut url = String::from("");
        stdin().read_line(&mut url).unwrap();
        let url = String::from("/api/app/post-admin");
        let url = url.trim().to_string();

        let web_entity = WebEntity::new(entity_path, url);
        println!("web entity:{:#?}", web_entity);
        let custom = true;
        web_entity.create_api();
        web_entity.create_store();
    } else {
        println!("abp entity path:");
        let mut entity_path = String::from("");
        stdin().read_line(&mut entity_path).unwrap();
        //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
        let entity_path = String::from(
            r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs",
        );

        let entity_path = entity_path.trim().to_string();
        let entity = Entity::new(String::from(entity_path));
        println!("entity:{:#?}", entity);
        let custom = true;
        entity.create_dto();
        entity.create_createorupdatedto();
        entity.create_pagedandsortedandfilterresultdto();
        entity.create_iservice(custom);
        entity.create_service(custom);
        entity.insert_mapper();
    }
}
