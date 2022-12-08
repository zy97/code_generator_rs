use crate::{AppFFIError, FFIPatternsError};
use code_generator::Entity;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type, patterns::string::AsciiPointer};
#[ffi_type(opaque)]
pub struct EntityGenerator {
    generator: Entity,
}

#[ffi_service(error = "AppFFIError", prefix = "entity_service_")]
impl EntityGenerator {
    #[ffi_service_ctor]
    pub fn new(path: AsciiPointer) -> Result<Self, FFIPatternsError> {
        Ok(Self {
            generator: Entity::new(path.as_str()?.to_owned())?,
        })
    }
    pub fn create_entity(
        namespace: AsciiPointer,
        id_type: AsciiPointer,
        name: AsciiPointer,
        dir: AsciiPointer,
    ) -> Result<(), FFIPatternsError> {
        println!("2");
        eprintln!("1");
        Entity::create_entity(
            namespace.as_str()?.to_owned(),
            id_type.as_str()?.to_owned(),
            name.as_str()?.to_owned(),
            dir.as_str()?.to_owned(),
        )?;
        Ok(())
    }
    pub fn create_dto(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator.create_dto(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_add_and_modify_dto(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .create_create_and_update_dto(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_ef_repository(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .create_ef_repository(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_exception(
        &self,
        exception_name: AsciiPointer,
        dir: AsciiPointer,
    ) -> Result<(), FFIPatternsError> {
        self.generator.create_exception(
            exception_name.as_str()?.to_owned(),
            dir.as_str()?.to_owned(),
        )?;
        Ok(())
    }
    pub fn create_iservice(&self, custom: bool, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .create_iservice(custom, dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_manager(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator.create_manager(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_pagedandsortedandfilterresultdto(
        &self,
        dir: AsciiPointer,
    ) -> Result<(), FFIPatternsError> {
        self.generator
            .create_pagedandsortedandfilterresultdto(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_repository_interface(&self, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .create_repository_interface(dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn create_service(&self, custom: bool, dir: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .create_service(custom, dir.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn insert_mapper(&self, file_path: AsciiPointer) -> Result<(), FFIPatternsError> {
        self.generator
            .insert_mapper(file_path.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn insert_efcore_entity_config(
        &self,
        file_path: AsciiPointer,
    ) -> Result<(), FFIPatternsError> {
        self.generator
            .insert_efcore_entity_config(file_path.as_str()?.to_owned())?;
        Ok(())
    }
    pub fn format_all(&self) -> Result<(), FFIPatternsError> {
        self.generator.format_all()?;
        Ok(())
    }
}
