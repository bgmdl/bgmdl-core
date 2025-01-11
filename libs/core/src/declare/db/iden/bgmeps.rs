use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum BgmEps {
    #[sea_orm(iden = "bgm_eps")]
    Table,
    Id,
    EpId,
    SubjectId,
    Name,
    Duration,
    AirDate,
    Desc,
    Ep,
    Sort,
    Comment,
    Disc,
    DurationSecond,
}
