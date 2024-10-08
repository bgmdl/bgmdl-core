use core::db::conn::get_connect;

use actix_web::{post, services, web, Scope};
use serde::Deserialize;

use crate::handler::ResultHandler;

#[derive(Deserialize, Debug, Clone)]
struct CheckLoginProps {
    username: String,
    password: String, //hashed password
}

#[post("/check")]
pub async fn check_login(data: web::Json<CheckLoginProps>) -> ResultHandler<String> {
    let db = get_connect("", ""); //TODO: get from global data.
    if db.is_err() {
        log::error!("Get database connection failed: {:?}", db.err().unwrap());
        return Ok(Json! {
            "error": "Internal server error"
        });
    }
    let db = db.unwrap();
    let result = core::user::check_user(&data.username, &data.password, &db);
    if result.is_err() {
        log::error!("Check login failed: {:?}", result.err().unwrap());
        return Ok(Json! {
            "error": "Internal server error"
        });
    }
    let result = result.unwrap();
    if result == true {
        Ok(Json! {
            "result": "success"
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