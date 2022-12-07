mod ffi_entity_service;
mod ffi_react_service;
use code_generator::CodeGeneratorError;
use ffi_entity_service::EntityGenerator;
use ffi_react_service::ReactGenerator;
use interoptopus::{ffi_type, pattern, patterns::result::FFIError, Inventory, InventoryBuilder};
use thiserror::Error;
pub fn entity_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(pattern!(EntityGenerator))
        .inventory()
}
pub fn react_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(pattern!(ReactGenerator))
        .inventory()
}
#[derive(Debug, Error)]
pub enum FFIPatternsError {
    #[error(transparent)]
    CodeGeneratorError(#[from] CodeGeneratorError),
    #[error(transparent)]
    InteropError(#[from] interoptopus::Error),
}
#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum AppFFIError {
    Ok = 0,
    NullPassed = 1,
    Panic = 2,
    CodeGeneratorError = 3,
    FFIPaternsError = 4,
}
// Gives special meaning to some of your error variants.
impl FFIError for AppFFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::NullPassed;
    const PANIC: Self = Self::Panic;
}
// How to map an `Error` to an `MyFFIError`.
impl From<FFIPatternsError> for AppFFIError {
    fn from(x: FFIPatternsError) -> Self {
        match x {
            FFIPatternsError::CodeGeneratorError(_) => Self::CodeGeneratorError,
            FFIPatternsError::InteropError(_) => Self::FFIPaternsError,
        }
    }
}
#[cfg(test)]
mod tests {
    use interoptopus::{util::NamespaceMappings, Error, Interop};
    use interoptopus_backend_csharp::WriteTypes;

    use super::*;

    #[test]
    fn bindings_csharp_for_entity() -> Result<(), Error> {
        use interoptopus_backend_csharp::overloads::DotNet;
        use interoptopus_backend_csharp::{Config, Generator};

        let config = Config {
            dll_name: "C:\\Users\\Administrator\\Desktop\\code_generator_rs\\target\\debug\\csharp_binding.dll".to_string(),
            namespace_mappings: NamespaceMappings::new("CodeGeneratorApp.Services"),
            class: "RawEntityGenerator".to_string(),
            rename_symbols:true,
            write_types:WriteTypes::NamespaceAndInteroptopusGlobal,
            ..Config::default()
        };

        Generator::new(config, entity_inventory())
            .add_overload_writer(DotNet::new())
            //.add_overload_writer(Unity::new())
            .write_file(
                r"../c#/CodeGeneratorApp/CodeGeneratorApp/Services/CodeGeneratorApp.Services/EntityGenerator.cs",
            )?;
        Ok(())
    }
    #[test]
    fn bindings_csharp_for_react() -> Result<(), Error> {
        use interoptopus_backend_csharp::overloads::DotNet;
        use interoptopus_backend_csharp::{Config, Generator};

        let config = Config {
            dll_name: "example_library".to_string(),
            namespace_mappings: NamespaceMappings::new("CodeGeneratorApp.Services"),
            ..Config::default()
        };

        Generator::new(config, react_inventory())
            .add_overload_writer(DotNet::new())
            .write_file(
                r"../c#/CodeGeneratorApp/CodeGeneratorApp/Services/CodeGeneratorApp.Services/ReactGenerator.cs",
            )?;
        Ok(())
    }
}
