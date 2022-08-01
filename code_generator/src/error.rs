use std::borrow::Cow;

use serde::de::value::CowStrDeserializer;

#[derive(thiserror::Error, Debug)]
pub enum CodeGeneratorError {
    #[error("file operate failure")]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
}
