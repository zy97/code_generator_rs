use code_generator::{projects::Model, projects_svc::*};
use serde::Serialize;

use crate::error::TauriError;

#[tauri::command]
pub async fn projects_get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<Model>), TauriError> {
    let projects = get_list(current, page_size, name).await?;
    Ok(projects)
}

#[tauri::command]
pub async fn projects_create(name: String) -> Result<Model, TauriError> {
    let projects = create(name).await?;
    Ok(projects)
}

#[tauri::command]
pub async fn projects_update(id: i32, name: String) -> Result<Model, TauriError> {
    let projects = update(id, name).await?;
    Ok(projects)
}

#[tauri::command]
pub async fn projects_delete(id: i32) -> Result<bool, TauriError> {
    let projects = delete(id).await?;
    let result = projects.rows_affected > 0;
    Ok(result)
}
#[tauri::command]
pub async fn projects_find(id: i32) -> Result<Option<Model>, TauriError> {
    let projects = find(id).await?;
    Ok(projects)
}
#[tauri::command]
pub async fn projects_get_many(ids: Vec<i32>) -> Result<Vec<ProjectModelWithTitle>, TauriError> {
    let projects = get_many_by_id(ids)
        .await?
        .into_iter()
        .map(|t| ProjectModelWithTitle {
            id: t.id,
            title: t.name,
        })
        .collect();
    Ok(projects)
}

#[derive(Serialize, Debug)]
pub struct ProjectModelOnlyId {
    pub id: i32,
}
#[derive(Serialize, Debug)]
pub struct ProjectModelWithTitle {
    id: i32,
    title: String,
}
