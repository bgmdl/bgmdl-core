use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum BgmData {
    #[sea_orm(iden = "bgm_download")]
    Table,
    Id,
    EpId,
    SubjectId,
    Name,
    BindBgmId,
    BingTaskId,
}
