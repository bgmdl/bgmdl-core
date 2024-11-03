use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, entity::*};
use crate::declare::db::entity::user::Entity as UserEntity;
use crate::declare::db::entity::user::Column as UserColumn;
use crate::declare::db::entity::user::ActiveModel as UserActiveModel;
use crate::declare::error::CoreError;

pub fn change_password(new_password: &str, db: &DatabaseConnection) {
    async_run!{
        let _ = UserEntity::update(UserActiveModel {
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