// extern crate serde;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoCollection {
    #[serde(rename = "collect")]
    pub collect: Option<i32>,

    #[serde(rename = "doing")]
    pub doing: Option<i32>,

    #[serde(rename = "dropped")]
    pub dropped: Option<i32>,

    #[serde(rename = "on_hold")]
    pub on_hold: Option<i32>,

    #[serde(rename = "wish")]
    pub wish: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoImages {
    #[serde(rename = "common")]
    pub common: Option<String>,

    #[serde(rename = "grid")]
    pub grid: Option<String>,

    #[serde(rename = "large")]
    pub large: Option<String>,

    #[serde(rename = "medium")]
    pub medium: Option<String>,

    #[serde(rename = "small")]
    pub small: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoInfoboxValueTypeB {
    #[serde(rename = "v")]
    pub v: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BangumiInfoInfoboxValue {
    TypeA(String),
    TypeB(Vec<BangumiInfoInfoboxValueTypeB>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoInfobox {
    #[serde(rename = "key")]
    pub key: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<BangumiInfoInfoboxValue>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoCount {
    #[serde(rename = "1")]
    pub count_1: Option<i32>,

    #[serde(rename = "10")]
    pub count_10: Option<i32>,

    #[serde(rename = "2")]
    pub count_2: Option<i32>,

    #[serde(rename = "3")]
    pub count_3: Option<i32>,

    #[serde(rename = "4")]
    pub count_4: Option<i32>,

    #[serde(rename = "5")]
    pub count_5: Option<i32>,

    #[serde(rename = "6")]
    pub count_6: Option<i32>,

    #[serde(rename = "7")]
    pub count_7: Option<i32>,

    #[serde(rename = "8")]
    pub count_8: Option<i32>,

    #[serde(rename = "9")]
    pub count_9: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoRating {
    #[serde(rename = "count")]
    pub count: Option<BangumiInfoCount>,

    #[serde(rename = "rank")]
    pub rank: Option<i32>,

    #[serde(rename = "score")]
    pub score: Option<f64>,

    #[serde(rename = "total")]
    pub total: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfoTags {
    #[serde(rename = "count")]
    pub count: Option<i32>,

    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiInfo {
    #[serde(rename = "collection")]
    pub collection: Option<BangumiInfoCollection>,

    #[serde(rename = "date")]
    pub date: Option<String>,

    #[serde(rename = "eps")]
    pub eps: Option<i32>,

    #[serde(rename = "id")]
    pub id: Option<i32>,

    #[serde(rename = "images")]
    pub images: Option<BangumiInfoImages>,

    #[serde(rename = "infobox")]
    pub infobox: Option<Vec<BangumiInfoInfobox>>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "name_cn")]
    pub name_cn: Option<String>,

    #[serde(rename = "nsfw")]
    pub nsfw: Option<bool>,

    #[serde(rename = "platform")]
    pub platform: Option<String>,

    #[serde(rename = "rating")]
    pub rating: Option<BangumiInfoRating>,

    #[serde(rename = "series")]
    pub series: Option<bool>,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<BangumiInfoTags>>,

    #[serde(rename = "total_episodes")]
    pub total_episodes: Option<i32>,

    #[serde(rename = "type")]
    pub root_type: Option<i32>,

    #[serde(rename = "volumes")]
    pub volumes: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiSearchImages {
    #[serde(rename = "common")]
    pub common: Option<String>,

    #[serde(rename = "grid")]
    pub grid: Option<String>,

    #[serde(rename = "large")]
    pub large: Option<String>,

    #[serde(rename = "medium")]
    pub medium: Option<String>,

    #[serde(rename = "small")]
    pub small: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiSearchList {
    #[serde(rename = "air_date")]
    pub air_date: Option<String>,

    #[serde(rename = "air_weekday")]
    pub air_weekday: Option<i32>,

    #[serde(rename = "id")]
    pub id: Option<i32>,

    #[serde(rename = "images")]
    pub images: Option<BangumiSearchImages>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "name_cn")]
    pub name_cn: Option<String>,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "type")]
    pub list_type: Option<i32>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiSearch {
    #[serde(rename = "list")]
    pub list: Option<Vec<BangumiSearchList>>,

    #[serde(rename = "results")]
    pub results: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiEpData {
    #[serde(rename = "airdate")]
    pub airdate: Option<String>,

    #[serde(rename = "comment")]
    pub comment: Option<i32>,

    #[serde(rename = "desc")]
    pub desc: Option<String>,

    #[serde(rename = "disc")]
    pub disc: Option<i32>,

    #[serde(rename = "duration")]
    pub duration: Option<String>,

    #[serde(rename = "duration_seconds")]
    pub duration_seconds: Option<i32>,

    #[serde(rename = "ep")]
    pub ep: Option<i32>,

    #[serde(rename = "id")]
    pub id: Option<i32>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "name_cn")]
    pub name_cn: Option<String>,

    #[serde(rename = "sort")]
    pub sort: Option<i32>,

    #[serde(rename = "subject_id")]
    pub subject_id: Option<i32>,

    #[serde(rename = "type")]
    pub data_type: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BangumiEp {
    #[serde(rename = "data")]
    pub data: Option<Vec<BangumiEpData>>,

    #[serde(rename = "limit")]
    pub limit: Option<i32>,

    #[serde(rename = "offset")]
    pub offset: Option<i32>,

    #[serde(rename = "total")]
    pub total: Option<i32>,
}
