use code_generator::get_expressions_in_template;

use crate::error::TauriError;

#[tauri::command]
pub fn get_expressions(template: String) -> Result<Vec<String>, TauriError> {
    let expressions = get_expressions_in_template(template)?;
    Ok(expressions)
}
