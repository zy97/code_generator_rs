#[tauri::command]
fn getList() -> Vec<Project> {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
