use sea_orm::{schema, ConnectOptions, Database, DatabaseConnection};

use crate::declare::error::CoreError;

pub fn get_connect(url: &str, schema: &str) -> Result<DatabaseConnection, CoreError> {
    let connection_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .to_owned();
    let db = async_run! {
        Database::connect(connection_options).await
    }?;
    Ok(db)
}