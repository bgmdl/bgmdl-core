use actix_web::{get, post, services, web, Scope};
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

#[get("/detail/{taskid}")]
#[perm("task.detail")]
pub async fn get_task_detail(data: web::Path<i32>) -> ResultHandler<String> {
    let taskid = data.into_inner();
    let (task_detail, task_option, task_status) = core::model::task::get_task_detail(taskid, &get_connect().await.ok().unwrap()).await?;
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