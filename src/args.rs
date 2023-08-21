use base64::Engine;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use clap::builder::PossibleValue;
use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, value_parser, ArgAction,
    Subcommand, ValueEnum,
};
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
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(value_parser!(Mode)),
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
    let class_info: ClassInfo = serde_json::from_str(&decoded_str).expect("Failed to parse JSON");

    let namespace = matches
        .get_one::<String>("namespace")
        .expect("获取命名空间失败")
        .to_string();

    let output = matches.get_one::<String>("output").map(|f| f.to_string());

    let mode = matches
        .get_one::<Mode>("MODE")
        .copied()
        .expect("'MODE' is required and parsing will fail if its missing");

    Ok(Arguments {
        class_info,
        namespace,
        output,
        mode,
    })
}
pub struct Arguments {
    pub class_info: ClassInfo,
    pub namespace: String,
    pub output: Option<String>,
    pub mode: Mode,
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Dto,
    CreateDto,
    UpdateDto,
    QueryDto,
    IService,
    Service,
    IRepository,
    Repository,
    Controller,
}
impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Dto,
            Self::CreateDto,
            Self::UpdateDto,
            Self::QueryDto,
            Self::IService,
            Self::Service,
            Self::IRepository,
            Self::Repository,
            Self::Controller,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::Dto => PossibleValue::new("dto"),
            Mode::CreateDto => PossibleValue::new("create_dto"),
            Mode::UpdateDto => PossibleValue::new("update_dto"),
            Mode::QueryDto => PossibleValue::new("query_dto"),
            Mode::IService => PossibleValue::new("iservice"),
            Mode::Service => PossibleValue::new("service"),
            Mode::IRepository => PossibleValue::new("irepository"),
            Mode::Repository => PossibleValue::new("repository"),
            Mode::Controller => PossibleValue::new("controller"),
        })
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
