use crate::DATABASE_URL;

use models::templates::{self, Model};
// use entities::templates::Model;
// use entities::{prelude::Templates, templates};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Database, DbErr, DeleteResult, EntityTrait,
    PaginatorTrait, QueryFilter, QueryTrait,
};

pub async fn create(
    name: String,
    content: String,
    project_id: i32,
    expressions: Vec<String>,
) -> Result<Model, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let expressions = expressions.join(",");
    let template = templates::ActiveModel {
        name: ActiveValue::Set(name),
        content: ActiveValue::Set(content),
        project_id: ActiveValue::Set(project_id),
        expressions: ActiveValue::Set(Some(expressions)),
        ..Default::default()
    };
    let res = template.insert(&db).await?;
    Ok(res)
}
pub async fn update(
    id: i32,
    name: String,
    content: String,
    project_id: i32,
    expressions: Vec<String>,
) -> Result<Model, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let expressions = expressions.join(",");
    let template = templates::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(name),
        content: ActiveValue::Set(content),
        expressions: ActiveValue::Set(Some(expressions)),
        project_id: ActiveValue::Set(project_id),
    };
    let res = Templates::update(template).exec(&db).await?;
    Ok(res)
}
pub async fn find(id: i32) -> Result<Option<templates::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let template: Option<templates::Model> = Templates::find_by_id(id).one(&db).await?;
    Ok(template)
}
pub async fn delete(id: i32) -> Result<DeleteResult, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let template = templates::ActiveModel {
        id: ActiveValue::Set(id), // The primary key must be set
        ..Default::default()
    };
    let result: DeleteResult = template.delete(&db).await?;
    Ok(result)
}
pub async fn get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<templates::Model>), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let mut select = Templates::find();

    select = select.apply_if(name, |query, value| {
        query.filter(templates::Column::Name.contains(&value))
    });
    // let sql: String = select.build(sea_orm::DatabaseBackend::Sqlite).sql;
    // println!("${sql}");
    let total = select.clone().count(&db).await?;
    let results = select
        .paginate(&db, page_size)
        .fetch_page(current - 1)
        .await?;

    Ok((total, results))
}
pub async fn get_all() -> Result<Vec<templates::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(templates::Entity::find().all(&db).await?)
}
