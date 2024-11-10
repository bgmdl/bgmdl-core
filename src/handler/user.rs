use core::{model, utils::db::conn::get_connect};
use actix_web::{post, services, web, Scope};
use crate::{declare::api::user::CheckLoginProps, handler::ResultHandler, utils::encryption::encode_password};

#[post("/check")]
pub async fn check_login(data: web::Json<CheckLoginProps>) -> ResultHandler<String> {
    let db = get_connect(get_env!(dblink).as_str(), get_env!(dbschema).as_str()).await;
    if db.is_err() {
        log::error!("Get database connection failed: {:?}", db.err().unwrap());
        return Ok(Json! {
            "error": "Internal server error"
        });
    }
    let db = db.unwrap();
    let password = encode_password(&data.password);
    let result = model::user::check_user(&data.username, &password, &db).await;
    if result.is_err() {
        log::error!("Check login failed: {:?}", result.err().unwrap());
        return Ok(Json! {
            "error": "Internal server error"
        });
    }
    let result = result.unwrap();
    if result {
        Ok(Json! {
            "result": "success",
            "_pwd": password
        })
    } else {
        Ok(Json! {
            "result": "failed"
        })
    }
}

pub fn service() -> Scope {
    let services = services![
        check_login
    ];
    web::scope("/api/user").service(services)
}