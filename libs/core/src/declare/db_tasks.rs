use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum Task {
    #[sea_orm(iden = "task")]
    Table,
    Id,
    Name,
    Password,
}
