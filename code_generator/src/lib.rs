use std::collections::HashMap;

use inflector::Inflector;
use tera::{to_value, try_get_value, Tera, Value};
extern crate encoding;
mod entities;
mod error;
pub use entities::Entity;
pub use entities::WebEntity;
pub use entities::Permission;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec![".ts", ".tsx"]);
        tera.register_filter("snake", snake);
        tera.register_filter("plural", plural);
        // println!("{:?}",tera);
        tera
    };
}
fn snake(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("snake", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}
fn plural(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = try_get_value!("plural", "value", String, value);
    Ok(to_value(&s.to_plural()).unwrap())
}

