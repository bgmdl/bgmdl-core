use actix_web::{post, services, web, Scope};
use macro_lib::perm;
use crate::declare::api::task::TaskAddProps;
use crate::utils::check_perm::check_user_permission;
use crate::handler::ResultHandler;
use crate::utils::db::get_connect;

#[post("/add")]
#[perm("task.add")]
pub async fn add_task(data: web::Json<TaskAddProps>) -> ResultHandler<String> {
    let data: TaskAddProps = data.into_inner();
    let _ = core::model::task::add_task(data.into(), &get_connect().await.ok().unwrap(), Some(1)).await;
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