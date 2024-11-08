// use core::{utils::db::conn::get_connect, model};
use actix_web::{post, services, web, Scope};
use macro_lib::perm;
use crate::declare::api::user::TaskAddProps;
use crate::utils::check_perm::check_user_permission;

use crate::handler::ResultHandler;

#[post("/add")]
#[perm("task.add")]
pub async fn add_task(_data: web::Json<TaskAddProps>) -> ResultHandler<String> {
    Ok(Json!{
        "status": "success",
    })
}

pub fn service() -> Scope {
    let services = services![
        add_task
    ];
    web::scope("/api/task").service(services)
}