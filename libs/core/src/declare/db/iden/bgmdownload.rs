use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum BangumiDownload {
    #[sea_orm(iden = "bgm_download")]
    Table,
    Id,
    EpId,
    FansubId,
    Time,
    BindBgmId,
    BingTaskId,
}
