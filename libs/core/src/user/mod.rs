use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, entity::*};
use crate::declare::db_user_entity::Entity as UserEntity;
use crate::declare::db_user_entity::Column as UserColumn;
use crate::declare::db_user_entity::ActiveModel as UserModel;
use crate::declare::error::CoreError;

pub fn change_password(new_password: &str, db: &DatabaseConnection) {
    async_run!{
        let _ = UserEntity::update(UserModel {
            id: Set(1),
            password: Set(new_password.to_string()),
            ..Default::default()
        })
        .filter(UserColumn::Id.eq(1))
        .exec(db).await;
    }
}

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