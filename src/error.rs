use std::{
    fmt,
    io::{self},
};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum CodeGeneratorError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error("regex no match")]
    RegexNoMatchError(#[from] RegexNoMatchError),
    #[error("{0}")]
    DprintError(String),
    #[error("{0}")]
    RenderError(#[from] askama::Error),
}
#[derive(Debug)]
pub struct RegexNoMatchError;
impl fmt::Display for RegexNoMatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}
impl From<io::Error> for RegexNoMatchError {
    fn from(error: io::Error) -> Self {
        println!("{}", error);
        RegexNoMatchError {}
    }
}

impl std::error::Error for RegexNoMatchError {}
