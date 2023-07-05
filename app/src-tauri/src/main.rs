// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
use commands::project::*;
use commands::template::*;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            projects_get_list,
            projects_create,
            projects_update,
            projects_delete,
            projects_find,
            projects_get_many,
            templates_get_list,
            templates_create,
            templates_update,
            templates_delete,
            templates_find
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
