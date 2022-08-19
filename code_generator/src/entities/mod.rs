mod entity;
mod permision;
mod web_entity;
use std::process::{Command, Stdio};

use encoding::{all::UTF_8, DecoderTrap, Encoding};
pub use entity::Entity;
pub use permision::Permission;
use walkdir::DirEntry;
pub use web_entity::WebEntity;

fn find(src_dir: &str, contain_name: &str, is_file: bool) -> DirEntry {
    let result = walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            if is_file {
                e.file_type().is_file() && e.file_name().to_str().unwrap().contains(contain_name)
            } else {
                e.file_type().is_dir() && e.file_name().to_str().unwrap().contains(contain_name)
            }
        })
        .nth(0)
        .unwrap();
    return result;
}
fn format_csharp(project_dir: &str, relative_file_path: &str) {
    let output = Command::new("cmd")
        // .creation_flags(0x08000000)
        .arg("/c")
        .current_dir(project_dir)
        .arg(format!(r"dotnet format --include {}", relative_file_path))
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
        println!("hello");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}",&output.status);
    println!("{}", String::from_utf8_lossy(&output.stderr));
    // println!("{}", UTF_8.decode(&output.stderr, DecoderTrap::Strict).unwrap());
}
#[cfg(test)]
mod tests {
    use super::format_csharp;

    #[test]
    fn format_csharp1() {
        format_csharp(
            r"C:\repo\Abp.Bom.Blog\src\Bom.Blog.Application.Contracts",
            r"Permissions\BlogPermissions.cs",
        );
    }
}
