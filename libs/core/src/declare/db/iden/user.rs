use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "user")]
    Table,
    Id,
    Name,
    Password,
}
