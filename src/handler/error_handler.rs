use core::declare::error::CoreError;

use actix_web::{get, Scope, services, web};
use super::ResultHandler;

#[get("/core_string")]
pub async fn string_err() -> ResultHandler<String> {
    Err(CoreError::StringError("error_test".to_string()))?;
    Ok("".to_string())
}

pub fn service() -> Scope {
    let services = services![
        string_err,
    ];
    web::scope("/api/error").service(services)
}