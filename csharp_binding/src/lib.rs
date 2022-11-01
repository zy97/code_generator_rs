mod ffi;
mod ffi_entity_service;
mod ffi_react_service;

use code_generator::CodeGeneratorError;
use ffi_entity_service::EntityGenerator;
use ffi_react_service::ReactGenerator;
use interoptopus::{ffi_type, pattern, patterns::result::FFIError, Inventory, InventoryBuilder};

pub fn entity_inventory() -> Inventory {
    InventoryBuilder::new()
        // .register(function!(create))
        // .register(function!(create_dto))
        // .register(function!(format_all))
        // .register(function!(dispose))
        .register(pattern!(EntityGenerator))
        .inventory()
}
pub fn react_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(pattern!(ReactGenerator))
        .inventory()
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
            CodeGeneratorError::DprintError(_) => Self::OtherError,
        }
    }
}
#[cfg(test)]
mod tests {
    use interoptopus::{util::NamespaceMappings, Error, Interop};

    use super::*;

    #[test]
    fn bindings_csharp_for_entity() -> Result<(), Error> {
        use interoptopus_backend_csharp::overloads::DotNet;
        use interoptopus_backend_csharp::{Config, Generator};

        let config = Config {
            dll_name: "example_library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        };

        Generator::new(config, entity_inventory())
            .add_overload_writer(DotNet::new())
            //.add_overload_writer(Unity::new())
            .write_file(
                r"../c#/CodeGeneratorApp/CodeGeneratorApp/CodeGeneratorApp.Core/Interop.cs",
            )?;
        // .write_file("../c#/Interop.cs")?;

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
