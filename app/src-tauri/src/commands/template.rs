use code_generator::{templates::Model, templates_svc::*};

use super::project::ProjectModelOnlyId;
#[tauri::command]
pub async fn templates_get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<ListModel>), String> {
    let templates = get_list(current, page_size, name)
        .await
        .expect("获取模板列表失败");
    println!(
        "get list: {:?},current:{},page_size:{}",
        templates, current, page_size
    );
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
) -> Result<Model, String> {
    let templates = create(name, content, project_id)
        .await
        .expect("创建模板失败");
    println!("create: {:?}", templates);
    Ok(templates)
}
#[tauri::command]
pub async fn templates_update(
    id: i32,
    name: String,
    content: String,
    project_id: i32,
) -> Result<Model, String> {
    let templates = update(id, name, content, project_id)
        .await
        .expect("更新模板失败");
    println!("update: {:?}", templates);
    Ok(templates)
}
#[tauri::command]
pub async fn templates_delete(id: i32) -> Result<bool, String> {
    let templates = delete(id).await.expect("删除模板失败");
    println!("delete: {:?}", templates);
    Ok(templates.rows_affected > 0)
}
#[tauri::command]
pub async fn templates_find(id: i32) -> Result<Option<Model>, String> {
    let templates = find(id).await.expect("获取模板失败");
    println!("get one: {:?}", templates);
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
