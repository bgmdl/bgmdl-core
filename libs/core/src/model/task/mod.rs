use crate::declare::db::entity::task::ActiveModel as TaskActiveModel;
use crate::declare::db::entity::task::Column as TaskColumn;
use crate::declare::db::entity::task::Entity as TaskEntity;
use crate::declare::db::entity::task::Model as TaskModel;

use crate::declare::db::entity::task_status::ActiveModel as TaskStatusActiveModel;
use crate::declare::db::entity::task_status::Column as TaskStatusColumn;
use crate::declare::db::entity::task_status::Entity as TaskStatusEntity;
use crate::declare::db::entity::task_status::Model as TaskStatusModel;

use crate::task;
use crate::task::model::TaskOption;
use crate::{declare::error::CoreError, task::model::TaskDetail};
use sea_orm::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

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
    log::debug!("Add task: {tid}");
    log::debug!("task detail: {:?}", &task);
    let created_at = now_time!();
    TaskEntity::insert(TaskActiveModel {
        ..(TaskModel { ..(&task).into() }
            .set_tid(tid)
            .set_created_at(created_at))
        .into()
    })
    .exec(db)
    .await?;
    let task_option = TaskOption {
        taskid: tid,
        tasktype: "once".to_string().into(),
        created_at,
    };
    log::trace!("add task {tid} into database done.");
    task::add_task(task, task_option, default.unwrap_or(1));
    Ok(TaskAddResult { id: tid })
}

pub async fn add_task_progress(
    task_id: i32,
    progress: f64,
    speed: i64,
    db: &DatabaseConnection,
) -> Result<(), CoreError> {
    let tsid = gen_id("task_status", db).await?;
    log::trace!("Add task_status(for tid:{:?}) tsid:{:?}", task_id, tsid);
    TaskStatusEntity::insert(TaskStatusActiveModel {
        tsid: Set(tsid),
        tid: Set(task_id),
        level: Set(-1),
        content: Set(format!("progress: {}, {}", progress, speed)),
        created_at: Set(now_time!()),
    })
    .exec(db)
    .await?;
    Ok(())
}

pub async fn get_task_detail(
    taskid: i32,
    db: &DatabaseConnection,
) -> Result<(TaskDetail, TaskOption, Vec<TaskStatusModel>), CoreError> {
    let task_detail = TaskEntity::find()
        .filter(TaskColumn::Tid.eq(taskid))
        .one(db)
        .await?;
    let task_status = TaskStatusEntity::find()
        .filter(TaskStatusColumn::Tid.eq(taskid))
        .all(db)
        .await?;
    Ok((
        task_detail.clone().unwrap().into(),
        task_detail.clone().unwrap().into(),
        task_status,
    ))
}
