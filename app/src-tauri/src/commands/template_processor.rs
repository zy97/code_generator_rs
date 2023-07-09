use code_generator::{get_expressions_in_template, process_template};
use erased_serde::Deserializer;

use crate::error::TauriError;

#[tauri::command]
pub fn get_expressions(template: String) -> Result<Vec<String>, TauriError> {
    let expressions = get_expressions_in_template(template)?;
    Ok(expressions)
}
#[tauri::command]
pub async fn process(id: i32, expressions: serde_json::Value) -> Result<String, TauriError> {
    // let data: serde_json::Value = serde_json::from_str(&expressions).unwrap();
    println!("data: {:?}", expressions);
    let result: String = process_template(id, expressions).await?;
    Ok(result)
}