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
#[cfg(test)]
mod test {
    use crate::args::get_args_from;

    use super::*;
    #[test]
    fn test_get_args() {
        let arg_vec = vec!["code_generator.exe", 
        "-m", "eyJOYW1lc3BhY2UiOiJXRVMuRW50aXR5LkVudGl0eSIsIkNsYXNzTmFtZSI6IkN1c3RvbWVyIiwiUHJvcGVydHlJbmZvcyI6W3siVHlwZSI6ImludCIsIlByb3BlcnR5TmFtZSI6IklkIiwiQ29tbWVudCI6IiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyBJRFxyXG4gICAgICAgIC8vLyA8L3N1bW1hcnk+XHJcbiAgICAgICAgIn0seyJUeXBlIjoic3RyaW5nIiwiUHJvcGVydHlOYW1lIjoiQ3VzdG9tZXJOYW1lIiwiQ29tbWVudCI6IlxyXG4gICAgICAgIC8vLyA8c3VtbWFyeT5cclxuICAgICAgICAvLy8g5a6i5oi35ZCN56ewXHJcbiAgICAgICAgLy8vIDwvc3VtbWFyeT5cclxuICAgICAgICAifSx7IlR5cGUiOiJzdHJpbmciLCJQcm9wZXJ0eU5hbWUiOiJSZW1hcmsiLCJDb21tZW50IjoiXHJcbiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyDlpIfms6hcclxuICAgICAgICAvLy8gPC9zdW1tYXJ5PlxyXG4gICAgICAgICJ9LHsiVHlwZSI6InN0cmluZyIsIlByb3BlcnR5TmFtZSI6IkNyZWF0ZVVzZXIiLCJDb21tZW50IjoiXHJcbiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyDliJvlu7rkurpcclxuICAgICAgICAvLy8gPC9zdW1tYXJ5PlxyXG4gICAgICAgICJ9LHsiVHlwZSI6IkRhdGVUaW1lPyIsIlByb3BlcnR5TmFtZSI6IkNyZWF0ZVRpbWUiLCJDb21tZW50IjoiXHJcbiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyDliJvlu7rml7bpl7RcclxuICAgICAgICAvLy8gPC9zdW1tYXJ5PlxyXG4gICAgICAgICJ9LHsiVHlwZSI6InN0cmluZyIsIlByb3BlcnR5TmFtZSI6IlVwZGF0ZVVzZXIiLCJDb21tZW50IjoiXHJcbiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyDmm7TmlrDkurpcclxuICAgICAgICAvLy8gPC9zdW1tYXJ5PlxyXG4gICAgICAgICJ9LHsiVHlwZSI6IkRhdGVUaW1lPyIsIlByb3BlcnR5TmFtZSI6IlVwZGF0ZVRpbWUiLCJDb21tZW50IjoiXHJcbiAgICAgICAgLy8vIDxzdW1tYXJ5PlxyXG4gICAgICAgIC8vLyDmm7TmlrDml7bpl7RcclxuICAgICAgICAvLy8gPC9zdW1tYXJ5PlxyXG4gICAgICAgICJ9XX0=", 
        "-n","WES.Entity.Model.Customers", 
        "dto"];
        match get_args_from(arg_vec) {
            Ok(args) => {
                render_wes_template(args);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
