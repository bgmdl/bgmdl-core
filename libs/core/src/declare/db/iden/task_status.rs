use sea_orm::{DeriveActiveEnum, DeriveIden, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum StatusEnum {
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "InProgress")]
    InProgress,
    #[sea_orm(string_value = "Done")]
    Done,
}

#[derive(DeriveIden)]
pub enum TaskStatus {
    #[sea_orm(iden = "task")]
    Table,
    #[sea_orm(iden = "tsid")]
    TSId,
    #[sea_orm(iden = "tid")]
    TId,
    #[sea_orm(iden = "level")]
    Level,
    #[sea_orm(iden = "content")]
    Content,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
