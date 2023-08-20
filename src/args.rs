use base64::Engine;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use clap::{crate_authors, crate_description, crate_name, crate_version, value_parser, Subcommand};
use clap::{Arg, Command};
use serde::Deserialize;

use crate::CodeGeneratorError;
fn command() -> clap::Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("model")
                .help("实体类信息")
                .long("model")
                .short('m')
                .required(true),
        )
        .arg(
            Arg::new("namespace")
                .help("生成类的命名空间")
                .long("namespace")
                .short('n')
                .required(true),
        )
        .arg(
            Arg::new("output")
                .help("生成类的路径")
                .long("output")
                .short('o')
                .required(false),
        )
}
pub fn get_args() -> Result<Arguments, CodeGeneratorError> {
    let command = command();
    let matches = command.get_matches();

    let model_string = matches
        .get_one::<String>("model")
        .expect("获取model参数失败")
        .to_string();

    let bytes = general_purpose::STANDARD.decode(model_string).unwrap();
    let decoded_str = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");
    println!("{}", decoded_str);
    let class_info: ClassInfo = serde_json::from_str(&decoded_str).expect("Failed to parse JSON");

    let namespace = matches
        .get_one::<String>("namespace")
        .expect("获取命名空间失败")
        .to_string();

    let output = matches.get_one::<String>("output").map(|f| f.to_string());

    Ok(Arguments {
        class_info,
        namespace,
        output,
    })
}
pub struct Arguments {
    pub class_info: ClassInfo,
    pub namespace: String,
    pub output: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassInfo {
    pub namespace: String,
    pub class_name: String,
    pub property_infos: Vec<PropertyInfo>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyInfo {
    #[serde(rename = "Type")]
    pub property_type: String,
    pub property_name: String,
    pub comment: String,
}
