use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter};
// use crate::declare::db::iden::task_status::StatusEnum;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "bgm_eps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ep_id: i32,
    pub subject_id: i32,
    pub name: String,
    pub duration: String,
    pub air_date: String,
    pub desc: String,
    pub ep: i32,
    pub sort: i32,
    pub comment: i32,
    pub disc: i32,
    pub duration_second: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
