use crate::declare::db::entity::task::ActiveModel as TaskActiveModel;
use crate::declare::db::entity::task::Entity as TaskEntity;
use crate::declare::db::entity::task::Model as TaskModel;
use crate::task;
use crate::{declare::error::CoreError, task::model::TaskDetail};
use sea_orm::{DatabaseConnection, EntityTrait};

use super::count::gen_id;

pub struct TaskAddResult {
    pub id: i32,
}

pub async fn add_task(
    task: TaskDetail,
    db: &DatabaseConnection,
    default: Option<i32>,
) -> Result<TaskAddResult, CoreError> {
    let tid = gen_id("task", db).await?;
    log::info!("Add task: {:?}", tid);
    let res = TaskEntity::insert(TaskActiveModel {
        ..(TaskModel { ..(&task).into() }
            .set_tid(tid)
            .set_created_at(chrono::offset::Utc::now().naive_utc()))
        .into()
    })
    .exec(db)
    .await;
    dbg!(&res);
    log::info!("Add task: {:?}", tid);
    task::add_task(task, default.unwrap_or(1));
    Ok(TaskAddResult { id: tid })
}
