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
    pub ep_bind: Option<Vec<i32>>,
}

impl From<BangumiModel> for BangumiInfo {
    fn from(data: BangumiModel) -> Self {
        let tags = if let Some(tags) = data.tags {
            serde_json::from_str(&tags).ok()
        } else {
            None
        };
        let ep_bind = if let Some(ep_bind) = data.ep_bind {
            serde_json::from_str(&ep_bind).ok()
        } else {
            None
        };
        BangumiInfo {
            db_id: Some(data.id),
            bgm_status: data.bgm_status.into(),
            total_ep: data.total_ep,
            now_ep: data.now_ep,
            bind_bgm_id: data.bind_bgm_id,
            year: data.year,
            season: data.season,
            image: data.image,
            name_cn: data.name_cn,
            name: Some(data.name),
            nsfw: data.nsfw,
            platform: data.platform,
            rating: data.rating,
            tags,
            summary: data.summary,
            ep_bind,
        }
    }
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
pub struct BangumiEp {
    pub id: i32,
    pub name: String,
    pub name_cn: String,
    pub air_date: NaiveDateTime,
    pub ep: i32,
    pub bgm_ep_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bangumi {
    pub id: i32,
    #[serde(flatten)]
    pub info: Option<BangumiInfo>,
    #[serde(flatten)]
    pub download: Option<BangumiDownload>,
    pub eps: Option<Vec<BangumiEp>>,
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
            ep_bind: {
                if let Some(ep_bind) = bgminfo.ep_bind.clone() {
                    serde_json::to_string(&ep_bind).ok()
                } else {
                    None
                }
            },
            summary: bgminfo.summary,
        }
    }
}

impl From<BangumiInfo> for BangumiModel {
    fn from(data: BangumiInfo) -> Self {
        (&data).into()
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
    ($bgm_data:expr, $lock_data:expr, {$($prop:ident: $new_prop:expr),*}) => {
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
        BangumiEntity::insert((&info).conv::<BangumiModel>().conv::<BangumiActiveModel>())
            .exec(db)
            .await?;
        log::trace!("add bangumi {id} into database done.");
        Ok(SaveInfoData::SaveData(Bangumi {
            id,
            info: Some(info),
            download: None,
            eps: None,
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
                let new_data =
                    bangumi::get_bangumi_info_with_bgmid(&client, self.bind_bgm_id.unwrap_or(1))
                        .await?;
                let (year, month) =
                    parse_date(new_data.date.unwrap_or("1000-01-01".to_string()).as_str());
                let ep_data =
                    bangumi::get_bangumi_ep(&client, self.bind_bgm_id.unwrap_or(1)).await?;
                sync_prop_with_check! (*self, lock_data, {
                    year: Some(year),
                    season: Some(month),
                    total_ep: new_data.eps,
                    image: {
                        if let Some(image) = new_data.images {
                            image.large
                        } else {
                            None
                        }
                    },
                    name: new_data.name,
                    name_cn: new_data.name_cn,
                    nsfw: new_data.nsfw,
                    platform: new_data.platform,
                    rating: Some(new_data.rating.unwrap().score.unwrap()),
                    tags: Some(new_data.tags.unwrap().iter().map(|x| x.name.clone().unwrap()).collect()),
                    summary: new_data.summary,
                    ep_bind: Some(ep_data.data.unwrap().iter().map(|x| x.id.unwrap()).collect())
                });
                Ok(())
            }
        }
    }

    pub async fn from_id(id: i32, db: &DatabaseConnection) -> Result<Self, CoreError> {
        let data = BangumiEntity::find()
            .filter(BangumiColumn::Id.eq(id))
            .one(db)
            .await?;
        if let Some(data) = data {
            Ok(data.conv::<Self>())
        } else {
            Err(CoreError::NotFound)
        }
    }

    pub async fn from_bgm_id(bgm_id: i32, db: &DatabaseConnection) -> Result<Self, CoreError> {
        let data = BangumiEntity::find()
            .filter(BangumiColumn::BindBgmId.eq(bgm_id))
            .one(db)
            .await?;
        if let Some(data) = data {
            Ok(data.conv::<Self>())
        } else {
            Err(CoreError::NotFound)
        }
    }
}

impl Bangumi {
    pub async fn get_info_from_db(&mut self, db: &DatabaseConnection) -> Result<(), CoreError> {
        let data = BangumiInfo::from_id(self.id, db).await?;
        self.info = Some(data);
        Ok(())
    }
}
