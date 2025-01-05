use core::fmt;

use chrono::NaiveDateTime;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use tap::Conv;

use crate::bangumi;
use crate::declare::db::entity::bgmdata::ActiveModel as BangumiActiveModel;
use crate::declare::db::entity::bgmdata::Column as BangumiColumn;
use crate::declare::db::entity::bgmdata::Entity as BangumiEntity;
use crate::declare::db::entity::bgmdata::Model as BangumiModel;

use crate::{declare::error::CoreError, model::count::gen_id};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BangumiStatus {
    Air,
    End,
    #[default]
    NotStart,
}

impl fmt::Display for BangumiStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BangumiStatus::Air => write!(f, "Air"),
            BangumiStatus::End => write!(f, "End"),
            BangumiStatus::NotStart => write!(f, "NotStart"),
        }
    }
}

impl From<String> for BangumiStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Air" | "air" => BangumiStatus::Air,
            "End" | "end" => BangumiStatus::End,
            _ => BangumiStatus::NotStart,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BangumiInfo {
    pub db_id: Option<i32>,
    /// None -> create new, Some -> update
    pub bind_bgm_id: Option<i32>,
    pub bgm_status: BangumiStatus,
    pub total_ep: Option<i32>,
    pub now_ep: Option<i32>,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub image: Option<String>,
    pub name_cn: Option<String>,
    pub name: Option<String>,
    pub nsfw: Option<bool>,
    pub platform: Option<String>,
    pub rating: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDownloadInfo {
    pub ep: i32,
    pub fansub: Vec<String>,
    pub download_time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDownload {
    pub download_info: Vec<BangumiDownloadInfo>,
    pub fansub_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bangumi {
    pub id: i32,
    #[serde(flatten)]
    pub info: Option<BangumiInfo>,
    #[serde(flatten)]
    pub download: Option<BangumiDownload>,
    /// lock_data: which data is locked by user.
    pub lock_data: Vec<String>,
}

impl From<&BangumiInfo> for BangumiModel {
    fn from(data: &BangumiInfo) -> Self {
        let bgminfo = data.clone();
        BangumiModel {
            id: bgminfo.db_id.unwrap_or(-1),
            name: bgminfo.name.clone().unwrap_or("unknown".to_string()),
            bgm_status: bgminfo.bgm_status.to_string(),
            total_ep: bgminfo.total_ep,
            now_ep: bgminfo.now_ep,
            bind_bgm_id: bgminfo.bind_bgm_id,
            year: bgminfo.year,
            season: bgminfo.season,
            image: bgminfo.image,
            name_cn: bgminfo.name_cn,
            nsfw: bgminfo.nsfw,
            platform: bgminfo.platform,
            rating: bgminfo.rating,
            tags: {
                if let Some(tags) = bgminfo.tags.clone() {
                    serde_json::to_string(&tags).ok()
                } else {
                    None
                }
            },
            summary: bgminfo.summary,
        }
    }
}

pub enum SyncType {
    BgmTv,
}

fn parse_date(data: &str) -> (i32, i32) {
    let date = data.split('-').collect::<Vec<&str>>();
    let year = date[0].parse::<i32>().unwrap();
    let month = date[1].parse::<i32>().unwrap();
    (year, month)
}

macro_rules! sync_prop_with_check {
    ($bgm_data:expr, $lock_data:expr, $($prop:ident: $new_prop:expr),*) => {
        $(
            if !$lock_data.contains(&stringify!($prop).to_string()) {
                $bgm_data.$prop = $new_prop;
            }
        )*
    };
}

#[allow(clippy::large_enum_variant)]
pub enum SaveInfoData {
    UpdateSuccess(()),
    SaveData(Bangumi),
}

impl From<SaveInfoData> for Bangumi {
    fn from(val: SaveInfoData) -> Self {
        match val {
            SaveInfoData::SaveData(data) => data,
            SaveInfoData::UpdateSuccess(_) => panic!("UpdateSuccess can't convert to Bangumi"),
        }
    }
}

impl BangumiInfo {
    pub async fn save_info(self, db: &DatabaseConnection) -> Result<SaveInfoData, CoreError> {
        let mut info = self.clone();
        if let Some(db_id) = self.db_id {
            BangumiEntity::update((&info).conv::<BangumiModel>().conv::<BangumiActiveModel>())
                .filter(BangumiColumn::Id.eq(db_id))
                .exec(db)
                .await?;
            return Ok(SaveInfoData::UpdateSuccess(()));
        }
        let id = gen_id("bgm", db).await?;
        info.db_id = Some(id);
        log::debug!("Add bangumi: {id}");
        log::debug!("bangumi data: {:?}", &self);
        let res =
            BangumiEntity::insert((&info).conv::<BangumiModel>().conv::<BangumiActiveModel>())
                .exec(db)
                .await;
        dbg!(&res);
        log::trace!("add bangumi {id} into database done.");
        Ok(SaveInfoData::SaveData(Bangumi {
            id,
            info: Some(info),
            download: None,
            lock_data: vec![],
        }))
    }

    pub async fn sync_info(
        &mut self,
        sync_type: SyncType,
        lock_data: Vec<String>,
    ) -> Result<(), CoreError> {
        match sync_type {
            SyncType::BgmTv => {
                let client = reqwest::Client::new();
                let new_data = bangumi::get_bangumi_info_with_bgmid_async(
                    client,
                    self.bind_bgm_id.unwrap_or(1),
                )
                .await?;
                let (year, month) =
                    parse_date(new_data.date.unwrap_or("1000-01-01".to_string()).as_str());
                sync_prop_with_check! {*self, lock_data,
                    year: Some(year),
                    season: Some(month),
                    image: {
                        if let Some(image) = new_data.images {
                            image.large
                        } else {
                            None
                        }
                    },
                    name_cn: new_data.name_cn,
                    nsfw: new_data.nsfw,
                    platform: new_data.platform,
                    rating: Some(new_data.rating.unwrap().score.unwrap()),
                    tags: Some(new_data.tags.unwrap().iter().map(|x| x.name.clone().unwrap()).collect()),
                    summary: new_data.summary
                };
                Ok(())
            }
        }
    }
}

impl Bangumi {}
