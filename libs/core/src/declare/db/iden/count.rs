use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum Count {
    #[sea_orm(iden = "count")]
    Table,
    #[sea_orm(iden = "key")]
    Key,
    #[sea_orm(iden = "value")]
    Value
}
