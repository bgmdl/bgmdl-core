use async_trait::async_trait;
use log::LevelFilter;
use macro_db_init::table_create;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database};
use sea_orm_migration::prelude::*;

use crate::declare::db::iden;

#[derive(DeriveMigrationName)]
pub struct MigrationBgmData;

#[async_trait]
impl MigrationTrait for MigrationBgmData {
    async fn up(&self, manage: &SchemaManager) -> Result<(), DbErr> {
        log::info!("Creating table user");
        manage
            .create_table(table_create!(iden::user::User, {
                Id: integer not_null primary_key,
                Name: string not_null,
                Password: string not_null,
            }))
            .await?;
        log::info!("Creating table task");
        manage
            .create_table(table_create!(iden::task::Task, {
                TId: integer not_null primary_key,
                Name: string not_null,
                Status: string not_null,
                Description: string not_null,
                CreatedAt: date_time not_null,
            }))
            .await?;
        log::info!("Creating table task_status");
        manage
            .create_table(table_create!(iden::task_status::TaskStatus, {
                TSId: integer not_null primary_key,
                TId: integer not_null,
                Level: integer not_null,
                Content: string not_null,
                CreatedAt: date_time not_null,
            }))
            .await?;
        log::info!("Creating table count");
        manage
            .create_table(table_create!(iden::count::Count, {
                Key: string not_null primary_key,
                Value: integer not_null,
            }))
            .await?;
        log::info!("Creating table bgmdata");
        manage
            .create_table(table_create!(iden::bgmdata::BgmData, {
                Id: integer not_null primary_key,
                BindBgmId: integer,
                Status: string,
                BgmName: string,
                TotalEp: integer,
                NowEp: integer,
                Year: integer,
                Season: integer,
                Image: string,
                NameCn: string,
                NSFW: boolean,
                Platform: string,
                Rating: float,
                Tags: string, // save as json
                Summary: string,
                Name: date_time not_null,
                EpBind: string,
            }))
            .await?;
        log::info!("Creating table bgmeps");
        manage
            .create_table(table_create! (iden::bgmeps::BgmEps, {
                Id: integer not_null primary_key,
                EpId: integer,
                SubjectId: integer,
                Name: string,
                Duration: string,
                AirDate: string,
                Desc: string,
                Ep: integer,
                Sort: integer,
                Comment: integer,
                Disc: integer,
                DurationSecond: integer,
            }))
            .await?;
        manage
            .create_table(table_create!(iden::bgmdownload::BangumiDownload, {
                Id: integer not_null primary_key,
                EpId: integer,
                FansubId: string,
                Time: date_time,
                BindBgmId: integer,
                BingTaskId: integer,
            }))
            .await?;
        Ok(())
    }

    async fn down(&self, manage: &SchemaManager) -> Result<(), DbErr> {
        log::warn!("Dropping table bgmdata");
        manage
            .drop_table(
                Table::drop()
                    .table(iden::bgmdata::BgmData::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        log::warn!("Dropping table user");
        manage
            .drop_table(
                Table::drop()
                    .table(iden::user::User::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        log::warn!("Dropping table task");
        manage
            .drop_table(
                Table::drop()
                    .table(iden::task::Task::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        log::warn!("Dropping table task_status");
        manage
            .drop_table(
                Table::drop()
                    .table(iden::task_status::TaskStatus::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manage
            .drop_table(
                Table::drop()
                    .table(iden::count::Count::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        log::warn!("Dropping table bgmeps");
        manage
            .drop_table(
                Table::drop()
                    .table(iden::bgmeps::BgmEps::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(MigrationBgmData)]
    }
}

pub fn init(url: &str, schema: &str, username: &str, hashed_password: &str) -> Result<(), DbErr> {
    let connection_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .sqlx_logging_level(LevelFilter::Trace)
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
        use crate::declare::db::entity::user as User;
        let data = User::ActiveModel {
            id: ActiveValue::set(1),
            name: ActiveValue::set(username.to_string()),
            password: ActiveValue::set(hashed_password.to_string()),
        };
        data.insert(&db).await
    }?;
    log::info!("Account initialized");
    log::info!("Init basic count");
    async_run! {
        use crate::declare::db::entity::count as Count;
        let data = Count::ActiveModel {
            key: ActiveValue::set("task".to_string()),
            value: ActiveValue::set(0),
        };
        let _ = data.insert(&db).await;
        let data = Count::ActiveModel {
            key: ActiveValue::set("bgm".to_string()),
            value: ActiveValue::set(0),
        };
        let _ = data.insert(&db).await;
        let data = Count::ActiveModel {
            key: ActiveValue::set("task_status".to_string()),
            value: ActiveValue::set(0),
        };
        let _ = data.insert(&db).await;
    };
    log::info!("Basic count initialized");
    Ok(())
}
