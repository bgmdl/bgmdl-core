use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database};
use sea_orm_migration::prelude::*;

use crate::declare::{db_bgmdata, db_user};

#[derive(DeriveMigrationName)]
pub struct MigrationBgmData;

#[async_trait]
impl MigrationTrait for MigrationBgmData {
    async fn up(&self, manage: &SchemaManager) -> Result<(), DbErr> {
        manage.create_table(
            Table::create()
                .table(db_bgmdata::BgmData::Table)
                .if_not_exists()
                .col(ColumnDef::new(db_bgmdata::BgmData::Id).integer().not_null().primary_key())
                .col(ColumnDef::new(db_bgmdata::BgmData::BgmId).integer().not_null())
                .col(ColumnDef::new(db_bgmdata::BgmData::Status).string().not_null())
                .col(ColumnDef::new(db_bgmdata::BgmData::BgmName).string().not_null())
                .to_owned()
        ).await?;
        manage.create_table(
            Table::create()
                .table(db_user::User::Table)
                .if_not_exists()
                .col(ColumnDef::new(db_user::User::Id).integer().not_null().primary_key())
                .col(ColumnDef::new(db_user::User::Name).string().not_null())
                .col(ColumnDef::new(db_user::User::Password).string().not_null())
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manage: &SchemaManager) -> Result<(), DbErr> {
        manage.drop_table(
            Table::drop()
                .table(db_bgmdata::BgmData::Table)
                .if_exists()
                .to_owned()
        ).await?;
        manage.drop_table(
            Table::drop()
                .table(db_user::User::Table)
                .if_exists()
                .to_owned()
        ).await?;
        Ok(())
    }
}
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(MigrationBgmData),
        ]
    }
}

pub fn init(url: &str, schema: &str, username: &str, hashed_password: &str) -> Result<(), DbErr> {
    let connection_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .to_owned();
    log::info!("Database connecting...");
    let db = async_run! {
        Database::connect(connection_options).await
    }?;
    log::info!("Database connected");
    async_run! {
        Migrator::up(&db, None).await
    }?;
    log::info!("Database migrated");
    log::info!("Init account");
    async_run! {
        use crate::declare::db_user_entity as User;
        let data = User::ActiveModel {
            id: ActiveValue::set(1),
            name: ActiveValue::set(username.to_string()),
            password: ActiveValue::set(hashed_password.to_string()),
        };
        data.insert(&db).await
    }?;
    Ok(())
}