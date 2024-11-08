use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::declare::error::CoreError;

pub async fn get_connect(url: &str, schema: &str) -> Result<DatabaseConnection, CoreError> {
    let connection_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .to_owned();
    let db = Database::connect(connection_options).await?;
    Ok(db)
}