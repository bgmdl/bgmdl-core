use core::declare::error::CoreError;
use std::sync::Mutex;

use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

lazy_static! {
    pub static ref DB_CONNECT: Mutex<Option<DatabaseConnection>> = Mutex::new(None);
}

pub async fn get_connect() -> Result<DatabaseConnection, CoreError> {
    if DB_CONNECT.lock().unwrap().is_none() || DB_CONNECT.lock().unwrap().as_ref().unwrap().ping().await.is_err() {
        let mut options = ConnectOptions::new(get_env!(dblink).as_str())
            .set_schema_search_path(get_env!(dbschema).as_str())
            .to_owned();
        options.max_connections(100);
        *DB_CONNECT.lock().unwrap() = Some(Database::connect(options).await?);
    }
    Ok(DB_CONNECT.lock().unwrap().as_ref().unwrap().clone())
}