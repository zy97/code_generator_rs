use crate::{
    db_entities::{prelude::*, *},
    DATABASE_URL,
};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbErr, EntityTrait};

async fn Insert() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = projects::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        ..Default::default()
    };
    let res = Projects::insert(project).exec(&db).await?;
    Ok(())
}
async fn Update() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = projects::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        ..Default::default()
    };
    let res = Projects::update(project).exec(&db).await?;
    Ok(())
}
async fn Find() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project: Option<projects::Model> = Projects::find_by_id(1).one(&db).await?;
    Ok(())
}
async fn Delete() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let project = projects::ActiveModel {
        id: ActiveValue::Set(1), // The primary key must be set
        ..Default::default()
    };
    let s = project.delete(&db).await?;
    Ok(())
}
async fn GetList() -> Result<Vec<projects::Model>, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(Projects::find().all(&db).await?)
}
