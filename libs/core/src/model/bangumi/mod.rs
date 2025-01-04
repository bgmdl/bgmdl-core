use core::fmt;

use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use tap::Conv;

use crate::declare::db::entity::bgmdata::ActiveModel as BangumiActiveModel;
use crate::declare::db::entity::bgmdata::Entity as BangumiEntity;
use crate::declare::db::entity::bgmdata::Model as BangumiModel;

use crate::{declare::error::CoreError, model::count::gen_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BangumiStatus {
    Air,
    End,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiData {
    pub bgm_status: BangumiStatus,
    pub total_ep: Option<i32>,
    pub now_ep: Option<i32>,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub image: Option<String>,
    pub name_cn: Option<String>,
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
    pub name: String,
    pub bind_bgm_id: Option<i32>,
    #[serde(flatten)]
    pub data: Option<BangumiData>,
    #[serde(flatten)]
    pub download: Option<BangumiDownload>,
    /// lock_data: which data is locked by user.
    pub lock_data: Vec<String>,
}

impl From<&Bangumi> for BangumiModel {
    fn from(data: &Bangumi) -> Self {
        let bgminfo = data.data.clone().unwrap_or(BangumiData {
            bgm_status: BangumiStatus::NotStart,
            total_ep: None,
            now_ep: None,
            year: None,
            season: None,
            image: None,
            name_cn: None,
            nsfw: None,
            platform: None,
            rating: None,
            tags: None,
            summary: None,
        });
        BangumiModel {
            id: data.id,
            name: data.name.clone(),
            bgm_status: bgminfo.bgm_status.to_string(),
            total_ep: bgminfo.total_ep,
            now_ep: bgminfo.now_ep,
            bind_bgm_id: data.bind_bgm_id,
            year: bgminfo.year,
            season: bgminfo.season,
            image: bgminfo.image,
            name_cn: bgminfo.name_cn,
            nsfw: bgminfo.nsfw,
            platform: bgminfo.platform,
            rating: bgminfo.rating,
            tags: bgminfo.tags.clone(),
            summary: bgminfo.summary,
        }
    }
}

pub enum SyncType {
    BgmTv,
}

impl Bangumi {
    pub async fn save_info(self, db: &DatabaseConnection) -> Result<(), CoreError> {
        let id = gen_id("bgm", db).await?;
        let bgm_data = self.clone();
        log::debug!("Add bangumi: {id}");
        log::debug!("bangumi data: {:?}", &self);
        BangumiEntity::insert(
            (&bgm_data)
                .conv::<BangumiModel>()
                .conv::<BangumiActiveModel>(),
        )
        .exec(db)
        .await?;
        log::trace!("add bangumi {id} into database done.");
        Ok(())
    }

    pub async fn sync_info(
        self,
        sync_type: SyncType,
        db: &DatabaseConnection,
    ) -> Result<(), CoreError> {
        match sync_type {
            SyncType::BgmTv => Ok(()),
        }
    }
}
