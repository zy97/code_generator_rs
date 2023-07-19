use crate::error::TauriError;
use code_generator::templates_svc::*;
use entity::templates::Model;

use super::project::ProjectModelOnlyId;
#[tauri::command]
pub async fn templates_get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<ListModel>), TauriError> {
    let templates = get_list(current, page_size, name).await?;
    Ok((
        templates.0,
        templates
            .1
            .into_iter()
            .map(|t| ListModel {
                id: t.id,
                content: t.content,
                name: t.name,
                project: ProjectModelOnlyId { id: t.project_id },
            })
            .collect(),
    ))
}
#[tauri::command]
pub async fn templates_create(
    name: String,
    content: String,
    project_id: i32,
    expressions: Vec<String>,
) -> Result<Model, TauriError> {
    let templates = create(name, content, project_id, expressions).await?;
    Ok(templates)
}
#[tauri::command]
pub async fn templates_update(
    id: i32,
    name: String,
    content: String,
    project_id: i32,
    expressions: Vec<String>,
) -> Result<Model, TauriError> {
    let templates = update(id, name, content, project_id, expressions).await?;
    Ok(templates)
}
#[tauri::command]
pub async fn templates_delete(id: i32) -> Result<bool, TauriError> {
    let templates = delete(id).await?;
    Ok(templates.rows_affected > 0)
}
#[tauri::command]
pub async fn templates_find(id: i32) -> Result<Option<DtoModel>, TauriError> {
    let templates = find(id).await?.map(|t| DtoModel {
        expressions: t
            .expressions
            .unwrap_or("".to_string())
            .split(",")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect(),
        id: t.id,
        content: t.content,
        name: t.name,
        project_id: t.project_id,
    });
    Ok(templates)
}
#[derive(serde::Serialize)]
pub struct ListModel {
    id: i32,
    name: String,
    content: String,
    project: ProjectModelOnlyId,
}
#[derive(serde::Serialize)]
pub struct InsertModel {
    name: String,
    content: String,
    project: ProjectModelOnlyId,
}
#[derive(serde::Serialize, Debug)]
pub struct DtoModel {
    id: i32,
    name: String,
    content: String,
    project_id: i32,
    expressions: Vec<String>,
}
