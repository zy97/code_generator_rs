mod ffi;

use interoptopus::{Inventory, InventoryBuilder, function};

use crate::ffi::{*};

pub fn my_inventory () -> Inventory{
 InventoryBuilder::new()
    .register(function!(create))
    .register(function!(create_dto))
    .register(function!(create_createorupdatedto))
    .register(function!(create_ef_repository))
    .register(function!(create_exception))
    .register(function!(create_iservice))
    .register(function!(create_manager))
    .register(function!(create_mancreate_pagedandsortedandfilterresultdtoager))
    .register(function!(create_repository_interface))
    .register(function!(create_service))
    .register(function!(insert_mapper))
    .register(function!(format_all))
    .register(function!(dispose))
    .inventory()
}

#[cfg(test)]
mod tests {
    use interoptopus::{Error, util::NamespaceMappings, Interop};

    use super::*;

    #[test]
    fn bindings_csharp() -> Result<(), Error> {
        use interoptopus_backend_csharp::{Config, Generator};
        use interoptopus_backend_csharp::overloads::{DotNet, Unity};
    
        let config = Config {
            dll_name: "example_library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        };
    
        Generator::new(config, my_inventory())
            .add_overload_writer(DotNet::new())
            //.add_overload_writer(Unity::new())
            .write_file(r"../c#/CodeGeneratorApp/CodeGeneratorApp/CodeGeneratorApp.Core/Interop.cs")?;
            // .write_file("../c#/Interop.cs")?;
    
        Ok(())
    }
}
