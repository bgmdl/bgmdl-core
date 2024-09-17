use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use crate::declare::db_user_entity::Entity as UserEntity;
use crate::declare::db_user_entity::Column as UserColumn;
use crate::declare::error::CoreError;

pub fn check_user(username: &str, password: &str, db: &DatabaseConnection) -> Result<bool, CoreError> {
    let user = async_run!{ 
        UserEntity::find()
            .filter(UserColumn::Name.eq(username))
            .one(db)
            .await
    }?;
    if let Some(user) = user {
        if user.password == password {
            return Ok(true);
        }
    }
    Ok(false)
}