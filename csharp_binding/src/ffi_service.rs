use code_generator::{CodeGeneratorError, Entity};
use interoptopus::{
    ffi_service, ffi_service_ctor, ffi_type,
    patterns::{result::FFIError, string::AsciiPointer}, ffi_service_method,
};
#[ffi_type(opaque)]
pub struct EntityGenerator {
    generator: Entity,
}

#[ffi_service(error = "AppFFIError", prefix = "entity_service_")]
impl EntityGenerator {
    #[ffi_service_ctor]
    pub fn new(path: AsciiPointer) -> Result<Self, CodeGeneratorError> {
        Ok(Self {
            generator: Entity::new(path.as_str().unwrap().to_owned())?,
        })
    }
    pub fn create_dto(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_dto()
    }
    pub fn create_createorupdatedto(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_createorupdatedto()
    }
    pub fn create_ef_repository(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_ef_repository()
    }
    pub fn create_exception(
        &self,
        exception_name: AsciiPointer,
        exception_code: AsciiPointer,
        exception_displayname: AsciiPointer,
    ) -> Result<(), CodeGeneratorError> {
        self.generator.create_exception(
            exception_name.as_str().ok().map(|s| s.to_owned()),
            exception_code.as_str().ok().map(|s| s.to_owned()),
            exception_displayname.as_str().ok().map(|s| s.to_owned()),
        )
    }
    pub fn create_iservice(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        self.generator.create_iservice(custom)
    }
    pub fn create_manager(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_manager()
    }
    pub fn create_pagedandsortedandfilterresultdto(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_pagedandsortedandfilterresultdto()
    }
    pub fn create_repository_interface(&self) -> Result<(), CodeGeneratorError> {
        self.generator.create_repository_interface()
    }
    pub fn create_service(&self, custom: bool) -> Result<(), CodeGeneratorError> {
        self.generator.create_service(custom)
    }
    pub fn insert_mapper(&self) -> Result<(), CodeGeneratorError> {
        self.generator.insert_mapper()
    }
    pub fn insert_efcore_entity_config(&self) -> Result<(), CodeGeneratorError> {
        self.generator.insert_efcore_entity_config()
    }
    #[ffi_service_method(on_panic="return_default")]
    pub fn format_all(&self) {
        self.generator.format_all()
    }
}
#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum AppFFIError {
    Ok = 0,
    NullPassed = 1,
    Panic = 2,
    OtherError = 3,
   
}
// Gives special meaning to some of your error variants.
impl FFIError for AppFFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::NullPassed;
    const PANIC: Self = Self::Panic;
}

// How to map an `Error` to an `MyFFIError`.
impl From<CodeGeneratorError> for AppFFIError {
    fn from(x: CodeGeneratorError) -> Self {
        match x {
            CodeGeneratorError::FileError(_) => Self::OtherError,
            CodeGeneratorError::RegexError(_) => Self::OtherError,
            CodeGeneratorError::RegexNoMatchError(_) => Self::OtherError,
            CodeGeneratorError::TeraError(_) => Self::OtherError,
        }
    }
}
