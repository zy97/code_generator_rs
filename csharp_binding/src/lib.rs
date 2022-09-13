use interoptopus::{Inventory, InventoryBuilder, function, ffi_function, ffi_type};
#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input:Vec2) ->Vec2{
    input
}
pub fn my_inventory () -> Inventory{
 InventoryBuilder::new()
    .register(function!(my_function))
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
            .write_file("Interop.cs")?;
    
        Ok(())
    }
}
