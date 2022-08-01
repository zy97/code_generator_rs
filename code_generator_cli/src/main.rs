use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use code_generator::{Entity, WebEntity};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Abp实体文件的路径
    #[clap(short, long, value_parser)]
    entity_path: Option<String>,

    /// react中实体的路径
    #[clap(short, long, value_parser)]
    web_entity_path: Option<String>,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,

    #[clap(subcommand)]
    command: Option<CreateCode>,
}
#[derive(Subcommand, Debug)]
enum CreateCode {
    /// 创建Abp Web Service
    Service {
        /// Abp实体文件的路径
        #[clap(short, long)]
        entity_path: String,
        /// true:生成IApplicationService，false:生成ICrudAppService
        #[clap(short, long, value_parser, default_value_t = false)]
        cumstom_service: bool,
    },
    /// 创建Abp Web frontend
    Web {
        /// react中实体的路径
        #[clap(short, long)]
        entity_path: String,
        ///请求api的路径：如/api/app/post-admin
        #[clap(short, long)]
        prefix_api_url: String,
    },
}
fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(CreateCode::Service {
            entity_path,
            cumstom_service,
        }) => {
            //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
            // let entity_path = String::from(
            //     r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Domain\Tests\Test.cs",
            // );

            let entity_path = entity_path.trim().to_string();
            let entity = Entity::new(String::from(entity_path))?;
            println!("entity:{:#?}", entity);
            let custom = cumstom_service;
            entity.create_dto()?;
            entity.create_createorupdatedto()?;
            entity.create_pagedandsortedandfilterresultdto()?;
            entity.create_iservice(custom)?;
            entity.create_service(custom)?;
            entity.insert_mapper()?;
        }
        Some(CreateCode::Web {
            entity_path,
            prefix_api_url,
        }) => {
            //如果从控制台接受输入，在windows下会有\r\n的结束符，在Unix下游\n的结束符
            // let entity_path = String::from(
            //     r"C:\Users\Administrator\Desktop\Bom.Blog\src\Bom.Blog.Web.Admin\src\data\models\Test.ts",
            // );

            // let url = String::from("/api/app/post-admin");

            let web_entity = WebEntity::new(entity_path, prefix_api_url);
            println!("web entity:{:#?}", web_entity);
            web_entity.create_api();
            web_entity.create_store();
            web_entity.create_page();
        }
        None => {
            println!("{:?}", args);
        }
    }
    Ok(())
}
