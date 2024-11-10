use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, entity::*};
use sea_orm::QueryFilter;
use crate::declare::db::entity::count::Entity as CountEntity;
use crate::declare::db::entity::count::ActiveModel as CountActiveModel;
use crate::declare::db::entity::count::Column as CountColumn;
use crate::declare::error::CoreError;

pub async fn get_id(key: &str, db: &DatabaseConnection) -> Result<i32, CoreError> {
    let count = CountEntity::find()
        .filter(CountColumn::Key.eq(key))
        .one(db)
        .await?;
    Ok(count.unwrap().value)
}

pub async fn gen_id(key: &str, db: &DatabaseConnection) -> Result<i32, CoreError> {
    let count = CountEntity::find()
        .filter(CountColumn::Key.eq(key))
        .one(db)
        .await?;
    let count = count.unwrap();
    let new_count = count.value + 1;
    CountEntity::update(CountActiveModel {
        value: Set(new_count),
        key: Set(key.to_string())
    })
    .filter(CountColumn::Key.eq(key))
    .exec(db)
    .await?;
    Ok(new_count)
}