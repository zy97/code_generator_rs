use crate::{AppFFIError, FFIPatternsError};
use code_generator::WebEntity;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type, patterns::string::AsciiPointer};
#[ffi_type(opaque)]
pub struct ReactGenerator {
    generator: WebEntity,
}
#[ffi_service(error = "AppFFIError", prefix = "react_service_")]
impl ReactGenerator {
    #[ffi_service_ctor]
    pub fn new(path: AsciiPointer) -> Result<Self, FFIPatternsError> {
        Ok(Self {
            generator: WebEntity::new(path.as_str()?.to_owned())?,
        })
    }
    pub fn create_api(
        &self,
        url_prefix: AsciiPointer,
        dir: AsciiPointer,
    ) -> Result<(), FFIPatternsError> {
        self.generator
            .create_api(url_prefix.as_str()?.to_owned(), dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_store(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator.create_store(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_page(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator.create_page(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn format_all(&self) -> Result<(), FFIPatternsError> {
        self.generator.format_all()?;
        Ok(())
    }
}
