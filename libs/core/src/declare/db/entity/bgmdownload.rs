use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "bgm_download")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ep_id: i32,
    pub subject_id: i32,
    pub name: String,
    pub duration: String,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
