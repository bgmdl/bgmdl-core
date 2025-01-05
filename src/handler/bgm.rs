use core::model::bangumi::{BangumiInfo, BangumiStatus, SaveInfoData, SyncType};

use actix_web::{get, services, web, HttpRequest, Scope};
use macro_lib::perm;
use serde::Deserialize;
use crate::utils::check_perm::check_user_permission;
use crate::handler::ResultHandler;
use crate::utils::db::get_connect;


#[derive(Debug, Deserialize)]
pub struct NewBgmProp {
    bgmid: i32,
}

#[get("/addNewFromBgmID")]
#[perm("bgm.detail")]
pub async fn new_bgm(req: HttpRequest) -> ResultHandler<String> {
    let bgmid = web::Query::<NewBgmProp>::from_query(req.query_string()).unwrap().bgmid;
    let db = get_connect().await.ok().unwrap();
    let mut new_bgm = BangumiInfo {
        bind_bgm_id: Some(bgmid),
        bgm_status: BangumiStatus::NotStart,
        ..Default::default()
    };
    new_bgm.sync_info(SyncType::BgmTv, vec![]).await?;
    let new_bgm = new_bgm.save_info(&db).await?;
    match new_bgm {
        SaveInfoData::SaveData(new_bgm) => {
            Ok(Json!{
                "status": "success",
                "data": new_bgm,
            })
        },
        _ => {
            Ok(Json!{
                "status": "failed",
                "msg": "Failed to save new bangumi info(Server Error! Please check log.)."
            })
        }
    }
}

pub fn service() -> Scope {
    let services = services![
        new_bgm,
    ];
    web::scope("/api/bangumi").service(services)
}