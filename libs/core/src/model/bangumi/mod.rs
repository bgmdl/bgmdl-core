use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BangumiStatus {
    Air,
    End,
    NotStart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiData {
    pub id: i32,
    pub name: String,
    pub bgm_status: BangumiStatus,
    pub total_ep: i32,
    pub now_ep: i32,
    pub bind_bgm_id: Option<i32>,
    pub year: i32,
    pub season: i32,
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
    #[serde(flatten)]
    pub data: BangumiData,
    #[serde(flatten)]
    pub download: BangumiDownload,
}

impl Bangumi {
    pub fn save(self) {}
}
