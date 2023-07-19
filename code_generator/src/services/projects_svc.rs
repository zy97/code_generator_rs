use crate::DATABASE_URL;
use entity::{
    prelude::Projects,
    projects::{self, Model},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Database, DbErr, DeleteResult, EntityTrait,
    PaginatorTrait, QueryFilter, QueryTrait,
};

pub async fn create(name: String) -> Result<Model, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let project = projects::ActiveModel {
        name: ActiveValue::Set(name),
        ..Default::default()
    };
    let res = project.insert(&db).await?;
    Ok(res)
}
pub async fn update(id: i32, name: String) -> Result<Model, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = projects::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(name),
    };
    let res = Projects::update(project).exec(&db).await?;
    Ok(res)
}
pub async fn find(id: i32) -> Result<Option<projects::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project: Option<projects::Model> = Projects::find_by_id(id).one(&db).await?;
    Ok(project)
}
pub async fn delete(id: i32) -> Result<DeleteResult, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = projects::ActiveModel {
        id: ActiveValue::Set(id), // The primary key must be set
        ..Default::default()
    };
    let result: DeleteResult = project.delete(&db).await?;
    Ok(result)
}
pub async fn get_list(
    current: u64,
    page_size: u64,
    name: Option<String>,
) -> Result<(u64, Vec<projects::Model>), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let mut select = Projects::find();

    select = select.apply_if(name, |query, value| {
        query.filter(projects::Column::Name.contains(&value))
    });
    // let sql: String = select.build(sea_orm::DatabaseBackend::Sqlite).sql;
    // println!("${sql}");
    let total = select.clone().count(&db).await?;
    println!("{}", total);
    let results = select
        .paginate(&db, page_size)
        .fetch_page(current - 1)
        .await?;

    Ok((total, results))
}
pub async fn get_many_by_id(ids: Vec<i32>) -> Result<Vec<projects::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = Projects::find()
        .filter(projects::Column::Id.is_in(ids))
        .all(&db)
        .await?;
    Ok(project)
}
