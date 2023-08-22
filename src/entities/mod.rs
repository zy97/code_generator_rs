mod wes;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use crate::{args::Arguments, CodeGeneratorError};

use self::wes::*;

fn open_file(file: &str) -> Result<File, CodeGeneratorError> {
    let mut options = OpenOptions::new();
    let file = options
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&file)?;
    Ok(file)
}
// 创建文件夹
pub fn create_dir(dir: &str) -> Result<(), CodeGeneratorError> {
    if !Path::new(dir).exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

// fn format_code(work_dir: String, files: Vec<String>) -> Result<(), CodeGeneratorError> {
//     if files.len() == 0 {
//         return Ok(());
//     }
//     let output = Command::new("cmd")
//         .arg("/c")
//         .current_dir(work_dir)
//         .arg(format!(r"dprint fmt {}", files.join(" ")))
//         .stdout(Stdio::piped())
//         .output()
//         .expect("cmd exec error!");
//     if &output.status == &ExitStatus::from_raw(1) {
//         let (output, ..) = UTF_8.decode(&output.stderr);
//         return Err(CodeGeneratorError::DprintError(output.to_string()));
//     }
//     println!("{:#?} format successful!", files);
//     Ok(())
// }
// fn format_single_file(file: String) -> Result<(), CodeGeneratorError> {
//     let work_dir = Path::new(&file).parent().unwrap().display().to_string();
//     let output = Command::new("cmd")
//         .arg("/c")
//         .current_dir(work_dir)
//         .arg(format!(r"dprint fmt {}", file))
//         .stdout(Stdio::piped())
//         .output()
//         .expect("cmd exec error!");
//     if &output.status == &ExitStatus::from_raw(1) {
//         let (output, ..) = UTF_8.decode(&output.stderr);
//         return Err(CodeGeneratorError::DprintError(output.to_string()));
//     }
//     eprintln!("{} format successful!", file);
//     Ok(())
// }

pub fn render_wes_template(args: Arguments) {
    let res = match args.mode {
        crate::args::Mode::Dto => create_dto(args.class_info, args.namespace, args.output),
        crate::args::Mode::CreateDto => {
            create_create_dto(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::UpdateDto => {
            create_update_dto(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::QueryDto => {
            create_query_dto(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::IService => {
            create_iservice(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::Service => create_service(args.class_info, args.namespace, args.output),
        crate::args::Mode::IRepository => {
            create_irepository(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::Repository => {
            create_repository(args.class_info, args.namespace, args.output)
        }
        crate::args::Mode::Controller => {
            create_controller(args.class_info, args.namespace, args.output)
        }
    };
}
