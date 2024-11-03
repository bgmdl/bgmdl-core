use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum Task {
    #[sea_orm(iden = "task")]
    Table,
    #[sea_orm(iden = "tid")]
    TId,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "description")]
    Description,
}
