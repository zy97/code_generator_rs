// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod error;
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
            templates_find,
            get_expressions,
            process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use sea_orm::DbErr;

use crate::commands::template_processor::get_expressions;
use crate::commands::template_processor::process;

// create the error type that represents all errors possible in our program
