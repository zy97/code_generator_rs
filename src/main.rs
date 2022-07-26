mod entities;
use entities::Entity;
use std::io::stdin;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("abp entity path:");
    let mut entity_path = String::from("");
    stdin().read_line(&mut entity_path).unwrap();
    //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
    let entity_path =
        String::from(r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs");

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
