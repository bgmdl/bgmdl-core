use crate::declare::db::entity::task::ActiveModel as TaskActiveModel;
use crate::declare::db::entity::task::Column as TaskColumn;
use crate::declare::db::entity::task::Entity as TaskEntity;
use crate::declare::db::entity::task::Model as TaskModel;

use crate::declare::db::entity::task_status::ActiveModel as TaskStatusActiveModel;
use crate::declare::db::entity::task_status::Entity as TaskStatusEntity;

use crate::task;
use crate::task::model::TaskOption;
use crate::{declare::error::CoreError, task::model::TaskDetail};
use sea_orm::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::count::gen_id;

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

#[derive(Debug, Clone)]
pub struct ExistTask {
    pub task_id: i32,
    pub task_detail: TaskDetail,
    pub task_option: TaskOption,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub task_detail: TaskDetail,
}

impl Task {
    pub async fn save(self, db: &DatabaseConnection) -> Result<ExistTask, CoreError> {
        let task_id = gen_id("task", db).await?;
        let task_detail = self.task_detail.clone();
        log::debug!("Add task: {task_id}");
        log::debug!("task detail: {:?}", &self);
        let created_at = now_time!();
        TaskEntity::insert(TaskActiveModel {
            ..(TaskModel {
                ..(&task_detail).into()
            }
            .set_tid(task_id)
            .set_created_at(created_at))
            .into()
        })
        .exec(db)
        .await?;
        let task_option = TaskOption {
            task_id,
            tasktype: "once".to_string().into(),
            created_at,
        };
        log::trace!("add task {task_id} into database done.");
        Ok(ExistTask {
            task_id,
            task_detail,
            task_option,
        })
    }
}

impl ExistTask {
    pub async fn from_db(task_id: i32, db: &DatabaseConnection) -> Result<ExistTask, CoreError> {
        let task_detail = TaskEntity::find()
            .filter(TaskColumn::Tid.eq(task_id))
            .one(db)
            .await?;
        let task_option = TaskOption {
            task_id,
            tasktype: "once".to_string().into(),
            created_at: task_detail.clone().unwrap().created_at,
        };
        Ok(ExistTask {
            task_id,
            task_detail: task_detail.clone().unwrap().into(),
            task_option,
        })
    }

    pub async fn run(self, priority: i32) -> Result<(), CoreError> {
        task::add_task(self.task_detail, self.task_option, priority);
        Ok(())
    }

    pub async fn drop_from_db(self, db: &DatabaseConnection) -> Result<Task, CoreError> {
        TaskEntity::delete(TaskActiveModel {
            tid: Set(self.task_id),
            ..Default::default()
        })
        .exec(db)
        .await?;
        Ok(Task {
            task_detail: self.task_detail,
        })
    }

    pub async fn add_progress(
        self,
        progress: f64,
        speed: i64,
        db: &DatabaseConnection,
    ) -> Result<(), CoreError> {
        let tsid = gen_id("task_status", db).await?;
        log::trace!(
            "Add task_status(for tid:{:?}) tsid:{:?}",
            self.task_id,
            tsid
        );
        TaskStatusEntity::insert(TaskStatusActiveModel {
            tsid: Set(tsid),
            tid: Set(self.task_id),
            level: Set(-1),
            content: Set(format!("progress: {}, {}", progress, speed)),
            created_at: Set(now_time!()),
        })
        .exec(db)
        .await?;
        Ok(())
    }
}

impl From<ExistTask> for Task {
    fn from(task: ExistTask) -> Self {
        Task {
            task_detail: task.task_detail,
        }
    }
}
