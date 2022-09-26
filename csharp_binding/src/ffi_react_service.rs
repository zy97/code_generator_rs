use crate::AppFFIError;
use code_generator::{CodeGeneratorError, WebEntity};
use interoptopus::{
    ffi_service, ffi_service_ctor, ffi_service_method, ffi_type, patterns::string::AsciiPointer,
};
#[ffi_type(opaque)]
pub struct ReactGenerator {
    generator: WebEntity,
}
#[ffi_service(error = "AppFFIError", prefix = "react_service_")]
impl ReactGenerator {
    #[ffi_service_ctor]
    pub fn new(path: AsciiPointer) -> Result<Self, CodeGeneratorError> {
        Ok(Self {
            generator: WebEntity::new(path.as_str().unwrap().to_owned())?,
        })
    }
    pub fn create_api(&self, url_prefix: AsciiPointer) -> Result<(), CodeGeneratorError> {
        self.generator
            .create_api(url_prefix.as_str().unwrap().to_owned())
    }
    pub fn create_store(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_store()
    }
    pub fn create_page(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_page()
    }
    #[ffi_service_method(on_panic = "return_default")]
    pub fn format_all(&self) {
        self.generator.format_all()
    }
}
