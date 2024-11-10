use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter};
use crate::declare::db::iden::task_status::StatusEnum;
use crate::task::model::TaskDetail;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tid: i32,
    pub name: String,
    pub status: StatusEnum,
    pub description: String,
    pub created_at: DateTime,
}
impl Model {
    pub fn set_status(self, status: StatusEnum) -> Self {
        Self {
            status,
            ..self
        }
    }

    pub fn set_created_at(self, created_at: DateTime) -> Self {
        Self {
            created_at,
            ..self
        }
    }

    pub fn set_tid(self, tid: i32) -> Self {
        Self {
            tid,
            ..self
        }
    }
}

impl From<&TaskDetail> for Model {
    fn from(task: &TaskDetail) -> Self {
        match task {
            TaskDetail::Download(task) => Model {
                tid: -1,
                name: "Download".to_string(),
                status: StatusEnum::Pending,
                created_at: Default::default(),
                description: Json! {
                    "url": task.url.clone(),
                    "path": task.save_path.clone(),
                },
            },
            TaskDetail::DownloadAll(task) => Model {
                tid: -1,
                name: "DownloadAll".to_string(),
                status: StatusEnum::Pending,
                created_at: Default::default(),
                description: Json! {
                    "url": task.url.clone(),
                    "path": task.save_path.clone(),
                },
            },
            TaskDetail::ChangeName(task) => Model {
                tid: -1,
                name: "ChangeName".to_string(),
                status: StatusEnum::Pending,
                created_at: Default::default(),
                description: Json! {
                    "path": task.path.clone(),
                    "name": task.name.clone()
                },
            },
            TaskDetail::ReportError(_task) => Model {
                tid: -1,
                name: "ReportError".to_string(),
                status: StatusEnum::Pending,
                created_at: Default::default(),
                description: Json! {
                    "msg": "ReportError".to_string()
                },
            },
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
}