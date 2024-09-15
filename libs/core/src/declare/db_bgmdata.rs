use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum BgmData {
    #[sea_orm(iden = "bgm_data")]
    Table,
    Id,
    BgmId, // link with bgm_id
    BgmName, // bgm name
    Status
}
