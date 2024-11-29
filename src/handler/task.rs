use actix_web::{post, services, web, Scope};
use macro_lib::perm;
use crate::declare::api::task::{TaskAddProps, TaskGetDetailProps};
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

#[post("/get_detail")]
#[perm("task.get_detail")]
pub async fn get_task_detail(data: web::Json<TaskGetDetailProps>) -> ResultHandler<String> {
    let data: TaskGetDetailProps = data.into_inner();
    let (task_detail, task_option, task_status) = core::model::task::get_task_detail(data.taskid, &get_connect().await.ok().unwrap()).await?;
    Ok(Json!{
        "status": "success",
        "task_detail": task_detail,
        "task_option": task_option,
        "task_status": task_status,
    })
}

pub fn service() -> Scope {
    let services = services![
        add_task,
        get_task_detail,
    ];
    web::scope("/api/task").service(services)
}