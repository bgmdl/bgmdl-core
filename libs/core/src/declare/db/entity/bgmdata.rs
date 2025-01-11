use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter};
// use crate::declare::db::iden::task_status::StatusEnum;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "bgm_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub bgm_status: String,
    pub total_ep: Option<i32>,
    pub now_ep: Option<i32>,
    pub bind_bgm_id: Option<i32>,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub image: Option<String>,
    pub name_cn: Option<String>,
    pub nsfw: Option<bool>,
    pub platform: Option<String>,
    pub rating: Option<f64>,
    pub tags: Option<String>,
    pub summary: Option<String>,
    pub ep_bind: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
