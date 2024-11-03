use sea_orm::{DatabaseConnection, EntityTrait};
use crate::task;
use crate::{declare::error::CoreError, task::model::TaskDetail};
use crate::declare::db::entity::task::Entity as TaskEntity;
use crate::declare::db::entity::task::ActiveModel as TaskActiveModel;
use crate::declare::db::entity::task::Model as TaskModel;

use super::count::gen_id;

pub struct TaskAddResult {
    pub id: i32,
}

pub fn add_task(task: TaskDetail, db: &DatabaseConnection, default: Option<i32>) -> Result<TaskAddResult, CoreError> {
    let tid = gen_id("task", db)?;
    async_run! {
        TaskEntity::insert(
            TaskActiveModel {
                ..(TaskModel {
                    ..(&task).into()
                }
                .set_tid(tid)
                .set_created_at(chrono::offset::Utc::now().naive_utc())).into()
            }
        )
        .exec(db)
        .await
    }?;
    let _ = task::add_task(task, default.unwrap_or(1));
    Ok(TaskAddResult {
        id: tid,
    })
}