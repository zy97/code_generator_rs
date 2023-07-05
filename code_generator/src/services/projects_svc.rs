use crate::{
    db_entities::{prelude::*, *},
    projects::{ActiveModel, Model},
    DATABASE_URL,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, Database, DbErr, DeleteResult, EntityTrait, InsertResult,
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
pub async fn get_list() -> Result<Vec<projects::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(Projects::find().all(&db).await?)
}
