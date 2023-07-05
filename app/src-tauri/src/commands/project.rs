use code_generator::{
    projects::{ActiveModel, Model},
    projects_svc::*,
};
#[tauri::command()]
pub async fn projects_get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<Model>), String> {
    let projects = get_list(current, page_size, name)
        .await
        .expect("获取项目列表失败");
    println!(
        "get list: {:?},current:{},page_size:{}",
        projects, current, page_size
    );
    Ok(projects)
}
#[tauri::command()]
pub async fn projects_create(name: String) -> Result<Model, String> {
    let projects = create(name).await.expect("创建项目失败");
    println!("create: {:?}", projects);
    Ok(projects)
}
#[tauri::command()]
pub async fn projects_update(id: i32, name: String) -> Result<Model, String> {
    let projects = update(id, name).await.expect("更新项目失败");
    println!("update: {:?}", projects);
    Ok(projects)
}
#[tauri::command()]
pub async fn projects_delete(id: i32) -> Result<bool, String> {
    let projects = delete(id).await.expect("删除项目失败");
    println!("delete: {:?}", projects);
    Ok(projects.rows_affected > 0)
}
#[tauri::command()]
pub async fn projects_find(id: i32) -> Result<Option<Model>, String> {
    println!("12");
    let projects = find(id).await.expect("获取项目失败");
    println!("get one: {:?}", projects);
    Ok(projects)
}
