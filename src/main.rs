use crate::args::{get_args, Arguments};
use args::ClassInfo;
use code_generator::CodeGeneratorError;
mod entities;
use entities::render_wes_template;
mod args;

fn main() {
    match main_result() {
        Ok(args) => {
            render_wes_template(args);
        }
        Err(e) => println!("Error: {}", e),
    }
}
fn main_result() -> Result<Arguments, CodeGeneratorError> {
    get_args()
}
