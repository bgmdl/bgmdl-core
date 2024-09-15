use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::{migrator, prelude::*};
use tokio::runtime;

use crate::declare::db_bgmdata;

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
        Ok(())
    }

    async fn down(&self, manage: &SchemaManager) -> Result<(), DbErr> {
        
        Ok(())
    }
}

macro_rules! async_run {
    ($($body:tt)*) => {{
        let bt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        bt.block_on(async {
            $($body)*
        })
    }};
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

pub fn init(url: &str, schema: &str) -> Result<(), DbErr> {
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
    Ok(())
}