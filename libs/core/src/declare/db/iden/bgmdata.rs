use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum BgmData {
    #[sea_orm(iden = "bgm_data")]
    Table,
    Id,
    BindBgmId, // link with bgm_id
    BgmName,   // bgm name
    Status,
    TotalEp,
    NowEp,
    Year,
    Season,
    #[sea_orm(iden = "nsfw")]
    NSFW,
    #[sea_orm(iden = "name_cn")]
    NameCn,
    Image,
    Platform,
    Summary,
    Tags,
    UpdateTime,
    CreateTime,
    Rating
}
